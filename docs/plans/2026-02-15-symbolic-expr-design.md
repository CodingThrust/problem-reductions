# Symbolic Expression System for Reduction Overhead

**Date:** 2026-02-15
**Status:** Approved

## Goal

Replace the current `Polynomial`/`Monomial`/`poly!` system with a general-purpose symbolic expression DSL that supports exponentials, logarithms, min/max, floor/ceil, and arbitrary arithmetic — not just polynomials.

## Motivation

The current `Polynomial` type only supports sums of monomials (coefficient × product of variables with integer exponents). It cannot represent:
- Exponential overhead: `1.44 ^ num_vertices`
- Logarithmic factors: `num_vertices * log2(num_edges)`
- Min/max: `max(num_vertices, num_edges)`
- Floor/ceil: `ceil(num_vertices / 2)`

These are needed to accurately model reduction overhead between problems.

## Design

### 1. Expression AST

```rust
#[derive(Clone, Debug)]
pub enum Expr {
    Num(f64),
    Var(Box<str>),
    BinOp { op: BinOp, lhs: Box<Expr>, rhs: Box<Expr> },
    Neg(Box<Expr>),
    Call { func: Func, args: Vec<Expr> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp { Add, Sub, Mul, Div, Pow }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Func { Log2, Log10, Ln, Exp, Sqrt, Min, Max, Floor, Ceil, Abs }
```

**Key decisions:**
- `Box<str>` for variable names (avoids `String` allocation overhead on clone)
- No `PartialEq` on `Expr` (f64 makes it dangerous; NaN != NaN)
- `Func` enum for built-ins (compile-time exhaustiveness, no runtime string matching)
- Separate `Neg` variant (cleaner than encoding as `0 - x`)
- No user-defined functions (not needed for this use case)

### 2. Evaluation

```rust
pub enum EvalError {
    UnknownVar(Box<str>),
    DivideByZero,
    Arity { func: Func, expected: usize, got: usize },
    Domain { func: Func, detail: Box<str> },
}
```

- Recursive tree walk with `ProblemSize` providing variable bindings
- Returns `Result<f64, EvalError>`
- **No NaN guarantee:** domain violations return `EvalError::Domain` instead of producing NaN
  - `log(-1)` → Domain error
  - Negative base + non-integer exponent → Domain error
  - `0 / 0` → DivideByZero
- `^` is right-associative: `a ^ b ^ c` = `a ^ (b ^ c)`

### 3. Parser

Pratt parser (precedence climbing), ~200-300 lines.

**Tokens:** `Num(f64)`, `Ident(Box<str>)`, `+`, `-`, `*`, `/`, `^`, `(`, `)`, `,`, `Eof`

**Precedence (lowest to highest):**

| Level | Operators | Associativity |
|-------|-----------|---------------|
| 1     | `+`, `-`  | Left          |
| 2     | `*`, `/`  | Left          |
| 3     | unary `-` | Prefix        |
| 4     | `^`       | Right         |

Function calls parsed as part of primary expressions.

**Grammar (for documentation; implementation uses Pratt):**
```
expr     = term (('+' | '-') term)*
term     = unary (('*' | '/') unary)*
unary    = '-' unary | power
power    = primary ('^' power)?
primary  = NUM | IDENT '(' args ')' | IDENT | '(' expr ')'
args     = expr (',' expr)*
```

**Ident resolution:** If followed by `(`, matched against `Func` variants (ASCII case-insensitive). Unknown function → `ParseError::UnknownFunction`.

**No implicit multiplication:** `2 num_vertices` is a parse error; must write `2 * num_vertices`.

**Tokenizer rules:**
- Numbers: `42`, `1.5`, `.5` — no scientific notation
- Idents: `[a-zA-Z_][a-zA-Z0-9_]*`
- Whitespace: skipped silently

**Parse errors with spans:**
```rust
pub struct Span { pub start: usize, pub end: usize }

pub enum ParseError {
    UnexpectedToken { expected: &'static str, got: Box<str>, span: Span },
    UnexpectedEof { expected: &'static str },
    UnknownFunction { name: Box<str>, span: Span },
    InvalidNumber { lexeme: Box<str>, span: Span },
}
```

### 4. Display and Serialization

**Display** uses minimal parenthesization:
- Parenthesize child when its precedence < parent precedence
- For right-associative `^`: parenthesize left child if same precedence
- Integer-valued floats: `3.0` → `"3"`, `1.5` → `"1.5"`
- Round-trip invariant: `parse(expr.to_string())` ≡ `expr` semantically

**Serde:** `Expr` serializes as its display string, deserializes by parsing. This means JSON contains human-readable expressions like `"1.44 ^ num_vertices"`.

### 5. Integration with ReductionOverhead

**`ReductionOverhead` stores `Expr`:**
```rust
pub struct ReductionOverhead {
    pub output_size: Vec<(&'static str, Expr)>,
}
```

**Constructor takes string pairs:**
```rust
impl ReductionOverhead {
    pub fn new(specs: Vec<(&'static str, &'static str)>) -> Self {
        // Parses each expression string immediately.
        // Panics on parse error (developer bug — these are static literals).
    }
}
```

**`evaluate_output_size` returns `Result`:**
```rust
pub fn evaluate_output_size(&self, input: &ProblemSize) -> Result<ProblemSize, EvalError>
```

Float → usize conversion: `round()` (matching current behavior), error on non-finite/negative/overflow.

**Reduction macro usage changes from:**
```rust
#[reduction(overhead = {
    ReductionOverhead::new(vec![
        ("num_vertices", poly!(num_vertices)),
        ("num_edges", poly!(num_edges)),
    ])
})]
```

**To:**
```rust
#[reduction(overhead = {
    ReductionOverhead::new(vec![
        ("num_vertices", "num_vertices"),
        ("num_edges", "num_edges"),
    ])
})]
```

The `#[reduction]` proc macro itself needs no changes (it passes through the token stream).

**JSON export format:**
```json
[
  {"field": "num_vertices", "expression": "num_vertices ^ 2"},
  {"field": "num_edges", "expression": "1.44 ^ num_vertices"}
]
```

### 6. File Organization

| Action  | File | Description |
|---------|------|-------------|
| **New** | `src/expr.rs` | `Expr`, `BinOp`, `Func`, parser, evaluator, Display, Serde (~400-500 lines) |
| **New** | `src/unit_tests/expr.rs` | Parser, eval, display round-trip, error case tests |
| **Delete** | `src/polynomial.rs` | Replaced by `src/expr.rs` |
| **Delete** | `src/unit_tests/polynomial.rs` | Replaced by `src/unit_tests/expr.rs` |
| **Modify** | `src/lib.rs` | `mod polynomial` → `mod expr`, update re-exports |
| **Modify** | `src/rules/registry.rs` | `Polynomial` → `Expr`, `new()` takes `(&str, &str)` pairs |
| **Modify** | `src/export.rs` | Remove `MonomialJson`, use expression strings |
| **Modify** | `src/rules/cost.rs` | Propagate `Result` from `evaluate_output_size` |
| **Modify** | `src/rules/graph.rs` | Propagate `Result` from `evaluate_output_size` |
| **Modify** | `src/rules/*.rs` (all reductions) | `poly!(x)` → `"x"` string literals |
