use uuid::Uuid;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricInsert {
    pub id: Uuid,
    pub data_point: String,
    pub data_group: Option<String>,
    pub data_type: String,
    pub data_value_numeric: f64,
}

impl MetricInsert {
}

pub struct MetricQuery {
    pub data_point: String,
    pub data_group: Option<String>,
}

pub struct MetricPointQuery {
    metric_query: MetricQuery,
    date: DateTime<Utc>,
}

pub struct MetricRangeQuery {
    metric_query: MetricQuery,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
}

