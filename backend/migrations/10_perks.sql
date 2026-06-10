-- Perks System: allow adding new perks without schema migrations
create table if not exists perk_definition (
    perk_id uuid primary key default uuid_generate_v1mc(),
    slug citext unique not null,
    display_name text not null,
    description text,
    base_duration interval,
    image_resource_id uuid,
    foreign key (image_resource_id) references "resource"(resource_id),
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('perk_definition');

create table if not exists agent_perk (
    agent_perk_id uuid primary key default uuid_generate_v1mc(),
    agent_id uuid not null references "user" (user_id) on delete cascade,
    perk_id uuid not null references perk_definition (perk_id) on delete cascade,
    activated_at timestamptz not null default now(),
    expires_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('agent_perk');

create index if not exists agent_perk_agent_id_idx on agent_perk (agent_id);
create index if not exists agent_perk_perk_id_idx on agent_perk (perk_id);
create index if not exists agent_perk_expires_at_idx on agent_perk (expires_at);
