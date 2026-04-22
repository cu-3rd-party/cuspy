-- Loot & Chests System
create table if not exists chest_type (
    chest_type_id uuid primary key default uuid_generate_v1mc(),
    slug text unique not null collate case_insensitive,
    rarity text not null,
    base_drop_rate float not null check (base_drop_rate >= 0 and base_drop_rate <= 1),
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('chest_type');

create table if not exists item (
    item_id uuid primary key default uuid_generate_v1mc(),
    slug text unique not null collate case_insensitive,
    item_type text not null check (item_type in ('GOLD', 'PERK', 'COSMETIC')),
    value jsonb not null default '{}'::jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('item');

create table if not exists loot_table (
    chest_type_id uuid not null references chest_type (chest_type_id) on delete cascade,
    item_id uuid not null references item (item_id) on delete cascade,
    chance float not null check (chance >= 0 and chance <= 1),
    primary key (chest_type_id, item_id)
);

create table if not exists agent_inventory (
    inventory_id uuid primary key default uuid_generate_v1mc(),
    agent_id uuid not null references "user" (user_id) on delete cascade,
    item_id uuid not null references item (item_id) on delete cascade,
    quantity bigint not null check (quantity >= 0) default 1,
    acquired_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('agent_inventory');

create index if not exists agent_inventory_agent_id_idx on agent_inventory (agent_id);
create index if not exists agent_inventory_item_id_idx on agent_inventory (item_id);
