create table if not exists audit_log (
    audit_log_id text primary key,
    request_id text not null,
    actor_user_id text references "user" (user_id) on delete set null,
    method text not null,
    request_uri text not null,
    matched_path text,
    status_code integer not null check (status_code between 100 and 599),
    duration_ms integer not null check (duration_ms >= 0),
    access_context text not null default '{}',
    created_at text not null default current_timestamp
);

create index if not exists audit_log_created_at_idx on audit_log (created_at desc);
create index if not exists audit_log_request_id_idx on audit_log (request_id);
create index if not exists audit_log_actor_user_id_idx on audit_log (actor_user_id);
create index if not exists audit_log_matched_path_idx on audit_log (matched_path, created_at desc);
create index if not exists audit_log_status_code_idx on audit_log (status_code, created_at desc);
