-- Your SQL goes here
alter table metric
add column data_grouping varchar(255);
create view metric_view as
select id,
    data_grouping,
    data_point,
    data_type,
    data_value_numeric,
    created_by,
    created_datetime,
    last_modified_by,
    last_modified_by_datetime
    from metric