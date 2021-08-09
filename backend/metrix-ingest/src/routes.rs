use crate::ingest_queue::InsertQueueManager;
use crate::request_models::MetricInsertRequest;
use crate::MetrixDatabaseConnection;
use rocket::serde::json::Json;

#[post("/metric/insert", format = "json", data = "<metric>")]
pub fn post_metric(metric: Json<MetricInsertRequest<'_>>) {
    let insert = metric.to_metric_insert();
    info!("Inserting record into queue manager");
    InsertQueueManager::insert(insert);
}

#[get("/metric?<data_point>&<data_group>")]
pub async fn get_metric(
    conn: MetrixDatabaseConnection,
    data_point: &str,
    data_group: &str,
) -> Json<Vec<metrix_models::MetricResult>> {
    let getter = crate::request_models::get_metric_model(data_point, data_group);
    let result = conn
        .run(move |c| metrix_database::get_metrics(&getter, c))
        .await;
    Json(result)
}

#[get("/metric/history?<data_point>&<data_group>&<date>")]
pub async fn get_metric_history(
    conn: MetrixDatabaseConnection,
    data_point: &str,
    data_group: &str,
    date: &str,
) -> Json<Vec<metrix_models::MetricResult>> {
    let getter = crate::request_models::get_metric_history_model(data_point, data_group, date);
    let result = conn
        .run(move |c| metrix_database::get_metric_history(&getter, c))
        .await;
    Json(result)
}

#[get("/metric/series?<data_point>&<data_group>&<start_date>&<end_date>")]
pub async fn get_metric_series(
    conn: MetrixDatabaseConnection,
    data_point: &str,
    data_group: &str,
    start_date: &str,
    end_date: &str,
) -> Json<Vec<metrix_models::MetricResult>> {
    let getter = crate::request_models::get_metric_series_model(
        data_point, data_group, start_date, end_date,
    );
    let result = conn
        .run(move |c| metrix_database::get_metric_series_history(&getter, c))
        .await;
    Json(result)
}

#[get("/metric/data-groupings")]
pub async fn get_metric_data_groupings(
    conn: MetrixDatabaseConnection,
) -> Json<Vec<metrix_models::MetricValue>> {
    let result = conn
        .run(move |c| metrix_database::get_metric_groups(c))
        .await;
    Json(result)
}

#[get("/metric/data-points?<data_grouping>")]
pub async fn get_metric_data_points_for_grouping(
    conn: MetrixDatabaseConnection,
    data_grouping: &str,
) -> Json<Vec<metrix_models::MetricValue>> {
    let grouping: String = data_grouping.to_string();
    let result = conn
        .run(move |c| metrix_database::get_metric_points_by_group(c, grouping))
        .await;
    Json(result)
}
