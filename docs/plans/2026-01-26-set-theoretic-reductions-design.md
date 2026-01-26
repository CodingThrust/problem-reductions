# Set-Theoretic Reduction Path Finding

## Overview

This design addresses issues #10 and #11 by introducing:

1. **Parametric Problem Modeling** - Problems carry graph type and weight type as parameters
2. **Set-Theoretic Path Finding** - Reduction rules apply when source ⊆ rule source AND rule target ⊆ target
3. **Automatic Registration** - Using `inventory` crate for distributed reduction registration
4. **Polynomial Overhead** - Output size expressed as polynomials over named input variables
5. **Customizable Cost Functions** - User-defined optimization goals for path finding

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Subsumption model | Graph topology + weight types combined | Most rigorous, catches invalid paths |
| Graph hierarchy | Trait markers with inventory registration | Compile-time safety + runtime queries |
| Weight hierarchy | Standard `From` trait | Leverages existing Rust ecosystem |
| Registration | `inventory` crate | Simple, works across crate boundaries |
| Overhead model | Polynomial with integer exponents | Sufficient for v1, captures most cases |
| Cost optimization | User-provided cost function trait | Different solvers have different needs |

---

## 1. Type Hierarchy

### 1.1 Graph Type Markers

```rust
/// Marker trait for graph types
pub trait GraphMarker: 'static {
    const NAME: &'static str;
}

/// Compile-time subtype relationship
pub trait GraphSubtype<G: GraphMarker>: GraphMarker {}

// Concrete graph types
pub struct SimpleGraph;
pub struct PlanarGraph;
pub struct UnitDiskGraph;
pub struct BipartiteGraph;

impl GraphMarker for SimpleGraph { const NAME: &'static str = "SimpleGraph"; }
impl GraphMarker for PlanarGraph { const NAME: &'static str = "PlanarGraph"; }
impl GraphMarker for UnitDiskGraph { const NAME: &'static str = "UnitDiskGraph"; }
impl GraphMarker for BipartiteGraph { const NAME: &'static str = "BipartiteGraph"; }
```

### 1.2 Graph Subtype Registration

```rust
pub struct GraphSubtypeEntry {
    pub subtype: &'static str,
    pub supertype: &'static str,
}

inventory::collect!(GraphSubtypeEntry);

/// Declare both trait impl and runtime registration
macro_rules! declare_graph_subtype {
    ($sub:ty => $sup:ty) => {
        impl GraphSubtype<$sup> for $sub {}
        inventory::submit!(GraphSubtypeEntry {
            subtype: <$sub as GraphMarker>::NAME,
            supertype: <$sup as GraphMarker>::NAME,
        });
    };
}

// Hierarchy declarations
declare_graph_subtype!(UnitDiskGraph => PlanarGraph);
declare_graph_subtype!(UnitDiskGraph => SimpleGraph);
declare_graph_subtype!(PlanarGraph => SimpleGraph);
declare_graph_subtype!(BipartiteGraph => SimpleGraph);
```

### 1.3 Weight Types

```rust
/// Marker for numeric weight types
pub trait NumericWeight: Clone + 'static {}

impl NumericWeight for bool {}
impl NumericWeight for i32 {}
impl NumericWeight for i64 {}
impl NumericWeight for f32 {}
impl NumericWeight for f64 {}

// Weight subsumption uses standard From trait:
// i32 → f64 valid (From<i32> for f64 exists)
// f64 → i32 invalid (no From impl)
```

---

## 2. Parametric Problem Modeling

### 2.1 Problem Definition

```rust
pub trait Problem {
    type GraphType: GraphMarker;
    type Weight: NumericWeight;
    type Size: Ord;

    const NAME: &'static str;

    fn problem_size(&self) -> ProblemSize;
    fn num_variables(&self) -> usize;
    fn num_flavors(&self) -> usize;
    fn energy_mode(&self) -> EnergyMode;
    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size>;
}
```

### 2.2 Problem Types with Parameters

```rust
/// Independent Set with graph type and weight type
pub struct IndependentSet<G: GraphMarker, W: NumericWeight> {
    graph: SimpleWeightedGraph<W>,
    _phantom: PhantomData<G>,
}

impl<G: GraphMarker, W: NumericWeight> Problem for IndependentSet<G, W> {
    type GraphType = G;
    type Weight = W;
    type Size = W;

    const NAME: &'static str = "IndependentSet";

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new()
            .with("n", self.graph.num_vertices())
            .with("m", self.graph.num_edges())
    }
}

/// SpinGlass with graph type and weight type
pub struct SpinGlass<G: GraphMarker, W: NumericWeight> {
    couplings: Vec<(usize, usize, W)>,
    fields: Vec<W>,
    _phantom: PhantomData<G>,
}

/// Satisfiability (no graph type, just weight)
pub struct Satisfiability<W: NumericWeight> {
    clauses: Vec<Clause>,
    num_variables: usize,
    _phantom: PhantomData<W>,
}

impl<W: NumericWeight> Problem for Satisfiability<W> {
    type GraphType = SimpleGraph;  // Default for non-graph problems
    type Weight = W;

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new()
            .with("v", self.num_variables)
            .with("c", self.clauses.len())
    }
}
```

### 2.3 ProblemSize with Named Fields

```rust
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ProblemSize {
    fields: HashMap<&'static str, usize>,
}

impl ProblemSize {
    pub fn new() -> Self {
        Self { fields: HashMap::new() }
    }

    pub fn with(mut self, name: &'static str, value: usize) -> Self {
        self.fields.insert(name, value);
        self
    }

    pub fn get(&self, name: &str) -> usize {
        self.fields.get(name).copied().unwrap_or(0)
    }
}
```

---

## 3. Polynomial Overhead Model

### 3.1 Polynomial Representation

```rust
/// A monomial: coefficient * product of (variable^exponent)
#[derive(Clone, Debug)]
pub struct Monomial {
    pub coefficient: f64,
    pub variables: Vec<(&'static str, u8)>,  // (name, exponent)
}

/// A polynomial: sum of monomials
#[derive(Clone, Debug)]
pub struct Polynomial {
    pub terms: Vec<Monomial>,
}

impl Polynomial {
    pub fn constant(c: f64) -> Self {
        Self { terms: vec![Monomial { coefficient: c, variables: vec![] }] }
    }

    pub fn var(name: &'static str) -> Self {
        Self { terms: vec![Monomial { coefficient: 1.0, variables: vec![(name, 1)] }] }
    }

    pub fn var_pow(name: &'static str, exp: u8) -> Self {
        Self { terms: vec![Monomial { coefficient: 1.0, variables: vec![(name, exp)] }] }
    }

    pub fn scale(mut self, c: f64) -> Self {
        for term in &mut self.terms {
            term.coefficient *= c;
        }
        self
    }

    pub fn add(mut self, other: Self) -> Self {
        self.terms.extend(other.terms);
        self
    }

    pub fn evaluate(&self, size: &ProblemSize) -> f64 {
        self.terms.iter().map(|mono| {
            let var_product: f64 = mono.variables.iter()
                .map(|(name, exp)| (size.get(name) as f64).powi(*exp as i32))
                .product();
            mono.coefficient * var_product
        }).sum()
    }
}
```

### 3.2 Polynomial Macro

```rust
macro_rules! poly {
    // Single variable: poly!(n)
    ($name:ident) => {
        Polynomial::var(stringify!($name))
    };
    // Variable with exponent: poly!(n^2)
    ($name:ident ^ $exp:literal) => {
        Polynomial::var_pow(stringify!($name), $exp)
    };
    // Scaled: poly!(3 * n)
    ($c:literal * $name:ident) => {
        Polynomial::var(stringify!($name)).scale($c as f64)
    };
    // Scaled with exponent: poly!(9 * n^2)
    ($c:literal * $name:ident ^ $exp:literal) => {
        Polynomial::var_pow(stringify!($name), $exp).scale($c as f64)
    };
    // Addition: poly!(n + m)
    ($a:ident + $($rest:tt)+) => {
        poly!($a).add(poly!($($rest)+))
    };
    // Scaled addition: poly!(3 * n + 9 * m^2)
    ($c:literal * $a:ident + $($rest:tt)+) => {
        poly!($c * $a).add(poly!($($rest)+))
    };
    ($c:literal * $a:ident ^ $e:literal + $($rest:tt)+) => {
        poly!($c * $a ^ $e).add(poly!($($rest)+))
    };
}
```

### 3.3 Reduction Overhead

```rust
#[derive(Clone, Debug)]
pub struct ReductionOverhead {
    /// Output size as polynomials of input size variables
    pub output_size: Vec<(&'static str, Polynomial)>,
}

impl ReductionOverhead {
    pub fn evaluate_output_size(&self, input: &ProblemSize) -> ProblemSize {
        let mut output = ProblemSize::new();
        for (name, poly) in &self.output_size {
            output.fields.insert(name, poly.evaluate(input) as usize);
        }
        output
    }
}
```

---

## 4. Reduction Registration

### 4.1 ReductionEntry

```rust
pub struct ReductionEntry {
    pub source_name: &'static str,
    pub target_name: &'static str,
    pub source_graph: &'static str,
    pub target_graph: &'static str,
    pub overhead: ReductionOverhead,
}

inventory::collect!(ReductionEntry);
```

### 4.2 Registration Macro

```rust
macro_rules! register_reduction {
    (
        $source:ty => $target:ty,
        output: { $($out_name:ident : $out_poly:expr),* $(,)? }
    ) => {
        inventory::submit! {
            ReductionEntry {
                source_name: <$source as Problem>::NAME,
                target_name: <$target as Problem>::NAME,
                source_graph: <<$source as Problem>::GraphType as GraphMarker>::NAME,
                target_graph: <<$target as Problem>::GraphType as GraphMarker>::NAME,
                overhead: ReductionOverhead {
                    output_size: vec![
                        $((stringify!($out_name), $out_poly)),*
                    ],
                },
            }
        }
    };
}
```

### 4.3 Usage Examples

```rust
// MaxCut → SpinGlass (1:1 mapping)
impl ReduceTo<SpinGlass<SimpleGraph, i32>> for MaxCut<SimpleGraph, i32> {
    // ... implementation
}

register_reduction!(
    MaxCut<SimpleGraph, i32> => SpinGlass<SimpleGraph, i32>,
    output: {
        n: poly!(n),
        m: poly!(m),
    }
);

// SAT → IndependentSet (blowup)
impl ReduceTo<IndependentSet<SimpleGraph, i32>> for Satisfiability<i32> {
    // ... implementation
}

register_reduction!(
    Satisfiability<i32> => IndependentSet<SimpleGraph, i32>,
    output: {
        n: poly!(3 * c),              // 3 vertices per clause
        m: poly!(c + 9 * c^2),        // intra + inter clause edges
    }
);
```

---

## 5. ReductionGraph

### 5.1 Graph Structure

```rust
use petgraph::graph::DiGraph;
use std::collections::{HashMap, HashSet};

pub struct ReductionGraph {
    /// Problem name → node index
    nodes: HashMap<&'static str, NodeIndex>,

    /// Directed graph of reductions
    graph: DiGraph<&'static str, ReductionEdge>,

    /// Graph subtype hierarchy (transitive closure)
    graph_hierarchy: HashMap<&'static str, HashSet<&'static str>>,
}

pub struct ReductionEdge {
    pub source_graph: &'static str,
    pub target_graph: &'static str,
    pub overhead: ReductionOverhead,
}
```

### 5.2 Initialization

```rust
impl ReductionGraph {
    pub fn new() -> Self {
        let mut rg = Self {
            nodes: HashMap::new(),
            graph: DiGraph::new(),
            graph_hierarchy: Self::build_graph_hierarchy(),
        };

        // Collect all registered reductions
        for entry in inventory::iter::<ReductionEntry> {
            rg.add_reduction(entry);
        }

        rg
    }

    fn build_graph_hierarchy() -> HashMap<&'static str, HashSet<&'static str>> {
        let mut supertypes: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        // Direct relationships from inventory
        for entry in inventory::iter::<GraphSubtypeEntry> {
            supertypes.entry(entry.subtype)
                .or_default()
                .insert(entry.supertype);
        }

        // Compute transitive closure
        loop {
            let mut changed = false;
            let types: Vec<_> = supertypes.keys().copied().collect();

            for sub in &types {
                let current_supers: Vec<_> = supertypes.get(sub)
                    .map(|s| s.iter().copied().collect())
                    .unwrap_or_default();

                for sup in current_supers {
                    if let Some(sup_supers) = supertypes.get(sup).cloned() {
                        let entry = supertypes.entry(sub).or_default();
                        for ss in sup_supers {
                            if entry.insert(ss) {
                                changed = true;
                            }
                        }
                    }
                }
            }

            if !changed { break; }
        }

        supertypes
    }

    pub fn is_graph_subtype(&self, sub: &str, sup: &str) -> bool {
        sub == sup || self.graph_hierarchy
            .get(sub)
            .map(|s| s.contains(sup))
            .unwrap_or(false)
    }
}
```

### 5.3 Set-Theoretic Path Validation

```rust
impl ReductionGraph {
    /// Check if reduction rule C→D can be used for A→B
    /// Requires: A ⊆ C (source) and D ⊆ B (target)
    pub fn rule_applicable(
        &self,
        want_source_graph: &str,  // A's graph type
        want_target_graph: &str,  // B's graph type
        rule_source_graph: &str,  // C's graph type
        rule_target_graph: &str,  // D's graph type
    ) -> bool {
        // A's graph must be subtype of C's graph (input constraint)
        let source_ok = self.is_graph_subtype(want_source_graph, rule_source_graph);

        // D's graph must be subtype of B's graph (output acceptable)
        let target_ok = self.is_graph_subtype(rule_target_graph, want_target_graph);

        source_ok && target_ok
    }
}
```

---

## 6. Cost Functions

### 6.1 PathCostFn Trait

```rust
pub trait PathCostFn {
    fn edge_cost(&self, edge: &ReductionEdge, current_size: &ProblemSize) -> f64;
}
```

### 6.2 Built-in Cost Functions

```rust
/// Minimize a single output field
pub struct Minimize(pub &'static str);

impl PathCostFn for Minimize {
    fn edge_cost(&self, edge: &ReductionEdge, size: &ProblemSize) -> f64 {
        edge.overhead.evaluate_output_size(size).get(self.0) as f64
    }
}

/// Minimize weighted sum of output fields
pub struct MinimizeWeighted(pub &'static [(&'static str, f64)]);

impl PathCostFn for MinimizeWeighted {
    fn edge_cost(&self, edge: &ReductionEdge, size: &ProblemSize) -> f64 {
        let output = edge.overhead.evaluate_output_size(size);
        self.0.iter()
            .map(|(field, weight)| weight * output.get(field) as f64)
            .sum()
    }
}

/// Minimize the maximum of specified fields
pub struct MinimizeMax(pub &'static [&'static str]);

impl PathCostFn for MinimizeMax {
    fn edge_cost(&self, edge: &ReductionEdge, size: &ProblemSize) -> f64 {
        let output = edge.overhead.evaluate_output_size(size);
        self.0.iter()
            .map(|field| output.get(field) as f64)
            .fold(0.0, f64::max)
    }
}

/// Lexicographic: minimize first field, break ties with subsequent
pub struct MinimizeLexicographic(pub &'static [&'static str]);

impl PathCostFn for MinimizeLexicographic {
    fn edge_cost(&self, edge: &ReductionEdge, size: &ProblemSize) -> f64 {
        let output = edge.overhead.evaluate_output_size(size);
        let mut cost = 0.0;
        let mut scale = 1.0;
        for field in self.0 {
            cost += scale * output.get(field) as f64;
            scale *= 1e-10;
        }
        cost
    }
}

/// Minimize number of reduction steps
pub struct MinimizeSteps;

impl PathCostFn for MinimizeSteps {
    fn edge_cost(&self, _edge: &ReductionEdge, _size: &ProblemSize) -> f64 {
        1.0
    }
}

/// Custom cost function from closure
pub struct CustomCost<F>(pub F);

impl<F: Fn(&ReductionEdge, &ProblemSize) -> f64> PathCostFn for CustomCost<F> {
    fn edge_cost(&self, edge: &ReductionEdge, size: &ProblemSize) -> f64 {
        (self.0)(edge, size)
    }
}
```

---

## 7. Path Finding

### 7.1 Dijkstra with Custom Cost

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;

impl ReductionGraph {
    pub fn find_cheapest_path<C: PathCostFn>(
        &self,
        source: (&str, &str),      // (problem_name, graph_type)
        target: (&str, &str),
        input_size: &ProblemSize,
        cost_fn: &C,
    ) -> Option<ReductionPath> {
        let mut costs: HashMap<&str, f64> = HashMap::new();
        let mut sizes: HashMap<&str, ProblemSize> = HashMap::new();
        let mut prev: HashMap<&str, (&str, EdgeIndex)> = HashMap::new();
        let mut heap = BinaryHeap::new();

        costs.insert(source.0, 0.0);
        sizes.insert(source.0, input_size.clone());
        heap.push(Reverse((OrderedFloat(0.0), source.0)));

        while let Some(Reverse((cost, node))) = heap.pop() {
            if node == target.0 {
                return Some(self.reconstruct_path(&prev, source.0, target.0));
            }

            if cost.0 > *costs.get(node).unwrap_or(&f64::INFINITY) {
                continue;
            }

            let current_size = sizes.get(node).unwrap();

            for edge_idx in self.graph.edges_from(self.nodes[node]) {
                let edge = &self.graph[edge_idx];
                let next_node = self.graph.edge_target(edge_idx);

                // Set-theoretic validation
                if !self.rule_applicable(
                    source.1, target.1,
                    edge.source_graph, edge.target_graph,
                ) {
                    continue;
                }

                let edge_cost = cost_fn.edge_cost(edge, current_size);
                let new_cost = cost.0 + edge_cost;
                let new_size = edge.overhead.evaluate_output_size(current_size);

                if new_cost < *costs.get(next_node).unwrap_or(&f64::INFINITY) {
                    costs.insert(next_node, new_cost);
                    sizes.insert(next_node, new_size);
                    prev.insert(next_node, (node, edge_idx));
                    heap.push(Reverse((OrderedFloat(new_cost), next_node)));
                }
            }
        }

        None
    }
}
```

### 7.2 Convenience Methods

```rust
impl ReductionGraph {
    pub fn find_path_minimizing(
        &self,
        source: (&str, &str),
        target: (&str, &str),
        input_size: &ProblemSize,
        field: &'static str,
    ) -> Option<ReductionPath> {
        self.find_cheapest_path(source, target, input_size, &Minimize(field))
    }

    pub fn find_shortest_path(
        &self,
        source: (&str, &str),
        target: (&str, &str),
        input_size: &ProblemSize,
    ) -> Option<ReductionPath> {
        self.find_cheapest_path(source, target, input_size, &MinimizeSteps)
    }
}
```

---

## 8. Type Conversion and Execution

### 8.1 Weight Conversion

```rust
pub trait ConvertWeights<W: NumericWeight> {
    type Output;
    fn convert_weights(self) -> Self::Output;
}

impl<G: GraphMarker, W1: NumericWeight, W2: NumericWeight + From<W1>>
    ConvertWeights<W2> for IndependentSet<G, W1>
{
    type Output = IndependentSet<G, W2>;

    fn convert_weights(self) -> Self::Output {
        IndependentSet {
            graph: self.graph.map_weights(W2::from),
            _phantom: PhantomData,
        }
    }
}
```

### 8.2 Path Execution

```rust
pub struct ReductionPath {
    pub steps: Vec<ReductionStep>,
    pub total_cost: f64,
}

pub struct ReductionStep {
    pub source: &'static str,
    pub target: &'static str,
    pub source_graph: &'static str,
    pub target_graph: &'static str,
}

/// High-level API for automatic path finding and execution
pub trait ReduceVia<T: Problem>: Problem {
    fn reduce_via(self, graph: &ReductionGraph) -> ComposedReductionResult<Self, T>
    where
        Self::GraphType: GraphSubtype<T::GraphType>,
        T::Weight: From<Self::Weight>;
}
```

### 8.3 Solution Extraction

```rust
pub struct ComposedReductionResult<S, T> {
    steps: Vec<Box<dyn AnyReductionResult>>,
    final_problem: T,
    _phantom: PhantomData<S>,
}

impl<S, T> ComposedReductionResult<S, T> {
    pub fn target_problem(&self) -> &T {
        &self.final_problem
    }

    pub fn extract_solution(&self, target_solution: &[bool]) -> Vec<bool> {
        let mut solution = target_solution.to_vec();
        for step in self.steps.iter().rev() {
            solution = step.extract_solution(&solution);
        }
        solution
    }
}
```

---

## 9. Usage Examples

### 9.1 Basic Reduction

```rust
let graph = ReductionGraph::new();

// Create a problem
let is: IndependentSet<UnitDiskGraph, i32> = create_unit_disk_is();

// Find path to SpinGlass with f64 weights
let path = graph.find_path_minimizing(
    ("IndependentSet", "UnitDiskGraph"),
    ("SpinGlass", "SimpleGraph"),
    &is.problem_size(),
    "n",  // minimize output spins
);

println!("Path: {:?}", path);
```

### 9.2 Custom Cost Function

```rust
// Weighted combination: vertices matter 2x more than edges
let path = graph.find_cheapest_path(
    ("SAT", "SimpleGraph"),
    ("SpinGlass", "SimpleGraph"),
    &sat.problem_size(),
    &MinimizeWeighted(&[("n", 2.0), ("m", 1.0)]),
);
```

### 9.3 Full Pipeline

```rust
let sat: Satisfiability<i32> = create_sat_problem();
let graph = ReductionGraph::new();

// Reduce to SpinGlass
let result = sat.reduce_via::<SpinGlass<SimpleGraph, f64>>(&graph);
let spinglass = result.target_problem();

// Solve SpinGlass
let sg_solution = solve_spinglass(spinglass);

// Extract back to SAT
let sat_solution = result.extract_solution(&sg_solution);
```

---

## 10. Migration Path

### Phase 1: Add New Types (Non-Breaking)
- Add `GraphMarker` trait and graph type markers
- Add `NumericWeight` marker trait
- Keep existing problem types working

### Phase 2: Parametrize Problems
- Update problem structs to take `<G, W>` parameters
- Remove old type aliases
- Update all reduction implementations

### Phase 3: Add Registration System
- Add `inventory` dependency
- Create registration macros
- Add polynomial overhead to all reductions

### Phase 4: Update ReductionGraph
- Implement set-theoretic path finding
- Add cost function support
- Implement path execution with weight conversion

---

## 11. Dependencies

```toml
[dependencies]
inventory = "0.3"
petgraph = "0.6"
ordered-float = "4.0"
```

---

## 12. Open Questions

1. **KSatisfiability<K>**: Should `KSatisfiability<3>` and `KSatisfiability<4>` be different nodes? Current design merges them.

2. **Non-graph problems**: Problems like SAT have no natural graph type. Using `SimpleGraph` as default is a workaround.

3. **Weight conversion losses**: `f64 → i32` is blocked by `From`. Should we support lossy conversions with explicit opt-in?

4. **Cross-crate extensions**: The `inventory` approach works across crates, but users need to ensure their graph subtypes are registered.
