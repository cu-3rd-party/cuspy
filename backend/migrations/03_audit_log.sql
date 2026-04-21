create extension if not exists pgcrypto;
create extension if not exists btree_gin;
create extension if not exists pg_trgm;

create table if not exists audit_log (
    audit_log_id uuid primary key default uuid_generate_v1mc(),
    request_id uuid not null,
    actor_user_id uuid references "user" (user_id) on delete set null,
    method text not null,
    request_uri text not null,
    matched_path text,
    status_code integer not null check (status_code between 100 and 599),
    duration_ms bigint not null check (duration_ms >= 0),
    access_context jsonb not null default '{}'::jsonb,
    ip_address inet generated always as (
        nullif(
            coalesce(
                access_context ->> 'real_ip',
                split_part(coalesce(access_context ->> 'forwarded_for', ''), ',', 1),
                access_context ->> 'remote_addr'
            ),
            ''
        )::inet
    ) stored,
    user_agent text generated always as (nullif(access_context ->> 'user_agent', '')) stored,
    created_at timestamptz not null default now()
);

create index if not exists audit_log_created_at_idx on audit_log (created_at desc);
create index if not exists audit_log_request_id_idx on audit_log (request_id);
create index if not exists audit_log_actor_user_id_idx on audit_log (actor_user_id);
create index if not exists audit_log_matched_path_idx on audit_log (matched_path, created_at desc);
create index if not exists audit_log_status_code_idx on audit_log (status_code, created_at desc);
create index if not exists audit_log_ip_address_idx on audit_log (ip_address);
create index if not exists audit_log_access_context_gin_idx on audit_log using gin (access_context jsonb_path_ops);
create index if not exists audit_log_request_uri_trgm_idx on audit_log using gin (request_uri gin_trgm_ops);
create index if not exists audit_log_user_agent_trgm_idx on audit_log using gin (user_agent gin_trgm_ops);
