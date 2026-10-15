# AGENTS.md — EnhEx contributor guide (human + AI agents)

## What this repo is
EnhEx: a readable regex language. One Rust core (`core/`), two bindings
(`bindings/python` via PyO3/maturin, `bindings/js` via WASM/wasm-pack).
`vscode/` extension lives on a separate branch — do not touch it here.

## Layout
- `core/src/` — lexer, parser, codegen, wasm entry (`wasm.rs`)
- `bindings/python/src/enhex/` — `__init__.py`, `cli.py`, native `_enhex`
- `bindings/js/src/` — `index.js`, `index.cjs`, `index.d.ts`, `index.test.js`
- `examples/` — `.enhex` patterns, each with `# expect: <regex>` first line
- `SPEC.md` — language spec (versioned with the language, not each release)

## Build / test (run before every change)
```bash
cargo test --manifest-path core/Cargo.toml   # 45 unit + doctests
python examples/_test.py                     # needs installed `enhex`; exits 1 on failure
# JS: wasm-pack build --target nodejs --out-dir pkg --out-name enhex-core  (in core/)
#     then: node scripts/copy-wasm.js && npm ci && npm test  (in bindings/js)
```

## Versioning (strict — read before releasing)
- Separated versioning: core, py, js bump independently (patch/minor only).
  Major bumps come from core only.
- Tags: `core-v*`, `py-v*`, `js-v*`. No bare `v*` tags.
- One version bump = manifests + changelogs + root `CHANGELOG.md` index together:
  `core/Cargo.toml` (+lock), `bindings/python/{Cargo.toml,pyproject.toml,src/enhex/__init__.py}`
  (+lock), `bindings/js/package.json`. Never bump one without the others.
- `RELEASE_NOTES.md` is the release source of truth; keep dep versions in it
  matching the lockfiles (`wasm-bindgen`, `thiserror`, `pyo3`, `typescript`).

## Docs / packaging
- Single source: root `README.md` + `LICENSE`. Binding/core copies are
  **generated at deploy time** by copy steps (`cp README.md …`, `copy-wasm.js`)
  and are gitignored — never hand-edit or commit them.
- Binding READMEs must agree with root on: package names (`enhexjs` for npm),
  Python = PyO3 native extension (not WASM), full import lists.

## CI (`.github/workflows/`)
- `test.yml` — gates PRs: cargo test, JS suite (with WASM build), `examples/_test.py`.
- `build-python.yml` (`py-v*` tags) — 14-job wheel matrix + PyPI publish.
- `publish-core.yml` (`core-v*`) — `cargo test` then `cargo publish`.
- `publish-js.yml` (`js-v*`) — wasm build, copy, `npm test`, `npm publish`.
- Publishing uses **OIDC trusted publishing** (`id-token: write`, no static tokens):
  crates.io via `rust-lang/crates-io-auth-action`, PyPI via
  `twine upload --skip-existing` (retry-safe; twine uses the same OIDC identity),
  npm via `npm publish --provenance` (workflow must be on the registry trust list).
- `codeql.yml` — 4 languages; Rust uses `build-mode: manual` + `cargo build`.
  Keep `paths:` filters so docs-only changes skip the matrix.

## Conventions
- Dependabot groups are load-bearing (`patterns`, list-form `assignees`) —
  the typo era produced one PR per bump; don't regress it.
- Every workflow has `concurrency: cancel-in-progress: true`.
- Governance: `CODEOWNERS` (`* @mkh-user`), PR/issue templates — keep them short.
- No `as any`/`@ts-ignore`, no empty catches, no committing without being asked.
