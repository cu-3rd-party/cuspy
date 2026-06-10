create table if not exists "agent_data" (
    agent_data_id uuid primary key default uuid_generate_v1mc(),

    codename text,
    academic_group text,
    academic_level text,
    course_number int,
    bachelor_track text,
    identification_name text,
    identification_image_id uuid,
    foreign key (identification_image_id) references "resource"(resource_id),
    physical_contact_allowed boolean not null default false,
    hugs_close_proximity_allowed boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

select trigger_updated_at('"agent_data"');
