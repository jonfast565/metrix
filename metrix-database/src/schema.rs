table! {
    metric (id) {
        id -> Uuid,
        data_point -> Varchar,
        data_type -> Varchar,
        data_value_numeric -> Numeric,
        created_by -> Varchar,
        created_datetime -> Timestamp,
        last_modified_by -> Varchar,
        last_modified_by_datetime -> Timestamp,
    }
}
