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

//! The history of the connections to the devices, and the notes left on them.
//!
//! The protocol is the one of the RustDesk clients (1.5): the controlled device posts `new`, the authentication
//! (who controls, and how) and `close` to `/api/audit/conn`; the controlling client asks for the guid of the record
//! (`/api/audit/conn/active`) and then sets the note by guid (`PUT /api/audit`), or posts it with the session
//! (`POST /api/audit/conn`).
use crate::state::ApiState;
use crate::UserId;
use sqlx::Row;
use utils::{guid_into_uuid, AuditConnEvent, AuditConnRecord};
use uuid::Uuid;

/// The longest note kept.
const MAX_NOTE: usize = 4000;

fn clip(note: &str) -> String {
    note.chars().take(MAX_NOTE).collect()
}

impl ApiState {
    /// Applies an event posted by a device. Events of unknown devices are dropped: the post is not authenticated.
    pub async fn audit_conn_event(&self, ev: &AuditConnEvent) {
        let db = &self.db;
        let Some(device_id) = ev.id.as_deref().filter(|id| !id.is_empty()) else {
            return;
        };
        let Ok(mut conn) = db.pool.acquire().await else {
            return;
        };
        let known = sqlx::query("SELECT 1 FROM peer WHERE id = ? LIMIT 1")
            .bind(device_id)
            .fetch_optional(&mut *conn)
            .await;
        if !matches!(known, Ok(Some(_))) {
            log::debug!("audit: unknown device {:?}", device_id);
            return;
        }
        if let Some(nonce) = ev.nonce.as_deref().filter(|n| !n.is_empty()) {
            let res = sqlx::query("INSERT OR IGNORE INTO conn_history_nonce (nonce) VALUES (?)")
                .bind(nonce)
                .execute(&mut *conn)
                .await;
            match res {
                Ok(done) if done.rows_affected() == 0 => return, // a retry
                Err(e) => {
                    log::error!("audit nonce error: {:?}", e);
                    return;
                }
                _ => {}
            }
            let _ =
                sqlx::query("DELETE FROM conn_history_nonce WHERE at < datetime('now', '-2 days')")
                    .execute(&mut *conn)
                    .await;
        }
        let session_id = ev.session_id.map(|s| s.to_string());
        match (ev.conn_id, session_id.as_deref()) {
            (Some(conn_id), Some(session_id)) => {
                let _ = sqlx::query(
                    "INSERT OR IGNORE INTO conn_history (guid, device_id, conn_id, session_id) VALUES (?, ?, ?, ?)",
                )
                .bind(Uuid::new_v4().as_bytes().to_vec())
                .bind(device_id)
                .bind(conn_id)
                .bind(session_id)
                .execute(&mut *conn)
                .await;
                let key = "device_id = ? AND conn_id = ? AND session_id = ?";
                if let Some(ip) = ev.ip.as_deref().filter(|ip| !ip.is_empty()) {
                    let _ = sqlx::query(&format!(
                        "UPDATE conn_history SET ip = ? WHERE {key} AND (ip IS NULL OR ip = '')"
                    ))
                    .bind(ip)
                    .bind(device_id)
                    .bind(conn_id)
                    .bind(session_id)
                    .execute(&mut *conn)
                    .await;
                }
                if let Some([id, name, ..]) = ev.peer.as_deref() {
                    let _ = sqlx::query(&format!(
                        "UPDATE conn_history SET controller_id = ?, controller_name = ?, conn_type = ? WHERE {key}"
                    ))
                    .bind(id)
                    .bind(name)
                    .bind(ev.conn_type.unwrap_or(0))
                    .bind(device_id)
                    .bind(conn_id)
                    .bind(session_id)
                    .execute(&mut *conn)
                    .await;
                }
                if ev.action.as_deref() == Some("close") {
                    let _ = sqlx::query(&format!(
                        "UPDATE conn_history SET ended_at = current_timestamp WHERE {key} AND ended_at IS NULL"
                    ))
                    .bind(device_id)
                    .bind(conn_id)
                    .bind(session_id)
                    .execute(&mut *conn)
                    .await;
                }
            }
            (None, Some(session_id)) => {
                // the controlling client posts its note with the session
                if let Some(note) = ev.note.as_deref() {
                    let _ = sqlx::query(
                        r#"UPDATE conn_history SET note = ? WHERE guid = (
                             SELECT guid FROM conn_history WHERE device_id = ? AND session_id = ?
                             ORDER BY started_at DESC LIMIT 1)"#,
                    )
                    .bind(clip(note))
                    .bind(device_id)
                    .bind(session_id)
                    .execute(&mut *conn)
                    .await;
                }
            }
            _ => {}
        }
    }

    /// The guid of the record of a session, for the client that controls it; the record is then the user's.
    /// `None` if there is none yet (the device may not have posted), or if it is another user's.
    pub async fn audit_conn_active_guid(
        &self,
        user_id: &UserId,
        device_id: &str,
        session_id: &str,
    ) -> Option<String> {
        let mut conn = self.db.pool.acquire().await.ok()?;
        let row = sqlx::query(
            r#"SELECT guid, "user" FROM conn_history WHERE device_id = ? AND session_id = ?
               ORDER BY started_at DESC LIMIT 1"#,
        )
        .bind(device_id)
        .bind(session_id)
        .fetch_optional(&mut *conn)
        .await
        .ok()??;
        let guid: Vec<u8> = row.get("guid");
        let owner: Option<Vec<u8>> = row.get("user");
        match owner {
            Some(owner) if &owner != user_id => return None,
            Some(_) => {}
            None => {
                let _ = sqlx::query(r#"UPDATE conn_history SET "user" = ? WHERE guid = ?"#)
                    .bind(user_id)
                    .bind(&guid)
                    .execute(&mut *conn)
                    .await;
            }
        }
        guid_into_uuid(guid)
    }

    /// Sets the note of a record: its user may, and an administrator.
    pub async fn audit_conn_set_note(
        &self,
        guid: &str,
        note: &str,
        user_id: &UserId,
        is_admin: bool,
    ) -> bool {
        let Some(guid) = utils::uuid_into_guid(guid) else {
            return false;
        };
        let Ok(mut conn) = self.db.pool.acquire().await else {
            return false;
        };
        sqlx::query(r#"UPDATE conn_history SET note = ? WHERE guid = ? AND (? OR "user" = ?)"#)
            .bind(clip(note))
            .bind(guid)
            .bind(is_admin)
            .bind(user_id)
            .execute(&mut *conn)
            .await
            .map(|done| done.rows_affected() > 0)
            .unwrap_or(false)
    }

    /// The history, newest first. `only_user` limits it to the connections of an account (all if `None`).
    /// `q` is searched in the devices, the controller, the account, the address and the note; `from` and `to`
    /// are days (`YYYY-MM-DD`), both included.
    pub async fn audit_conn_list(
        &self,
        only_user: Option<&UserId>,
        q: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
        offset: u32,
        limit: u32,
    ) -> (u32, Vec<AuditConnRecord>) {
        let filter = r#"
            WHERE (?1 IS NULL OR a."user" = ?1)
              AND (?2 IS NULL OR a.device_id LIKE ?2 OR a.controller_id LIKE ?2 OR a.controller_name LIKE ?2
                   OR a.ip LIKE ?2 OR a.note LIKE ?2
                   OR (SELECT name FROM "user" WHERE guid = a."user") LIKE ?2
                   OR (SELECT json_extract(info, '$.hostname') FROM peer WHERE id = a.device_id LIMIT 1) LIKE ?2)
              AND (?3 IS NULL OR a.started_at >= ?3)
              AND (?4 IS NULL OR a.started_at < date(?4, '+1 day'))"#;
        let q = q.filter(|q| !q.is_empty()).map(|q| format!("%{q}%"));
        let from = from.filter(|d| !d.is_empty());
        let to = to.filter(|d| !d.is_empty());
        let Ok(mut conn) = self.db.pool.acquire().await else {
            return (0, Vec::new());
        };
        let total: i64 =
            sqlx::query_scalar(&format!("SELECT COUNT(*) FROM conn_history a {filter}"))
                .bind(only_user)
                .bind(&q)
                .bind(from)
                .bind(to)
                .fetch_one(&mut *conn)
                .await
                .unwrap_or(0);
        let rows = sqlx::query(&format!(
            r#"SELECT a.guid, a.device_id, a.controller_id, a.controller_name, a.conn_type, a.ip, a.note,
                      a.started_at, a.ended_at,
                      (SELECT name FROM "user" WHERE guid = a."user") AS user_name,
                      (SELECT json_extract(info, '$.hostname') FROM peer WHERE id = a.device_id LIMIT 1) AS device_name
               FROM conn_history a {filter}
               ORDER BY a.started_at DESC, a.guid DESC LIMIT ?5 OFFSET ?6"#
        ))
        .bind(only_user)
        .bind(&q)
        .bind(from)
        .bind(to)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&mut *conn)
        .await;
        let records = match rows {
            Ok(rows) => rows
                .iter()
                .map(|row| AuditConnRecord {
                    guid: guid_into_uuid(row.get::<Vec<u8>, _>("guid")).unwrap_or_default(),
                    device_id: row.get("device_id"),
                    device_name: row.get("device_name"),
                    controller_id: row.get("controller_id"),
                    controller_name: row.get("controller_name"),
                    user: row.get("user_name"),
                    conn_type: row.get::<i64, _>("conn_type") as i32,
                    ip: row.get("ip"),
                    note: row.get("note"),
                    started_at: row.get("started_at"),
                    ended_at: row.get("ended_at"),
                })
                .collect(),
            Err(e) => {
                log::error!("audit_conn_list error: {:?}", e);
                Vec::new()
            }
        };
        (total as u32, records)
    }
}
