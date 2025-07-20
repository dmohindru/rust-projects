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
- **Supports Unix-style CLI design**:
  - `--output` format (JSON or plain text)
  - Read from piped input (stdin)
  - Output designed for use in pipes and scripts

## Tech Stack

- [clap](https://crates.io/crates/clap) for command-line parsing
- [serde_json](https://crates.io/crates/serde_json) for JSON backend storage
- [colored](https://crates.io/crates/colored) for colorized output
- [mockall](https://crates.io/crates/mockall) for mocking in tests
- [atty](https://crates.io/crates/atty) to detect interactive input/output

## Usage

### Get all todos

```sh
todo get all
```

### Get todo by ID

```sh
todo get id <todo_id>
echo 123 | todo get id
```

### Get todo by name (returns all matching)

```sh
todo get name <search_string>
echo "shopping" | todo get name
```

### Add a todo (via CLI flags)

```sh
todo add --name <todo_name> --description <todo_description>
```

### Add a todo (via piped JSON)

```sh
echo '{"name": "Buy milk", "description": "From Aldi"}' | todo add
```

### Add todos from file

```sh
todo add < todo.json
```

### Mark todo as complete (by ID)

```sh
todo complete <todo_id>
echo 123 | todo complete
```

### Delete a todo (by ID)

```sh
todo delete <todo_id>
echo 123 | todo delete
```

## Output Format

You can choose between human-readable text and machine-friendly JSON output using the `--output` (or `-o`) flag.

```sh
todo get all --output json
todo get id 123 -o text
```

- `text` (default): easy-to-read, line-based format, suitable for tools like `grep` or `awk`.
- `json`: structured output for scripting or further processing.

Example:

```sh
todo get all -o text | grep done=false
```

## Storage

- Todos are stored in a JSON file next to the executable for simplicity.

## Architecture

- **Argument Parser/Validator:** Parses and validates CLI arguments, creates DTOs for the service layer.
- **Service Module:** Handles business logic, interacts with repository, formats todos for printing, prepares summaries.
- **Repository Module:** Persists todos to disk (JSON file), handles save/update/delete operations.
- **Formatter Module:** Handles output formatting in either JSON or greppable text form.

## Testing

- Uses TDD approach.
- Mocks with `mockall` for isolated unit tests.

## Unix Philosophy

This CLI tool follows the Unix philosophy:

> "Expect the output of every program to become the input to another, as yet unknown, program. Don't clutter output with extraneous information. Avoid stringently columnar or binary input formats. Don't insist on interactive input."

### Adhered Practices

- Outputs can be piped to `grep`, `jq`, `awk`, etc.
- Supports machine-readable JSON output (`--output json`)
- Accepts JSON or ID inputs via `stdin` (pipe or redirection)
- All commands are non-interactive and script-friendly

### Supported Commands via `stdin`

| Command    | Description       | Example         |
| ---------- | ----------------- | --------------- | -------------- |
| `add`      | JSON object input | `echo '{{...}}' | todo add`      |
| `get id`   | ID from stdin     | `echo 123       | todo get id`   |
| `get name` | Name from stdin   | `echo "milk"    | todo get name` |
| `complete` | ID from stdin     | `echo 123       | todo complete` |
| `delete`   | ID from stdin     | `echo 123       | todo delete`   |

### Caveats

- **Don't mix CLI args and stdin for same value**: For example, avoid `todo delete 123` and `echo 123 | todo delete` together. Pick one input method.
- **Batch input** (e.g. multiple IDs): Handle line-by-line processing from `stdin`. Make sure your code skips/handles malformed lines cleanly.
- **Per-line error reporting**: Especially for batch operations like `delete`, errors should not abort the whole operation.
- **Avoid interactive input**: Never prompt the user via `stdin`; everything should be scriptable.

### Example Use in Unix Pipelines

```sh
# Get only completed todos
todo get all -o text | grep done=true

# Add todo from another script
cat new_todo.json | todo add

# Complete todos from a list of IDs
cat ids.txt | todo complete

# Delete todos based on a filter
todo get all -o json | jq '.[] | select(.done == true) | .id' | todo delete
```

---
