use assert_cmd::Command;
use nanoid::nanoid;
use predicates::str::contains;
use serde_json::{from_str, to_string_pretty};
use std::fs::{read, write};
use tempfile::NamedTempFile;
use todo::todo_repo::Todo;

const ID_LENGTH: usize = 7;

fn get_todo_list() -> Vec<Todo> {
    vec![
        Todo {
            id: nanoid!(ID_LENGTH),
            name: String::from("First Todo"),
            description: String::from("First Todo description"),
            completed: false,
        },
        Todo {
            id: nanoid!(ID_LENGTH),
            name: String::from("Second Todo"),
            description: String::from("Second Todo description"),
            completed: false,
        },
        Todo {
            id: nanoid!(ID_LENGTH),
            name: String::from("Third Todo"),
            description: String::from("Third Todo description"),
            completed: false,
        },
    ]
}

fn setup(todo_list: &Vec<Todo>) -> NamedTempFile {
    let tempfile = NamedTempFile::new().expect("failed to create temp file");
    let data = to_string_pretty(todo_list).unwrap();
    write(tempfile.path(), data).unwrap();
    tempfile
}

#[test]
fn should_exit_with_non_zero_exit_code_for_invalid_data_file() {
    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg("/tmp/not-existence-file.json")
        .arg("get all");

    cmd.assert().failure();
}

#[test]
fn get_all_should_return_all_todos_with_zero_exit_code() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file").arg(path).arg("get").arg("all");

    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(todos[0].name.as_str()))
        .stdout(contains(todos[1].name.as_str()))
        .stdout(contains(todos[2].name.as_str()));
}

#[test]
fn get_id_should_return_todo_if_present_with_zero_exit_code() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("get")
        .arg("id")
        .arg(todos[1].id.as_str());

    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(todos[1].name.as_str()));
}

#[test]
fn get_id_should_exit_with_non_zero_exit_code_when_todo_not_present() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();
    let random_id = "Some-Random-Id";

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("get")
        .arg("id")
        .arg(random_id);

    cmd.assert()
        .failure()
        .code(1)
        .stderr(contains("Error retrieving todo by id"));
}

#[test]
fn get_name_should_return_todo_if_present_with_zero_exit_code() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("get")
        .arg("name")
        .arg("Todo");

    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(todos[0].name.as_str()))
        .stdout(contains(todos[1].name.as_str()))
        .stdout(contains(todos[2].name.as_str()));
}

#[test]
fn get_name_should_exit_with_non_zero_exit_code_when_todo_not_present() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("get")
        .arg("name")
        .arg("Some-Random-Name");

    cmd.assert()
        .failure()
        .code(1)
        .stderr(contains("Error retrieving todo by name"));
}

#[test]
fn add_todo_should_return_new_todo_with_zero_exit_code() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();
    let new_todo_name = "New Todo";
    let new_todo_description = "New Todo Description";

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("add")
        .arg("--name")
        .arg(new_todo_name)
        .arg("--description")
        .arg(new_todo_description);

    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(new_todo_name));

    let file_contents: Vec<u8> = read(path).unwrap();
    let text_content = String::from_utf8_lossy(&file_contents);
    let updated_todo_list: Vec<Todo> = from_str(&text_content).unwrap();
    let result = updated_todo_list
        .iter()
        .any(|todo| todo.name == new_todo_name);

    assert!(result);
}

#[test]
fn complete_todo_should_mark_todo_complete_with_zero_exit_code() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("complete")
        .arg("--id")
        .arg(todos[0].id.as_str());

    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(todos[0].name.as_str()));

    let file_contents: Vec<u8> = read(path).unwrap();
    let text_content = String::from_utf8_lossy(&file_contents);
    let updated_todo_list: Vec<Todo> = from_str(&text_content).unwrap();
    let result = updated_todo_list
        .iter()
        .any(|todo| todo.id == todos[0].id && todo.completed);

    assert!(result);
}

#[test]
fn complete_todo_should_exit_with_non_zero_exit_code_when_todo_not_present() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("complete")
        .arg("--id")
        .arg("Some-Random-Id");

    cmd.assert()
        .failure()
        .code(1)
        .stderr(contains("Unable to mark todo completed"));
}

#[test]
fn delete_todo_should_delete_todo_with_zero_exit_code() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("delete")
        .arg("--id")
        .arg(todos[0].id.as_str());

    cmd.assert()
        .success()
        .code(0)
        .stdout(contains(todos[0].name.as_str()));

    let file_contents: Vec<u8> = read(path).unwrap();
    let text_content = String::from_utf8_lossy(&file_contents);
    let updated_todo_list: Vec<Todo> = from_str(&text_content).unwrap();
    let result = updated_todo_list.iter().any(|todo| todo.id == todos[0].id);

    assert!(!result);
}

#[test]
fn delete_todo_should_exit_with_non_zero_exit_code_when_todo_not_present() {
    let todos = get_todo_list();
    let tempfile = setup(&todos);
    let path = tempfile.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("--file")
        .arg(path)
        .arg("delete")
        .arg("--id")
        .arg("Some-Random-Id");

    cmd.assert()
        .failure()
        .code(1)
        .stderr(contains("Unable to delete todo"));
}
