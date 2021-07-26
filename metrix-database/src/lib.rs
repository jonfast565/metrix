#[macro_use]
extern crate diesel;

use crate::diesel::RunQueryDsl;

pub mod models;
pub mod schema;

use diesel::PgConnection;

pub fn insert_metric(model: &models::MetricInsertDb, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(model)
        .execute(conn)
        .expect("Error inserting row into database");
}

pub fn insert_metrics(models: Vec<&models::MetricInsertDb>, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(models)
        .execute(conn)
        .expect("Error inserting rows into database");
}

pub fn get_metric(model: models::MetricQueryDb, conn: &PgConnection) {
    use schema::metric::dsl::*;
}

pub fn get_metric_history(model: models::MetricPointQueryDb, conn: &PgConnection) {
    use schema::metric::dsl::*;
}

pub fn get_metric_series_history(model: models::MetricRangeQueryDb, conn: &PgConnection) {
    use schema::metric::dsl::*;
}
