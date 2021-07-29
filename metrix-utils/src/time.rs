use chrono::NaiveDateTime;

pub fn parse_iso_date_string(s: String) -> NaiveDateTime {
    match NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S") {
        Ok(x) => x,
        Err(_) => panic!("Date {} is invalid. It should be in the format Y-m-dTH:M:S", s)
    }
}

pub fn parse_iso_date_str(s: &str) -> NaiveDateTime {
    match NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        Ok(x) => x,
        Err(_) => panic!("Date {} is invalid. It should be in the format Y-m-dTH:M:S", s)
    }
}