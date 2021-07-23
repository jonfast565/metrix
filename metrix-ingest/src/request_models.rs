use rocket::serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MetricRequest<'r> {
    pub id: Cow<'r, String>,
    pub data_point: Cow<'r, String>,
    pub data_type: Cow<'r, String>,
    pub data_value_numeric: f64,
}

#[derive(FromForm)]
pub struct OptionsRequest<'r> {
    name: Option<&'r str>,
}

#[derive(FromForm)]
pub struct OptionsHistoryRequest<'r> {
    name: Option<&'r str>,
    date: Option<&'r str>,
}

#[derive(FromForm)]
pub struct OptionsHistorySeriesRequest<'r> {
    name: Option<&'r str>,
    start_date: Option<&'r str>,
    end_date: Option<&'r str>,
}
