-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE metric (
    id uuid primary key not null default uuid_generate_v4(),
    data_point varchar(255) not null,
    data_type varchar(255) not null,
    data_value_numeric decimal(50, 50) not null,
    created_by varchar(255) not null default session_user,
    created_datetime datetime not null default now(),
    last_modified_by varchar(255) not null default session_user,
    last_modified_by_datetime datetime not null default now()
);