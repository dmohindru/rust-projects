use std::io::{Read, Write};

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
    TodoSaveError(String),
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
        let output_result = to_string_pretty(&all_todos);
        let output = match output_result {
            Ok(output) => output,
            Err(pretty_print_error) => {
                return Err(TodoErrors::TodoSaveError(pretty_print_error.to_string()));
            }
        };

        match self.writer.write_all(output.as_bytes()) {
            Ok(_) => Ok(()),
            Err(save_error) => Err(TodoErrors::TodoSaveError(save_error.to_string())),
        }
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

    pub fn get_todo_by_name(&mut self, todo_name: String) -> Result<Vec<Todo>, TodoErrors> {
        let todos_result = self.load_all();
        let todos = match todos_result {
            Ok(todos) => todos,
            Err(todo_error) => return Err(todo_error),
        };

        let todo_name_lowercase = todo_name.to_lowercase();
        let found_todos: Vec<Todo> = todos
            .iter()
            .filter(|todo| todo.name.to_lowercase().contains(&todo_name_lowercase))
            .cloned()
            .collect();

        if found_todos.is_empty() {
            Err(TodoErrors::TodoGetError(format!(
                "Todo by name: {} not found",
                todo_name
            )))
        } else {
            Ok(found_todos)
        }
    }

    pub fn add_todo(&mut self, add_command_args: &AddCommandArgs) -> Result<Todo, TodoErrors> {
        let todos_result = self.load_all();
        let mut todos = match todos_result {
            Ok(todos) => todos,
            Err(todo_error) => return Err(todo_error),
        };

        let new_todo = Todo {
            id: nanoid!(),
            name: String::from(&add_command_args.name),
            description: String::from(&add_command_args.description),
            completed: false,
        };
        let todo_to_return = new_todo.clone();
        todos.push(new_todo);

        match self.save_all(todos) {
            Ok(_) => Ok(todo_to_return),
            Err(save_error) => Err(save_error),
        }
    }

    pub fn delete_todo(&mut self, todo_id: &str) -> Result<Todo, TodoErrors> {
        todo!();
    }

    pub fn mark_todo_complete(&mut self, todo_id: &str) -> Result<Todo, TodoErrors> {
        todo!();
    }

    #[cfg(test)]
    pub fn into_writer(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use crate::{cli::add, todo_repo};

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
    fn should_return_single_todo_by_name_when_present() {
        let saved_todos = get_todo_list();
        let (input_cur, output_cur) = setup(&saved_todos);
        let mut todo_repo = TodoRepository::new(input_cur, output_cur);
        let found_todos = todo_repo.get_todo_by_name(String::from("first")).unwrap();
        let expected_todo_list = vec![saved_todos[0].clone()];
        assert_eq!(expected_todo_list, found_todos);
    }

    #[test]
    fn should_return_multiple_todos_by_name_when_present() {
        let saved_todos = get_todo_list();
        let (input_cur, output_cur) = setup(&saved_todos);
        let mut todo_repo = TodoRepository::new(input_cur, output_cur);
        let found_todos = todo_repo.get_todo_by_name(String::from("todo")).unwrap();
        assert_eq!(saved_todos, found_todos);
    }

    #[test]
    fn should_return_err_with_message_when_todo_by_name_not_present() {
        let saved_todos = get_todo_list();
        let todo_search_str = "some random";
        let (input_cur, output_cur) = setup(&saved_todos);
        let mut todo_repo = TodoRepository::new(input_cur, output_cur);
        let find_result = todo_repo.get_todo_by_name(String::from_str(todo_search_str).unwrap());
        assert!(
            matches!(find_result, Err(TodoErrors::TodoGetError(ref msg)) if msg.contains(&format!("Todo by name: {} not found", todo_search_str)))
        )
    }

    #[test]
    fn should_return_new_added_todo_to_datafile() {
        let saved_todos = get_todo_list();
        let (input_cur, output_cur) = setup(&saved_todos);
        let mut todo_repo = TodoRepository::new(input_cur, output_cur);
        let add_command_args = AddCommandArgs {
            name: String::from("New Todo"),
            description: String::from("New Todo Description"),
        };
        let added_todo = todo_repo.add_todo(&add_command_args).unwrap();

        assert_eq!(&add_command_args.name, &added_todo.name);
        assert_eq!(&add_command_args.description, &added_todo.description);
        assert_eq!(&false, &added_todo.completed);

        // Convert written data back to string
        let output_bytes = todo_repo.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();

        // Deserialize for assertion
        let updated_todos: Vec<Todo> = from_str(&output_str).unwrap();
        let mut expected_todos = saved_todos.clone();
        expected_todos.push(added_todo);
        let test_pair: Vec<_> = expected_todos
            .into_iter()
            .zip(updated_todos.into_iter())
            .collect();

        test_pair.iter().for_each(|pair| {
            assert_eq!(pair.0.name, pair.1.name);
            assert_eq!(pair.0.description, pair.1.description);
            assert_eq!(pair.0.completed, pair.1.completed);
        });
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
