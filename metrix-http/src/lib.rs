pub const BASE_URL: &'static str = "http://localhost:8000";

//#[post("/metric", format = "json", data = "<metric>")]
pub async fn post_metric(model: metrix_models::MetricInsertPartial) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/metric", BASE_URL)).json(&model).send().await;
    let result = match res {
        Ok(_) => Ok(()),
        Err(x) => Err(x)
    };
    result
}

//#[get("/metric?<data_point>&<data_group>")]
pub async fn get_metric(
    query: metrix_models::MetricQuery
) -> Result<Vec<metrix_models::MetricResult>, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = vec![
        ("data_point", query.data_point),
        ("data_group", query.data_group.unwrap())
    ];
    let res = client.get(format!("{}/metric", BASE_URL)).query(&params).send().await;
    let result = match res {
        Ok(x) => Ok(x.json::<Vec<metrix_models::MetricResult>>().await.unwrap()),
        Err(x) => Err(x)
    };
    result
}

// #[get("/metric-history?<data_point>&<data_group>&<date>")]
pub async fn get_metric_history(
    query: metrix_models::MetricPointQuery
) -> Result<Vec<metrix_models::MetricResult>, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = vec![
        ("data_point", query.metric_query.data_point),
        ("data_group", query.metric_query.data_group.unwrap()),
        ("date", query.date.format(metrix_utils::time::FORMAT_STRING).to_string())
    ];
    let res = client.get(format!("{}/metric-history", BASE_URL)).query(&params).send().await;
    let result = match res {
        Ok(x) => Ok(x.json::<Vec<metrix_models::MetricResult>>().await.unwrap()),
        Err(x) => Err(x)
    };
    result
}

// #[get("/metric-series?<data_point>&<data_group>&<start_date>&<end_date>")]
pub async fn get_metric_series(
    query: metrix_models::MetricRangeQuery
) -> Result<Vec<metrix_models::MetricResult>, reqwest::Error> {
    let client = reqwest::Client::new();
    let params = vec![
        ("data_point", query.metric_query.data_point),
        ("data_group", query.metric_query.data_group.unwrap()),
        ("start_date", query.start_date.format(metrix_utils::time::FORMAT_STRING).to_string()),
        ("end_date", query.end_date.format(metrix_utils::time::FORMAT_STRING).to_string())
    ];
    let res = client.get(format!("{}/metric-series", BASE_URL)).query(&params).send().await;
    let result = match res {
        Ok(x) => Ok(x.json::<Vec<metrix_models::MetricResult>>().await.unwrap()),
        Err(x) => Err(x)
    };
    result
}