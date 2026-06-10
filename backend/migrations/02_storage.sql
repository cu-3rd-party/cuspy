-- Наверное это контринтуитивно, но я не хочу поднимать S3 для хранения аватарок
-- но все равно хранить base64 внутри agent_data json очень плохая затея.
-- так что буду хранить в бд метаданные и на диске хранить путь к файлу

create table if not exists "resource" (
    resource_id uuid primary key default uuid_generate_v1mc(),
    file_path varchar(512) not null,
    file_size int not null, -- метаданные на будущее
    mime_type varchar(20),  -- метаданные на будущее
    checksum varchar(64) unique,   -- дедубликация
    created_at timestamptz not null default now(),
    updated_at timestamptz  -- обновляется автоматически (все же любят триггеры да?)
);

select trigger_updated_at('"resource"');

create index if not exists resource_checksum_idx on "resource" (checksum);
create index if not exists resource_location on "resource" (file_path);
