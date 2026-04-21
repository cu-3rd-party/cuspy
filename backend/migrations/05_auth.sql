create table if not exists auth_user (
    auth_user_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null unique references "user" (user_id) on delete cascade,
    email text collate case_insensitive unique not null,
    password_hash text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('auth_user');

create index if not exists auth_user_email_idx on auth_user (email);
