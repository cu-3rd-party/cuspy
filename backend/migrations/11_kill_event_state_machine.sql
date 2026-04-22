alter table kill_event
    add column if not exists details jsonb not null default '{}'::jsonb,
    add column if not exists killer_confirmed_at timestamptz,
    add column if not exists victim_confirmed_at timestamptz,
    add column if not exists moderation_reason text,
    add column if not exists rating_applied_at timestamptz;

alter table kill_event
    drop constraint if exists kill_event_killer_id_victim_id_check;

alter table kill_event
    add constraint kill_event_killer_id_victim_id_check
    check (killer_id <> victim_id);

create or replace function apply_kill_rating_history()
    returns trigger as
$$
declare
    killer_rating bigint;
    victim_rating bigint;
    rating_delta bigint := 25;
begin
    if new.status = 'ADMIN_APPROVED'
       and coalesce(old.status, '') <> 'ADMIN_APPROVED'
       and new.rating_applied_at is null then
        select coalesce((
            select rating
            from rating_history
            where user_id = new.killer_id
            order by created_at desc, rating_history_id desc
            limit 1
        ), 1000)
        into killer_rating;

        select coalesce((
            select rating
            from rating_history
            where user_id = new.victim_id
            order by created_at desc, rating_history_id desc
            limit 1
        ), 1000)
        into victim_rating;

        insert into rating_history (rating_history_id, user_id, rating, change, reason)
        values
            (
                uuid_generate_v1mc(),
                new.killer_id,
                killer_rating + rating_delta,
                rating_delta,
                format('kill:%s:killer', new.kill_event_id)
            ),
            (
                uuid_generate_v1mc(),
                new.victim_id,
                victim_rating - rating_delta,
                -rating_delta,
                format('kill:%s:victim', new.kill_event_id)
            );

        new.rating_applied_at = now();
    end if;

    return new;
end;
$$ language plpgsql;

drop trigger if exists apply_kill_rating_history on kill_event;

create trigger apply_kill_rating_history
    before update on kill_event
    for each row
    execute function apply_kill_rating_history();
