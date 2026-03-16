# SDA Gaps

This file tracks the work needed to turn the current SDA seed runtime into the canonical implementation for this repository.

Current note:
Exact numeric semantics now use an exact rational representation. Non-terminating rationals are bridged through a canonical JSON wrapper (`{"$type":"num","$value":"p/q"}`), but the broader JSON bridge for all value kinds still needs a fuller canonicalization pass.

## Migration

- [ ] Remove the legacy nested Rust workspace under `SDA/sda` after the top-level port is stable.
- [ ] Update top-level documentation to point contributors at `crates/sda-core` and `crates/sda-cli`.
- [ ] Add regression tests that prove the top-level crates are the only supported implementation path.

## Determinism And Canonical Semantics

- [x] Replace `f64` numbers with an exact numeric representation that preserves equality and canonical round-tripping.
- [x] Make `Bag` equality extensional with multiplicity rather than insertion-order based.
- [x] Make `Map` equality extensional rather than insertion-order based.
- [x] Confirm whether `Prod` equality should also be extensional and implement the chosen contract explicitly.
- [x] Remove implicit ambient `_` input binding from the public `run` path and replace it with an explicit host input binding.

## Surface Conformance

- [x] Restrict standalone `Map` literal keys to string literals only.
- [ ] Decide whether `BagKV` literal keys should remain permissive or be aligned more tightly with the standalone surface.
- [x] Expand comprehension parsing to support the full intended expression surface, not only identifier-led shorthand forms.
- [ ] Add tests for Unicode and ASCII synonym parity across all supported operators.
- [ ] Add tests for comment handling, whitespace insensitivity, and string escape semantics from the spec.

## Core Data Model

- [ ] Implement the `Bytes` value kind end-to-end: syntax, runtime value, JSON or host bridge, and tests.
- [ ] Decide and document canonical JSON bridging rules for non-JSON carriers and wrapper values.
- [x] Revisit `keys(map)` so it returns the carrier required by the spec.
- [ ] Confirm `values(map)` ordering semantics and document whether they are canonical or host-derived.

## Errors And Diagnostics

- [ ] Implement the remaining stable error tags listed in the SDA spec.
- [ ] Separate runtime type errors from spec-stable `Fail(code, msg)` results more rigorously.
- [ ] Improve parser diagnostics around selector ambiguity, invalid map keys, and unsupported comprehension forms.
- [ ] Add conformance tests for all stable error codes and message strings.

## CLI And Tooling

- [ ] Replace the ad hoc positional CLI with the intended command surface (`eval`, `check`, `fmt` at minimum).
- [ ] Add fixture-driven CLI tests that exercise stdin, file input, stdout, and failure exits.
- [ ] Add a formatter or canonical pretty-printer plan for SDA source.
- [ ] Decide whether `cargo sda-run` remains a developer alias only or becomes part of documented workflow.

## Conformance Harness

- [ ] Build a spec-indexed conformance suite layout separate from implementation unit tests.
- [ ] Add tests for selector semantics on `Map`, `Prod`, and `BagKV` edge cases.
- [ ] Add property-style tests for set, bag, and map algebra where determinism matters.
- [ ] Add regression coverage for every gap closed from this file.