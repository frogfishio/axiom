# 04. Enrichment (Normative)

## 04.1 Sources
- Index[K,V] unique keys
- MultiIndex[K,V] duplicate keys

Standalone v0.1: K MUST be Str.

## 04.2 Lookup
Index:
- lookup -> Opt
- lookup! -> Res (t_src_missing)

MultiIndex:
- lookupAll -> Bag (total)
- lookupUnique -> Res (t_src_missing / t_src_duplicate)
- lookupFirst/lookupLast -> Opt (total policies)

## 04.3 Joins
joinIndex(A,index,leftKey,project):
- A in {Seq, Bag, Set}
- non-strict default: missing drops rows
- carrier preservation: Seq->Seq, Bag->Bag, Set->Set

joinMulti(A,multi,leftKey,project):
- expands by multiplicity
- carrier: Seq->Seq, Bag->Bag, Set->Bag (normative)

## 04.4 groupBy + agg
groupBy(A,keyFn) -> Map[K, subcarrier]
agg(map, spec) -> Map[K,R]
Minimum aggregates: count, distinct.

## 04.5 Explain (recommended)
explain(program) should show sources+versions and join policies.
