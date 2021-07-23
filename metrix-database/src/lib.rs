#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

pub fn insert_metric(model: &metrix_models::MetricInsert) {

}

pub fn insert_metrics(model: Vec<&metrix_models::MetricInsert>) {

}

pub fn get_metric(model: metrix_models::MetricQuery) {

}

pub fn get_metric_history(model: metrix_models::MetricPointQuery) {

}

pub fn get_metric_series_history(model: metrix_models::MetricRangeQuery) {

}
