use crate::request_models::{
    MetricInsertRequest, MetricPointQueryRequest, MetricQueryRequest, MetricRangeRequest,
};
use crate::MetrixDatabaseConnection;
use rocket::serde::json::{Json};

#[post("/metric", format = "json", data = "<metric>")]
pub async fn post_metric(conn: MetrixDatabaseConnection, metric: Json<MetricInsertRequest<'_>>) {
    let insert = metric.to_metric_insert();
    conn.run(move |c| metrix_database::insert_metric(&insert, c))
        .await;
}

#[get("/metric?<opt..>")]
pub async fn get_metric(
    conn: MetrixDatabaseConnection,
    opt: MetricQueryRequest<'_>,
) -> Json<Vec<metrix_models::MetricResult>> {
    let getter = opt.to_metric_query();
    let result = conn
        .run(move |c| metrix_database::get_metrics(&getter, c))
        .await;
    Json(result)
}

#[get("/metric-point?<opt..>")]
pub async fn get_metric_history(
    conn: MetrixDatabaseConnection,
    opt: MetricPointQueryRequest<'_>,
) -> Json<Vec<metrix_models::MetricResult>> {
    let getter = opt.to_metric_point_query();
    let result = conn
        .run(move |c| metrix_database::get_metric_history(&getter, c))
        .await;
    Json(result)
}

#[get("/metric-series?<opt..>")]
pub async fn get_metric_series(
    conn: MetrixDatabaseConnection,
    opt: MetricRangeRequest<'_>,
) -> Json<Vec<metrix_models::MetricResult>> {
    let getter = opt.to_metric_point_query();
    let result = conn
        .run(move |c| metrix_database::get_metric_series_history(&getter, c))
        .await;
    Json(result)
}
