#[macro_use]
extern crate diesel;

use self::models::*;

mod mappers;
pub mod models;
mod queries;
pub mod schema;

use diesel::PgConnection;

pub fn insert_metric(model: &metrix_models::MetricInsert, conn: &PgConnection) {
    let metric_db = mappers::map_insert_model(model);
    queries::insert_metric_internal(&metric_db, conn);
}

pub fn insert_metrics(models: &Vec<metrix_models::MetricInsert>, conn: &PgConnection) {
    let db_models: Vec<MetricInsertDb> = models
        .into_iter()
        .map(|model| mappers::map_insert_model(model))
        .collect();

    queries::insert_metrics_internal(&db_models, conn);
}

pub fn get_metrics(
    model: &metrix_models::MetricQuery,
    conn: &PgConnection,
) -> Vec<metrix_models::MetricResult> {
    let db_model = MetricQueryDb {
        data_group: Some(model.data_group.as_ref().unwrap().to_string()),
        data_point: model.data_point.to_string(),
    };
    let results = queries::get_metrics_internal(db_model, conn);
    let result = mappers::map_results(results);
    result
}

pub fn get_metric_history(
    model: &metrix_models::MetricPointQuery,
    conn: &PgConnection,
) -> Vec<metrix_models::MetricResult> {
    let db_model = MetricPointQueryDb {
        data_group: Some(model.metric_query.data_group.as_ref().unwrap().to_string()),
        data_point: model.metric_query.data_point.to_string(),
        date: model.date,
    };
    let results = queries::get_metric_history_internal(db_model, conn);
    let result = mappers::map_results(results);
    result
}

pub fn get_metric_series_history(
    model: &metrix_models::MetricRangeQuery,
    conn: &PgConnection,
) -> Vec<metrix_models::MetricResult> {
    let db_model = MetricRangeQueryDb {
        data_group: Some(model.metric_query.data_group.as_ref().unwrap().to_string()),
        data_point: model.metric_query.data_point.to_string(),
        start_date: model.start_date,
        end_date: model.end_date,
    };
    let results = queries::get_metric_series_history_internal(db_model, conn);
    let result = mappers::map_results(results);
    result
}

pub fn get_metric_groups(conn: &PgConnection) -> Vec<metrix_models::MetricValue> {
    let results = queries::get_metric_groups_internal(conn);
    let result = mappers::map_string_option_to_model(results);
    result
}

pub fn get_metric_points_by_group(
    conn: &PgConnection,
    data_grouping: String,
) -> Vec<metrix_models::MetricValue> {
    let results = queries::get_metric_data_points_by_group(conn, data_grouping);
    let result = mappers::map_string_to_model(results);
    result
}
