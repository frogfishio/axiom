# ETL And Unix Shell Use

## Thesis

The Axiom family should help not only embedded application runtimes, but also shell-driven ETL and batch processing.

That is one reason the system should exist as separate binaries, not only as libraries.

## Why Binaries Matter

In many environments, the first real workflow is not a compiled application.
It is:

- a shell script
- a CI job
- a cron task
- a Makefile target
- an Airflow step
- a container command

Those environments already know how to run binaries, pass files, pipe stdout, capture stderr, and track exit codes.

If the Axiom family provides good standalone tools, users can adopt them immediately without committing to:

- a new host language
- a new orchestration platform
- a new packaging system

## ETL Roles By Binary

### `sda`

In ETL, `sda` is the extraction and normalization worker.

Typical jobs:

- clean up inbound JSON
- normalize inconsistent shapes
- preserve duplicate semantics where needed
- derive stable keys
- map large payloads into smaller internal records
- validate assumptions before downstream stages

### `enr`

In ETL, `enr` is the explicit data-combination worker.

Typical jobs:

- join one dataset with another
- define required vs optional matches
- define duplicate handling
- produce explainable enriched records

### `shape`

In ETL, `shape` is the outbound contract emitter.

Typical jobs:

- emit warehouse rows
- emit API result shapes
- emit document-oriented outputs
- later emit fixed-width or binary formats

### `axiom`

In ETL, `axiom` is the side-effect and orchestration worker.

Typical jobs:

- fetch from services over HTTP
- read and write files
- manage retries and timeouts
- control multi-step data movement
- sequence transform and enrichment stages

## Unix Model

Each tool should behave like a good Unix program:

- read from stdin by default
- write to stdout by default
- use stderr for diagnostics
- return stable exit codes
- avoid hidden ambient behavior

That allows natural composition.

Example:

```sh
sda eval -f normalize.sda < inbound.json \
  | sda eval -f extract.sda \
  | shape eval -f outbound.shp \
  > outbound.json
```

## Observable Pipelines

A major benefit of separate binaries is observability.

Pipelines can expose intermediate artifacts:

- `request.json`
- `service-response.json`
- `enriched.json`
- `output.json`

That makes it easier to:

- debug failures
- replay incidents
- compare expected vs actual behavior
- build fixture-driven tests
- reason about where semantics live

This is especially valuable for Lambda-like workflows, where production data often needs to be replayed locally.

## ETL Example

A realistic shell-driven ETL flow:

```sh
sda eval -f extract.sda -i inbound/event.json -o work/request.json
sda eval -f derive-request.sda -i work/request.json -o work/http-request.json
axiom http --request work/http-request.json -o work/service-response.json
sda eval -f normalize-response.sda -i work/service-response.json -o work/customer.json
enr eval -f join-customer.enr --left work/request.json --right work/customer.json -o work/enriched.json
shape eval -f emit-result.shp -i work/enriched.json -o outbound/result.json
```

This is not a fallback mode.
It is one of the intended operating modes of the system.

## Relationship To Axiom Glue

The standalone binaries and the Axiom glue language are complementary.

The binaries help with:

- immediate adoption
- shell-based ETL
- transparent debugging
- use inside existing orchestrators

The Axiom DSL helps later with:

- reusable workflow definitions
- explicit stage wiring
- dependency planning
- controlled orchestration semantics

The architecture is stronger if both modes are first-class.

## Product Implication

The product should be understandable in two sentences:

- the semantic engines can be used directly as Unix-style ETL tools
- Axiom can later orchestrate them as a minimal workflow language

That keeps adoption simple and preserves a path to a richer system later.
