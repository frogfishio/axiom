# sda

`sda` is the command-line interface for Structured Data Algebra.

It evaluates SDA programs over JSON input, validates source without executing it, and emits canonical SDA formatting for editor and CI workflows.

## Install

```sh
cargo install sda
```

## Commands

```sh
sda eval -e 'values(input)' < event.json
sda check -f extract.sda
sda fmt -f extract.sda --check
```

## What SDA Is For

- deterministic transformation over structured data
- exact numeric semantics
- explicit success and failure values
- stable formatting and validation in automation

## Documentation

- Repository: https://github.com/frogfishio/axiom
- User manual: https://github.com/frogfishio/axiom/blob/main/SDA/USER_MANUAL.md
- Cheat sheet: https://github.com/frogfishio/axiom/blob/main/SDA/CHEATSHEET.md
- jq guide: https://github.com/frogfishio/axiom/blob/main/SDA/FOR_JQ_USERS.md
- Formal specification: https://github.com/frogfishio/axiom/blob/main/SDA/SDA_SPEC.md

## Library

If you want to embed SDA in a Rust program rather than shell out to the CLI, use the `sda-lib` crate.