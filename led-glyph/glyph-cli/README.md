# Introduction

This is the CLI application that generates glyph bitmap binary data file suitable for rendering for small led matrix like 5X5, 8X8 etc

# Usage

Use `glyph-cli` to generate binary glyph bitmaps for single characters or strings, or to print info about an existing glyph binary file.

## Options

- `-c`, `--char <CHAR>`
  - Generate bitmap for a single character (e.g., `-c A`).
- `-s`, `--string <STRING>`
  - Generate bitmap data for a string (requires `--mode`).
- `-m`, `--mode <scroll|next>`
  - Mode to render strings: `scroll` (continuous scrolling) or `next` (advance by character frames).
- `-i`, `--info`
  - Print information about an existing glyph binary file.
- `-g`, `--grid-size <N>`
  - Specify square grid size (e.g., `5`, `8`). **Required** for generation and info commands.
- `-f`, `--file <FILE>`
  - File path to write output data (for `-c` or `-s`) or to read input file (for `-i`).
- `-h`, `--help`
  - Show help information (provided by the CLI framework).
- `-V`, `--version`
  - Show version information (provided by the CLI framework).

## Permitted option combinations

- Single character generation: `-c -g -f`
- String generation: `-s -m -g -f` (you must provide `--mode` when using `--string`)
- Print file info: `-i -g -f`

## Examples

- Generate character 'A' on a 5x5 grid and write to `a_5x5.bin`:

  ```sh
  glyph-cli -c A -g 5 -f a_5x5.bin
  ```

- Generate a scrolling string "HI" on an 8x8 grid:

  ```sh
  glyph-cli -s "HI" -m scroll -g 8 -f hi_8x8.bin
  ```

- Print information about a glyph binary file:

  ```sh
  glyph-cli -i -g 5 -f hi_5x5.bin
  ```

## Notes

- `--grid-size` is mandatory for generation and for printing file info.
- `--mode` is required when using `--string`.
- The generated files are binary glyph data suitable for small LED matrix renderers.

## Build & Run

Build the CLI and run directly with Cargo:

```sh
cargo build --bin glyph-cli
cargo run --bin glyph-cli -- -c A -g 5 -f out.bin
```
