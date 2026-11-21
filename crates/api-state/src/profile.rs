// Copyright (c) 2026 Ronan LE MEILLAT for SCTG Development
//
// This file is part of the SCTGDesk project.
//
// SCTGDesk is free software: you can redistribute it and/or modify
// it under the terms of the Affero General Public License version 3 as
// published by the Free Software Foundation.
//
// SCTGDesk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero General Public License for more details.
//
// You should have received a copy of the Affero General Public License
// along with SCTGDesk. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.

//! The profile of a user (avatar, language), the change of password, and the settings an administrator edits.
//!
//! Plain `sqlx::query` is used on purpose: the compile-time checked macros validate against a database
//! created from `db.sql`, which does not know the `avatar` and `language` columns (see `Database::init_db`).
use crate::password::UserPasswordInfo;
use crate::state::ApiState;
use crate::UserId;
use base64::Engine;
use sqlx::Row;
use utils::{UserProfile, UserProfileUpdate};

/// The languages of the console.
pub const LANGUAGES: [&str; 6] = ["en-US", "fr-FR", "de-DE", "es-ES", "pt-PT", "it-IT"];
/// The biggest avatar kept, once decoded. The console shrinks pictures before sending them.
pub const MAX_AVATAR_BYTES: usize = 256 * 1024;
/// The shortest password accepted when a user changes it.
pub const MIN_PASSWORD_LEN: usize = 8;

/// Why a profile or password change was refused.
#[derive(Debug, PartialEq, Eq)]
pub enum ProfileError {
    /// Malformed value (bad data URL, unknown language, short password): HTTP 400.
    Invalid(String),
    /// The decoded avatar exceeds [`MAX_AVATAR_BYTES`]: HTTP 413.
    TooLarge,
    /// The current password is wrong or missing: HTTP 403.
    WrongPassword,
    /// The user or the database is not available: HTTP 500.
    Internal,
}

/// Checks that `value` is a base64 `data:` URL of a png, jpeg, webp or gif no bigger than
/// [`MAX_AVATAR_BYTES`]. The type is checked here, not trusted from the browser, because the value is later put
/// in an `<img src>` of other users' pages.
pub fn validate_avatar(value: &str) -> Result<(), ProfileError> {
    let rest = value
        .strip_prefix("data:")
        .ok_or_else(|| ProfileError::Invalid("The avatar must be a data URL.".into()))?;
    let (meta, data) = rest
        .split_once(',')
        .ok_or_else(|| ProfileError::Invalid("The avatar must be a data URL.".into()))?;
    let mime = meta
        .strip_suffix(";base64")
        .ok_or_else(|| ProfileError::Invalid("The avatar must be base64 encoded.".into()))?;
    if !["image/png", "image/jpeg", "image/webp", "image/gif"].contains(&mime) {
        return Err(ProfileError::Invalid("Unsupported image type.".into()));
    }
    // reject before decoding what is obviously too big (4 base64 chars carry 3 bytes)
    if data.len() / 4 * 3 > MAX_AVATAR_BYTES + 3 {
        return Err(ProfileError::TooLarge);
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|_| ProfileError::Invalid("The avatar is not valid base64.".into()))?;
    if bytes.len() > MAX_AVATAR_BYTES {
        return Err(ProfileError::TooLarge);
    }
    Ok(())
}

impl ApiState {
    /// Deletes connection records: the listed guids, or all of them. The caller checks the user is an
    /// administrator. Returns how many rows were removed (0 on a database error).
    pub async fn audit_conn_delete(&self, guids: &[String], all: bool) -> u64 {
        let Ok(mut conn) = self.db.pool.acquire().await else {
            return 0;
        };
        if all {
            return sqlx::query("DELETE FROM conn_history")
                .execute(&mut *conn)
                .await
                .map(|r| r.rows_affected())
                .unwrap_or(0);
        }
        let mut deleted = 0;
        for guid in guids {
            // an unparsable guid cannot match anything: skip it rather than fail the whole purge
            let Some(bytes) = utils::uuid_into_guid(guid) else {
                continue;
            };
            deleted += sqlx::query("DELETE FROM conn_history WHERE guid = ?")
                .bind(bytes)
                .execute(&mut *conn)
                .await
                .map(|r| r.rows_affected())
                .unwrap_or(0);
        }
        deleted
    }

    /// The profile of a user, `None` if the user does not exist.
    pub async fn user_profile(&self, user_id: &UserId) -> Option<UserProfile> {
        let mut conn = self.db.pool.acquire().await.ok()?;
        let row =
            sqlx::query(r#"SELECT name, email, role, avatar, language FROM "user" WHERE guid = ?"#)
                .bind(user_id)
                .fetch_optional(&mut *conn)
                .await
                .ok()??;
        Some(UserProfile {
            name: row.get("name"),
            email: row.get::<Option<String>, _>("email").unwrap_or_default(),
            admin: row.get::<i64, _>("role") != 0,
            avatar: row.get("avatar"),
            language: row.get("language"),
        })
    }

    /// Applies a profile update (absent fields are kept) and returns the new profile.
    pub async fn user_profile_update(
        &self,
        user_id: &UserId,
        update: UserProfileUpdate,
    ) -> Result<UserProfile, ProfileError> {
        // validate everything before writing anything: no half-applied update
        if let Some(language) = &update.language {
            if !LANGUAGES.contains(&language.as_str()) {
                return Err(ProfileError::Invalid("Unknown language.".into()));
            }
        }
        let avatar = match &update.avatar {
            None => None,
            Some(None) => Some(None),
            Some(Some(v)) if v.is_empty() => Some(None),
            Some(Some(v)) => {
                validate_avatar(v)?;
                Some(Some(v.clone()))
            }
        };
        let mut conn = self
            .db
            .pool
            .acquire()
            .await
            .map_err(|_| ProfileError::Internal)?;
        if let Some(avatar) = avatar {
            sqlx::query(r#"UPDATE "user" SET avatar = ? WHERE guid = ?"#)
                .bind(avatar)
                .bind(user_id)
                .execute(&mut *conn)
                .await
                .map_err(|_| ProfileError::Internal)?;
        }
        if let Some(language) = &update.language {
            sqlx::query(r#"UPDATE "user" SET language = ? WHERE guid = ?"#)
                .bind(language)
                .bind(user_id)
                .execute(&mut *conn)
                .await
                .map_err(|_| ProfileError::Internal)?;
        }
        drop(conn);
        self.user_profile(user_id)
            .await
            .ok_or(ProfileError::Internal)
    }

    /// Changes the password of a user after checking the current one. An account whose stored hash is
    /// empty (no local password) may set one without it.
    pub async fn user_change_password(
        &self,
        user_id: &UserId,
        current: Option<&str>,
        new: &str,
    ) -> Result<(), ProfileError> {
        if new.chars().count() < MIN_PASSWORD_LEN {
            return Err(ProfileError::Invalid(format!(
                "The password must have at least {MIN_PASSWORD_LEN} characters."
            )));
        }
        let mut conn = self
            .db
            .pool
            .acquire()
            .await
            .map_err(|_| ProfileError::Internal)?;
        let stored: Option<String> =
            sqlx::query_scalar(r#"SELECT password FROM "user" WHERE guid = ?"#)
                .bind(user_id)
                .fetch_optional(&mut *conn)
                .await
                .map_err(|_| ProfileError::Internal)?;
        let stored = stored.ok_or(ProfileError::Internal)?;
        if !stored.is_empty() {
            // `verify` errors on a malformed hash: treat it as a mismatch, never as a success
            let ok = current
                .map(|c| bcrypt::verify(c, &stored).unwrap_or(false))
                .unwrap_or(false);
            if !ok {
                return Err(ProfileError::WrongPassword);
            }
        }
        sqlx::query(r#"UPDATE "user" SET password = ? WHERE guid = ?"#)
            .bind(UserPasswordInfo::hash_password(new))
            .bind(user_id)
            .execute(&mut *conn)
            .await
            .map_err(|_| ProfileError::Internal)?;
        Ok(())
    }

    /// Forgets the cached OAuth2 providers so the next lookup rereads `oauth2.toml`.
    pub async fn invalidate_oauth2_config(&self) {
        self.oauth2_providers.write().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::AddUserRequest;

    const PNG: &str = "data:image/png;base64,iVBORw0KGgo=";

    #[test]
    fn avatar_validation() {
        assert_eq!(validate_avatar(PNG), Ok(()));
        assert!(matches!(
            validate_avatar("http://x/a.png"),
            Err(ProfileError::Invalid(_))
        ));
        assert!(matches!(
            validate_avatar("data:image/svg+xml;base64,AAAA"),
            Err(ProfileError::Invalid(_))
        ));
        assert!(matches!(
            validate_avatar("data:image/png;base64,@@@"),
            Err(ProfileError::Invalid(_))
        ));
        let big = format!("data:image/png;base64,{}", "A".repeat(MAX_AVATAR_BYTES * 2));
        assert_eq!(validate_avatar(&big), Err(ProfileError::TooLarge));
    }

    async fn state(name: &str) -> (ApiState, UserId) {
        let p =
            std::env::temp_dir().join(format!("profile-{}-{}.sqlite3", name, std::process::id()));
        for ext in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{}", p.display(), ext));
        }
        let state = ApiState::new_with_db(&p).await;
        state
            .add_user(AddUserRequest {
                name: "alice".into(),
                password: "pw-alice-1".into(),
                confirm_password: "pw-alice-1".into(),
                email: "a@example.org".into(),
                is_admin: false,
                group_name: "Default".into(),
            })
            .await;
        let id: UserId = sqlx::query_scalar(r#"SELECT guid FROM "user" WHERE name = 'alice'"#)
            .fetch_one(&mut *state.db.pool.acquire().await.unwrap())
            .await
            .unwrap();
        (state, id)
    }

    #[tokio::test]
    async fn profile_roundtrip_and_validation() {
        let (state, id) = state("profile").await;
        let p = state.user_profile(&id).await.unwrap();
        assert_eq!(
            (p.name.as_str(), p.avatar.as_deref(), p.language.as_deref()),
            ("alice", None, None)
        );
        let p = state
            .user_profile_update(
                &id,
                UserProfileUpdate {
                    avatar: Some(Some(PNG.into())),
                    language: Some("fr-FR".into()),
                },
            )
            .await
            .unwrap();
        assert_eq!(p.avatar.as_deref(), Some(PNG));
        assert_eq!(p.language.as_deref(), Some("fr-FR"));
        // an absent field is kept
        let p = state
            .user_profile_update(
                &id,
                UserProfileUpdate {
                    avatar: None,
                    language: Some("de-DE".into()),
                },
            )
            .await
            .unwrap();
        assert_eq!(p.avatar.as_deref(), Some(PNG));
        // an unknown language is refused and changes nothing
        assert!(state
            .user_profile_update(
                &id,
                UserProfileUpdate {
                    avatar: Some(None),
                    language: Some("xx".into())
                }
            )
            .await
            .is_err());
        assert!(state.user_profile(&id).await.unwrap().avatar.is_some());
        // null removes
        let p = state
            .user_profile_update(
                &id,
                UserProfileUpdate {
                    avatar: Some(None),
                    language: None,
                },
            )
            .await
            .unwrap();
        assert!(p.avatar.is_none());
    }

    #[tokio::test]
    async fn password_change() {
        let (state, id) = state("password").await;
        assert_eq!(
            state
                .user_change_password(&id, Some("nope"), "newpassword1")
                .await,
            Err(ProfileError::WrongPassword)
        );
        assert_eq!(
            state.user_change_password(&id, None, "newpassword1").await,
            Err(ProfileError::WrongPassword)
        );
        assert!(matches!(
            state
                .user_change_password(&id, Some("pw-alice-1"), "short")
                .await,
            Err(ProfileError::Invalid(_))
        ));
        assert_eq!(
            state
                .user_change_password(&id, Some("pw-alice-1"), "newpassword1")
                .await,
            Ok(())
        );
        assert!(state
            .user_login(
                &"alice".to_string(),
                UserPasswordInfo::from_password("newpassword1"),
                false
            )
            .await
            .is_some());
    }

    #[tokio::test]
    async fn audit_delete() {
        let (state, _) = state("audit").await;
        let mut conn = state.db.pool.acquire().await.unwrap();
        // a fresh database only gets db.sql, not the migrations (see Database::init_db): create the table
        // from its migration so the test does not depend on that
        sqlx::raw_sql(include_str!(
            "../../api-server/db_v2/migrations/20261110100000_conn_history.sql"
        ))
        .execute(&mut *conn)
        .await
        .unwrap();
        for i in 0..3u8 {
            sqlx::query("INSERT INTO conn_history (guid, device_id, conn_id, session_id, conn_type) VALUES (?, 'd', ?, ?, 0)")
                .bind(vec![i; 16])
                .bind(i as i64)
                .bind(i.to_string())
                .execute(&mut *conn)
                .await
                .unwrap();
        }
        drop(conn);
        let guid = utils::guid_into_uuid(vec![0u8; 16]).unwrap();
        assert_eq!(
            state
                .audit_conn_delete(&[guid, "garbage".into()], false)
                .await,
            1
        );
        assert_eq!(state.audit_conn_delete(&[], false).await, 0);
        assert_eq!(state.audit_conn_delete(&[], true).await, 2);
    }
}
