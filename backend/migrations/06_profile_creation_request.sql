create table if not exists profile_request (
    profile_request_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null references "user" (user_id) on delete cascade,
    requested_profile_data_id uuid not null,
    foreign key (requested_profile_data_id) references "agent_data"(agent_data_id),
    status text not null check (status in ('sent', 'confirmed', 'rejected')) default 'sent',
    reviewer_note text,
    reviewed_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('profile_request');

create index if not exists profile_request_user_id_idx on profile_request (user_id, created_at desc);
create index if not exists profile_request_status_idx on profile_request (status, created_at desc);
