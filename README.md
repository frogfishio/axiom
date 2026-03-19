# axiom

This repository contains the Axiom family of tools, including SDA, a small language and command-line tool for reading, checking, and reshaping structured data.

## Published SDA Crates

The SDA surface is split into two publishable Rust crates:

- `sda`: command-line interface for evaluating, checking, and formatting SDA programs
- `sda-lib`: Rust library for parsing, validating, formatting, and evaluating SDA programs over JSON values

Install the CLI from crates.io:

```sh
cargo install sda
```

Embed the library in Rust:

```toml
[dependencies]
sda-lib = "1"
serde_json = "1"
```

## Your First 3 SDA Commands

If you want to try SDA immediately, start with these:

```sh
sda check -f SDA/examples/getting_started/person_name.sda
sda fmt -f SDA/examples/getting_started/person_name.sda --check
sda eval -f SDA/examples/getting_started/person_name.sda -i SDA/examples/getting_started/person.json
```

These three commands answer three different questions:

1. is the SDA source valid?
2. is it formatted correctly?
3. what happens when I run it on real data?

Start here if you want the approachable guide rather than the formal specification:

- [SDA User Manual](SDA/USER_MANUAL.md)
- [SDA Cheat Sheet](SDA/CHEATSHEET.md)
- [SDA for jq Users](SDA/FOR_JQ_USERS.md)
- [SDA Scholarly Introduction](SDA/INTRODUCTION.html)
- [Commands Overview](COMMANDS.md)

If you need the formal contract instead of the beginner guide:

- [SDA Specification](SDA/SDA_SPEC.md)

If you are preparing a release, see [RELEASE.md](RELEASE.md).
