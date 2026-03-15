# 01. Values, Carriers, and Equality (Normative)

Axiom manipulates **tree values** and (optionally) **streams** of tree values.

## 01.1 Value kinds

Axiom defines the following value kinds:

- **Null**
- **Bool**: `true | false`
- **Num**: exact numeric values (host chooses representation; MUST preserve equality)
- **Str**: Unicode strings
- **Bytes**: raw bytes
- **Seq[T]**: ordered sequence (duplicates allowed)
- **Set[T]**: mathematical set (unique, unordered)
- **Bag[T]**: multiset (duplicates allowed, unordered)
- **Map[K,V]**: mapping with unique keys (unordered)
- **Prod{...}**: product/record with named fields; may be schema-known
- **Bind(K,V)**: first-class key/value binding value
- **BagKV[K,V]**: multiset of `Bind(k,v)` values (duplicates of keys allowed)
- **Opt[T]**: `Some(v)` or `None`
- **Res[T]**: `Ok(v)` or `Fail(code,msg)`
- **Stream[T]** (Optional in v0.1): potentially unbounded ordered stream

Conformance note:
- If `Stream` is not implemented, hosts MUST reject programs requiring it.

## 01.2 Null vs absence (Critical)

`Null` is a **data value**. It is NOT a synonym for absence.

- **Absence** means there is no binding for a key/field in a carrier.
- **Null** means a binding exists and its value is `Null`.

Selectors + eliminators (`<k>`, `<k>?`, `<k>!`) operate on **bindings**, not on stored values.

Example:
- If `m = Map{ "x" -> Null }`, then `m<"x">? = Some(Null)` and `m<"x">! = Ok(Null)`.
- If `m = Map{ }`, then `m<"x">? = None` and `m<"x">! = Fail(t_sda_missing_key, "missing key")`.

## 01.3 Equality

`=` compares values for equality.

Requirements:
- Equality MUST be reflexive, symmetric, transitive.
- For `Set`, equality is extensional (same members).
- For `Bag`, equality is extensional with multiplicity.
- For `Seq`, equality is positional.
- For `Map`, equality is extensional (same bindings).
- For `Prod`, equality is extensional over its field bindings.
- For `Bytes`, equality is bytewise.

Opaque host values MUST NOT appear inside positions that require equality (e.g., Set membership). If a host embeds opaque values,
it MUST prevent their use in Set/Map keys and in membership checks.

## 01.4 Key domains

Standalone Axiom:
- `Map` keys MUST be `Str`.
- `BagKV` keys MUST be `Str`.

Hosts MAY extend key domains (e.g., Symbols) but MUST document syntax and equality.
