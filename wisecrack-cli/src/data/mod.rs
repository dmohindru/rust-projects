use core::fmt;

use chrono::{DateTime, Utc};
use reqwest::{blocking::Client, header::ACCEPT};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DadJoke {
    joke: String,
    status: u16,
}

#[derive(Debug, Deserialize)]
struct Quotes {
    q: String,
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

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Time: {}", self.time)?;
        writeln!(f, "Response Code: {}", self.response_code)?;
        match &self.data {
            Some(d) => writeln!(f, "Data: {}", d)?,
            None => writeln!(f, "Data: <none>")?,
        }
        match &self.error_message {
            Some(e) => writeln!(f, "Error: {}", e)?,
            None => writeln!(f, "Error: <none>")?,
        }
        Ok(())
    }
}

pub fn fetch_dad_jokes() -> Data {
    let client = Client::new();
    let url = "https://icanhazdadjoke.com";
    let response = match client.get(url).header(ACCEPT, "application/json").send() {
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

pub fn fetch_quote() -> Data {
    let client = Client::new();
    let url = "https://zenquotes.io/api/random";
    let response = match client.get(url).send() {
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
    let response_status = response.status().as_u16();

    let quotes: Vec<Quotes> = match response.json() {
        Ok(quotes) => quotes,
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
    let quote_text = quotes.get(0).map(|q| q.q.clone()).unwrap();
    Data {
        data: Some(quote_text),
        time: Utc::now(),
        error_message: None,
        response_code: response_status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dad_joke_api() {
        let data = fetch_dad_jokes();
        println!("{}", data);
    }

    #[test]
    fn test_quotes_api() {
        let data = fetch_quote();
        println!("{}", data);
    }
}
