
pub async fn send_metric(
    hostname: String,
    data_point: String,
    data_type: String,
    value: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let model = metrix_models::MetricInsertPartial {
        data_group: Some(hostname),
        data_point: data_point,
        data_type: data_type,
        data_value_numeric: value,
    };
    Ok(metrix_http::post_metric(model).await?)
}
