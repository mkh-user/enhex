# Changelog

All notable changes to EnhEx Rust core will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [core-v0.3.0] - 2026-05-29

### Added

- Add WASM port support for JavaScript binding
- Add raw RegEx support with `regex(...)` and `/.../`

## [core-v0.2.0] - 2026-05-09

### Added

- Add `hex_digit` atom
- Add lazy quantifiers support
- Add negated class support
- Add backreference support
- Add `non_digit`, `non_word_char`, `non_whitespace` atoms
- Add `carriage_return`, `hex_digit`, `null`, `vertical_tab`, `form_feed`, `bell`, `backslash` atoms
- Examples in `examples/`

## [core-v0.1.0] - 2026-05-07

### Added

- Rust core with basic features
- Lexer, Parser, Code Generator

[core-v0.3.0]: https://github.com/mkh-user/enhex/releases/tag/core-v0.3.0
[core-v0.2.0]: https://github.com/mkh-user/enhex/releases/tag/core-v0.2.0
[core-v0.1.0]: https://github.com/mkh-user/enhex/releases/tag/core-v0.1.0
