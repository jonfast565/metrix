
//#[post("/metric", format = "json", data = "<metric>")]
pub async fn post_metric(model: metrix_models::MetricInsertPartial) {
    
}

//#[get("/metric?<data_point>&<data_group>")]
pub async fn get_metric(
    query: metrix_models::MetricQuery
) -> Vec<metrix_models::MetricResult> {
    Vec::<metrix_models::MetricResult>::new()
}

// #[get("/metric-history?<data_point>&<data_group>&<date>")]
pub async fn get_metric_history(
    query: metrix_models::MetricPointQuery
) -> Vec<metrix_models::MetricResult> {
    Vec::<metrix_models::MetricResult>::new()
}

// #[get("/metric-series?<data_point>&<data_group>&<start_date>&<end_date>")]
pub async fn get_metric_series(
    query: metrix_models::MetricRangeQuery
) -> Vec<metrix_models::MetricResult> {
    Vec::<metrix_models::MetricResult>::new()
}