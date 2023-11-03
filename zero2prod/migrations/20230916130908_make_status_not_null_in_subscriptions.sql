begin;
    update subscriptions
        set status = 'confirmed'
        where status is null;

    alter table subscriptions alter column status set not null;
commit;
    -- there's no alter column in Sqlite
    -- create table temp_sub (
    --     id text not null,
    --     email text not null unique,
    --     name text not null,
    --     subscribed_at text not null,
    --     status text not null,
    --     primary key (id)
    -- );

    -- insert into temp_sub(id, email, name, subscribed_at, status)
    -- select id, email, name, subscribed_at, status
    -- from subscriptions;

    -- drop table subscriptions;

    -- alter table temp_sub rename to subscriptions;
