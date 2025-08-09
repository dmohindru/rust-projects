use crate::cli::OutputFormat;
use crate::todo_repo::Todo;
use serde_json::{error::Error, from_str, to_string_pretty};
use std::io::Write;

pub struct TodoPrinter<W: Write> {
    writer: W,
}

impl<W: Write> TodoPrinter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn print_single_todo(&mut self, todo: Todo, format: OutputFormat) {
        todo!()
    }

    pub fn print_list_todo(&mut self, todo: Vec<Todo>, format: OutputFormat) {
        todo!()
    }

    #[cfg(test)]
    pub fn into_writer(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use nanoid::nanoid;

    use super::*;
    use std::io::Cursor;

    const ID_LENGTH: usize = 7;

    fn get_todo_single() -> Todo {
        Todo {
            id: nanoid!(ID_LENGTH),
            name: String::from("First Todo"),
            description: String::from("First Todo description"),
            completed: false,
        }
    }

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

    #[test]
    fn should_print_single_todo_in_text_format() {
        todo!()
    }

    #[test]
    fn should_print_single_todo_in_json_format() {
        todo!()
    }

    #[test]
    fn should_print_list_of_todos_in_text_format() {
        todo!()
    }

    #[test]
    fn should_print_list_of_todos_in_json_format() {
        todo!()
    }
}
