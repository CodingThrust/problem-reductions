# Overhead System Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the `Polynomial`-based overhead system with a general `Expr` AST, compile-time macro-parsed expression strings, and per-problem inherent getters.

**Architecture:** The `#[reduction]` proc macro parses expression strings at compile time and emits both compiled Rust getter-calling code (for evaluation + compiler validation) and symbolic `Expr` AST literals (for composition + export). Problems provide inherent getter methods instead of trait-level `problem_size_names()`/`problem_size_values()`.

**Tech Stack:** Rust proc macros (syn/quote), Pratt parser, serde, inventory

---

## Phase 1: Add `Expr` type (additive, no breaking changes)

### Task 1: Create `Expr` enum and basic operations

**Files:**
- Create: `src/expr.rs`
- Create: `src/unit_tests/expr.rs`
- Modify: `src/lib.rs` (add module)

**Step 1: Write failing tests for Expr construction and evaluation**

Create `src/unit_tests/expr.rs`:
```rust
use super::*;
use crate::types::ProblemSize;

#[test]
fn test_expr_const_eval() {
    let e = Expr::Const(42.0);
    let size = ProblemSize::new(vec![]);
    assert_eq!(e.eval(&size), 42.0);
}

#[test]
fn test_expr_var_eval() {
    let e = Expr::Var("n");
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(e.eval(&size), 10.0);
}

#[test]
fn test_expr_add_eval() {
    // n + 3
    let e = Expr::add(Expr::Var("n"), Expr::Const(3.0));
    let size = ProblemSize::new(vec![("n", 7)]);
    assert_eq!(e.eval(&size), 10.0);
}

#[test]
fn test_expr_mul_eval() {
    // 3 * n
    let e = Expr::mul(Expr::Const(3.0), Expr::Var("n"));
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(e.eval(&size), 15.0);
}

#[test]
fn test_expr_pow_eval() {
    // n^2
    let e = Expr::pow(Expr::Var("n"), Expr::Const(2.0));
    let size = ProblemSize::new(vec![("n", 4)]);
    assert_eq!(e.eval(&size), 16.0);
}

#[test]
fn test_expr_exp_eval() {
    let e = Expr::Exp(Box::new(Expr::Const(1.0)));
    let size = ProblemSize::new(vec![]);
    assert!((e.eval(&size) - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_expr_log_eval() {
    let e = Expr::Log(Box::new(Expr::Const(std::f64::consts::E)));
    let size = ProblemSize::new(vec![]);
    assert!((e.eval(&size) - 1.0).abs() < 1e-10);
}

#[test]
fn test_expr_sqrt_eval() {
    let e = Expr::Sqrt(Box::new(Expr::Const(9.0)));
    let size = ProblemSize::new(vec![]);
    assert_eq!(e.eval(&size), 3.0);
}

#[test]
fn test_expr_complex() {
    // n^2 + 3*m
    let e = Expr::add(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        Expr::mul(Expr::Const(3.0), Expr::Var("m")),
    );
    let size = ProblemSize::new(vec![("n", 4), ("m", 2)]);
    assert_eq!(e.eval(&size), 22.0); // 16 + 6
}
```

**Step 2: Run tests to verify they fail**

Run: `make test` (or `cargo test expr`)
Expected: compilation errors — `Expr` type doesn't exist yet.

**Step 3: Implement `Expr` enum with eval**

Create `src/expr.rs`:
```rust
//! General symbolic expression AST for reduction overhead.

use crate::types::ProblemSize;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// A symbolic math expression over problem size variables.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    /// Numeric constant.
    Const(f64),
    /// Named variable (e.g., "num_vertices").
    Var(&'static str),
    /// Addition: a + b.
    Add(Box<Expr>, Box<Expr>),
    /// Multiplication: a * b.
    Mul(Box<Expr>, Box<Expr>),
    /// Exponentiation: base ^ exponent.
    Pow(Box<Expr>, Box<Expr>),
    /// Exponential function: exp(a).
    Exp(Box<Expr>),
    /// Natural logarithm: log(a).
    Log(Box<Expr>),
    /// Square root: sqrt(a).
    Sqrt(Box<Expr>),
}

impl Expr {
    /// Convenience constructors (avoid Box::new noise).
    pub fn add(a: Expr, b: Expr) -> Self {
        Expr::Add(Box::new(a), Box::new(b))
    }
    pub fn mul(a: Expr, b: Expr) -> Self {
        Expr::Mul(Box::new(a), Box::new(b))
    }
    pub fn pow(base: Expr, exp: Expr) -> Self {
        Expr::Pow(Box::new(base), Box::new(exp))
    }

    /// Evaluate the expression given concrete variable values.
    pub fn eval(&self, vars: &ProblemSize) -> f64 {
        match self {
            Expr::Const(c) => *c,
            Expr::Var(name) => vars.get(name).unwrap_or(0) as f64,
            Expr::Add(a, b) => a.eval(vars) + b.eval(vars),
            Expr::Mul(a, b) => a.eval(vars) * b.eval(vars),
            Expr::Pow(base, exp) => base.eval(vars).powf(exp.eval(vars)),
            Expr::Exp(a) => a.eval(vars).exp(),
            Expr::Log(a) => a.eval(vars).ln(),
            Expr::Sqrt(a) => a.eval(vars).sqrt(),
        }
    }
}

#[cfg(test)]
#[path = "unit_tests/expr.rs"]
mod tests;
```

Add to `src/lib.rs`:
```rust
pub(crate) mod expr;
```

**Step 4: Run tests to verify they pass**

Run: `cargo test expr`
Expected: all tests pass.

**Step 5: Commit**

```bash
git add src/expr.rs src/unit_tests/expr.rs src/lib.rs
git commit -m "feat: add Expr AST type with eval (phase 1 of overhead redesign)"
```

---

### Task 2: Add `variables()`, `substitute()`, and `Display` to `Expr`

**Files:**
- Modify: `src/expr.rs`
- Modify: `src/unit_tests/expr.rs`

**Step 1: Write failing tests**

Append to `src/unit_tests/expr.rs`:
```rust
#[test]
fn test_expr_variables() {
    let e = Expr::add(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        Expr::mul(Expr::Const(3.0), Expr::Var("m")),
    );
    let vars = e.variables();
    assert_eq!(vars, HashSet::from(["n", "m"]));
}

#[test]
fn test_expr_substitute() {
    // n^2, substitute n → (a + b)
    let e = Expr::pow(Expr::Var("n"), Expr::Const(2.0));
    let replacement = Expr::add(Expr::Var("a"), Expr::Var("b"));
    let mut mapping = HashMap::new();
    mapping.insert("n", &replacement);
    let result = e.substitute(&mapping);
    // Should be (a + b)^2
    let size = ProblemSize::new(vec![("a", 3), ("b", 2)]);
    assert_eq!(result.eval(&size), 25.0); // (3+2)^2
}

#[test]
fn test_expr_display_simple() {
    assert_eq!(format!("{}", Expr::Const(5.0)), "5");
    assert_eq!(format!("{}", Expr::Var("n")), "n");
}

#[test]
fn test_expr_display_add() {
    let e = Expr::add(Expr::Var("n"), Expr::Const(3.0));
    assert_eq!(format!("{e}"), "n + 3");
}

#[test]
fn test_expr_display_mul() {
    let e = Expr::mul(Expr::Const(3.0), Expr::Var("n"));
    assert_eq!(format!("{e}"), "3 * n");
}

#[test]
fn test_expr_display_pow() {
    let e = Expr::pow(Expr::Var("n"), Expr::Const(2.0));
    assert_eq!(format!("{e}"), "n^2");
}

#[test]
fn test_expr_display_exp() {
    let e = Expr::Exp(Box::new(Expr::Var("n")));
    assert_eq!(format!("{e}"), "exp(n)");
}

#[test]
fn test_expr_display_nested() {
    // n^2 + 3 * m
    let e = Expr::add(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        Expr::mul(Expr::Const(3.0), Expr::Var("m")),
    );
    assert_eq!(format!("{e}"), "n^2 + 3 * m");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test expr`
Expected: FAIL — `variables()`, `substitute()`, `Display` not implemented.

**Step 3: Implement the methods**

Add to `src/expr.rs`:
```rust
impl Expr {
    // ... existing methods ...

    /// Collect all variable names referenced in this expression.
    pub fn variables(&self) -> HashSet<&'static str> {
        let mut vars = HashSet::new();
        self.collect_variables(&mut vars);
        vars
    }

    fn collect_variables(&self, vars: &mut HashSet<&'static str>) {
        match self {
            Expr::Const(_) => {}
            Expr::Var(name) => { vars.insert(name); }
            Expr::Add(a, b) | Expr::Mul(a, b) | Expr::Pow(a, b) => {
                a.collect_variables(vars);
                b.collect_variables(vars);
            }
            Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) => {
                a.collect_variables(vars);
            }
        }
    }

    /// Substitute variables with other expressions.
    pub fn substitute(&self, mapping: &HashMap<&str, &Expr>) -> Expr {
        match self {
            Expr::Const(c) => Expr::Const(*c),
            Expr::Var(name) => {
                if let Some(replacement) = mapping.get(name) {
                    (*replacement).clone()
                } else {
                    Expr::Var(name)
                }
            }
            Expr::Add(a, b) => Expr::add(a.substitute(mapping), b.substitute(mapping)),
            Expr::Mul(a, b) => Expr::mul(a.substitute(mapping), b.substitute(mapping)),
            Expr::Pow(a, b) => Expr::pow(a.substitute(mapping), b.substitute(mapping)),
            Expr::Exp(a) => Expr::Exp(Box::new(a.substitute(mapping))),
            Expr::Log(a) => Expr::Log(Box::new(a.substitute(mapping))),
            Expr::Sqrt(a) => Expr::Sqrt(Box::new(a.substitute(mapping))),
        }
    }

    /// Check if this expression is a polynomial (no exp/log/sqrt, integer exponents only).
    pub fn is_polynomial(&self) -> bool {
        match self {
            Expr::Const(_) | Expr::Var(_) => true,
            Expr::Add(a, b) | Expr::Mul(a, b) => a.is_polynomial() && b.is_polynomial(),
            Expr::Pow(base, exp) => {
                base.is_polynomial() && matches!(exp.as_ref(), Expr::Const(c) if *c >= 0.0 && (*c - c.round()).abs() < 1e-10)
            }
            Expr::Exp(_) | Expr::Log(_) | Expr::Sqrt(_) => false,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Const(c) => {
                let ci = c.round() as i64;
                if (*c - ci as f64).abs() < 1e-10 {
                    write!(f, "{ci}")
                } else {
                    write!(f, "{c}")
                }
            }
            Expr::Var(name) => write!(f, "{name}"),
            Expr::Add(a, b) => write!(f, "{a} + {b}"),
            Expr::Mul(a, b) => {
                // Parenthesize additions inside multiplication
                let left = if matches!(a.as_ref(), Expr::Add(_, _)) {
                    format!("({a})")
                } else {
                    format!("{a}")
                };
                let right = if matches!(b.as_ref(), Expr::Add(_, _)) {
                    format!("({b})")
                } else {
                    format!("{b}")
                };
                write!(f, "{left} * {right}")
            }
            Expr::Pow(base, exp) => {
                let base_str = if matches!(base.as_ref(), Expr::Add(_, _) | Expr::Mul(_, _)) {
                    format!("({base})")
                } else {
                    format!("{base}")
                };
                write!(f, "{base_str}^{exp}")
            }
            Expr::Exp(a) => write!(f, "exp({a})"),
            Expr::Log(a) => write!(f, "log({a})"),
            Expr::Sqrt(a) => write!(f, "sqrt({a})"),
        }
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test expr`
Expected: all tests pass.

**Step 5: Commit**

```bash
git add src/expr.rs src/unit_tests/expr.rs
git commit -m "feat: add variables, substitute, Display to Expr"
```

---

## Phase 2: Proc macro expression parser

### Task 3: Add Pratt parser to the proc macro crate

**Files:**
- Create: `problemreductions-macros/src/parser.rs`
- Create: `problemreductions-macros/tests/parse_tests.rs`
- Modify: `problemreductions-macros/src/lib.rs` (add module)

The parser operates on `&str` (the contents of the string literal from the macro attribute) and produces a token stream that constructs `Expr` values.

**Step 1: Write failing parser tests**

Create `problemreductions-macros/tests/parse_tests.rs`:
```rust
use problemreductions_macros::__parse_overhead_expr;

// We'll expose a helper proc macro for testing that takes a string
// and outputs the Expr construction code. This is tested by compilation.

// For now, test the parser module directly via unit tests inside the crate.
```

Since proc macro crates can't be tested with normal `#[test]` easily for internal parse logic, add unit tests inside the module.

Create `problemreductions-macros/src/parser.rs`:
```rust
//! Pratt parser for overhead expression strings.
//!
//! Parses expressions like:
//! - "num_vertices"
//! - "num_vertices^2"
//! - "num_edges + num_vertices^2"
//! - "3 * num_vertices"
//! - "exp(num_vertices^2)"
//! - "sqrt(num_edges)"
//!
//! Grammar:
//!   expr     = term (('+' | '-') term)*
//!   term     = factor (('*' | '/') factor)*
//!   factor   = unary ('^' factor)?        // right-associative
//!   unary    = '-' unary | primary
//!   primary  = NUMBER | IDENT | func_call | '(' expr ')'
//!   func_call = ('exp' | 'log' | 'sqrt') '(' expr ')'

use proc_macro2::TokenStream;
use quote::quote;

/// Parsed expression node (intermediate representation before codegen).
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedExpr {
    Const(f64),
    Var(String),
    Add(Box<ParsedExpr>, Box<ParsedExpr>),
    Sub(Box<ParsedExpr>, Box<ParsedExpr>),
    Mul(Box<ParsedExpr>, Box<ParsedExpr>),
    Div(Box<ParsedExpr>, Box<ParsedExpr>),
    Pow(Box<ParsedExpr>, Box<ParsedExpr>),
    Neg(Box<ParsedExpr>),
    Exp(Box<ParsedExpr>),
    Log(Box<ParsedExpr>),
    Sqrt(Box<ParsedExpr>),
}

// ... tokenizer and parser implementation ...
// (detailed in Step 3)
```

**Step 2: Implement tokenizer**

Tokens needed: `Number(f64)`, `Ident(String)`, `Plus`, `Minus`, `Star`, `Slash`, `Caret`, `LParen`, `RParen`.

```rust
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => { chars.next(); }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            '*' => { chars.next(); tokens.push(Token::Star); }
            '/' => { chars.next(); tokens.push(Token::Slash); }
            '^' => { chars.next(); tokens.push(Token::Caret); }
            '(' => { chars.next(); tokens.push(Token::LParen); }
            ')' => { chars.next(); tokens.push(Token::RParen); }
            c if c.is_ascii_digit() || c == '.' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' { num.push(c); chars.next(); }
                    else { break; }
                }
                let val: f64 = num.parse().map_err(|_| format!("invalid number: {num}"))?;
                tokens.push(Token::Number(val));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' { ident.push(c); chars.next(); }
                    else { break; }
                }
                tokens.push(Token::Ident(ident));
            }
            _ => return Err(format!("unexpected character: '{ch}'")),
        }
    }
    Ok(tokens)
}
```

**Step 3: Implement Pratt parser**

```rust
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }
    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.advance() {
            Some(ref tok) if tok == expected => Ok(()),
            Some(tok) => Err(format!("expected {expected:?}, got {tok:?}")),
            None => Err(format!("expected {expected:?}, got end of input")),
        }
    }

    fn parse_expr(&mut self) -> Result<ParsedExpr, String> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = self.advance().unwrap();
            let right = self.parse_term()?;
            left = match op {
                Token::Plus => ParsedExpr::Add(Box::new(left), Box::new(right)),
                Token::Minus => ParsedExpr::Sub(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<ParsedExpr, String> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek(), Some(Token::Star) | Some(Token::Slash)) {
            let op = self.advance().unwrap();
            let right = self.parse_factor()?;
            left = match op {
                Token::Star => ParsedExpr::Mul(Box::new(left), Box::new(right)),
                Token::Slash => ParsedExpr::Div(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<ParsedExpr, String> {
        let base = self.parse_unary()?;
        if matches!(self.peek(), Some(Token::Caret)) {
            self.advance();
            let exp = self.parse_factor()?; // right-associative
            Ok(ParsedExpr::Pow(Box::new(base), Box::new(exp)))
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<ParsedExpr, String> {
        if matches!(self.peek(), Some(Token::Minus)) {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(ParsedExpr::Neg(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<ParsedExpr, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(ParsedExpr::Const(n)),
            Some(Token::Ident(name)) => {
                // Check for function call: exp(...), log(...), sqrt(...)
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.advance(); // consume '('
                    let arg = self.parse_expr()?;
                    self.expect(&Token::RParen)?;
                    match name.as_str() {
                        "exp" => Ok(ParsedExpr::Exp(Box::new(arg))),
                        "log" => Ok(ParsedExpr::Log(Box::new(arg))),
                        "sqrt" => Ok(ParsedExpr::Sqrt(Box::new(arg))),
                        _ => Err(format!("unknown function: {name}")),
                    }
                } else {
                    Ok(ParsedExpr::Var(name))
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Some(tok) => Err(format!("unexpected token: {tok:?}")),
            None => Err("unexpected end of input".to_string()),
        }
    }
}

/// Parse an expression string into a ParsedExpr.
pub fn parse_expr(input: &str) -> Result<ParsedExpr, String> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr()?;
    if parser.pos != parser.tokens.len() {
        return Err(format!("unexpected trailing tokens at position {}", parser.pos));
    }
    Ok(expr)
}
```

**Step 4: Add codegen functions**

Two codegen functions — one produces `Expr` AST construction code, the other produces Rust evaluation code that calls getters.

```rust
impl ParsedExpr {
    /// Generate TokenStream that constructs an `Expr` value.
    pub fn to_expr_tokens(&self) -> TokenStream {
        match self {
            ParsedExpr::Const(c) => quote! { crate::expr::Expr::Const(#c) },
            ParsedExpr::Var(name) => quote! { crate::expr::Expr::Var(#name) },
            ParsedExpr::Add(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { crate::expr::Expr::add(#a, #b) }
            }
            ParsedExpr::Sub(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { crate::expr::Expr::add(#a, crate::expr::Expr::mul(crate::expr::Expr::Const(-1.0), #b)) }
            }
            ParsedExpr::Mul(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { crate::expr::Expr::mul(#a, #b) }
            }
            ParsedExpr::Div(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { crate::expr::Expr::mul(#a, crate::expr::Expr::pow(#b, crate::expr::Expr::Const(-1.0))) }
            }
            ParsedExpr::Pow(base, exp) => {
                let base = base.to_expr_tokens();
                let exp = exp.to_expr_tokens();
                quote! { crate::expr::Expr::pow(#base, #exp) }
            }
            ParsedExpr::Neg(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::mul(crate::expr::Expr::Const(-1.0), #a) }
            }
            ParsedExpr::Exp(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::Exp(Box::new(#a)) }
            }
            ParsedExpr::Log(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::Log(Box::new(#a)) }
            }
            ParsedExpr::Sqrt(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::Sqrt(Box::new(#a)) }
            }
        }
    }

    /// Generate TokenStream that evaluates the expression by calling getter methods
    /// on a source variable `src`.
    pub fn to_eval_tokens(&self, src_ident: &syn::Ident) -> TokenStream {
        match self {
            ParsedExpr::Const(c) => quote! { (#c as f64) },
            ParsedExpr::Var(name) => {
                let getter = syn::Ident::new(name, proc_macro2::Span::call_site());
                quote! { (#src_ident.#getter() as f64) }
            }
            ParsedExpr::Add(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a + #b) }
            }
            ParsedExpr::Sub(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a - #b) }
            }
            ParsedExpr::Mul(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a * #b) }
            }
            ParsedExpr::Div(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a / #b) }
            }
            ParsedExpr::Pow(base, exp) => {
                let base = base.to_eval_tokens(src_ident);
                let exp = exp.to_eval_tokens(src_ident);
                quote! { f64::powf(#base, #exp) }
            }
            ParsedExpr::Neg(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { (-(#a)) }
            }
            ParsedExpr::Exp(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { f64::exp(#a) }
            }
            ParsedExpr::Log(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { f64::ln(#a) }
            }
            ParsedExpr::Sqrt(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { f64::sqrt(#a) }
            }
        }
    }

    /// Collect all variable names in the expression.
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }

    fn collect_vars(&self, vars: &mut Vec<String>) {
        match self {
            ParsedExpr::Const(_) => {}
            ParsedExpr::Var(name) => vars.push(name.clone()),
            ParsedExpr::Add(a, b) | ParsedExpr::Sub(a, b)
            | ParsedExpr::Mul(a, b) | ParsedExpr::Div(a, b)
            | ParsedExpr::Pow(a, b) => {
                a.collect_vars(vars);
                b.collect_vars(vars);
            }
            ParsedExpr::Neg(a) | ParsedExpr::Exp(a) | ParsedExpr::Log(a) | ParsedExpr::Sqrt(a) => {
                a.collect_vars(vars);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_var() {
        assert_eq!(parse_expr("num_vertices").unwrap(), ParsedExpr::Var("num_vertices".into()));
    }

    #[test]
    fn test_parse_const() {
        assert_eq!(parse_expr("42").unwrap(), ParsedExpr::Const(42.0));
    }

    #[test]
    fn test_parse_pow() {
        let e = parse_expr("n^2").unwrap();
        assert_eq!(e, ParsedExpr::Pow(
            Box::new(ParsedExpr::Var("n".into())),
            Box::new(ParsedExpr::Const(2.0)),
        ));
    }

    #[test]
    fn test_parse_add_mul() {
        // n + 3 * m  →  n + (3*m)
        let e = parse_expr("n + 3 * m").unwrap();
        assert_eq!(e, ParsedExpr::Add(
            Box::new(ParsedExpr::Var("n".into())),
            Box::new(ParsedExpr::Mul(
                Box::new(ParsedExpr::Const(3.0)),
                Box::new(ParsedExpr::Var("m".into())),
            )),
        ));
    }

    #[test]
    fn test_parse_exp() {
        let e = parse_expr("exp(n^2)").unwrap();
        assert_eq!(e, ParsedExpr::Exp(Box::new(ParsedExpr::Pow(
            Box::new(ParsedExpr::Var("n".into())),
            Box::new(ParsedExpr::Const(2.0)),
        ))));
    }

    #[test]
    fn test_parse_complex() {
        // 3 * n^2 + exp(m) — should parse correctly
        let e = parse_expr("3 * n^2 + exp(m)").unwrap();
        assert!(matches!(e, ParsedExpr::Add(_, _)));
    }

    #[test]
    fn test_parse_parens() {
        let e = parse_expr("(n + m)^2").unwrap();
        assert!(matches!(e, ParsedExpr::Pow(_, _)));
    }

    #[test]
    fn test_variables() {
        let e = parse_expr("n^2 + 3 * m + exp(k)").unwrap();
        assert_eq!(e.variables(), vec!["k", "m", "n"]);
    }
}
```

**Step 5: Run tests**

Run: `cargo test -p problemreductions-macros`
Expected: all parser tests pass.

**Step 6: Commit**

```bash
git add problemreductions-macros/src/parser.rs
git commit -m "feat: add Pratt expression parser to proc macro crate"
```

---

### Task 4: Update `#[reduction]` macro to support new syntax

**Files:**
- Modify: `problemreductions-macros/src/lib.rs`

The macro should support **both** old syntax (for backwards compatibility during migration) and new syntax:

Old: `overhead = { ReductionOverhead::new(vec![...]) }`
New: `overhead = { num_vars = "num_vertices^2", num_constraints = "num_edges" }`

Detection: if the content starts with an identifier followed by `=` and a string literal, it's new syntax. Otherwise, treat the braced content as raw token stream (old syntax).

**Step 1: Update `ReductionAttrs` parsing**

Add a new variant to represent parsed overhead fields:
```rust
enum OverheadSpec {
    /// Old syntax: raw token stream (ReductionOverhead::new(...))
    Legacy(TokenStream2),
    /// New syntax: list of (field_name, expression_string) pairs
    Parsed(Vec<(String, String)>),
}
```

Update `ReductionAttrs` to store `OverheadSpec` and the parsing logic to detect which format is used.

**Step 2: Update `generate_reduction_entry` to emit dual code**

For `OverheadSpec::Parsed`, use the parser from Task 3:
- Parse each expression string at compile time
- Emit `overhead_fn` that constructs `ReductionOverhead` with `Expr` AST
- Emit `overhead_eval_fn` that calls getters on the concrete source type
- Report parse errors as compile errors with `syn::Error`

For `OverheadSpec::Legacy`, emit the old behavior unchanged.

**Step 3: Update `ReductionEntry` to include the new field**

This requires modifying `src/rules/registry.rs` to add `overhead_eval_fn`. For now, legacy reductions pass a no-op eval fn:
```rust
pub overhead_eval_fn: Option<fn(&dyn Any) -> ProblemSize>,
```

Using `Option` allows legacy code to work with `None` while new syntax populates `Some(...)`.

**Step 4: Test with one reduction**

Pick a simple reduction (e.g., `maximumindependentset_qubo.rs`) and convert it to new syntax as a proof:

Before:
```rust
#[reduction(
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices))]) }
)]
```

After:
```rust
#[reduction(overhead = {
    num_vars = "num_vertices",
})]
```

Run: `cargo test maximumindependentset_qubo`
Expected: passes (both old overhead_fn and new eval paths work).

**Step 5: Commit**

```bash
git add problemreductions-macros/src/lib.rs src/rules/registry.rs
git commit -m "feat: support new overhead expression syntax in #[reduction] macro"
```

---

## Phase 3: Add inherent getters to all problem types

### Task 5: Add getters to graph problem types

**Files:**
- Modify: `src/models/graph/maximum_independent_set.rs`
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/traveling_salesman.rs`

For each graph problem that has `problem_size_names = ["num_vertices", "num_edges"]`, add inherent getters. Most already have `graph()` accessor, so the getters are trivial:

```rust
impl<G: Graph, W: WeightElement> MaximumIndependentSet<G, W> {
    pub fn num_vertices(&self) -> usize { self.graph().num_vertices() }
    pub fn num_edges(&self) -> usize { self.graph().num_edges() }
}
```

Check each problem's `problem_size_values()` to see what getters are needed — some problems may have additional fields. For example:
- Most graph problems: `num_vertices`, `num_edges`
- KColoring: `num_vertices`, `num_edges` (same)
- TravelingSalesman: check the actual fields

**Step 1: Add getters to all 9 graph problem files**

Read each file's `problem_size_values()` to determine exact getters needed. Add inherent `impl` blocks with `pub fn` getters. If a getter already exists as a public method, skip it.

**Step 2: Run tests**

Run: `cargo test`
Expected: all existing tests pass (getters are additive).

**Step 3: Commit**

```bash
git add src/models/graph/
git commit -m "feat: add inherent size getters to graph problem types"
```

---

### Task 6: Add getters to remaining problem types

**Files:**
- Modify: `src/models/satisfiability/sat.rs`
- Modify: `src/models/satisfiability/ksat.rs`
- Modify: `src/models/optimization/qubo.rs`
- Modify: `src/models/optimization/ilp.rs`
- Modify: `src/models/optimization/spin_glass.rs`
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`
- Modify: `src/models/specialized/circuit.rs`
- Modify: `src/models/specialized/factoring.rs`
- Modify: `src/models/specialized/paintshop.rs`
- Modify: `src/models/specialized/bmf.rs`
- Modify: `src/models/specialized/biclique_cover.rs`

Same approach: read `problem_size_values()` for each, add inherent getter methods. Examples:
- Satisfiability: `num_vars()`, `num_clauses()`, `num_literals()` (may already exist)
- QUBO: `num_vars()`
- ILP: `num_vars()`, `num_constraints()`
- SpinGlass: check fields
- CircuitSAT: `num_variables()`, `num_assignments()`

**Step 1: Add getters to all remaining problem files**

**Step 2: Run tests**

Run: `cargo test`
Expected: all tests pass.

**Step 3: Commit**

```bash
git add src/models/
git commit -m "feat: add inherent size getters to SAT, optimization, set, and specialized problems"
```

---

## Phase 4: Migrate all reductions to new syntax

### Task 7: Migrate simple reductions (single field, simple expression)

**Files:** ~15 reduction files with simple `poly!(var)` or `poly!(var^N)` patterns.

Target files (identified from grep): `maximumindependentset_qubo.rs`, `coloring_qubo.rs`, `ksatisfiability_qubo.rs` (K2), `ilp_qubo.rs`, `maximumsetpacking_qubo.rs`, `minimumvertexcover_qubo.rs`, `spinglass_qubo.rs`, etc.

For each file, replace:
```rust
overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices))]) }
```
with:
```rust
overhead = { num_vars = "num_vertices" }
```

And:
```rust
overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices ^ 2))]) }
```
with:
```rust
overhead = { num_vars = "num_vertices^2" }
```

**Step 1: Migrate files**

Mechanical replacement. Remove any `use crate::poly;` or `use crate::rules::registry::ReductionOverhead;` imports that become unused.

**Step 2: Run tests**

Run: `cargo test`
Expected: all tests pass.

**Step 3: Commit**

```bash
git add src/rules/
git commit -m "refactor: migrate simple reductions to new overhead syntax"
```

---

### Task 8: Migrate complex reductions (multi-field, compound expressions)

**Files:** Remaining ~20 reduction files with multi-field or compound polynomial expressions.

These include reductions like `factoring_circuit.rs`, `circuit_spinglass.rs`, `sat_coloring.rs`, `maximumindependentset_ilp.rs`, etc.

For compound expressions using `poly!() + poly!()`:
```rust
overhead = {
    ReductionOverhead::new(vec![
        ("num_vars", poly!(num_vars) + poly!(num_clauses)),
    ])
}
```
becomes:
```rust
overhead = {
    num_vars = "num_vars + num_clauses",
}
```

For multi-field:
```rust
overhead = {
    ReductionOverhead::new(vec![
        ("num_vars", poly!(num_vertices)),
        ("num_constraints", poly!(num_edges)),
    ])
}
```
becomes:
```rust
overhead = {
    num_vars = "num_vertices",
    num_constraints = "num_edges",
}
```

**Step 1: Migrate files**

Read each file's current overhead carefully. Convert polynomial expressions to string syntax. Some expressions may use `poly!(a * b)` (product) — convert to `"a * b"`.

**Step 2: Run tests**

Run: `cargo test`
Expected: all tests pass.

**Step 3: Commit**

```bash
git add src/rules/
git commit -m "refactor: migrate complex reductions to new overhead syntax"
```

---

### Task 9: Migrate variant cast reductions

**Files:**
- Modify: `src/rules/mod.rs` (the `impl_variant_reduction!` macro)
- Modify: cast files (`kcoloring_casts.rs`, `maximumindependentset_casts.rs`, etc.)

The `impl_variant_reduction!` macro uses `ReductionOverhead::identity(fields)`. This still works with the new system since identity overhead maps each field to itself. Update the macro to use the new syntax if possible, or keep `ReductionOverhead::identity()` updated to use `Expr::Var` instead of `Polynomial::var`.

Since `ReductionOverhead::identity()` will now construct `Expr::Var` values (after Phase 5), this migration may be minimal — just ensure the macro still compiles.

**Step 1: Verify variant casts still compile and pass tests**

Run: `cargo test`
Expected: all tests pass.

**Step 2: Commit (if changes needed)**

```bash
git commit -m "refactor: update variant cast macro for new overhead system"
```

---

## Phase 5: Remove deprecated APIs

### Task 10: Switch `ReductionOverhead` from `Polynomial` to `Expr`

**Files:**
- Modify: `src/rules/registry.rs`
- Modify: `src/export.rs`
- Modify: `src/rules/cost.rs` (if needed)

**Step 1: Update `ReductionOverhead` to use `Expr`**

```rust
pub struct ReductionOverhead {
    pub output_size: Vec<(&'static str, Expr)>,
}
```

Update all methods: `evaluate_output_size` calls `Expr::eval`, `compose` calls `Expr::substitute`, `input_variable_names` calls `Expr::variables`, `identity` creates `Expr::Var` values.

**Step 2: Update `export.rs`**

Replace `MonomialJson`/`OverheadEntry` with the new format:
```rust
pub struct OverheadEntry {
    pub field: String,
    pub expr: Expr,
    pub formula: String,
}
```

**Step 3: Run tests, fix any compilation errors**

Run: `cargo test`
Fix any remaining references to `Polynomial` in overhead contexts.

**Step 4: Commit**

```bash
git add src/rules/registry.rs src/export.rs
git commit -m "refactor: switch ReductionOverhead from Polynomial to Expr"
```

---

### Task 11: Remove `problem_size_names` and `problem_size_values` from `Problem` trait

**Files:**
- Modify: `src/traits.rs`
- Modify: all 21 model files (remove trait method impls)
- Modify: `src/lib.rs` (remove `problem_size` re-export if no longer used)
- Modify: `src/types.rs` (keep `ProblemSize` but remove `from_names_values` if unused)

**Step 1: Remove from trait definition**

Remove `problem_size_names()` and `problem_size_values()` from the `Problem` trait in `src/traits.rs`. Remove the `problem_size()` helper function.

**Step 2: Remove implementations from all 21 model files**

Remove the `problem_size_names()` and `problem_size_values()` method bodies from each Problem impl.

**Step 3: Remove `source_size_names_fn` and `target_size_names_fn` from `ReductionEntry`**

Update `src/rules/registry.rs` and the proc macro to no longer emit these fields.

**Step 4: Fix compilation errors**

Search for all remaining uses of `problem_size_names`, `problem_size_values`, `problem_size(`, `source_size_names_fn`, `target_size_names_fn` and update or remove them.

**Step 5: Run tests**

Run: `cargo test`
Fix any remaining failures.

**Step 6: Commit**

```bash
git add src/traits.rs src/models/ src/rules/registry.rs src/lib.rs problemreductions-macros/src/lib.rs
git commit -m "refactor: remove problem_size_names/values from Problem trait"
```

---

### Task 12: Remove `Polynomial` and `poly!` macro

**Files:**
- Delete: `src/polynomial.rs`
- Delete: `src/unit_tests/polynomial.rs`
- Modify: `src/lib.rs` (remove `mod polynomial`)

**Step 1: Search for any remaining `Polynomial` or `poly!` references**

Run: `cargo build` — if it compiles, no references remain.

**Step 2: Delete files**

**Step 3: Run full test suite**

Run: `make check`
Expected: fmt + clippy + test all pass.

**Step 4: Commit**

```bash
git add -A
git commit -m "refactor: remove Polynomial type and poly! macro (replaced by Expr)"
```

---

## Phase 6: Update documentation and exports

### Task 13: Regenerate exports and update docs

**Files:**
- Modify: `docs/src/reductions/reduction_graph.json` (auto-generated)
- Modify: `docs/paper/reductions.typ` (if format-overhead needs updating)
- Modify: CLAUDE.md (update conventions)
- Regenerate: `tests/data/` ground truth JSON (if format changed)

**Step 1: Regenerate reduction graph JSON**

Run: `make rust-export`
Check that the new JSON format has `expr` + `formula` fields instead of `polynomial`.

**Step 2: Update paper if needed**

Check `docs/paper/reductions.typ` — the `format-overhead` function reads `formula` fields. If the field name changed, update it.

**Step 3: Regenerate test data**

Run: `make qubo-testdata`
If example JSON format changed, regenerate example outputs.

**Step 4: Run full CI check**

Run: `make check`
Expected: all pass.

**Step 5: Update CLAUDE.md**

Update the Architecture section to reference `Expr` instead of `Polynomial`, and document the new `#[reduction]` syntax.

**Step 6: Commit**

```bash
git add -A
git commit -m "docs: update exports and documentation for new overhead system"
```

---

### Task 14: Update MCP server (if applicable)

**Files:**
- Check: `problemreductions-cli/` for any MCP-specific overhead formatting

**Step 1: Search for overhead-related code in CLI/MCP**

The MCP server's `inspect_problem` and `reduce` tools return overhead info. Ensure they use the new `formula` field.

**Step 2: Run MCP tests**

Run: `make mcp-test`
Expected: all pass.

**Step 3: Commit if changes needed**

```bash
git commit -m "fix: update MCP server for new overhead format"
```
