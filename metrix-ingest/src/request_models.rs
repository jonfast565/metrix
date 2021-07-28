use rocket::serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MetricInsertRequest<'r> {
    pub id: Cow<'r, String>,
    pub data_point: Cow<'r, String>,
    pub data_type: Cow<'r, String>,
    pub data_group: Cow<'r, String>,
    pub data_value_numeric: f64,
}

impl MetricInsertRequest<'_> {
    pub fn to_metric_insert(&self) -> metrix_models::MetricInsert {
        metrix_models::MetricInsert {
            id: Uuid::parse_str(&self.id).unwrap(),
            data_type: self.data_type.to_string(),
            data_point: self.data_point.to_string(),
            data_group: Some(self.data_group.to_string()),
            data_value_numeric: self.data_value_numeric
        }
    }
}

#[derive(FromForm)]
pub struct MetricQueryRequest<'r> {
    data_point: Option<&'r str>,
    data_group: Option<&'r str>,
}

impl MetricQueryRequest<'_> {
    pub fn to_metric_query(&self) -> metrix_models::MetricQuery {
        metrix_models::MetricQuery {
            data_point: self.data_point.unwrap().to_string(),
            data_group: Some(self.data_group.unwrap().to_string())
        }
    }
}

#[derive(FromForm)]
pub struct MetricPointQueryRequest<'r> {
    data_point: Option<&'r str>,
    data_group: Option<&'r str>,
    date: Option<&'r str>,
}

impl MetricPointQueryRequest<'_> {
    pub fn to_metric_point_query(&self) -> metrix_models::MetricPointQuery {
        metrix_models::MetricPointQuery {
            metric_query: metrix_models::MetricQuery {
                data_point: self.data_point.unwrap().to_string(),
                data_group: Some(self.data_group.unwrap().to_string())
            },
            date: metrix_utils::parse_iso_date_str(self.date.unwrap())
        }
    }
}

#[derive(FromForm)]
pub struct MetricRangeRequest<'r> {
    data_point: Option<&'r str>,
    data_group: Option<&'r str>,
    start_date: Option<&'r str>,
    end_date: Option<&'r str>,
}

impl MetricRangeRequest<'_> {
    pub fn to_metric_point_query(&self) -> metrix_models::MetricRangeQuery {
        metrix_models::MetricRangeQuery {
            metric_query: metrix_models::MetricQuery {
                data_point: self.data_point.unwrap().to_string(),
                data_group: Some(self.data_group.unwrap().to_string())
            },
            start_date: metrix_utils::parse_iso_date_str(self.start_date.unwrap()),
            end_date: metrix_utils::parse_iso_date_str(self.end_date.unwrap())
        }
    }
}
