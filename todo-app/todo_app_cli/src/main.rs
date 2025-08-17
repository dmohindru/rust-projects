mod cli;
mod printer;
mod todo_repo;

use clap::Parser;
use cli::{
    AddCommandArgs, Commands, CompleteCommandArgs, DeleteCommandArgs, OutputFormat, TodoCli,
};
use dirs::home_dir;
use std::io::Stdout;

use crate::cli::GetCommand;
use crate::printer::TodoPrinter;
use crate::todo_repo::{FileDataAccess, TodoRepository};

// TODO: Implementing reading data from piped input from other programs
// TODO: Write integration tests
fn main() {
    let cli = TodoCli::parse();
    let path = match cli.file {
        Some(file_path) => file_path,
        None => {
            let mut default_path = home_dir().expect("Could not find home directory");
            default_path.push("tmp");
            std::fs::create_dir_all(&default_path).expect("Failed to create tmp directory in home");
            default_path.push("todo.json");
            String::from(default_path.to_str().unwrap())
        }
    };
    let file_data_access = FileDataAccess::new(path);
    let mut todo_repo = TodoRepository::new(file_data_access);
    let mut todo_printer = TodoPrinter::<Stdout>::new(std::io::stdout());
    let output_format = cli.output.unwrap_or(OutputFormat::Text);

    match &cli.command {
        Commands::Get { get_command } => {
            handle_get_command(
                &mut todo_repo,
                &mut todo_printer,
                get_command,
                output_format,
            );
        }
        Commands::Add(add_args) => {
            handle_add_command(&mut todo_repo, &mut todo_printer, add_args, output_format);
        }
        Commands::Complete(complete_args) => {
            handle_complete_command(
                &mut todo_repo,
                &mut todo_printer,
                complete_args,
                output_format,
            );
        }
        Commands::Delete(delete_args) => {
            handle_delete_command(
                &mut todo_repo,
                &mut todo_printer,
                delete_args,
                output_format,
            );
        }
    }
}

fn handle_get_command(
    todo_repo: &mut TodoRepository<FileDataAccess>,
    todo_printer: &mut TodoPrinter<Stdout>,
    get_command: &GetCommand,
    output_format: OutputFormat,
) {
    match get_command {
        GetCommand::All => {
            handle_get_all(todo_repo, todo_printer, output_format);
        }
        GetCommand::Id(todo_id_args) => {
            handle_get_todo_by_id(
                todo_repo,
                todo_printer,
                String::from(&todo_id_args.todo_id),
                output_format,
            );
        }
        GetCommand::Name(todo_name_args) => {
            handle_get_todo_by_name(
                todo_repo,
                todo_printer,
                String::from(&todo_name_args.search_string),
                output_format,
            );
        }
    }
}

fn handle_get_all(
    todo_repo: &mut TodoRepository<FileDataAccess>,
    todo_printer: &mut TodoPrinter<Stdout>,
    output_format: OutputFormat,
) {
    match todo_repo.get_all_todos() {
        Ok(all_todos) => todo_printer.print_list_todo(all_todos, output_format),
        Err(e) => eprintln!("Error retrieving todos: {}", e.error_message()),
    }
}

fn handle_get_todo_by_id(
    todo_repo: &mut TodoRepository<FileDataAccess>,
    todo_printer: &mut TodoPrinter<Stdout>,
    todo_id: String,
    output_format: OutputFormat,
) {
    match todo_repo.get_todo_by_id(todo_id) {
        Ok(todo) => todo_printer.print_single_todo(todo, output_format),
        Err(e) => eprintln!("Error retrieving todo by id: {}", e.error_message()),
    }
}

fn handle_get_todo_by_name(
    todo_repo: &mut TodoRepository<FileDataAccess>,
    todo_printer: &mut TodoPrinter<Stdout>,
    todo_name: String,
    output_format: OutputFormat,
) {
    match todo_repo.get_todo_by_name(todo_name) {
        Ok(all_todo) => todo_printer.print_list_todo(all_todo, output_format),
        Err(e) => eprintln!("Error retrieving todo by id: {}", e.error_message()),
    }
}

fn handle_add_command(
    todo_repo: &mut TodoRepository<FileDataAccess>,
    todo_printer: &mut TodoPrinter<Stdout>,
    add_command_args: &AddCommandArgs,
    output_format: OutputFormat,
) {
    match todo_repo.add_todo(add_command_args) {
        Ok(todo) => {
            todo_printer.print_single_todo(todo, output_format);
        }
        Err(e) => {
            eprintln!("Unable to add a todo: {}", e.error_message())
        }
    }
}

fn handle_complete_command(
    todo_repo: &mut TodoRepository<FileDataAccess>,
    todo_printer: &mut TodoPrinter<Stdout>,
    complete_command_args: &CompleteCommandArgs,
    output_format: OutputFormat,
) {
    match todo_repo.mark_todo_complete(String::from(&complete_command_args.id)) {
        Ok(todo) => todo_printer.print_single_todo(todo, output_format),
        Err(e) => {
            eprintln!("Unable to mark todo completed: {}", e.error_message())
        }
    }
}

fn handle_delete_command(
    todo_repo: &mut TodoRepository<FileDataAccess>,
    todo_printer: &mut TodoPrinter<Stdout>,
    delete_command_args: &DeleteCommandArgs,
    output_format: OutputFormat,
) {
    match todo_repo.delete_todo(String::from(&delete_command_args.id)) {
        Ok(todo) => todo_printer.print_single_todo(todo, output_format),
        Err(e) => {
            eprintln!("Unable to delete todo: {}", e.error_message())
        }
    }
}
