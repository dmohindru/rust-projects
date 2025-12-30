# 1️⃣ Bitmap Font Generation

## US-1: Create a bitmap for a single character

As a CLI user

- I want to generate a bitmap for a single ASCII character
- So that I can display text on an LED matrix without manually designing pixels.

**Acceptance Criteria**

- User specifies:
  - character ('A', 'z', '?', '!', etc.)
  - grid size (5x5 or 8x8). More sizes to come later.
- Output is a binary bitmap matching the grid size
- Only printable ASCII characters are accepted
- Unsupported characters fail with a clear error

## US-2: Generate a full font set

As a CLI user

- I want to generate bitmaps for all printable ASCII characters
- So that I can embed a complete font table in firmware.

**Acceptance Criteria**

- Generates bitmaps for ASCII range 0x20–0x7E
- All characters share the same grid size
- Output is grouped in a structured way (e.g. array or map)

# 2️⃣ Bitmap Size & Constraints

## US-3: Control bitmap grid size

As a CLI user
I want to specify the bitmap grid size.

So that I can target different LED matrices.

**Acceptance Criteria**

- Grid sizes from 5x5 and 8x8 more for later to come.
- Width and height may be equal (square) for MVP.
- Invalid sizes fail early with clear messaging.

## US-4: Consistent glyph alignment

As a CLI user

- I want glyphs to be aligned consistently within the grid
- So that scrolling text does not visually jitter.

**Acceptance Criteria**

- All glyphs:
  - share a baseline
  - have consistent left/right padding rules
- Spacing rules are deterministic and documented

# 3️⃣ Scrolling Text Animation

## US-5: Generate scrolling text frames

As a CLI user

- I want to generate animation frames for scrolling text
- So that I can display messages on an LED matrix.

**Acceptance Criteria**

- User provides:
  - text string
  - font size (grid size)
  - scroll direction (left → right minimum)
- Output is an ordered list of frames
- Each frame is a bitmap matching the target matrix size

## US-6: Control spacing between characters

As a CLI user

- I want to control spacing between characters
- So that scrolling text is readable.

**Acceptance Criteria**

- User can specify:
  - minimum inter-character spacing (in pixels)
- Default spacing exists if not provided

# 4️⃣ Output Formats (Critical for MVP)

## US-7: Export Rust-friendly output

As a Rust firmware developer

- I want to export bitmaps in Rust syntax
- So that I can copy-paste directly into my firmware.

**Acceptance Criteria**

- Output supports:
  - const arrays
  - frame arrays for animations
- No post-processing required by user

## US-8: Export raw binary or hex

As a firmware developer

- I want to export bitmaps as raw bytes
- So that I can store them in flash efficiently.

**Acceptance Criteria**

- Output as:
  - hex bytes
  - bit-packed rows or columns (documented)
- Endianness is explicit

# 5️⃣ CLI Usability

## US-9: Predictable CLI interface

As a CLI user

- I want clear commands and flags
- So that I can script this tool.

**Acceptance Criteria**

- Supports --help
- Commands are composable and script-friendly
- No interactive mode required for MVP

## US-10: Deterministic output

As a CI user

- I want deterministic output for the same inputs
- So that I can commit generated assets to git.

**Acceptance Criteria**

- Same inputs → byte-for-byte identical output
- No randomness unless explicitly requested
