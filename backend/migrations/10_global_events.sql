-- "Smite" (Global Events) System
create table if not exists global_event (
    global_event_id uuid primary key default uuid_generate_v1mc(),
    event_type text not null check (event_type in ('GLOBAL_TARGET', 'NIGHT_HUNT', 'SMIITE')),
    trigger_id uuid not null references "user" (user_id) on delete set null,
    target_id uuid references "user" (user_id) on delete set null,
    start_time timestamptz not null,
    end_time timestamptz not null,
    payload jsonb not null default '{}'::jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('global_event');

create index if not exists global_event_start_time_idx on global_event (start_time);
create index if not exists global_event_end_time_idx on global_event (end_time);
create index if not exists global_event_target_id_idx on global_event (target_id);
