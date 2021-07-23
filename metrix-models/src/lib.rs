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
