-- Your SQL goes here
alter table metric
add column data_grouping varchar(255);
alter table metric
alter column data_value_numeric
set data type numeric(50, 10);
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