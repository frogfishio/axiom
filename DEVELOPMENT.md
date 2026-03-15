# Development

This repository is a Rust workspace. The active SDA implementation now lives in the top-level crates:

- `crates/sda-core`: lexer, parser, evaluator, stdlib, JSON bridge, tests
- `crates/sda-cli`: executable surface for running SDA expressions

## Common commands

```sh
cargo test --workspace
cargo test -p sda-core
cargo run -p sda-cli --bin sda -- '1 + 2'
echo '{"name":"Ada"}' | cargo run -p sda-cli --bin sda -- '_<"name">'
```

If you want shorter commands, the workspace defines cargo aliases in `.cargo/config.toml`:

```sh
cargo test-all
cargo sda-test
cargo sda-run -- '1 + 2'
```

## Current structure

- `SDA/`: language specification, notes, examples, grammar materials
- `crates/sda-core`: implementation work
- `GAPS.md`: backlog of spec and product gaps with concrete tasks

## Development expectations

- Treat `SDA/SDA_SPEC.md` as the semantic contract.
- Fix semantic gaps at the root rather than adding compatibility shims.
- Prefer adding conformance-style tests before changing evaluator behavior.
- Keep `sda-core` pure; no ambient IO in the evaluator or stdlib.