use diesel::pg::data_types::PgNumeric;
use chrono::prelude::*;
use uuid::Uuid;
use crate::schema::metric;

#[derive(Debug, Insertable)]
#[table_name = "metric"]
pub struct MetricInsertDb {
    pub id: Uuid,
    pub data_point: String,
    pub data_grouping: Option<String>,
    pub data_type: String,
    pub data_value_numeric: PgNumeric,
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