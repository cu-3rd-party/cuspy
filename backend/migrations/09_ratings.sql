-- Rating history to track progression and prevent data loss
create table if not exists rating_history
(
    rating_history_id uuid primary key     default uuid_generate_v1mc(),
    user_id           uuid        not null references "user" (user_id) on delete cascade,
    rating            bigint      not null,
    change            bigint      not null,
    reason            text,
    created_at        timestamptz not null default now()
);

create index if not exists rating_history_user_id_created_at_idx on rating_history (user_id, created_at desc);

create or replace function update_user_rating_from_history()
    returns trigger as
$$
begin
    -- Обновляем рейтинг пользователя: прибавляем изменение (change) к текущему значению
    update "user"
    set rating     = rating + new.change,
        updated_at = now()
    where user_id = new.user_id;

    return new;
end;
$$ language plpgsql;

create trigger trg_rating_history_update_user_rating
    after insert
    on rating_history
    for each row
execute function update_user_rating_from_history();
