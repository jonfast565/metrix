use crate::models::*;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;

pub fn map_string_option_to_model(result: Vec<Option<String>>) -> Vec<String> {
    let results = result
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    results
}

pub fn map_results(results: Vec<MetricResultDb>) -> Vec<metrix_models::MetricResult> {
    let result = results
        .into_iter()
        .map(|x| metrix_models::MetricResult {
            id: x.id,
            data_point: x.data_point,
            data_type: x.data_type,
            data_value_numeric: x.data_value_numeric,
            created_datetime: x.created_datetime,
            data_grouping: x.data_grouping,
        })
        .collect::<Vec<metrix_models::MetricResult>>();
    result
}

pub fn map_insert_model(model: &metrix_models::MetricInsert) -> MetricInsertDb {
    MetricInsertDb {
        id: model.id,
        data_point: model.data_point.to_string(),
        data_grouping: Some(model.data_group.as_ref().unwrap().to_string()),
        data_type: model.data_type.to_string(),
        data_value_numeric: BigDecimal::from_f64(model.data_value_numeric).unwrap(),
    }
}
