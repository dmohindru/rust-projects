# Validations (Temp)

## 2️⃣ What Goes Where (Concrete Table)

✅ Client (CLI) — Structural Validation

The CLI knows:

- the input format (JSON)
- how users make mistakes
- how to give friendly errors

**Examples that belong in the CLI:**
| Check | Why it’s structural |
| ---------------------------- | ------------------- |
| File exists / readable | I/O boundary |
| JSON parses | Syntax |
| `width` and `height` present | Schema |
| `glyphs` is a map | Shape |
| Each glyph key is a string | JSON rule |
| Each row is a string | Type |
| Row length == declared width | Shape |
| Row contains only `0`/`1` | Lexical |
| Number of rows == height | Shape |

## ✅ Library (glyph-lib) — Semantic Validation

The library knows:

- what a glyph is
- what a font is
- what operations must always be valid

**Examples that belong in the library:**
| Check | Why it’s semantic |
| -------------------------------- | ----------------- |
| Glyph key is exactly one char | Domain rule |
| Glyph is printable ASCII | Domain rule |
| Font has required glyphs | Domain rule |
| All glyphs share same dimensions | Invariant |
| Packed bits fit into storage | Invariant |
| Font can render requested string | Capability |
| Scrolling doesn’t overflow | Domain logic |

## 3️⃣ Rule of Thumb (Memorize This)

```
If the rule depends on your domain model → library
If the rule depends on input format → client
```

Or even shorter:

```
Boundary problems → client
Invariant problems → library
```
