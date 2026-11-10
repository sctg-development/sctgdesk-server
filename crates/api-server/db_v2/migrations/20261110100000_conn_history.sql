-- The history of the connections made to the devices, as the devices report it (POST /api/audit/conn), with the
-- note a technician leaves on a connection. One row per connection: (device_id, conn_id, session_id).
CREATE TABLE IF NOT EXISTS conn_history (
    guid blob primary key not null,
    device_id text not null,          -- the controlled device
    conn_id integer not null,
    session_id text not null,         -- a u64: kept as text
    controller_id text,               -- the device that controls
    controller_name text,
    conn_type tinyint not null default 0,  -- 0 remote, 1 file transfer, 2 port forward, 3 camera, 4 terminal
    ip text,
    "user" blob,                      -- the account that controlled, known when it asks for the guid of the record
    note text not null default '',
    started_at datetime not null default (current_timestamp),
    ended_at datetime
) without rowid;
CREATE UNIQUE INDEX IF NOT EXISTS index_conn_history_key ON conn_history (device_id, conn_id, session_id);
CREATE INDEX IF NOT EXISTS index_conn_history_started ON conn_history (started_at);
CREATE INDEX IF NOT EXISTS index_conn_history_user ON conn_history ("user");
-- the devices send a nonce with every post and retry: it makes the posts idempotent
CREATE TABLE IF NOT EXISTS conn_history_nonce (
    nonce text primary key not null,
    at datetime not null default (current_timestamp)
) without rowid;
