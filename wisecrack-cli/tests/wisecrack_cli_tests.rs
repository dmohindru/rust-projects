use assert_cmd::Command;

#[test]
fn should_exit_with_success_code_for_dad_jokes_api_success_response() {
    let mut cmd = Command::cargo_bin("wisecrack").unwrap();
    cmd.arg("--joke");
    cmd.assert().success().code(0);
}

#[test]
fn should_exit_with_error_code_for_dad_jokes_api_error_response() {}

#[test]
fn should_exit_with_success_code_for_quotes_api_success_response() {}

#[test]
fn should_exit_with_error_code_for_quotes_api_error_response() {}
