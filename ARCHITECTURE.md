# Axiom Architecture

## Thesis

The problem we are solving is not just "transform some JSON."

It is:

1. receive structured input
2. extract and normalize useful data
3. call external systems using derived requests
4. normalize the responses
5. combine original and fetched data
6. shape the final result

That is too much responsibility for one language.

The architecture should therefore separate:

- pure transformation
- external effects
- enrichment and joins
- final output shaping
- orchestration

## The Split

We want a family of small tools, not one monolith.

The system has four layers:

1. `SDA`
2. `Enrichment`
3. `Shaping`
4. `Axiom`

`SDA`, `Enrichment`, and `Shaping` are semantic engines.
`Axiom` is the effectful orchestration layer.

Implementation priority is not equal across these layers.
`SDA` is the cornerstone and the workhorse.
The rest of the system should be built on top of a strong `SDA`, not alongside a weak one.

## SDA

`SDA` is the pure data transformation language.

It is responsible for:

- extracting slices from structured input
- normalization
- validation
- derived values and keys
- reshaping one dataset into another dataset
- explicit semantics for `Null`, absence, duplicates, and carrier behavior

`SDA` must stay pure.

It does not:

- call HTTP services
- read or write files
- manage retries, caching, or auth
- perform workflow control

Rule: `SDA` computes requests and normalizes responses, but never performs effects.

## Enrichment

`Enrichment` describes how datasets from different sources are combined.

Its concerns are:

- source declarations
- lookup semantics
- unique vs multi matches
- join behavior
- missing and duplicate policies
- provenance and explainability

This may begin as a semantic library before it becomes its own language.

Rule: enrichment should make source usage and join policy explicit, not hide them in host code.

## Shaping

`Shaping` is responsible for final output construction.

Its concerns are:

- final response contracts
- output schemas
- rendering and layout
- export-oriented structure
- binary or fixed-width output later if needed

This layer exists only if output concerns become distinct enough from pure transformation.

Rule: shaping should stay output-focused and not become a second general-purpose transform language.

## Axiom

`Axiom` is the orchestration language.

This is the real effectful layer. It coordinates the semantic engines and interacts with the outside world.

`Axiom` owns:

- stage sequencing
- state-machine transitions
- named inputs and outputs
- HTTP CRUD
- file CRUD
- retries, timeouts, auth, and caching policy
- parsing and routing of intermediate values
- invocation of `SDA`, `Enrichment`, and `Shaping`

Rule: `Axiom` performs requests and moves workflow state forward, but should not absorb all business transformation logic.

## Boundary Rules

These boundaries matter more than syntax.

1. `SDA` stays pure.
2. `Axiom` owns effects.
3. Enrichment stays explicit.
4. Shaping stays output-focused.
5. Each layer should eventually have stable semantics, failures, tests, CLI entry points, and library APIs.

## Delivery Model

The architecture supports four modes of use.

### 1. Standalone CLIs

- `sda`
- `enr`
- `shape`
- `axiom`

This supports shell-first workflows and quick experiments.

### 2. Shell Composition

The tools should be usable directly from shell scripts and CI pipelines.

Example:

```sh
sda -f extract.sda < event.json > useful.json
enr -f join.enr --in useful.json > joined.json
shape -f result.shp < joined.json > result.json
axiom run workflow.ax
```

### 3. Embedded Libraries

Each semantic engine should also be embeddable from host languages such as Oberon, C, Rust, or Go.

Conceptually:

- `sda_eval(program, input) -> value | fail`
- `enr_eval(program, input, source_provider) -> value | fail`
- `shape_eval(program, input) -> value | bytes | fail`

This lets a host language provide the glue before the Axiom orchestration DSL exists.

### 4. Axiom Orchestration

When workflow patterns stabilize, `Axiom` becomes the thin declarative glue over the three engines.

## Product Framing

The cleanest framing is:

- `SDA` is the pure transformation engine
- `Axiom` is the orchestration engine for workflows with external effects
- `Enrichment` and `Shaping` are distinct semantic layers when their complexity justifies it

In short:

`Axiom` is not the transformation language.

`Axiom` is the glue that coordinates transformation, acquisition, enrichment, and shaping.

For implementation planning, this also means:

- `SDA` is the first real product
- `Axiom` is a later orchestration layer built around `SDA`
- `Enrichment` and `Shaping` should split out only when they justify their own surfaces

## Implementation Order

The pragmatic order is:

1. make `SDA` solid as a library and CLI
2. make `sda` useful enough to stand alone as a shell and ETL tool
3. build the minimum useful `Axiom` runtime for effectful orchestration
4. split out `Enrichment` if source and join semantics need their own surface
5. split out `Shaping` if output construction diverges from pure transformation
6. add the Axiom glue syntax only after repeated orchestration patterns are clear

We should not start by building four complete grammars.
We should start by building the workhorse well.
