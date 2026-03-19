# Release Checklist

This repository publishes the SDA surface as two crates:

1. `sda-lib`
2. `sda`

The order matters because `sda` depends on `sda-lib` by its published crate name.

## Preflight

Run the full validation set from the repository root:

```sh
cargo test --workspace
cargo package -p sda-lib --allow-dirty --no-verify
cargo package --list -p sda --allow-dirty
```

Notes:

- `cargo package -p sda` cannot fully verify against crates.io until `sda-lib` has been published and indexed.
- `make dist` syncs the root `BUILD` value into `crates/sda-cli/BUILD` before producing the release binary.

## Publish `sda-lib`

```sh
cargo publish -p sda-lib
```

Wait for crates.io to index the release before publishing `sda`.

## Publish `sda`

```sh
cargo publish -p sda
```

## Post-publish Verification

Check the published crate pages:

- https://crates.io/crates/sda-lib
- https://crates.io/crates/sda

Check docs.rs:

- https://docs.rs/sda-lib
- https://docs.rs/sda

Check installation:

```sh
cargo install sda --force
sda --version
sda --license
```