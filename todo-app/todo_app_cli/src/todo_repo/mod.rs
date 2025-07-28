use std::{
    cmp::Ordering,
    io::{Read, Write},
};

use crate::cli::AddCommandArgs;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::{error::Error, from_str, to_string_pretty};

const ID_LENGTH: usize = 7;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, PartialEq)]
pub enum TodoErrors {
    TodoAddError(String),
    TodoDeleteError(String),
    TodoGetError(String),
    TodoUpdateError(String),
}

pub struct TodoRepository<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> TodoRepository<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    fn load_all(&mut self) -> Result<Vec<Todo>, TodoErrors> {
        let mut input = String::new();
        let read_result = self.reader.read_to_string(&mut input);
        match read_result {
            Err(read_error) => return Err(TodoErrors::TodoGetError(read_error.to_string())),
            _ => {}
        };

        let todos_parse_result: Result<Vec<Todo>, Error> = from_str(&input);
        match todos_parse_result {
            Ok(todos) => Ok(todos),
            Err(parse_error) => Err(TodoErrors::TodoGetError(parse_error.to_string())),
        }
    }

    fn save_all(&mut self, all_todos: Vec<Todo>) -> Result<(), TodoErrors> {
        todo!();
    }

    pub fn get_all_todos(&mut self) -> Result<Vec<Todo>, TodoErrors> {
        self.load_all()
    }

    pub fn get_todo_by_id(&mut self, todo_id: String) -> Result<Todo, TodoErrors> {
        let todos_result = self.load_all();
        let todos = match todos_result {
            Ok(todos) => todos,
            Err(todo_error) => return Err(todo_error),
        };

        match todos.iter().find(|todo| todo.id == todo_id) {
            Some(todo) => Ok(todo.clone()),
            None => Err(TodoErrors::TodoGetError(format!(
                "Todo by id:{} not found",
                todo_id
            ))),
        }
    }

    pub fn get_todo_by_name(&mut self, todo_name: String) -> Result<Todo, TodoErrors> {
        todo!();
    }

    pub fn add_todo(&mut self, add_command_args: &AddCommandArgs) -> Result<Todo, TodoErrors> {
        todo!();
    }

    pub fn delete_todo(&mut self, todo_id: &str) -> Result<Todo, TodoErrors> {
        todo!();
    }

    pub fn mark_todo_complete(&mut self, todo_id: &str) -> Result<Todo, TodoErrors> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Cursor, str::FromStr};

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
                description: String::from("First Todo description"),
                completed: false,
            },
            Todo {
                id: nanoid!(ID_LENGTH),
                name: String::from("Third Todo"),
                description: String::from("First Todo description"),
                completed: false,
            },
        ]
    }

    fn setup(todos: &Vec<Todo>) -> (Cursor<String>, Cursor<Vec<u8>>) {
        let input_str = to_string_pretty(todos).unwrap();
        let input_cursor = Cursor::new(input_str);
        let output_cursor = Cursor::new(Vec::<u8>::new());
        (input_cursor, output_cursor)
    }

    #[test]
    fn should_return_empty_todo_list_if_datafile_not_exist() {
        let empty_todos = Vec::<Todo>::new();
        let (input_cur, output_cur) = setup(&empty_todos);
        let mut todo_repository = TodoRepository::new(input_cur, output_cur);
        let todos = todo_repository.get_all_todos().unwrap();
        assert_eq!(&empty_todos, &todos);
    }

    #[test]
    fn should_return_non_empty_todo_list_if_datafile_exists() {
        let saved_todos = get_todo_list();
        let (input_cur, output_cur) = setup(&saved_todos);
        let mut todo_repository = TodoRepository::new(input_cur, output_cur);
        let todos = todo_repository.get_all_todos().unwrap();
        assert_eq!(&saved_todos, &todos);
    }

    #[test]
    fn should_return_todo_by_id_when_present() {
        let saved_todos = get_todo_list();
        let (input_cur, output_cur) = setup(&saved_todos);
        let mut todo_repository = TodoRepository::new(input_cur, output_cur);
        let second_todo = &saved_todos[1];
        let todo_id = String::from_str(&second_todo.id).unwrap();
        let todo_by_id = todo_repository.get_todo_by_id(todo_id).unwrap();
        assert_eq!(second_todo, &todo_by_id);
    }

    #[test]
    fn should_return_err_with_message_when_todo_by_id_not_present() {
        let not_present_id = nanoid!();
        let saved_todos = get_todo_list();
        let (input_cur, output_cur) = setup(&saved_todos);
        let mut todo_repository = TodoRepository::new(input_cur, output_cur);
        let get_result = todo_repository.get_todo_by_id(String::from(&not_present_id));
        assert!(
            matches!(get_result, Err(TodoErrors::TodoGetError(ref msg)) if msg.contains(&format!("Todo by id:{} not found", &not_present_id))
            )
        );
    }

    #[test]
    fn should_return_todo_by_name_when_present() {
        todo!()
    }

    #[test]
    fn should_return_err_with_message_when_todo_by_name_not_present() {
        todo!();
    }

    #[test]
    fn should_return_new_added_todo_to_datafile() {
        todo!();
    }

    #[test]
    fn should_return_err_with_message_when_add_todo_fails() {
        todo!();
    }

    #[test]
    fn should_return_deleted_todo_by_id_when_present() {
        todo!();
    }

    #[test]
    fn should_return_err_with_message_when_delete_todo_by_id_not_present() {
        todo!();
    }

    #[test]
    fn should_mark_todo_completed_and_return_by_it_when_present() {
        todo!();
    }

    #[test]
    fn should_return_err_with_message_when_mark_todo_complete_is_absent() {
        todo!();
    }
}
