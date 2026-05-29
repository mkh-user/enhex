# EnhEx: Enhanced Expression

_Regex, Enhanced for Readability._

EnhEx is a simple, readable language for writing regular expressions. Write patterns like sentences. Get standard Regex output. Use it anywhere — Python, JavaScript, Rust, CLI, browser.

---

## Why EnhEx?

Regex is powerful but painfully unreadable. After a few months, even your own patterns look like alien code.

EnhEx fixes this:

- **Write** patterns in a clean, human-readable syntax
- **Compile** to standard Regex that works everywhere
- **One core**, written in Rust, compiled to WASM — runs identically in every language

### Language Support Status:

- Rust: ✅ Native
- Python: ✅ `enhex` at PyPI with WASM
- JavaScript: ✅ `enhexjs` at NPM with WASM

---

## Quick Example

### EnhEx Input
```
start + one_or_more(word_char | dot | dash) + "@" + one_or_more(word_char | dash) + dot + tld() + end
```

### Regex Output
```
^[\w\.-]+@[\w-]+\.[a-z]{2,10}$
```

Same logic, but you can actually **read** the first one.

---

## Installation

### Python
```bash
pip install enhex
```

### Rust
```bash
cargo install enhex-core
```

### JavaScript / TypeScript
```bash
npm install enhexjs
```

### CLI (via Python)
```bash
pip install enhex
enhex compile "start + one_or_more(digit) + end"
# Output: ^\d+$
```

---

## Usage

### Python
```python
import enhex as ex

# Compile a pattern string
pattern = ex.compile('start + one_or_more(digit) + end')

# Compile from a .enhex file
phone_pattern = ex.compile_file('phone.enhex')

# Use with standard re module
import re
if re.match(pattern, "367812009"):
    print("Valid number!")
```

### JavaScript

```javascript
import { compile } from 'enhexjs';

const pattern = compile('start + one_or_more(digit) + end');
const regex = new RegExp(pattern);

if (regex.test('12345')) {
    console.log('Only digits!');
}

// Or automatic RegExp creation:
const re = compileRegExp('start + one_or_more(digit) + end');

if (re.test('12345')) {
    console.log('Only digits!');
}

```

### Rust
```rust
use enhex_core::compile;

let pattern = compile("start + one_or_more(digit) + end").unwrap();
let re = regex::Regex::new(&pattern).unwrap();

assert!(re.is_match("12345"));
```

### CLI
```bash
# Compile a pattern string
enhex compile 'start + exactly(10, digit) + end'

# Compile a .enhex file
enhex compile phone.enhex

# Show version
enhex version
```

---

## Syntax Overview

### Atoms (Basic Building Blocks)

| EnhEx | Description | Regex Equivalent |
|:------|:------------|:-----------------|
| `digit` | Any digit 0-9 | `\d` |
| `word_char` | Letter, digit, or underscore | `\w` |
| `whitespace` | Space, tab, or newline | `\s` |
| `lowercase` | Lowercase letters a-z | `[a-z]` |
| `uppercase` | Uppercase letters A-Z | `[A-Z]` |
| `letter` | Any letter | `[a-zA-Z]` |
| `anything` | Any character | `.` |
| `dot` | Literal dot | `\.` |
| `dash` | Literal dash | `\-` |
| `tab` | Literal tab | `\t` |
| `newline` | Literal newline | `\n` |
| `hex_digit` | Single hex digit (upper or lower) | `[\da-fA-F]` |

### Quantifiers

| EnhEx | Regex Equivalent |
|:------|:-----------------|
| `one_or_more(X)` | `X+` |
| `zero_or_more(X)` | `X*` |
| `optional(X)` | `X?` |
| `exactly(N, X)` | `X{N}` |
| `at_least(N, X)` | `X{N,}` |
| `between(N, M, X)` | `X{N,M}` |
| `one_or_more_lazy(X)` | `X+?` |
| `zero_or_more_lazy(X)` | `X*?` |
| `optional_lazy(X)` | `X??` |

### Composition & Anchors

| EnhEx | Meaning | Regex Equivalent |
|:------|:--------|:-----------------|
| `X + Y` | X followed by Y | `XY` |
| `X \| Y` | X or Y | `X\|Y` |
| `"text"` | Literal text | escaped text |
| `start` | Start of string | `^` |
| `end` | End of string | `$` |
| `word_boundary` | Word boundary | `\b` |

### Groups

| EnhEx | Regex Equivalent |
|:------|:-----------------|
| `group(X)` | `(X)` |
| `non_capturing(X)` | `(?:X)` |
| `named("name", X)` | `(?P<name>X)` |
| `not(X)` | `[^X]` |

### Lookaround

| EnhEx | Regex Equivalent |
|:------|:-----------------|
| `followed_by(X)` | `(?=X)` |
| `not_followed_by(X)` | `(?!X)` |
| `preceded_by(X)` | `(?<=X)` |
| `not_preceded_by(X)` | `(?<!X)` |

### Backreferences

| EnhEx | Regex Equivalent |
|:------|:-----------------|
| `backref(1)` | `\1` |
| `backref("name")` | `(?P=name)` |

### Raw Regex Literal

| EnhEx | Regex |
|:---|:---|
| `regex("\\d{3}-\\d{4}")` | `\d{3}-\d{4}` |
| `/[\w\.-]+@[\w-]+\.[a-z]{2,10}/` | `[\w\.-]+@[\w-]+\.[a-z]{2,10}` |

### Built-in Presets

| EnhEx | Matches |
|:------|:--------|
| `tld()` | Top-level domain (com, org, ir, ...) |
| `email()` | Full email address |
| `url()` | URL |
| `ipv4()` | IPv4 address |

---

## File Format

EnhEx patterns are stored in `.enhex` files:

`email.enhex`:
```
start + one_or_more(word_char | dot | dash) + "@" + one_or_more(word_char | dash) + "." + tld() + end
```

---

## Project Structure

```
enhex/
├── core/          # Rust core engine
├── bindings/
│   ├── python/    # Python package
│   └── js/        # JavaScript/TypeScript package
├── examples/      # Example .enhex patterns
├── vscode/        # VSCode extension (coming soon)
├── playground/    # Web playground (coming soon)
├── SPEC.md        # Full language specification
└── README.md
```

---

## Roadmap

- [x] Language specification
- [x] Rust core engine + WASM
- [x] Python binding
- [x] CLI tool
- [x] JavaScript/TypeScript binding
- [ ] VSCode extension (syntax highlighting + live preview)
- [ ] Web playground

---

## License

MIT © Mahan Khalili

---

*RegEx, Enhanced.*
