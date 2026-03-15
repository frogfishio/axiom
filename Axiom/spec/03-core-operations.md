# 03. Core Operations (Normative)

## 03.1 Selectors
- Selector token: `name` (identifier) or `"name"` (string).
- In selector positions only, bare identifier `email` is shorthand for string `"email"`.
- This shorthand does NOT apply to Map literal keys.

## 03.2 Eliminators

### Total projection (Prod only): x<k>
- Allowed only on schema-known `Prod`.
- Unknown field on known shape is a compile-time error.
- If host allows unknown shape at runtime, MUST fail with Fail(t_sda_wrong_shape,"wrong shape").

### Optional: x<k>?
- Map: missing -> None; present -> Some(v)
- BagKV: 0 -> None; 1 -> Some(v); >1 -> None (non-strict default)
- Otherwise: Fail(t_sda_wrong_shape,"wrong shape")

### Required: x<k>!
- Map: missing -> Fail(t_sda_missing_key,"missing key"); present -> Ok(v)
- BagKV: 0 -> Fail(t_sda_missing_key,"missing key"); 1 -> Ok(v); >1 -> Fail(t_sda_duplicate_key,"duplicate key")
- Otherwise: Fail(t_sda_wrong_shape,"wrong shape")

## 03.3 Normalization
- normalizeUnique(BagKV) : Res[Map] (fails on duplicates)
- normalizeFirst/normalizeLast : Map (total policies)
- asBagKV(Bag) : Res[BagKV] (fails on wrong shape)

## 03.4 Comprehensions
- One generator in v0.1; nested comprehensions allowed.
- Carrier preservation:
  - Seq -> Seq (order preserved)
  - Set -> Set
  - Bag -> Bag (multiplicity preserved)
  - BagKV source binds elements as Bind; result is Bag.

## 03.5 Pipe
- E |> R binds placeholder _ / • to E on RHS.
- Placeholder is read-only.
- Unbound placeholder -> Fail(t_sda_unbound_placeholder,"unbound placeholder").
- No implicit Opt/Res unwrapping.

## 03.6 Required functions
typeOf, keys, values, count
Bind, asBagKV
mapOpt/bindOpt/orElseOpt
mapRes/bindRes/orElseRes
