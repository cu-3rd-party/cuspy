create table if not exists rating_history (
    rating_history_id text primary key,
    user_id text not null references "user" (user_id) on delete cascade,
    rating integer not null,
    change integer not null,
    reason text,
    created_at text not null default current_timestamp
);

create index if not exists rating_history_user_id_created_at_idx on rating_history (user_id, created_at desc);
