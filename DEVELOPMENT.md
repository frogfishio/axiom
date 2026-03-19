# Development

This repository is a Rust workspace. The active SDA implementation now lives in the top-level crates:

- `crates/sda-core`: lexer, parser, evaluator, stdlib, JSON bridge, tests
- `crates/sda-cli`: executable surface for running SDA expressions

## Common commands

```sh
cargo test --workspace
cargo test -p sda-core
cargo run -p sda-cli --bin sda -- '1 + 2'
echo '{"name":"Ada"}' | cargo run -p sda-cli --bin sda -- 'input<"name">!'
echo '{"name":"Ada"}' | cargo run -p sda-cli --bin sda -- --bind root 'root<"name">!'
```

If you want shorter commands, the workspace defines cargo aliases in `.cargo/config.toml`:

```sh
cargo test-all
cargo sda-test
cargo sda-run -- '1 + 2'
```

These aliases are developer conveniences only. The shipped/public command surface is the `sda` binary; examples intended for users or external documentation should use `sda ...`, not `cargo sda-run`.

## Current structure

- `SDA/`: language specification, notes, examples, grammar materials
- `crates/sda-core`: implementation work
- `GAPS.md`: backlog of spec and product gaps with concrete tasks

## Development expectations

- Treat `SDA/SDA_SPEC.md` as the semantic contract.
- Fix semantic gaps at the root rather than adding compatibility shims.
- Prefer adding conformance-style tests before changing evaluator behavior.
- Keep `sda-core` pure; no ambient IO in the evaluator or stdlib.
- Standalone host input is bound explicitly. The default binding is `input`, and `_` remains reserved for pipe placeholder semantics.

## Canonical JSON bridge

- JSON primitives bridge directly: `null`, booleans, finite JSON numbers, strings, arrays -> `Null`, `Bool`, `Num`, `Str`, `Seq`.
- Plain JSON objects bridge to `Map` by default.
- Standalone `Bytes` values use `Bytes("...")` with an even-length base16 string. Output canonicalizes bytes to lowercase base16.
- Non-JSON carriers and wrapper values use reserved tagged objects:
	- `{"$type":"bytes","$base16":"00ff"}`
	- `{"$type":"set","$items":[...]}`
	- `{"$type":"bag","$items":[...]}`
	- `{"$type":"prod","$fields":{...}}`
	- `{"$type":"bagkv","$items":[[k,v], ...]}`
	- `{"$type":"bind","$key":k,"$val":v}`
	- `{"$type":"some","$value":v}`
	- `{"$type":"none"}`
	- `{"$type":"ok","$value":v}`
	- `{"$type":"fail","$code":"...","$msg":"..."}`
	- `{"$type":"num","$value":"p/q"}` for non-terminating exact rationals
- `Lambda` values are not host-round-trippable; they serialize as `{"$type":"fn"}` for display only.
- Reserved `$type` tags take precedence on decode. To preserve deterministic round-tripping for plain `Map` values that would collide with that namespace, `sda-core` emits an explicit wrapper only for those cases:
	- `{"$type":"map","$entries":{...}}`
- Unknown `$type` tags are treated as ordinary `Map` content rather than wrapper values.

## Ordering notes

- `Map` is unordered semantically, so any API that returns a sequence from a map must define its own order.
- `Set` and `Bag` are canonicalized on JSON output by sorting their item encodings by canonical JSON text. `Set` output also removes duplicate items if a malformed host value is constructed.
- `BagKV` is canonicalized on JSON output by sorting pair encodings by canonical JSON text.
- `Map` JSON output is canonicalized by ascending string key order.
- `values(map)` is canonicalized by ascending string key order in standalone SDA.
- `values(prod)` remains declaration order because `Prod` is a shaped record rather than an unordered map surface.