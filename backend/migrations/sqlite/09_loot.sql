create table if not exists chest_type (
    chest_type_id text primary key,
    slug text unique not null collate nocase,
    rarity text not null,
    base_drop_rate real not null check (base_drop_rate >= 0 and base_drop_rate <= 1),
    created_at text not null default current_timestamp,
    updated_at text
);

create table if not exists item (
    item_id text primary key,
    slug text unique not null collate nocase,
    item_type text not null check (item_type in ('GOLD', 'PERK', 'COSMETIC')),
    value text not null default '{}',
    created_at text not null default current_timestamp,
    updated_at text
);

create table if not exists loot_table (
    chest_type_id text not null references chest_type (chest_type_id) on delete cascade,
    item_id text not null references item (item_id) on delete cascade,
    chance real not null check (chance >= 0 and chance <= 1),
    primary key (chest_type_id, item_id)
);

create table if not exists agent_inventory (
    inventory_id text primary key,
    agent_id text not null references "user" (user_id) on delete cascade,
    item_id text not null references item (item_id) on delete cascade,
    quantity integer not null check (quantity >= 0) default 1,
    acquired_at text not null default current_timestamp,
    updated_at text
);

create index if not exists agent_inventory_agent_id_idx on agent_inventory (agent_id);
create index if not exists agent_inventory_item_id_idx on agent_inventory (item_id);
