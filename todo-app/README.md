# Rust Todo CLI App

A command-line application to manage your todos, built in Rust.

## Features

- Create a todo
- Get all todos
- Get a single todo by ID or name
- Mark todo as completed
- Delete a todo
- Color-based console printing (red for pending, green for completed)
- TDD approach with mocks

## Tech Stack

- [clap](https://crates.io/crates/clap) for command-line parsing
- [serde_json](https://crates.io/crates/serde_json) for JSON backend storage
- [colored](https://crates.io/crates/colored) for colorized output
- [mockall](https://crates.io/crates/mockall) for mocking in tests

## Usage

### Get all todos

```sh
todo get all
```

### Get todo by ID

```sh
todo get id <todo_id>
```

### Get todo by name (returns all matching)

```sh
todo get name <search_string>
```

### Add a todo

```sh
todo add --name <todo_name> --description <todo_description>
```

### Mark todo as complete

```sh
todo complete <todo_id>
```

### Delete a todo

```sh
todo delete <todo_id>
```

## Storage

- Todos are stored in a JSON file next to the executable for simplicity.

## Architecture

- **Argument Parser/Validator:** Parses and validates CLI arguments, creates DTOs for the service layer.
- **Service Module:** Handles business logic, interacts with repository, formats todos for printing, prepares summaries.
- **Repository Module:** Persists todos to disk (JSON file), handles save/update/delete operations.

## Testing

- Uses TDD approach.
- Mocks with `mockall` for isolated unit tests.

---
