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
    TodoAddError,
    TodoGetError,
    TodoDeleteError,
    TodoUpdateError,
}

fn load_todo(file_path: &str) -> Result<Vec<Todo>, TodoErrors> {
    todo!();
}

fn add_todo(file_path: &str, add_command_args: &AddCommandArgs) -> Result<Todo, TodoErrors> {
    todo!();
}

fn delete_todo(file_path: &str, todo_id: &str) -> Result<Todo, TodoErrors> {
    todo!();
}

fn mark_todo_complete(file_path: &str, todo_id: &str) -> Result<Todo, TodoErrors> {
    todo!();
}
