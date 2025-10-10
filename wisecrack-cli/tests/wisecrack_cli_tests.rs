use assert_cmd::Command;
use mockito::Server;
use predicates::str::contains;
#[test]
fn should_exit_with_success_code_for_dad_jokes_api_success_response() {
    let expected_joke = "Mocked joke for testing.";
    let mut server = Server::new();
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
    let mut cmd = Command::cargo_bin("wisecrack").unwrap();
    cmd.env("WISECRACK_DAD_JOKE_API", &url).arg("--joke");
    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(expected_joke));
    mock.assert();
}

#[test]
fn should_exit_with_error_code_for_dad_jokes_api_error_response() {
    let error_body = r#"{"message": "Internal Server Error"}"#;
    let mut server = mockito::Server::new();
    let url = server.url();

    let mock = server
        .mock("GET", "/")
        .match_header("Accept", "application/json")
        .with_status(500)
        .with_body(error_body)
        .create();
    let mut cmd = Command::cargo_bin("wisecrack").unwrap();
    cmd.env("WISECRACK_DAD_JOKE_API", &url).arg("-j");
    cmd.assert().failure();
    mock.assert();
}

#[test]
fn should_exit_with_success_code_for_quotes_api_success_response() {
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

    let mut cmd = Command::cargo_bin("wisecrack").unwrap();
    cmd.env("WISECRACK_QUOTES_API", format!("{}/random", &url))
        .arg("--quote");
    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(expected_quote));
    mock.assert();
}

#[test]
fn should_exit_with_error_code_for_quotes_api_error_response() {
    let error_body = r#"{"message": "Internal Server Error"}"#;
    let mut server = mockito::Server::new();
    let url = server.url();

    let mock = server
        .mock("GET", "/random")
        .with_status(500)
        .with_body(error_body)
        .create();
    let mut cmd = Command::cargo_bin("wisecrack").unwrap();
    cmd.env("WISECRACK_QUOTES_API", format!("{}/random", &url))
        .arg("-q");
    cmd.assert().failure();
    mock.assert();
}
