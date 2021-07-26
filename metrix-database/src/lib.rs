#[macro_use]
extern crate diesel;
use self::diesel::prelude::*;
use self::models::*;

pub mod models;
pub mod schema;

use diesel::PgConnection;

pub fn insert_metric(model: &MetricInsertDb, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(model)
        .execute(conn)
        .expect("Error inserting row into database");
}

pub fn insert_metrics(models: Vec<&MetricInsertDb>, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(models)
        .execute(conn)
        .expect("Error inserting rows into database");
}

pub fn get_metric(model: MetricQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
    use schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .limit(1)
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric");
    results
}

pub fn get_metric_history(model: MetricPointQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
    use schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .filter(created_datetime.gt(model.date))
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric history");
    results
}

pub fn get_metric_series_history(model: MetricRangeQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
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
