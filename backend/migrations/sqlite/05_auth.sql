create table if not exists auth_user (
    auth_user_id text primary key,
    user_id text not null unique references "user" (user_id) on delete cascade,
    login_identifier text unique not null collate nocase,
    password_hash text,
    created_at text not null default current_timestamp,
    updated_at text not null default current_timestamp
);

create index if not exists auth_user_login_identifier_idx on auth_user (login_identifier);
