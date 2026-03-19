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
- `TypeError`
- `NotCallable`
- `DivByZero`
- `ArityMismatch`

This is the single biggest remaining semantic decision.

Required decision:

- which of these become SDA-stable semantic failures
- which remain host/profile diagnostics

Until this is fixed, the language boundary is still partially ambiguous.

### 2. Conformance Coverage Is Too Small For The Real Contract

The spec has been tightened, but the conformance suite still lags behind the actual semantic surface.

Important gaps include:

- full comprehension coverage by spec section
- explicit pipe composition cases using placeholder-based stages
- membership on `Seq`, `Map`, and `Prod`
- profile helper behavior beyond the current minimal cases
- failure-boundary tests for combinators and operator misuse

The implementation has useful unit tests, but much semantic proof still lives outside the spec-indexed conformance harness.

### 3. Standalone Grammar And Parser Still Differ On A Few Boundary Conditions

The spec now defines the standalone position more clearly, but the parser still leaves some conditions to generic parse failures rather than stable tagged conditions or explicit profile rules.

Important examples:

- unsupported comprehension shapes
- invalid selector-like constructs
- invalid map or bagkv entry usage
- invocation of reserved placeholder in declaration-like positions

This is not a correctness disaster, but it is still weaker than the clarified spec posture.

### 4. Helper And Combinator Error Boundaries Are Still Hostish

The standalone helpers and core combinators mostly implement the right success semantics, but their wrong-shape and bad-arity behavior is inconsistent.

Some helpers return SDA `Fail(...)` values for wrong-shape carrier conditions.
Others still raise host-side `EvalError::TypeError` or `ArityMismatch`.

This inconsistency matters because helpers are now explicitly divided between:

- core combinators
- standalone profile helpers

The repository needs one rule for how misuse is surfaced in each category.

### 5. Spec Examples Are Now Sharper Than The Tests

The spec was clarified to require explicit placeholder usage and explicit `Bind(...)` in standalone comprehension yields.

The implementation already fits those rules better than the old prose did, but the tests do not yet prove that alignment clearly enough.

That creates a regression risk:

- future contributors may accidentally reintroduce the older ambiguous reading

## Prioritized Worklist

### Priority 1: Close The Failure Boundary

1. Decide the final taxonomy for dynamic misuse:
   - SDA `Fail(code, msg)`
   - parse/static rejection
   - profile/invocation diagnostics

2. Apply that rule consistently across:
   - operators
   - combinators
   - helper functions
   - lambda application

3. Update conformance tests so the chosen boundary is provable.

### Priority 2: Expand Spec-Indexed Conformance

4. Add a complete `§9 Comprehensions` conformance block.
5. Add `§10 Pipe` coverage beyond unbound placeholder.
6. Add membership coverage for `Seq`, `Map`, and `Prod`.
7. Add helper-profile conformance for `typeOf`, `keys`, `values`, and `count` misuse as well as success cases.

### Priority 3: Tighten Diagnostics And Parse Boundaries

8. Improve parser diagnostics where standalone restrictions matter materially.
9. Decide whether any additional stable parse-time tags are needed or whether generic parse diagnostics remain sufficient outside the current tagged cases.

### Priority 4: Keep The Spec Executable

10. Add regression tests that replay the worked examples from `SDA/SDA_SPEC.md`.
11. Add regression tests that prove the standalone profile rejects or avoids the old ambiguous readings:
    - no implicit pipe application
    - no required general `k -> v` expression sugar

## Recommended Implementation Order

1. Freeze the failure taxonomy.
2. Expand conformance to match the clarified spec.
3. Refactor helper and operator misuse paths to match the taxonomy.
4. Improve parser diagnostics only after the semantic boundary is fixed.
5. Revisit CLI ergonomics after the semantic contract is closed.

## Exit Criteria

This audit is resolved only when:

- the implementation and spec agree on the failure boundary
- the standalone examples are executable as written
- the conformance suite proves the clarified semantics rather than relying mainly on unit tests
- no remaining behavior depends on accidental host conventions where the spec claims certainty