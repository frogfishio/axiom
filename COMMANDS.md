# Commands

## Purpose

This document proposes the first command-line surface for the Axiom family.

The goal is not to define every option up front.
The goal is to make the product shape concrete enough to build and test.

It is also meant to justify why these runtimes should exist as standalone binaries, not only as embeddable libraries.
The Unix shell and ETL use case is part of the product, not an afterthought.

The tool family is:

- `sda`
- `enr`
- `shape`
- `axiom`

Each tool should work in two modes:

- standalone CLI
- embeddable runtime behind a library API

## Command Design Rules

The CLI family should follow these rules:

1. read structured input from stdin by default
2. write structured output to stdout by default
3. use files only when explicitly requested
4. keep flags predictable across tools
5. expose enough debug and explain output for testing
6. make failure codes stable and machine-friendly

These rules make the tools usable in:

- shell scripts
- CI jobs
- ad hoc ETL pipelines
- data repair and replay workflows
- local debugging against captured fixtures

## Why Separate Binaries Matter

The standalone tools are useful even when users never adopt the Axiom workflow language.

They support a Unix-style model where each binary does one narrow job well:

- `sda`: extract, normalize, validate, and reshape data
- `enr`: combine datasets with explicit join policy
- `shape`: emit final output contracts
- `axiom`: perform side effects and coordinate steps

That matters for ETL because many real pipelines begin as shell scripts, cron jobs, CI steps, or small batch tasks.
Users should be able to compose these tools directly with pipes, temp files, and standard Unix process control.

The binaries are therefore not just "developer utilities." They are a primary execution surface.

In implementation terms, `sda` is the priority binary.
It is the workhorse of the family and the first tool that should become genuinely solid.

## Common Conventions

The tools should converge on a shared CLI style.

Common flags:

- `-f, --file <path>`: load a program from a file
- `-e, --expr <text>`: run a short inline expression
- `-i, --input <path>`: read input from a file instead of stdin
- `-o, --output <path>`: write output to a file instead of stdout
- `--format <json|cbor|text>`: select input or output format where needed
- `--pretty`: pretty-print structured output
- `--explain`: emit plan or provenance information
- `--trace`: emit step-by-step runtime information
- `--check`: parse and validate only

Not every tool needs every flag in v0, but the family should feel consistent.

## `sda`

`sda` is the pure transformation CLI.

### Responsibilities

- parse an SDA program
- evaluate it against input data
- return the resulting value or failure
- support check/trace/explain modes for testing

### Proposed commands

```sh
sda eval -f extract.sda < event.json
sda eval -e 'Map{ "x" -> 1 }' < input.json
sda check -f extract.sda
sda --version
sda --license
sda fmt -f extract.sda
sda fmt --stdin-filepath extract.sda < extract.sda
sda fmt -f extract.sda --check
sda fmt -f extract.sda --write
```

### Minimum v0 subcommands

- `eval`: run a program
- `check`: parse and validate
- `--version`: print the shipped semantic version and build number
- `--license`: print copyright and license notice
- `fmt`: parse, validate, emit canonical SDA source, read from stdin for editor integrations, or enforce it with `--check` / `--write`

### Example

```sh
sda eval -f extract.sda < event.json > useful.json
```

## `enr`

`enr` evaluates enrichment and join programs.

In early versions, this may be a very small tool or even a thin wrapper over a library runtime.
It is not part of the first required shipping surface.

### Responsibilities

- load declared sources or source snapshots
- evaluate joins and lookup policies
- make missing and duplicate behavior explicit
- emit enriched output plus optional provenance

### Proposed commands

```sh
enr eval -f join.enr --left request.json --right customer.json
enr check -f join.enr
enr explain -f join.enr --left request.json --right customer.json
```

### Minimum v0 subcommands

- `eval`
- `check`
- `explain`

### Example

```sh
enr eval -f join-customer.enr --left request.json --right customer.json > enriched.json
```

## `shape`

`shape` emits final output contracts.

### Responsibilities

- consume normalized or enriched internal data
- construct final output documents or bytes
- validate output shape
- later support alternative encodings or layouts

### Proposed commands

```sh
shape eval -f result.shp < enriched.json
shape check -f result.shp
shape explain -f result.shp < enriched.json
```

### Minimum v0 subcommands

- `eval`
- `check`

This tool is post-v0 unless output shaping clearly diverges from plain SDA transformation.

### Example

```sh
shape eval -f result.shp < enriched.json > output.json
```

## `axiom`

`axiom` is the orchestration CLI.

This is the only effectful tool in the family.
It is also post-v0 if the immediate goal is to ship the SDA cornerstone first.

### Responsibilities

- run workflows
- manage stage inputs and outputs
- perform HTTP CRUD
- perform file CRUD
- apply retry, timeout, auth, and caching policy
- invoke `sda`, `enr`, and `shape` runtimes

### Proposed commands

```sh
axiom run -f workflow.ax
axiom check -f workflow.ax
axiom plan -f workflow.ax
axiom trace -f workflow.ax
axiom http --request http-request.json
```

### Minimum v0 subcommands

- `run`: execute a workflow
- `check`: validate workflow syntax and references
- `plan`: show stages and dependencies
- `trace`: run with step-level diagnostics

### Example

```sh
axiom run -f customer-lookup.ax --input event.json > output.json
```

## Shell-First Composition

The family should work well even before the `axiom` DSL exists.

Example:

```sh
sda eval -f extract.sda < event.json > request.json
sda eval -f derive-request.sda < request.json > http-request.json
axiom http --request http-request.json > service-response.json
sda eval -f normalize-customer.sda < service-response.json > customer.json
enr eval -f join-customer.enr --left request.json --right customer.json > enriched.json
shape eval -f result.shp < enriched.json > output.json
```

That path is important because it lets us validate the architecture before building the full orchestration language.

It also matters for ETL.
Many teams already have an orchestration environment and do not want a new workflow DSL immediately.
They may prefer:

- shell scripts
- Makefiles
- CI pipelines
- Airflow steps
- cron jobs
- container entrypoints

The separate binaries let them adopt the semantic layers one at a time.

## ETL Pattern

A simple ETL flow in Unix terms may look like:

```sh
cat raw-events.json \
  | sda eval -f normalize-event.sda \
  | sda eval -f extract-business-record.sda \
  > extracted.json

axiom http --request requests/customer-request.json \
  > customer-response.json

enr eval -f join-customer.enr --left extracted.json --right customer-response.json \
  | shape eval -f warehouse-row.shp \
  > warehouse-row.json
```

Or in a batch-oriented style:

```sh
sda eval -f extract.sda -i inbound/event.json -o work/request.json
axiom run -f fetch-customer.ax --input work/request.json -o work/customer.json
enr eval -f join.enr --left work/request.json --right work/customer.json -o work/enriched.json
shape eval -f emit.shp -i work/enriched.json -o outbound/result.json
```

This style gives operators:

- observable intermediate artifacts
- easy replay from captured data
- clear step boundaries
- compatibility with existing ETL infrastructure

## Library Parity

Each CLI should correspond to a library API with the same semantic boundary.

Conceptually:

- `sda_eval(program, input, opts)`
- `enr_eval(program, input, sources, opts)`
- `shape_eval(program, input, opts)`
- `axiom_run(workflow, input, runtime, opts)`

The CLI is not the product by itself. It is one delivery surface over the same engines.

## Suggested Build Priority

The first commands worth implementing are:

1. `sda eval`
2. `sda check`
3. `sda fmt`
4. `axiom http`
5. `axiom run`
6. `enr eval`
7. `shape eval`

The main point is that `sda` should become solid before the rest of the family grows around it.
