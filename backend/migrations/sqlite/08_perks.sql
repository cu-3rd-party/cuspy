create table if not exists perk_definition (
    perk_id text primary key,
    slug text unique not null collate nocase,
    display_name text not null,
    description text,
    base_duration text,
    config text not null default '{}',
    created_at text not null default current_timestamp,
    updated_at text
);

create table if not exists agent_perk (
    agent_perk_id text primary key,
    agent_id text not null references "user" (user_id) on delete cascade,
    perk_id text not null references perk_definition (perk_id) on delete cascade,
    activated_at text not null default current_timestamp,
    expires_at text,
    instance_metadata text not null default '{}',
    created_at text not null default current_timestamp,
    updated_at text
);

create index if not exists agent_perk_agent_id_idx on agent_perk (agent_id);
create index if not exists agent_perk_perk_id_idx on agent_perk (perk_id);
create index if not exists agent_perk_expires_at_idx on agent_perk (expires_at);
