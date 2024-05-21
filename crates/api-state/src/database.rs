use crate::password::UserPasswordInfo;
use crate::types;
use crate::UserId;
use serde::Serialize;
use sqlx::{
    pool::PoolConnection,
    sqlite::{Sqlite, SqliteConnectOptions, SqliteJournalMode, SqlitePool},
    QueryBuilder,
};
use utils::Group;
use utils::Peer;
use std::env;
use std::path::Path;
use utils::types::AddressBook;
use utils::AbPeer;
use utils::AbTag;
use utils::UpdateUserRequest;
use utils::UserListResponse;

use base64::prelude::{Engine as _, BASE64_STANDARD};

use uuid::Uuid;

pub struct Database {
    pool: SqlitePool,
}

pub struct DatabaseConnection {
    conn: PoolConnection<Sqlite>,
}

pub struct DatabaseUserInfo {
    pub active: bool,
    pub admin: bool,
}

#[derive(Serialize, Debug)]
pub struct DatabaseUserPasswordInfo {
    pub password: String,
    pub username: String,
    pub user_id: UserId,
}

macro_rules! unwrap_or_return_tuple {
    ($first:expr, $opt:expr) => {
        match $opt {
            Some(v) => v,
            None => return ($first, None),
        }
    };
}

impl Database {
    pub async fn open<P: AsRef<Path>>(db_filename: P) -> Self {
        let db_opts = SqliteConnectOptions::new()
            .filename(db_filename.as_ref())
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(db_opts).await.unwrap();

        Self::init_db(&pool).await;

        Self { pool }
    }

    async fn init_db(pool: &SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        // default admin password is "Hello,world!"

        // run migrations only if the db is empty
        let res = sqlx::query!(
            r#"
            SELECT
                name
            FROM
                sqlite_master
            WHERE
                type='table'
        "#
        )
        .fetch_all(&mut conn)
        .await;
        if res.is_err() {
            log::debug!("init_db error: {:?}", res);
            return;
        }
        let res = res.unwrap();
        if res.len() > 0 {
            let migrator = sqlx::migrate!("../../db_v2/migrations/");
            migrator.run(pool).await.unwrap();
        }

        // query_file! macro use path relative to Cargo.toml
        sqlx::query_file!("../../db_v2/create/db.sql")
            .execute(&mut conn)
            .await
            .unwrap();
        // Load and run migrations
    }

    pub async fn find_user_by_name(
        &self,
        username: &str,
    ) -> (DatabaseConnection, Option<(UserId, Option<String>, DatabaseUserInfo)>) {
        let mut conn = DatabaseConnection {
            conn: self.pool.acquire().await.unwrap(),
        };

        let res = unwrap_or_return_tuple!(
            conn,
            sqlx::query!(
                r#"
            SELECT
                guid,
                status,
                role,
                email
            FROM
                user
            WHERE
                name = ?
        "#,
                username
            )
            .fetch_one(&mut conn.conn)
            .await
            .ok()
        );

        let user_id: UserId = res.guid;
        let email = res.email;
        let dbi = DatabaseUserInfo {
            active: res.status == 1,
            admin: res.role == 1,
        };

        (conn, Some((user_id, email, dbi)))
    }

    pub async fn get_user_hashed_password(
        &self,
        mut conn: DatabaseConnection,
        user_id: UserId,
    ) -> (DatabaseConnection, Option<DatabaseUserPasswordInfo>) {
        let res = unwrap_or_return_tuple!(
            conn,
            sqlx::query!(
                r#"
            SELECT
                guid,
                name,
                password
            FROM
                user
            WHERE
                guid = ?
        "#,
                user_id
            )
            .fetch_one(&mut conn.conn)
            .await
            .ok()
        );

        let dbpi = DatabaseUserPasswordInfo {
            password: res.password,
            username: res.name,
            user_id: res.guid,
        };

        (conn, Some(dbpi))
    }

    /// Get the hashed password info for a user with the given username
    ///
    /// # Arguments
    /// * `conn` - The database connection
    /// * `username` - The username of the user
    ///
    /// # Returns
    /// * `DatabaseConnection` - The database connection
    /// * `Option<DatabaseUserPasswordInfo>` - The hashed password info for the user if the user exists, `None` otherwise
    pub async fn get_user_hashed_password_with_username(
        &self,
        mut conn: DatabaseConnection,
        username: String,
    ) -> (DatabaseConnection, Option<DatabaseUserPasswordInfo>) {
        let res = unwrap_or_return_tuple!(
            conn,
            sqlx::query!(
                r#"
            SELECT
                guid,
                name,
                password
            FROM
                user
            WHERE
                name = ?
        "#,
                username
            )
            .fetch_one(&mut conn.conn)
            .await
            .ok()
        );

        let dbpi = DatabaseUserPasswordInfo {
            password: res.password,
            username: res.name,
            user_id: res.guid,
        };

        (conn, Some(dbpi))
    }

    pub async fn get_legacy_address_book(&self, user_id: UserId) -> Option<AddressBook> {
        let mut conn = self.pool.acquire().await.unwrap();

        let res = sqlx::query!(
            r#"
            SELECT
                ab as "ab!: String"
            FROM
                ab_legacy
            WHERE
                user_guid = ?
        "#,
            user_id
        )
        .fetch_one(&mut conn)
        .await
        .ok()?;

        let ab = AddressBook { ab: res.ab };

        Some(ab)
    }

    // Update user password
    // first check if password is correct
    // if correct, update password
    // if not correct, return error
    pub async fn update_user_password(
        &self,
        username: String,
        old_password: String,
        new_password: String,
    ) -> Option<()> {
        let conn = DatabaseConnection {
            conn: self.pool.acquire().await.unwrap(),
        };
        let (_, dbpi) = self
            .get_user_hashed_password_with_username(conn, username)
            .await;

        if dbpi.is_none() {
            return None;
        }

        let dbpi = dbpi.unwrap();
        let user_id = dbpi.user_id.clone();
        let old_password_info = UserPasswordInfo::from_password(old_password.as_str());
        if !old_password_info.check(dbpi) {
            return None;
        }
        let new_password_hashed = UserPasswordInfo::hash_password(new_password.as_str());
        let mut tx = self.pool.begin().await.unwrap();
        let res = sqlx::query!(
            r#"
            UPDATE
                user
            SET
                password = ?
            WHERE
                guid = ?
        "#,
            new_password_hashed,
            user_id
        )
        .execute(&mut tx)
        .await
        .ok()?
        .rows_affected();
        tx.commit().await.ok()?;
        if res == 0 {
            return None;
        }
        Some(())
    }

    // reset user password
    pub async fn reset_user_password(&self, username: String, new_password: String) -> Option<()> {
        let new_password_hashed = UserPasswordInfo::hash_password(new_password.as_str());
        let mut tx = self.pool.begin().await.unwrap();
        let res = sqlx::query!(
            r#"
            UPDATE
                user
            SET
                password = ?
            WHERE
                name = ?
        "#,
            new_password_hashed,
            username
        )
        .execute(&mut tx)
        .await
        .ok()?
        .rows_affected();
        tx.commit().await.ok()?;
        if res == 0 {
            return None;
        }
        Some(())
    }

    pub async fn update_legacy_address_books(
        &self,
        mut values: Vec<(UserId, AddressBook)>,
    ) -> Option<()> {
        let mut tx = self.pool.begin().await.unwrap();

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
            // spaces as that might interfere with identifiers or quoted strings where exact
            // values may matter.
            "INSERT OR REPLACE INTO ab_legacy (user_guid, ab) ",
        );

        let values_count = values.len() as u64;

        // Note that `.into_iter()` wasn't needed here since `users` is already an iterator.
        query_builder.push_values(values.drain(..), |mut b, value| {
            b.push_bind(value.0).push_bind(value.1.ab);
        });

        let query = query_builder.build();

        let res = query.execute(&mut tx).await.ok()?.rows_affected();

        if res != values_count {
            return None;
        }

        tx.commit().await.ok()?;

        Some(())
    }

    pub async fn ui_get_all_users(&self) -> Option<Vec<types::UserInfo>> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query_as!(
            types::UserInfo,
            r#"
            SELECT
                user.guid as id,
                user.status as "active!: bool",
                user.role as "admin!: bool",
                user.name as username,
                user.password,
                ab_legacy.ab as "address_book!: String"
            FROM
                user
                LEFT JOIN ab_legacy
                    ON ab_legacy.user_guid = user.guid
            "#
        )
        .fetch_all(&mut conn)
        .await
        .ok()?;

        Some(res)
    }

    pub async fn ui_get_user_info(&self, username: String) -> Option<types::UserInfo> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query_as!(
            types::UserInfo,
            r#"
            SELECT
                user.guid as id,
                user.status as "active!: bool",
                user.role as "admin!: bool",
                user.name as username,
                user.password,
                ab_legacy.ab as "address_book!: String" 
            FROM
                user
                LEFT JOIN ab_legacy
                    ON ab_legacy.user_guid = user.guid
            WHERE
                user.name = ?
            "#,
            username
        )
        .fetch_one(&mut conn)
        .await
        .ok()?;

        Some(res)
    }

    pub async fn create_user(
        &self,
        username: String,
        password: String,
        admin: bool,
    ) -> Option<UserId> {
        let password_hashed = UserPasswordInfo::hash_password(password.as_str());
        let mut tx = self.pool.begin().await.unwrap();
        let _res = sqlx::query!(
            r#"
            INSERT INTO
                user (status, role, name, password)
            VALUES
                (1, ?, ?, ?)
        "#,
            admin,
            username,
            password_hashed
        )
        .execute(&mut tx)
        .await
        .ok()?;

        let user_id: (Vec<u8>,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(&mut tx)
            .await
            .ok()?;

        tx.commit().await.ok()?;
        Some(user_id.0 as UserId)
    }

    pub async fn delete_user(&self, user_id: UserId) -> Option<()> {
        let mut tx = self.pool.begin().await.unwrap();
        let res = sqlx::query!(
            r#"
            DELETE FROM
                user
            WHERE
                guid = ?
        "#,
            user_id
        )
        .execute(&mut tx)
        .await
        .ok()?
        .rows_affected();
        tx.commit().await.ok()?;
        if res == 0 {
            return None;
        }
        Some(())
    }

    pub async fn update_systeminfo(&self, systeminfo: utils::SystemInfo) -> Option<()> {
        let mut tx = self.pool.begin().await.unwrap();
        let mut systeminfo = systeminfo;
        let uuid = systeminfo.uuid.clone().unwrap();

        let uuid_decoded = BASE64_STANDARD.decode(uuid);
        if uuid_decoded.is_ok() {
            let uuid_decoded = uuid_decoded.unwrap();
            log::debug!(
                "uuid_decoded: {:?} {:?}",
                uuid_decoded,
                String::from_utf8(uuid_decoded.clone())
            );
            // get old info (if any for keeping ip setted by hbbs)
            let res = sqlx::query!(
                r#"SELECT info as "info!: String" FROM peer WHERE uuid = ?"#,
                uuid_decoded
            )
            .fetch_one(&mut tx)
            .await;
            if res.is_err() {
                log::debug!("peer select error: {:?}", res);
                return None;
            } else {
                let res = res.unwrap();
                let old_systeminfo: utils::SystemInfo =
                    rocket::serde::json::from_str(&res.info).unwrap();
                systeminfo.ip = old_systeminfo.ip.clone();
            }
            let systeminfo_string = rocket::serde::json::to_string(&systeminfo).unwrap();
            log::debug!("systeminfo_string: {:?}", systeminfo_string);
            let res = sqlx::query!(
                r#"UPDATE peer SET info = ? WHERE uuid = ?"#,
                systeminfo_string,
                uuid_decoded
            )
            .execute(&mut tx)
            .await
            .ok()?
            .rows_affected();
            tx.commit().await.ok()?;
            if res == 0 {
                return None;
            } else {
                return Some(());
            }
        }
        Some(())
    }

    pub async fn update_heartbeat(&self, heartbeat: utils::HeartbeatRequest) -> Option<()> {
        let uuid = heartbeat.uuid.clone();
        let uuid_decoded = BASE64_STANDARD.decode(uuid);
        if uuid_decoded.is_ok() {
            let uuid_decoded = uuid_decoded.unwrap();
            log::debug!(
                "uuid_decoded: {:?} {:?}",
                uuid_decoded,
                String::from_utf8(uuid_decoded.clone())
            );
            let res = sqlx::query!(
                r#"UPDATE peer SET last_online = current_timestamp WHERE uuid = ?"#,
                uuid_decoded
            )
            .execute(&self.pool)
            .await;
            if res.is_err() {
                log::debug!("update_heartbeat error: {:?}", res);
                return None;
            }
            let res = res.unwrap().rows_affected();

            if res == 0 {
                return None;
            } else {
                log::debug!("update_heartbeat row affected: {:?}", res);
                return Some(());
            }
        }
        None
    }

    /// Get user for oauth2 flow
    /// if the user does not exist, create it with status=0 and role=0
    /// if environment variable OAUTH2_CREATE_USER is set to 1, create the user with status=1 and role=0
    /// if the user exists return its guid
    ///
    /// # Arguments
    /// * `id` - peer id
    /// * `uuid` - peer uuid
    ///
    /// # Returns
    /// Option<(UserId, name: String,DatabaseUserInfo)>
    ///
    pub async fn get_user_for_oauth2(
        &self,
        id: String,
        email: String,
        uuid: String,
    ) -> Option<(UserId, String, DatabaseUserInfo)> {
        let mut conn = DatabaseConnection {
            conn: self.pool.acquire().await.unwrap(),
        };
        let status = { env::var("OAUTH2_CREATE_USER").unwrap_or("0".to_string()) == "1" };
        let ab_guid = Uuid::new_v4().as_bytes().to_vec();
        let user_guid = Uuid::new_v4().as_bytes().to_vec();
        let random_password = Uuid::new_v4().to_string();
        let hashed_random_password = UserPasswordInfo::hash_password(random_password.as_str());
        log::debug!(
            "user: {:?}/{:?} has random_password: {:?}",
            uuid,
            id,
            random_password
        );
        let name = format!("{}'s Personal Address Book", id);
        let res = sqlx::query!(
            r#"
            INSERT OR IGNORE INTO user(guid, grp, team, status, role, name, email, password)
                VALUES (?,
                    (SELECT guid FROM grp  WHERE name = 'Default'),
                    (SELECT guid FROM team  WHERE name = 'Default'), ?, 0, ?, ?, ?);
            INSERT OR IGNORE INTO ab(guid, name, owner, personal, info)
                VALUES (?,?,?,1,'{}');
            "#,
            user_guid,
            status,
            id,
            email,
            hashed_random_password,
            ab_guid,
            name,
            user_guid
        )
        .execute(&mut conn.conn)
        .await;
        if res.is_err() {
            log::error!(
                "get_user_for_oauth2 error while creating/getting user: {:?}",
                res
            );
        }
        let res = sqlx::query!(
            r#"  
            SELECT guid, status, role, name FROM user WHERE name = ?;
            "#,
            id
        )
        .fetch_one(&mut conn.conn)
        .await;

        if res.is_err() {
            log::error!(
                "get_user_for_oauth2 error while creating/getting user: {:?}",
                res
            );
            return None;
        }
        let res = res.unwrap();
        let user_id: UserId = res.guid;
        let dbi = DatabaseUserInfo {
            active: res.status == 1,
            admin: res.role == 1,
        };
        Some((user_id, res.name, dbi))
    }

    pub async fn get_personal_address_book(&self, user_id: UserId) {
        let mut conn = self.pool.acquire().await.unwrap();

        let _res = sqlx::query!(
            r#"
            SELECT
                guid
            FROM
                ab
            WHERE
                owner = ?
                AND personal = 1
        "#,
            user_id
        )
        .fetch_one(&mut conn)
        .await
        .ok();
    }

    /// Get the personal address book guid for a user
    /// if the user is inactive (status != 1) return None
    pub async fn get_ab_personal_guid(&self, user_id: UserId) -> Option<String> {
        let mut conn = self.pool.acquire().await.unwrap();

        let res = sqlx::query!(
            r#"
            SELECT
                a.guid
            FROM
                ab as a, user as u
            WHERE
                u.guid = ?
                AND a.personal = 1
                AND a.owner = u.guid
                AND u.status = 1
        "#,
            user_id
        )
        .fetch_one(&mut conn)
        .await;
        if res.is_err() {
            log::error!("get_ab_personal_guid error: {:?}", res);
            return None;
        }

        let res = res.unwrap();
        let guid_u8: Result<[u8; 16], _> = res.guid.try_into();
        if guid_u8.is_err() {
            log::error!("get_ab_personal_guid error: {:?}", guid_u8);
            return None;
        }
        let guid_u8: [u8; 16] = guid_u8.unwrap();
        let guid = Uuid::from_bytes(guid_u8).to_string();
        Some(guid)
    }

    /// Add a peer to the address book
    pub async fn add_peer_to_ab(&self, ab: &str, ab_peer: AbPeer) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("add_peer_to_ab error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let ab_peer_guid = Uuid::new_v4().as_bytes().to_vec();
        let ab_peer_json = rocket::serde::json::to_string(&ab_peer).unwrap();
        let res = sqlx::query!(
            r#"
            -- must be improved !
            DELETE FROM ab_peer WHERE ab = ? AND id = ?;
            INSERT OR IGNORE INTO ab_peer (
                guid,
                ab,
                peer,
                id,
                note,
                created_at,
                info
                )
                VALUES (?, ?, (select guid from peer where id = ?), ?, ?, current_timestamp, ?)
        "#,
            ab_guid,
            ab_peer.id,
            ab_peer_guid,
            ab_guid,
            ab_peer.id,
            ab_peer.id,
            "",
            ab_peer_json
        )
        .execute(&mut conn)
        .await;
        if res.is_err() {
            log::error!("add_peer_to_ab error: {:?}", res);
            return None;
        }
        Some(())
    }

    pub async fn get_peers_from_ab(&self, ab: &str) -> Option<Vec<AbPeer>> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("get_peers_from_ab error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            SELECT
                info
            FROM
                ab_peer
            WHERE
                ab_peer.ab = ?
        "#,
            ab_guid
        )
        .fetch_all(&mut conn)
        .await;
        if res.is_err() {
            log::error!("get_peers_from_ab error: {:?}", res);
            return None;
        }
        let res = res.unwrap();
        let mut ab_peers = Vec::new();
        for row in res {
            let ab_peer: AbPeer = rocket::serde::json::from_str(&row.info).unwrap();
            log::debug!("ab_peer: {:?}", ab_peer);
            ab_peers.push(ab_peer);
        }
        Some(ab_peers)
    }

    pub async fn delete_peer_from_ab(&self, ab: &str, id: &str) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("delete_peer_from_ab error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            DELETE FROM ab_peer WHERE ab = ? AND id = ?
        "#,
            ab_guid,
            id
        )
        .execute(&mut conn)
        .await;
        if res.is_err() {
            log::error!("delete_peer_from_ab error: {:?}", res);
            return None;
        }
        Some(())
    }

    pub async fn get_ab_peer(&self, ab: &str, id: &str) -> Option<AbPeer> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("get_ab_peer error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            SELECT
                info
            FROM
                ab_peer
            WHERE
                ab_peer.ab = ? AND ab_peer.id = ?
        "#,
            ab_guid,
            id
        )
        .fetch_one(&mut conn)
        .await;
        if res.is_err() {
            log::error!("get_ab_peer error: {:?}", res);
            return None;
        }
        let res = res.unwrap();
        let ab_peer: AbPeer = rocket::serde::json::from_str(&res.info).unwrap();
        Some(ab_peer)
    }

    pub async fn add_tag_to_ab(&self, ab: &str, tag: AbTag) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("add_tag_to_ab error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            DELETE FROM ab_tag WHERE ab = ? AND name = ?;
            INSERT OR IGNORE INTO ab_tag (ab, name, color) VALUES (?, ?, ?)
        "#,
            ab_guid,
            tag.name,
            ab_guid,
            tag.name,
            tag.color
        )
        .execute(&mut conn)
        .await;
        if res.is_err() {
            log::error!("add_tag_to_ab error: {:?}", res);
            return None;
        }
        Some(())
    }

    pub async fn get_ab_tags(&self, ab: &str) -> Option<Vec<AbTag>> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("get_ab_tags error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            SELECT
                name,
                color
            FROM
                ab_tag
            WHERE
                ab = ?
        "#,
            ab_guid
        )
        .fetch_all(&mut conn)
        .await;
        if res.is_err() {
            log::error!("get_ab_tags error: {:?}", res);
            return None;
        }
        let res = res.unwrap();
        let mut ab_tags = Vec::new();
        for row in res {
            let ab_tag = AbTag {
                name: row.name,
                color: row.color as u32,
            };
            ab_tags.push(ab_tag);
        }
        Some(ab_tags)
    }

    pub async fn get_ab_tag(&self, ab: &str, tag: &str) -> Option<AbTag> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("get_ab_tag error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            SELECT
                name,
                color
            FROM
                ab_tag
            WHERE
                ab = ?
                AND name = ?
        "#,
            ab_guid,
            tag
        )
        .fetch_all(&mut conn)
        .await;
        if res.is_err() {
            log::error!("get_ab_tags error: {:?}", res);
            return None;
        }
        let res = res.unwrap();
        if res.len() == 0 {
            return None;
        }
        let ab_tag = AbTag {
            name: res[0].name.clone(),
            color: res[0].color as u32,
        };
        Some(ab_tag)
    }

    pub async fn rename_ab_tag(&self, ab: &str, old_name: &str, tag: AbTag) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("rename_ab_tag error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            UPDATE ab_tag SET name = ?, color = ? WHERE ab = ? AND name = ?
        "#,
            tag.name,
            tag.color,
            ab_guid,
            old_name
        )
        .execute(&mut conn)
        .await;
        if res.is_err() {
            log::error!("rename_ab_tag error: {:?}", res);
            return None;
        }
        Some(())
    }

    pub async fn delete_tag_from_ab(&self, ab: &str, tag: &str) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let ab_guid = Uuid::parse_str(ab);
        if ab_guid.is_err() {
            log::error!("delete_tag_from_ab error: {:?}", ab_guid);
            return None;
        }
        let ab_guid = ab_guid.unwrap().as_bytes().to_vec();
        let res = sqlx::query!(
            r#"
            DELETE FROM ab_tag WHERE ab = ? AND name = ?
        "#,
            ab_guid,
            tag
        )
        .execute(&mut conn)
        .await;
        if res.is_err() {
            log::error!("delete_tag_from_ab error: {:?}", res);
            return None;
        }
        Some(())
    }

    pub async fn add_user(
        &self,
        name: String,
        password: String,
        email: String,
        is_admin: bool,
        group_name: String,
    ) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let user_guid = Uuid::new_v4().as_bytes().to_vec();
        // Check if the group exists
        let res = sqlx::query!(
            r#"
            SELECT
                guid
            FROM
                grp
            WHERE
                name = ?
        "#,
            group_name
        )
        .fetch_all(&mut conn)
        .await;
        if res.is_err() {
            log::error!("add_user error: {:?}", res);
            return None;
        }
        let res = res.unwrap();
        if res.len() == 0 {
            return None;
        }
        let group_guid: Vec<u8> = res[0].guid.clone();
        let ab_guid = Uuid::new_v4().as_bytes().to_vec();
        let password_hashed = UserPasswordInfo::hash_password(password.as_str());
        let res = sqlx::query!(
            r#"
            INSERT OR IGNORE INTO user(guid, grp, team, status, role, name, password, email)
                VALUES (?,
                    ?,
                    (SELECT guid FROM team  WHERE name = 'Default'), 1, ?, ?, ?, ?);
            INSERT OR IGNORE INTO ab(guid, name, owner, personal, info)
                VALUES (?,"Personal Address Book",?,1,'{}');
            "#,
            user_guid,
            group_guid,
            is_admin,
            name,
            password_hashed,
            email,
            ab_guid,
            user_guid
        )
        .execute(&mut conn)
        .await;
        if res.is_err() {
            log::error!("add_user error: {:?}", res);
            return None;
        }
        Some(())
    }

    /// Change user status
    /// if status is 1, the user is active
    /// if status is 0, the user is inactive
    pub async fn user_change_status(&self, name: &str, status: u32) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query!(
            r#"
            UPDATE
                user
            SET
                status = ?
            WHERE
                name = ?
        "#,
            status,
            name
        )
        .execute(&mut conn)
        .await;
        if res.is_err() {
            log::error!("change_user_status error: {:?}", res);
            return None;
        }
        Some(())
    }

    pub async fn get_all_users(
        &self,
        name: Option<&str>,
        email: Option<&str>,
        current: u32,
        page_size: u32,
    ) -> Option<Vec<UserListResponse>> {
        let mut conn = self.pool.acquire().await.unwrap();
        let mut email_filter = "%";
        let mut name_filter = "%";
        if email.is_some() {
            email_filter = email.unwrap();
        }
        if name.is_some() {
            name_filter = name.unwrap();
        }
        let current = {
            if current < 1 {
                1
            } else {
                current
            }
        };
        let offset = ((current - 1) * page_size) as i32;
        let page_size = page_size as i32;

        let res = sqlx::query!(
            r#"
            SELECT
                user.guid as id,
                user.status as "active!: i32",
                user.role as "admin!: bool",
                user.name as username,
                user.email as email,
                user.note as note,
                grp.name as group_name
            FROM
                user
                LEFT JOIN grp
                    ON user.grp = grp.guid
            WHERE
                user.name LIKE ?
                AND user.email LIKE ?
            LIMIT ?
            OFFSET ?
        "#,
            name_filter,
            email_filter,
            page_size,
            offset
        )
        .fetch_all(&mut conn)
        .await;
        if res.is_err() {
            log::error!("get_all_users error: {:?}", res);
            return None;
        }
        let res = res.unwrap();
        let mut users: Vec<UserListResponse> = Vec::new();
        for row in res {
            let guid_u8: Result<[u8; 16], _> = row.id.try_into();
            if guid_u8.is_err() {
                log::error!("get_ab_personal_guid error: {:?}", guid_u8);
                return None;
            }
            let guid_u8: [u8; 16] = guid_u8.unwrap();
            let guid = Uuid::from_bytes(guid_u8).to_string();
            let user = UserListResponse {
                guid: guid,
                name: row.username,
                email: row.email.unwrap_or("".to_string()),
                note: row.note,
                status: row.active,
                is_admin: row.admin,
                group_name: row.group_name.unwrap_or("Defaut".to_string()),
            };
            users.push(user);
        }
        Some(users)
    }

    pub async fn user_update(
        &self,
        user_id: UserId,
        user_parameters: UpdateUserRequest,
    ) -> Option<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        let mut query = "UPDATE user SET ".to_string();
        let mut query_params = Vec::new();
        if user_parameters.name.is_some() {
            query.push_str("name = ?, ");
            query_params.push(user_parameters.name.unwrap());
        }
        if user_parameters.email.is_some() {
            query.push_str("email = ?, ");
            query_params.push(user_parameters.email.unwrap());
        }
        if user_parameters.note.is_some() {
            query.push_str("note = ?, ");
            query_params.push(user_parameters.note.unwrap());
        }
        if user_parameters.password.is_some() && user_parameters.confirm_password.is_some() {
            let password = user_parameters.password.unwrap();
            let confirm_password = user_parameters.confirm_password.unwrap();
            if password == confirm_password {
                let password_hashed = UserPasswordInfo::hash_password(password.as_str());
                query.push_str("password = ?, ");
                query_params.push(password_hashed);
            }
        }
        if let Some(status) = user_parameters.status {
            query.push_str("status = ?, ");
            query_params.push(status.to_string());
        }
        if let Some(is_admin) = user_parameters.is_admin {
            query.push_str("role = ?, ");
            query_params.push(is_admin.to_string());
        }
        query.pop();
        query.pop();

        query.push_str(" WHERE guid = ?");

        log::debug!("query: {:?}", query);
        let mut res = sqlx::query(&query);
        for param in query_params {
            res = res.bind(param);
        }
        let _res = res.bind(user_id).execute(&mut conn).await;

        Some(())
    }

    pub async fn  get_all_peers(&self) -> Option<Vec<Peer>> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query!(
            r#"
            SELECT
                guid,
                id,
                uuid,
                status,
                info as "info!: String",
                last_online  as "last_online!: String"
            FROM
                peer
        "#
        )
        .fetch_all(&mut conn)
        .await
        .ok()?;

        let mut peers:Vec<Peer> = Vec::new();
        for row in res {
            let uuid = guid_into_uuid(row.guid).unwrap_or("".to_string());
            let peer_info = serde_json::from_str::<utils::PeerInfo>(&row.info);
            if peer_info.is_err() {
                log::error!("get_all_peers error: {:?}", peer_info);
                return None;
            }
            let peer_info = peer_info.unwrap();
            peers.push(Peer{
                id: row.id,
                guid: uuid,
                info: peer_info,
                last_online: row.last_online.into(),
                status: row.status as i32,
                strategy_name: "-".to_string(),
            });
        }
        Some(peers)
    }

    pub async fn get_groups(&self, offset: u32, page_size: u32)->Option<Vec<Group>>{
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query!(
            r#"
            SELECT
                guid,
                team,
                name,
                note,
                created_at as "created_at!: String",
                info as "info!: String"
            FROM
                grp
            LIMIT ?
            OFFSET ?
        "#,
            page_size,
            offset
        )
        .fetch_all(&mut conn)
        .await
        .ok()?;
        let mut groups:Vec<Group> = Vec::new();
        for row in res {
            let guid = guid_into_uuid(row.guid).unwrap_or("".to_string());
            let team  = guid_into_uuid(row.team).unwrap_or("".to_string());
            groups.push(Group{
                guid: guid,
                name: row.name,
                team: team,
                note: row.note,
                created_at: row.created_at.into(),
                access_to: Vec::<String>::new(),
                accessed_from: Vec::<String>::new(),
                info: row.info
            });
        }
        Some(groups)
    
    }
}

fn guid_into_uuid(guid: Vec<u8>) -> Option<String> {
    let guid_u8: Result<[u8; 16], _> = guid.try_into();
    if guid_u8.is_err() {
        log::error!("get_ab_personal_guid error: {:?}", guid_u8);
        return None;
    }
    let guid_u8: [u8; 16] = guid_u8.unwrap();
    let guid = Uuid::from_bytes(guid_u8).to_string();
    Some(guid)
}
