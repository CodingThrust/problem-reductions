# Rust Features Guide

This guide explains all key Rust language features used in the problem-reductions library. Perfect for Rust newcomers to understand the codebase.

## Table of Contents

1. [Traits](#1-traits)
2. [Generics](#2-generics)
3. [Associated Types](#3-associated-types)
4. [Trait Bounds](#4-trait-bounds)
5. [PhantomData](#5-phantomdata)
6. [Type Aliases](#6-type-aliases)
7. [Enums](#7-enums)
8. [Pattern Matching](#8-pattern-matching)
9. [Derive Macros](#9-derive-macros)
10. [Declarative Macros](#10-declarative-macros)
11. [Modules and Visibility](#11-modules-and-visibility)
12. [Iterators](#12-iterators)
13. [Closures](#13-closures)
14. [Error Handling](#14-error-handling)
15. [Lifetimes](#15-lifetimes)
16. [Const and Static](#16-const-and-static)
17. [Marker Traits](#17-marker-traits)
18. [Builder Pattern](#18-builder-pattern)
19. [Serde Serialization](#19-serde-serialization)
20. [Common Standard Library Types](#20-common-standard-library-types)

---

## 1. Traits

Traits define shared behavior. They're similar to interfaces in other languages.

### Basic Trait Definition

```rust
// Define a trait with required methods
pub trait Problem {
    fn num_variables(&self) -> usize;
    fn num_flavors(&self) -> usize;
}

// Implement the trait for a type
impl Problem for MyProblem {
    fn num_variables(&self) -> usize {
        self.variables.len()
    }

    fn num_flavors(&self) -> usize {
        2  // Binary problem
    }
}
```

### Provided Methods (Default Implementations)

```rust
pub trait Problem {
    fn num_variables(&self) -> usize;  // Required

    // Provided method with default implementation
    fn variables(&self) -> std::ops::Range<usize> {
        0..self.num_variables()
    }
}
```

Implementors get `variables()` for free but can override it.

### Trait Inheritance (Supertraits)

```rust
// ConstraintSatisfactionProblem requires Problem to be implemented first
pub trait ConstraintSatisfactionProblem: Problem {
    fn constraints(&self) -> Vec<LocalConstraint>;
}
```

**Used in this library**:
- `src/traits.rs` - `Problem`, `ConstraintSatisfactionProblem`
- `src/solvers/mod.rs` - `Solver`
- `src/models/graph/template.rs` - `GraphConstraint`

---

## 2. Generics

Generics allow writing code that works with multiple types.

### Generic Structs

```rust
// W is a type parameter (defaults to i32)
pub struct GraphProblem<C, W = i32> {
    weights: Vec<W>,
    // ...
}

// Usage
let problem: GraphProblem<MyConstraint, i32> = ...;
let problem: GraphProblem<MyConstraint, f64> = ...;  // Different weight type
let problem: GraphProblem<MyConstraint> = ...;       // Uses default W = i32
```

### Generic Functions

```rust
// T is a type parameter
fn find_best<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,  // T must be comparable
{
    items.iter().max()
}
```

### Generic Impl Blocks

```rust
// Implement for all W types that meet the bounds
impl<C: GraphConstraint, W: Clone + Default> GraphProblem<C, W> {
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        // ...
    }
}
```

**Used in this library**:
- `GraphProblem<C, W>` - generic over constraint type and weight type
- `SolutionSize<T>` - generic over size type
- `Solver::find_best<P: Problem>` - generic over problem type

---

## 3. Associated Types

Associated types are type placeholders in traits, defined by implementors.

```rust
pub trait Problem {
    // Associated type - implementor chooses the concrete type
    type Size: Clone + PartialOrd;

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size>;
}

// Implementation specifies the concrete type
impl Problem for IndependentSet {
    type Size = i32;  // This problem uses i32 for sizes

    fn solution_size(&self, config: &[usize]) -> SolutionSize<i32> {
        // ...
    }
}
```

### Why Associated Types vs Generics?

```rust
// With associated type (cleaner)
fn solve<P: Problem>(problem: &P) -> P::Size { ... }

// With generics (more verbose)
fn solve<P: Problem<Size = S>, S>(problem: &P) -> S { ... }
```

Associated types are used when there's exactly one type that makes sense per implementation.

**Used in this library**:
- `Problem::Size` - the objective value type

---

## 4. Trait Bounds

Trait bounds constrain generic types to those implementing certain traits.

### Where Clauses

```rust
impl<W> Problem for GraphProblem<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    // W must implement all these traits
}
```

### Multiple Bounds

```rust
// Using + to require multiple traits
fn process<T: Clone + Debug + Send>(item: T) { ... }

// Equivalent where clause (more readable for many bounds)
fn process<T>(item: T)
where
    T: Clone + Debug + Send,
{ ... }
```

### Common Trait Bounds in This Library

| Bound | Meaning |
|-------|---------|
| `Clone` | Can be duplicated with `.clone()` |
| `Copy` | Can be copied implicitly (bit-by-bit) |
| `Default` | Has a default value via `Default::default()` |
| `PartialOrd` | Can be compared with `<`, `>`, etc. |
| `Send` | Safe to send between threads |
| `Sync` | Safe to share references between threads |
| `'static` | Contains no borrowed references (or they're `'static`) |

**Used in this library**:
- `W: Clone + Default + PartialOrd + Num + Zero + AddAssign`
- `C: GraphConstraint` (which requires `Clone + Send + Sync + 'static`)

---

## 5. PhantomData

`PhantomData<T>` is a zero-size type that tells the compiler "this struct logically owns a T".

### The Problem

```rust
// This won't compile - C is unused!
pub struct GraphProblem<C, W> {
    weights: Vec<W>,
    // C is not used anywhere in fields
}
```

### The Solution

```rust
use std::marker::PhantomData;

pub struct GraphProblem<C, W> {
    weights: Vec<W>,
    _constraint: PhantomData<C>,  // Zero-size, just marks that C is "used"
}

// Creating PhantomData
let _constraint = PhantomData::<MyConstraint>;
// or
let _constraint: PhantomData<MyConstraint> = PhantomData;
```

### Why Use It?

1. **Type safety**: Different `C` types create different `GraphProblem` types
2. **Zero runtime cost**: `PhantomData` has size 0
3. **Compiler satisfaction**: Makes unused type parameters valid

**Used in this library**:
- `GraphProblem<C, W>` uses `PhantomData<C>` to carry the constraint type

---

## 6. Type Aliases

Type aliases create new names for existing types.

### Basic Alias

```rust
// Create a shorter name
pub type IndependentSetT<W = i32> = GraphProblem<IndependentSetConstraint, W>;

// Usage - these are equivalent:
let p1: GraphProblem<IndependentSetConstraint, i32> = ...;
let p2: IndependentSetT<i32> = ...;
let p3: IndependentSetT = ...;  // Uses default W = i32
```

### Result Type Alias (Common Pattern)

```rust
// In error.rs
pub type Result<T> = std::result::Result<T, ProblemError>;

// Usage - cleaner than writing the full type
fn load_problem() -> Result<MyProblem> { ... }
```

**Used in this library**:
- `IndependentSetT<W>`, `VertexCoverT<W>`, etc.
- `Result<T>` in error handling

---

## 7. Enums

Enums define types that can be one of several variants.

### Basic Enum

```rust
pub enum EnergyMode {
    LargerSizeIsBetter,   // Maximization
    SmallerSizeIsBetter,  // Minimization
}

// Usage
let mode = EnergyMode::LargerSizeIsBetter;
```

### Enum with Data

```rust
pub enum ProblemCategory {
    Graph(GraphSubcategory),           // Contains a GraphSubcategory
    Satisfiability(SatSubcategory),    // Contains a SatSubcategory
    Set(SetSubcategory),
}

// Usage
let cat = ProblemCategory::Graph(GraphSubcategory::Independent);
```

### Enum Methods

```rust
impl EnergyMode {
    pub fn is_maximization(&self) -> bool {
        matches!(self, EnergyMode::LargerSizeIsBetter)
    }

    pub fn is_better<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        match self {
            EnergyMode::LargerSizeIsBetter => a > b,
            EnergyMode::SmallerSizeIsBetter => a < b,
        }
    }
}
```

**Used in this library**:
- `EnergyMode` - optimization direction
- `ProblemCategory` - problem classification
- `ComplexityClass` - P, NP-complete, etc.
- `ProblemError` - error types

---

## 8. Pattern Matching

Pattern matching extracts data from enums and other types.

### Match Expression

```rust
match self.energy_mode() {
    EnergyMode::LargerSizeIsBetter => {
        // Maximization logic
    }
    EnergyMode::SmallerSizeIsBetter => {
        // Minimization logic
    }
}
```

### If Let (Single Pattern)

```rust
// Only handle one variant
if let Some(weight) = self.weights.get(i) {
    println!("Weight: {}", weight);
}

// Equivalent to:
match self.weights.get(i) {
    Some(weight) => println!("Weight: {}", weight),
    None => {}
}
```

### Matches! Macro

```rust
// Returns bool - cleaner than match for simple checks
pub fn is_hard(&self) -> bool {
    matches!(
        self,
        ComplexityClass::NpComplete | ComplexityClass::NpHard
    )
}
```

### Destructuring

```rust
// Extract data from enum variants
match category {
    ProblemCategory::Graph(sub) => {
        println!("Graph subcategory: {:?}", sub);
    }
    _ => {}  // Wildcard - matches anything else
}

// Destructure structs
let SolutionSize { size, is_valid } = problem.solution_size(&config);
```

**Used throughout the library** for handling enums and Option/Result types.

---

## 9. Derive Macros

Derive macros automatically implement traits for your types.

### Common Derives

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplexityClass {
    P,
    NpComplete,
    NpHard,
}
```

| Derive | What It Does |
|--------|--------------|
| `Debug` | Enables `{:?}` formatting for printing |
| `Clone` | Adds `.clone()` method for deep copy |
| `Copy` | Enables implicit copying (for small types) |
| `PartialEq` | Enables `==` and `!=` comparison |
| `Eq` | Marks that equality is reflexive (a == a) |
| `Hash` | Enables use as HashMap key |
| `Default` | Adds `Default::default()` constructor |

### Serde Derives

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GraphProblem<C, W> {
    weights: Vec<W>,
    #[serde(skip)]  // Don't serialize this field
    _constraint: PhantomData<C>,
}
```

**Used in this library**:
- Most types derive `Debug`, `Clone`
- Enums often derive `Copy`, `PartialEq`, `Eq`, `Hash`
- Problem types derive `Serialize`, `Deserialize`

---

## 10. Declarative Macros

Declarative macros (`macro_rules!`) generate code at compile time.

### Basic Macro

```rust
#[macro_export]  // Makes macro available to users of the crate
macro_rules! quick_problem_test {
    // Pattern to match
    (
        $problem_type:ty,                           // Type
        $constructor:ident($($args:expr),*),        // Identifier and expressions
        solution: [$($sol:expr),*],                 // Expressions
        expected_size: $size:expr,
        is_valid: $valid:expr
    ) => {
        // Code to generate
        {
            let problem = <$problem_type>::$constructor($($args),*);
            let solution = vec![$($sol),*];
            let result = problem.solution_size(&solution);
            assert_eq!(result.size, $size);
            assert_eq!(result.is_valid, $valid);
        }
    };
}
```

### Macro Fragment Types

| Fragment | Matches |
|----------|---------|
| `$name:ident` | Identifier (variable/function name) |
| `$e:expr` | Expression |
| `$t:ty` | Type |
| `$p:pat` | Pattern |
| `$b:block` | Block `{ ... }` |
| `$s:stmt` | Statement |
| `$l:lifetime` | Lifetime `'a` |
| `$m:meta` | Attribute content |
| `$tt:tt` | Single token tree |

### Repetition

```rust
// $(...),* means "zero or more, comma-separated"
// $(...),+ means "one or more, comma-separated"
// $(...);* means "zero or more, semicolon-separated"

macro_rules! create_vec {
    ($($element:expr),*) => {
        vec![$($element),*]
    };
}

let v = create_vec![1, 2, 3];  // Expands to vec![1, 2, 3]
```

### Complex Macro Example

```rust
#[macro_export]
macro_rules! graph_problem_tests {
    (
        problem_type: $problem:ty,
        constraint_type: $constraint:ty,
        test_cases: [
            $(
                ($name:ident, $n:expr, [$($edge:expr),*], [$($sol:expr),*], $size:expr, $is_max:expr)
            ),* $(,)?  // Optional trailing comma
        ]
    ) => {
        mod generated_tests {
            use super::*;

            $(  // Repeat for each test case
                mod $name {
                    use super::*;

                    #[test]
                    fn test_creation() {
                        let problem = <$problem>::new($n, vec![$($edge),*]);
                        assert_eq!(problem.num_variables(), $n);
                    }

                    #[test]
                    fn test_solution() {
                        let problem = <$problem>::new($n, vec![$($edge),*]);
                        let solution = vec![$($sol),*];
                        let result = problem.solution_size(&solution);
                        assert_eq!(result.size, $size);
                    }
                }
            )*
        }
    };
}
```

**Used in this library**:
- `src/testing/macros.rs` - `graph_problem_tests!`, `complement_test!`, `quick_problem_test!`

---

## 11. Modules and Visibility

Rust organizes code into modules with explicit visibility.

### Module Declaration

```rust
// In lib.rs or main.rs
pub mod models;      // Load from models/mod.rs or models.rs
pub mod solvers;
mod internal;        // Private module (no pub)

// In models/mod.rs
pub mod graph;       // Load from models/graph/mod.rs
pub mod sat;
```

### Visibility Modifiers

| Modifier | Meaning |
|----------|---------|
| (none) | Private to current module |
| `pub` | Public to everyone |
| `pub(crate)` | Public within this crate only |
| `pub(super)` | Public to parent module |
| `pub(in path)` | Public to specific path |

```rust
pub struct MyStruct {
    pub public_field: i32,
    private_field: i32,           // Private
    pub(crate) crate_field: i32,  // Crate-public
}
```

### Re-exports

```rust
// In lib.rs - make nested items available at crate root
pub use models::graph::IndependentSetT;
pub use solvers::BruteForce;

// Users can now write:
use problemreductions::IndependentSetT;
// Instead of:
use problemreductions::models::graph::IndependentSetT;
```

### Prelude Pattern

```rust
// In prelude.rs - common imports bundled together
pub use crate::traits::{Problem, ConstraintSatisfactionProblem};
pub use crate::types::{EnergyMode, SolutionSize};
pub use crate::solvers::BruteForce;

// Users import everything at once:
use problemreductions::prelude::*;
```

**Used in this library**:
- `src/lib.rs` - module declarations and re-exports
- `src/prelude.rs` - common imports
- Each subdirectory has `mod.rs` for organization

---

## 12. Iterators

Iterators provide a way to process sequences of elements.

### Iterator Trait

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // Many provided methods...
}
```

### Common Iterator Methods

```rust
let numbers = vec![1, 2, 3, 4, 5];

// map - transform each element
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// filter - keep elements matching predicate
let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();

// fold - accumulate into single value
let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);

// enumerate - add indices
for (i, x) in numbers.iter().enumerate() {
    println!("Index {}: {}", i, x);
}

// any/all - boolean checks
let has_five = numbers.iter().any(|&x| x == 5);
let all_positive = numbers.iter().all(|&x| x > 0);

// find - get first matching element
let first_even = numbers.iter().find(|&&x| x % 2 == 0);

// chain - combine iterators
let combined: Vec<i32> = vec![1, 2].into_iter()
    .chain(vec![3, 4].into_iter())
    .collect();
```

### iter() vs into_iter() vs iter_mut()

```rust
let mut v = vec![1, 2, 3];

// iter() - borrows, yields &T
for x in v.iter() { /* x is &i32 */ }

// iter_mut() - mutable borrow, yields &mut T
for x in v.iter_mut() { *x += 1; }

// into_iter() - takes ownership, yields T
for x in v.into_iter() { /* x is i32, v is consumed */ }
```

### Custom Iterator

```rust
pub struct ConfigIterator {
    current: usize,
    total: usize,
    num_variables: usize,
    num_flavors: usize,
}

impl Iterator for ConfigIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total {
            return None;
        }
        let config = index_to_config(self.current, self.num_variables, self.num_flavors);
        self.current += 1;
        Some(config)
    }
}
```

**Used in this library**:
- `ConfigIterator` for enumerating all configurations
- Extensive use of iterator combinators in solvers and constraint evaluation

---

## 13. Closures

Closures are anonymous functions that can capture their environment.

### Basic Closure

```rust
// Closure with inferred types
let add = |a, b| a + b;
let result = add(2, 3);  // 5

// Closure with explicit types
let add: fn(i32, i32) -> i32 = |a, b| a + b;

// Multi-line closure
let complex = |x| {
    let y = x * 2;
    y + 1
};
```

### Capturing Variables

```rust
let factor = 2;

// Borrow by reference (Fn trait)
let multiply = |x| x * factor;

// Borrow mutably (FnMut trait)
let mut count = 0;
let mut increment = || { count += 1; };

// Take ownership (FnOnce trait)
let data = vec![1, 2, 3];
let consume = move || {
    println!("{:?}", data);
    // data is now owned by the closure
};
```

### Closures as Arguments

```rust
// Using iterator methods
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)      // Closure as argument
    .filter(|x| x > &5)  // Another closure
    .collect();

// Custom function taking closure
fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,  // F is a closure that takes and returns i32
{
    f(f(x))
}
```

**Used in this library**:
- Iterator chains in constraint evaluation
- Map/filter operations on solutions

---

## 14. Error Handling

Rust uses `Result` and `Option` for error handling instead of exceptions.

### Option Type

```rust
enum Option<T> {
    Some(T),
    None,
}

// Usage
fn find_weight(&self, index: usize) -> Option<&W> {
    self.weights.get(index)  // Returns None if out of bounds
}

// Handling Option
match result {
    Some(value) => println!("Found: {}", value),
    None => println!("Not found"),
}

// Shortcuts
let value = result.unwrap();           // Panics if None
let value = result.unwrap_or(default); // Use default if None
let value = result.expect("message");  // Panics with message if None
let value = result?;                   // Return None early if None
```

### Result Type

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Usage
fn parse_config(s: &str) -> Result<Config, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    Ok(Config::new(s))
}

// Handling Result
match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(e) => println!("Error: {:?}", e),
}

// The ? operator - propagate errors
fn load_and_process() -> Result<Output, Error> {
    let config = parse_config(input)?;  // Returns Err early if error
    let data = load_data(&config)?;
    Ok(process(data))
}
```

### thiserror Crate

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProblemError {
    #[error("invalid configuration size: expected {expected}, got {got}")]
    InvalidConfigSize { expected: usize, got: usize },

    #[error("invalid problem: {0}")]
    InvalidProblem(String),
}
```

### Panic vs Result

```rust
// Use panic for programming errors (bugs)
assert_eq!(weights.len(), num_vertices);  // Panics if false

// Use Result for recoverable errors
fn load_file(path: &str) -> Result<Data, IoError> { ... }
```

**Used in this library**:
- `ProblemError` enum with `thiserror`
- `assert!` and `assert_eq!` for invariant checking
- `Option` for optional weights and metadata

---

## 15. Lifetimes

Lifetimes ensure references are valid for as long as they're used.

### Basic Lifetime Annotation

```rust
// 'a is a lifetime parameter
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// Return value lives as long as both inputs
```

### Lifetime in Structs

```rust
// Struct containing a reference needs lifetime
struct ProblemRef<'a> {
    data: &'a [usize],
}

impl<'a> ProblemRef<'a> {
    fn new(data: &'a [usize]) -> Self {
        Self { data }
    }
}
```

### Static Lifetime

```rust
// 'static means the reference lives for the entire program
const NAME: &'static str = "Independent Set";

// Often written without 'static (it's inferred for string literals)
const NAME: &str = "Independent Set";
```

### Lifetime Elision

Rust infers lifetimes in common cases:

```rust
// These are equivalent:
fn first(s: &str) -> &str { ... }
fn first<'a>(s: &'a str) -> &'a str { ... }

// Rules:
// 1. Each input reference gets its own lifetime
// 2. If one input reference, output gets same lifetime
// 3. If &self or &mut self, output gets self's lifetime
```

**Used in this library**:
- `&'static str` for constant strings in `ProblemInfo`
- `&'static [&'static str]` for aliases arrays
- Mostly elided (automatic) in method signatures

---

## 16. Const and Static

### Const

Compile-time constants, inlined everywhere they're used.

```rust
impl GraphConstraint for IndependentSetConstraint {
    const NAME: &'static str = "Independent Set";
    const ENERGY_MODE: EnergyMode = EnergyMode::LargerSizeIsBetter;
}

// In traits - associated constants
pub trait GraphConstraint {
    const NAME: &'static str;  // Must be provided by implementor
    const ALIASES: &'static [&'static str] = &[];  // Default value
}
```

### Const Fn

Functions that can be evaluated at compile time.

```rust
impl ProblemInfo {
    // Can be called in const context
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            complexity_class: ComplexityClass::NpComplete,
            // ...
        }
    }

    pub const fn with_complexity(mut self, class: ComplexityClass) -> Self {
        self.complexity_class = class;
        self
    }
}

// Can create ProblemInfo at compile time
const MY_INFO: ProblemInfo = ProblemInfo::new("My Problem", "Description")
    .with_complexity(ComplexityClass::NpComplete);
```

### Static

Global variables with a fixed memory address.

```rust
// Mutable static requires unsafe
static mut COUNTER: usize = 0;

// Prefer const or lazy_static for most cases
```

**Used in this library**:
- Associated `const` in `GraphConstraint` trait
- `const fn` builder methods in `ProblemInfo`
- `&'static str` for string constants

---

## 17. Marker Traits

Marker traits indicate properties without providing methods.

### Send and Sync

```rust
// Send: Safe to transfer ownership between threads
// Sync: Safe to share references between threads (&T is Send)

pub trait GraphConstraint: Clone + Send + Sync + 'static {
    // Implementations must be thread-safe
}
```

### Sized

```rust
// Most types are Sized (known size at compile time)
// ?Sized allows dynamically-sized types (like trait objects)
fn process<T: ?Sized>(value: &T) { ... }
```

### Copy

```rust
#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

// Copy types are implicitly copied (not moved)
let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // p1 is copied, both are valid
```

**Used in this library**:
- `GraphConstraint: Send + Sync + 'static` for thread safety
- Small enums derive `Copy` (e.g., `ComplexityClass`, `EnergyMode`)

---

## 18. Builder Pattern

A pattern for constructing complex objects step by step.

### Basic Builder

```rust
pub struct ProblemInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub complexity_class: ComplexityClass,
    pub aliases: &'static [&'static str],
}

impl ProblemInfo {
    // Start with required fields
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            complexity_class: ComplexityClass::Unknown,
            aliases: &[],
        }
    }

    // Builder methods return Self for chaining
    pub const fn with_complexity(mut self, class: ComplexityClass) -> Self {
        self.complexity_class = class;
        self
    }

    pub const fn with_aliases(mut self, aliases: &'static [&'static str]) -> Self {
        self.aliases = aliases;
        self
    }
}

// Usage - method chaining
let info = ProblemInfo::new("My Problem", "Description")
    .with_complexity(ComplexityClass::NpComplete)
    .with_aliases(&["MP", "MyProb"]);
```

**Used in this library**:
- `ProblemInfo` builder for metadata
- `GraphTestCase` builder for test cases

---

## 19. Serde Serialization

Serde provides automatic serialization/deserialization.

### Basic Usage

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Problem {
    name: String,
    size: usize,
}

// Serialize to JSON
let json = serde_json::to_string(&problem)?;

// Deserialize from JSON
let problem: Problem = serde_json::from_str(&json)?;
```

### Field Attributes

```rust
#[derive(Serialize, Deserialize)]
pub struct GraphProblem<C, W> {
    weights: Vec<W>,

    #[serde(skip)]  // Don't serialize this field
    _constraint: PhantomData<C>,

    #[serde(default)]  // Use Default if missing
    optional_field: Option<String>,

    #[serde(rename = "numVertices")]  // Different name in JSON
    num_vertices: usize,
}
```

### Custom Serialization

```rust
use serde::{Serializer, Deserializer};

#[derive(Serialize, Deserialize)]
pub struct MyType {
    #[serde(serialize_with = "serialize_special")]
    special_field: SpecialType,
}

fn serialize_special<S>(value: &SpecialType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Custom serialization logic
    serializer.serialize_str(&value.to_string())
}
```

**Used in this library**:
- Problem types derive `Serialize, Deserialize`
- `#[serde(skip)]` for `PhantomData` fields
- Category enums are serializable

---

## 20. Common Standard Library Types

### Vec

Dynamic array.

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);

// Macro shorthand
let v = vec![1, 2, 3];

// Access
let first = v[0];
let maybe = v.get(0);  // Returns Option<&T>

// Iterate
for x in &v { ... }
for x in v.iter() { ... }
for x in v.into_iter() { ... }  // Consumes v
```

### HashMap

Key-value store.

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", 42);

let value = map.get("key");  // Option<&V>
let value = map["key"];      // Panics if missing

for (key, value) in &map { ... }
```

### Range

```rust
// Range types
0..5      // 0, 1, 2, 3, 4 (exclusive end)
0..=5     // 0, 1, 2, 3, 4, 5 (inclusive end)
..5       // RangeTo
5..       // RangeFrom
..        // RangeFull

// Usage
for i in 0..5 { ... }
let slice = &array[1..3];
```

### String vs &str

```rust
// &str - string slice, borrowed, immutable
let s: &str = "hello";

// String - owned, growable
let s: String = String::from("hello");
let s: String = "hello".to_string();

// Convert
let slice: &str = &owned_string;
let owned: String = slice.to_string();
```

### Box

Heap allocation for single values.

```rust
// Allocate on heap
let boxed: Box<i32> = Box::new(5);

// Useful for recursive types
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

---

## Summary: Most Important Features

For extending this library, focus on these features:

1. **Traits** - Define shared behavior (`Problem`, `GraphConstraint`)
2. **Generics** - Write flexible code (`GraphProblem<C, W>`)
3. **Associated Types** - Type placeholders in traits (`Problem::Size`)
4. **Trait Bounds** - Constrain generic types (`W: Clone + Default`)
5. **PhantomData** - Carry unused type parameters
6. **Type Aliases** - Convenient names (`IndependentSetT<W>`)
7. **Enums** - Multiple variants (`EnergyMode`, `ProblemCategory`)
8. **Derive Macros** - Auto-implement traits (`#[derive(Debug, Clone)]`)
9. **Declarative Macros** - Code generation (`graph_problem_tests!`)
10. **Iterators** - Process collections functionally

## Further Reading

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [std documentation](https://doc.rust-lang.org/std/)
