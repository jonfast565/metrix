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

#[derive(Debug)]
pub struct MetricQueryDb {
    pub data_point: String,
    pub data_group: Option<String>,
}

#[derive(Debug)]
pub struct MetricPointQueryDb {
    pub metric_query: MetricQueryDb,
    pub date: NaiveDateTime,
}

#[derive(Debug)]
pub struct MetricRangeQueryDb {
    pub metric_query: MetricQueryDb,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}