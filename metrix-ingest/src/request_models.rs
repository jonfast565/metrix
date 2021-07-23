
use std::borrow::Cow;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MetricRequest<'r> {
    pub id: Cow<'r, String>,
    pub data_point: Cow<'r, String>,
    pub data_type: Cow<'r, String>,
    pub data_value_numeric: f64,
}

#[derive(FromForm)]
pub struct Options<'r> {
    name: Option<&'r str>,
}