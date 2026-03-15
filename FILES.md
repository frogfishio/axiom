# Files And Layout

## Purpose

This document proposes file extensions and project layout for the Axiom family.

The goals are:

- make programs easy to recognize
- keep the semantic layers separate
- support both shell-first and project-based usage
- leave room for library embedding and later tooling

## File Types

Proposed source file extensions:

- `.sda`: SDA programs
- `.enr`: Enrichment programs
- `.shp`: Shaping programs
- `.ax`: Axiom workflow programs

These should remain narrow in meaning:

- `.sda` for pure transforms
- `.enr` for source lookup and join semantics
- `.shp` for output shaping
- `.ax` for orchestration and effects

## Basic Single-Workflow Layout

For a small project:

```text
project/
  workflows/
    customer-lookup.ax
  sda/
    extract.sda
    derive-request.sda
    normalize-customer.sda
    refine-result.sda
  enr/
    join-customer.enr
  shape/
    result.shp
  fixtures/
    event.json
    service-response.json
    expected-output.json
```

This layout keeps the four semantic areas visible.

## Alternative Minimal Layout

For quick experiments or shell-first use:

```text
project/
  extract.sda
  derive-request.sda
  normalize-customer.sda
  join-customer.enr
  result.shp
  customer-lookup.ax
  event.json
```

This is acceptable for prototypes, but it scales poorly.

## Workflow References

An `.ax` workflow should reference other program files explicitly.

Conceptually:

```text
use sda "sda/extract.sda"
use sda "sda/derive-request.sda"
use sda "sda/normalize-customer.sda"
use enr "enr/join-customer.enr"
use shape "shape/result.shp"
```

Whether this uses explicit import syntax, named stages, or embedded blocks is an implementation detail.
The important point is that the workflow should make stage dependencies visible.

## Fixtures

Fixtures matter because the whole project is intended to be testable and reproducible.

Recommended fixture types:

- input events
- service responses
- source snapshots
- expected intermediate values
- expected final outputs

Suggested layout:

```text
fixtures/
  lambda/
    input.json
    service-a-response.json
    expected-request.json
    expected-customer.json
    expected-output.json
```

## Proposed Test Layout

As the tools mature, tests should live beside fixtures and expected results.

Example:

```text
tests/
  sda/
    extract/
      input.json
      program.sda
      expected.json
  enr/
    join-customer/
      left.json
      right.json
      program.enr
      expected.json
  shape/
    result/
      input.json
      program.shp
      expected.json
  axiom/
    customer-lookup/
      workflow.ax
      input.json
      service-a-response.json
      expected.json
```

This encourages conformance-style testing instead of ad hoc demos.

## Packaging Direction

A project may later want a manifest, but we should not rush one.

Possible future manifest:

- `axiom.toml`

It could describe:

- workflow entry points
- source roots
- fixture locations
- runtime profiles
- service endpoint aliases

But v0 does not require a manifest if file references are explicit.

## Output Files

By default, tools should prefer stdout.
Files are for persistence, fixtures, and debugging.

Common generated artifacts may include:

- normalized intermediate JSON
- explain plans
- trace logs
- cached HTTP responses

If we later standardize these, a build-style layout might look like:

```text
.axiom/
  cache/
  traces/
  explain/
  tmp/
```

That should remain an implementation detail until there is a real runtime.

## Naming Guidance

Prefer stage-oriented names over generic names.

Good examples:

- `extract-customer.sda`
- `derive-customer-request.sda`
- `normalize-customer-response.sda`
- `join-customer-profile.enr`
- `emit-customer-result.shp`
- `customer-lookup.ax`

Weak examples:

- `step1.sda`
- `test.enr`
- `final.shp`

The filenames should help users understand the pipeline without opening every file.

## Minimal Working Set

A practical first project could be just:

```text
customer-lookup.ax
extract.sda
derive-request.sda
normalize-response.sda
result.shp
fixtures/input.json
fixtures/response.json
fixtures/expected-output.json
```

That is enough to prove the architecture without inventing a large package system.
