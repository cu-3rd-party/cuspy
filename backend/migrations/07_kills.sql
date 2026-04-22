-- State machine for kill events: Reported -> Victim Confirmed -> Admin Approved
create table if not exists kill_event (
    kill_event_id uuid primary key default uuid_generate_v1mc(),
    killer_id uuid not null references "user" (user_id) on delete cascade,
    victim_id uuid not null references "user" (user_id) on delete cascade,
    status text not null check (status in ('REPORTED', 'VICTIM_CONFIRMED', 'ADMIN_APPROVED', 'REJECTED')) default 'REPORTED',
    evidence_url text,
    reported_at timestamptz not null default now(),
    confirmed_at timestamptz,
    moderated_at timestamptz,
    moderator_id uuid references "user" (user_id) on delete set null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('kill_event');

create index if not exists kill_event_killer_id_idx on kill_event (killer_id);
create index if not exists kill_event_victim_id_idx on kill_event (victim_id);
create index if not exists kill_event_status_idx on kill_event (status);
create index if not exists kill_event_created_at_idx on kill_event (created_at desc);
