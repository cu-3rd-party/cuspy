create table if not exists "user"
(
    user_id       uuid primary key     default uuid_generate_v1mc(),
    username    text collate case_insensitive,
    agent_data_id uuid,
    foreign key (agent_data_id) references "agent_data" ("agent_data_id"),
    rating        bigint      not null default 0,
    is_admin      boolean     not null default false,
    created_at    timestamptz not null default now(),
    updated_at    timestamptz
);

select trigger_updated_at('"user"');

create index if not exists user_username_idx on "user" (username);
create index if not exists user_is_admin_idx on "user" (is_admin);
