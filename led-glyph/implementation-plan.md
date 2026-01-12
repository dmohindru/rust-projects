# Summary: Glyph / LED Matrix Asset Pipeline Design

## 1️⃣ Code Organization: lib + client

**Workspace layout**

- Use a Cargo workspace with:
  - glyph-lib → core domain logic
  - glyph-cli → user-facing CLI

This separation ensures:

- The core logic is reusable (CLI, future web UI, build scripts)
- The CLI remains thin and replaceable

```text
glyph-workspace/
├── glyph-lib/
│   └── src/
├── glyph-cli/
│   └── src/
└── Cargo.toml (workspace)

```

## 2️⃣ Ownership of Logic: Library vs Client

**Client (CLI) Responsibilities**

**Structural / IO-facing concerns**

- Reading files (JSON, paths, flags)
- Deserializing JSON into DTOs
- Basic structural validation:
  - Correct JSON shape
  - Strings contain only '0' and '1'
  - Each row has consistent width
- Reporting user-friendly errors
- Writing output files (.bin, optional helpers)

**Library Responsibilities**
Domain / semantic logic

- Validating glyph semantics:
  - Consistent dimensions across glyphs
  - Supported glyph sizes (e.g., 5×5, 8×8)
- Converting glyphs into internal representations
- Laying out text strings using glyphs
- Generating animation frames (e.g., scrolling)
- Packing bits into binary frame format
- Producing Vec<u8> for output

**Key principle:**

> The library should never know where data came from or where it’s written.

## 3️⃣ JSON Format With Metadata (Recommended)

**High-level structure**

```json
{
  "meta": {
    "name": "5x5-basic-ascii",
    "width": 5,
    "height": 5,
    "bit_order": "msb_left",
    "version": 1
  },
  "glyphs": {
    "A": ["01110", "10001", "11111", "10001", "10001"],
    "B": ["11110", "10001", "11110", "10001", "11110"]
  }
}
```

**Why this format**

- Readable
- Easy to edit by hand
- Width is visually obvious
- Language-agnostic

**DTO representation (client-side)**
Conceptually:

```rust
HashMap<char, Vec<String>>
```

The CLI:

- Deserializes JSON
- Performs structural checks
- Converts to a domain-safe structure before passing to the lib

## 4️⃣ Writing Animation Data to .bin

**Library output**

- The library produces:

```rust
Vec<u8>
```

**Binary layout (MVP)**

- No metadata
- Frames written sequentially

```css
[frame0][frame1][frame2]...
```

**Frame layout**

```css
[row0 bytes][row1 bytes]...[rowN bytes]
```

**Why binary output**

- Compact
- Fast
- no_std friendly
- Portable across languages

The CLI writes this buffer directly to a .bin file.

## 5️⃣ Reading .bin Data in Firmware

**Rust firmware**

```rust
const ANIM: &[u8] = include_bytes!("anim.bin");

let frame = &ANIM[i * FRAME_SIZE .. (i + 1) * FRAME_SIZE];
```

**C / C++**

```c
const uint8_t* frame = anim + i * FRAME_SIZE;
```

**MicroPython**

```python
frame = data[i*FRAME_SIZE:(i+1)*FRAME_SIZE]
```

> No parsing or decoding required — just indexing.

## 6️⃣ Using u8 for Arbitrary Grid Sizes

**Key rule**

`u8` is a storage unit, not a size limit.

For any matrix width:

```ini
bytes_per_row = ceil(width / 8)
frame_size = height × bytes_per_row
```

**Examples**
| Matrix | Bytes per row | Frame size |
| ------ | ------------- | ---------- |
| 5×5 | 1 | 5 bytes |
| 8×8 | 1 | 8 bytes |
| 12×8 | 2 | 16 bytes |
| 16×16 | 2 | 32 bytes |

**Benefits**

- Endianness-safe
- Portable
- Scales to any width
- Matches how LED hardware is driven

## 7️⃣ DTO Representation (Client Side)

Conceptually:

```rust
struct GlyphFileDto {
    meta: MetaDto,
    glyphs: HashMap<char, Vec<String>>,
}


struct MetaDto {
    name: String,
    width: usize,
    height: usize,
    bit_order: BitOrderDto,
    version: u32,
}
```

**Client-side validation examples**

- glyph.len() == meta.height
- each string length == meta.width
- characters are '0' or '1'
- glyph keys are visible ASCII

## 8️⃣ Library Domain Representation (After Conversion)

```rust
struct Glyph {
    bitmap: Vec<u8>, // packed rows
}

struct Font {
    width: usize,
    height: usize,
    glyphs: HashMap<char, Glyph>,
}
```

**At this point:**

- No strings
- No JSON
- No metadata text
- Just binary-safe data

## Final Principle

> **JSON is an authoring format.**

> **Binary is a delivery format.**

> **The library lives strictly in between.**
