# Axiom Core Draft

## Purpose

This document defines a minimal core for Axiom as the effectful orchestration layer over SDA, ENR, and ENR+.

It is not a full syntax specification.
It is a semantic-core draft.

## 1. Core Idea

Axiom is a language of staged artefact flow.

It does not primarily express transformation meaning itself.
It expresses:

- which artefacts exist
- which stage consumes which artefact
- which effects happen between stages
- under what conditions a stage runs
- what policy applies to each effectful action
- which artefact is ultimately emitted

Conceptually, Axiom is to artefact pipelines what SDA is to transformation and ENR is to relation.

## 2. Minimal Semantic Objects

### 2.1 Artefact

An artefact is a named runtime value at a workflow boundary.

Examples:

- decoded event payload
- SDA result
- HTTP response body after normalization
- enrichment output
- final response artefact

### 2.2 Stage

A stage is a named step that consumes input artefacts and produces an output artefact or effect outcome.

Minimum stage classes:

- `decode`
- `sda`
- `enr`
- `enr+`
- `http`
- `file`
- `emit`

Hosts may provide more stage classes, but these capture the minimum architecture.

### 2.3 Guard

A guard is an explicit condition controlling whether a stage or branch executes.

It is evaluated over already-bound artefacts.

### 2.4 Policy

A policy is explicit workflow-visible configuration governing an effectful stage.

Examples:

- retry count
- timeout
- auth mode
- cache mode

Policies are orchestration concerns, not SDA or ENR semantics.

## 3. Minimal Stage Semantics

### 3.1 Decode Stage

Purpose:

- turn a host-provided raw value into a carrier value or artefact suitable for SDA/ENR stages

This is where host noise begins to enter the semantic pipeline.

### 3.2 SDA Stage

Purpose:

- invoke an SDA program on an artefact
- produce a reduced artefact

Rule:

The meaning of the transform itself belongs to SDA, not to Axiom.

### 3.3 ENR Stage

Purpose:

- invoke ENR over one or more artefacts
- produce a relational artefact

Rule:

Match and multiplicity semantics belong to ENR, not to Axiom.

### 3.4 ENR+ Stage

Purpose:

- invoke candidate-aware or explanation-rich enrichment logic
- produce a candidate-aware or resolved relational artefact

Rule:

Candidate and resolution semantics belong to ENR+, not to Axiom.

### 3.5 HTTP Stage

Purpose:

- perform explicit external request or CRUD action
- produce a raw response artefact

This is an effectful stage.

Policy examples:

- timeout
- retries
- auth
- cache

### 3.6 File Stage

Purpose:

- perform explicit file read or write at a workflow boundary

This is also effectful.

### 3.7 Emit Stage

Purpose:

- declare the workflow result artefact
- return or emit it through the host boundary

## 4. Workflow Shape

A minimal Axiom workflow can express:

1. input artefact binding
2. stage invocation
3. guarded branch or conditional stage execution
4. stage output naming
5. explicit effect stage policy
6. final output emission

Conceptually:

```text
input -> decode -> a
a -> sda(extract) -> b
when condition(b):
  b -> http(request) -> raw
  raw -> sda(normalize) -> c
  (b, c) -> enr(join) -> e
  e -> sda(refine) -> out
emit out
```

This is the intended core pattern.

## 5. Host Model

Axiom is host-embedded.

The clearest host model is AWS Lambda:

- the host provides event and context
- Axiom runs inside the invocation boundary
- Axiom stages acquire and route artefacts
- SDA and ENR perform pure semantic work
- the host receives the final output artefact

This host model is illustrative, not exclusive.

## 6. Boundary Rules

### 6.1 Pure Semantic Delegation

Axiom MUST delegate:

- transformation semantics to SDA
- relation semantics to ENR
- candidate-aware relation and explanation semantics to ENR+

### 6.2 Explicit Effects

Axiom MUST make network, file, and other external actions explicit as effectful stages.

### 6.3 No Hidden Semantic Drift

Axiom MUST NOT reinterpret SDA or ENR failure semantics as different transform or relation meanings.

### 6.4 Artefact Visibility

Intermediate artefacts should be nameable and routable.

This is important for:

- debugging
- replay
- fixture-driven testing
- local incident reconstruction

## 7. Minimal Laws

### 7.1 Stage Boundary Law

Every workflow transition occurs through an explicit stage boundary.

### 7.2 Effect Boundary Law

Every effectful action is visible in workflow semantics.

### 7.3 Delegation Law

Axiom controls when a semantic engine runs, not what that engine means.

### 7.4 Artefact Flow Law

Stage outputs become named artefacts available to later stages and guards.

### 7.5 Policy Visibility Law

Retry, timeout, auth, cache, and similar behaviors must be attached to effectful stages explicitly or through explicit profile defaults.

## 8. Minimal Useful Axiom Core

The smallest serious first version of Axiom needs:

- named artefacts
- `let`-style stage bindings
- decode stage
- SDA stage invocation
- ENR stage invocation
- HTTP stage invocation
- guard or `when` form
- explicit output or `emit`
- effect policy attachment

That is enough to express the intended Lambda-style orchestration model without turning Axiom into a general-purpose host language.

## 9. Relationship To The Family

The family roles remain:

- `SDA` reduces noise into artefacts
- `ENR` expands artefacts into explicit relation
- `ENR+` resolves and explains richer candidate-aware relation
- `Axiom` orchestrates the movement between stages and owns effects

That is the system Axiom is meant to serve.