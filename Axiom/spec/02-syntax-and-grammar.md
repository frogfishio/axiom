# 02. Syntax and Grammar (Normative with Informative Appendix)

Axiom uses SDA-compatible surface forms and extends them with enrichment and PIC blocks.

## 02.1 Comments and lexing
- Line comments begin with `;;` and continue to end of line.
- Whitespace is insignificant except to separate tokens.
- String literals MUST support escapes: `\"`, `\\`, `\n`, `\t`.

## 02.2 Unicode and ASCII operators

Unicode operators MAY be used. Hosts MUST accept the ASCII equivalents listed.

Synonyms (MUST parse to same AST):
- `→` and `->` (binding / map entry sugar)
- `↦` and `=>` (lambda)

Reserved ASCII operator spellings (not identifiers):
- `->`, `=>`
- `union`, `inter`, `diff`
- `bunion`, `bdiff`
- `yield`
- `_` (pipe placeholder)

## 02.3 Core surface forms

- Let: `let x = Expr;`
- Lambda (single param): `x => Expr`
- Function call: `f(a, b)`
- Pipe: `E |> R` (placeholder `_` or Unicode `•` bound on RHS)
- Selector/eliminator: `x<k>`, `x<k>?`, `x<k>!`
- Comprehension: `{ yield E | a in A and P(a) }` (carrier-preserving)
- Literals:
  - `Seq[ ... ]`, `Set{ ... }`, `Bag{ ... }`, `BagKV{ k -> v, ... }`
  - `Map{ "k" -> v, ... }` (keys MUST be Str literals)
  - `Prod{ name: v, ... }`

## 02.4 PIC blocks and overlays

PIC codec blocks are introduced via:
- `pic{ ... }` producing a `Codec[T]` value

Overlay syntax:
- `overlay NAME size N choose by FIELD { TAG => ARM, ... }`
- Each ARM is a codec that MUST have fixed size N.

OCCURS syntax:
- `occurs(N, CODEC)`
- `occurs(FIELD, CODEC)` where FIELD refers to a prior decoded field value

Variable-size OCCURS MUST use explicit framing:
- `occurs(N, lenPrefix(U16BE, CODEC))`
- `occurs(N, until(STOP, CODEC))`
- `occurs(N, stride(BYTES, CODEC))`

## Appendix A — Minimal grammar sketch (Informative)

Program   ::= { Stmt }
Stmt      ::= "let" IDENT "=" Expr ";"
          |  Expr ";"

Expr      ::= Pipe
Pipe      ::= Or { "|>" Or }
Or        ::= And { ("or" | "∨") And }
And       ::= Not { ("and" | "∧") Not }
Not       ::= { ("not" | "¬") } Cmp
Cmp       ::= Add { ("=" | "!=" | "≠" | "<" | "<=" | "≤" | ">" | ">=" | "≥") Add }
Add       ::= Mul { ("+" | "-") Mul }
Mul       ::= Unary { ("*" | "/") Unary }
Unary     ::= { "-" } Postfix
Postfix   ::= Primary { SelectorAccess }
SelectorAccess ::= "<" Selector ">" [ "?" | "!" ]
Primary   ::= Literal | IDENT | Placeholder | Call | Lambda | "(" Expr ")"

Placeholder ::= "_" | "•"
Call      ::= IDENT "(" [ Args ] ")"
Args      ::= Expr { "," Expr }
Lambda    ::= IDENT ("=>" | "↦") Expr

Selector  ::= IDENT | STRING

Literal   ::= "Null" | "true" | "false" | NUM | STRING
          |  "Seq" "[" [ ExprList ] "]"
          |  "Set" "{" [ ExprList ] "}"
          |  "Bag" "{" [ ExprList ] "}"
          |  "BagKV" "{" [ BindList ] "}"
          |  "Map" "{" [ MapEntryList ] "}"
          |  "Prod" "{" [ ProdFieldList ] "}"

ExprList  ::= Expr { "," Expr }
BindList  ::= BindEntry { "," BindEntry }
BindEntry ::= Selector ("->" | "→") Expr
MapEntryList ::= MapEntry { "," MapEntry }
MapEntry  ::= STRING ("->" | "→") Expr
ProdFieldList ::= ProdField { "," ProdField }
ProdField ::= IDENT ":" Expr
