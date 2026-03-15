# 00. Introduction (Normative)

Axiom is a small, formal, portable language for transforming and enriching structured data at scale.

It is designed to be embedded into hosts (batch systems, stream processors, ETL tools, data pipelines)
and to compile to multiple backends. Axiom programs are **pure and deterministic** by default:
there is no ambient IO. External interaction is expressed only through host-provided inputs and
host-provided **Sources** (immutable datasets with declared semantics).

Unless otherwise stated, this specification is **Normative**.

## 00.1 Normative vs Informative

- **Normative** sections contain requirements using MUST/SHALL/SHOULD.
- **Informative** sections provide guidance, rationale, and examples.

If a section is not explicitly marked Informative, it is Normative.

## 00.2 Observational model

- A program evaluates to either a **value** or `Fail(code, msg)`.
- The only observable behavior is the final value (including wrappers `Opt`/`Res`).
- There is no ambient IO.
- Order is observable only for `Seq` and `Stream` (if implemented).
- `Set`, `Bag`, `Map`, `BagKV`, and `Prod` are compared extensionally (see Equality).

## 00.3 Design constraints

Axiom MUST:
1. Distinguish **absence** from `Null`.
2. Preserve duplicates until a **policy** is explicitly applied.
3. Provide stable failure codes/messages suitable for conformance testing.
4. Avoid hidden coercions between carriers (e.g., BagKV → Map).
5. Make enrichment explicit, versioned, deterministic, and explainable.
6. Provide a declarative binary boundary story (PIC codecs) with stable failures.

## 00.4 Non-goals (v0.1)

- General-purpose scripting runtime
- Ambient IO primitives (files, network, database connections)
- Implicit schema inference
- User-defined mutation/state
- Full dependent typing
- Macro system (hosts may add)
