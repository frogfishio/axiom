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

The implementation still exposes many dynamic conditions as host-side `EvalError` values instead of SDA-level `Fail(code, msg)` results or a clearly separate invocation boundary.

Examples include:

- `UnboundVar`
- `NotCallable`
- `ArityMismatch`

The main remaining ambiguity is now narrower than before.

Required decision:

- which of these become SDA-stable semantic failures
- which remain host/profile diagnostics

Status update:

- wrong-shape semantic misuse has now been normalized to `Fail(t_sda_wrong_shape, ...)`
- division by zero has now been normalized to `Fail(t_sda_div_by_zero, ...)`

The remaining open invocation/profile boundary is now mostly about:

- unbound names
- not-callable values
- arity mismatch

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

The spec now defines the standalone position more clearly, but the parser still leaves some conditions to generic parse failures rather than stable tagged conditions or explicit profile rules.

Important examples:

- unsupported comprehension shapes
- invalid selector-like constructs
- invalid map or bagkv entry usage
- invocation of reserved placeholder in declaration-like positions

This is not a correctness disaster, but it is still weaker than the clarified spec posture.

### 4. Helper And Combinator Error Boundaries

The standalone helpers and core combinators now mostly implement the right success semantics, and wrong-shape behavior is substantially more consistent.

Wrong-shape misuse now resolves to SDA `Fail(...)` values across the main standalone surface.
Remaining host-side issues are mostly invocation-level, especially arity and callability.

This inconsistency matters because helpers are now explicitly divided between:

- core combinators
- standalone profile helpers

The repository still needs one final explicit rule for invocation misuse across all callable forms.

### 5. Spec Examples Are Now Sharper Than The Tests

The spec was clarified to require explicit placeholder usage and explicit `Bind(...)` in standalone comprehension yields.

The implementation already fits those rules better than the old prose did, but the tests do not yet prove that alignment clearly enough.

That creates a regression risk:

- future contributors may accidentally reintroduce the older ambiguous reading

## Prioritized Worklist

### Priority 1: Close The Remaining Invocation Boundary

1. Decide the final taxonomy for dynamic misuse:
   - SDA `Fail(code, msg)`
   - parse/static rejection
   - profile/invocation diagnostics

2. Apply that remaining rule consistently across:
   - lambda application
   - named callable lookup
   - CLI/runtime invocation boundaries

3. Update conformance tests so the chosen boundary is provable.

### Priority 2: Tighten Diagnostics And CLI Coverage

4. Improve parser diagnostics where standalone restrictions matter materially.
5. Add CLI black-box tests for `eval`, `check`, and `fmt`.

### Priority 3: Keep The Spec Executable

6. Decide whether any additional stable parse-time tags are needed or whether generic parse diagnostics remain sufficient outside the current tagged cases.

7. Add regression tests that prove the standalone profile rejects or avoids the old ambiguous readings:
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