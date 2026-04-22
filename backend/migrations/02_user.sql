create table if not exists "user" (
    user_id uuid primary key default uuid_generate_v1mc(),
    telegram_id bigint unique not null,
    agent_name text collate case_insensitive,
    agent_data jsonb not null default '{}'::jsonb,
    is_admin boolean not null default false,
    is_sharing_location boolean not null default false,
    current_location jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('"user"');

create index if not exists user_agent_name_idx on "user" (agent_name);
create index if not exists user_is_admin_idx on "user" (is_admin);
