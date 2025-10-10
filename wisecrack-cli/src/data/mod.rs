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

#[derive(Clone)]
pub struct Data {
    pub data: Option<String>,
    pub time: DateTime<Utc>,
    pub error_message: Option<String>,
    pub response_code: u16,
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

pub fn fetch_dad_jokes(base_url: &str) -> Data {
    let client = Client::new();
    let response = match client
        .get(base_url)
        .header(ACCEPT, "application/json")
        .send()
    {
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

    if !response.status().is_success() {
        let code = response.status().as_u16();
        let body = response.text().unwrap_or_else(|_| "Unknown error".into());
        return Data {
            data: None,
            time: Utc::now(),
            error_message: Some(format!("HTTP error {}: {}", code, body)),
            response_code: code,
        };
    }

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

pub fn fetch_quote(base_url: &str) -> Data {
    let client = Client::new();
    let response = match client.get(base_url).send() {
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
    if !response.status().is_success() {
        let code = response.status().as_u16();
        let body = response.text().unwrap_or_else(|_| "Unknown error".into());
        return Data {
            data: None,
            time: Utc::now(),
            error_message: Some(format!("HTTP error {}: {}", code, body)),
            response_code: code,
        };
    }
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
    fn should_return_dad_joke_with_success() {
        let expected_joke = "Mocked joke for testing.";
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("GET", "/")
            .match_header("Accept", "application/json")
            .with_status(201)
            .with_body(format!(
                "{{\"id\":\"Dt4hNJJmykb\",\"joke\":\"{}\",\"status\":201}}",
                expected_joke
            ))
            .create();

        let data = fetch_dad_jokes(url.as_str());
        mock.assert();
        assert_eq!(data.response_code, 201);
        assert_eq!(data.data.as_deref(), Some(expected_joke));
        assert!(data.error_message.is_none());
    }

    #[test]
    fn should_return_error_from_dad_joke_api() {
        let error_body = r#"{"message": "Internal Server Error"}"#;
        let mut server = mockito::Server::new();
        let url = server.url();

        let mock = server
            .mock("GET", "/")
            .match_header("Accept", "application/json")
            .with_status(500)
            .with_body(error_body)
            .create();

        let data = fetch_dad_jokes(url.as_str());
        mock.assert();
        assert_eq!(data.response_code, 500, "Expected HTTP 500 status code");
        assert!(data.data.is_none(), "Expected no data for failed API call");
        assert!(
            data.error_message.is_some(),
            "Expected an error message for failed API call"
        );

        if let Some(err) = &data.error_message {
            assert!(
                err.contains("500") || err.contains("Internal Server Error"),
                "Error message should mention failure reason"
            );
        }
    }

    #[test]
    fn should_return_quotes_with_success() {
        let expected_quote = "Mocked quote for testing.";
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("GET", "/random")
            .with_status(200)
            .with_body(format!(
                "[{{\"a\":\"Some Author\",\"q\":\"{}\",\"h\":\"Some more text\"}}]",
                expected_quote
            ))
            .create();

        let data = fetch_quote(format!("{}/random", url.as_str()).as_str());
        mock.assert();
        assert_eq!(data.response_code, 200);
        assert_eq!(data.data.as_deref(), Some(expected_quote));
        assert!(data.error_message.is_none());
    }

    #[test]
    fn should_return_error_from_quotes_api() {
        let error_body = r#"{"message": "Internal Server Error"}"#;
        let mut server = mockito::Server::new();
        let url = server.url();

        let mock = server
            .mock("GET", "/random")
            .with_status(500)
            .with_body(error_body)
            .create();

        let data = fetch_quote(format!("{}/random", url.as_str()).as_str());
        mock.assert();
        assert_eq!(data.response_code, 500, "Expected HTTP 500 status code");
        assert!(data.data.is_none(), "Expected no data for failed API call");
        assert!(
            data.error_message.is_some(),
            "Expected an error message for failed API call"
        );

        if let Some(err) = &data.error_message {
            assert!(
                err.contains("500") || err.contains("Internal Server Error"),
                "Error message should mention failure reason"
            );
        }
    }
}
