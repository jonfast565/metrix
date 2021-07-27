use rocket::serde::json::{json, Json, Value};
use crate::MetrixDatabaseConnection;
use crate::request_models::{
    MetricQueryRequest, MetricPointQueryRequest, MetricRangeRequest, MetricInsertRequest
};


#[post("/metric", format = "json", data = "<metric>")]
pub async fn post_metric(conn: MetrixDatabaseConnection, metric: Json<MetricInsertRequest<'_>>) {
    let insert = metric.to_metric_insert();
    conn.run(move |c| metrix_database::insert_metric(&insert, c)).await;
}

#[get("/metric?<opt..>")]
pub fn get_metric(conn: MetrixDatabaseConnection, opt: MetricQueryRequest<'_>) {
    let getter = opt.to_metric_query();
}

#[get("/metric-point?<opt..>")]
pub fn get_metric_history(conn: MetrixDatabaseConnection, opt: MetricPointQueryRequest<'_>) {
    let getter = opt.to_metric_point_query();
}

#[get("/metric-series?<opt..>")]
pub fn get_metric_series(conn: MetrixDatabaseConnection, opt: MetricRangeRequest<'_>) {
    let getter = opt.to_metric_point_query();
}