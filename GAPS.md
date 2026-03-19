# SDA Gaps

This file tracks the work needed to turn the current SDA seed runtime into the canonical implementation for this repository.

Current note:
The JSON bridge is now explicit: plain objects decode to `Map` unless they use a reserved SDA wrapper tag, and colliding plain maps are emitted through `{"$type":"map","$entries":{...}}` so wrapper-tag precedence stays deterministic without losing round-tripping.

Spec note:
`SDA/SDA_SPEC.md` and `SDA/DOCTRINE.md` now define a sharper boundary between the semantic core, the embedding / transformation model, the standalone profile, and outer orchestration. New work should be evaluated against that boundary rather than older, looser assumptions.

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
- [x] Decide whether `BagKV` literal keys should remain permissive or be aligned more tightly with the standalone surface.

Decision note:
Standalone `BagKV` keys now follow the spec grammar for selector positions: `IDENT | STRING`. This stays intentionally broader than standalone `Map` keys, which remain string-literal-only.
- [x] Expand comprehension parsing to support the full intended expression surface, not only identifier-led shorthand forms.
- [x] Add tests for Unicode and ASCII synonym parity across all supported operators.
- [x] Add tests for comment handling, whitespace insensitivity, and string escape semantics from the spec.

## Core Data Model

- [x] Implement the `Bytes` value kind end-to-end: syntax, runtime value, JSON or host bridge, and tests.
- [x] Decide and document canonical JSON bridging rules for non-JSON carriers and wrapper values.
- [x] Revisit `keys(map)` so it returns the carrier required by the spec.
- [x] Confirm `values(map)` ordering semantics and document whether they are canonical or host-derived.

Decision note:
`values(map)` is canonical in standalone SDA: it returns a `Seq` ordered by ascending string key. This avoids leaking parse or host object insertion order from an unordered carrier. `values(prod)` remains declaration-order.

## Errors And Diagnostics

- [x] Implement the remaining stable error tags listed in the SDA spec.
- [x] Freeze the final failure taxonomy across static rejection, SDA `Fail(code, msg)`, and profile / host diagnostics.
- [x] Separate runtime type and invocation errors from spec-stable `Fail(code, msg)` results according to that taxonomy.
- [ ] Improve parser diagnostics around selector ambiguity, invalid map keys, and unsupported comprehension forms.
- [x] Add conformance tests for all stable error codes and message strings.

Decision note:
Standalone SDA now treats wrong-shape semantic misuse as SDA `Fail(t_sda_wrong_shape, "wrong shape")` across operators, comprehensions, helpers, and core combinators. Parse-time/static conditions remain parse diagnostics. Invocation-level concerns such as unbound names, bad call arity, division by zero, and not-callable values remain host/profile diagnostics for now.

## CLI And Tooling

- [x] Replace the ad hoc positional CLI with the intended command surface (`eval`, `check`, `fmt` at minimum).
- [ ] Add fixture-driven CLI tests that exercise stdin, file input, stdout, and failure exits.
- [ ] Add a formatter or canonical pretty-printer plan for SDA source.
- [ ] Decide whether `cargo sda-run` remains a developer alias only or becomes part of documented workflow.

## Conformance Harness

- [x] Build a spec-indexed conformance suite layout separate from implementation unit tests.
- [x] Add tests for selector semantics on `Map`, `Prod`, and `BagKV` edge cases.
- [x] Add finite-corpus law tests for set, bag, and map algebra where determinism matters.
- [x] Add full spec-indexed conformance coverage for `§9 Comprehensions`.
- [x] Add full spec-indexed conformance coverage for `§10 Pipe` beyond unbound-placeholder cases.
- [x] Add conformance coverage for standalone membership on `Seq`, `Map`, and `Prod`.
- [x] Add conformance coverage for standalone helper misuse and success cases (`typeOf`, `keys`, `values`, `count`).
- [x] Add regression tests that replay the worked examples in `SDA/SDA_SPEC.md`.
- [ ] Add regression tests proving the standalone profile's explicit choices:
	- no implicit pipe argument insertion
	- no required general `k -> v` expression sugar
- [ ] Add regression coverage for every gap closed from this file.