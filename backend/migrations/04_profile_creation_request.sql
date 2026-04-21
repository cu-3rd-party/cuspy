create table if not exists profile_creation_request (
    profile_creation_request_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null references "user" (user_id) on delete cascade,
    requested_profile_data jsonb not null,
    status text not null check (status in ('sent', 'confirmed', 'rejected')) default 'sent',
    reviewer_note text,
    reviewed_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('profile_creation_request');

create index if not exists profile_creation_request_user_id_idx on profile_creation_request (user_id, created_at desc);
create index if not exists profile_creation_request_status_idx on profile_creation_request (status, created_at desc);
create index if not exists profile_creation_request_requested_profile_data_gin_idx on profile_creation_request using gin (requested_profile_data jsonb_path_ops);
