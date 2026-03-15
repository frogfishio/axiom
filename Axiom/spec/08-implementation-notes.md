# 08. Implementation Notes (Informative)

Axiom is designed for:
AST → MIR → SIR → LLVM/WASM/JVM/CLR.

Lower most operations to runtime calls and keep semantics stable.
Prefer portable bytecode artifacts for embedding, and compile PIC layouts to plans.
