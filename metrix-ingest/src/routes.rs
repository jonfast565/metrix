use rocket::serde::json::{json, Json, Value};

use crate::request_models::{
    MetricQueryRequest, MetricPointQueryRequest, MetricRangeRequest, MetricInsertRequest
};

#[post("/metric", format = "json", data = "<metric>")]
pub fn post_metric(metric: Json<MetricInsertRequest<'_>>) {
    let insert = metric.to_metric_insert();
}

#[get("/metric?<opt..>")]
pub fn get_metric(opt: MetricQueryRequest<'_>) {
    let getter = opt.to_metric_query();
}

#[get("/metric-point?<opt..>")]
pub fn get_metric_history(opt: MetricPointQueryRequest<'_>) {
    let getter = opt.to_metric_point_query();
}

#[get("/metric-series?<opt..>")]
pub fn get_metric_series(opt: MetricRangeRequest<'_>) {
    let getter = opt.to_metric_point_query();
}