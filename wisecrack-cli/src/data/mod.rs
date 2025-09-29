use chrono::{DateTime, Utc};
pub struct Data {
    data: Option<String>,
    time: DateTime<Utc>,
    error_message: Option<String>,
    response_code: u16,
}

impl Data {
    pub fn new(
        data: Option<String>,
        time: DateTime<Utc>,
        error_message: Option<String>,
        response_code: u16,
    ) -> Self {
        Self {
            data,
            time,
            error_message,
            response_code,
        }
    }
}

pub fn fetch_dad_jokes(config_file: Option<String>) -> Data {
    Data::new(Some(String::from("Some Dad joke")), Utc::now(), None, 200)
}

pub fn fetch_quote(config_file: Option<String>) -> Data {
    Data::new(None, Utc::now(), Some(String::from("Error happened")), 403)
}
