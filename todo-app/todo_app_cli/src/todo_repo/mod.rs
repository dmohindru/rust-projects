use std::io::{Read, Write};

use crate::cli::AddCommandArgs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub completed: bool,
}

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
        todo!();
    }

    fn save_all(&mut self, all_todos: Vec<Todo>) -> Result<(), TodoErrors> {
        todo!();
    }

    pub fn get_all_todos(&mut self) -> Result<Vec<Todo>, TodoErrors> {
        todo!();
    }

    pub fn get_todo_by_id(&mut self, todo_id: String) -> Result<Todo, TodoErrors> {
        todo!();
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

    #[test]
    fn should_return_empty_todo_list_if_datafile_not_exist() {
        todo!();
    }

    #[test]
    fn should_return_non_empty_todo_list_if_datafile_exists() {
        todo!();
    }

    #[test]
    fn should_return_todo_by_id_when_present() {
        todo!()
    }

    #[test]
    fn should_return_err_with_message_when_todo_by_id_not_present() {
        todo!();
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
