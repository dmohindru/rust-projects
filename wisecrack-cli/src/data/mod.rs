use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DadJoke {
    joke: String,
    status: u16,
}

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

pub trait DataProvider {
    fn fetch_dad_jokes(&self) -> Data;
}

pub struct ApiDataProvider {
    client: Client,
}

impl ApiDataProvider {
    pub fn new() -> Self {
        ApiDataProvider {
            client: Client::new(),
        }
    }
}

impl DataProvider for ApiDataProvider {
    fn fetch_dad_jokes(&self) -> Data {
        let url = "https://icanhazdadjoke.com";
        let response = match self.client.get(url).send() {
            Ok(res) => res,
            Err(e) => {
                return Data {
                    data: None,
                    time: Utc::now(),
                    error_message: Some(e.to_string()),
                    // TODO: find a better way to extract this value
                    response_code: 500,
                };
            }
        };

        let joke: DadJoke = match response.json() {
            Ok(joke) => joke,
            Err(e) => {
                return Data {
                    data: None,
                    time: Utc::now(),
                    error_message: Some(e.to_string()),
                    // TODO: find a better way to extract this value
                    response_code: 500,
                };
            }
        };
        Data {
            data: Some(joke.joke),
            time: Utc::now(),
            error_message: None,
            response_code: joke.status,
        }
    }
}

pub fn fetch_dad_jokes() -> Data {
    Data::new(Some(String::from("Some Dad joke")), Utc::now(), None, 200)
}

pub fn fetch_quote() -> Data {
    Data::new(None, Utc::now(), Some(String::from("Error happened")), 403)
}
