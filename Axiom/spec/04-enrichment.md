# 04. Enrichment (Normative)

## 04.0 Thesis

SDA is reduction.
Enrichment is expansion.

Enrichment relates one reduced artefact to another artefact or dataset without hiding multiplicity, uniqueness assumptions, or source semantics.

If SDA is the certainty kernel for transformation, Enrichment is the certainty kernel for relation.

Enrichment stays pure.

It does not perform:

- acquisition
- orchestration
- retries
- auth
- caching
- HTTP or file IO

Those belong to Axiom.

Rule: all enrichment is expressed as formation of a match bag plus explicit interpretation of that bag.

## 04.1 Primitive Semantic Object

For:

- left value `l`
- right dataset `R`
- left key function `kL`
- right key function `kR`

define:

```text
Match(l, R, kL, kR) = { r ∈ R | kR(r) = kL(l) }
```

Requirements:

- `Match(...)` MUST preserve duplicates from `R`
- `Match(...)` MUST produce a `Bag`
- equality in the predicate MUST use SDA equality
- `Null` is a value, not absence
- an empty match bag means no match

Rule: the primitive enrichment result is always a bag of matches.

## 04.2 Core Surface Form

The canonical surface form is:

```text
{ r ∈ R | kR(r) = kL(l) }
```

Example:

```text
{ c ∈ customers | c.id = l.customer_id }
```

Requirements:

- `l` is the current left value
- `R` is a dataset value
- `kL` and `kR` are SDA expressions
- the result is a `Bag`

This form is the normative basis of all other enrichment operators.

## 04.3 Cardinality Operators

Cardinality operators interpret a match bag.

### 04.3.1 `one?`

```text
one?(B)
```

Meaning:

- `count(B) = 0 -> None`
- `count(B) = 1 -> Some(v)`
- `count(B) > 1 -> Fail(t_enr_duplicate, "duplicate match")`

Type:

```text
Bag[T] -> Opt[T] | Fail
```

### 04.3.2 `one!`

```text
one!(B)
```

Meaning:

- `count(B) = 0 -> Fail(t_enr_missing, "missing match")`
- `count(B) = 1 -> v`
- `count(B) > 1 -> Fail(t_enr_duplicate, "duplicate match")`

Type:

```text
Bag[T] -> T | Fail
```

### 04.3.3 `first`

```text
first(B)
```

Meaning:

- `count(B) = 0 -> None`
- `count(B) >= 1 -> first element under source-defined order`

Type:

```text
Bag[T] -> Opt[T]
```

If source order is not defined, evaluation MUST fail with:

```text
Fail(t_enr_unordered_policy, "unordered policy")
```

### 04.3.4 `last`

```text
last(B)
```

Meaning:

- `count(B) = 0 -> None`
- `count(B) >= 1 -> last element under source-defined order`

Type:

```text
Bag[T] -> Opt[T]
```

If source order is not defined, evaluation MUST fail with:

```text
Fail(t_enr_unordered_policy, "unordered policy")
```

### 04.3.5 `only`

```text
only(B)
```

Meaning:

- `count(B) = 1 -> v`
- otherwise -> failure

Type:

```text
Bag[T] -> T | Fail
```

This is the stricter exact-uniqueness spelling.

Rule: enrichment MUST NOT collapse a match bag implicitly.

## 04.4 Row Combination Operators

These operators specify how interpreted matches enter the result.

### 04.4.1 Attach

```text
l + { name: E }
```

Meaning:

- evaluate `l`
- evaluate `E`
- return a product equal to `l` with field `name` attached

Requirements:

- `l` MUST be `Prod`
- `name` MUST be a valid field selector
- default field-collision policy in v0.1 is failure

On collision, evaluation MUST fail with:

```text
Fail(t_enr_field_collision, "field collision")
```

Example:

```text
l + { customer: one!({ c ∈ customers | c.id = l.customer_id }) }
```

### 04.4.2 Merge

```text
merge(l, r)
```

Meaning:

- both operands are products
- return a combined product

Requirements:

- both operands MUST be `Prod`
- default collision policy in v0.1 is failure

On collision, evaluation MUST fail with:

```text
Fail(t_enr_field_collision, "field collision")
```

If either operand is not a product, evaluation MUST fail with:

```text
Fail(t_enr_wrong_shape, "wrong shape")
```

### 04.4.3 Expand

Expansion is expressed by yielding over the match bag.

Canonical form:

```text
{ yield merge(l, r) | l ∈ L, r ∈ { s ∈ R | kR(s) = kL(l) } }
```

Rule: expansion is not a hidden join mode; it is comprehension over explicit multiplicity.

## 04.5 Dataset-Level Enrichment

Dataset-level enrichment is expressed by SDA-style comprehensions over left rows.

### 04.5.1 Attach required one

```text
{ yield l + { customer: one!({ c ∈ customers | c.id = l.customer_id }) } | l ∈ orders }
```

### 04.5.2 Attach optional one

```text
{ yield l + { customer: one?({ c ∈ customers | c.id = l.customer_id }) } | l ∈ orders }
```

### 04.5.3 Attach grouped many

```text
{ yield l + { items: { i ∈ items | i.order_id = l.id } } | l ∈ orders }
```

### 04.5.4 Expand many

```text
{ yield merge(l, i) | l ∈ orders, i ∈ { i ∈ items | i.order_id = l.id } }
```

If multi-generator comprehensions are not supported in the underlying SDA surface, a conforming host MAY provide an equivalent enrichment expansion form, but MUST preserve the same semantics.

Rule: grouped enrichment preserves the left row count; expansion multiplies rows by match multiplicity.

## 04.6 Sources

Enrichment source declarations are semantic, not operational.

Minimum source kinds:

- `Index[K,V]`
- `MultiIndex[K,V]`
- `Dataset[V]`

Meaning:

- `Index[K,V]` declares unique-key semantics
- `MultiIndex[K,V]` declares duplicate keys may occur
- `Dataset[V]` declares no uniqueness semantics

Example:

```text
source customers : Index[Str, Customer]
source items     : MultiIndex[Str, Item]
```

Requirements:

- source declarations MUST NOT encode transport
- source declarations MUST NOT encode auth, retries, caching, timeouts, or HTTP behavior
- acquisition of source values belongs to Axiom

Rule: source declarations state match semantics only.

## 04.7 Carriers

### 04.7.1 Match carrier

The primitive match result MUST be `Bag`.

### 04.7.2 Left-carrier preservation

For attachment-style enrichment:

- `Seq -> Seq`
- `Bag -> Bag`
- `Set -> Set`

For expansion-style enrichment:

- `Seq -> Seq`
- `Bag -> Bag`
- `Set -> Bag`

This is normative.

## 04.8 Failures

Enrichment MUST provide stable failures.

Required failure codes:

- `t_enr_missing` — required match missing
- `t_enr_duplicate` — uniqueness violated
- `t_enr_unordered_policy` — `first` or `last` used without defined order
- `t_enr_field_collision` — illegal attach or merge collision
- `t_enr_wrong_shape` — invalid operand shape
- `t_enr_invalid_key` — invalid key type or key evaluation failure

Hosts MAY define additional enrichment failure codes, but MUST NOT change the meaning of the required codes.

## 04.9 Laws

### 04.9.1 Primitive reduction law

All enrichment forms reduce to:

```text
{ r ∈ R | kR(r) = kL(l) }
```

plus explicit interpretation of the resulting bag.

### 04.9.2 No implicit uniqueness

A match bag MUST NOT be implicitly reduced to one value.

### 04.9.3 No implicit dropping

A left row MUST NOT disappear unless the enclosing expression explicitly omits it.

### 04.9.4 Null is not no match

If a matched row contains `Null`, that row still counts as a match.

### 04.9.5 Duplicate preservation

Duplicates in `R` MUST remain duplicates in the match bag until an explicit operator resolves or rejects them.

## 04.10 Extension Boundary

The following belong to the first serious extension layer over the ENR kernel, not to the irreducible core defined above:

- candidate bags
- candidate refinement operators
- explicit resolution objects
- provenance-rich decision structures
- explanation and quality reporting

These extensions are valuable, but they MUST preserve the core laws above.

In particular, they MUST NOT:

- hide multiplicity
- collapse a match bag implicitly
- replace explicit relation formation with opaque host policy

## 04.11 Minimal Useful Subset

The smallest serious first version of enrichment is:

- match-bag comprehension
- `one?`
- `one!`
- `first`
- `last`
- `only`
- attach via `l + { field: E }`
- `merge(l, r)`
- dataset comprehensions using `yield`
- semantic source declarations

That is enough to express:

- optional lookup
- required lookup
- grouped one-to-many
- expanded one-to-many

without prematurely turning enrichment into a large decision-analysis language.
