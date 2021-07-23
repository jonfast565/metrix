use rocket::serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MetricRequest<'r> {
    pub id: Cow<'r, String>,
    pub data_point: Cow<'r, String>,
    pub data_type: Cow<'r, String>,
    pub data_group: Cow<'r, String>,
    pub data_value_numeric: f64,
}

impl MetricRequest<'_> {
    pub fn to_metric(&self) {
        
    }
}

#[derive(FromForm)]
pub struct OptionsRequest<'r> {
    data_point: Option<&'r str>,
    data_group: Option<&'r str>,
}

#[derive(FromForm)]
pub struct OptionsHistoryRequest<'r> {
    data_point: Option<&'r str>,
    data_group: Option<&'r str>,
    date: Option<&'r str>,
}

#[derive(FromForm)]
pub struct OptionsHistorySeriesRequest<'r> {
    data_point: Option<&'r str>,
    data_group: Option<&'r str>,
    start_date: Option<&'r str>,
    end_date: Option<&'r str>,
}
