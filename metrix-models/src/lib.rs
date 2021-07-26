use bigdecimal::BigDecimal;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricInsert {
    pub id: Uuid,
    pub data_point: String,
    pub data_group: Option<String>,
    pub data_type: String,
    pub data_value_numeric: f64,
}

pub struct MetricQuery {
    pub data_point: String,
    pub data_group: Option<String>,
}

pub struct MetricPointQuery {
    pub metric_query: MetricQuery,
    pub date: NaiveDateTime,
}

pub struct MetricRangeQuery {
    pub metric_query: MetricQuery,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

pub struct MetricResult {
    pub id: Uuid,
    pub data_point: String,
    pub data_type: String,
    pub data_value_numeric: BigDecimal,
    pub created_by: String,
    pub created_datetime: NaiveDateTime,
    pub last_modified_by: String,
    pub last_modified_by_datetime: NaiveDateTime,
    pub data_grouping: Option<String>,
}