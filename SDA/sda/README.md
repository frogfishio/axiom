# SDA Implementation Note

The active SDA implementation no longer lives in this nested workspace.

Use the top-level crates instead:

- `crates/sda-core`
- `crates/sda-cli`

This directory remains under `SDA/` only as part of the specification area. The implementation backlog is tracked in `GAPS.md`, and development commands are documented in `DEVELOPMENT.md`.
                 | NOT Not ;

3.3 Comparisons

Cmp            ::= Setish
                 | Cmp EQ  Setish
                 | Cmp NEQ Setish
                 | Cmp LT  Setish
                 | Cmp LE  Setish
                 | Cmp GT  Setish
                 | Cmp GE  Setish ;

3.4 Set/bag/seq operators and membership

Setish         ::= Add
                 | Setish UNION  Add
                 | Setish INTER  Add
                 | Setish DIFF   Add
                 | Setish BUNION Add
                 | Setish BDIFF  Add
                 | Add IN Add ;

Notes:
	•	IN is membership (tokenized from ∈ or in).
	•	UNION/INTER/DIFF/BUNION/BDIFF are the keyword forms (from either glyph or ASCII name, lex-normalized).

3.5 Arithmetic (optional but consistent)

Add            ::= Mul
                 | Add PLUS  Mul
                 | Add MINUS Mul ;

Mul            ::= Unary
                 | Mul STAR  Unary
                 | Mul SLASH Unary ;

Unary          ::= Postfix
                 | MINUS Unary ;

3.6 Postfix (selector access, calls)

Postfix        ::= Primary { PostfixOp } ;

PostfixOp      ::= SelectorAccess
                 | CallSuffix ;

CallSuffix     ::= LPAREN [ ArgList ] RPAREN ;

ArgList        ::= Expr { COMMA Expr } ;

Selector access

SelectorAccess ::= SEL_L Selector SEL_R [ QMARK | BANG ] ;

	•	No suffix = total projection (only valid on Prod / schema-known).
	•	? = optional extraction
	•	! = required extraction

Selector       ::= IDENT
                 | STRING ;

(Additional rule from your spec: bare IDENT selector means the label text; that’s semantic, but the grammar accepts it.)

---

4) Primary forms

Primary        ::= Literal
                 | IDENT
                 | PLACEHOLDER
                 | Lambda
                 | ParenExpr
                 | Comprehension ;

ParenExpr      ::= LPAREN Expr RPAREN ;

4.1 Lambda (single argument)

Lambda         ::= IDENT LAMBDA Expr ;

	•	LAMBDA is ↦ or => normalized.

---

5) Literals and carriers

Literal        ::= NULL | TRUE | FALSE | NUM | STRING
                 | SeqLit
                 | SetLit
                 | BagLit
                 | MapLit
                 | ProdLit
                 | BagKVLit
                 | OptLit
                 | ResLit ;

5.1 Seq / Set / Bag

SeqLit         ::= SEQ LBRACK [ ExprList ] RBRACK ;

SetLit         ::= SET LBRACE [ ExprList ] RBRACE ;

BagLit         ::= BAG LBRACE [ ExprList ] RBRACE ;

ExprList       ::= Expr { COMMA Expr } ;

5.2 Map (standalone restriction: keys are STRING only)

MapLit         ::= MAP LBRACE [ MapEntryList ] RBRACE ;

MapEntryList   ::= MapEntry { COMMA MapEntry } ;

MapEntry       ::= STRING ARROW Expr ;

	•	ARROW is → or ->, normalized.
	•	The “STRING-only keys in standalone” is enforced by grammar here.

5.3 Prod

ProdLit        ::= PROD LBRACE [ ProdFieldList ] RBRACE ;

ProdFieldList  ::= ProdField { COMMA ProdField } ;

ProdField      ::= IDENT COLON Expr ;

5.4 BagKV

BagKVLit       ::= BAGKV LBRACE [ BindEntryList ] RBRACE ;

BindEntryList  ::= BindEntry { COMMA BindEntry } ;

BindEntry      ::= Selector ARROW Expr ;

	•	This matches your current surface: BagKV{ k -> v, ... } where k can be IDENT or STRING (since selector positions allow both).

5.5 Option/Result wrappers

OptLit         ::= SOME LPAREN Expr RPAREN
                 | NONE ;

ResLit         ::= OK   LPAREN Expr RPAREN
                 | FAIL LPAREN Expr COMMA Expr RPAREN ;

(If you want Fail(code,msg) to require code be a STRING, do it in semantics or constrain here.)

---

6) Comprehensions

Comprehensions use { ... }, which overlaps with Set literal braces, so we disambiguate by the presence of IN/∈ and |/∣ in the body.

6.1 Comprehension surface

Comprehension  ::= LBRACE ComprBody RBRACE ;

ComprBody      ::= ComprNoYield
                 | ComprWithYield ;

ComprNoYield   ::= IDENT IN Expr [ ComprPred ] ;

ComprWithYield ::= YIELD Expr BAR IDENT IN Expr [ ComprPred ] ;

ComprPred      ::= BAR Expr ;

Notes:
	•	BAR is | or ∣, lex-normalized.
	•	This matches:
	•	{ a ∈ A ∣ P(a) }
	•	{ yield E(a) ∣ a ∈ A ∧ P(a) }

6.2 Important disambiguation rule (normative)

If the parser sees { … } and the body can be parsed as a comprehension (i.e., it contains a generator IDENT IN Expr), it MUST parse as Comprehension, otherwise it is one of:
	•	SetLit / BagLit / MapLit / ProdLit / BagKVLit depending on the leading keyword token (SET, BAG, etc.)
	•	Plain { ... } without a leading carrier keyword is reserved (or parse error), unless you explicitly add “bare set literal” in the future. (Right now your spec uses Set{...} not {...}.)

---

7) Bison-friendly precedence table (suggested)

In bison terms (highest to lowest; pick exact):
	•	Postfix selector/call binds tightest
	•	unary -
	•	* /
	•	+ -
	•	IN and set/bag ops (UNION/INTER/DIFF/BUNION/BDIFF)
	•	comparisons = != < <= > >=
	•	NOT
	•	AND
	•	OR
	•	PIPE (|>)

Also declare PIPE left-assoc.

---

8) Reserved identifier constraints (semantic rules)

These aren’t grammar necessities, but they’re spec-level constraints you probably want stated:
	•	PLACEHOLDER (_/•) is not an IDENT and cannot appear as:
	•	let _ = ...
	•	lambda parameter
	•	map key (already impossible with STRING-only map keys)
	•	keyword tokens (let, yield, in, etc.) are not identifiers.



# SDA SPEC (standalone)

SDA = **Structured Data Algebra**.

SDA is a small, formal, portable language for transforming structured data.

SDA is **not** a database/query language (SQL/Mongo) and not a scripting runtime.
Its purpose is to give hosts a *closed, testable algebra* for reshaping, validating,
and normalizing structured data without hidden coercions.

### What SDA is for (Normative)

SDA exists to make common “tree in → tree out” transformations precise and portable (JSON/CBOR/MsgPack/Protobuf after parsing).
It is designed to be embedded into other languages and runtimes so that:

- hosts can standardize data semantics over tree carriers (absence vs `Null`, duplicates, ordering)
- tooling can reason about transformations (parsing, normalization boundaries, failure modes)
- implementations can be conformance-tested against stable behavior

### How SDA is used (Normative)

A host uses SDA in one of two ways:

1. **Standalone execution**: the host provides an input tree value and an SDA program; the program
   evaluates to a value (or `Fail(code,msg)`).
2. **Embedded execution**: a larger language treats SDA as an embedded expression form (e.g.
   delimited by host syntax such as `⟪ ... ⟫`), evaluates it against a host value, and receives
   the resulting value.

In both modes, SDA is pure: it cannot perform ambient IO. Any external interaction is done by
host-provided inputs and host-installed functions.
It is designed to be:

- **algebraically explicit** (no hidden coercions)
- **shape- and cardinality-honest** (absence/duplicates are first-class)
- **host-agnostic** (works over any “tree” representation)
- **deterministic** (no ambient IO; host provides data + caps)
- **conformance-testable** (stable errors, stable semantics)

SDA is intended to be embeddable in other languages, but this document specifies
SDA as a **standalone** language.

---

## 0.0 Normative vs informative

This specification uses two kinds of requirements:

- **Normative**: required for conformance (MUST/SHALL).
- **Informative**: rationale, examples, and non-binding guidance.

Unless a section is explicitly marked **Informative**, it should be treated as **Normative**.

### 0.1 Observational model (Normative)

SDA is a pure, deterministic transformation language.

- A program evaluates to either a **value** or a `Fail(code, msg)`.
- The only observable result is the final value (including `Ok/Fail` wrappers if present).
- There is no ambient IO. Any host interaction is via host-provided input values and host-installed functions.
- Order is observable only for `Seq`. `Set`, `Bag`, `Map`, and `Prod` are compared extensionally (see §1.2).

## 0.2 Comments and lexing

- Line comments begin with `;;` and continue to the end of the line.
- SDA is whitespace-insensitive except where whitespace is required to separate tokens.
- String literals support at least the following escape sequences: `\"`, `\\`, `\n`, `\t`.

## 0.3 Notation

- Unicode operators: `∈ ∪ ∩ \ ∧ ∨ ¬ ≠ ≤ ≥ → ↦ ⟨ ⟩ ⊆ ∣ ⊎ ⊖ •`
- ASCII fallbacks:
  - `∈` -> `in`
  - `∧` -> `and`
  - `∨` -> `or`
  - `¬` -> `not`
  - `≠` -> `!=`
  - `→` -> `->`   ;; exact synonym
  - `↦` -> `=>`   ;; exact synonym
  - `⟨x⟩` -> `<x>`
  - `∣` -> `|`
  - `⊎` -> `bunion`
  - `⊖` -> `bdiff`
  - `•` -> `_`
- The following ASCII operator spellings are reserved keywords and not identifiers:
  - `->`, `=>`
  - `union`, `inter`, `diff`
  - `bunion`, `bdiff`

Conformance note (Normative):
For the following token pairs, the Unicode and ASCII spellings are exact synonyms and MUST parse to the same AST nodes:
- `→` and `->` (binding / map entry / BagKV entry sugar)
- `↦` and `=>` (lambda)
No other Unicode/ASCII equivalence is implied by this section unless explicitly listed.

Unless otherwise stated, examples show Unicode first and ASCII in comments.

---

## 1. Core data model

SDA manipulates **tree values**. A host provides input values (parsed from JSON/CBOR/etc.) and consumes output
values. SDA itself has no ambient IO.

### 1.1 Value kinds

SDA defines the following value kinds:

- **Null**: the absence of a value. (SDA prefers explicit absence; see §4.)
- **Bool**: `true | false`
- **Num**: exact numeric values (host chooses representation; must preserve equality)
- **Str**: Unicode string
- **Bytes**: raw bytes
- **Seq**: ordered sequence of values — a finite list `[v₀, v₁, …, vₙ₋₁]`; order and duplicates are significant.
- **Set**: mathematical set — a finite collection of unique, unordered values `{v₀, …}`.
- **Bag**: multiset — a finite collection where duplicates are allowed and order is insignificant.
- **Map**: finite *partial* function from `Str` keys to values (unique keys, no defined order). See §3.4.
- **Prod**: finite *total* function over a *fixed label set* (named fields). See §3.5.
- **BagKV**: bag of `Bind(k,v)` pairs; duplicate keys permitted. Used for unnormalized input (e.g. HTTP headers).
- **Bind**: a first-class key/value pair `Bind(k, v)`. See §3.3.1.1.
- **Opt**: `Some(v)` or `None` — the option type.
- **Res**: `Ok(v)` or `Fail(code, msg)` — the result type.

**Map vs Prod (key distinction)**:
`Map` is *partial*: a key may be absent, and projecting an absent key returns `None` or `Fail`. `Prod` is
*total* over its label set: in a host with a declared shape, every field is always present. Implementations
MUST represent them as distinct value kinds (see §3.4–3.5).

### 1.2 Equality

`=` compares values for equality.

Requirements:
- Equality must be **reflexive**, **symmetric**, **transitive**.
- For `Set`, equality is extensional (same members).
- For `Bag`, equality is extensional with multiplicity.
- For `Seq`, equality is positional.
- For `Map`, equality is extensional (same bindings).

When a host cannot provide a meaningful equality for a value (e.g. opaque handle),
it must not allow that value in positions requiring equality (e.g. Set membership).

---

## 2. Variables, scopes, and programs

An SDA program is a sequence of statements.

- Variables are immutable by default; rebinding is allowed via `let`.
- Scope is lexical.

### 2.1 Grammar sketch (informal)

- Statement:
  - `let IDENT = Expr ;`
  - `Expr ;` (expression statement)
- Expression:
  - literals, identifiers
  - function calls `f(a, b)`

SDA is intentionally small; hosts may restrict or extend the statement layer.

### 2.2 Lambda expressions

SDA supports minimal lambda expressions for use with combinators.

- Syntax:
  - Unicode: `x ↦ Expr`
  - ASCII:  `x => Expr`
- Only single-parameter lambdas are supported in v1.
- Lambdas are pure expressions and follow lexical scope.
- The bound variable is scoped within the lambda body.
- Lambdas are primarily intended for use with combinators such as `mapRes`, `bindRes`, `mapOpt`, `bindOpt`.

---

## 3. Carriers (Seq / Set / Bag / Map / Prod)

SDA distinguishes carriers by algebra.

### 3.1 Seq

Ordered, may contain duplicates.

- Literal: `Seq[ a, b, c ]`

### 3.2 Set

Unordered, unique elements.

- Literal: `Set{ a, b, c }`
- Membership: `x ∈ S`  (ASCII: `x in S`)

### 3.3 Bag

Unordered, duplicates allowed.

- Literal: `Bag{ a, b, a }`

### 3.3.1 BagKV (bag of key/value pairs)

SDA also supports a keyed bag carrier used to model unnormalized inputs such as
HTTP headers and JSON with duplicate keys.

- Literal: `BagKV{ k -> v, k2 -> v2, k -> v3 }`  ;; duplicates allowed

### 3.3.1.1 Bind (first-class binding value)

SDA defines a first-class binding value:

- `Bind(k, v)` produces a binding from key `k` to value `v`.
- Surface sugar: `k -> v` desugars to `Bind(k, v)`.

Bindings are normal values: they can appear inside `Seq`, `Set`, or `Bag`.
`BagKV{ ... }` is a bag whose elements are `Bind(k,v)` values.

`BagKV` is distinct from `Map`: it permits duplicate keys until explicitly
normalized (see §7).

### 3.4 Map

**Mathematical model (Normative)**: `Map` is a finite partial function `f : K ⇀ Value`, where in standalone SDA
the key domain `K` is restricted to `Str`. Two `Map` values are equal (extensionally) iff they have the same
domain and agree on every key: `M₁ = M₂ ⟺ dom(M₁) = dom(M₂) ∧ ∀k ∈ dom(M₁). M₁(k) = M₂(k)`.

Properties:
- Keys are **unique**: each key appears at most once.
- There is **no defined field order** (Map equality is extensional).
- Missing keys are **absent** (not `Null`); see §4.

Rules:
- Literal: `Map{ "k" -> v, "k2" -> v2 }`
- In standalone SDA, Map literal keys MUST be `Str` literals.
- Bare identifiers are NOT allowed as Map keys.
- Hosts MAY extend the key domain (e.g. symbols) but MUST document the surface syntax.
- If a host extends keys beyond `Str`, `keys(map)` MUST return a `Set` of the extended key values.

### 3.5 Prod (records)

**Mathematical model (Normative)**: `Prod` is a finite **total** function `f : L → Value` where `L` is a fixed,
finite set of **labels** (field names). Two `Prod` values are equal iff they have the same label set and agree on
every label: `P₁ = P₂ ⟺ labels(P₁) = labels(P₂) ∧ ∀ℓ ∈ labels(P₁). P₁(ℓ) = P₂(ℓ)`.

**Prod vs Map (Normative)**:
- `Map` is a *partial* function — keys may be absent. Selector access on a missing key returns `None` / `Fail(t_sda_missing_key, …)`.
- `Prod` is a *total* function over a *known label set* — in a host that declares the shape, every field is
  guaranteed present. Total projection (no `?`/`!` suffix) is valid on `Prod` when the shape is known.
- In standalone SDA, `Prod` behaves like `Map` at runtime (total projection falls back to the same absent-key
  semantics unless a host declares shape). Hosts that model records with a statically-known shape SHOULD use
  `Prod`, not `Map`.
- Implementations SHOULD represent `Prod` and `Map` as distinct value kinds, even if their runtime access
  semantics are identical in standalone mode.

Rules:
- Literal (standalone form): `Prod{ name: "steve", age: 11 }`
- Field names follow identifier syntax (IDENT).
- Duplicate field names in a `Prod` literal are a static error.

---

## 4. Absence, optionality, and errors

SDA separates:

- **absence** (a key is not present)
- **null** (a value that is explicitly null)
- **failure** (an assertion violation)

### 4.0.1 Null vs absence (Normative)

`Null` is a **data value** (e.g. JSON `null`). It is not a synonym for absence.

- **Absence** means there is no binding for a key/field in a carrier.
- **Null** means a binding exists and its value is `Null`.
- Optional extraction (`?`) and required extraction (`!`) report absence based on bindings, not on whether the stored value is `Null`.

Example:

- If `m` is `Map{ "x" -> Null }`, then `m<"x">? = Some(Null)` and `m<"x">! = Ok(Null)`.
- If `m` is `Map{ }`, then `m<"x">? = None` and `m<"x">! = Fail(t_sda_missing_key, "missing key")`.

SDA provides two result wrappers:

- `Opt[T]`: `Some(v)` or `None`
- `Res[T]`: `Ok(v)` or `Fail(code, msg)`

Standalone syntax:

- `Some(v)`, `None`
- `Ok(v)`, `Fail(code, msg)`

Stable error codes are strings (or symbols if the host supports them).

---

## 5. Selectors

A **selector** identifies a field/key by label.

- Selector token: `name` (identifier) or `"name"` (string)
- A bare identifier selector (e.g. `email`) is shorthand for the string `"email"`.
- This shorthand applies only to selectors, not to Map literals.
- Multi-selector literal: `{a b c}` (compile-time only, see §5.2)

### 5.1 Selector addressing operator

SDA uses angle brackets to avoid dot access.

- Unicode: `x⟨k⟩` (ASCII: `x<k>`)

Angle brackets do not imply success; success depends on the eliminator suffix.

### 5.2 Static selectors

The multi-selector literal `{a b c}` is **static-only**.

- It is valid only where the grammar requires a static selector set.
- Attempting to store/pass it as a runtime value is an error:
  - `t_sda_selector_not_static`

Duplicates in a selector set are illegal:
- `{a a}` -> `t_sda_duplicate_label_in_selector`

---

## 6. The Three Eliminators (core)

SDA defines exactly three lawful eliminators. They differ only in tolerance.

### 6.1 Total eliminator (product projection)

Form:

- `x⟨k⟩` (no suffix)

Semantics:

- **Allowed only on Prod** (or host-declared schema-known product-like values).
- If `k` is not a field of `x`, this is a **compile-time** error when shape is known.
- If shape is not known at compile time, the host must either:
  - reject compiling SDA requiring total projection, or
  - treat it as runtime error `t_sda_wrong_shape`.

Type behavior:

- `Prod{name:T,...}⟨name⟩ : T`

Note: The total eliminator `x⟨k⟩` is legal only on Prod (schema-known). Optional (`?`) and required (`!`) eliminators are the only ones legal on Map or BagKV.

### 6.2 Optional eliminator (partial extraction)

Form:

- `x⟨k⟩?`

Semantics:

- On `Map`:
  - missing -> `None`
  - present -> `Some(v)`
- On `Bag` (key/value bags; see §6.4):
  - 0 matches -> `None`
  - 1 match -> `Some(v)`
  - >1 matches -> `None` (non-strict) or `Fail(t_sda_duplicate_key, ...)` (strict variant)

This spec defines the **non-strict** default for `?` on bags.

### 6.3 Required eliminator (assertive extraction)

Form:

- `x⟨k⟩!`

Semantics:

- On `Map`:
  - missing -> `Fail(t_sda_missing_key, "missing key")`
  - present -> `Ok(v)`
- On `Bag` (key/value bags):
  - 0 matches -> `Fail(t_sda_missing_key, "missing key")`
  - 1 match -> `Ok(v)`
  - >1 matches -> `Fail(t_sda_duplicate_key, "duplicate key")`

### 6.4 Key/value Bag vs element Bag

SDA supports two bag styles:

- `Bag[V]`: a bag of values
- `BagKV`: a bag of bindings `k -> v` (syntactic sugar for a bag of pairs `(k, v)`)

Eliminators `⟨k⟩?` and `⟨k⟩!` apply only to **BagKV**.

`BagKV` is a *carrier*; `Bind(k,v)` is a *value*.
If you have a `Bag` of `Bind` values, convert it explicitly via `asBagKV(...)` (see §11.1).

If applied to `Bag[V]`, fail with:
- `t_sda_wrong_shape`

---

### 6.5 Totality and failure table (Normative)

The following table is normative. It defines which operations are total and which may fail.

Operation                Input kind(s)                     Output        Total  Failure codes
----------------------  --------------------------------  ------------  -----  -----------------------------
x⟨k⟩                    Prod (schema-known)               value         Yes    (compile-time unknown field)
x⟨k⟩?                   Map, BagKV                        Opt[value]    Yes    t_sda_wrong_shape
x⟨k⟩!                   Map, BagKV                        Res[value]    No     t_sda_wrong_shape,
                                                                           t_sda_missing_key,
                                                                           t_sda_duplicate_key
normalizeUnique(bagkv)  BagKV                             Res[Map]      No     t_sda_wrong_shape,
                                                                           t_sda_duplicate_key
normalizeFirst(bagkv)   BagKV                             Map           Yes    t_sda_wrong_shape
normalizeLast(bagkv)    BagKV                             Map           Yes    t_sda_wrong_shape
asBagKV(bag)            Bag                              Res[BagKV]     No     t_sda_wrong_shape

Notes:
- “Total” means the operation does not produce `Fail(...)` in a conforming implementation.
- Compile-time shape errors (e.g. projecting a non-existent field on a schema-known `Prod`) are not represented as `Fail(...)` and are reported at compile time by the host toolchain.

## 7. Normalization (algebra changes)

Normalization changes a BagKV into a Map with an explicit policy.

### 7.1 normalizeUnique

`normalizeUnique(bagkv) : Res[Map]`

- If any key appears more than once -> `Fail(t_sda_duplicate_key, ...)`
- Otherwise -> `Ok(map)`

### 7.2 normalizeFirst / normalizeLast

`normalizeFirst(bagkv) : Map` (first wins)

`normalizeLast(bagkv) : Map` (last wins)

These are policy functions; they never fail.

### 7.3 Rationale

Normalization is not an optimization. It is a change of algebra:

- BagKV permits multiplicity.
- Map requires uniqueness.

SDA forbids silent conversion.

---

## 8. Set / Bag / Seq operators (closed algebra)

### 8.1 Set operators

For sets `A`, `B`:

- union: `A ∪ B`
- intersection: `A ∩ B`
- difference: `A \ B`

ASCII:
- `A union B`
- `A inter B`
- `A diff B`

### 8.2 Bag operators

Bags support:

- bag union (add multiplicities): `A ⊎ B`  (ASCII: `A bunion B`)
- bag difference (subtract multiplicities, floor at 0): `A ⊖ B` (ASCII: `A bdiff B`)

If the host does not support `⊎` / `⊖` glyphs, it must support the ASCII names.

### 8.3 Seq operators

Sequences support:

- concatenation: `A ++ B`

### 8.4 Membership

- `x ∈ Set{...}` is defined.
- `x ∈ Bag{...}` is defined (ignores multiplicity; presence test).
- For multiplicity, hosts may provide `count(x, bag)`.

### 8.5 Algebraic laws (Informative)

These laws are provided to guide implementations and conformance tests.

Set laws (for `∪` and `∩`):
- Commutative: `A ∪ B = B ∪ A`, `A ∩ B = B ∩ A`
- Associative: `(A ∪ B) ∪ C = A ∪ (B ∪ C)` and similarly for `∩`
- Idempotent: `A ∪ A = A`, `A ∩ A = A`

Bag laws (for `⊎`):
- Commutative: `A ⊎ B = B ⊎ A`
- Associative: `(A ⊎ B) ⊎ C = A ⊎ (B ⊎ C)`

Seq laws (for `++`):
- Associative: `(A ++ B) ++ C = A ++ (B ++ C)`
- Not commutative in general.

---

## 9. Comprehensions (the query core)

SDA provides a comprehension form to express filtering and projection.

### 9.1 Set comprehension

Unicode:

`{ a ∈ A ∣ P(a) }`

ASCII:

`{ a in A | P(a) }`

Semantics:

- v1 comprehensions support exactly one generator (`a ∈ A`).
- The bound variable is scoped only within the predicate and yield expression.
- Nested comprehensions are allowed.
- If `A` is a `Set`, result is a `Set`.
- If `A` is a `Seq`, result is a `Seq` (preserves order).
- If `A` is a `Bag`, result is a `Bag` (preserves multiplicity).

This is the "same comprehension, carrier-preserving" rule.

### 9.2 Projection in comprehensions

Allow a `yield` clause:

Unicode:

`{ yield E(a) ∣ a ∈ A ∧ P(a) }`

ASCII:

`{ yield E(a) | a in A and P(a) }`

If `yield` is absent, yield the bound variable.

### 9.3 Predicate language

Predicates use:

- booleans, comparisons (`=`, `≠`, `<`, `<=`, `>`, `>=`)
- logical operators `∧`, `∨`, `¬`
- membership `∈`

Example:

`{ a ∈ A ∣ a⟨name⟩ = "steve" ∧ a⟨city⟩ ∈ Set{"la","ny"} }`

### 9.4 BagKV comprehensions

When the source carrier `A` is a `BagKV`, the bound variable ranges over
bindings of the form `k -> v`.

Binding form:

- `b ∈ A` where `A : BagKV` binds `b` as a pair `(k, v)`.

Accessors:

- `b⟨key⟩` yields `k`
- `b⟨val⟩` yields `v`

Example:

```
{ b ∈ headers ∣ b⟨key⟩ = "content-type" }
```

Projection example:

```
{ yield b⟨val⟩ ∣ b ∈ headers ∧ b⟨key⟩ = "content-type" }
```

Carrier preservation:

- If `A` is `BagKV`, the result of the comprehension is a `Bag` of yielded values.

Important:

- If the comprehension yields bindings (via `yield k -> v` or `yield Bind(k,v)`), the result is still a `Bag` (of `Bind` values), not a `BagKV`.
- To treat that result as a keyed bag again, the program must convert explicitly:
  - `asBagKV(bagOfBindings) -> Res[BagKV]`

Rationale:

A comprehension is a selection/projection. Producing a keyed carrier (`BagKV`) would otherwise require implicit synthesis and validation of keys.
SDA forbids that implicit algebra change.

---

## 10. Pipe operator (composition)

SDA provides a pipe operator for left-to-right composition.

### 10.1 Syntax

- Unicode / ASCII: `E |> R`

### 10.2 Semantics

`E |> R` evaluates `E`, then evaluates `R` with the result of `E` bound to a
special placeholder.

- Unicode placeholder: `•` (U+2022 BULLET)
- ASCII placeholder: `_` (reserved token; not an identifier)

These placeholders are equivalent; hosts may accept either surface form.

Formally:

- Evaluate `E` to value `v`.
- Evaluate `R` in an environment where `• = v` (Unicode surface) or `_ = v` (ASCII surface).
- The value of the expression is the result of evaluating `R`.

The placeholder is read-only, cannot be declared or assigned, and is scoped only
to the right-hand side of the nearest enclosing `|>`.

### 10.3 Associativity and precedence

- `|>` is **left-associative**.
- `a |> b |> c` parses as `(a |> b) |> c`.
- `|>` has lower precedence than arithmetic, comparison, and logical operators,
  but higher precedence than statement termination (`;`).

### 10.3.1 Desugaring law (Informative)

`E |> R` is equivalent to evaluating `E` once and substituting its value into the placeholder in `R`:

- Desugaring: `E |> R  ≡  (let tmp = E; R[tmp/•])`

This is a law of meaning, not a required surface syntax feature.

### 10.4 Intended use

The pipe operator is the primary mechanism for multi-step data transformation in
raw SDA.

Examples:

```
input
|> normalizeUnique()
|> •<"content-type">!
```

```
A
|> { a ∈ • ∣ a<city> = "la" }
```

### 10.5 No implicit effect handling

The pipe operator is purely syntactic composition.

- It does **not** implicitly unwrap `Opt` or `Res` (including values referenced via the placeholder).
- Failure propagation must be explicit via library functions or eliminators.

This restriction is intentional and preserves algebraic clarity.

### 10.6 Placeholder binding (• / _)

The pipe placeholder has two equivalent spellings:

- Unicode: `•` (U+2022 BULLET)
- ASCII: `_`

Rules:

- The placeholder refers to the value produced by the **left-hand side** of the nearest enclosing `|>`.
- The placeholder is **read-only** and **not a normal identifier**:
  - it cannot be declared (`let _ = ...` is illegal)
  - it cannot be assigned
  - it cannot be shadowed by user variables
- The placeholder is in scope **only** on the right-hand side of its `|>`.
- Using `•` / `_` when no enclosing `|>` provides a binding MUST fail with:
  - `Fail(t_sda_unbound_placeholder, "unbound placeholder")`  ;; stable code + stable msg
- `•` / `_` are reserved tokens.
- They may not be used as identifiers, lambda parameters, or Map keys.

Examples:

```
A |> { a ∈ • ∣ a<city> = "la" }
```

```
_  ;; error: unbound placeholder
```

---

## 11. Functions (minimal standard library)

Standalone SDA defines a tiny required library.

### 11.1 Required functions

- `typeOf(x) -> Str`
- `keys(map) -> Set`
- `values(map) -> Seq`
- `count(x, bag) -> Num`
- `normalizeUnique(bagkv) -> Res[Map]`
- `normalizeFirst(bagkv) -> Map`
- `normalizeLast(bagkv) -> Map`
- `Bind(k, v) -> Bind`                ;; constructs a binding value
- `asBagKV(bag) -> Res[BagKV]`        ;; explicit Bag-of-Bind -> BagKV conversion
- `mapOpt(opt, f) -> Opt`             ;; Option functor map
- `bindOpt(opt, f) -> Opt`            ;; Option monadic bind (Some -> f(v), None -> None)
- `orElseOpt(opt, other) -> Opt`      ;; Option fallback
- `mapRes(res, f) -> Res`             ;; Result functor map (Ok -> Ok(f(v)))
- `bindRes(res, f) -> Res`            ;; Result monadic bind / andThen (Ok -> f(v), Fail -> Fail)
- `orElseRes(res, other) -> Res`      ;; Result fallback

### 11.2 Semantics of combinators

These combinators are pure (no IO) and deterministic.

- `mapOpt(None, f) = None`
- `mapOpt(Some(v), f) = Some(f(v))`

- `bindOpt(None, f) = None`
- `bindOpt(Some(v), f) = f(v)`

- `orElseOpt(None, other) = other`
- `orElseOpt(Some(v), other) = Some(v)`

- `mapRes(Fail(code,msg), f) = Fail(code,msg)`
- `mapRes(Ok(v), f) = Ok(f(v))`

- `bindRes(Fail(code,msg), f) = Fail(code,msg)`
- `bindRes(Ok(v), f) = f(v)`

- `orElseRes(Fail(_, _), other) = other`
- `orElseRes(Ok(v), other) = Ok(v)`

`asBagKV(bag)` semantics:

- If `bag` is not a `Bag`, return `Fail(t_sda_wrong_shape, "wrong shape")`.
- If any element is not a `Bind(k,v)`, return `Fail(t_sda_wrong_shape, "wrong shape")`.
- In standalone SDA, if any binding key `k` is not a `Str`, return `Fail(t_sda_wrong_shape, "wrong shape")`.
- Otherwise return `Ok(BagKV{ ... })` containing the same bindings in unspecified order (bag semantics).

Hosts may add more.

---

## 12. Error codes (stable) and diagnostics

### 12.1 Runtime Fail codes (Normative)

The following error codes are stable. Conforming implementations MUST use these exact strings in `Fail(code, msg)`
values wherever the corresponding condition is detected.

| Code | Triggered by | Stable `msg` |
|---|---|---|
| `t_sda_wrong_shape` | Value has the wrong carrier kind for the operation | `"wrong shape"` |
| `t_sda_missing_key` | Required key/field not present | `"missing key"` |
| `t_sda_duplicate_key` | Key appears more than once where uniqueness is required | `"duplicate key"` |
| `t_sda_selector_not_static` | Selector expression is not a literal at compile time | `"selector not static"` |
| `t_sda_duplicate_label_in_selector` | Two labels in a multi-selector are identical | `"duplicate label in selector"` |
| `t_sda_unknown_field` | Field name not in the declared shape (compile-time) | `"unknown field"` |
| `t_sda_unbound_placeholder` | `_` / `•` used outside any `\|>` context | `"unbound placeholder"` |

`Fail(code, msg)` requirements:
- `code`: MUST be one of the stable tags above (a Str).
- `msg`: MUST be the stable English short message in the table above (a Str). Implementations MAY append
  additional detail after a `: ` separator (e.g. `"missing key: email"`), but the prefix MUST match exactly.

### 12.2 Diagnostic quality for symbol resolution (Normative)

When a program references a name that cannot be resolved, the implementation MUST distinguish:

- **Unbound identifier**: a name used in an expression that was never declared.
  - Error kind: runtime eval error (not a `Fail` value).
  - Diagnostic MUST include the name: e.g. `unbound variable 'foo'`.
- **Unbound placeholder**: `_` or `•` used outside any `|>`.
  - Result: `Fail(t_sda_unbound_placeholder, "unbound placeholder")` (a value, not a runtime error).

Implementations that compile to a grammar pack (seed/pack toolchain) MUST additionally:
- Name the unresolved symbol in the diagnostic: e.g. `unresolved symbol 'Selector' in rule 'BindEntry'`.
- Not substitute a generic "failed to load pack" message for an unresolved-symbol error.

### 12.3 Diagnostic quality for token collisions (Normative)

A conforming implementation (or its compiler) MUST emit a diagnostic when a keyword-like token is shadowed by an
identifier due to a priority ordering issue. Specifically:

- If a token definition compiles to a lower-priority matcher (e.g. regex, priority 100) than string-literal
  matchers (priority 200), and the token's string value matches a reserved keyword or a carrier-constructor name,
  the compiler MUST emit a warning of the form:
  `warning: token 'DiffKW' compiles to regex priority; may be shadowed by Identifier — split into separate string tokens`.
- Implementations that use a non-priority lexer (e.g. explicit keyword check before identifier dispatch) are
  exempt from this requirement because the collision cannot occur.

---

## 13. Worked examples

### 13.1 JSON-ish filter (carrier-preserving)

Input `A` is a sequence of records (Prod values).

```
{ a ∈ A ∣ a⟨name⟩ = "steve" ∧ a⟨city⟩ ∈ Set{"la","ny"} }
```

### 13.2 Map access (optional vs required)

```
let emailOpt = m<email>?;   ;; Unicode: m⟨email⟩?
let emailReq = m<email>!;   ;; returns Res
```

### 13.3 BagKV normalization

```
let mRes = normalizeUnique(headers);
let m = bindRes(mRes, m => m);   ;; explicit unwrapping
let ctRes = m<"content-type">!;
let ct = bindRes(ctRes, ct => ct);
```

### 13.4 BagKV duplicate behavior

For `b = BagKV{ "k" -> 1, "k" -> 2 }`:

- `b<k>?` -> `None`
- `b<k>!` -> `Fail(t_sda_duplicate_key, "duplicate key")`

### 13.5 Building BagKV via comprehension + explicit conversion

```
let pairs = { yield "x" -> 1 ∣ a ∈ Seq[1,2,3] };
let headersRes = asBagKV(pairs);
let headers = bindRes(headersRes, h => h);   ;; explicit Bag -> BagKV with explicit unwrapping
```

### 13.6 Result/Option combinators in a pipeline

```
input
|> normalizeUnique()                 ;; Res[Map]
|> bindRes(_, m => m<"content-type">!)
```

```
let city = mapOpt(user<city>?, c => c);
```

---

## 14. Conformance requirements

A conforming SDA implementation MUST:

1. Implement the three eliminators with the exact semantics in §6.
2. Implement normalization semantics in §7.
3. Preserve carrier in comprehensions (§9.1).
4. Emit stable error codes (§12).
5. Provide both Unicode and ASCII spellings for operators listed in §0.
6. Implement pipe semantics (§10) with correct scoping of `•` / `_` and left-associativity.
7. Implement BagKV comprehension binding and projection semantics (§9.4).

### 14.1 Minimal conformance suite outline (Informative)

A minimal test suite SHOULD include canonical cases for:

- Placeholder scoping:
  - `_` / `•` outside of any `|>` must raise `t_sda_unbound_placeholder`.
  - Nested pipes bind `_` / `•` to the nearest enclosing `|>`.
- BagKV duplicate behavior:
  - `BagKV{ "k" -> 1, "k" -> 2 }<"k">? = None`
  - `BagKV{ "k" -> 1, "k" -> 2 }<"k">! = Fail(t_sda_duplicate_key, "duplicate key")`
- Normalization:
  - `normalizeUnique` fails on duplicates and succeeds otherwise.
  - `normalizeFirst` / `normalizeLast` are total and deterministic.
- Carrier preservation in comprehensions:
  - Source `Seq` yields `Seq`, source `Set` yields `Set`, source `Bag` yields `Bag`.
- Null vs absence:
  - `Map{ "x" -> Null }<"x">? = Some(Null)` and `Map{ }<"x">? = None`.

---


## 15. Non-goals (v1)

- Full static typing (hosts may add it)
- IO, networking, database bindings
- Implicit schema inference
- Dot-access syntax
- Pipeline stages as a core SDA construct (embedding delimiters are host-defined).

---

# Appendix A — Minimal grammar (Informative)

This appendix is a compact, reader-friendly grammar sketch so the surface forms can be parsed
without hunting through the spec. It is informative; the normative semantics are in the main
sections.

Notes:
- Terminals in quotes are ASCII spellings; Unicode spellings are listed alongside where relevant.
- Whitespace is generally insignificant except to separate tokens.

## A.1 Lexical tokens (sketch)

- `IDENT`  ::= `[A-Za-z_][A-Za-z0-9_]*` except reserved keywords/tokens
- `STRING` ::= `" ... "` with escapes `\"`, `\\`, `\n`, `\t`
- `NUM`    ::= host-defined numeric literal surface (implementations SHOULD support decimal)

Reserved tokens (not identifiers):
- Operators/keywords: `in and or not != -> => union inter diff bunion bdiff` and `yield`
- Pipe placeholder: `_` and (Unicode) `•`

## A.2 Program and statements

```
Program   ::= { Stmt }
Stmt      ::= LetStmt | ExprStmt
LetStmt   ::= "let" IDENT "=" Expr ";"
ExprStmt  ::= Expr ";"
```

## A.3 Expressions (core)

```
Expr      ::= Pipe
Pipe      ::= Or { "|>" Or }
Or        ::= And { ("or" | "∨") And }
And       ::= Not { ("and" | "∧") Not }
Not       ::= { ("not" | "¬") } Cmp
Cmp       ::= Add { ("=" | "!=" | "≠" | "<" | "<=" | "≤" | ">" | ">=" | "≥") Add }
Add       ::= Mul { ("+" | "-") Mul }
Mul       ::= Unary { ("*" | "/") Unary }
Unary     ::= { ("-" ) } Postfix

Postfix   ::= Primary { SelectorAccess }
SelectorAccess ::= "<" Selector ">" [ "?" | "!" ]
              |  "⟨" Selector "⟩" [ "?" | "!" ]

Primary   ::= Literal
          |  IDENT
          |  Placeholder
          |  Call
          |  Lambda
          |  "(" Expr ")"

Placeholder ::= "_" | "•"

Call      ::= IDENT "(" [ Args ] ")"
Args      ::= Expr { "," Expr }

Lambda    ::= IDENT ("=>" | "↦") Expr
```

## A.4 Selectors

```
Selector  ::= IDENT | STRING
```

Selector shorthand:
- A bare `IDENT` selector is treated as the string label of the same text (e.g. `email` ≡ `"email"`) **only** in selector positions.
- This shorthand does **not** apply to Map literal keys (see §3.4).

## A.5 Literals and carriers

```
Literal   ::= Null | Bool | NUM | STRING
          |  SeqLit | SetLit | BagLit | MapLit | ProdLit | BagKVLit

Null      ::= "Null"
Bool      ::= "true" | "false"

SeqLit    ::= "Seq" "[" [ ExprList ] "]"
SetLit    ::= "Set" "{" [ ExprList ] "}"
BagLit    ::= "Bag" "{" [ ExprList ] "}"

MapLit    ::= "Map" "{" [ MapEntryList ] "}"
MapEntryList ::= MapEntry { "," MapEntry }
MapEntry  ::= STRING ("->" | "→") Expr

ProdLit   ::= "Prod" "{" [ ProdFieldList ] "}"
ProdFieldList ::= ProdField { "," ProdField }
ProdField ::= IDENT ":" Expr

BagKVLit  ::= "BagKV" "{" [ BindList ] "}"
BindList  ::= BindEntry { "," BindEntry }
BindEntry ::= (Selector | STRING) ("->" | "→") Expr

ExprList  ::= Expr { "," Expr }
```

## A.6 Comprehensions

Set/bag/seq comprehensions share one surface form; the output carrier preserves the input carrier (§9.1).

```
Compr     ::= "{" ComprBody "}"
ComprBody ::= [ "yield" Expr "|" ] IDENT ("in" | "∈") Expr [ ("|" | "∣") Pred ]
Pred      ::= Expr
```

Examples:
- `{ a in A | a<name> = "steve" }`
- `{ yield a<city> | a ∈ A ∧ a<name> = "steve" }`

```

---

# Appendix B — Embedding examples (Informative)

This appendix is **informative**. It shows how a host language may embed SDA.
Nothing in this appendix changes SDA semantics.

## B.1 Oberon-7z style embedding examples

Assumptions:

- The host language has a string type `STRING` and a tree value type `Tree`.
- The host exposes an SDA evaluator:

  - `SDA.Eval(program: STRING; input: Tree): SDA.Result`
  - where `SDA.Result` can represent either a value or `Fail(code,msg)`.

- The host chooses its own delimiter syntax for embedded SDA. Examples below use `{{ ... }}` as an embedding delimiter.

### B.1.1 Filter a sequence of records

Input: `A` is a `Seq` of `Prod` values.

SDA:

```
{ a ∈ A ∣ a⟨name⟩ = "steve" ∧ a⟨city⟩ ∈ Set{"la","ny"} }
```

Host (Oberon-7z sketch):

```
MODULE Example;

IMPORT SDA;

VAR
  input: Tree;
  out: SDA.Result;
  program: STRING;

BEGIN
  program := "{ a ∈ A ∣ a<name> = \"steve\" and a<city> in Set{\"la\",\"ny\"} }";
  (* The host decides how to bind top-level names like A.
     One common approach: provide `input` as the implicit root, and set A := input. *)
  out := SDA.Eval(program, input);
  (* Host inspects `out` and converts to its preferred representation. *)
END Example.
```

### B.1.2 Normalize untrusted key/value bags

Input: `headers` is a `BagKV` (duplicate keys allowed).

SDA:

```
headers
|> normalizeUnique()
|> bindRes(_, m => m<"content-type">!)
```

Host (Oberon-7z sketch):

```
MODULE Headers;

IMPORT SDA;

VAR
  headers: Tree;
  out: SDA.Result;
  program: STRING;

BEGIN
  program := "headers |> normalizeUnique() |> bindRes(_, m => m<\"content-type\">!)";
  out := SDA.Eval(program, headers);
END Headers.
```

### B.1.3 Optional vs required extraction

SDA:

```
let emailOpt = user<email>?;
let emailReq = user<email>!;
```

Host (Oberon-7z sketch):

```
MODULE OptReq;

IMPORT SDA;

VAR
  user: Tree;
  out: SDA.Result;
  program: STRING;

BEGIN
  program := "let emailOpt = user<email>?; let emailReq = user<email>!; emailReq;";
  out := SDA.Eval(program, user);
END OptReq.
```

Notes:

- The embedding delimiter (e.g. `{{ ... }}`) is intentionally **not** part of SDA.
- Name binding (e.g. what `A`, `user`, `headers` refer to) is host-defined.
- Error handling is host-defined; SDA guarantees stable error codes/messages at the language boundary.
```


