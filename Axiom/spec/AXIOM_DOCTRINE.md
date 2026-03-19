# Axiom Doctrine

## Thesis

Axiom is the orchestration layer of the family.

If:

- `SDA` is reduction
- `ENR` is expansion
- `ENR+` is candidate-aware enrichment and explanation

then `Axiom` is the effectful glue that acquires artefacts, routes them through those semantic engines, and makes workflow state and policy explicit.

Axiom is not another transformation language.
Axiom is not another matching language.
Axiom is the orchestration language around pure semantic kernels.

## Why Axiom Exists

Real workflows must do more than transform one value.

They must:

- receive host input
- decode or parse raw payloads
- run reduction stages
- decide whether more context is needed
- call external systems
- normalize responses
- enrich one artefact against another
- reduce again
- return or emit a final artefact

That is too much responsibility for SDA or ENR.

So Axiom exists to coordinate those stages while keeping:

- transformation semantics in SDA
- relation semantics in ENR
- candidate-aware explanation in ENR+

## Host Picture

The best mental model is Axiom inside AWS Lambda.

In that picture:

- the host provides the invocation boundary
- Axiom receives an event and context
- Axiom names and routes artefacts through staged work
- SDA reduces host noise into meaningful artefacts
- ENR expands artefacts into relation space
- SDA reduces again
- Axiom performs required effects between those pure passes
- the host returns the final emitted result

That is the role.

## What Axiom Owns

Axiom owns:

- named artefacts
- stage sequencing
- effectful acquisition and emission
- guards and workflow routing
- decoding and encoding boundaries
- retries, timeouts, auth, and caching policy
- invocation of SDA, ENR, ENR+, and later shaping
- workflow-visible state transitions

## What Axiom Must Not Own

Axiom must not:

- absorb SDA transformation semantics
- absorb ENR matching semantics
- hide effectful calls behind transformation-looking syntax
- become a general-purpose imperative runtime by default
- silently reinterpret artefacts or carrier semantics
- move multiplicity or absence policy out of SDA/ENR and into host-like control code

If Axiom becomes a second transform or join language, the family loses the point of its separation.

## Non-Negotiable Principle

Axiom must make effect boundaries explicit.

That means:

- pure stage invocation is explicit
- effectful acquisition is explicit
- routing conditions are explicit
- named artefacts are explicit
- policy on side effects is explicit

No hidden network call should look like a pure transform.

## Artefact-Oriented Model

The right center for Axiom is not variables in the abstract.
It is artefacts moving through stages.

The system works because each stage consumes and emits artefacts with clearer semantic roles.

Typical artefact classes:

- raw host artefacts
- reduced artefacts
- relational artefacts
- resolved or candidate-aware relational artefacts
- final output artefacts

Axiom's job is to route these artefacts between stages without redefining their inner semantics.

## Minimal Axiom Responsibility

At minimum, Axiom should be able to express:

1. bind input artefacts
2. decode or normalize raw host values into stage inputs
3. invoke SDA over artefacts
4. invoke ENR or ENR+ over artefacts
5. perform explicit HTTP or file operations
6. branch or guard on explicit conditions
7. bind stage outputs to names
8. emit final output artefacts

That is enough to express the intended Lambda-style workflow without turning Axiom into a general programming language.

## Family Relationship

The family rhythm is:

- host noise arrives
- SDA reduces it
- ENR expands it against other reduced artefacts
- SDA reduces again
- Axiom orchestrates the movement and owns the effects

In short:

```text
SDA reduces.
ENR expands.
ENR+ explains and resolves richer relation.
Axiom acquires, routes, and emits.
```

## Standard For A Sound Axiom Core

Axiom is in a sound core state only when:

1. effect boundaries are explicit
2. stage invocation boundaries are explicit
3. artefacts are named and routed explicitly
4. SDA and ENR semantics remain delegated, not duplicated
5. host integration details do not redefine pure stage meaning
6. retries, auth, timeouts, caching, and emission are policy-visible rather than ambient

That is the bar.