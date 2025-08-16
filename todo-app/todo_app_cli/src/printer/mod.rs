use crate::cli::OutputFormat;
use crate::todo_repo::Todo;
use serde_json::to_string_pretty;
use std::io::Write;

pub struct TodoPrinter<W: Write> {
    writer: W,
}

impl<W: Write> TodoPrinter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn print_single_todo(&mut self, todo: Todo, format: OutputFormat) {
        let output_str = match format {
            OutputFormat::Text => Self::get_todo_text_format(todo),
            OutputFormat::Json => to_string_pretty(&todo).unwrap(),
        };
        writeln!(self.writer, "{}", output_str).unwrap();
    }

    pub fn print_list_todo(&mut self, todo_list: Vec<Todo>, format: OutputFormat) {
        let output_str = match format {
            OutputFormat::Text => todo_list
                .into_iter()
                .map(|todo| Self::get_todo_text_format(todo))
                .collect::<Vec<_>>()
                .join("\n"),
            OutputFormat::Json => to_string_pretty(&todo_list).unwrap(),
        };
        writeln!(self.writer, "{}", output_str).unwrap();
    }

    fn first_10_chars(s: &String) -> String {
        s.chars().take(10).collect()
    }

    fn get_todo_text_format(todo: Todo) -> String {
        // expected format
        // id done title description
        format!(
            "{}\t{}\t{}\t\t{}",
            todo.id,
            todo.completed,
            Self::first_10_chars(&todo.name),
            todo.description
        )
    }

    #[cfg(test)]
    pub fn into_writer(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nanoid::nanoid;
    use std::io::Cursor;

    const ID_LENGTH: usize = 7;

    fn get_todo_single() -> Todo {
        Todo {
            id: nanoid!(ID_LENGTH),
            name: String::from("First Todo Long Title"),
            description: String::from("First Todo very long description"),
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

    fn setup() -> Cursor<Vec<u8>> {
        Cursor::new(Vec::<u8>::new())
    }

    fn first_10_chars(s: &String) -> String {
        s.chars().take(10).collect()
    }

    fn get_expected_text_format(todo: Todo) -> String {
        // expected format
        // id done title description
        format!(
            "{}\t{}\t{}\t\t{}",
            todo.id,
            todo.completed,
            first_10_chars(&todo.name),
            todo.description
        )
    }

    #[test]
    fn should_print_single_todo_in_text_format() {
        let output_cur = setup();
        let mut printer = TodoPrinter::new(output_cur);
        let single_todo = get_todo_single();
        printer.print_single_todo(single_todo.clone(), OutputFormat::Text);

        // Convert written data back to string
        let output_bytes = printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();

        let expected_output = format!("{}\n", get_expected_text_format(single_todo));

        assert_eq!(expected_output, output_str);
    }

    #[test]
    fn should_print_single_todo_in_json_format() {
        let output_cur = setup();
        let mut printer = TodoPrinter::new(output_cur);
        let single_todo = get_todo_single();
        printer.print_single_todo(single_todo.clone(), OutputFormat::Json);

        // Convert written data back to string
        let output_bytes = printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();

        let expected_output = format!("{}\n", to_string_pretty(&single_todo).unwrap());

        assert_eq!(expected_output, output_str);
    }

    #[test]
    fn should_print_list_of_todos_in_text_format() {
        let output_cur = setup();
        let mut printer = TodoPrinter::new(output_cur);
        let list_todo = get_todo_list();
        printer.print_list_todo(list_todo.clone(), OutputFormat::Text);

        // Convert written data back to string
        let output_bytes = printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();

        let list_output = list_todo
            .into_iter()
            .map(|todo| get_expected_text_format(todo))
            .collect::<Vec<_>>()
            .join("\n");
        let expected_output = format!("{}\n", list_output);

        assert_eq!(expected_output, output_str);
    }

    #[test]
    fn should_print_list_of_todos_in_json_format() {
        let output_cur = setup();
        let mut printer = TodoPrinter::new(output_cur);
        let list_todo = get_todo_list();
        printer.print_list_todo(list_todo.clone(), OutputFormat::Json);

        // Convert written data back to string
        let output_bytes = printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();

        let expected_output = format!("{}\n", to_string_pretty(&list_todo).unwrap());

        assert_eq!(expected_output, output_str);
    }
}
