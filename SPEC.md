# EnhEx Language Specification

**Version:** 0.3.0

---

## 1. Overview

EnhEx (Enhanced Expression) is a human-readable language for defining text patterns.
It compiles to standard Regex that works in any Regex engine.

### Design Principles

- **Readable**: Patterns read like sentences, not symbol soup.
- **Composable**: Simple building blocks combine into complex patterns.
- **Deterministic**: Every valid EnhEx expression produces exactly one Regex output.
- **Portable**: One core engine (Rust + WASM) runs identically everywhere.

---

## 2. Fundamental Concepts

Every EnhEx pattern is an **Expression**. All expressions evaluate to a Regex fragment.

### Expression Types

| Type | Description | Example |
|:---|:---|:---|
| Atom | Smallest meaningful unit | `digit`, `word_char` |
| Literal | Quoted literal text | `"hello"`, `"@"` |
| Quantified | Expression with a quantifier | `one_or_more(digit)` |
| Sequence | Chain of expressions | `digit + dot + digit` |
| Alternation | Choice between expressions | `digit \| letter` |
| Group | Grouped expression | `group(...)`, `non_capturing(...)` |
| Anchor | Position in string | `start`, `end` |
| Lookaround | Lookahead / lookbehind | `followed_by(...)` |
| Preset | Named built-in pattern | `email()`, `tld()` |

---

## 3. Atoms

Atoms are the smallest building blocks. Each maps directly to a Regex character class.

### Syntax
```
atom := 'digit'
      | 'non_digit'
      | 'word_char'
      | 'non_word_char'
      | 'whitespace'
      | 'non_whitespace'
      | 'lowercase'
      | 'uppercase'
      | 'letter'
      | 'anything'
      | 'dot'
      | 'dash'
      | 'tab'
      | 'newline'
      | 'carriage_return'
      | 'hex_digit'
      | 'null'
      | 'vertical_tab'
      | 'form_feed'
      | 'bell'
      | 'backslash'
```

### Translation Table

| EnhEx | Description | Regex Equivalent |
|:---|:---|:---|
| `digit` | Any digit 0–9 | `\d` |
| `non_digit` | Any non-digit | `\D` |
| `word_char` | Letter, digit, or underscore | `\w` |
| `non_word_char` | Any non-word character | `\W` |
| `whitespace` | Space, tab, or newline | `\s` |
| `non_whitespace` | Any non-whitespace | `\S` |
| `lowercase` | Lowercase letters a–z | `[a-z]` |
| `uppercase` | Uppercase letters A–Z | `[A-Z]` |
| `letter` | Any letter a–z or A–Z | `[a-zA-Z]` |
| `anything` | Any single character | `.` |
| `dot` | Literal period character | `\.` |
| `dash` | Literal hyphen character | `\-` |
| `tab` | Literal tab character | `\t` |
| `newline` | Literal newline character | `\n` |
| `carriage_return` | Literal carriage return | `\r` |
| `hex_digit` | Any hex digit 0-9, a-f, A-F | `[\da-fA-F]` |
| `null` | Null character (ASCII 0) | `\0` |
| `vertical_tab` | Vertical tab character | `\v` |
| `form_feed` | Form feed character | `\f` |
| `bell` | Bell character (ASCII 7) | `\a` |
| `backslash` | Literal backslash character | `\\` |

### Character Class Integration

These atoms can all be used inside `not()` and in alternations for character class generation:

```
# Non-whitespace character class
non_whitespace # -> \S

# Any non-digit, non-letter
not(digit | letter) # -> [^\da-zA-Z]

# Whitespace or newline alternatives
whitespace | carriage_return # -> [\s\r]
```

### Negated Atom Shorthands

For the three standard negated shorthands (`\D`, `\W`, `\S`), `EnhEx` provides direct atoms so you don't need to write `not(digit)`:

| EnhEx | RegEx | Equivalent `not()` Form | Regex |
|:---|:---|:---|:---|
| `non_digit` | `\D` | `not(digit)` | `[^\d]` |
| `non_word_char` | `\W` | `not(word_char)` | `[^\w]` |
| `non_whitespace` | `\S` | `not(whitespace)` | `[^\s]` |

**Recommendation:** Use the direct atom when you just need the negated shorthand. Use `not(...)` when you want to negate a custom character class:

```
# Use direct atom for standard negation
non_whitespace # -> \S

# Use not() for custom negation
not(digit | letter | "_") # -> [^\da-zA-Z_]
```

---

## 4. Literals

A literal is any text enclosed in double quotes. It is treated as an exact string to match.

Characters with special meaning in Regex (`.`, `+`, `*`, `?`, `[`, `]`, `(`, `)`, `/`, etc.)
are **automatically escaped** in the output.

### Syntax
```
literal := '"' <characters> '"'
```

### Examples

| EnhEx | Regex |
|:---|:---|
| `"hello"` | `hello` |
| `"@"` | `@` |
| `"https://"` | `https:\/\/` |
| `"."` | `\.` |
| `"+"` | `\+` |

---

## 5. Raw Regex Literal / `regex()` Identifier

| EnhEx | Regex |
|:---|:---|
| `regex("\\d{3}-\\d{4}")` | `\d{3}-\d{4}` |
| `` `/[\w\.-]+@[\w-]+\.[a-z]{2,10}/` `` | `[\w\.-]+@[\w-]+\.[a-z]{2,10}` |

Raw regex literals are useful when you need to embed a complex existing regex directly without converting it to EnhEx syntax. The `regex("...")` form takes a string (where backslashes must be escaped), while the `/.../` form takes a literal regex (where **no escaping is needed** unless you need to include a literal `/` or `\`).

---

## 6. Quantifiers

Quantifiers specify how many times an expression should repeat. Lazy quantifiers match the **shortest** possible string instead of the longest; Use them when you want to stop at the first match rather than the last.

### Syntax
```
quantifier := 'one_or_more'       '(' expression ')'
            | 'zero_or_more'      '(' expression ')'
            | 'optional'          '(' expression ')'
            | 'exactly'           '(' integer ',' expression ')'
            | 'at_least'          '(' integer ',' expression ')'
            | 'between'           '(' integer ',' integer ',' expression ')'
            | 'one_or_more_lazy'  '(' expression ')'
            | 'zero_or_more_lazy' '(' expression ')'
            | 'optional_lazy'     '(' expression ')'
```

### Translation Table

| EnhEx | Regex | Meaning |
|:---|:---|:---|
| `one_or_more(X)` | `X+` | One or more |
| `zero_or_more(X)` | `X*` | Zero or more |
| `optional(X)` | `X?` | Zero or one |
| `exactly(N, X)` | `X{N}` | Exactly N times |
| `at_least(N, X)` | `X{N,}` | N or more |
| `between(N, M, X)` | `X{N,M}` | Between N and M times |
| `one_or_more_lazy(X)` | `X+?` | One or more (lazy) |
| `zero_or_more_lazy(X)` | `X*?` | Zero or more (lazy) |
| `optional_lazy(X)` | `X??` | Zero or one (lazy) |

### Examples

| EnhEx | Regex |
|:---|:---|
| `one_or_more(digit)` | `\d+` |
| `exactly(3, digit)` | `\d{3}` |
| `between(2, 5, letter)` | `[a-zA-Z]{2,5}` |
| `optional(dash)` | `\-?` |
| `zero_or_more(anything)` | `.*` |
| `one_or_more_lazy(digit)` | `\d+?` |
| `zero_or_more_lazy(anything)` | `.*?` |

---

## 7. Sequence

A sequence chains expressions together with `+`, meaning "this followed by that."

### Syntax
```
sequence := expression ('+' expression)+
```

### Operator Precedence

`+` has the **lowest** precedence of all binary operators (lower than `|`).

### Examples

| EnhEx | Regex |
|:---|:---|
| `digit + dot + digit` | `\d\.\d` |
| `"09" + exactly(9, digit)` | `09\d{9}` |
| `start + one_or_more(word_char) + end` | `^\w+$` |
| `letter + zero_or_more(letter \| digit)` | `[a-zA-Z][a-zA-Z\d]*` |

---

## 8. Alternation

Alternation expresses "or" between expressions using `|`.

### Syntax
```
alternation := expression ('|' expression)+
```

### Operator Precedence

`|` has higher precedence than `+`, but lower than quantifiers and groups.
Use `group(...)` to override precedence when needed.

### Examples

| EnhEx | Regex |
|:---|:---|
| `digit \| letter` | `\d\|[a-zA-Z]` |
| `"http" \| "https"` | `http\|https` |
| `group("http" \| "https") + "://"` | `(http\|https):\/\/` |

---

## 9. Groups

Groups wrap expressions for capturing, non-capturing, or naming purposes.

### Syntax
```
group := 'group'          '(' expression ')'
       | 'non_capturing'  '(' expression ')'
       | 'named'          '(' string_literal ',' expression ')'
       | 'not'            '(' expression ')'
```

### Translation Table

| EnhEx | Regex | Purpose |
|:---|:---|:---|
| `group(X)` | `(X)` | Capturing group |
| `non_capturing(X)` | `(?:X)` | Non-capturing group |
| `named("name", X)` | `(?P<name>X)` | Named capturing group |
| `not(X)` | `[^X]` | Negated character class |

### Examples

| EnhEx | Regex |
|:---|:---|
| `group(digit \| letter)` | `(\d\|[a-zA-Z])` |
| `non_capturing(dot + digit)` | `(?:\.\d)` |
| `named("area", exactly(3, digit))` | `(?P<area>\d{3})` |
| `not(lowercase)` | `[^a-z]` |

---

## 10. Anchors

Anchors match positions within a string, not characters.

### Syntax
```
anchor := 'start'
        | 'end'
        | 'word_boundary'
```

### Translation Table

| EnhEx | Regex | Matches |
|:---|:---|:---|
| `start` | `^` | Beginning of string |
| `end` | `$` | End of string |
| `word_boundary` | `\b` | Word boundary |

### Examples

| EnhEx | Regex |
|:---|:---|
| `start + one_or_more(digit) + end` | `^\d+$` |
| `word_boundary + "cat" + word_boundary` | `\bcat\b` |

---

## 11. Lookaround

Lookaround expressions assert conditions before or after the current position
without consuming characters.

### Syntax
```
lookaround := 'followed_by'      '(' expression ')'
            | 'not_followed_by'  '(' expression ')'
            | 'preceded_by'      '(' expression ')'
            | 'not_preceded_by'  '(' expression ')'
```

### Translation Table

| EnhEx | Regex | Meaning |
|:---|:---|:---|
| `followed_by(X)` | `(?=X)` | Must be followed by X |
| `not_followed_by(X)` | `(?!X)` | Must NOT be followed by X |
| `preceded_by(X)` | `(?<=X)` | Must be preceded by X |
| `not_preceded_by(X)` | `(?<!X)` | Must NOT be preceded by X |

### Examples

| EnhEx | Regex |
|:---|:---|
| `digit + followed_by("$")` | `\d(?=\$)` |
| `not_preceded_by("@") + word_char` | `(?<!@)\w` |

---

## 12. Backreference

A backreference matches the same text that was matched by a capturing group earlier in the pattern.

### Syntax
```
backref := 'backref' '(' string_literal ')'
         | 'backref' '(' integer ')'
```

### Translation Table

| EnhEx | Regex Equivalent |
|:---|:---|
| `backref(i)` | `\i` |
| `backref("X")` | `(?P=X)` |

### Examples

| EnhEx | Regex |
|:---|:---|
| `start + group(digit) + "-" + backref(1) + end` | `^(\d)-\1$` |
| `start + named("tag", one_or_more(word_char)) + ">" + zero_or_more(anything) + "</" + backref("tag") + ">" + end` | `^(?P<tag>\w+)>.*</(?P=tag)>$` |

---

## 13. Presets

Presets are named, built-in patterns for common use cases.
They expand to their full Regex equivalent at compile time.

### Syntax
```
preset := 'tld'   '(' ')'
        | 'email' '(' ')'
        | 'url'   '(' ')'
        | 'ipv4'  '(' ')'
```

### Translation Table

| EnhEx | Regex Equivalent |
|:---|:---|
| `tld()` | `[a-z]{2,10}` |
| `email()` | Full email pattern (RFC 5322 simplified) |
| `url()` | Full URL pattern |
| `ipv4()` | `(?:\d{1,3}\.){3}\d{1,3}` |

### Usage

Presets can be used standalone or combined with other expressions:

```
start + email() + end
start + ipv4() + ":" + one_or_more(digit) + end
```

---

## 14. Operator Precedence

From highest to lowest priority:

| Priority | Operators |
|:---|:---|
| 1 (highest) | Literals, Atoms, Anchors |
| 2 | Quantifiers, Groups, Presets, Lookaround |
| 3 | Alternation (`\|`) |
| 4 (lowest) | Sequence (`+`) |

Use `group(...)` to override **RegEx** precedence when needed. `(...)` can be
used to override **EnhEx** precedence.

### Example

```
# These are different:
digit | letter + digit        → \d|[a-zA-Z]\d
group(digit | letter) + digit → (\d|[a-zA-Z])\d
```

---

## 15. Whitespace

Whitespace characters (spaces, tabs, newlines) **are ignored** between tokens,
except inside literal strings.

### Examples

All of these are equivalent:

```
digit+dot+digit
digit + dot + digit
digit
    + dot
    + digit
```

---

## 16. Comments

Comments begin with `#` and continue to the end of the line.
They are ignored by the compiler.

### Syntax
```
comment := '#' <any character except newline> <newline>
```

### Example

```
# This is a valid email pattern
start
    + one_or_more(word_char | dot | dash)  # local part
    + "@"                                   # at sign
    + one_or_more(word_char | dash)         # domain
    + "."                                   # dot
    + tld()                                 # top-level domain
    + end
```

---

## 17. Complete Grammar (EBNF)

```ebnf
(* Top level *)
pattern := expression

(* Expressions *)
expression := sequence
            | alternation

sequence := term ('+' term)*

alternation := sequence ('|' sequence)*

term := atom
      | literal
      | regex
      | quantified
      | group
      | anchor
      | lookaround
      | backref
      | preset
      | '(' expression ')'

(* Atoms *)
atom := 'digit'
      | 'word_char'
      | 'whitespace'
      | 'lowercase'
      | 'uppercase'
      | 'letter'
      | 'anything'
      | 'dot'
      | 'dash'
      | 'tab'
      | 'newline'
      | 'hex_digit'

(* Literals *)
literal := '"' {character - '"'} '"'

(* RegEx Literals *)
regex := '/' {character - '/'} '/'
       | 'regex' '(' literal ')'

(* Quantifiers *)
quantified := quantifier_name '(' expression ')'
            | 'exactly'   '(' integer ',' expression ')'
            | 'at_least'  '(' integer ',' expression ')'
            | 'between'   '(' integer ',' integer ',' expression ')'

quantifier_name := 'one_or_more'
                 | 'zero_or_more'
                 | 'optional'
                 | 'one_or_more_lazy'
                 | 'zero_or_more_lazy'
                 | 'optional_lazy'

(* Groups *)
group := 'group'          '(' expression ')'
       | 'non_capturing'  '(' expression ')'
       | 'named'          '(' string_literal ',' expression ')'
       | 'not'            '(' expression ')'

(* Anchors *)
anchor := 'start'
        | 'end'
        | 'word_boundary'

(* Lookaround *)
lookaround := 'followed_by'      '(' expression ')'
            | 'not_followed_by'  '(' expression ')'
            | 'preceded_by'      '(' expression ')'
            | 'not_preceded_by'  '(' expression ')'

(* Backref *)
backref := 'backref' '(' string_literal ')'
         | 'backref' '(' integer ')'

(* Presets *)
preset := 'tld'   '(' ')'
        | 'email' '(' ')'
        | 'url'   '(' ')'
        | 'ipv4'  '(' ')'

(* Primitives *)
integer := [0-9]+
string_literal := '"' {character - '"'} '"'
```

---

## 18. Example Patterns

### Email Address
```
start + one_or_more(word_char | dot | dash) + "@" + one_or_more(word_char | dash) + "." + tld() + end
```
→ `^[\w\.-]+@[\w-]+\.[a-z]{2,10}$`

### URL
```
start + "https?://" + one_or_more(word_char | dot | dash) + "." + tld() + optional("/" + zero_or_more(anything)) + end
```
→ `^https?://[\w\.-]+\.[a-z]{2,10}(?:/.*)?$`

### IPv4 Address
```
start + ipv4() + end
```
→ `^(?:\d{1,3}\.){3}\d{1,3}$`

### Date (YYYY-MM-DD)
```
start + exactly(4, digit) + "-" + between(1, 2, digit) + "-" + between(1, 2, digit) + end
```
→ `^\d{4}-\d{1,2}-\d{1,2}$`

### Hex Color Code
```
start + "#" + exactly(6, digit | letter) + end
```
→ `^#[\da-zA-Z]{6}$`

### Username (alphanumeric, 3–16 chars)
```
start + between(3, 16, word_char) + end
```
→ `^\w{3,16}$`

---

## 19. Error Handling

The compiler must produce clear, human-readable errors.

### Error Types

| Error | Example | Message |
|:---|:---|:---|
| Unclosed literal | `"hello` | `Error: Unclosed string literal at line 1, column 7` |
| Unknown token | `digitt` | `Error: Unknown token 'digitt' at line 1, column 1. Did you mean 'digit'?` |
| Missing comma (quantifier) | `exactly(3 digit)` | `Error: Expected ',' but found 'digit' at line 1, column 11` |
| Unbalanced parentheses | `group(digit` | `Error: Unclosed '(' at line 1, column 6` |

---

*End of Specification*
