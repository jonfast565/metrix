use chrono::NaiveDateTime;

pub fn parse_iso_date_string(s: String) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.fZ").unwrap()
}

pub fn parse_iso_date_str(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.fZ").unwrap()
}