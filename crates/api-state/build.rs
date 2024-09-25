// Copyright (c) 2024 Ronan LE MEILLAT for SCTG Development
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
use sqlx::{Connection, Executor, SqliteConnection};
use std::env;


#[tokio::main]
async fn main() {
    let db_path = env::var("DATABASE_URL").unwrap_or("sqlite://db_v2.sqlite3".to_string());
    let mut conn = SqliteConnection::connect(&format!("{}", db_path))
        .await
        .expect("Failed to open database");
    conn.execute(
        r#"
        INSERT OR IGNORE INTO peer (guid, id, uuid, pk, created_at, "user", status, note, region, strategy, info, last_online) VALUES
        (x'95CC7775BA37481DAD7214A4F6CE5A94', 'TESTUSER', randomblob(16), randomblob(16), '1901-01-01 12:00:00', randomblob(16), 0, '', NULL, randomblob(16), '{}', '1901-01-01 12:00:00');
        "#
    )
    .await
    .expect("Failed to insert test data");

}
