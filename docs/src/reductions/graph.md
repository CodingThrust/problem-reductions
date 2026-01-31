# Reduction Graph

The `ReductionGraph` allows discovering reduction paths between problem types.

## Reduction Diagram

```mermaid
flowchart TD
    subgraph Specialized["Specialized Problems"]
        Factoring
    end

    subgraph Circuit["Circuit Problems"]
        CircuitSAT
    end

    subgraph SAT["Satisfiability Problems"]
        Satisfiability
        KSat["KSatisfiability&lt;3&gt;"]
    end

    subgraph Graph["Graph Problems"]
        IS["IndependentSet"]
        VC["VertexCovering"]
        Coloring
        DS["DominatingSet"]
        Matching
    end

    subgraph Set["Set Problems"]
        SP["SetPacking"]
        SC["SetCovering"]
    end

    subgraph Optimization["Optimization Problems"]
        SG_i32["SpinGlass&lt;i32&gt;"]
        SG_f64["SpinGlass&lt;f64&gt;"]
        QUBO["QUBO&lt;f64&gt;"]
        MaxCut["MaxCut&lt;i32&gt;"]
        ILP["ILP"]
    end

    %% Factoring chain
    Factoring --> CircuitSAT
    Factoring --> ILP
    CircuitSAT --> SG_i32

    %% SAT reductions
    Satisfiability <--> KSat
    Satisfiability --> IS
    Satisfiability --> Coloring
    Satisfiability --> DS

    %% Graph problem reductions
    IS <--> VC
    IS <--> SP
    VC --> SC
    Matching --> SP

    %% Optimization reductions
    SG_f64 <--> QUBO
    SG_i32 <--> MaxCut

    %% ILP reductions
    Coloring --> ILP

    %% Styling
    style Factoring fill:#f9f,stroke:#333
    style CircuitSAT fill:#f9f,stroke:#333
    style Satisfiability fill:#bbf,stroke:#333
    style KSat fill:#bbf,stroke:#333
    style IS fill:#bfb,stroke:#333
    style VC fill:#bfb,stroke:#333
    style Coloring fill:#bfb,stroke:#333
    style DS fill:#bfb,stroke:#333
    style Matching fill:#bfb,stroke:#333
    style SP fill:#fbb,stroke:#333
    style SC fill:#fbb,stroke:#333
    style SG_i32 fill:#ff9,stroke:#333
    style SG_f64 fill:#ff9,stroke:#333
    style QUBO fill:#ff9,stroke:#333
    style MaxCut fill:#ff9,stroke:#333
    style ILP fill:#ff9,stroke:#333
```

## Legend

| Color | Category |
|-------|----------|
| Pink | Specialized/Circuit Problems |
| Blue | Satisfiability Problems |
| Green | Graph Problems |
| Red | Set Problems |
| Yellow | Optimization Problems |

Bidirectional arrows (`<-->`) indicate reductions exist in both directions.

## Usage

```rust
use problemreductions::prelude::*;
use problemreductions::rules::ReductionGraph;

let graph = ReductionGraph::new();

// Check if direct reduction exists
let has_direct = graph.has_direct_reduction::<IndependentSet<i32>, VertexCovering<i32>>();

// Find all paths between types
let paths = graph.find_paths::<SetPacking<i32>, VertexCovering<i32>>();

// Find shortest path
let shortest = graph.find_shortest_path::<SetPacking<i32>, VertexCovering<i32>>();

// Get statistics
println!("Types: {}, Reductions: {}", graph.num_types(), graph.num_reductions());
```

## Registered Reductions

| Source | Target | Bidirectional |
|--------|--------|---------------|
| IndependentSet | VertexCovering | Yes |
| IndependentSet | SetPacking | Yes |
| VertexCovering | SetCovering | No |
| Matching | SetPacking | No |
| SpinGlass&lt;f64&gt; | QUBO&lt;f64&gt; | Yes |
| SpinGlass&lt;i32&gt; | MaxCut&lt;i32&gt; | Yes |
| Satisfiability | KSatisfiability&lt;3&gt; | Yes |
| Satisfiability | IndependentSet | No |
| Satisfiability | Coloring | No |
| Satisfiability | DominatingSet | No |
| CircuitSAT | SpinGlass&lt;i32&gt; | No |
| Factoring | CircuitSAT | No |
| Coloring | ILP | No |
| Factoring | ILP | No |

## API

```rust
impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions.
    pub fn new() -> Self;

    /// Check if a direct reduction exists from S to T.
    pub fn has_direct_reduction<S: 'static, T: 'static>(&self) -> bool;

    /// Find all paths from source to target type.
    pub fn find_paths<S: 'static, T: 'static>(&self) -> Vec<ReductionPath>;

    /// Find the shortest path from source to target type.
    pub fn find_shortest_path<S: 'static, T: 'static>(&self) -> Option<ReductionPath>;

    /// Get all registered problem type names.
    pub fn problem_types(&self) -> Vec<&'static str>;

    /// Get the number of registered problem types.
    pub fn num_types(&self) -> usize;

    /// Get the number of registered reductions.
    pub fn num_reductions(&self) -> usize;
}
```
