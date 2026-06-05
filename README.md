# EnhEx: Enhanced Expression

_Regex, Enhanced for Readability._

![PyPI - Downloads](https://img.shields.io/pypi/dw/enhex?logo=python&logoColor=yellow&label=PyPI%20downloads&labelColor=black&color=white)
![NPM - Downloads](https://img.shields.io/npm/dw/enhexjs?logo=npm&logoColor=red&label=npm%20downloads&labelColor=black&color=white)
![Crates.io - Downloads](https://img.shields.io/crates/d/enhex_core?logo=rust&logoColor=orange&label=core%20downloads&labelColor=black&color=white)

EnhEx is a simple, readable language for writing regular expressions. Write patterns like sentences. Get standard Regex output. Use it anywhere — Python, JavaScript, Rust, CLI, browser.

---

## Why EnhEx?

Regex is powerful but painfully unreadable. After a few months, even your own patterns look like alien code.

EnhEx fixes this:

- **Write** patterns in a clean, human-readable syntax
- **Compile** to standard Regex that works everywhere
- **One core**, written in Rust, compiled to WASM or native extension — runs identically in every language

### Language Support Status

- Rust: ✅ Native
- Python: ✅ `enhex` at PyPI with native extension (PyO3)
- JavaScript: ✅ `enhexjs` at NPM with WASM (WASM BindGen)

---

## Quick Example

### EnhEx Input

```enhex
start + one_or_more(word_char | dot | dash) + "@" + one_or_more(word_char | dash) + dot + tld() + end
```

### Regex Output

```regex
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

### Python Library

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

### JavaScript Pakcage

```javascript
import { enhex, compile, compileRegExp } from 'enhexjs';

// or compile('start + one_or_more(digit) + end'):
const pattern = enhex`start + one_or_more(digit) + end`;
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

### Rust Crate

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

## Syntax

See [EnhEx Language Specification](https://github.com/mkh-user/enhex/blob/main/SPEC.md) for complete syntax.

---

## File Format

EnhEx patterns are stored in `.enhex` files:

`email.enhex`:

```enhex
start + one_or_more(word_char | dot | dash) + "@" + one_or_more(word_char | dash) + "." + tld() + end
```

---

## Development

### Project Structure

```text
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

### Roadmap

- [x] Language specification
- [x] Rust core engine + WASM
- [x] Python binding
- [x] CLI tool
- [x] JavaScript/TypeScript binding
- [ ] VSCode extension (syntax highlighting + live preview)
- [ ] Web playground

### Versioning Policy

EnhEx uses a separated versioning for core and each bindings, for example this list maybe current last versions:

```text
core-v0.2
py-v0.5
js-v0.3.1
```

Rules:

- Each core version change results in the same type of version increment across all bindings. (`core-v0.4 -> core-v0.5`: `py-v0.6 -> py-v0.7`, `js-v0.5 -> js-v0.6`)
- Each binding can have a patch or minor version increment (the major version is only changed by the core), and this change has no effect on the core version or other bindings.

The changelog for the core and each binding is available **in a separate file**; see [CHANGELOG.md](CHANGELOG.md) for an overview.

---

## License

MIT © Mahan Khalili

---

_RegEx, Enhanced._
