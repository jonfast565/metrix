use bigdecimal::BigDecimal;
use diesel::pg::data_types::{PgTimestamp};
use chrono::prelude::*;
use uuid::Uuid;
use crate::schema::metric;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "metric"]
pub struct MetricInsertDb {
    pub id: Uuid,
    pub data_point: String,
    pub data_grouping: Option<String>,
    pub data_type: String,
    pub data_value_numeric: BigDecimal,
}

#[derive(Debug, Queryable)]
pub struct MetricQueryDb {
    pub data_point: String,
    pub data_group: Option<String>,
}

#[derive(Debug, Queryable)]
pub struct MetricPointQueryDb {
    pub data_point: String,
    pub data_group: Option<String>,
    pub date: NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct MetricRangeQueryDb {
    pub data_point: String,
    pub data_group: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct MetricResultDb {
    pub id: Uuid,
    pub data_point: String,
    pub data_type: String,
    pub data_value_numeric: BigDecimal,
    pub created_by: String,
    pub created_datetime: PgTimestamp,
    pub last_modified_by: String,
    pub last_modified_by_datetime: PgTimestamp,
    pub data_grouping: Option<String>,
}