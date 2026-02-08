---
paths:
  - "docs/paper/**/*.typ"
---

# Documentation Requirements

The technical paper (`docs/paper/reductions.typ`) must include:

1. **Table of Contents** - Auto-generated outline of all sections
2. **Problem Data Structures** - Rust struct with fields in a code block
3. **Reduction Examples** - Minimal working example showing reduce → solve → extract

## Pattern

```typst
#definition("Problem Name")[
  Mathematical definition...
]

// Rust data structure
```rust
pub struct ProblemName<W = i32> {
    field1: Type1,
    field2: Type2,
}
`` `

#theorem[
  *(Source → Target)* Reduction description...
]

// Minimal working example from closed-loop tests
```rust
let source = SourceProblem::new(...);
let reduction = ReduceTo::<TargetProblem>::reduce_to(&source);
let target = reduction.target_problem();
// ... solve and extract
`` `
```
