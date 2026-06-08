create table if not exists profile_creation_request (
    profile_creation_request_id text primary key,
    user_id text not null references "user" (user_id) on delete cascade,
    requested_profile_data text not null,
    status text not null check (status in ('sent', 'confirmed', 'rejected')) default 'sent',
    reviewer_note text,
    reviewed_at text,
    created_at text not null default current_timestamp,
    updated_at text not null default current_timestamp
);

create index if not exists profile_creation_request_user_id_idx on profile_creation_request (user_id, created_at desc);
create index if not exists profile_creation_request_status_idx on profile_creation_request (status, created_at desc);
