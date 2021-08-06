use chrono::NaiveDateTime;

pub const FORMAT_STRING : &'static str = "%Y-%m-%dT%H:%M:%S";

pub fn parse_iso_date_string(s: String) -> NaiveDateTime {
    match NaiveDateTime::parse_from_str(&s, FORMAT_STRING) {
        Ok(x) => x,
        Err(_) => panic!("Date {} is invalid. It should be in the format Y-m-dTH:M:S", s)
    }
}

pub fn parse_iso_date_str(s: &str) -> NaiveDateTime {
    match NaiveDateTime::parse_from_str(s, FORMAT_STRING) {
        Ok(x) => x,
        Err(_) => panic!("Date {} is invalid. It should be in the format Y-m-dTH:M:S", s)
    }
}