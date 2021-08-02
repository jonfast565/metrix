use bigdecimal::BigDecimal;
use std::str::FromStr;
use bytesize::ByteSize;
use crate::MetricSizePartial;

pub fn get_size_from_bytesize(size: ByteSize) -> MetricSizePartial {
    let unit_string = size.to_string_as(false);
    let unit_split : Vec<&str> = unit_string.split(" ").collect(); 
    MetricSizePartial {
        data_type: unit_split[0].to_string(),
        data_value_numeric: BigDecimal::from_str(unit_split[1]).unwrap()
    }
}