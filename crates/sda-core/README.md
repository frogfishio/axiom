# sda-lib

`sda-lib` is the Rust library for Structured Data Algebra.

It provides a small host-facing API for parsing, validating, formatting, and evaluating standalone SDA programs over JSON values.

## Install

```toml
[dependencies]
sda-lib = "1"
serde_json = "1"
```

## Example

```rust
let output = sda_lib::run("input<\"name\">!", serde_json::json!({"name": "Ada"}))?;
assert_eq!(output, serde_json::json!({"$type": "ok", "$value": "Ada"}));
# Ok::<(), sda_lib::SdaError>(())
```

## API Surface

- `run`: evaluate an SDA program against JSON bound as `input`
- `run_with_input_binding`: evaluate against a caller-chosen binding name
- `check`: parse and validate source without evaluating it
- `format_source`: emit canonical SDA formatting
- `from_json` / `to_json`: bridge between JSON and SDA values

## Documentation

- Repository: https://github.com/frogfishio/axiom
- Formal specification: https://github.com/frogfishio/axiom/blob/main/SDA/SDA_SPEC.md
- User manual: https://github.com/frogfishio/axiom/blob/main/SDA/USER_MANUAL.md
- jq guide: https://github.com/frogfishio/axiom/blob/main/SDA/FOR_JQ_USERS.md