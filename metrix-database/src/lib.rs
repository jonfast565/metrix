#[macro_use]
extern crate diesel;
use self::diesel::prelude::*;
use self::models::*;

pub mod models;
pub mod schema;

use diesel::PgConnection;

pub fn insert_metric() {

}

pub fn insert_metrics() {

}

pub fn get_metrics() {

}

pub fn get_metric_history() {

}

pub fn get_metric_series_history() {
    
}

fn insert_metric_internal(model: &MetricInsertDb, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(model)
        .execute(conn)
        .expect("Error inserting row into database");
}

fn insert_metrics_internal(models: Vec<&MetricInsertDb>, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(models)
        .execute(conn)
        .expect("Error inserting rows into database");
}

fn get_metric_internal(model: MetricQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
    use schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .limit(1)
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric");
    results
}

fn get_metric_history_internal(model: MetricPointQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
    use schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .filter(created_datetime.gt(model.date))
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric history");
    results
}

fn get_metric_series_history_internal(model: MetricRangeQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
    use schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .filter(created_datetime.gt(model.start_date))
        .filter(created_datetime.lt(model.end_date))
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric series history");
    results
}
