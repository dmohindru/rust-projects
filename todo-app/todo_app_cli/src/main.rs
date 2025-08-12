mod cli;
mod printer;
mod todo_repo;

use clap::Parser;
use cli::{Commands, OutputFormat, TodoCli};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Stdout};

use crate::cli::GetCommand;
use crate::printer::TodoPrinter;
use crate::todo_repo::TodoRepository;
fn main() {
    let cli = TodoCli::parse();

    let (reader, writer) = open_data_file("/tmp/todo.json");
    let mut todo_repo = TodoRepository::new(reader, writer);
    let mut todo_printer = TodoPrinter::<Stdout>::new(std::io::stdout());
    let output_format = cli.output.unwrap_or(OutputFormat::Text);
    // if let Some(output) = cli.output {
    //     println!("Printing output in format: {:?}", output);
    // } else {
    //     println!("Printing output in default format of text");
    // }

    match &cli.command {
        Commands::Get { get_command } => {
            //println!("Get command params is: {get_command:?}");
            handle_get_command(
                &mut todo_repo,
                &mut todo_printer,
                get_command,
                output_format,
            );
        }
        Commands::Add(add_args) => {
            println!("Add command params is: {add_args:?}")
        }
        Commands::Complete(complete_args) => {
            println!("Complete command params is: {complete_args:?}")
        }
        Commands::Delete(delete_args) => {
            println!("Delete command params is: {delete_args:?}")
        }
    }
}

fn open_data_file(file_path: &str) -> (BufReader<File>, BufWriter<File>) {
    // Open the file for reading (create if it doesn't exist)
    let reader_file = OpenOptions::new()
        .read(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file for reading");
    let reader = BufReader::new(reader_file);

    // Open the file for writing (overwrite, not append; create if it doesn't exist)
    let writer_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // This ensures the file is overwritten
        .open(file_path)
        .expect("Failed to open file for writing");
    let writer = BufWriter::new(writer_file);

    (reader, writer)
}

fn handle_get_command(
    todo_repo: &mut TodoRepository<BufReader<File>, BufWriter<File>>,
    todo_printer: &mut TodoPrinter<Stdout>,
    get_command: &GetCommand,
    output_format: OutputFormat,
) {
    match get_command {
        GetCommand::All => {
            let all_todos = todo_repo.get_all_todos().unwrap();
            todo_printer.print_list_todo(all_todos, output_format);
        }
        GetCommand::Id(todo_id_args) => {
            let todo_by_id = todo_repo.get_todo_by_id(String::from(&todo_id_args.todo_id));
        }
        GetCommand::Name(todo_name_args) => {
            let todo_by_name =
                todo_repo.get_todo_by_name(String::from(&todo_name_args.search_string));
        }
    }
}

fn handle_get_all(
    todo_repo: &mut TodoRepository<BufReader<File>, BufWriter<File>>,
    todo_printer: &mut TodoPrinter<Stdout>,
    output_format: OutputFormat,
) {
    match todo_repo.get_all_todos() {
        Ok(all_todos) => todo_printer.print_list_todo(all_todos, output_format),
        Err(e) => eprintln!("Error retrieving todos: {}", e.error_message()),
    }
}
