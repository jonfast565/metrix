#[macro_use]
extern crate diesel;

use self::diesel::prelude::*;
use self::models::*;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;

pub mod models;
pub mod schema;

use diesel::PgConnection;

pub fn insert_metric(model: &metrix_models::MetricInsert, conn: &PgConnection) {
    let metric_db = MetricInsertDb {
        id: model.id,
        data_point: model.data_point.to_string(),
        data_grouping: Some(model.data_group.as_ref().unwrap().to_string()),
        data_type: model.data_type.to_string(),
        data_value_numeric: BigDecimal::from_f64(model.data_value_numeric).unwrap(),
    };
    insert_metric_internal(&metric_db, conn);
}

pub fn insert_metrics(models: &Vec<metrix_models::MetricInsert>, conn: &PgConnection) {
    let db_models: Vec<MetricInsertDb> = models
        .into_iter()
        .map(|model| MetricInsertDb {
            id: model.id,
            data_point: model.data_point.to_string(),
            data_grouping: Some(model.data_group.as_ref().unwrap().to_string()),
            data_type: model.data_type.to_string(),
            data_value_numeric: BigDecimal::from_f64(model.data_value_numeric).unwrap(),
        })
        .collect();

    insert_metrics_internal(&db_models, conn);
}

pub fn get_metrics(
    model: &metrix_models::MetricQuery,
    conn: &PgConnection,
) -> Vec<metrix_models::MetricResult> {
    let db_model = MetricQueryDb {
        data_group: Some(model.data_group.as_ref().unwrap().to_string()),
        data_point: model.data_point.to_string(),
    };
    let results = get_metrics_internal(db_model, conn);
    let result = results
        .into_iter()
        .map(|x| metrix_models::MetricResult {
            id: x.id,
            data_point: x.data_point,
            data_type: x.data_type,
            data_value_numeric: x.data_value_numeric,
            created_by: x.created_by,
            created_datetime: x.created_datetime,
            last_modified_by: x.last_modified_by,
            last_modified_by_datetime: x.last_modified_by_datetime,
            data_grouping: x.data_grouping,
        })
        .collect::<Vec<metrix_models::MetricResult>>();
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
    let results = get_metric_history_internal(db_model, conn);
    let result = results
        .into_iter()
        .map(|x| metrix_models::MetricResult {
            id: x.id,
            data_point: x.data_point,
            data_type: x.data_type,
            data_value_numeric: x.data_value_numeric,
            created_by: x.created_by,
            created_datetime: x.created_datetime,
            last_modified_by: x.last_modified_by,
            last_modified_by_datetime: x.last_modified_by_datetime,
            data_grouping: x.data_grouping,
        })
        .collect::<Vec<metrix_models::MetricResult>>();
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
    let results = get_metric_series_history_internal(db_model, conn);
    let result = results
        .into_iter()
        .map(|x| metrix_models::MetricResult {
            id: x.id,
            data_point: x.data_point,
            data_type: x.data_type,
            data_value_numeric: x.data_value_numeric,
            created_by: x.created_by,
            created_datetime: x.created_datetime,
            last_modified_by: x.last_modified_by,
            last_modified_by_datetime: x.last_modified_by_datetime,
            data_grouping: x.data_grouping,
        })
        .collect::<Vec<metrix_models::MetricResult>>();
    result
}

fn insert_metric_internal(model: &MetricInsertDb, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(model)
        .execute(conn)
        .expect("Error inserting row into database");
}

fn insert_metrics_internal(models: &Vec<MetricInsertDb>, conn: &PgConnection) {
    diesel::insert_into(schema::metric::table)
        .values(models)
        .execute(conn)
        .expect("Error inserting rows into database");
}

fn get_metrics_internal(model: MetricQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
    use schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .limit(1)
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric");
    results
}

fn get_metric_history_internal(
    model: MetricPointQueryDb,
    conn: &PgConnection,
) -> Vec<MetricResultDb> {
    use schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .filter(created_datetime.gt(model.date))
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric history");
    results
}

fn get_metric_series_history_internal(
    model: MetricRangeQueryDb,
    conn: &PgConnection,
) -> Vec<MetricResultDb> {
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
