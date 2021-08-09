use crate::diesel::prelude::*;
use crate::models::*;

pub fn insert_metric_internal(model: &MetricInsertDb, conn: &PgConnection) {
    diesel::insert_into(crate::schema::metric::table)
        .values(model)
        .execute(conn)
        .expect("Error inserting row into database");
}

pub fn insert_metrics_internal(models: &Vec<MetricInsertDb>, conn: &PgConnection) {
    diesel::insert_into(crate::schema::metric::table)
        .values(models)
        .execute(conn)
        .expect("Error inserting rows into database");
}

pub fn get_metrics_internal(model: MetricQueryDb, conn: &PgConnection) -> Vec<MetricResultDb> {
    use crate::schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .limit(1)
        .order(created_datetime.desc())
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric");
    results
}

pub fn get_metric_history_internal(
    model: MetricPointQueryDb,
    conn: &PgConnection,
) -> Vec<MetricResultDb> {
    use crate::schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .filter(created_datetime.gt(model.date))
        .order(created_datetime.asc())
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric history");
    results
}

pub fn get_metric_series_history_internal(
    model: MetricRangeQueryDb,
    conn: &PgConnection,
) -> Vec<MetricResultDb> {
    use crate::schema::metric::dsl::*;
    let results = metric
        .filter(data_point.eq(model.data_point))
        .filter(data_grouping.eq(model.data_group))
        .filter(created_datetime.gt(model.start_date))
        .filter(created_datetime.lt(model.end_date))
        .order(created_datetime.asc())
        .load::<MetricResultDb>(conn)
        .expect("Error loading metric series history");
    results
}

pub fn get_metric_groups_internal(conn: &PgConnection) -> Vec<Option<String>> {
    use crate::schema::metric::dsl::*;
    let results = metric
    .select(data_grouping)
    .distinct()
    .load::<Option<String>>(conn)
    .expect("Error loading groups");
    results
}
