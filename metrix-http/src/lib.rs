
//#[post("/metric", format = "json", data = "<metric>")]
pub async fn post_metric(model: metrix_models::MetricInsertPartial) {
    let client = reqwest::Client::new();
    let _res = client.post("http://localhost:8000/metric").json(&model).send().await;
}

//#[get("/metric?<data_point>&<data_group>")]
pub async fn get_metric(
    query: metrix_models::MetricQuery
) -> Vec<metrix_models::MetricResult> {
    let client = reqwest::Client::new();
    let params = vec![
        ("data_point", query.data_point),
        ("data_group", query.data_group.unwrap())
    ];
    let res = client.get("http://localhost:8000/metric").query(&params).send().await;
    let result = match res {
        Ok(x) => x.json::<Vec<metrix_models::MetricResult>>().await.unwrap(),
        Err(_) => panic!("Unable to get metric")
    };
    result
}

// #[get("/metric-history?<data_point>&<data_group>&<date>")]
pub async fn get_metric_history(
    query: metrix_models::MetricPointQuery
) -> Vec<metrix_models::MetricResult> {
    let client = reqwest::Client::new();
    let params = vec![
        ("data_point", query.metric_query.data_point),
        ("data_group", query.metric_query.data_group.unwrap()),
        ("date", query.date.format(metrix_utils::time::FORMAT_STRING).to_string())
    ];
    let res = client.get("http://localhost:8000/metric-history").query(&params).send().await;
    let result = match res {
        Ok(x) => x.json::<Vec<metrix_models::MetricResult>>().await.unwrap(),
        Err(_) => panic!("Unable to get metric")
    };
    result
}

// #[get("/metric-series?<data_point>&<data_group>&<start_date>&<end_date>")]
pub async fn get_metric_series(
    query: metrix_models::MetricRangeQuery
) -> Vec<metrix_models::MetricResult> {
    let client = reqwest::Client::new();
    let params = vec![
        ("data_point", query.metric_query.data_point),
        ("data_group", query.metric_query.data_group.unwrap()),
        ("start_date", query.start_date.format(metrix_utils::time::FORMAT_STRING).to_string()),
        ("end_date", query.end_date.format(metrix_utils::time::FORMAT_STRING).to_string())
    ];
    let res = client.get("http://localhost:8000/metric-series").query(&params).send().await;
    let result = match res {
        Ok(x) => x.json::<Vec<metrix_models::MetricResult>>().await.unwrap(),
        Err(_) => panic!("Unable to get metric")
    };
    result
}