use assert_cmd::Command;
use nanoid::nanoid;
use serde_json::to_string_pretty;
use std::fs::write;
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
fn setup() -> NamedTempFile {
    let tempfile = NamedTempFile::new().expect("failed to create temp file");
    let data = to_string_pretty(&get_todo_list()).unwrap();
    write(tempfile.path(), data).unwrap();
    tempfile
}

#[test]
fn should_exit_with_non_zero_exit_code_for_invalid_data_file() {
    todo!()
}

#[test]
fn get_all_should_return_all_todos_with_zero_exit_code() {
    todo!()
}

#[test]
fn get_id_should_return_todo_if_present_with_zero_exit_code() {
    todo!()
}

#[test]
fn get_id_should_exit_with_non_zero_exit_code_when_todo_not_present() {
    todo!()
}

#[test]
fn get_name_should_return_todo_if_present_with_zero_exit_code() {
    todo!()
}

#[test]
fn get_name_should_exit_with_non_zero_exit_code_when_todo_not_present() {}

#[test]
fn add_todo_should_return_new_todo_with_zero_exit_code() {
    todo!()
}

#[test]
fn complete_todo_should_mark_todo_complete_with_zero_exit_code() {
    todo!()
}

#[test]
fn complete_todo_should_exit_with_non_zero_exit_code_when_todo_not_present() {
    todo!()
}

#[test]
fn delete_todo_should_delete_todo_with_zero_exit_code() {
    todo!()
}

#[test]
fn delete_todo_should_exit_with_non_zero_exit_code_when_todo_not_present() {
    todo!()
}
