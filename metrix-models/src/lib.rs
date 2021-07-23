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

impl MetricInsert {
    pub fn new_metric_insert(
        id: &String,
        data_type: &String,
        data_point: &String,
        data_group: &String,
        data_value_numeric: f64,
    ) -> MetricInsert {
        MetricInsert {
            id: Uuid::parse_str(id).unwrap(),
            data_type: data_type.to_string(),
            data_point: data_point.to_string(),
            data_group: Some(data_group.to_string()),
            data_value_numeric: data_value_numeric
        }
    }
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
    end_date: DateTime<Utc>,
}
