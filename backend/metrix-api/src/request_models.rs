use rocket::serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MetricInsertRequest<'r> {
    pub data_point: Cow<'r, String>,
    pub data_type: Cow<'r, String>,
    pub data_group: Cow<'r, String>,
    pub data_value_numeric: f64,
}

impl MetricInsertRequest<'_> {
    pub fn to_metric_insert(&self) -> metrix_models::MetricInsert {
        metrix_models::MetricInsert {
            id: Uuid::new_v4(),
            data_type: self.data_type.to_string(),
            data_point: self.data_point.to_string(),
            data_group: Some(self.data_group.to_string()),
            data_value_numeric: self.data_value_numeric,
        }
    }
}

pub fn get_metric_model(
    data_point: &str,
    data_group: &str
) -> metrix_models::MetricQuery {
    metrix_models::MetricQuery {
        data_point: data_point.to_string(),
        data_group: match data_group {
            "" => None,
            s => Some(s.to_string())
        }
    }
}

pub fn get_metric_history_model(
    data_point: &str,
    data_group: &str,
    date: &str
) -> metrix_models::MetricPointQuery {
    metrix_models::MetricPointQuery {
        metric_query: metrix_models::MetricQuery {
            data_point: data_point.to_string(),
            data_group: match data_group {
                "" => None,
                s => Some(s.to_string()),
            },
        },
        date: metrix_utils::time::parse_iso_date_str(date),
    }
}

pub fn get_metric_series_model( 
    data_point: &str,
    data_group: &str,
    start_date: &str,
    end_date: &str) -> metrix_models::MetricRangeQuery {
        metrix_models::MetricRangeQuery {
            metric_query: metrix_models::MetricQuery {
                data_point: data_point.to_string(),
                data_group: match data_group {
                    "" => None,
                    s => Some(s.to_string()),
                },
            },
            start_date: metrix_utils::time::parse_iso_date_str(start_date),
            end_date: metrix_utils::time::parse_iso_date_str(end_date),
        }
    }
