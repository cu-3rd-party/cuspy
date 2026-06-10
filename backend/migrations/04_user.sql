
create table if not exists "user" (
    user_id uuid primary key default uuid_generate_v1mc(),
    telegram_id bigint unique,
    agent_name text collate case_insensitive,
    agent_data_id uuid,
    foreign key (agent_data_id) references "agent_data"("agent_data_id"),
    rating bigint not null default 0,
    is_admin boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('"user"');

create index if not exists user_agent_name_idx on "user" (agent_name);
create index if not exists user_is_admin_idx on "user" (is_admin);
