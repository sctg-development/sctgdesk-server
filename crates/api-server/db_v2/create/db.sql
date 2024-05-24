PRAGMA foreign_keys=OFF;
CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20230201012014,'init','2024-04-28 15:32:32',1,X'7f59f878ac7838fb3a8bd0cb166963afeeb1cb84ca3bc3db9e29820a16c931b76b68c533d83327e494ee95ca6d4f4212',58536330);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20230720124454,'peer','2024-04-28 15:32:33',1,X'21861731df5ab27e2cee40f3ecec6eeb1b0e5a1a8578a8e932fbba0d6e95789dc5f94990a534ec5ca132c737fe0d5614',21359792);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20230803132453,'strategy','2024-04-28 15:32:33',1,X'ebb715a78f10f8e78ac59232a09a243274e946709b807245f5e5b5dc5aeb50d2104946eee92283f0dd52c5a081016fee',7428080);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20230921031721,'audit alarm','2024-04-28 15:32:33',1,X'647032494e9c1824e46515f120d1b5558c4243e571418b525a755d5f4c6802a0e42fbf969d455368e07fbffe03d609ca',9519685);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20231221032102,'group info','2024-04-28 15:32:33',1,X'f7a3a414482189c1f683148cdfc199d5a97de222055d06e4835d6c44d5ca32f9228bddcb8e1e0baa1383480df038475b',5082425);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20231226082926,'user auth2fa','2024-04-28 15:32:33',1,X'9d0bc99dafce9518736571ba978f224ad04a8f2b233b0ad1021053d0e4cc3964f37455affbf29f10343d1bf27fa6221e',5792826);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20240110130211,'third auth remove credential','2024-04-28 15:32:33',1,X'3474c8e8d342b626be38c12680590113871ccae94eb493ccc12b53c98789b7f53f6fa424ca19c7e7120031f5554b3a4c',6146984);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20240130142656,'user third auth identifier lowercase','2024-04-28 15:32:33',1,X'331c4e620c15973356896362090cf12b6b86e32e85d7b91b9d5c78c8a6a646f04b55b5f27ebe8b15ca5df9926056cdea',5773415);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20240206101908,'third auth identifier recover if not ldap','2024-04-28 15:32:33',1,X'f368f38babc0c1db84ae69b27386737e36fdd4c55601b34d331943926abe54a7ed5a19d4a98160a215ea4971726118a2',2930167);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20240229024003,'custom client','2024-04-28 15:32:33',1,X'65155da9cb73ed6be4fd4c774262fedcee335c8bb3f0afe7e01e472185e6156d85167005140b40e2e8dec62daac94edf',10640401);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20240312015505,'ab','2024-04-28 15:32:33',1,X'259e8610332b04cdb6d4079a89c7bf84031a7d5e1a28698f6c389498083cdb6e6eede66a332db944576194a6550fa09e',20373558);
-- INSERT OR IGNORE INTO _sqlx_migrations VALUES(20240410090154,'add last online','2024-04-28 15:32:33',1,X'304b6f42a9aa273b01b230d8d6b79e95bfe996cfc11caac5bc63bf75a2d73892e68a66b6b9d4391ee6007262b087d0f8',8618959);
CREATE TABLE IF NOT EXISTS "ab_legacy" (
                "user_guid"	BLOB NOT NULL,
                "ab"	JSON NOT NULL,
                FOREIGN KEY("user_guid") REFERENCES "user"("guid"),
                PRIMARY KEY("user_guid")
            );
CREATE TABLE IF NOT EXISTS team (
                                    guid blob primary key not null,
                                    name varchar(100) not null,
                                    email varchar(100) not null,
                                    note varchar(300),
                                    info text not null,
                                    created_at datetime not null default(current_timestamp),
                                    updated_at datetime not null default(current_timestamp)
) without rowid;
INSERT OR IGNORE INTO team VALUES(X'018f255622f77778a006702ca5c23714','Default','Default',NULL,'{}','2024-04-28 15:32:33','2024-04-28 15:32:33');
CREATE TABLE IF NOT EXISTS session (
                                    "id" VARCHAR(100) PRIMARY KEY,
                                    "ttl_secs" INTEGER NOT NULL,
                                    "user" blob not null,
                                    info text,
                                    expiry_at datetime not null,
                                    created_at datetime not null default(current_timestamp)
) without rowid;
INSERT OR IGNORE INTO session VALUES('YFMotxzHT7qxoorhyNy/bA==',2592000,X'018f2556230179eb91a2cffe5ced4236','{"ip":"::ffff:192.168.65.1","device_uuid":"RTlENEQxQ0UtMkY5Mi01ODg2LUE4QzEtMkQ4QjRFOEMwNDUz","os":"macos","type":"client","name":"blingster"}','2024-05-28 15:32:52','2024-04-28 15:32:52');
CREATE TABLE IF NOT EXISTS peer (
                                    guid blob primary key not null,
                                    id varchar(100) not null,
                                    uuid blob not null,
                                    pk blob not null,
                                    created_at datetime not null default(current_timestamp),
                                    "user" blob,
                                    status tinyint not null default(1),
                                    note varchar(300),
                                    region text null,
                                    strategy blob, 
                                    info JSON not null DEFAULT '{}', "last_online" datetime not null default('2011-11-16 11:55:19')) without rowid;
CREATE TABLE IF NOT EXISTS "user" (
                                    guid blob primary key not null,
                                    name varchar(100) not null,
                                    email varchar(100),
                                    phone char(30),
                                    password varchar(100) not null,
                                    note varchar(300),
                                    status tinyint not null,
                                    grp blob not null,
                                    created_at datetime not null default(current_timestamp),
                                    team blob not null,
                                    role tinyint not null, 
                                    info JSON not null DEFAULT '{}', strategy blob, "tfa" blob null) without rowid;
-- $2b$12$.SfrHeV5frv0FQTM8X33OerLMu1YthqJdMP1oqpFUGh7dfgr41iGG = bcrypt('Hello,world!')
INSERT OR IGNORE INTO user VALUES(X'018f2556230179eb91a2cffe5ced4236','admin',NULL,NULL,'$2b$12$.SfrHeV5frv0FQTM8X33OerLMu1YthqJdMP1oqpFUGh7dfgr41iGG','',1,X'018f255622fb73ee9afdbbcdc0cc387b','2024-04-28 15:32:33',X'018f255622f77778a006702ca5c23714',1,'{"email_alarm_notification":true}',NULL,NULL);
CREATE TABLE IF NOT EXISTS grp (
                                    guid blob primary key not null,
                                    team blob not null,
                                    name varchar(100) not null,
                                    note varchar(300),
                                    created_at datetime not null default(current_timestamp), 
                                    "info" text not null default '{}') without rowid;
INSERT OR IGNORE INTO grp VALUES(X'018f255622fb73ee9afdbbcdc0cc387b',X'018f255622f77778a006702ca5c23714','Default',NULL,'2024-04-28 15:32:33','{}');
CREATE TABLE IF NOT EXISTS user_data (
                                    "user" blob not null,
                                    type varchar(30) not null,
                                    data text not null,
                                    updated_at datetime not null,
                                    primary key("user", type)
) without rowid;
CREATE TABLE IF NOT EXISTS audit_conn (
                                    guid blob primary key not null,
                                    type tinyint,
                                    remote blob not null,
                                    local blob,
                                    created_at datetime not null default(strftime('%Y-%m-%d %H:%M:%f', 'now')),
                                    end_time datetime,
                                    note text,
                                    info text not null
);
CREATE TABLE IF NOT EXISTS audit_file (
                                    guid blob primary key not null,
                                    remote blob not null,
                                    local blob,
                                    created_at datetime not null default(strftime('%Y-%m-%d %H:%M:%f', 'now')),
                                    type tinyint not null,
                                    path text not null,
                                    is_file tinyint not null,
                                    info text not null
);
CREATE TABLE IF NOT EXISTS audit_alarm (
                                    guid blob primary key not null,
                                    type tinyint not null,
                                    created_at datetime not null default(strftime('%Y-%m-%d %H:%M:%f', 'now')),
                                    info text not null, 
                                    user blob DEFAULT NULL, device blob DEFAULT NULL);
CREATE TABLE IF NOT EXISTS audit_console (
                                    guid blob primary key not null,
                                    type tinyint not null,
                                    created_at datetime not null default(strftime('%Y-%m-%d %H:%M:%f', 'now')),
                                    operator blob not null,
                                    iop smallint not null,
                                    info text not null
);
CREATE TABLE IF NOT EXISTS user_third_auth (
                                    "user" blob not null,
                                    type varchar(30) not null,
                                    identifier varchar(255) not null,
                                    expiry timestamp, 
                                    info text not null,
                                    created_at datetime not null default(current_timestamp),
                                    updated_at datetime not null, "identifier_bak" varchar(255) not null default '',
                                    primary key("user", type)
) without rowid;
CREATE TABLE IF NOT EXISTS strategy (
                                    guid blob primary key not null,
                                    team blob not null,
                                    name varchar(100) not null,
                                    created_at datetime not null default(current_timestamp),
                                    modified_at datetime not null default(current_timestamp),
                                    options text not null default '',
                                    status tinyint not null default 0
) without rowid;
INSERT OR IGNORE INTO strategy VALUES(X'018f255623167a02b31c5599e7cd5b5e',X'018f255622f77778a006702ca5c23714','Default','2024-04-28 15:32:33','2024-04-28 15:32:33','{}',0);
CREATE TABLE IF NOT EXISTS cross_grp (
    incoming blob not null,
    outgoing blob not null,
    created_at datetime not null default(current_timestamp),
    CONSTRAINT constraint_incoming_outgoing PRIMARY KEY (incoming, outgoing)
) without rowid;
CREATE TABLE IF NOT EXISTS settings (
    key varchar(100) primary key not null,
    value text not null
) without rowid;
CREATE TABLE IF NOT EXISTS "custom_client" (
                                    "guid" blob primary key not null,
                                    "team" blob not null,
                                    "name" varchar(100) not null,
                                    "os" tinyint not null,  -- 1: windows, 2: mac, 3: linux, 4: android, 5: ios
                                    "info" JSON not null,
                                    "status" JSON,
                                    "note" varchar(300),
                                    "created_at" integer not null default(current_timestamp)
);
CREATE TABLE IF NOT EXISTS ab (
                                    guid blob primary key not null,
                                    name varchar(100) not null,
                                    owner blob not null,
                                    personal tinyint not null default 0,
                                    note varchar(300),
                                    created_at datetime not null default(current_timestamp),
                                    info text not null
) without rowid;
INSERT OR IGNORE INTO ab VALUES(X'018f255623117efa9d25470a9160c6d5','admin''s Personal Address Book',X'018f2556230179eb91a2cffe5ced4236',1,NULL,'2024-04-28 15:32:33','{}');
INSERT OR IGNORE INTO ab VALUES(X'018f255623117efa9d25470a9160c6d7','Default''s Shared Address Book',X'018f2556230179eb91a2cffe5ced4236',0,NULL,'2024-04-28 15:32:33','{}');
CREATE TABLE IF NOT EXISTS ab_peer (
                                    guid blob primary key not null,
                                    ab blob not null,
                                    peer blob,
                                    id text,
                                    note varchar(300),
                                    created_at datetime not null default(current_timestamp),
                                    deleted_at datetime,
                                    info text not null
) without rowid;
CREATE TABLE IF NOT EXISTS ab_tag (
                                    ab blob not null,
                                    name varchar(100) not null,
                                    color INTEGER not null,
                                    CONSTRAINT constraint_ab_name PRIMARY KEY (ab, name)
) without rowid;
CREATE TABLE IF NOT EXISTS ab_rule (
                                    guid blob primary key not null,
                                    ab blob not null,
                                    "user" blob,
                                    grp blob,
                                    rule tinyint not null,
                                    created_at datetime not null default(current_timestamp)
) without rowid;
INSERT OR IGNORE INTO ab_rule VALUES(X'018f255623117efa9d25470a9160c6d9',X'018f255623117efa9d25470a9160c6d7',NULL,X'018f255622fb73ee9afdbbcdc0cc387b',3,current_timestamp);
CREATE UNIQUE INDEX IF NOT EXISTS index_team_name on team (name);
CREATE INDEX IF NOT EXISTS index_session_user on session ("user");
CREATE INDEX IF NOT EXISTS index_session_expiry_at on session (expiry_at);
CREATE UNIQUE INDEX IF NOT EXISTS index_peer_id on peer (id);
CREATE INDEX IF NOT EXISTS index_peer_user on peer ("user");
CREATE INDEX IF NOT EXISTS index_peer_created_at on peer (created_at);
CREATE INDEX IF NOT EXISTS index_peer_status on peer (status);
CREATE UNIQUE INDEX IF NOT EXISTS index_user_name on "user" (name);
CREATE UNIQUE INDEX IF NOT EXISTS index_user_email on "user" (email);
CREATE INDEX IF NOT EXISTS index_user_group on "user" (grp);
CREATE INDEX IF NOT EXISTS index_user_team on "user" (team);
CREATE INDEX IF NOT EXISTS index_user_created_at on "user" (created_at);
CREATE INDEX IF NOT EXISTS index_user_status on "user" (status);
CREATE UNIQUE INDEX IF NOT EXISTS index_grp_name on grp (name);
CREATE INDEX IF NOT EXISTS index_grp_team on grp (team);
CREATE INDEX IF NOT EXISTS index_grp_created_at on grp (created_at);
CREATE INDEX IF NOT EXISTS index_user_data_user on user_data ("user");
CREATE INDEX IF NOT EXISTS index_user_data_type on user_data (type);
CREATE INDEX IF NOT EXISTS index_audit_conn_created_at on audit_conn (created_at);
CREATE INDEX IF NOT EXISTS index_audit_conn_remote on audit_conn (remote);
CREATE INDEX IF NOT EXISTS index_audit_conn_type on audit_conn (type);
CREATE INDEX IF NOT EXISTS index_audit_file_created_at on audit_file (created_at);
CREATE INDEX IF NOT EXISTS index_audit_file_remote on audit_file (remote);
CREATE INDEX IF NOT EXISTS index_audit_file_type on audit_file (type);
CREATE INDEX IF NOT EXISTS index_audit_alarm_created_at on audit_alarm (created_at);
CREATE INDEX IF NOT EXISTS index_audit_alarm_type on audit_alarm (type);
CREATE INDEX IF NOT EXISTS index_audit_console_created_at on audit_console (created_at);
CREATE INDEX IF NOT EXISTS index_audit_console_operator on audit_console (operator);
CREATE INDEX IF NOT EXISTS index_audit_console_type on audit_console (type);
CREATE UNIQUE INDEX IF NOT EXISTS uniq_user_third_auth_type_identifer on user_third_auth (type, identifier);
CREATE INDEX IF NOT EXISTS index_strategy_name on strategy (name);
CREATE INDEX IF NOT EXISTS index_strategy_team on strategy (team);
CREATE INDEX IF NOT EXISTS index_cross_grp_incoming on cross_grp (incoming);
CREATE INDEX IF NOT EXISTS index_cross_grp_outgoing on cross_grp (outgoing);
CREATE INDEX IF NOT EXISTS index_user_strategy ON user (strategy);
CREATE INDEX IF NOT EXISTS index_peer_strategy ON peer (strategy);
CREATE INDEX IF NOT EXISTS index_audit_alarm_user ON audit_alarm (user);
CREATE INDEX IF NOT EXISTS index_audit_alarm_device ON audit_alarm (device);
CREATE UNIQUE INDEX IF NOT EXISTS "uniq_custom_client_name" on "custom_client" ("team", "name");
CREATE INDEX IF NOT EXISTS "index_custom_client_created_at" on "custom_client" ("created_at");
CREATE UNIQUE INDEX IF NOT EXISTS index_ab_name on ab (name);
CREATE INDEX IF NOT EXISTS index_ab_owner on ab (owner);
CREATE INDEX IF NOT EXISTS index_ab_personal_created_at on ab (personal, created_at);
CREATE UNIQUE INDEX IF NOT EXISTS index_ab_peer_ab_peer on ab_peer (ab, peer);
CREATE UNIQUE INDEX IF NOT EXISTS index_ab_peer_ab_id on ab_peer (ab, id);
CREATE INDEX IF NOT EXISTS index_ab_peer_peer on ab_peer (peer);
CREATE INDEX IF NOT EXISTS index_ab_peer_ab_created_at on ab_peer (ab, created_at);
CREATE INDEX IF NOT EXISTS index_ab_peer_ab_deleted_at on ab_peer (ab, deleted_at);
CREATE INDEX IF NOT EXISTS index_ab_rule_user on ab_rule ("user");
CREATE INDEX IF NOT EXISTS index_ab_rule_grp on ab_rule (grp);
CREATE INDEX IF NOT EXISTS index_ab_rule_ab_created_at on ab_rule (ab, created_at);
