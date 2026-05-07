# EnhEx Language Specification

**Version:** 0.1.0

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
```

### Translation Table

| Atom | Description | Regex |
|:---|:---|:---|
| `digit` | Any digit 0–9 | `\d` |
| `word_char` | Letter, digit, or underscore | `\w` |
| `whitespace` | Space, tab, or newline | `\s` |
| `lowercase` | Lowercase letters a–z | `[a-z]` |
| `uppercase` | Uppercase letters A–Z | `[A-Z]` |
| `letter` | Any letter a–z or A–Z | `[a-zA-Z]` |
| `anything` | Any single character | `.` |
| `dot` | Literal period character | `\.` |
| `dash` | Literal hyphen character | `\-` |
| `tab` | Literal tab character | `\t` |
| `newline` | Literal newline character | `\n` |

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

## 5. Quantifiers

Quantifiers specify how many times an expression should repeat.

### Syntax
```
quantifier := 'one_or_more'  '(' expression ')'
            | 'zero_or_more' '(' expression ')'
            | 'optional'     '(' expression ')'
            | 'exactly'      '(' integer ',' expression ')'
            | 'at_least'     '(' integer ',' expression ')'
            | 'between'      '(' integer ',' integer ',' expression ')'
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

### Examples

| EnhEx | Regex |
|:---|:---|
| `one_or_more(digit)` | `\d+` |
| `exactly(3, digit)` | `\d{3}` |
| `between(2, 5, letter)` | `[a-zA-Z]{2,5}` |
| `optional(dash)` | `\-?` |
| `zero_or_more(anything)` | `.*` |

---

## 6. Sequence

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

## 7. Alternation

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

## 8. Groups

Groups wrap expressions for capturing, non-capturing, or naming purposes.

### Syntax
```
group := 'group'          '(' expression ')'
       | 'non_capturing'  '(' expression ')'
       | 'named'          '(' string_literal ',' expression ')'
```

### Translation Table

| EnhEx | Regex | Purpose |
|:---|:---|:---|
| `group(X)` | `(X)` | Capturing group |
| `non_capturing(X)` | `(?:X)` | Non-capturing group |
| `named("name", X)` | `(?P<name>X)` | Named capturing group |

### Examples

| EnhEx | Regex |
|:---|:---|
| `group(digit \| letter)` | `(\d\|[a-zA-Z])` |
| `non_capturing(dot + digit)` | `(?:\.\d)` |
| `named("area", exactly(3, digit))` | `(?P<area>\d{3})` |

---

## 9. Anchors

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

## 10. Lookaround

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

## 11. Presets

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

## 12. Operator Precedence

From highest to lowest priority:

| Priority | Operators |
|:---|:---|
| 1 (highest) | Literals, Atoms, Anchors |
| 2 | Quantifiers, Groups, Presets, Lookaround |
| 3 | Alternation (`\|`) |
| 4 (lowest) | Sequence (`+`) |

Use `group(...)` to override precedence when needed.

### Example

```
# These are different:
digit | letter + digit       → \d|[a-zA-Z]\d
group(digit | letter) + digit → (\d|[a-zA-Z])\d
```

---

## 13. Whitespace

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

## 14. Comments

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

## 15. Complete Grammar (EBNF)

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
      | quantified
      | group
      | anchor
      | lookaround
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

(* Literals *)
literal := '"' {character - '"'} '"'

(* Quantifiers *)
quantified := quantifier_name '(' expression ')'
            | 'exactly'   '(' integer ',' expression ')'
            | 'at_least'  '(' integer ',' expression ')'
            | 'between'   '(' integer ',' integer ',' expression ')'

quantifier_name := 'one_or_more'
                 | 'zero_or_more'
                 | 'optional'

(* Groups *)
group := 'group'          '(' expression ')'
       | 'non_capturing'  '(' expression ')'
       | 'named'          '(' string_literal ',' expression ')'

(* Anchors *)
anchor := 'start'
        | 'end'
        | 'word_boundary'

(* Lookaround *)
lookaround := 'followed_by'      '(' expression ')'
            | 'not_followed_by'  '(' expression ')'
            | 'preceded_by'      '(' expression ')'
            | 'not_preceded_by'  '(' expression ')'

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

## 16. Example Patterns

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

## 17. Error Handling

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