create table if not exists kill_event (
    kill_event_id text primary key,
    killer_id text not null references "user" (user_id) on delete cascade,
    victim_id text not null references "user" (user_id) on delete cascade,
    status text not null check (status in ('REPORTED', 'VICTIM_CONFIRMED', 'ADMIN_APPROVED', 'REJECTED')) default 'REPORTED',
    evidence_url text,
    reported_at text not null default current_timestamp,
    confirmed_at text,
    moderated_at text,
    moderator_id text references "user" (user_id) on delete set null,
    created_at text not null default current_timestamp,
    updated_at text
);

create index if not exists kill_event_killer_id_idx on kill_event (killer_id);
create index if not exists kill_event_victim_id_idx on kill_event (victim_id);
create index if not exists kill_event_status_idx on kill_event (status);
create index if not exists kill_event_created_at_idx on kill_event (created_at desc);
