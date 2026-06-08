alter table kill_event add column details text not null default '{}';
alter table kill_event add column killer_confirmed_at text;
alter table kill_event add column victim_confirmed_at text;
alter table kill_event add column moderation_reason text;
alter table kill_event add column rating_applied_at text;
