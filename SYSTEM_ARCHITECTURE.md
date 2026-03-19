# System Architecture Note

## Thesis

The system is not one language.
It is a family of semantic layers that cooperate over artefacts.

The central rhythm is:

- reduce
- expand
- reduce again
- orchestrate the movement between stages

In this family:

- `SDA` is reduction
- `ENR` is expansion
- `ENR+` is candidate-aware enrichment and explanation on top of ENR
- `Axiom` is orchestration

This note ties those layers together as one system.

## The Problem Shape

Real-world structured input is usually not the artefact you actually want.

Hosts such as AWS Lambda typically give you:

- wrapper-heavy event payloads
- transport metadata
- duplicate or conflicting paths to the same business fact
- partial or noisy records
- effect boundaries to external systems

The system therefore cannot be a single transform pass.

It needs alternating phases that:

1. remove host and transport noise
2. recover stable semantic artefacts
3. relate one artefact to another artefact or dataset
4. reduce the enriched result into the next stable artefact
5. repeat until the final business artefact is reached

## The Four Layers

### 1. SDA: Reduction Kernel

`SDA` is the certainty kernel for structured-data transformation.

Its job is to reduce noisy or over-structured inputs into semantically sharper artefacts.

`SDA` owns:

- extraction
- normalization
- validation
- derived values and keys
- explicit absence versus `Null`
- explicit multiplicity-aware carrier semantics

`SDA` does not own:

- acquisition
- orchestration
- HTTP or file effects
- hidden host policy

Practical role:

- take host noise in
- emit the first trustworthy artefact out

### 2. ENR: Expansion Kernel

`ENR` is the certainty kernel for relation.

Its job is to expand one reduced artefact against another artefact or dataset, making relational possibility explicit.

`ENR` owns:

- match-bag formation
- explicit multiplicity preservation
- explicit cardinality interpretation
- attach, merge, and expand semantics
- semantic source declarations

`ENR` does not own:

- acquisition
- retries
- orchestration
- hidden uniqueness assumptions

Practical role:

- take stable artefacts in
- produce explicit relational structure out

### 3. ENR+: Candidate And Explanation Layer

`ENR+` is the first serious extension over the ENR kernel.

Its job is to make enrichment operationally serious without losing the core.

`ENR+` adds:

- candidate bags
- refinement before resolution
- ranking and preference
- provenance and evidence
- explicit resolved outcomes
- explanation and quality metrics

This layer must preserve ENR core laws:

- no implicit uniqueness
- no implicit dropping
- null is not no match
- duplicates survive until explicit interpretation

Practical role:

- make ambiguity inspectable rather than hidden
- support serious operational matching and explanation

### 4. Axiom: Orchestration Layer

`Axiom` is the effectful glue language.

Its job is to acquire artefacts, route them through semantic stages, and manage the workflow around those stages.

`Axiom` owns:

- stage sequencing
- named artefacts
- conditions and guards
- HTTP and file effects
- retries, timeouts, auth, and caching policy
- invocation of `SDA`, `ENR`, `ENR+`, and later shaping

`Axiom` does not own the inner semantics of transformation or relation.

Practical role:

- move artefacts between pure stages
- make effect boundaries explicit

## The System Rhythm

The family works by alternating reduction and expansion passes.

Conceptually:

```text
raw input
  -> SDA
  -> artefact A

raw input 2
  -> SDA
  -> artefact B

(A, B)
  -> ENR
  -> relational artefact E

E
  -> SDA
  -> artefact C

C and more sources
  -> ENR or ENR+
  -> relational artefact E2

E2
  -> SDA
  -> final artefact
```

This is not accidental.
It is the intended architectural pattern.

## Lambda Host Picture

The easiest way to visualize the system is to imagine `Axiom` running inside AWS Lambda.

In that picture:

- Lambda provides the invocation boundary and host environment
- Axiom receives the event and controls the workflow inside the invocation
- SDA removes platform noise and emits usable artefacts
- ENR expands those artefacts against related datasets
- SDA reduces enriched structures into the next artefact
- Axiom decides whether further effects or stages are needed
- Lambda returns the final emitted result

One invocation might look like this:

```text
AWS event
  -> decode
  -> SDA extract
  -> artefact A

artefact A
  -> derive request pieces with SDA
  -> Axiom performs HTTP call
  -> raw response
  -> SDA normalize
  -> artefact B

(A, B)
  -> ENR
  -> relational artefact E

E
  -> SDA refine
  -> artefact C

artefact C
  -> emit response
```

That is the concrete operating model.

## Boundary Rules

The family stays coherent only if these boundaries hold.

1. `SDA` stays pure.
2. `ENR` stays pure.
3. `ENR+` may add operational seriousness, but may not redefine the ENR kernel.
4. `Axiom` owns effects and stage routing, not transformation or relation semantics.
5. No layer may hide multiplicity or absence behind convenience defaults.
6. Stable semantic failures belong in the semantic layers; transport and invocation concerns belong in the host or orchestration layer.

## Artefacts As The Shared Center

The system should be thought of as artefact-oriented.

Each layer consumes an artefact and emits another artefact with a different semantic role.

Typical artefact classes are:

- raw host artefacts
- reduced semantic artefacts
- relational artefacts
- candidate-aware relational artefacts
- final emitted artefacts

This matters because the layers are not interchangeable.

- `SDA` reduces artefacts
- `ENR` expands artefacts into relation space
- `ENR+` makes that relation space inspectable and policy-aware
- `Axiom` routes artefacts through the system

## Why The Split Matters

If the layers collapse into one language, the system loses the very guarantees it is being built to provide.

Without the split:

- transformation logic leaks into orchestration
- relation policy leaks into host code
- effects leak into semantics
- multiplicity gets hidden behind convenience
- noise and meaning become mixed again

With the split:

- host noise is reduced explicitly
- relation is made explicit
- ambiguity is preserved until interpreted
- effects are visible and controlled
- the system can alternate reduction and expansion without losing certainty

## Short Form

If the whole family needs one compact statement, it is this:

```text
SDA reduces noise into artefacts.
ENR expands artefacts into explicit relation.
ENR+ explains and resolves that relation without hiding it.
Axiom orchestrates the movement between stages and owns effects.
```

That is the system.