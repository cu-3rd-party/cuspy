create table if not exists "user" (
    user_id uuid primary key default uuid_generate_v1mc(),
    telegram_id bigint unique not null,
    rating bigint,
    agent_name text collate case_insensitive,
    agent_data jsonb not null default '{}'::jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('"user"');

create index if not exists user_agent_name_idx on "user" (agent_name);
