// Problem Reductions: A Mathematical Reference
#import "reduction-diagram.typ": reduction-graph, graph-data
#import "@preview/cetz:0.4.2": canvas, draw
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show link: set text(blue)

// Set up theorem environments with ctheorems
#show: thmrules.with(qed-symbol: $square$)

#let theorem = thmplain("theorem", "Theorem").with(numbering: none)
#let proof = thmproof("proof", "Proof")
#let definition = thmbox(
  "definition",
  "Definition",
  fill: rgb("#f8f8f8"),
  stroke: (left: 2pt + rgb("#4a86e8")),
  inset: (x: 1em, y: 0.8em),
  base_level: 1,
)

#align(center)[
  #text(size: 16pt, weight: "bold")[Problem Reductions: Models and Transformations]
  #v(0.5em)
  #text(size: 11pt)[Technical Documentation]
  #v(0.3em)
  #text(size: 10pt, style: "italic")[github.com/CodingThrust/problem-reductions]
  #v(1em)
]

#block(width: 100%, inset: (x: 2em, y: 1em))[
  *Abstract.* We present formal definitions for computational problems and polynomial-time reductions implemented in the `problemreductions` library. For each reduction, we state theorems with constructive proofs that preserve solution structure.
]


// Table of contents
#outline(title: "Contents", indent: 1.5em, depth: 2)

#pagebreak()

= Introduction

A _reduction_ from problem $A$ to problem $B$, denoted $A arrow.long B$, is a polynomial-time transformation of $A$-instances into $B$-instances such that: (1) the transformation runs in polynomial time, (2) solutions to $B$ can be efficiently mapped back to solutions of $A$, and (3) optimal solutions are preserved. @fig:reduction-graph shows the #graph-data.edges.len() reductions connecting #graph-data.nodes.len() problem types.

== Notation

We use the following notation throughout. An _undirected graph_ $G = (V, E)$ consists of a vertex set $V$ and edge set $E subset.eq binom(V, 2)$. For a set $S$, $overline(S)$ or $V backslash S$ denotes its complement. We write $|S|$ for cardinality. For Boolean variables, $overline(x)$ denotes negation ($not x$). A _literal_ is a variable $x$ or its negation $overline(x)$. A _clause_ is a disjunction of literals. A formula in _conjunctive normal form_ (CNF) is a conjunction of clauses. We abbreviate Independent Set as IS, Vertex Cover as VC, and use $n$ for problem size, $m$ for number of clauses, and $k_j = |C_j|$ for clause size.

#figure(
  reduction-graph(width: 18mm, height: 14mm),
  caption: [Reduction graph. Colors: green (graph), red (set), yellow (optimization), blue (satisfiability), pink (specialized).]
) <fig:reduction-graph>

= Problem Definitions <sec:problems>

== Graph Problems

In all graph problems below, $G = (V, E)$ denotes an undirected graph with $|V| = n$ vertices and $|E|$ edges.

#definition("Independent Set (IS)")[
  Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing $sum_(v in S) w(v)$ such that no two vertices in $S$ are adjacent: $forall u, v in S: (u, v) in.not E$.

  _Reduces to:_ Set Packing (@def:set-packing), QUBO (@def:qubo).

  _Reduces from:_ Vertex Cover (@def:vertex-cover), SAT (@def:satisfiability), Set Packing (@def:set-packing).

  ```rust
  pub struct IndependentSet<W = i32> {
      graph: UnGraph<(), ()>,  // The underlying graph
      weights: Vec<W>,         // Weights for each vertex
  }
  ```
] <def:independent-set>

#definition("Vertex Cover (VC)")[
  Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ such that every edge has at least one endpoint in $S$: $forall (u, v) in E: u in S or v in S$.

  _Reduces to:_ Independent Set (@def:independent-set), Set Covering (@def:set-covering), QUBO (@def:qubo).

  _Reduces from:_ Independent Set (@def:independent-set).

  ```rust
  pub struct VertexCovering<W = i32> {
      graph: UnGraph<(), ()>,  // The underlying graph
      weights: Vec<W>,         // Weights for each vertex
  }
  ```
] <def:vertex-cover>

#definition("Max-Cut")[
  Given $G = (V, E)$ with weights $w: E -> RR$, find partition $(S, overline(S))$ maximizing $sum_((u,v) in E: u in S, v in overline(S)) w(u, v)$.

  _Reduces to:_ Spin Glass (@def:spin-glass).

  _Reduces from:_ Spin Glass (@def:spin-glass).

  ```rust
  pub struct MaxCut<W = i32> {
      graph: UnGraph<(), W>,  // Weighted graph (edge weights)
  }
  ```
] <def:max-cut>

#definition("Graph Coloring")[
  Given $G = (V, E)$ and $k$ colors, find $c: V -> {1, ..., k}$ minimizing $|{(u, v) in E : c(u) = c(v)}|$.

  _Reduces to:_ ILP (@def:ilp), QUBO (@def:qubo).

  _Reduces from:_ SAT (@def:satisfiability).

  ```rust
  pub struct Coloring {
      num_colors: usize,       // Number of available colors (K)
      graph: UnGraph<(), ()>,  // The underlying graph
  }
  ```
] <def:coloring>

#definition("Dominating Set")[
  Given $G = (V, E)$ with weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ s.t. $forall v in V: v in S or exists u in S: (u, v) in E$.

  _Reduces from:_ SAT (@def:satisfiability).

  ```rust
  pub struct DominatingSet<W = i32> {
      graph: UnGraph<(), ()>,  // The underlying graph
      weights: Vec<W>,         // Weights for each vertex
  }
  ```
] <def:dominating-set>

#definition("Matching")[
  Given $G = (V, E)$ with weights $w: E -> RR$, find $M subset.eq E$ maximizing $sum_(e in M) w(e)$ s.t. $forall e_1, e_2 in M: e_1 inter e_2 = emptyset$.

  _Reduces to:_ Set Packing (@def:set-packing).

  ```rust
  pub struct Matching<W = i32> {
      num_vertices: usize,     // Number of vertices
      graph: UnGraph<(), W>,   // Weighted graph
      edge_weights: Vec<W>,    // Weights for each edge
  }
  ```
] <def:matching>

#definition("Unit Disk Graph (Grid Graph)")[
  A graph $G = (V, E)$ where vertices $V$ are points on a 2D lattice and $(u, v) in E$ iff the Euclidean distance $d(u, v) <= r$ for some radius $r$. A _King's subgraph_ uses the King's graph lattice (8-connectivity square grid) with $r approx 1.5$.
]

== Set Problems

#definition("Set Packing")[
  Given universe $U$, collection $cal(S) = {S_1, ..., S_m}$ with $S_i subset.eq U$, weights $w: cal(S) -> RR$, find $cal(P) subset.eq cal(S)$ maximizing $sum_(S in cal(P)) w(S)$ s.t. $forall S_i, S_j in cal(P): S_i inter S_j = emptyset$.

  _Reduces to:_ Independent Set (@def:independent-set), QUBO (@def:qubo).

  _Reduces from:_ Independent Set (@def:independent-set), Matching (@def:matching).

  ```rust
  pub struct SetPacking<W = i32> {
      sets: Vec<Vec<usize>>,  // Collection of sets
      weights: Vec<W>,        // Weights for each set
  }
  ```
] <def:set-packing>

#definition("Set Covering")[
  Given universe $U$, collection $cal(S)$ with weights $w: cal(S) -> RR$, find $cal(C) subset.eq cal(S)$ minimizing $sum_(S in cal(C)) w(S)$ s.t. $union.big_(S in cal(C)) S = U$.

  _Reduces from:_ Vertex Cover (@def:vertex-cover).

  ```rust
  pub struct SetCovering<W = i32> {
      universe_size: usize,   // Size of the universe
      sets: Vec<Vec<usize>>,  // Collection of sets
      weights: Vec<W>,        // Weights for each set
  }
  ```
] <def:set-covering>

== Optimization Problems

#definition("Spin Glass (Ising Model)")[
  Given $n$ spin variables $s_i in {-1, +1}$, pairwise couplings $J_(i j) in RR$, and external fields $h_i in RR$, minimize the Hamiltonian (energy function): $H(bold(s)) = -sum_((i,j)) J_(i j) s_i s_j - sum_i h_i s_i$.

  _Reduces to:_ Max-Cut (@def:max-cut), QUBO (@def:qubo).

  _Reduces from:_ Circuit-SAT (@def:circuit-sat), Max-Cut (@def:max-cut), QUBO (@def:qubo).

  ```rust
  pub struct SpinGlass<W = f64> {
      num_spins: usize,                    // Number of spins
      interactions: Vec<((usize, usize), W)>,  // J_ij couplings
      fields: Vec<W>,                      // h_i on-site fields
  }
  ```
] <def:spin-glass>

#definition("QUBO")[
  Given $n$ binary variables $x_i in {0, 1}$, upper-triangular matrix $Q in RR^(n times n)$, minimize $f(bold(x)) = sum_(i=1)^n Q_(i i) x_i + sum_(i < j) Q_(i j) x_i x_j$ (using $x_i^2 = x_i$ for binary variables).

  _Reduces to:_ Spin Glass (@def:spin-glass).

  _Reduces from:_ Spin Glass (@def:spin-glass), Independent Set (@def:independent-set), Vertex Cover (@def:vertex-cover), Graph Coloring (@def:coloring), Set Packing (@def:set-packing), $k$-SAT (@def:k-sat), ILP (@def:ilp).

  ```rust
  pub struct QUBO<W = f64> {
      num_vars: usize,         // Number of variables
      matrix: Vec<Vec<W>>,     // Q matrix (upper triangular)
  }
  ```
] <def:qubo>

#definition("Integer Linear Programming (ILP)")[
  Given $n$ integer variables $bold(x) in ZZ^n$, constraint matrix $A in RR^(m times n)$, bounds $bold(b) in RR^m$, and objective $bold(c) in RR^n$, find $bold(x)$ minimizing $bold(c)^top bold(x)$ subject to $A bold(x) <= bold(b)$ and variable bounds.

  _Reduces to:_ QUBO (@def:qubo).

  _Reduces from:_ Graph Coloring (@def:coloring), Factoring (@def:factoring).

  ```rust
  pub struct ILP {
      num_vars: usize,                  // Number of variables
      bounds: Vec<VarBounds>,           // Bounds per variable
      constraints: Vec<LinearConstraint>, // Linear constraints
      objective: Vec<(usize, f64)>,     // Sparse objective
      sense: ObjectiveSense,            // Maximize or Minimize
  }

  pub struct VarBounds { lower: Option<i64>, upper: Option<i64> }
  pub struct LinearConstraint {
      terms: Vec<(usize, f64)>,  // (var_index, coefficient)
      cmp: Comparison,           // Le, Ge, or Eq
      rhs: f64,
  }
  ```
] <def:ilp>

== Satisfiability Problems

#definition("SAT")[
  Given a CNF formula $phi = and.big_(j=1)^m C_j$ with $m$ clauses over $n$ Boolean variables, where each clause $C_j = or.big_i ell_(j i)$ is a disjunction of literals, find an assignment $bold(x) in {0, 1}^n$ such that $phi(bold(x)) = 1$ (all clauses satisfied).

  _Reduces to:_ Independent Set (@def:independent-set), Graph Coloring (@def:coloring), Dominating Set (@def:dominating-set), $k$-SAT (@def:k-sat).

  _Reduces from:_ $k$-SAT (@def:k-sat).

  ```rust
  pub struct Satisfiability<W = i32> {
      num_vars: usize,           // Number of variables
      clauses: Vec<CNFClause>,   // Clauses in CNF
      weights: Vec<W>,           // Weights per clause (MAX-SAT)
  }

  pub struct CNFClause {
      literals: Vec<i32>,  // Signed: +i for x_i, -i for NOT x_i
  }
  ```
] <def:satisfiability>

#definition([$k$-SAT])[
  SAT with exactly $k$ literals per clause.

  _Reduces to:_ SAT (@def:satisfiability), QUBO (@def:qubo).

  _Reduces from:_ SAT (@def:satisfiability).

  ```rust
  pub struct KSatisfiability<const K: usize, W = i32> {
      num_vars: usize,           // Number of variables
      clauses: Vec<CNFClause>,   // Each clause has exactly K literals
      weights: Vec<W>,           // Weights per clause
  }
  ```
] <def:k-sat>

#definition("Circuit-SAT")[
  Given a Boolean circuit $C$ composed of logic gates (AND, OR, NOT, XOR) with $n$ input variables, find an input assignment $bold(x) in {0,1}^n$ such that $C(bold(x)) = 1$.

  _Reduces to:_ Spin Glass (@def:spin-glass).

  _Reduces from:_ Factoring (@def:factoring).

  ```rust
  pub struct CircuitSAT<W = i32> {
      circuit: Circuit,          // The boolean circuit
      variables: Vec<String>,    // Variable names in order
      weights: Vec<W>,           // Weights per assignment
  }
  ```
] <def:circuit-sat>

#definition("Factoring")[
  Given a composite integer $N$ and bit sizes $m, n$, find integers $p in [2, 2^m - 1]$ and $q in [2, 2^n - 1]$ such that $p times q = N$. Here $p$ has $m$ bits and $q$ has $n$ bits.

  _Reduces to:_ Circuit-SAT (@def:circuit-sat), ILP (@def:ilp).

  ```rust
  pub struct Factoring {
      m: usize,      // Bits for first factor
      n: usize,      // Bits for second factor
      target: u64,   // The number to factor
  }
  ```
] <def:factoring>

= Reductions <sec:reductions>

== Trivial Reductions

#theorem[
  *(IS $arrow.l.r$ VC)* $S subset.eq V$ is independent iff $V backslash S$ is a vertex cover, with $|"IS"| + |"VC"| = |V|$. [_Problems:_ @def:independent-set, @def:vertex-cover.]
] <thm:is-vc>

#proof[
  ($arrow.r.double$) If $S$ is independent, for any $(u, v) in E$, at most one endpoint lies in $S$, so $V backslash S$ covers all edges. ($arrow.l.double$) If $C$ is a cover, for any $u, v in V backslash C$, $(u, v) in.not E$, so $V backslash C$ is independent.
]

```rust
// Minimal example: IS -> VC -> extract solution
let is_problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
let result = ReduceTo::<VertexCovering<i32>>::reduce_to(&is_problem);
let vc_problem = result.target_problem();

let solver = BruteForce::new();
let vc_solutions = solver.find_best(vc_problem);
let is_solution = result.extract_solution(&vc_solutions[0]);
assert!(is_problem.solution_size(&is_solution).is_valid);
```

#theorem[
  *(IS $arrow.r$ Set Packing)* Construct $U = E$, $S_v = {e in E : v in e}$, $w(S_v) = w(v)$. Then $I$ is independent iff ${S_v : v in I}$ is a packing. [_Problems:_ @def:independent-set, @def:set-packing.]
] <thm:is-to-setpacking>

#proof[
  Independence implies disjoint incident edge sets; conversely, disjoint edge sets imply no shared edges.
]

```rust
// Minimal example: IS -> SetPacking -> extract solution
let is_problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
let result = ReduceTo::<SetPacking<i32>>::reduce_to(&is_problem);
let sp_problem = result.target_problem();

let solver = BruteForce::new();
let sp_solutions = solver.find_best(sp_problem);
let is_solution = result.extract_solution(&sp_solutions[0]);
assert!(is_problem.solution_size(&is_solution).is_valid);
```

#theorem[
  *(VC $arrow.r$ Set Covering)* Construct $U = {0, ..., |E|-1}$, $S_v = {i : e_i "incident to" v}$, $w(S_v) = w(v)$. Then $C$ is a cover iff ${S_v : v in C}$ covers $U$. [_Problems:_ @def:vertex-cover, @def:set-covering.]
] <thm:vc-to-setcovering>

#theorem[
  *(Matching $arrow.r$ Set Packing)* Construct $U = V$, $S_e = {u, v}$ for $e = (u,v)$, $w(S_e) = w(e)$. Then $M$ is a matching iff ${S_e : e in M}$ is a packing. [_Problems:_ @def:matching, @def:set-packing.]
] <thm:matching-to-setpacking>

#theorem[
  *(Spin Glass $arrow.l.r$ QUBO)* The substitution $s_i = 2x_i - 1$ yields $H_"SG"(bold(s)) = H_"QUBO"(bold(x)) + "const"$. [_Problems:_ @def:spin-glass, @def:qubo.]
] <thm:spinglass-qubo>

#proof[
  Expanding $-sum_(i,j) J_(i j) (2x_i - 1)(2x_j - 1) - sum_i h_i (2x_i - 1)$ gives $Q_(i j) = -4J_(i j)$, $Q_(i i) = 2sum_j J_(i j) - 2h_i$.
]

```rust
// Minimal example: SpinGlass -> QUBO -> extract solution
let sg = SpinGlass::new(2, vec![((0, 1), -1.0)], vec![0.5, -0.5]);
let result = ReduceTo::<QUBO>::reduce_to(&sg);
let qubo = result.target_problem();

let solver = BruteForce::new();
let qubo_solutions = solver.find_best(qubo);
let sg_solution = result.extract_solution(&qubo_solutions[0]);
assert_eq!(sg_solution.len(), 2);
```

== Penalty-Method QUBO Reductions <sec:penalty-method>

The _penalty method_ @glover2019 @lucas2014 converts a constrained optimization problem into an unconstrained QUBO by adding quadratic penalty terms. Given an objective $"obj"(bold(x))$ to minimize and constraints $g_k (bold(x)) = 0$, construct:
$ f(bold(x)) = "obj"(bold(x)) + P sum_k g_k (bold(x))^2 $
where $P$ is a penalty weight large enough that any constraint violation costs more than the entire objective range. Since $g_k (bold(x))^2 >= 0$ with equality iff $g_k (bold(x)) = 0$, minimizers of $f$ are feasible and optimal for the original problem. Because binary variables satisfy $x_i^2 = x_i$, the resulting $f$ is a quadratic in $bold(x)$, i.e.\ a QUBO.

#theorem[
  *(IS $arrow.r$ QUBO)* Given $G = (V, E)$ with weights $w$, construct upper-triangular $Q in RR^(n times n)$ with $Q_(i i) = -w_i$ and $Q_(i j) = P$ for $(i,j) in E$ ($i < j$), where $P = 1 + sum_i w_i$. Then minimizing $f(bold(x)) = sum_i Q_(i i) x_i + sum_(i<j) Q_(i j) x_i x_j$ is equivalent to maximizing the IS objective. [_Problems:_ @def:independent-set, @def:qubo.]
] <thm:is-to-qubo>

#proof[
  _Construction._ The IS objective is: maximize $sum_i w_i x_i$ subject to $x_i x_j = 0$ for $(i,j) in E$. Applying the penalty method (@sec:penalty-method):
  $ f(bold(x)) = -sum_i w_i x_i + P sum_((i,j) in E) x_i x_j $
  Reading off the QUBO coefficients: diagonal $Q_(i i) = -w_i$ (linear terms), off-diagonal $Q_(i j) = P$ for edges $i < j$ (quadratic penalty).

  _Correctness._ If $bold(x)$ has any adjacent pair $(x_i = 1, x_j = 1)$ with $(i,j) in E$, the penalty $P > sum_i w_i >= -sum_i Q_(i i) x_i$ exceeds the maximum objective gain, so $bold(x)$ is not a minimizer. Among independent sets ($x_i x_j = 0$ for all edges), $f(bold(x)) = -sum_(i in S) w_i$, minimized exactly when $S$ is a maximum-weight IS.
]

```rust
// Minimal example: IS -> QUBO -> extract solution
let is = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let result = ReduceTo::<QUBO>::reduce_to(&is);
let qubo = result.target_problem();

let solver = BruteForce::new();
let solutions = solver.find_best(qubo);
let is_solution = result.extract_solution(&solutions[0]);
assert!(is.solution_size(&is_solution).is_valid);
```

#theorem[
  *(VC $arrow.r$ QUBO)* Given $G = (V, E)$ with weights $w$, construct upper-triangular $Q$ with $Q_(i i) = w_i - P dot "deg"(i)$ and $Q_(i j) = P$ for $(i,j) in E$ ($i < j$), where $P = 1 + sum_i w_i$ and $"deg"(i)$ is the degree of vertex $i$. [_Problems:_ @def:vertex-cover, @def:qubo.]
] <thm:vc-to-qubo>

#proof[
  _Construction._ The VC objective is: minimize $sum_i w_i x_i$ subject to $x_i + x_j >= 1$ for $(i,j) in E$. Applying the penalty method (@sec:penalty-method), the constraint $x_i + x_j >= 1$ is violated iff $x_i = x_j = 0$, with penalty $(1 - x_i)(1 - x_j)$:
  $ f(bold(x)) = sum_i w_i x_i + P sum_((i,j) in E) (1 - x_i)(1 - x_j) $
  Expanding: $(1 - x_i)(1 - x_j) = 1 - x_i - x_j + x_i x_j$.
  Summing over all edges, each vertex $i$ appears in $"deg"(i)$ terms. The QUBO coefficients are: diagonal $Q_(i i) = w_i - P dot "deg"(i)$ (objective plus linear penalty), off-diagonal $Q_(i j) = P$ for edges. The constant $P |E|$ does not affect the minimizer.
]

```rust
// Minimal example: VC -> QUBO -> extract solution
let vc = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
let result = ReduceTo::<QUBO>::reduce_to(&vc);
let qubo = result.target_problem();

let solver = BruteForce::new();
let solutions = solver.find_best(qubo);
let vc_solution = result.extract_solution(&solutions[0]);
assert!(vc.solution_size(&vc_solution).is_valid);
```

#theorem[
  *(KColoring $arrow.r$ QUBO)* Given $G = (V, E)$ with $k$ colors, construct upper-triangular $Q in RR^(n k times n k)$ using one-hot encoding $x_(v,c) in {0,1}$ ($n k$ variables indexed by $v dot k + c$). [_Problems:_ @def:coloring, @def:qubo.]
] <thm:coloring-to-qubo>

#proof[
  _Construction._ Applying the penalty method (@sec:penalty-method), the QUBO objective combines a one-hot constraint penalty and an edge conflict penalty:
  $ f(bold(x)) = P_1 sum_(v in V) (1 - sum_(c=1)^k x_(v,c))^2 + P_2 sum_((u,v) in E) sum_(c=1)^k x_(u,c) x_(v,c) $

  _One-hot expansion._ For each vertex $v$, using $x_(v,c)^2 = x_(v,c)$:
  $ (1 - sum_c x_(v,c))^2 = 1 - sum_c x_(v,c) + 2 sum_(c_1 < c_2) x_(v,c_1) x_(v,c_2) $
  This yields diagonal $Q_(v k+c, v k+c) = -P_1$ and intra-vertex off-diagonal $Q_(v k+c_1, v k+c_2) = 2 P_1$ for $c_1 < c_2$.

  _Edge penalty._ For each edge $(u,v)$ and color $c$, the term $P_2 x_(u,c) x_(v,c)$ contributes to $Q_(u k+c, v k+c) += P_2$ (with appropriate index ordering).

  In our implementation, $P_1 = P = 1 + n$ and $P_2 = P\/2$.

  _Solution extraction._ For each vertex $v$, find $c$ with $x_(v,c) = 1$.
]

```rust
// Minimal example: KColoring -> QUBO -> extract solution
let kc = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
let result = ReduceTo::<QUBO>::reduce_to(&kc);
let qubo = result.target_problem();

let solver = BruteForce::new();
let solutions = solver.find_best(qubo);
let kc_solution = result.extract_solution(&solutions[0]);
assert_eq!(solutions.len(), 6); // 3! valid 3-colorings of K3
```

#theorem[
  *(SetPacking $arrow.r$ QUBO)* Equivalent to IS on the intersection graph: $Q_(i i) = -w_i$ and $Q_(i j) = P$ for overlapping sets $i, j$ ($i < j$), where $P = 1 + sum_i w_i$. [_Problems:_ @def:set-packing, @def:qubo.]
] <thm:setpacking-to-qubo>

#proof[
  Two sets conflict iff they share an element. The intersection graph has sets as vertices and edges between conflicting pairs. Applying the penalty method (@sec:penalty-method) yields the same QUBO as IS on this graph: diagonal rewards selection, off-diagonal penalizes overlap. Correctness follows from the IS→QUBO proof.
]

```rust
// Minimal example: SetPacking -> QUBO -> extract solution
let sp = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3, 4]]);
let result = ReduceTo::<QUBO>::reduce_to(&sp);
let qubo = result.target_problem();

let solver = BruteForce::new();
let solutions = solver.find_best(qubo);
let sp_solution = result.extract_solution(&solutions[0]);
assert!(sp.solution_size(&sp_solution).is_valid);
```

#theorem[
  *(2-SAT $arrow.r$ QUBO)* Given a Max-2-SAT instance with $m$ clauses over $n$ variables, construct upper-triangular $Q in RR^(n times n)$ where each clause $(ell_i or ell_j)$ contributes a penalty gadget encoding its unique falsifying assignment. [_Problems:_ @def:k-sat, @def:qubo.]
] <thm:ksat-to-qubo>

#proof[
  _Construction._ Applying the penalty method (@sec:penalty-method), each 2-literal clause has exactly one falsifying assignment (both literals false). The penalty for that assignment is a quadratic function of $x_i, x_j$:

  #table(
    columns: (auto, auto, auto, auto),
    inset: 4pt,
    align: left,
    table.header([*Clause*], [*Falsified when*], [*Penalty*], [*QUBO contributions*]),
    [$x_i or x_j$], [$x_i=0, x_j=0$], [$(1-x_i)(1-x_j)$], [$Q_(i i) -= 1, Q_(j j) -= 1, Q_(i j) += 1$],
    [$overline(x_i) or x_j$], [$x_i=1, x_j=0$], [$x_i(1-x_j)$], [$Q_(i i) += 1, Q_(i j) -= 1$],
    [$x_i or overline(x_j)$], [$x_i=0, x_j=1$], [$(1-x_i)x_j$], [$Q_(j j) += 1, Q_(i j) -= 1$],
    [$overline(x_i) or overline(x_j)$], [$x_i=1, x_j=1$], [$x_i x_j$], [$Q_(i j) += 1$],
  )

  Summing over all clauses, $f(bold(x)) = sum_j "penalty"_j (bold(x))$ counts falsified clauses. Minimizers of $f$ maximize satisfied clauses.
]

```rust
// Minimal example: 2-SAT -> QUBO -> extract solution
let ksat = KSatisfiability::<2, i32>::new(3, vec![
    CNFClause::new(vec![1, 2]),   // x1 OR x2
    CNFClause::new(vec![-1, 3]),  // NOT x1 OR x3
    CNFClause::new(vec![2, -3]),  // x2 OR NOT x3
]);
let result = ReduceTo::<QUBO>::reduce_to(&ksat);
let qubo = result.target_problem();

let solver = BruteForce::new();
let solutions = solver.find_best(qubo);
let sat_solution = result.extract_solution(&solutions[0]);
assert!(ksat.solution_size(&sat_solution).is_valid);
```

#theorem[
  *(Binary ILP $arrow.r$ QUBO)* Given binary ILP: maximize $bold(c)^top bold(x)$ subject to $A bold(x) = bold(b)$, $bold(x) in {0,1}^n$, construct upper-triangular $Q = -"diag"(bold(c) + 2P bold(b)^top A) + P A^top A$ where $P = 1 + ||bold(c)||_1 + ||bold(b)||_1$. [_Problems:_ @def:ilp, @def:qubo.]
] <thm:ilp-to-qubo>

#proof[
  _Step 1: Normalize constraints._ Convert inequalities to equalities using slack variables: $bold(a)_k^top bold(x) <= b_k$ becomes $bold(a)_k^top bold(x) + sum_(s=0)^(S_k - 1) 2^s y_(k,s) = b_k$ where $S_k = ceil(log_2 (b_k + 1))$ slack bits. For $>=$ constraints, the slack has a negative sign. The extended system is $A' bold(x)' = bold(b)$ with $bold(x)' = (bold(x), bold(y)) in {0,1}^(n')$. For minimization, negate $bold(c)$ to convert to maximization.

  _Step 2: QUBO construction._ Applying the penalty method (@sec:penalty-method), combine objective and penalty:
  $ f(bold(x)') = -bold(c')^top bold(x)' + P sum_(k=1)^m (bold(a)'_k^(top) bold(x)' - b_k)^2 $
  where $bold(c)' = (bold(c), bold(0))$. Expanding the quadratic penalty:
  $ = bold(x)'^(top) A'^(top) A' bold(x)' - 2 bold(b)^top A' bold(x)' + ||bold(b)||_2^2 $
  Combining with $-bold(c')^top bold(x)'$ and dropping constants:
  $ Q = -"diag"(bold(c)' + 2P bold(b)^top A') + P A'^(top) A' $
  The diagonal contains linear terms; the upper triangle of $A'^(top) A'$ gives quadratic terms (doubled for upper-triangular convention).

  _Solution extraction._ Discard slack variables: return $bold(x)' [0..n]$.
]

```rust
// Minimal example: binary ILP -> QUBO -> extract solution
let ilp = ILP::binary(3,
    vec![
        LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
        LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
    ],
    vec![(0, 1.0), (1, 2.0), (2, 3.0)],
    ObjectiveSense::Maximize,
);
let result = ReduceTo::<QUBO<f64>>::reduce_to(&ilp);
let qubo = result.target_problem();

let solver = BruteForce::new();
let solutions = solver.find_best(qubo);
let ilp_solution = result.extract_solution(&solutions[0]);
assert_eq!(ilp_solution, vec![1, 0, 1]); // obj = 4
```

== Non-Trivial Reductions

#theorem[
  *(SAT $arrow.r$ IS)* @karp1972 Given CNF $phi$ with $m$ clauses, construct graph $G$ such that $phi$ is satisfiable iff $G$ has an IS of size $m$. [_Problems:_ @def:satisfiability, @def:independent-set.]
] <thm:sat-to-is>

#proof[
  _Construction._ For $phi = and.big_(j=1)^m C_j$ with $C_j = (ell_(j,1) or ... or ell_(j,k_j))$:

  _Vertices:_ For each literal $ell_(j,i)$ in clause $C_j$, create $v_(j,i)$. Total: $|V| = sum_j k_j$.

  _Edges:_ (1) Intra-clause cliques: $E_"clause" = {(v_(j,i), v_(j,i')) : i != i'}$. (2) Conflict edges: $E_"conflict" = {(v_(j,i), v_(j',i')) : j != j', ell_(j,i) = overline(ell_(j',i'))}$.

  _Correctness._ ($arrow.r.double$) A satisfying assignment selects one true literal per clause; these vertices form an IS of size $m$ (no clause edges by selection, no conflict edges by consistency). ($arrow.l.double$) An IS of size $m$ must contain exactly one vertex per clause (by clause cliques); the corresponding literals are consistent (by conflict edges) and satisfy $phi$.

  _Solution extraction._ For $v_(j,i) in S$ with literal $x_k$: set $x_k = 1$; for $overline(x_k)$: set $x_k = 0$.
]

#theorem[
  *(SAT $arrow.r$ 3-Coloring)* @garey1979 Given CNF $phi$, construct graph $G$ such that $phi$ is satisfiable iff $G$ is 3-colorable. [_Problems:_ @def:satisfiability, @def:coloring.]
] <thm:sat-to-coloring>

#proof[
  _Construction._ (1) Base triangle: TRUE, FALSE, AUX vertices with all pairs connected. (2) Variable gadget for $x_i$: vertices $"pos"_i$, $"neg"_i$ connected to each other and to AUX. (3) Clause gadget: for $(ell_1 or ... or ell_k)$, apply OR-gadgets iteratively producing output $o$, then connect $o$ to FALSE and AUX.

  _OR-gadget$(a, b) arrow.bar o$:_ Five vertices encoding $o = a or b$: if both $a, b$ have FALSE color, $o$ cannot have TRUE color.

  _Solution extraction._ Set $x_i = 1$ iff $"color"("pos"_i) = "color"("TRUE")$.
]

#theorem[
  *(SAT $arrow.r$ Dominating Set)* @garey1979 Given CNF $phi$ with $n$ variables and $m$ clauses, $phi$ is satisfiable iff the constructed graph has a dominating set of size $n$. [_Problems:_ @def:satisfiability, @def:dominating-set.]
] <thm:sat-to-dominatingset>

#proof[
  _Construction._ (1) Variable triangle for $x_i$: vertices $"pos"_i = 3i$, $"neg"_i = 3i+1$, $"dum"_i = 3i+2$ forming a triangle. (2) Clause vertex $c_j = 3n+j$ connected to $"pos"_i$ if $x_i in C_j$, to $"neg"_i$ if $overline(x_i) in C_j$.

  _Correctness._ Each triangle requires at least one vertex in any dominating set. Size-$n$ set must take exactly one per triangle, which dominates clause vertices iff corresponding literals satisfy all clauses.

  _Solution extraction._ Set $x_i = 1$ if $"pos"_i$ selected; $x_i = 0$ if $"neg"_i$ selected.
]

#theorem[
  *(SAT $arrow.l.r$ $k$-SAT)* @cook1971 @garey1979 Any SAT formula converts to $k$-SAT ($k >= 3$) preserving satisfiability. [_Problems:_ @def:satisfiability, @def:k-sat.]
] <thm:sat-ksat>

#proof[
  _Small clauses ($|C| < k$):_ Pad $(ell_1 or ... or ell_r)$ with auxiliary $y$: $(ell_1 or ... or ell_r or y or overline(y) or ...)$ to length $k$.

  _Large clauses ($|C| > k$):_ Split $(ell_1 or ... or ell_r)$ with auxiliaries $y_1, ..., y_(r-k)$:
  $ (ell_1 or ... or ell_(k-1) or y_1) and (overline(y_1) or ell_k or ... or y_2) and ... and (overline(y_(r-k)) or ell_(r-k+2) or ... or ell_r) $

  _Correctness._ Original clause true $arrow.l.r$ auxiliary chain can propagate truth through new clauses.
]

#theorem[
  *(CircuitSAT $arrow.r$ Spin Glass)* @whitfield2012 @lucas2014 Each gate maps to a gadget whose ground states encode valid I/O. [_Problems:_ @def:circuit-sat, @def:spin-glass.]
] <thm:circuit-to-spinglass>

#proof[
  _Spin mapping:_ $sigma in {0,1} arrow.bar s = 2sigma - 1 in {-1, +1}$.

  _Gate gadgets_ (inputs 0,1; output 2; auxiliary 3 for XOR) are shown in @tab:gadgets. Allocate spins per variable, instantiate gadgets, sum Hamiltonians. Ground states correspond to satisfying assignments.
]

#figure(
  table(
    columns: (auto, auto, auto),
    inset: 4pt,
    align: left,
    table.header([*Gate*], [*Couplings $J$*], [*Fields $h$*]),
    [AND], [$J_(01)=1, J_(02)=J_(12)=-2$], [$h_0=h_1=-1, h_2=2$],
    [OR], [$J_(01)=1, J_(02)=J_(12)=-2$], [$h_0=h_1=1, h_2=-2$],
    [NOT], [$J_(01)=1$], [$h_0=h_1=0$],
    [XOR], [$J_(01)=1, J_(02)=J_(12)=-1, J_(03)=J_(13)=-2, J_(23)=2$], [$h_0=h_1=-1, h_2=1, h_3=2$],
  ),
  caption: [Ising gadgets for logic gates. Ground states match truth tables.]
) <tab:gadgets>

#theorem[
  *(Factoring $arrow.r$ Circuit-SAT)* An array multiplier with output constrained to $N$ is satisfiable iff $N$ factors within bit bounds. _(Folklore; no canonical reference.)_ [_Problems:_ @def:factoring, @def:circuit-sat.]
] <thm:factoring-to-circuit>

#proof[
  _Construction._ Build $m times n$ array multiplier for $p times q$:

  _Full adder $(i,j)$:_ $s_(i,j) + 2c_(i,j) = (p_i and q_j) + s_"prev" + c_"prev"$ via:
  $ a := p_i and q_j, quad t_1 := a xor s_"prev", quad s_(i,j) := t_1 xor c_"prev" $
  $ t_2 := t_1 and c_"prev", quad t_3 := a and s_"prev", quad c_(i,j) := t_2 or t_3 $

  _Output constraint:_ $M_k := "bit"_k(N)$ for $k = 1, ..., m+n$.

  _Solution extraction._ $p = sum_i p_i 2^(i-1)$, $q = sum_j q_j 2^(j-1)$.
]

#theorem[
  *(Spin Glass $arrow.l.r$ Max-Cut)* @barahona1982 @lucas2014 Ground states of Ising models correspond to maximum cuts. [_Problems:_ @def:spin-glass, @def:max-cut.]
] <thm:spinglass-maxcut>

#proof[
  _MaxCut $arrow.r$ SpinGlass:_ Set $J_(i j) = w_(i j)$, $h_i = 0$. Maximizing cut equals minimizing $-sum J_(i j) s_i s_j$ since $s_i s_j = -1$ when $s_i != s_j$.

  _SpinGlass $arrow.r$ MaxCut:_ If $h_i = 0$: direct mapping $w_(i j) = J_(i j)$. Otherwise, add ancilla $a$ with $w_(i,a) = h_i$.

  _Solution extraction._ Without ancilla: identity. With ancilla: if $sigma_a = 1$, flip all spins before removing ancilla.
]

```rust
// Minimal example: SpinGlass -> MaxCut -> extract solution
let sg = SpinGlass::new(3, vec![((0, 1), 1), ((1, 2), 1), ((0, 2), 1)], vec![0, 0, 0]);
let result = ReduceTo::<MaxCut<i32>>::reduce_to(&sg);
let maxcut = result.target_problem();

let solver = BruteForce::new();
let maxcut_solutions = solver.find_best(maxcut);
let sg_solution = result.extract_solution(&maxcut_solutions[0]);
assert_eq!(sg_solution.len(), 3);
```

#theorem[
  *(Coloring $arrow.r$ ILP)* The $k$-coloring problem reduces to binary ILP with $|V| dot k$ variables and $|V| + |E| dot k$ constraints. [_Problems:_ @def:coloring, @def:ilp.]
] <thm:coloring-to-ilp>

#proof[
  _Construction._ For graph $G = (V, E)$ with $k$ colors:

  _Variables:_ Binary $x_(v,c) in {0, 1}$ for each vertex $v in V$ and color $c in {1, ..., k}$. Interpretation: $x_(v,c) = 1$ iff vertex $v$ has color $c$.

  _Constraints:_ (1) Each vertex has exactly one color: $sum_(c=1)^k x_(v,c) = 1$ for all $v in V$. (2) Adjacent vertices have different colors: $x_(u,c) + x_(v,c) <= 1$ for all $(u, v) in E$ and $c in {1, ..., k}$.

  _Objective:_ Feasibility problem (minimize 0).

  _Correctness._ ($arrow.r.double$) A valid $k$-coloring assigns exactly one color per vertex with different colors on adjacent vertices; setting $x_(v,c) = 1$ for the assigned color satisfies all constraints. ($arrow.l.double$) Any feasible ILP solution has exactly one $x_(v,c) = 1$ per vertex; this defines a coloring, and constraint (2) ensures adjacent vertices differ.

  _Solution extraction._ For each vertex $v$, find $c$ with $x_(v,c) = 1$; assign color $c$ to $v$.
]

#theorem[
  *(Factoring $arrow.r$ ILP)* Integer factorization reduces to binary ILP using McCormick linearization with $O(m n)$ variables and constraints. [_Problems:_ @def:factoring, @def:ilp.]
] <thm:factoring-to-ilp>

#proof[
  _Construction._ For target $N$ with $m$-bit factor $p$ and $n$-bit factor $q$:

  _Variables:_ Binary $p_i, q_j in {0,1}$ for factor bits; binary $z_(i j) in {0,1}$ for products $p_i dot q_j$; integer $c_k >= 0$ for carries at each bit position.

  _Product linearization (McCormick):_ For each $z_(i j) = p_i dot q_j$:
  $ z_(i j) <= p_i, quad z_(i j) <= q_j, quad z_(i j) >= p_i + q_j - 1 $

  _Bit-position equations:_ For each bit position $k$:
  $ sum_(i+j=k) z_(i j) + c_(k-1) = N_k + 2 c_k $
  where $N_k$ is the $k$-th bit of $N$ and $c_(-1) = 0$.

  _No overflow:_ $c_(m+n-1) = 0$.

  _Correctness._ The McCormick constraints enforce $z_(i j) = p_i dot q_j$ for binary variables. The bit equations encode $p times q = N$ via carry propagation, matching array multiplier semantics.

  _Solution extraction._ Read $p = sum_i p_i 2^i$ and $q = sum_j q_j 2^j$ from the binary variables.
]

_Example: Factoring 15._ The following Rust code demonstrates the closed-loop reduction (requires `ilp` feature: `cargo add problemreductions --features ilp`):

```rust
use problemreductions::prelude::*;

// 1. Create factoring instance: find p (4-bit) × q (4-bit) = 15
let problem = Factoring::new(4, 4, 15);

// 2. Reduce to ILP
let reduction = ReduceTo::<ILP>::reduce_to(&problem);
let ilp = reduction.target_problem();

// 3. Solve ILP
let solver = ILPSolver::new();
let ilp_solution = solver.solve(ilp).unwrap();

// 4. Extract factoring solution
let extracted = reduction.extract_solution(&ilp_solution);

// 5. Verify: reads factors and confirms p × q = 15
let (p, q) = problem.read_factors(&extracted);
assert_eq!(p * q, 15); // e.g., (3, 5) or (5, 3)
```

== Unit Disk Mapping

#theorem[
  *(IS $arrow.r$ GridGraph IS)* @nguyen2023 Any MIS problem on a general graph $G$ can be reduced to MIS on a unit disk graph (King's subgraph) with at most quadratic overhead in the number of vertices. [_Problem:_ @def:independent-set.]
] <thm:is-to-gridgraph>

#proof[
  _Construction (Copy-Line Method)._ Given $G = (V, E)$ with $n = |V|$:

  1. _Vertex ordering:_ Compute a path decomposition of $G$ to obtain vertex order $(v_1, ..., v_n)$. The pathwidth determines the grid height.

  2. _Copy lines:_ For each vertex $v_i$, create an L-shaped "copy line" on the grid:
  $ "CopyLine"(v_i) = {(r, c_i) : r in [r_"start", r_"stop"]} union {(r_i, c) : c in [c_i, c_"stop"]} $
  where positions are determined by the vertex order and edge structure.

  3. _Crossing gadgets:_ When two copy lines cross (corresponding to an edge $(v_i, v_j) in E$), insert a crossing gadget that enforces: at most one of the two lines can be "active" (all vertices selected).

  4. _MIS correspondence:_ Each copy line has MIS contribution $approx |"line"|/2$. The gadgets add overhead $Delta$ such that:
  $ "MIS"(G_"grid") = "MIS"(G) + Delta $

  _Solution extraction._ For each copy line, check if the majority of its vertices are in the grid MIS. Map back: $v_i in S$ iff copy line $i$ is active.

  _Correctness._ ($arrow.r.double$) An IS in $G$ maps to selecting all copy line vertices for included vertices; crossing gadgets ensure no conflicts. ($arrow.l.double$) A grid MIS maps back to an IS by the copy line activity rule.
]

*Example: Petersen Graph.*#footnote[Generated using `cargo run --example export_petersen_mapping` from the accompanying code repository.] The Petersen graph ($n=10$, MIS$=4$) maps to a $30 times 42$ King's subgraph with 219 nodes and overhead $Delta = 89$. Solving MIS on the grid yields $"MIS"(G_"grid") = 4 + 89 = 93$. The weighted and unweighted KSG mappings share identical grid topology (same node positions and edges); only the vertex weights differ. With triangular lattice encoding @nguyen2023, the same graph maps to a $42 times 60$ grid with 395 nodes and overhead $Delta = 375$, giving $"MIS"(G_"tri") = 4 + 375 = 379$.

// Load JSON data
#let petersen = json("petersen_source.json")
#let square_weighted = json("petersen_square_weighted.json")
#let square_unweighted = json("petersen_square_unweighted.json")
#let triangular_mapping = json("petersen_triangular.json")

// Draw Petersen graph with standard layout
#let draw-petersen-cetz(data) = canvas(length: 1cm, {
  import draw: *
  let r-outer = 1.2
  let r-inner = 0.6

  // Positions: outer pentagon (0-4), inner star (5-9)
  let positions = ()
  for i in range(5) {
    let angle = 90deg - i * 72deg
    positions.push((calc.cos(angle) * r-outer, calc.sin(angle) * r-outer))
  }
  for i in range(5) {
    let angle = 90deg - i * 72deg
    positions.push((calc.cos(angle) * r-inner, calc.sin(angle) * r-inner))
  }

  // Draw edges
  for edge in data.edges {
    let (u, v) = (edge.at(0), edge.at(1))
    line(positions.at(u), positions.at(v), stroke: 0.6pt + gray)
  }

  // Draw nodes
  for (k, pos) in positions.enumerate() {
    circle(pos, radius: 0.12, fill: blue, stroke: none)
  }
})

// Draw King's Subgraph from JSON nodes - uses pre-computed edges
#let draw-grid-cetz(data, cell-size: 0.2) = canvas(length: 1cm, {
  import draw: *
  let grid-data = data.grid_graph

  // Get node positions (col, row) for drawing
  let grid-positions = grid-data.nodes.map(n => (n.col, n.row))
  let weights = grid-data.nodes.map(n => n.weight)

  // Use pre-computed edges from JSON
  let edges = grid-data.edges

  // Scale for drawing
  let vertices = grid-positions.map(p => (p.at(0) * cell-size, -p.at(1) * cell-size))

  // Draw edges
  for edge in edges {
    let (k, l) = (edge.at(0), edge.at(1))
    line(vertices.at(k), vertices.at(l), stroke: 0.4pt + gray)
  }

  // Draw nodes with weight-based color
  for (k, pos) in vertices.enumerate() {
    let w = weights.at(k)
    let color = if w == 1 { blue } else if w == 2 { red } else { green }
    circle(pos, radius: 0.04, fill: color, stroke: none)
  }
})

// Draw triangular lattice from JSON nodes - uses pre-computed edges
// Matches Rust's GridGraph physical_position_static for Triangular with offset_even_cols=true:
//   x = row + offset (where offset = 0.5 if col is even)
//   y = col * sqrt(3)/2
#let draw-triangular-cetz(data, cell-size: 0.2) = canvas(length: 1cm, {
  import draw: *
  let grid-data = data.grid_graph

  // Get node positions with triangular geometry for drawing
  // Match Rust GridGraph::physical_position_static for Triangular:
  //   x = row + 0.5 (if col is even, since offset_even_cols=true)
  //   y = col * sqrt(3)/2
  let sqrt3_2 = calc.sqrt(3) / 2
  let grid-positions = grid-data.nodes.map(n => {
    let offset = if calc.rem(n.col, 2) == 0 { 0.5 } else { 0.0 }
    let x = n.row + offset
    let y = n.col * sqrt3_2
    (x, y)
  })
  let weights = grid-data.nodes.map(n => n.weight)

  // Use pre-computed edges from JSON
  let edges = grid-data.edges

  // Scale for drawing
  let vertices = grid-positions.map(p => (p.at(0) * cell-size, -p.at(1) * cell-size))

  // Draw edges
  for edge in edges {
    let (k, l) = (edge.at(0), edge.at(1))
    line(vertices.at(k), vertices.at(l), stroke: 0.3pt + gray)
  }

  // Draw nodes with weight-based color
  for (k, pos) in vertices.enumerate() {
    let w = weights.at(k)
    let color = if w == 1 { blue } else if w == 2 { red } else { green }
    circle(pos, radius: 0.025, fill: color, stroke: none)
  }
})

#figure(
  grid(
    columns: 3,
    gutter: 1.5em,
    align(center + horizon)[
      #draw-petersen-cetz(petersen)
      (a) Petersen graph
    ],
    align(center + horizon)[
      #draw-grid-cetz(square_weighted)
      (b) King's subgraph (weighted)
    ],
    align(center + horizon)[
      #draw-triangular-cetz(triangular_mapping)
      (c) Triangular lattice (weighted)
    ],
  ),
  caption: [Unit disk mappings of the Petersen graph. Blue: weight 1, red: weight 2, green: weight 3.],
) <fig:petersen-mapping>

*Weighted Extension.* For MWIS, copy lines use weighted vertices (weights 1, 2, or 3). Source weights $< 1$ are added to designated "pin" vertices.

*QUBO Mapping.* A QUBO problem $min bold(x)^top Q bold(x)$ maps to weighted MIS on a grid by:
1. Creating copy lines for each variable
2. Using XOR gadgets for couplings: $x_"out" = not(x_1 xor x_2)$
3. Adding weights for linear and quadratic terms

= Summary <sec:summary>

#let gray = rgb("#e8e8e8")

#figure(
  table(
    columns: (auto, auto, auto),
    inset: 5pt,
    align: left,
    table.header([*Reduction*], [*Overhead*], [*Reference*]),
    table.cell(fill: gray)[IS $arrow.l.r$ VC], table.cell(fill: gray)[$O(|V|)$], table.cell(fill: gray)[—],
    table.cell(fill: gray)[IS $arrow.r$ SetPacking], table.cell(fill: gray)[$O(|V| + |E|)$], table.cell(fill: gray)[—],
    table.cell(fill: gray)[Matching $arrow.r$ SetPacking], table.cell(fill: gray)[$O(|E|)$], table.cell(fill: gray)[—],
    table.cell(fill: gray)[VC $arrow.r$ SetCovering], table.cell(fill: gray)[$O(|V| + |E|)$], table.cell(fill: gray)[—],
    [IS $arrow.r$ QUBO], [$O(n)$], [@lucas2014 @glover2019],
    [VC $arrow.r$ QUBO], [$O(n)$], [@lucas2014 @glover2019],
    [KColoring $arrow.r$ QUBO], [$O(n dot k)$], [@lucas2014 @glover2019],
    [SetPacking $arrow.r$ QUBO], [$O(n)$], [@glover2019],
    [2-SAT $arrow.r$ QUBO], [$O(n)$], [@glover2019],
    [Binary ILP $arrow.r$ QUBO], [$O(n)$], [@lucas2014 @glover2019],
    [SAT $arrow.r$ IS], [$O(sum_j |C_j|^2)$], [@karp1972],
    [SAT $arrow.r$ 3-Coloring], [$O(n + sum_j |C_j|)$], [@garey1979],
    [SAT $arrow.r$ DominatingSet], [$O(3n + m)$], [@garey1979],
    [SAT $arrow.l.r$ $k$-SAT], [$O(sum_j |C_j|)$], [@cook1971 @garey1979],
    [CircuitSAT $arrow.r$ SpinGlass], [$O(|"gates"|)$], [@whitfield2012 @lucas2014],
    [Factoring $arrow.r$ CircuitSAT], [$O(m n)$], [Folklore],
    [SpinGlass $arrow.l.r$ MaxCut], [$O(n + |J|)$], [@barahona1982 @lucas2014],
    table.cell(fill: gray)[Coloring $arrow.r$ ILP], table.cell(fill: gray)[$O(|V| dot k + |E| dot k)$], table.cell(fill: gray)[—],
    table.cell(fill: gray)[Factoring $arrow.r$ ILP], table.cell(fill: gray)[$O(m n)$], table.cell(fill: gray)[—],
    [IS $arrow.r$ GridGraph IS], [$O(n^2)$], [@nguyen2023],
  ),
  caption: [Summary of reductions. Gray rows indicate trivial (complement/isomorphism) reductions.]
) <tab:summary>

#bibliography("references.bib", style: "ieee")
