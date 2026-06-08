create table if not exists "user" (
    user_id text primary key,
    telegram_id integer unique not null,
    agent_name text collate nocase,
    agent_data text not null default '{}',
    is_admin integer null default 0,
    is_sharing_location integer not null default 0,
    current_location text,
    created_at text not null default current_timestamp,
    updated_at text
);

create index if not exists user_agent_name_idx on "user" (agent_name);
create index if not exists user_is_admin_idx on "user" (is_admin);
