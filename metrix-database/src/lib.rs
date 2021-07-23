#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use crate::diesel::ExpressionMethods;
use diesel::PgConnection;

pub fn insert_metric(model: &metrix_models::MetricInsert, conn: &PgConnection) {
    use schema::metric::dsl::*;
    let value_to_be_inserted = (
        id.eq(model.id), 
        data_grouping.eq(model.data_group), 
        data_point.eq(model.data_point), 
        data_type.eq(model.data_type), 
        data_value_numeric.eq(model.data_value_numeric));
    diesel::insert_into(schema::metric::table).values(value_to_be_inserted);
}

pub fn insert_metrics(model: Vec<&metrix_models::MetricInsert>, conn: &PgConnection) {
    use schema::metric::dsl::*;

}

pub fn get_metric(model: metrix_models::MetricQuery, conn: &PgConnection) {
    use schema::metric::dsl::*;

}

pub fn get_metric_history(model: metrix_models::MetricPointQuery, conn: &PgConnection) {
    use schema::metric::dsl::*;

}

pub fn get_metric_series_history(model: metrix_models::MetricRangeQuery, conn: &PgConnection) {
    use schema::metric::dsl::*;

}
