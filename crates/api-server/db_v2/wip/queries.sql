-- Get the personal address book  guid of the user
select guid
from ab
where owner = $1
    and personal = 1;

-- Count how many shared address books
select count(*) as count
from ab as a
where a.personal = 0;

-- 
select a.guid,
    a.name,
    u.name as owner,
    a.note,
    3 as rule
from ab as a
    inner join "user" as u on a.owner = u.guid
where a.personal = 0
order by a.created_at desc
limit $1 offset $2;

-- Get the shared address book guid of the user
select name,
    owner as "owner: _",
    personal
from ab
where guid = $1;

-- Get the peers of the address book
SELECT COUNT(*)
FROM ab_peer AS a
    LEFT JOIN peer AS b ON a.peer = b.guid
WHERE a.ab = $1
    AND a.deleted_at IS NULL;

-- Wip
SELECT CASE
        WHEN a.peer IS NOT NULL THEN b.id
        WHEN a.id IS NOT NULL THEN a.id
        ELSE '-'
    END AS id,
    CASE
        WHEN a.peer IS NOT NULL THEN 1
        ELSE 0
    END as same_server,
    a.note,
    a.info as ab_peer_info,
    b.info as peer_info
FROM ab_peer AS a
    LEFT JOIN peer AS b ON a.peer = b.guid
WHERE a.ab = $1
    AND a.deleted_at IS NULL
order by a.created_at desc
limit $2 offset $3;


-- Get the tags of address book
select name,
    color
from ab_tag
where ab = $1