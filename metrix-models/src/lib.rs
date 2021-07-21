use uuid::Uuid;

pub struct Metric {
    pub id: Uuid,
    pub data_point: String,
    pub data_type: String,
    pub data_value_numeric: f64,
}

impl Metric {
}