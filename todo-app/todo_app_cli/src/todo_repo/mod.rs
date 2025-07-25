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

    fn get_all_todos(&mut self) -> Result<Vec<Todo>, TodoErrors> {
        todo!();
    }

    fn get_todo_by_id(&mut self, todo_id: String) -> Result<Todo, TodoErrors> {
        todo!();
    }

    fn get_todo_by_name(&mut self, todo_name: String) -> Result<Todo, TodoErrors> {
        todo!();
    }

    fn add_todo(&mut self, add_command_args: &AddCommandArgs) -> Result<Todo, TodoErrors> {
        todo!();
    }

    fn delete_todo(&mut self, todo_id: &str) -> Result<Todo, TodoErrors> {
        todo!();
    }

    fn mark_todo_complete(&mut self, todo_id: &str) -> Result<Todo, TodoErrors> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
