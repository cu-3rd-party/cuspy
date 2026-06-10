-- State machine for kill events: Reported -> Victim Confirmed -> Admin Approved
create table if not exists kill_event (
    kill_event_id uuid primary key default uuid_generate_v1mc(),
    killer_id uuid not null references "user" (user_id) on delete cascade,
    victim_id uuid not null references "user" (user_id) on delete cascade,
    -- PENDING - никто еще ничего не подтверждал
    -- REPORTED - один из людей сказал что это случилось
    -- CONFIRMED - другой человек тоже сказал что это случилось
    -- ADMIN_APPROVED - админ еще и подтвердил что это случилось
    -- REJECTED - чето пошло не так и убийство отменено
    status text not null check (status in ('PENDING', 'REPORTED', 'CONFIRMED', 'ADMIN_APPROVED', 'REJECTED')) default 'PENDING',
    evidence_resource_id uuid,
    foreign key (evidence_resource_id) references "resource"(resource_id),
    details jsonb not null default '{}'::jsonb,
    killer_confirmed_at timestamptz,
    victim_confirmed_at timestamptz,
    confirmed_at timestamptz,
    moderated_at timestamptz,
    moderator_id uuid references "user" (user_id) on delete set null,
    moderation_reason text,
    rating_applied_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    check ( killer_id <> victim_id )
);

select trigger_updated_at('kill_event');

create index if not exists kill_event_killer_id_idx on kill_event (killer_id);
create index if not exists kill_event_victim_id_idx on kill_event (victim_id);
create index if not exists kill_event_status_idx on kill_event (status);
create index if not exists kill_event_created_at_idx on kill_event (created_at desc);
