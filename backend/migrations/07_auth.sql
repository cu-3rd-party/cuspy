create table if not exists auth_user
(
    auth_user_id     uuid primary key                     default uuid_generate_v1mc(),
    user_id          uuid                                 references "user" (user_id) on delete cascade,
    telegram_id      bigint unique,
    email            text unique,
    password_hash    text,
    created_at       timestamptz                          not null default now(),
    updated_at       timestamptz                          not null default now()
);

select trigger_updated_at('auth_user');

create index if not exists auth_user_login_identifier_idx on auth_user (telegram_id);
create index if not exists auth_user_login_identifier_idx on auth_user (email);
