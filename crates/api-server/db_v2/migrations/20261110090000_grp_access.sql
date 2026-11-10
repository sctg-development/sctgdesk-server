-- Which groups a group may connect to: a row (src, dst) means that the users of `src` may see and reach the users
-- and the devices of `dst`. The `allowed_outgoings` of a group are its rows as `src`, its `allowed_incomings` are its
-- rows as `dst`. A group always reaches itself.
CREATE TABLE IF NOT EXISTS grp_access (
    src blob not null,
    dst blob not null,
    primary key (src, dst)
) without rowid;
CREATE INDEX IF NOT EXISTS index_grp_access_dst ON grp_access (dst);
