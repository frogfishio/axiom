# SDA Implementation Audit

## Purpose

This document records the remaining deltas between the clarified SDA specification and the current Rust implementation.

It is not a general brainstorm.
It is a spec-driven implementation audit.

Baseline verified on 2026-03-19:

- `cargo test -p sda-core` passes
- the main outstanding work is semantic-contract alignment, not repairing a failing build

## Status Summary

The project is in a strong intermediate state:

- the core runtime is substantial and tested
- the standalone surface is mostly coherent
- determinism and canonicalization are already treated seriously

The main remaining work is to close the gap between:

- the clarified SDA spec in `SDA/SDA_SPEC.md`
- the doctrine in `SDA/DOCTRINE.md`
- the actual runtime behavior in `crates/sda-core`

## Areas That Already Align Well

These areas are in reasonably good shape relative to the clarified spec.

### 1. Exact Numbers And Canonical Semantics

- exact rational representation is implemented
- canonical JSON bridging for non-terminating rationals exists
- equality is no longer `f64`-style approximate host behavior

### 2. Carrier Distinctions

- `Map`, `Prod`, `Bag`, `Set`, `Seq`, and `BagKV` are distinct runtime carrier kinds
- `Prod` total projection is operationally distinct from `Map` lookup
- normalization from `BagKV` to `Map` is explicit

### 3. Deterministic Output Behavior

- `values(map)` uses canonical key ordering
- canonical JSON output for unordered carriers is implemented
- wrapper-tag collisions are handled explicitly in the JSON bridge

### 4. Placeholder-Based Pipe Semantics

- the implementation already behaves as placeholder-based composition
- it does not perform implicit argument insertion on pipe

This now matches the clarified spec.

### 5. Standalone Surface Restrictions

- `Map` literal keys are restricted to string literals
- `BagKV` literal keys follow selector-like positions
- keyword handling is effectively case-insensitive in the standalone lexer

## Remaining Deltas

These are the concrete areas where the implementation still does not fully match the clarified spec or where the boundary remains too loose to claim semantic closure.

### 1. Failure Taxonomy Is Still Split

The clarified spec now distinguishes:

- static invalidity
- core SDA semantic failures
- profile / invocation diagnostics

The implementation no longer leaks the main standalone invocation failures as host-side `EvalError` values.

Required decision:

- which of these become SDA-stable semantic failures
- which remain host/profile diagnostics

Status update:

- wrong-shape semantic misuse has now been normalized to `Fail(t_sda_wrong_shape, ...)`
- division by zero has now been normalized to `Fail(t_sda_div_by_zero, ...)`
- unbound names now normalize to `Fail(t_sda_unbound_name, ...)`
- non-callable invocation now normalizes to `Fail(t_sda_not_callable, ...)`
- arity mismatch now normalizes to `Fail(t_sda_arity_mismatch, ...)`

The remaining open profile boundary is now mostly outside the core evaluator itself: CLI/runtime invocation faults and any future host-installed extensions.

### 2. Conformance Coverage Is Too Small For The Real Contract

The spec has been tightened, but the conformance suite still lags behind the actual semantic surface.

Important remaining gaps are smaller now.

The spec-indexed harness now covers:

- comprehensions
- explicit pipe composition and non-implicit application
- membership on `Seq`, `Map`, and `Prod`
- helper misuse and success cases
- worked examples from the spec

The main remaining proof gaps are parser-boundary regressions and CLI black-box coverage rather than the core semantics above.

### 3. Standalone Grammar And Parser Still Differ On A Few Boundary Conditions

The spec now defines the standalone position more clearly, and the parser now covers several important boundary conditions with stable tags, but some conditions still fall back to generic parse failures.

Important examples:

- unsupported comprehension shapes now reject with an explicit generator-shape diagnostic when the source looks like `... in ...` but the binding is not an identifier
- invalid selector-like constructs
- other standalone profile restrictions not yet given explicit stable tags

This is not a correctness disaster, but it is still weaker than the clarified spec posture.

### 4. Helper And Combinator Error Boundaries

The standalone helpers and core combinators now mostly implement the right success semantics, and wrong-shape behavior is substantially more consistent.

Wrong-shape misuse now resolves to SDA `Fail(...)` values across the main standalone surface.
Remaining host-side issues are mostly outside the core evaluator now.

This inconsistency matters because helpers are now explicitly divided between:

- core combinators
- standalone profile helpers

That callable-boundary rule is now explicit in the runtime and spec for the standalone surface.

### 5. Spec Examples Are Now Sharper Than The Tests

The spec was clarified to require explicit placeholder usage and explicit `Bind(...)` in standalone comprehension yields.

The implementation already fits those rules better than the old prose did, but the tests do not yet prove that alignment clearly enough.

That creates a regression risk:

- future contributors may accidentally reintroduce the older ambiguous reading

## Prioritized Worklist

### Priority 1: Tighten Remaining Parser Boundaries

1. Decide whether any additional parse-time standalone restrictions deserve stable tags.

2. Apply that rule consistently across any remaining unsupported selector-edge or other standalone-only forms that are still weaker than the clarified profile.

3. Update conformance tests so the chosen parse boundary is provable.

### Priority 2: Tighten CLI Coverage

4. Add broader CLI black-box tests for mixed success/failure cases.

### Priority 3: Keep The Spec Executable

5. Keep the explicit-choice regressions in place:
   - no implicit pipe application
   - no required general `k -> v` expression sugar

## Recommended Implementation Order

1. Finish the remaining invocation-boundary decision.
2. Improve parser diagnostics where the standalone profile is intentionally strict.
3. Add and keep CLI black-box tests.
4. Revisit canonical formatting after the command surface is stable.

## Exit Criteria

This audit is resolved only when:

- the implementation and spec agree on the failure boundary
- the standalone examples are executable as written
- the conformance suite proves the clarified semantics rather than relying mainly on unit tests
- no remaining behavior depends on accidental host conventions where the spec claims certainty