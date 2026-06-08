create table if not exists global_event (
    global_event_id text primary key,
    event_type text not null check (event_type in ('GLOBAL_TARGET', 'NIGHT_HUNT', 'SMIITE')),
    trigger_id text references "user" (user_id) on delete set null,
    target_id text references "user" (user_id) on delete set null,
    start_time text not null,
    end_time text not null,
    payload text not null default '{}',
    created_at text not null default current_timestamp,
    updated_at text
);

create index if not exists global_event_start_time_idx on global_event (start_time);
create index if not exists global_event_end_time_idx on global_event (end_time);
create index if not exists global_event_target_id_idx on global_event (target_id);
