// Problem Reductions: A Mathematical Reference
#let graph-data = json("../src/reductions/reduction_graph.json")
#import "@preview/cetz:0.4.2": canvas, draw
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#import "lib.typ": g-node, g-edge, petersen-graph, house-graph, octahedral-graph, draw-grid-graph, draw-triangular-graph, graph-colors, selem, sregion, draw-node-highlight, draw-edge-highlight, draw-node-colors, sregion-selected, sregion-dimmed, gate-and, gate-or, gate-xor

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show link: set text(blue)

// Set up theorem environments with ctheorems
#show: thmrules.with(qed-symbol: $square$)

// === Example JSON helpers ===
// Load canonical example database directly from the checked-in fixture file.
#let example-db = json("data/examples.json")

#let load-example(source, target, source-variant: none, target-variant: none) = {
  let matches = example-db.rules.filter(r =>
    r.source.problem == source and
    r.target.problem == target and
    (source-variant == none or r.source.variant == source-variant) and
    (target-variant == none or r.target.variant == target-variant)
  )
  if matches.len() == 1 {
    matches.at(0)
  } else if matches.len() == 0 {
    panic("Missing canonical rule example for " + source + " -> " + target)
  } else {
    panic("Ambiguous canonical rule example for " + source + " -> " + target)
  }
}

#let load-model-example(name, variant: none) = {
  let matches = example-db.models.filter(m =>
    m.problem == name and
    (variant == none or m.variant == variant)
  )
  if matches.len() == 1 {
    matches.at(0)
  } else if matches.len() == 0 {
    panic("Missing canonical model example for " + name)
  } else {
    panic("Ambiguous canonical model example for " + name)
  }
}

#let graph-num-vertices(instance) = instance.graph.num_vertices
#let graph-num-edges(instance) = instance.graph.edges.len()
#let spin-num-spins(instance) = instance.fields.len()
#let sat-num-clauses(instance) = instance.clauses.len()
#let subsetsum-num-elements(instance) = instance.sizes.len()
#let circuit-num-gates(instance) = instance.circuit.assignments.len()
#let circuit-num-variables(instance) = instance.variables.len()

#let example-name(source, target) = lower(source) + "_to_" + lower(target)

#let problem-schemas = json("../src/reductions/problem_schemas.json")

// Problem display names for theorem headers
#let display-name = (
  "AdditionalKey": [Additional Key],
  "MaximumIndependentSet": [Maximum Independent Set],
  "MinimumVertexCover": [Minimum Vertex Cover],
  "MaxCut": [Max-Cut],
  "GraphPartitioning": [Graph Partitioning],
  "GeneralizedHex": [Generalized Hex],
  "HamiltonianCircuit": [Hamiltonian Circuit],
  "BiconnectivityAugmentation": [Biconnectivity Augmentation],
  "HamiltonianPath": [Hamiltonian Path],
  "ShortestWeightConstrainedPath": [Shortest Weight-Constrained Path],
  "UndirectedTwoCommodityIntegralFlow": [Undirected Two-Commodity Integral Flow],
  "LengthBoundedDisjointPaths": [Length-Bounded Disjoint Paths],
  "IsomorphicSpanningTree": [Isomorphic Spanning Tree],
  "KthBestSpanningTree": [Kth Best Spanning Tree],
  "KColoring": [$k$-Coloring],
  "KClique": [$k$-Clique],
  "MinimumDominatingSet": [Minimum Dominating Set],
  "MaximumMatching": [Maximum Matching],
  "TravelingSalesman": [Traveling Salesman],
  "MaximumClique": [Maximum Clique],
  "MaximumSetPacking": [Maximum Set Packing],
  "MinimumSetCovering": [Minimum Set Covering],
  "ComparativeContainment": [Comparative Containment],
  "SetBasis": [Set Basis],
  "MinimumCardinalityKey": [Minimum Cardinality Key],
  "SpinGlass": [Spin Glass],
  "QUBO": [QUBO],
  "ILP": [Integer Linear Programming],
  "Knapsack": [Knapsack],
  "PartiallyOrderedKnapsack": [Partially Ordered Knapsack],
  "Satisfiability": [SAT],
  "NAESatisfiability": [NAE-SAT],
  "KSatisfiability": [$k$-SAT],
  "CircuitSAT": [CircuitSAT],
  "ConjunctiveQueryFoldability": [Conjunctive Query Foldability],
  "Factoring": [Factoring],
  "KingsSubgraph": [King's Subgraph MIS],
  "TriangularSubgraph": [Triangular Subgraph MIS],
  "MaximalIS": [Maximal Independent Set],
  "BMF": [Boolean Matrix Factorization],
  "PaintShop": [Paint Shop],
  "BicliqueCover": [Biclique Cover],
  "BalancedCompleteBipartiteSubgraph": [Balanced Complete Bipartite Subgraph],
  "BoundedComponentSpanningForest": [Bounded Component Spanning Forest],
  "BinPacking": [Bin Packing],
  "BoyceCoddNormalFormViolation": [Boyce-Codd Normal Form Violation],
  "ClosestVectorProblem": [Closest Vector Problem],
  "ConsecutiveSets": [Consecutive Sets],
  "MinimumMultiwayCut": [Minimum Multiway Cut],
  "OptimalLinearArrangement": [Optimal Linear Arrangement],
  "RuralPostman": [Rural Postman],
  "LongestCommonSubsequence": [Longest Common Subsequence],
  "ExactCoverBy3Sets": [Exact Cover by 3-Sets],
  "SubsetSum": [Subset Sum],
  "Partition": [Partition],
  "MinimumFeedbackArcSet": [Minimum Feedback Arc Set],
  "MinimumFeedbackVertexSet": [Minimum Feedback Vertex Set],
  "ConjunctiveBooleanQuery": [Conjunctive Boolean Query],
  "ConsecutiveBlockMinimization": [Consecutive Block Minimization],
  "ConsecutiveOnesSubmatrix": [Consecutive Ones Submatrix],
  "DirectedTwoCommodityIntegralFlow": [Directed Two-Commodity Integral Flow],
  "MinMaxMulticenter": [Min-Max Multicenter],
  "FlowShopScheduling": [Flow Shop Scheduling],
  "MinimumCutIntoBoundedSets": [Minimum Cut Into Bounded Sets],
  "MinimumSumMulticenter": [Minimum Sum Multicenter],
  "MinimumTardinessSequencing": [Minimum Tardiness Sequencing],
  "MultipleChoiceBranching": [Multiple Choice Branching],
  "MultipleCopyFileAllocation": [Multiple Copy File Allocation],
  "MultiprocessorScheduling": [Multiprocessor Scheduling],
  "PartitionIntoPathsOfLength2": [Partition into Paths of Length 2],
  "PartitionIntoTriangles": [Partition Into Triangles],
  "PrecedenceConstrainedScheduling": [Precedence Constrained Scheduling],
  "PrimeAttributeName": [Prime Attribute Name],
  "QuadraticAssignment": [Quadratic Assignment],
  "QuantifiedBooleanFormulas": [Quantified Boolean Formulas (QBF)],
  "RectilinearPictureCompression": [Rectilinear Picture Compression],
  "ResourceConstrainedScheduling": [Resource Constrained Scheduling],
  "SchedulingWithIndividualDeadlines": [Scheduling With Individual Deadlines],
  "SequencingToMinimizeMaximumCumulativeCost": [Sequencing to Minimize Maximum Cumulative Cost],
  "SequencingToMinimizeWeightedCompletionTime": [Sequencing to Minimize Weighted Completion Time],
  "SequencingToMinimizeWeightedTardiness": [Sequencing to Minimize Weighted Tardiness],
  "SequencingWithReleaseTimesAndDeadlines": [Sequencing with Release Times and Deadlines],
  "SequencingWithinIntervals": [Sequencing Within Intervals],
  "ShortestCommonSupersequence": [Shortest Common Supersequence],
  "StaffScheduling": [Staff Scheduling],
  "SteinerTree": [Steiner Tree],
  "SteinerTreeInGraphs": [Steiner Tree in Graphs],
  "StringToStringCorrection": [String-to-String Correction],
  "StrongConnectivityAugmentation": [Strong Connectivity Augmentation],
  "SubgraphIsomorphism": [Subgraph Isomorphism],
  "SumOfSquaresPartition": [Sum of Squares Partition],
  "TimetableDesign": [Timetable Design],
  "TwoDimensionalConsecutiveSets": [2-Dimensional Consecutive Sets],
)

// Definition label: "def:<ProblemName>" — each definition block must have a matching label


// Generate theorem label from source/target names (uses full names for consistency)
#let reduction-label(source, target) = {
  label("thm:" + source + "-to-" + target)
}

// State for tracking which reduction rules are described in the paper
#let covered-rules = state("covered-rules", ())

// Extract reductions for a problem from graph-data (returns (name, label) pairs)
#let get-reductions-to(problem-name) = {
  graph-data.edges
    .filter(e => graph-data.nodes.at(e.source).name == problem-name)
    .map(e => (name: graph-data.nodes.at(e.target).name, lbl: reduction-label(graph-data.nodes.at(e.source).name, graph-data.nodes.at(e.target).name)))
    .dedup(key: e => e.name)
}

#let get-reductions-from(problem-name) = {
  graph-data.edges
    .filter(e => graph-data.nodes.at(e.target).name == problem-name)
    .map(e => (name: graph-data.nodes.at(e.source).name, lbl: reduction-label(graph-data.nodes.at(e.source).name, graph-data.nodes.at(e.target).name)))
    .dedup(key: e => e.name)
}

// Render a single reduction with link (uses context to skip broken links gracefully)
#let render-reduction-link(r) = {
  context {
    if query(r.lbl).len() > 0 { link(r.lbl)[#r.name] }
    else { r.name }
  }
}

// Render complexity from graph-data nodes
#let render-complexity(name) = {
  let nodes = graph-data.nodes.filter(n => n.name == name)
  if nodes.len() == 0 { return }
  let seen = ()
  let entries = ()
  for node in nodes {
    if node.complexity not in seen {
      seen.push(node.complexity)
      entries.push(node.complexity)
    }
  }
  block(above: 0.5em)[
    #set text(size: 9pt)
    - Complexity: #entries.map(e => raw(e)).join("; ").
  ]
}

// Render the "Reduces to/from" lines for a problem
#let render-reductions(problem-name) = {
  let reduces-to = get-reductions-to(problem-name)
  let reduces-from = get-reductions-from(problem-name)
  if reduces-to.len() > 0 or reduces-from.len() > 0 {
    block(above: 0.5em)[
    #set text(size: 9pt)
      #if reduces-to.len() > 0 [
        - Reduces to: #reduces-to.map(render-reduction-link).join(", "). \
      ]
      #if reduces-from.len() > 0 [
        - Reduces from: #reduces-from.map(render-reduction-link).join(", ").
      ]
    ]
  }
}

// Render a problem's JSON schema as a field table (subtle styling)
#let render-schema(name) = {
  let schema = problem-schemas.find(s => s.name == name)
  if schema == none { return }
  block(
    stroke: (left: 2pt + luma(180)),
    inset: (left: 8pt),
  )[
    #set text(size: 9pt)
    #table(
      columns: (auto, 1fr),
      inset: (x: 2pt, y: 3pt),
      align: (left, left),
      stroke: none,
      table.header(
        text(fill: luma(30), raw(name)),
      ),
      table.hline(stroke: 0.3pt + luma(200)),
      ..schema.fields.map(f => (
        text(fill: luma(60), raw(f.name)),
        text(fill: luma(60), raw(f.description))
      )).flatten()
    )
  ]
}

// Render a concrete example box from JSON data (unified schema)
#let reduction-example(data, caption: none, body) = {
  block(
    width: 100%,
    inset: (x: 1em, y: 0.8em),
    fill: rgb("#f0f7ff"),
    stroke: (left: 2pt + rgb("#4a86e8")),
  )[
    #if caption != none {
      text(weight: "bold")[Example: #caption]
      parbreak()
    }
    *Source:* #data.source.problem
    #h(1em)
    *Target:* #data.target.problem
    #if body != none { parbreak(); body }
  ]
}

#let theorem = thmplain("theorem", [#h(-1.2em)Rule], base_level: 1)
#let proof = thmproof("proof", "Proof")
#let definition = thmbox(
  "definition",
  "Definition",
  fill: rgb("#f8f8f8"),
  stroke: (left: 2pt + rgb("#4a86e8")),
  inset: (x: 1em, y: 0.8em),
  breakable: true,
  base_level: 1,
)

// Problem definition wrapper: auto-adds schema, complexity, reductions list, and label
#let problem-def(name, def, body) = {
  let lbl = label("def:" + name)
  let title = display-name.at(name)
  [#definition(title)[
    #def
    #render-complexity(name)
    #render-reductions(name)
    #render-schema(name)

    #body
  ]
  #lbl]
}

// Find edge in graph-data by source/target names
#let find-edge(source, target) = {
  let edge = graph-data.edges.find(e => graph-data.nodes.at(e.source).name == source and graph-data.nodes.at(e.target).name == target)
  if edge == none {
    edge = graph-data.edges.find(e => graph-data.nodes.at(e.source).name == target and graph-data.nodes.at(e.target).name == source)
  }
  edge
}

// Build display name from a graph-data node (name + variant)
#let variant-display(node) = {
  let base = display-name.at(node.name)
  if node.variant.len() == 0 { return base }
  let parts = ()
  if "graph" in node.variant and node.variant.graph != "SimpleGraph" {
    parts.push(node.variant.graph)
  }
  if "weight" in node.variant {
    if node.variant.weight == "i32" { parts.push("weighted") }
    else if node.variant.weight == "f64" { parts.push("real-weighted") }
  }
  if "k" in node.variant { parts.push[$k$-ary] }
  if parts.len() > 0 { [#base (#parts.join(", "))] } else { base }
}

// Format overhead fields as inline text
#let format-overhead(overhead) = {
  let parts = overhead.map(o => raw(o.field + " = " + o.formula))
  [_Overhead:_ #parts.join(", ").]
}

// Unified function for reduction rules: theorem + proof + optional example
#let reduction-rule(
  source, target,
  example: false,
  example-source-variant: none,
  example-target-variant: none,
  example-caption: none,
  extra: none,
  theorem-body, proof-body,
) = {
  let arrow = sym.arrow.r
  let edge = find-edge(source, target)
  let src-disp = if edge != none { variant-display(graph-data.nodes.at(edge.source)) }
                 else { display-name.at(source) }
  let tgt-disp = if edge != none { variant-display(graph-data.nodes.at(edge.target)) }
                 else { display-name.at(target) }
  let src-lbl = label("def:" + source)
  let tgt-lbl = label("def:" + target)
  let overhead = if edge != none and edge.overhead.len() > 0 { edge.overhead } else { none }
  let thm-lbl = label("thm:" + source + "-to-" + target)
  covered-rules.update(old => old + ((source, target),))

  [
    #v(1em)
    #theorem[
    *(*#context { if query(src-lbl).len() > 0 { link(src-lbl)[#src-disp] } else [#src-disp] }* #arrow *#context { if query(tgt-lbl).len() > 0 { link(tgt-lbl)[#tgt-disp] } else [#tgt-disp] }*)* #theorem-body
    #if overhead != none { linebreak(); format-overhead(overhead) }
  ] #thm-lbl]

  proof[#proof-body]

  if example {
    let data = load-example(
      source,
      target,
      source-variant: example-source-variant,
      target-variant: example-target-variant,
    )
    pad(left: 1.5em, reduction-example(data, caption: example-caption)[#extra])
  }
}

#align(center)[
  #text(size: 16pt, weight: "bold")[Problem Reductions: Models and Transformations]
  #v(0.5em)
  #text(size: 11pt)[Jin-Guo Liu#super[1] #h(1em) Xi-Wei Pan#super[1]]
  #v(0.3em)
  #text(size: 9pt)[#super[1]Hong Kong University of Science and Technology (Guangzhou)]
  #v(0.3em)
  #text(size: 10pt, style: "italic")[github.com/CodingThrust/problem-reductions]
  #v(1em)
]

#block(width: 100%, inset: (x: 2em, y: 1em))[
  *Abstract.* We present formal definitions for computational problems and polynomial-time reductions implemented in the `problem-reductions` library. For each reduction, we state theorems with constructive proofs that preserve solution structure.
]


// Table of contents
#outline(title: "Contents", indent: 1.5em, depth: 2)

#pagebreak()

= Introduction

A _reduction_ from problem $A$ to problem $B$, denoted $A arrow.long B$, is a polynomial-time transformation of $A$-instances into $B$-instances such that: (1) the transformation runs in polynomial time, (2) solutions to $B$ can be efficiently mapped back to solutions of $A$, and (3) optimal solutions are preserved. The library implements #graph-data.edges.len() reductions connecting #graph-data.nodes.len() problem types.

== Notation

We use the following notation throughout. An _undirected graph_ $G = (V, E)$ consists of a vertex set $V$ and edge set $E subset.eq binom(V, 2)$. For a set $S$, $overline(S)$ or $V backslash S$ denotes its complement. We write $|S|$ for cardinality. A _clique_ in $G$ is a subset $K subset.eq V$ where every pair of distinct vertices is adjacent: $(u, v) in E$ for all distinct $u, v in K$. A _unit disk graph_ is a graph where vertices are points on a 2D lattice and $(u, v) in E$ iff $d(u, v) <= r$ for some radius $r$; a _King's subgraph_ uses the 8-connectivity square grid with $r approx 1.5$. For Boolean variables, $overline(x)$ denotes negation ($not x$). A _literal_ is a variable $x$ or its negation $overline(x)$. A _clause_ is a disjunction of literals. A formula in _conjunctive normal form_ (CNF) is a conjunction of clauses. We abbreviate Independent Set as IS, Vertex Cover as VC, and use $n$ for problem size, $m$ for number of clauses, and $k_j = |C_j|$ for clause size.

= Problem Definitions <sec:problems>

Each problem definition follows this structure:

#block(
  inset: (x: 1em, y: 0.8em),
  fill: rgb("#f8f8f8"),
  stroke: (left: 2pt + rgb("#4a86e8")),
)[
  *Definition N (Problem Name).* Formal problem statement defining input, constraints, and objective.

  #block(
    stroke: (left: 2pt + luma(180)),
    inset: (left: 8pt),
  )[
    #set text(size: 9pt)
    #table(
      columns: (auto, 1fr),
      inset: (x: 6pt, y: 3pt),
      align: (left, left),
      stroke: none,
      table.header(text(fill: luma(30), raw("ProblemName"))),
      table.hline(stroke: 0.3pt + luma(200)),
      text(fill: luma(60), raw("field_name")), text(fill: luma(60), raw("Field description from JSON schema")),
    )
  ]

  #set text(size: 9pt, fill: luma(60))
  _Reduces to:_ ProblemA, ProblemB. \
  _Reduces from:_ ProblemC.
]

The gray schema table shows the JSON field names used in the library's data structures. The reduction links at the bottom connect to the corresponding theorems in @sec:reductions.



== Graph Problems

In all graph problems below, $G = (V, E)$ denotes an undirected graph with $|V| = n$ vertices and $|E|$ edges.

#{
  // MIS has two entries in examples.json; select the unit-weight variant
  let x = load-model-example("MaximumIndependentSet", variant: (graph: "SimpleGraph", weight: "One"))
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  // Pick optimal config = {v1, v3, v5, v9} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let S = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let alpha = sol.metric.Valid
  [
    #problem-def("MaximumIndependentSet")[
      Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing $sum_(v in S) w(v)$ such that no two vertices in $S$ are adjacent: $forall u, v in S: (u, v) in.not E$.
    ][
    One of Karp's 21 NP-complete problems @karp1972, MIS appears in wireless network scheduling, register allocation, and coding theory @shannon1956. Solvable in polynomial time on bipartite graphs (König's theorem), interval graphs, chordal graphs, and cographs. The best known algorithm runs in $O^*(1.1996^n)$ time via measure-and-conquer branching @xiao2017. On geometric graphs (King's subgraph, triangular subgraph, unit disk graphs), MIS admits subexponential $O^*(c^sqrt(n))$ algorithms for some constant $c$, via geometric separation @alber2004.

    *Example.* Consider the Petersen graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, and unit weights $w(v) = 1$ for all $v in V$. The graph is 3-regular (every vertex has degree 3). A maximum independent set is $S = {#S.map(i => $v_#i$).join(", ")}$ with $w(S) = sum_(v in S) w(v) = #alpha = alpha(G)$. No two vertices in $S$ share an edge, and no vertex can be added without violating independence.

    #figure({
      let pg = petersen-graph()
      draw-node-highlight(pg.vertices, pg.edges, S)
    },
    caption: [The Petersen graph with a maximum independent set $S = {#S.map(i => $v_#i$).join(", ")}$ shown in blue ($alpha(G) = #alpha$). Outer vertices $v_0, ..., v_4$ form a pentagon; inner vertices $v_5, ..., v_9$ form a pentagram. Unit weights $w(v_i) = 1$.],
    ) <fig:petersen-mis>
    ]
  ]
}

#{
  let x = load-model-example("MinimumVertexCover")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  // Pick optimal config = {v0, v3, v4} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let cover = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let wS = sol.metric.Valid
  let complement = sol.config.enumerate().filter(((i, v)) => v == 0).map(((i, _)) => i)
  let alpha = complement.len()
  [
    #problem-def("MinimumVertexCover")[
      Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ such that every edge has at least one endpoint in $S$: $forall (u, v) in E: u in S or v in S$.
    ][
    One of Karp's 21 NP-complete problems @karp1972. Vertex Cover is the complement of Independent Set: $S$ is a vertex cover iff $V backslash S$ is an independent set, so $|"VC"| + |"IS"| = n$. Central to parameterized complexity, admitting FPT algorithms in $O^*(1.2738^k)$ time parameterized by solution size $k$. The best known exact algorithm runs in $O^*(1.1996^n)$ via the MIS complement @xiao2017.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, and unit weights $w(v) = 1$. A minimum vertex cover is $S = {#cover.map(i => $v_#i$).join(", ")}$ with $w(S) = #wS$: #edges.map(((u, v)) => {
      let by = if cover.contains(u) and cover.contains(v) { "both" } else if cover.contains(u) { $v_#u$ } else { $v_#v$ }
      [$(v_#u, v_#v)$ by #by]
    }).join("; "). The complement ${#complement.map(i => $v_#i$).join(", ")}$ is a maximum independent set ($alpha(G) = #alpha$, confirming $|"VC"| = n - alpha = #wS$).

    #figure({
      let hg = house-graph()
      draw-node-highlight(hg.vertices, hg.edges, cover)
    },
    caption: [The house graph with a minimum vertex cover $S = {#cover.map(i => $v_#i$).join(", ")}$ shown in blue ($w(S) = #wS$). Every edge is incident to at least one blue vertex.],
    ) <fig:house-vc>
    ]
  ]
}

#{
  let x = load-model-example("MaxCut")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  // Pick optimal config = S={v0, v3} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let side-s = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let side-sbar = sol.config.enumerate().filter(((i, v)) => v == 0).map(((i, _)) => i)
  let cut-val = sol.metric.Valid
  let cut-edges = edges.filter(e => side-s.contains(e.at(0)) != side-s.contains(e.at(1)))
  let uncut-edges = edges.filter(e => side-s.contains(e.at(0)) == side-s.contains(e.at(1)))
  [
    #problem-def("MaxCut")[
      Given $G = (V, E)$ with weights $w: E -> RR$, find partition $(S, overline(S))$ maximizing $sum_((u,v) in E: u in S, v in overline(S)) w(u, v)$.
    ][
    Max-Cut is NP-hard on general graphs @barahona1982 but polynomial-time solvable on planar graphs. The Goemans-Williamson SDP relaxation achieves a 0.878-approximation ratio @goemans1995, which is optimal assuming the Unique Games Conjecture. The best known exact algorithm runs in $O^*(2^(omega n slash 3))$ time via algebraic 2-CSP techniques @williams2005, where $omega < 2.372$ is the matrix multiplication exponent.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, and unit weights $w(e) = 1$. The partition $S = {#side-s.map(i => $v_#i$).join(", ")}$, $overline(S) = {#side-sbar.map(i => $v_#i$).join(", ")}$ cuts #cut-val of #ne edges: #cut-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", "). #if uncut-edges.len() == 1 [Only the edge #uncut-edges.map(((u, v)) => $(v_#u, v_#v)$).at(0) is uncut (both endpoints in $overline(S)$).] #if uncut-edges.len() > 1 [The edges #uncut-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ") are uncut.] The cut value is $sum w(e) = #cut-val$.

    #figure({
      let hg = house-graph()
      let cut-edges = hg.edges.filter(e => side-s.contains(e.at(0)) != side-s.contains(e.at(1)))
      draw-edge-highlight(hg.vertices, hg.edges, cut-edges, side-s)
    },
    caption: [The house graph with max cut $S = {#side-s.map(i => $v_#i$).join(", ")}$ (blue) vs $overline(S) = {#side-sbar.map(i => $v_#i$).join(", ")}$ (white). Cut edges shown in bold blue; #cut-val of #ne edges are cut.],
    ) <fig:house-maxcut>
    ]
  ]
}
#{
  let x = load-model-example("GraphPartitioning")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let config = x.optimal_config
  let cut-val = x.optimal_value.Valid
  let side-a = range(nv).filter(i => config.at(i) == 0)
  let side-b = range(nv).filter(i => config.at(i) == 1)
  let cut-edges = edges.filter(e => config.at(e.at(0)) != config.at(e.at(1)))
  [
    #problem-def("GraphPartitioning")[
      Given an undirected graph $G = (V, E)$ with $|V| = n$ (even), find a partition of $V$ into two disjoint sets $A$ and $B$ with $|A| = |B| = n slash 2$ that minimizes the number of edges crossing the partition:
      $ "cut"(A, B) = |{(u, v) in E : u in A, v in B}|. $
    ][
      Graph Partitioning is a core NP-hard problem arising in VLSI design, parallel computing, and scientific simulation, where balanced workload distribution with minimal communication is essential. Closely related to Max-Cut (which _maximizes_ rather than _minimizes_ the cut) and to the Ising Spin Glass model. NP-completeness was proved by Garey, Johnson and Stockmeyer @garey1976. Arora, Rao and Vazirani @arora2009 gave an $O(sqrt(log n))$-approximation algorithm. The best known unconditional exact algorithm is brute-force enumeration of all $binom(n, n slash 2) = O^*(2^n)$ balanced partitions; no faster worst-case algorithm is known. Cygan et al. @cygan2014 showed that Minimum Bisection is fixed-parameter tractable in $O(2^(O(k^3)) dot n^3 log^3 n)$ time parameterized by bisection width $k$. Standard partitioning tools include METIS, KaHIP, and Scotch.

      *Example.* Consider the graph $G$ with $n = #nv$ vertices and #ne edges. The optimal balanced partition is $A = {#side-a.map(i => $v_#i$).join($,$)}$, $B = {#side-b.map(i => $v_#i$).join($,$)}$, with cut value #cut-val.

      #figure(
        canvas(length: 1cm, {
          // Two-column layout for balanced partition
          let half = int(nv / 2)
          let verts = (
            ..range(half).map(i => (0, (half - 1 - i))),
            ..range(half).map(i => (2.5, (half - 1 - i))),
          )
          // Draw edges
          for (u, v) in edges {
            let crossing = config.at(u) != config.at(v)
            g-edge(verts.at(u), verts.at(v),
              stroke: if crossing { 2pt + graph-colors.at(1) } else { 1pt + luma(180) })
          }
          // Draw partition regions
          import draw: *
          on-layer(-1, {
            rect((-0.5, -0.5), (0.5, half - 0.5),
              fill: graph-colors.at(0).transparentize(90%),
              stroke: (dash: "dashed", paint: graph-colors.at(0), thickness: 0.8pt))
            content((0, half - 0.2), text(8pt, fill: graph-colors.at(0))[$A$])
            rect((2.0, -0.5), (3.0, half - 0.5),
              fill: graph-colors.at(1).transparentize(90%),
              stroke: (dash: "dashed", paint: graph-colors.at(1), thickness: 0.8pt))
            content((2.5, half - 0.2), text(8pt, fill: graph-colors.at(1))[$B$])
          })
          // Draw nodes
          for (k, pos) in verts.enumerate() {
            let in-a = config.at(k) == 0
            g-node(pos, name: "v" + str(k),
              fill: if in-a { graph-colors.at(0) } else { graph-colors.at(1) },
              label: text(fill: white)[$v_#k$])
          }
        }),
        caption: [Graph with $n = #nv$ vertices partitioned into $A$ (blue) and $B$ (red). The #cut-val crossing edges are shown in bold red; internal edges are gray.],
      ) <fig:graph-partitioning>
    ]
  ]
}
#problem-def("MinimumCutIntoBoundedSets")[
  Given an undirected graph $G = (V, E)$ with edge weights $w: E -> ZZ^+$, designated vertices $s, t in V$, a positive integer $B <= |V|$, and a positive integer $K$, determine whether there exists a partition of $V$ into disjoint sets $V_1$ and $V_2$ such that $s in V_1$, $t in V_2$, $|V_1| <= B$, $|V_2| <= B$, and
  $ sum_({u,v} in E: u in V_1, v in V_2) w({u,v}) <= K. $
][
Minimum Cut Into Bounded Sets (Garey & Johnson ND17) combines the classical minimum $s$-$t$ cut problem with a balance constraint on partition sizes. Without the balance constraint ($B = |V|$), the problem reduces to standard minimum $s$-$t$ cut, solvable in polynomial time via network flow. Adding the requirement $|V_1| <= B$ and $|V_2| <= B$ makes the problem NP-complete; it remains NP-complete even for $B = |V| slash 2$ and unit edge weights (the minimum bisection problem) @garey1976. Applications include VLSI layout, load balancing, and graph bisection.

The best known exact algorithm is brute-force enumeration of all $2^n$ vertex partitions in $O(2^n)$ time. For the special case of minimum bisection, Cygan et al. @cygan2014 showed fixed-parameter tractability with respect to the cut size. No polynomial-time finite approximation factor exists for balanced graph partition unless $P = N P$ (Andreev and Racke, 2006). Arora, Rao, and Vazirani @arora2009 gave an $O(sqrt(log n))$-approximation for balanced separator.

*Example.* Consider $G$ with 4 vertices and edges $(v_0, v_1)$, $(v_1, v_2)$, $(v_2, v_3)$ with unit weights, $s = v_0$, $t = v_3$, $B = 3$, $K = 1$. The partition $V_1 = {v_0, v_1}$, $V_2 = {v_2, v_3}$ gives cut weight $w({v_1, v_2}) = 1 <= K$. Both $|V_1| = 2 <= 3$ and $|V_2| = 2 <= 3$. Answer: YES.
]
#problem-def("BiconnectivityAugmentation")[
  Given an undirected graph $G = (V, E)$, a set $F$ of candidate edges on $V$ with $F inter E = emptyset$, weights $w: F -> RR$, and a budget $B in RR$, find $F' subset.eq F$ such that $sum_(e in F') w(e) <= B$ and the augmented graph $G' = (V, E union F')$ is biconnected, meaning $G'$ is connected and deleting any single vertex leaves it connected.
][
Biconnectivity augmentation is a classical network-design problem: add backup links so the graph survives any single vertex failure. The weighted candidate-edge formulation modeled here captures communication, transportation, and infrastructure planning settings where only a prescribed set of new links is feasible and each carries a cost. In this library, the exact baseline is brute-force enumeration over the $m = |F|$ candidate edges, yielding $O^*(2^m)$ time and matching the exported complexity metadata for the model.

*Example.* Consider the path graph $v_0 - v_1 - v_2 - v_3 - v_4 - v_5$ with candidate edges $(v_0, v_2)$, $(v_0, v_3)$, $(v_0, v_4)$, $(v_1, v_3)$, $(v_1, v_4)$, $(v_1, v_5)$, $(v_2, v_4)$, $(v_2, v_5)$, $(v_3, v_5)$ carrying weights $(1, 2, 3, 1, 2, 3, 1, 2, 1)$ and budget $B = 4$. Selecting $F' = {(v_0, v_2), (v_1, v_3), (v_2, v_4), (v_3, v_5)}$ uses total weight $1 + 1 + 1 + 1 = 4$ and eliminates every articulation point: after deleting any single vertex, the remaining graph is still connected. Reducing the budget to $B = 3$ makes the instance infeasible, because one of the path endpoints remains attached through a single articulation vertex.

#figure(
  canvas(length: 1cm, {
    import draw: *
    // 6 vertices in a horizontal line
    let verts = range(6).map(k => (k * 1.5, 0))
    let path-edges = ((0,1),(1,2),(2,3),(3,4),(4,5))
    // Candidate edges: (u, v, weight, selected?)
    let candidates = (
      (0, 2, 1, true), (0, 3, 2, false), (0, 4, 3, false),
      (1, 3, 1, true), (1, 4, 2, false), (1, 5, 3, false),
      (2, 4, 1, true), (2, 5, 2, false), (3, 5, 1, true),
    )
    let blue = graph-colors.at(0)
    let green = graph-colors.at(2)
    let gray = luma(180)
    // Draw path edges (existing graph)
    for (u, v) in path-edges {
      g-edge(verts.at(u), verts.at(v), stroke: 2pt + black)
    }
    // Draw candidate edges as arcs above the path
    for (u, v, w, sel) in candidates {
      let mid-x = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
      let span = v - u
      let height = span * 0.4
      let ctrl = (mid-x, height)
      bezier(verts.at(u), verts.at(v), ctrl,
        stroke: if sel { 2.5pt + green } else { (dash: "dashed", paint: gray, thickness: 0.8pt) })
      // Weight label
      content((mid-x, height + 0.25),
        text(7pt, fill: if sel { green.darken(30%) } else { gray })[#w])
    }
    // Draw nodes
    for (k, pos) in verts.enumerate() {
      g-node(pos, name: "v" + str(k), label: [$v_#k$])
    }
  }),
  caption: [Biconnectivity Augmentation on a 6-vertex path with $B = 4$. Existing edges are black; green arcs show the selected augmentation $F'$ (total weight 4); dashed gray arcs are unselected candidates. The resulting graph $G' = (V, E union F')$ is biconnected.],
) <fig:biconnectivity-augmentation>
]
#{
  let x = load-model-example("HamiltonianCircuit")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let circuit = sol.config
  // Build circuit edges from consecutive vertices (including wrap-around)
  let circuit-edges = range(circuit.len()).map(i => (circuit.at(i), circuit.at(calc.rem(i + 1, circuit.len()))))
  [
    #problem-def("HamiltonianCircuit")[
      *Instance:* An undirected graph $G = (V, E)$.

      *Question:* Does $G$ contain a _Hamiltonian circuit_ --- a closed path that visits every vertex exactly once?
    ][
      The Hamiltonian Circuit problem is one of Karp's original 21 NP-complete problems @karp1972, and is listed as GT37 in Garey & Johnson @garey1979.
      It is closely related to the Traveling Salesman Problem: while TSP seeks to minimize the total weight of a Hamiltonian cycle on a weighted complete graph, the Hamiltonian Circuit problem simply asks whether _any_ such cycle exists on a general (unweighted) graph.

      A configuration is a permutation $pi$ of the vertices, interpreted as the order in which they are visited.
      The circuit is valid when every consecutive pair $(pi(i), pi(i+1 mod n))$ is an edge in $G$.

      *Algorithms.*
      The classical Held--Karp dynamic programming algorithm @heldkarp1962 solves the problem in $O(n^2 dot 2^n)$ time and $O(n dot 2^n)$ space.
      Björklund's randomized "Determinant Sums" algorithm achieves $O^*(1.657^n)$ time for general graphs and $O^*(sqrt(2)^n)$ for bipartite graphs @bjorklund2014.

      *Example.* Consider the triangular prism graph $G$ on #nv vertices with #ne edges. The permutation $[#circuit.map(v => str(v)).join(", ")]$ forms a Hamiltonian circuit: each consecutive pair #circuit-edges.map(((u, v)) => $(#u, #v)$).join($,$) is an edge of $G$, and the path returns to the start.

      #figure({
        let blue = graph-colors.at(0)
        let gray = luma(200)
        canvas(length: 1cm, {
          import draw: *
          // Triangular prism: outer triangle + inner triangle
          let r-out = 1.8
          let r-in = 0.9
          let verts = range(3).map(k => {
            let angle = 90deg - k * 120deg
            (calc.cos(angle) * r-out, calc.sin(angle) * r-out)
          }) + range(3).map(k => {
            let angle = 90deg - k * 120deg
            (calc.cos(angle) * r-in, calc.sin(angle) * r-in)
          })
          for (u, v) in edges {
            let on-circuit = circuit-edges.any(e => (e.at(0) == u and e.at(1) == v) or (e.at(0) == v and e.at(1) == u))
            g-edge(verts.at(u), verts.at(v), stroke: if on-circuit { 2pt + blue } else { 1pt + gray })
          }
          for (k, pos) in verts.enumerate() {
            g-node(pos, name: "v" + str(k),
              fill: blue,
              label: text(fill: white)[$v_#k$])
          }
        })
      },
      caption: [Hamiltonian Circuit in the triangular prism graph. Blue edges show the circuit $#circuit.map(v => $v_#v$).join($arrow$) arrow v_#(circuit.at(0))$.],
      ) <fig:hamiltonian-circuit>
    ]
  ]
}


#problem-def("BoundedComponentSpanningForest")[
  Given an undirected graph $G = (V, E)$ with vertex weights $w: V -> ZZ_(gt.eq 0)$, a positive integer $K <= |V|$, and a positive bound $B$, determine whether there exists a partition of $V$ into $t$ non-empty sets $V_1, dots, V_t$ with $1 <= t <= K$ such that each induced subgraph $G[V_i]$ is connected and each part satisfies $sum_(v in V_i) w(v) <= B$.
][
Bounded Component Spanning Forest appears as ND10 in Garey and Johnson @garey1979. It asks for a decomposition into a bounded number of connected pieces, each with bounded total weight, so it naturally captures contiguous districting and redistricting-style constraints where each district must remain connected while respecting a population cap. A direct exhaustive search over component labels gives an $O^*(K^n)$ baseline, but subset-DP techniques via inclusion-exclusion improve the exact running time to $O^*(3^n)$ @bjorklund2009.

*Example.* Consider the graph on vertices ${v_0, v_1, dots, v_7}$ with edges $(v_0, v_1)$, $(v_1, v_2)$, $(v_2, v_3)$, $(v_3, v_4)$, $(v_4, v_5)$, $(v_5, v_6)$, $(v_6, v_7)$, $(v_0, v_7)$, $(v_1, v_5)$, $(v_2, v_6)$; vertex weights $(2, 3, 1, 2, 3, 1, 2, 1)$; component limit $K = 3$; and bound $B = 6$. The partition
$V_1 = {v_0, v_1, v_7}$,
$V_2 = {v_2, v_3, v_4}$,
$V_3 = {v_5, v_6}$
is feasible: each set induces a connected subgraph, the component weights are $2 + 3 + 1 = 6$, $1 + 2 + 3 = 6$, and $1 + 2 = 3$, and exactly three non-empty components are used. Therefore this instance is a YES instance.

#figure(
  canvas(length: 1cm, {
    import draw: *
    // 8 vertices in a circular layout (radius 1.6)
    let r = 1.6
    let verts = range(8).map(k => {
      let angle = 90deg - k * 45deg
      (calc.cos(angle) * r, calc.sin(angle) * r)
    })
    let weights = (2, 3, 1, 2, 3, 1, 2, 1)
    let edges = ((0,1),(1,2),(2,3),(3,4),(4,5),(5,6),(6,7),(0,7),(1,5),(2,6))
    // Partition: V1={0,1,7} blue, V2={2,3,4} green, V3={5,6} red
    let partition = (0, 0, 1, 1, 1, 2, 2, 0)
    let comp-colors = (graph-colors.at(0), graph-colors.at(2), graph-colors.at(1))
    // Draw edges: bold colored for intra-component, gray for cross-component
    for (u, v) in edges {
      if partition.at(u) == partition.at(v) {
        g-edge(verts.at(u), verts.at(v),
          stroke: 2pt + comp-colors.at(partition.at(u)))
      } else {
        g-edge(verts.at(u), verts.at(v),
          stroke: 1pt + luma(180))
      }
    }
    // Draw nodes colored by partition, with weight labels
    for (k, pos) in verts.enumerate() {
      let c = comp-colors.at(partition.at(k))
      g-node(pos, name: "v" + str(k),
        fill: c,
        label: text(fill: white)[$v_#k$])
      let angle = 90deg - k * 45deg
      let lpos = (calc.cos(angle) * (r + 0.5), calc.sin(angle) * (r + 0.5))
      content(lpos, text(7pt)[$w = #(weights.at(k))$])
    }
  }),
  caption: [Bounded Component Spanning Forest on 8 vertices with $K = 3$ and $B = 6$. The partition $V_1 = {v_0, v_1, v_7}$ (blue, weight 6), $V_2 = {v_2, v_3, v_4}$ (green, weight 6), $V_3 = {v_5, v_6}$ (red, weight 3) is feasible. Bold colored edges are intra-component; gray edges cross components.],
) <fig:bcsf>
]
#{
  let x = load-model-example("LengthBoundedDisjointPaths")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let s = x.instance.source
  let t = x.instance.sink
  let J = x.instance.num_paths_required
  let K = x.instance.max_length
  let chosen-verts = (0, 1, 2, 3, 6)
  let chosen-edges = ((0, 1), (1, 6), (0, 2), (2, 3), (3, 6))
  [
    #problem-def("LengthBoundedDisjointPaths")[
      Given an undirected graph $G = (V, E)$, distinct terminals $s, t in V$, and positive integers $J, K$, determine whether $G$ contains at least $J$ pairwise internally vertex-disjoint paths from $s$ to $t$, each using at most $K$ edges.
    ][
      Length-Bounded Disjoint Paths is the bounded-routing version of the classical disjoint-path problem, with applications in network routing and VLSI where multiple connections must fit simultaneously under quality-of-service limits. Garey & Johnson list it as ND41 and summarize the sharp threshold proved by Itai, Perl, and Shiloach: the problem is NP-complete for every fixed $K >= 5$, polynomial-time solvable for $K <= 4$, and becomes polynomial again when the length bound is removed entirely @garey1979. The implementation here uses the natural $J dot |V|$ binary membership encoding, so brute-force search over configurations runs in $O^*(2^(J dot |V|))$.

      *Example.* Consider the graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, terminals $s = v_#s$, $t = v_#t$, $J = #J$, and $K = #K$. The two paths $P_1 = v_0 arrow v_1 arrow v_6$ and $P_2 = v_0 arrow v_2 arrow v_3 arrow v_6$ are both of length at most 3, and their internal vertex sets ${v_1}$ and ${v_2, v_3}$ are disjoint. Hence this instance is satisfying. The third branch $v_0 arrow v_4 arrow v_5 arrow v_6$ is available but unused, so the instance has multiple satisfying path-slot assignments.

      #figure(
        canvas(length: 1cm, {
          let blue = graph-colors.at(0)
          let gray = luma(180)
          let verts = (
            (0, 1),    // v0 = s
            (1.3, 1.8),
            (1.3, 1.0),
            (2.6, 1.0),
            (1.3, 0.2),
            (2.6, 0.2),
            (3.9, 1),  // v6 = t
          )
          for (u, v) in edges {
            let selected = chosen-edges.any(e =>
              (e.at(0) == u and e.at(1) == v) or (e.at(0) == v and e.at(1) == u)
            )
            g-edge(verts.at(u), verts.at(v),
              stroke: if selected { 2pt + blue } else { 1pt + gray })
          }
          for (k, pos) in verts.enumerate() {
            let active = chosen-verts.contains(k)
            g-node(pos, name: "v" + str(k),
              fill: if active { blue } else { white },
              label: if active {
                text(fill: white)[
                  #if k == s { $s$ }
                  else if k == t { $t$ }
                  else { $v_#k$ }
                ]
              } else [
                #if k == s { $s$ }
                else if k == t { $t$ }
                else { $v_#k$ }
              ])
          }
        }),
        caption: [A satisfying Length-Bounded Disjoint Paths instance with $s = v_0$, $t = v_6$, $J = 2$, and $K = 3$. The highlighted paths are $v_0 arrow v_1 arrow v_6$ and $v_0 arrow v_2 arrow v_3 arrow v_6$; the lower branch through $v_4, v_5$ remains unused.],
      ) <fig:length-bounded-disjoint-paths>
    ]
  ]
}
#{
  let x = load-model-example("GeneralizedHex")
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let source = x.instance.source
  let target = x.instance.target
  let winning-path = ((0, 1), (1, 4), (4, 5))
  [
    #problem-def("GeneralizedHex")[
      Given an undirected graph $G = (V, E)$ and distinct terminals $s, t in V$, determine whether Player 1 has a forced win in the vertex-claiming Shannon switching game where the players alternately claim vertices of $V backslash {s, t}$, coloring them blue and red respectively, and Player 1 wins iff the final coloring contains an $s$-$t$ path whose internal vertices are all blue.
    ][
      Generalized Hex is the vertex version of the Shannon switching game listed by Garey & Johnson (A8 GP1). Even and Tarjan proved that deciding whether the first player has a winning strategy is PSPACE-complete @evenTarjan1976. The edge-claiming Shannon switching game is a classical contrast point: Bruno and Weinberg showed that the edge version is polynomial-time solvable via matroid methods @brunoWeinberg1970.

      The implementation evaluates the decision problem directly rather than searching over candidate assignments. The instance has `dims() = []`, and `evaluate([])` runs a memoized minimax search over the ternary states (unclaimed, blue, red) of the nonterminal vertices. This preserves the alternating-game semantics of the original problem instead of collapsing the game into a static coloring predicate.

      *Example.* The canonical fixture uses the six-vertex graph with terminals $s = v_#source$ and $t = v_#target$, and edges #edges.map(((u, v)) => $(v_#u, v_#v)$).join(", "). Vertex $v_4$ is the unique neighbor of $t$, so Player 1 opens by claiming $v_4$. Player 2 can then block at most one of $v_1$, $v_2$, and $v_3$; Player 1 responds by claiming one of the remaining branch vertices, completing a blue path $v_0 arrow v_i arrow v_4 arrow v_5$. The fixture database therefore has exactly one satisfying configuration: the empty configuration, which triggers the internal game-tree evaluator on the initial board.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let gray = luma(185)
          let verts = (
            (0, 1.0),
            (1.6, 2.2),
            (1.6, 1.0),
            (1.6, -0.2),
            (3.3, 1.0),
            (5.0, 1.0),
          )
          for (u, v) in edges {
            let on-path = winning-path.any(e =>
              (e.at(0) == u and e.at(1) == v) or
              (e.at(0) == v and e.at(1) == u)
            )
            g-edge(
              verts.at(u),
              verts.at(v),
              stroke: if on-path { 2pt + blue } else { 1pt + gray },
            )
          }
          for (k, pos) in verts.enumerate() {
            let highlighted = k == source or k == 1 or k == 4 or k == target
            g-node(
              pos,
              name: "v" + str(k),
              fill: if highlighted { blue } else { white },
              stroke: 1pt + if highlighted { blue } else { gray },
              label: text(fill: if highlighted { white } else { black })[$v_#k$],
            )
          }
          content((0, 1.55), text(8pt)[$s$])
          content((5.0, 1.55), text(8pt)[$t$])
        }),
        caption: [A winning Generalized Hex instance. Player 1 first claims $v_4$, then answers any red move on $\{v_1, v_2, v_3\}$ by taking a different branch vertex and completing a blue path from $s = v_0$ to $t = v_5$.],
      ) <fig:generalized-hex>
    ]
  ]
}
#{
  let x = load-model-example("HamiltonianPath")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  // Pick optimal config = [0, 2, 4, 3, 1, 5] to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let path = sol.config
  // Build path edges from consecutive vertices in the path
  let path-edges = range(path.len() - 1).map(i => (path.at(i), path.at(i + 1)))
  [
    #problem-def("HamiltonianPath")[
      Given a graph $G = (V, E)$, determine whether $G$ contains a _Hamiltonian path_, i.e., a simple path that visits every vertex exactly once.
    ][
      A classical NP-complete decision problem from Garey & Johnson (A1.3 GT39), closely related to _Hamiltonian Circuit_. Finding a Hamiltonian path in $G$ is equivalent to finding a Hamiltonian circuit in an augmented graph $G'$ obtained by adding a new vertex adjacent to all vertices of $G$. The problem remains NP-complete for planar graphs, cubic graphs, and bipartite graphs.

      The best known exact algorithm is Björklund's randomized $O^*(1.657^n)$ "Determinant Sums" method @bjorklund2014, which applies to both Hamiltonian path and circuit. The classical Held--Karp dynamic programming algorithm solves it in $O(n^2 dot 2^n)$ deterministic time.

      Variables: $n = |V|$ values forming a permutation. Position $i$ holds the vertex visited at step $i$. A configuration is satisfying when it forms a valid permutation of all vertices and consecutive vertices are adjacent in $G$.

      *Example.* Consider the graph $G$ on #nv vertices with edges ${#edges.map(((u, v)) => $(#u, #v)$).join(", ")}$. The sequence $[#path.map(v => str(v)).join(", ")]$ is a Hamiltonian path: it visits every vertex exactly once, and each consecutive pair is adjacent --- #path-edges.map(((u, v)) => $(#u, #v)$).join($,$) $in E$.

      #figure({
        let blue = graph-colors.at(0)
        let gray = luma(200)
        canvas(length: 1cm, {
          import draw: *
          let verts = ((0, 1.5), (1.5, 1.5), (3, 1.5), (1.5, 0), (3, 0), (0, 0))
          for (u, v) in edges {
            let on-path = path-edges.any(e => (e.at(0) == u and e.at(1) == v) or (e.at(0) == v and e.at(1) == u))
            g-edge(verts.at(u), verts.at(v), stroke: if on-path { 2pt + blue } else { 1pt + gray })
          }
          for (k, pos) in verts.enumerate() {
            g-node(pos, name: "v" + str(k),
              fill: blue,
              label: text(fill: white)[$v_#k$])
          }
        })
      },
      caption: [Hamiltonian Path in a #{nv}-vertex graph. Blue edges show the path $#path.map(v => $v_#v$).join($arrow$)$.],
      ) <fig:hamiltonian-path>
    ]
  ]
}
#{
  let x = load-model-example("UndirectedTwoCommodityIntegralFlow")
  let satisfying_count = 1
  let source1 = x.instance.source_1
  let source2 = x.instance.source_2
  let sink1 = x.instance.sink_1
  [
    #problem-def("UndirectedTwoCommodityIntegralFlow")[
      Given an undirected graph $G = (V, E)$, specified terminals $s_1, s_2, t_1, t_2 in V$, edge capacities $c: E -> ZZ^+$, and requirements $R_1, R_2 in ZZ^+$, determine whether there exist two integral flow functions $f_1, f_2$ that orient each used edge for each commodity, respect the shared edge capacities, conserve flow at every vertex in $V backslash {s_1, s_2, t_1, t_2}$, and deliver at least $R_i$ units of net flow into $t_i$ for each commodity $i in {1, 2}$.
    ][
      Undirected Two-Commodity Integral Flow is the undirected counterpart of the classical two-commodity integral flow problem from Garey \& Johnson (ND39) @garey1979. Even, Itai, and Shamir proved that it remains NP-complete even when every capacity is 1, but becomes polynomial-time solvable when all capacities are even, giving a rare parity-driven complexity dichotomy @evenItaiShamir1976.

      The implementation uses four variables per undirected edge ${u, v}$: $f_1(u, v)$, $f_1(v, u)$, $f_2(u, v)$, and $f_2(v, u)$. In the unit-capacity regime, each edge has exactly five meaningful local states: unused, commodity 1 in either direction, or commodity 2 in either direction, which matches the catalog bound $O(5^m)$ for $m = |E|$.

      *Example.* Consider the graph with edges $(0, 2)$, $(1, 2)$, and $(2, 3)$, capacities $(1, 1, 2)$, sources $s_1 = v_#source1$, $s_2 = v_#source2$, and shared sink $t_1 = t_2 = v_#sink1$. The optimal configuration in the fixture database sets $f_1(0, 2) = 1$, $f_2(1, 2) = 1$, and $f_1(2, 3) = f_2(2, 3) = 1$, with all reverse-direction variables zero. The only nonterminal vertex is $v_2$, where each commodity has one unit of inflow and one unit of outflow, so conservation holds. Vertex $v_3$ receives one unit of net inflow from each commodity, and the shared edge $(2,3)$ uses its full capacity 2. The fixture database contains #satisfying_count satisfying configuration for this instance, shown below.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let teal = rgb("#76b7b2")
          let gray = luma(190)
          let verts = ((0, 1.2), (0, -1.2), (2.0, 0), (4.0, 0))
          let labels = (
            [$s_1 = v_0$],
            [$s_2 = v_1$],
            [$v_2$],
            [$t_1 = t_2 = v_3$],
          )
          let edges = ((0, 2), (1, 2), (2, 3))
          for (u, v) in edges {
            g-edge(verts.at(u), verts.at(v), stroke: 1pt + gray)
          }
          g-edge(verts.at(0), verts.at(2), stroke: 1.8pt + blue)
          g-edge(verts.at(1), verts.at(2), stroke: (paint: teal, thickness: 1.8pt, dash: "dashed"))
          g-edge(verts.at(2), verts.at(3), stroke: 1.8pt + blue)
          g-edge(verts.at(2), verts.at(3), stroke: (paint: teal, thickness: 1.8pt, dash: "dashed"))
          for (i, pos) in verts.enumerate() {
            let fill = if i == 0 { blue } else if i == 1 { teal } else if i == 3 { rgb("#e15759") } else { white }
            g-node(pos, name: "utcif-" + str(i), fill: fill, label: if i == 2 { labels.at(i) } else { text(fill: white)[#labels.at(i)] })
          }
          content((1.0, 0.95), text(8pt, fill: gray)[$c = 1$])
          content((1.0, -0.95), text(8pt, fill: gray)[$c = 1$])
          content((3.0, 0.35), text(8pt, fill: gray)[$c = 2$])
        }),
        caption: [Canonical shared-capacity YES instance for Undirected Two-Commodity Integral Flow. Solid blue carries commodity 1 and dashed teal carries commodity 2; both commodities share the edge $(v_2, v_3)$ of capacity 2.],
      ) <fig:undirected-two-commodity-integral-flow>
    ]
  ]
}
#{
  let x = load-model-example("IsomorphicSpanningTree")
  let g-edges = x.instance.graph.edges
  let t-edges = x.instance.tree.edges
  let nv = x.instance.graph.num_vertices
  let nt = x.instance.tree.num_vertices
  // optimal config = identity mapping [0,1,2,3]
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let pi = sol.config
  // Map tree edges through the bijection
  let mapped-edges = t-edges.map(((u, v)) => (pi.at(u), pi.at(v)))
  [
    #problem-def("IsomorphicSpanningTree")[
      Given a graph $G = (V, E)$ and a tree $T = (V_T, E_T)$ with $|V| = |V_T|$, determine whether $G$ contains a spanning tree isomorphic to $T$: does there exist a bijection $pi: V_T -> V$ such that for every edge ${u, v} in E_T$, ${pi(u), pi(v)} in E$?
    ][
      A classical NP-complete problem listed as ND8 in Garey & Johnson @garey1979. The Isomorphic Spanning Tree problem strictly generalizes Hamiltonian Path: a graph $G$ has a Hamiltonian path if and only if $G$ contains a spanning tree isomorphic to the path $P_n$. The problem remains NP-complete even when $T$ is restricted to trees of bounded degree @papadimitriou1982.

      Brute-force enumeration of all bijections $pi: V_T -> V$ and checking each against the edge set of $G$ runs in $O(n! dot n)$ time. No substantially faster exact algorithm is known for general instances.

      Variables: $n = |V|$ values forming a permutation. Position $i$ holds the graph vertex that tree vertex $i$ maps to under $pi$. A configuration is satisfying when it forms a valid permutation and every tree edge maps to a graph edge.

      *Example.* Consider $G = K_#nv$ (the complete graph on #nv vertices) and $T$ the star $S_#(nt - 1)$ with center $0$ and leaves ${#range(1, nt).map(i => str(i)).join(", ")}$. Since $K_#nv$ contains all possible edges, any bijection $pi$ maps the star's edges to edges of $G$. For instance, the identity mapping $pi(i) = i$ gives the spanning tree ${#mapped-edges.map(((u, v)) => $(#u, #v)$).join(", ")} subset.eq E(K_#nv)$.

      #figure({
        let blue = graph-colors.at(0)
        let gray = luma(200)
        canvas(length: 1cm, {
          import draw: *
          let gv = ((0, 0), (1.5, 0), (1.5, 1.5), (0, 1.5))
          let tree-edges-mapped = mapped-edges
          for (u, v) in g-edges {
            let is-tree = tree-edges-mapped.any(e => (e.at(0) == u and e.at(1) == v) or (e.at(0) == v and e.at(1) == u))
            g-edge(gv.at(u), gv.at(v), stroke: if is-tree { 2pt + blue } else { 1pt + gray })
          }
          for (k, pos) in gv.enumerate() {
            let is-center = k == pi.at(0)
            g-node(pos, name: "g" + str(k),
              fill: if is-center { blue } else { white },
              label: if is-center { text(fill: white)[$v_#k$] } else { [$v_#k$] })
          }
          content((2.5, 0.75), text(10pt)[$arrow.l.double$])
          let tv = ((3.5, 0.75), (5.0, 0), (5.0, 0.75), (5.0, 1.5))
          for (u, v) in t-edges {
            g-edge(tv.at(u), tv.at(v), stroke: 2pt + blue)
          }
          for (k, pos) in tv.enumerate() {
            let is-center = k == 0
            g-node(pos, name: "t" + str(k),
              fill: if is-center { blue } else { white },
              label: if is-center { text(fill: white)[$u_#k$] } else { [$u_#k$] })
          }
        })
      },
      caption: [Isomorphic Spanning Tree: the graph $G = K_#nv$ (left) contains a spanning tree isomorphic to the star $S_#(nt - 1)$ (right, blue edges). The identity mapping $pi(u_i) = v_i$ embeds all #t-edges.len() star edges into $G$. Center vertex $v_#(pi.at(0))$ shown in blue.],
      ) <fig:isomorphic-spanning-tree>
    ]
  ]
}
#{
  let x = load-model-example("ShortestWeightConstrainedPath")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let lengths = x.instance.edge_lengths
  let weights = x.instance.edge_weights
  let s = x.instance.source_vertex
  let t = x.instance.target_vertex
  let K = x.instance.length_bound
  let W = x.instance.weight_bound
  let path-config = x.optimal_config
  let path-edges = edges.enumerate().filter(((idx, _)) => path-config.at(idx) == 1).map(((idx, e)) => e)
  let path-order = (0, 2, 3, 5)
  [
    #problem-def("ShortestWeightConstrainedPath")[
      Given an undirected graph $G = (V, E)$ with positive edge lengths $l: E -> ZZ^+$, positive edge weights $w: E -> ZZ^+$, designated vertices $s, t in V$, and bounds $K, W in ZZ^+$, determine whether there exists a simple path $P$ from $s$ to $t$ such that $sum_(e in P) l(e) <= K$ and $sum_(e in P) w(e) <= W$.
    ][
      Also called the _restricted shortest path_ or _resource-constrained shortest path_ problem. Garey and Johnson list it as ND30 and show NP-completeness via transformation from Partition @garey1979. The model captures bicriteria routing: one resource measures path length or delay, while the other captures a second consumable budget such as cost, risk, or bandwidth. Because pseudo-polynomial dynamic programming formulations are known @joksch1966, the hardness is weak rather than strong; approximation schemes were later developed by Hassin @hassin1992 and improved by Lorenz and Raz @lorenzraz2001.

      The implementation catalog reports the natural brute-force complexity of the edge-subset encoding used here: with $m = |E|$ binary variables, exhaustive search over all candidate subsets costs $O^*(2^m)$. A configuration is satisfying precisely when the selected edges form a single simple $s$-$t$ path and both resource sums stay within their bounds.

      *Example.* Consider the graph on #nv vertices with source $s = v_#s$, target $t = v_#t$, length bound $K = #K$, and weight bound $W = #W$. Edge labels are written as $(l(e), w(e))$. The highlighted path $#path-order.map(v => $v_#v$).join($arrow$)$ uses edges ${#path-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$, so its total length is $4 + 1 + 4 = 9 <= #K$ and its total weight is $1 + 3 + 3 = 7 <= #W$. This instance has 2 satisfying edge selections; another feasible path is $v_0 arrow v_1 arrow v_4 arrow v_5$.

      #figure({
        let blue = graph-colors.at(0)
        let gray = luma(200)
        let verts = ((0, 1), (1.5, 1.8), (1.5, 0.2), (3, 1.8), (3, 0.2), (4.5, 1))
        canvas(length: 1cm, {
          import draw: *
          for (idx, (u, v)) in edges.enumerate() {
            let on-path = path-config.at(idx) == 1
            g-edge(verts.at(u), verts.at(v), stroke: if on-path { 2pt + blue } else { 1pt + gray })
            let mx = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
            let my = (verts.at(u).at(1) + verts.at(v).at(1)) / 2
            let dx = if idx == 7 { -0.25 } else if idx == 5 or idx == 6 { 0.15 } else { 0 }
            let dy = if idx == 0 or idx == 2 or idx == 5 { 0.16 } else if idx == 1 or idx == 4 or idx == 6 { -0.16 } else if idx == 7 { 0.12 } else { 0 }
            draw.content(
              (mx + dx, my + dy),
              text(7pt, fill: luma(80))[#("(" + str(int(lengths.at(idx))) + ", " + str(int(weights.at(idx))) + ")")]
            )
          }
          for (k, pos) in verts.enumerate() {
            let on-path = path-order.any(v => v == k)
            g-node(pos, name: "v" + str(k),
              fill: if on-path { blue } else { white },
              label: if on-path { text(fill: white)[$v_#k$] } else { [$v_#k$] })
          }
        })
      },
      caption: [Shortest Weight-Constrained Path instance with edge labels $(l(e), w(e))$. The highlighted path $v_0 arrow v_2 arrow v_3 arrow v_5$ satisfies both bounds.],
      ) <fig:shortest-weight-constrained-path>
    ]
  ]
}
#{
  let x = load-model-example("KColoring")
  let nv = graph-num-vertices(x.instance)
  let k = x.instance.num_colors
  // Pick optimal config = [0,1,1,0,2] to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let coloring = sol.config
  // Group vertices by color (1-indexed in display)
  let color-groups = range(k).map(c => coloring.enumerate().filter(((i, v)) => v == c).map(((i, _)) => i))
  [
    #problem-def("KColoring")[
      Given $G = (V, E)$ and $k$ colors, find $c: V -> {1, ..., k}$ minimizing $|{(u, v) in E : c(u) = c(v)}|$.
    ][
    Graph coloring arises in register allocation, frequency assignment, and scheduling @garey1979. Deciding $k$-colorability is NP-complete for $k >= 3$ but solvable in $O(n+m)$ for $k=2$ via bipartiteness testing. For $k = 3$, the best known algorithm runs in $O^*(1.3289^n)$ @beigel2005; for $k = 4$ in $O^*(1.7159^n)$ @wu2024; for $k = 5$ in $O^*((2-epsilon)^n)$ @zamir2021. In general, inclusion-exclusion achieves $O^*(2^n)$ @bjorklund2009.

    *Example.* Consider the house graph $G$ with $k = #k$ colors. The coloring #range(nv).map(i => $c(v_#i) = #(coloring.at(i) + 1)$).join(", ") is proper: no adjacent pair shares a color, so the number of conflicts is 0. The house graph has chromatic number $chi(G) = #k$ because the triangle $(v_2, v_3, v_4)$ requires #k colors.

    #figure({
      let hg = house-graph()
      draw-node-colors(hg.vertices, hg.edges, coloring)
    },
    caption: [A proper #{k}-coloring of the house graph. Colors: #color-groups.enumerate().map(((c, verts)) => $#verts.map(i => $c(v_#i)$).join($=$) = #(c + 1)$).join(", "). Zero conflicts.],
    ) <fig:house-coloring>
    ]
  ]
}
#{
  let x = load-model-example("MinimumDominatingSet")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges
  // Pick optimal config = {v2, v3} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let S = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let wS = sol.metric.Valid
  // Compute neighbors dominated by each vertex in S
  let dominated = S.map(s => {
    let nbrs = ()
    for (u, v) in edges {
      if u == s and v not in S { nbrs.push(v) }
      if v == s and u not in S { nbrs.push(u) }
    }
    nbrs
  })
  [
    #problem-def("MinimumDominatingSet")[
      Given $G = (V, E)$ with weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ s.t. $forall v in V: v in S or exists u in S: (u, v) in E$.
    ][
    Dominating Set models facility location: each vertex in $S$ "covers" itself and its neighbors. Applications include wireless sensor placement and social network influence maximization. W[2]-complete when parameterized by solution size $k$, making it strictly harder than Vertex Cover in the parameterized hierarchy. The best known exact algorithm runs in $O^*(1.4969^n)$ via measure-and-conquer @vanrooij2011.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices and unit weights $w(v) = 1$. The set $S = {#S.map(i => $v_#i$).join(", ")}$ is a minimum dominating set with $w(S) = #wS$: #S.zip(dominated).map(((s, nbrs)) => [vertex $v_#s$ dominates ${#nbrs.map(i => $v_#i$).join(", ")}$]).join(" and ") (both also dominate each other). No single vertex can dominate all others, so $gamma(G) = #wS$.

    #figure({
      let hg = house-graph()
      draw-node-highlight(hg.vertices, hg.edges, S)
    },
    caption: [The house graph with minimum dominating set $S = {#S.map(i => $v_#i$).join(", ")}$ (blue, $gamma(G) = #wS$). Every white vertex is adjacent to at least one blue vertex.],
    ) <fig:house-ds>
    ]
  ]
}
#{
  let x = load-model-example("MaximumMatching")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  // Pick optimal config [1,0,0,0,1,0] = edges {(0,1),(2,4)} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let matched-edges = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => edges.at(i))
  let wM = sol.metric.Valid
  // Collect matched vertices
  let matched-verts = ()
  for (u, v) in matched-edges {
    if u not in matched-verts { matched-verts.push(u) }
    if v not in matched-verts { matched-verts.push(v) }
  }
  let unmatched = range(nv).filter(i => i not in matched-verts)
  [
    #problem-def("MaximumMatching")[
      Given $G = (V, E)$ with weights $w: E -> RR$, find $M subset.eq E$ maximizing $sum_(e in M) w(e)$ s.t. $forall e_1, e_2 in M: e_1 inter e_2 = emptyset$.
    ][
    Unlike most combinatorial optimization problems on general graphs, maximum matching is solvable in polynomial time $O(n^3)$ by Edmonds' blossom algorithm @edmonds1965, which introduced the technique of shrinking odd cycles into pseudo-nodes. Matching theory underpins assignment problems, network flows, and the Tutte-Berge formula for matching deficiency.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, and unit weights $w(e) = 1$. A maximum matching is $M = {#matched-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$ with $w(M) = #wM$. Each matched edge is vertex-disjoint from the others. #if unmatched.len() == 1 [Vertex $v_#(unmatched.at(0))$ is unmatched; since $n$ is odd, no perfect matching exists.] #if unmatched.len() > 1 [Vertices #unmatched.map(i => $v_#i$).join(", ") are unmatched.]

    #figure({
      let hg = house-graph()
      draw-edge-highlight(hg.vertices, hg.edges, matched-edges, matched-verts)
    },
    caption: [The house graph with a maximum matching $M = {#matched-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$ (blue edges, $w(M) = #wM$). Matched vertices shown in blue; #unmatched.map(i => $v_#i$).join(", ") #if unmatched.len() == 1 [is] else [are] unmatched.],
    ) <fig:house-matching>
    ]
  ]
}

#{
  let x = load-model-example("TravelingSalesman")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges
  let ew = x.instance.edge_weights
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let tour-edges = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => edges.at(i))
  let tour-cost = sol.metric.Valid
  // Build ordered tour from tour-edges starting at vertex 0
  let tour-order = (0,)
  let remaining = tour-edges
  for _ in range(nv - 1) {
    let curr = tour-order.last()
    let next-edge = remaining.find(e => e.at(0) == curr or e.at(1) == curr)
    let next-v = if next-edge.at(0) == curr { next-edge.at(1) } else { next-edge.at(0) }
    tour-order.push(next-v)
    remaining = remaining.filter(e => e != next-edge)
  }
  // Format weight list for display
  let weight-labels = edges.map(((u, v)) => {
    let idx = edges.position(e => e == (u, v))
    (u: u, v: v, w: ew.at(idx))
  })
  [
    #problem-def("TravelingSalesman")[
      Given an undirected graph $G=(V,E)$ with edge weights $w: E -> RR$, find an edge set $C subset.eq E$ that forms a cycle visiting every vertex exactly once and minimizes $sum_(e in C) w(e)$.
    ][
    One of the most intensely studied NP-hard problems, with applications in logistics, circuit board drilling, and DNA sequencing. The best known exact algorithm runs in $O^*(2^n)$ time and space via Held-Karp dynamic programming @heldkarp1962. No $O^*((2-epsilon)^n)$ algorithm is known, and improving the exponential space remains open.

    *Example.* Consider the complete graph $K_#nv$ with vertices ${#range(nv).map(i => $v_#i$).join(", ")}$ and edge weights #weight-labels.map(l => $w(v_#(l.u), v_#(l.v)) = #(int(l.w))$).join(", "). The optimal tour is $#tour-order.map(v => $v_#v$).join($arrow$) arrow v_#(tour-order.at(0))$ with cost $#tour-edges.map(((u, v)) => {
      let idx = edges.position(e => e == (u, v) or e == (v, u))
      str(int(ew.at(idx)))
    }).join(" + ") = #tour-cost$.

    #figure({
      let verts = ((0, 0), (1.5, 0), (1.5, 1.5), (0, 1.5))
      let all-edges = ((0,1),(1,2),(2,3),(0,3),(0,2),(1,3))
      let weights = ew.map(w => str(int(w)))
      canvas(length: 1cm, {
        for (idx, (u, v)) in all-edges.enumerate() {
          let on-tour = tour-edges.any(t => (t.at(0) == u and t.at(1) == v) or (t.at(0) == v and t.at(1) == u))
          g-edge(verts.at(u), verts.at(v),
            stroke: if on-tour { 2pt + graph-colors.at(0) } else { 1pt + luma(200) })
          let mx = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
          let my = (verts.at(u).at(1) + verts.at(v).at(1)) / 2
          let dx = if u == 0 and v == 2 { -0.25 } else if u == 1 and v == 3 { 0.25 } else { 0 }
          let dy = if u == 0 and v == 2 { 0.15 } else if u == 1 and v == 3 { 0.15 } else { 0 }
          draw.content((mx + dx, my + dy), text(7pt, fill: luma(80))[#weights.at(idx)])
        }
        for (k, pos) in verts.enumerate() {
          g-node(pos, name: "v" + str(k),
            fill: graph-colors.at(0),
            label: text(fill: white)[$v_#k$])
        }
      })
    },
    caption: [Complete graph $K_#nv$ with weighted edges. The optimal tour $#tour-order.map(v => $v_#v$).join($arrow$) arrow v_#(tour-order.at(0))$ (blue edges) has cost #tour-cost.],
    ) <fig:k4-tsp>
    ]
  ]
}
#{
  let x = load-model-example("SteinerTree")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let weights = x.instance.edge_weights
  let terminals = x.instance.terminals
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let tree-edge-indices = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let tree-edges = tree-edge-indices.map(i => edges.at(i))
  let cost = sol.metric.Valid
  // Steiner vertices: in tree but not terminals
  let tree-verts = tree-edges.map(e => (e.at(0), e.at(1))).fold((), (acc, pair) => {
    let (u, v) = pair
    let acc2 = if acc.contains(u) { acc } else { acc + (u,) }
    if acc2.contains(v) { acc2 } else { acc2 + (v,) }
  })
  let steiner-verts = tree-verts.filter(v => not terminals.contains(v))
  [
    #problem-def("SteinerTree")[
      Given an undirected graph $G = (V, E)$ with edge weights $w: E -> RR_(>= 0)$ and a set of terminal vertices $T subset.eq V$ with $|T| >= 2$, find a tree $S = (V_S, E_S)$ in $G$ such that $T subset.eq V_S$, minimizing $sum_(e in E_S) w(e)$. Vertices in $V_S backslash T$ are called _Steiner vertices_.
    ][
    One of Karp's 21 NP-complete problems @karp1972, foundational in network design with applications in telecommunications backbone routing, VLSI chip interconnect, pipeline planning, and phylogenetic tree construction. When $T = V$, the problem reduces to the minimum spanning tree (polynomial). The NP-hardness arises from choosing which Steiner vertices to include.

    The best known exact algorithm runs in $O^*(3^(|T|) dot n + 2^(|T|) dot n^2)$ time via Dreyfus--Wagner dynamic programming over terminal subsets @dreyfuswagner1971. Byrka _et al._ achieved a $ln(4) + epsilon approx 1.39$-approximation @byrka2013; the classic 2-approximation uses the minimum spanning tree of the terminal distance graph.

    // Find the unique direct terminal-terminal edge (both endpoints in T, not in the optimal tree)
    #let terminal-set = terminals
    #let direct-tt-edges = edges.enumerate().filter(((i, e)) => {
      terminal-set.contains(e.at(0)) and terminal-set.contains(e.at(1)) and not tree-edge-indices.contains(i)
    })
    #let tt-edge = direct-tt-edges.at(0)
    #let tt-idx = tt-edge.at(0)
    #let tt-u = tt-edge.at(1).at(0)
    #let tt-v = tt-edge.at(1).at(1)

    *Example.* Consider $G$ with $n = #nv$ vertices, $m = #ne$ edges, and terminals $T = {#terminals.map(t => $v_#t$).join(", ")}$. The optimal Steiner tree uses edges ${#tree-edges.map(e => $(v_#(e.at(0)), v_#(e.at(1)))$).join(", ")}$ with Steiner vertices ${#steiner-verts.map(v => $v_#v$).join(", ")}$ acting as relay points. The total cost is #tree-edge-indices.map(i => $#(weights.at(i))$).join($+$) $= #cost$. Note the only direct terminal--terminal edge $(v_#tt-u, v_#tt-v)$ has weight #weights.at(tt-idx), equaling the entire Steiner tree cost.

    #figure({
      // Layout: v0 top-left, v1 top-center, v2 top-right, v3 bottom-center, v4 bottom-right
      let verts = ((0, 1.2), (1.2, 1.2), (2.4, 1.2), (1.2, 0), (2.4, 0))
      canvas(length: 1cm, {
        for (idx, (u, v)) in edges.enumerate() {
          let on-tree = tree-edge-indices.contains(idx)
          g-edge(verts.at(u), verts.at(v),
            stroke: if on-tree { 2pt + graph-colors.at(0) } else { 1pt + luma(200) })
          let mx = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
          let my = (verts.at(u).at(1) + verts.at(v).at(1)) / 2
          let dx = if u == 0 and v == 3 { -0.3 } else if u == 2 and v == 3 { 0.3 } else { 0 }
          let dy = if u == 0 and v == 1 { 0.2 } else if u == 1 and v == 2 { 0.2 } else if u == 2 and v == 4 { 0.3 } else { 0 }
          draw.content((mx + dx, my + dy), text(7pt, fill: luma(80))[#weights.at(idx)])
        }
        for (k, pos) in verts.enumerate() {
          let is-terminal = terminals.contains(k)
          g-node(pos, name: "v" + str(k),
            fill: if is-terminal { graph-colors.at(0) } else { white },
            stroke: if is-terminal { none } else { 1pt + graph-colors.at(0) },
            label: text(fill: if is-terminal { white } else { black })[$v_#k$])
        }
      })
    },
    caption: [Steiner tree on #nv vertices with terminals $T = {#terminals.map(t => $v_#t$).join(", ")}$ (filled blue). Steiner vertices #steiner-verts.map(v => $v_#v$).join(", ") (outlined) relay connections. Blue edges form the optimal tree with cost #cost.],
    ) <fig:steiner-tree>
    ]
  ]
}
#{
  let x = load-model-example("StrongConnectivityAugmentation")
  let nv = x.instance.graph.num_vertices
  let ne = x.instance.graph.arcs.len()
  let arcs = x.instance.graph.arcs
  let candidates = x.instance.candidate_arcs
  let bound = x.instance.bound
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let chosen = candidates.enumerate().filter(((i, _)) => sol.config.at(i) == 1).map(((i, arc)) => arc)
  let total-weight = chosen.map(a => a.at(2)).sum()
  let blue = graph-colors.at(0)
  [
    #problem-def("StrongConnectivityAugmentation")[
      Given a directed graph $G = (V, A)$, a set $C subset.eq (V times V backslash A) times ZZ_(> 0)$ of weighted candidate arcs, and a bound $B in ZZ_(>= 0)$, determine whether there exists a subset $C' subset.eq C$ such that $sum_((u, v, w) in C') w <= B$ and the augmented digraph $(V, A union {(u, v) : (u, v, w) in C'})$ is strongly connected.
    ][
    Strong Connectivity Augmentation models network design problems where a partially connected directed communication graph may be repaired by buying additional arcs. Eswaran and Tarjan showed that the unweighted augmentation problem is solvable in linear time, while the weighted variant is substantially harder @eswarantarjan1976. The decision version recorded as ND19 in Garey and Johnson is NP-complete @garey1979. The implementation here uses one binary variable per candidate arc, so brute-force over the candidate set yields a worst-case bound of $O^*(2^m)$ where $m = "num_potential_arcs"$. #footnote[No exact algorithm improving on brute-force is claimed here for the weighted candidate-arc formulation implemented in the codebase.]

    *Example.* The canonical instance has $n = #nv$ vertices, $|A| = #ne$ existing arcs, and bound $B = #bound$. The base graph is the directed path $v_0 -> v_1 -> v_2 -> v_3 -> v_4$ — every vertex can reach those ahead of it, but vertex $v_4$ is a sink with no outgoing arcs. The #candidates.len() candidate arcs with weights are: #candidates.map(a => $w(v_#(a.at(0)), v_#(a.at(1))) = #(a.at(2))$).join(", "). The cheapest single arc that closes the cycle is $(v_4, v_0)$, but its weight $10 > B$ exceeds the budget, so strong connectivity must be achieved via a two-hop return path. The pair #chosen.map(a => $(v_#(a.at(0)), v_#(a.at(1)))$).join(" and ") with weights #chosen.map(a => $#(a.at(2))$).join($+$) $= #total-weight = B$ creates the path $v_4 -> v_1 -> v_0$, making the augmented graph strongly connected at exactly the budget limit. Alternative escape arcs from $v_4$ (to $v_3$ or $v_2$) are equally cheap but land on vertices from which reaching $v_0$ within the remaining budget is impossible.

    #figure({
      let verts = ((0, 0), (1.5, 0), (3.0, 0), (4.5, 0), (6.0, 0))
      let highlighted = chosen.map(a => (a.at(0), a.at(1))).flatten()
      canvas(length: 1cm, {
        // Vertices (drawn first so edges can reference named anchors)
        for (k, pos) in verts.enumerate() {
          g-node(pos, name: "v" + str(k),
            fill: if highlighted.contains(k) { blue.transparentize(65%) } else { white },
            label: [$v_#k$])
        }
        // Base arcs (black, between named nodes)
        for (u, v) in arcs {
          draw.line("v" + str(u), "v" + str(v),
            stroke: 1pt + black,
            mark: (end: "straight", scale: 0.4))
        }
        // Chosen augmenting arcs (blue, curved above the path)
        let r = 0.24
        for (idx, arc) in chosen.enumerate() {
          let (u, v, w) = arc
          let pu = verts.at(u)
          let pv = verts.at(v)
          let rise = 0.7 + 0.3 * calc.abs(u - v)
          let ctrl = ((pu.at(0) + pv.at(0)) / 2, rise)
          // Shorten start toward control point
          let dx-s = ctrl.at(0) - pu.at(0)
          let dy-s = ctrl.at(1) - pu.at(1)
          let ds = calc.sqrt(dx-s * dx-s + dy-s * dy-s)
          let p0 = (pu.at(0) + r * dx-s / ds, pu.at(1) + r * dy-s / ds)
          // Shorten end toward control point
          let dx-e = ctrl.at(0) - pv.at(0)
          let dy-e = ctrl.at(1) - pv.at(1)
          let de = calc.sqrt(dx-e * dx-e + dy-e * dy-e)
          let p1 = (pv.at(0) + r * dx-e / de, pv.at(1) + r * dy-e / de)
          draw.bezier(p0, p1, ctrl,
            stroke: 1.6pt + blue,
            mark: (end: "straight", scale: 0.5),
          )
          // Weight label
          draw.content(
            ((pu.at(0) + pv.at(0)) / 2, rise + 0.3),
            text(7pt, fill: blue)[$#w$],
          )
        }
      })
    },
    caption: [Strong Connectivity Augmentation on a #{nv}-vertex path digraph. Black arcs form the base path $A$; blue arcs are the unique augmentation (#chosen.map(a => $(v_#(a.at(0)), v_#(a.at(1)))$).join(", ")) with total weight $#total-weight = B = #bound$.],
    ) <fig:strong-connectivity-augmentation>
    ]
  ]
}
#{
  let x = load-model-example("MinimumMultiwayCut")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let weights = x.instance.edge_weights
  let terminals = x.instance.terminals
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let cut-edge-indices = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let cut-edges = cut-edge-indices.map(i => edges.at(i))
  let cost = sol.metric.Valid
  [
    #problem-def("MinimumMultiwayCut")[
      Given an undirected graph $G=(V,E)$ with edge weights $w: E -> RR_(>0)$ and a set of $k$ terminal vertices $T = {t_1, ..., t_k} subset.eq V$, find a minimum-weight set of edges $C subset.eq E$ such that no two terminals remain in the same connected component of $G' = (V, E backslash C)$.
    ][
    The Minimum Multiway Cut problem generalizes the classical minimum $s$-$t$ cut: for $k=2$ it reduces to max-flow and is solvable in polynomial time, but for $k >= 3$ on general graphs it becomes NP-hard @dahlhaus1994. The problem arises in VLSI design, image segmentation, and network design. A $(2 - 2 slash k)$-approximation is achievable in polynomial time by taking the union of the $k - 1$ cheapest isolating cuts @dahlhaus1994. The best known exact algorithm runs in $O^*(1.84^k)$ time (suppressing polynomial factors) via submodular functions on isolating cuts @cao2013.

    *Example.* Consider a graph with $n = #nv$ vertices, $m = #ne$ edges, and $k = #terminals.len()$ terminals $T = {#terminals.map(t => $#t$).join(", ")}$, with edge weights #edges.zip(weights).map(((e, w)) => $w(#(e.at(0)), #(e.at(1))) = #w$).join(", "). The optimal multiway cut removes edges ${#cut-edges.map(e => $(#(e.at(0)), #(e.at(1)))$).join(", ")}$ with total weight #cut-edge-indices.map(i => $#(weights.at(i))$).join($+$) $= #cost$, placing each terminal in a distinct component.

    #figure({
      let verts = ((0, 0.8), (1.2, 1.5), (2.4, 0.8), (1.8, -0.2), (0.6, -0.2))
      canvas(length: 1cm, {
        for (idx, (u, v)) in edges.enumerate() {
          let is-cut = cut-edge-indices.contains(idx)
          g-edge(verts.at(u), verts.at(v),
            stroke: if is-cut { (paint: red, thickness: 2pt, dash: "dashed") } else { 1pt + luma(120) })
          let mx = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
          let my = (verts.at(u).at(1) + verts.at(v).at(1)) / 2
          let dy = if idx == 5 { 0.15 } else { 0 }
          draw.content((mx, my + dy), text(7pt, fill: luma(80))[#weights.at(idx)])
        }
        for (k, pos) in verts.enumerate() {
          let is-terminal = terminals.contains(k)
          g-node(pos, name: "v" + str(k),
            fill: if is-terminal { graph-colors.at(0) } else { luma(180) },
            label: text(fill: white)[$#k$])
        }
      })
    },
    caption: [Minimum Multiway Cut with terminals ${#terminals.map(t => $#t$).join(", ")}$ (blue). Dashed red edges form the optimal cut (weight #cost).],
    ) <fig:multiway-cut>
    ]
  ]
}
#{
  let x = load-model-example("OptimalLinearArrangement")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let K = x.instance.bound
  let config = x.optimal_config
  // Compute total cost
  let total-cost = edges.map(e => calc.abs(config.at(e.at(0)) - config.at(e.at(1)))).sum()
  [
    #problem-def("OptimalLinearArrangement")[
      Given an undirected graph $G=(V,E)$ and a non-negative integer $K$, is there a bijection $f: V -> {0, 1, dots, |V|-1}$ such that $sum_({u,v} in E) |f(u) - f(v)| <= K$?
    ][
      A classical NP-complete decision problem from Garey & Johnson (GT42) @garey1979, with applications in VLSI design, graph drawing, and sparse matrix reordering. The problem asks whether vertices can be placed on a line so that the total "stretch" of all edges is at most $K$.

      NP-completeness was established by Garey, Johnson, and Stockmeyer @gareyJohnsonStockmeyer1976, via reduction from Simple Max Cut. The problem remains NP-complete on bipartite graphs, but is solvable in polynomial time on trees. The best known exact algorithm for general graphs uses dynamic programming over subsets in $O^*(2^n)$ time and space (Held-Karp style), analogous to TSP.

      *Example.* Consider a graph with #nv vertices and #ne edges, with bound $K = #K$. The arrangement $f = (#config.map(c => str(c)).join(", "))$ gives total cost $#edges.map(e => $|#config.at(e.at(0)) - #config.at(e.at(1))|$).join($+$) = #total-cost lt.eq #K$, so this is a YES instance.
    ]
  ]
}
#{
  let x = load-model-example("KClique")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let k = x.instance.k
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let K = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let clique-edges = edges.filter(e => K.contains(e.at(0)) and K.contains(e.at(1)))
  [
    #problem-def("KClique")[
      Given an undirected graph $G = (V, E)$ and an integer $k$, determine whether there exists a subset $K subset.eq V$ with $|K| >= k$ such that every pair of distinct vertices in $K$ is adjacent.
    ][
    $k$-Clique is the classical decision version of Clique, one of Karp's original NP-complete problems @karp1972 and listed as GT19 in Garey and Johnson @garey1979. Unlike Maximum Clique, the threshold $k$ is part of the input, so this formulation is the natural target for decision-to-decision reductions such as $3$SAT $arrow.r$ Clique. The best known exact algorithm matches Maximum Clique via the complement reduction to Maximum Independent Set and runs in $O^*(1.1996^n)$ @xiao2017.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, and threshold $k = #k$. The set $K = {#K.map(i => $v_#i$).join(", ")}$ is a valid witness because all three pairs #clique-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ") are edges, so $|K| = 3 >= #k$ and this is a YES instance. This witness is unique, and no $4$-clique exists because every vertex outside $K$ misses at least one edge to the other selected vertices.

    #figure({
      let hg = house-graph()
      draw-edge-highlight(hg.vertices, hg.edges, clique-edges, K)
    },
    caption: [The house graph with satisfying witness $K = {#K.map(i => $v_#i$).join(", ")}$ for $k = #k$. The selected vertices and their internal clique edges are highlighted in blue.],
    ) <fig:house-kclique>
    ]
  ]
}
#{
  let x = load-model-example("MaximumClique")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  // optimal config = {v2, v3, v4}
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let K = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let omega = sol.metric.Valid
  // Edges within the clique
  let clique-edges = edges.filter(e => K.contains(e.at(0)) and K.contains(e.at(1)))
  [
    #problem-def("MaximumClique")[
      Given $G = (V, E)$, find $K subset.eq V$ maximizing $|K|$ such that all pairs in $K$ are adjacent: $forall u, v in K: (u, v) in E$. Equivalent to MIS on the complement graph $overline(G)$.
    ][
    Maximum Clique arises in social network analysis (finding tightly-connected communities), bioinformatics (protein interaction clusters), and coding theory. The problem is equivalent to Maximum Independent Set on the complement graph $overline(G)$. The best known algorithm runs in $O^*(1.1996^n)$ via the complement reduction to MIS @xiao2017. Robson's direct backtracking algorithm achieves $O^*(1.1888^n)$ using exponential space @robson2001.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices and $|E| = #ne$ edges. The triangle $K = {#K.map(i => $v_#i$).join(", ")}$ is a maximum clique of size $omega(G) = #omega$: all three pairs #clique-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ") are edges. No #(omega + 1)-clique exists because vertices $v_0$ and $v_1$ each have degree 2 and are not adjacent to all of ${#K.map(i => $v_#i$).join(", ")}$.

    #figure({
      let hg = house-graph()
      draw-edge-highlight(hg.vertices, hg.edges, clique-edges, K)
    },
    caption: [The house graph with maximum clique $K = {#K.map(i => $v_#i$).join(", ")}$ (blue, $omega(G) = #omega$). All edges within the clique are shown in bold blue.],
    ) <fig:house-clique>
    ]
  ]
}
#{
  let x = load-model-example("MaximalIS")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  // optimal config = {v0,v2,v4} with w=3 (maximum-weight maximal IS)
  let opt = (config: x.optimal_config, metric: x.optimal_value)
  let S-opt = opt.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let w-opt = opt.metric.Valid
  // Suboptimal maximal IS {v1,v3} with w=2 (hardcoded — no longer in fixture)
  let S-sub = (1, 3)
  let w-sub = 2
  [
    #problem-def("MaximalIS")[
      Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing $sum_(v in S) w(v)$ such that $S$ is independent ($forall u, v in S: (u, v) in.not E$) and maximal (no vertex $u in V backslash S$ can be added to $S$ while maintaining independence).
    ][
    The maximality constraint (no vertex can be added) distinguishes this from MIS, which only requires maximum weight. Every maximum independent set is maximal, but not vice versa. The enumeration bound of $O^*(3^(n slash 3))$ for listing all maximal independent sets @tomita2006 is tight: Moon and Moser @moonmoser1965 showed every $n$-vertex graph has at most $3^(n slash 3)$ maximal independent sets, achieved by disjoint triangles.

    *Example.* Consider the path graph $P_#nv$ with $n = #nv$ vertices, edges $(v_i, v_(i+1))$ for $i = 0, ..., #(ne - 1)$, and unit weights $w(v) = 1$. The set $S = {#S-sub.map(i => $v_#i$).join(", ")}$ is a maximal independent set: no two vertices in $S$ are adjacent, and neither $v_0$ (adjacent to $v_1$), $v_2$ (adjacent to both), nor $v_4$ (adjacent to $v_3$) can be added. However, $S' = {#S-opt.map(i => $v_#i$).join(", ")}$ with $w(S') = #w-opt$ is a strictly larger maximal IS, illustrating that maximality does not imply maximum weight.

    #figure({
      draw-node-highlight(((0, 0), (1, 0), (2, 0), (3, 0), (4, 0)), edges, S-sub)
    },
    caption: [Path $P_#nv$ with maximal IS $S = {#S-sub.map(i => $v_#i$).join(", ")}$ (blue, $w(S) = #w-sub$). $S$ is maximal --- no white vertex can be added --- but not maximum: ${#S-opt.map(i => $v_#i$).join(", ")}$ achieves $w = #w-opt$.],
    ) <fig:path-maximal-is>
    ]
  ]
}

#{
  let x = load-model-example("MinimumFeedbackVertexSet")
  let nv = graph-num-vertices(x.instance)
  let ne = x.instance.graph.arcs.len()
  let arcs = x.instance.graph.arcs
  // Pick optimal config = {v0} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let S = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let wS = sol.metric.Valid
  [
    #problem-def("MinimumFeedbackVertexSet")[
      Given a directed graph $G = (V, A)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ such that the induced subgraph $G[V backslash S]$ is a directed acyclic graph (DAG).
    ][
    One of Karp's 21 NP-complete problems ("Feedback Node Set") @karp1972. Applications include deadlock detection in operating systems, loop breaking in circuit design, and Bayesian network structure learning. The directed version is strictly harder than undirected FVS: the best known exact algorithm runs in $O^*(1.9977^n)$ @razgon2007, compared to $O^*(1.7548^n)$ for undirected graphs. An $O(log n dot log log n)$-approximation exists @even1998.

    *Example.* Consider the directed graph $G$ with $n = #nv$ vertices, $|A| = #ne$ arcs, and unit weights. The arcs form two overlapping directed cycles: $C_1 = v_0 -> v_1 -> v_2 -> v_0$ and $C_2 = v_0 -> v_3 -> v_4 -> v_1$. The set $S = {#S.map(i => $v_#i$).join(", ")}$ with $w(S) = #wS$ is a minimum feedback vertex set: removing $v_#(S.at(0))$ breaks both cycles, leaving a DAG with topological order $(v_3, v_4, v_1, v_2)$. No 0-vertex set suffices since $C_1$ and $C_2$ overlap only at $v_0$ and $v_1$, and removing $v_1$ alone leaves $C_1' = v_0 -> v_3 -> v_4 -> v_1 -> v_2 -> v_0$.

    #figure({
      let verts = ((0, 1), (2, 1), (1, 0), (-0.5, -0.2), (0.8, -0.5))
      canvas(length: 1cm, {
        for (u, v) in arcs {
          draw.line(verts.at(u), verts.at(v),
            stroke: 1pt + black,
            mark: (end: "straight", scale: 0.4))
        }
        for (k, pos) in verts.enumerate() {
          let s = S.contains(k)
          g-node(pos, name: "v" + str(k),
            fill: if s { graph-colors.at(0) } else { white },
            label: if s { text(fill: white)[$v_#k$] } else { [$v_#k$] })
        }
      })
    },
    caption: [A directed graph with FVS $S = {#S.map(i => $v_#i$).join(", ")}$ (blue, $w(S) = #wS$). Removing $v_#(S.at(0))$ breaks both directed cycles $v_0 -> v_1 -> v_2 -> v_0$ and $v_0 -> v_3 -> v_4 -> v_1$, leaving a DAG.],
    ) <fig:fvs-example>
    ]
  ]
}

#problem-def("PartitionIntoPathsOfLength2")[
  Given $G = (V, E)$ with $|V| = 3q$, determine if $V$ can be partitioned into $q$ disjoint sets $V_1, ..., V_q$ of three vertices each, such that each $V_t$ induces at least two edges in $G$.
][
A classical NP-complete problem from Garey and Johnson @garey1979[Ch.~3, p.~76], proved hard by reduction from 3-Dimensional Matching. Each triple in the partition must form a path of length 2 (exactly two edges, i.e., a $P_3$ subgraph) or a triangle (all three edges). The problem models constrained grouping scenarios where cluster connectivity is required. The best known exact approach uses subset DP in $O^*(3^n)$ time.

*Example.* Consider the graph $G$ with $n = 9$ vertices and edges ${0,1}, {1,2}, {3,4}, {4,5}, {6,7}, {7,8}$ (plus cross-edges ${0,3}, {2,5}, {3,6}, {5,8}$). Setting $q = 3$, the partition $V_1 = {0,1,2}$, $V_2 = {3,4,5}$, $V_3 = {6,7,8}$ is valid: $V_1$ contains edges ${0,1}, {1,2}$ (path $0 dash.em 1 dash.em 2$), $V_2$ contains ${3,4}, {4,5}$, and $V_3$ contains ${6,7}, {7,8}$.
]

#{
  let x = load-model-example("SteinerTreeInGraphs")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges
  let ne = edges.len()
  let terminals = x.instance.terminals
  let weights = x.instance.edge_weights
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let opt-weight = sol.metric.Valid
  // Derive tree edges from optimal config
  let tree-edge-indices = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let tree-edges = tree-edge-indices.map(i => edges.at(i))
  // Steiner vertices: non-terminal vertices that appear in tree edges
  let steiner-verts = range(nv).filter(v => not terminals.contains(v) and tree-edges.any(e => e.at(0) == v or e.at(1) == v))
  [
    #problem-def("SteinerTreeInGraphs")[
      Given an undirected graph $G = (V, E)$ with edge weights $w: E -> RR_(>= 0)$ and a set of terminal vertices $R subset.eq V$, find a subtree $T$ of $G$ that spans all terminals in $R$ and minimizes the total edge weight $sum_(e in T) w(e)$.
    ][
    A classical NP-complete problem from Karp's list (as "Steiner Tree in Graphs," Garey & Johnson ND12) @karp1972. Central to network design, VLSI layout, and phylogenetic reconstruction. The problem generalizes minimum spanning tree (where $R = V$) and shortest path (where $|R| = 2$). The Dreyfus--Wagner dynamic programming algorithm @dreyfuswagner1971 solves it in $O(3^k dot n + 2^k dot n^2 + n^3)$ time, where $k = |R|$ and $n = |V|$. Bjorklund et al. @bjorklund2007 achieved $O^*(2^k)$ using subset convolution over the Mobius algebra, and Nederlof @nederlof2009 gave an $O^*(2^k)$ polynomial-space algorithm.

    *Example.* Consider a graph $G$ with $n = #nv$ vertices and $|E| = #ne$ edges. The terminals are $R = {#terminals.map(i => $v_#i$).join(", ")}$ (blue). The optimal Steiner tree uses Steiner vertex #steiner-verts.map(i => $v_#i$).join(", ") (gray, dashed border) and edges #tree-edges.map(e => [$\{v_#(e.at(0)), v_#(e.at(1))\}$]).join(", ") with total weight #tree-edge-indices.map(i => str(weights.at(i))).join(" + ") $= #opt-weight$.

    #figure({
      // Graph: 6 vertices arranged in two rows (layout positions)
      let verts = ((0, 1), (1.5, 1), (3, 1), (1.5, -0.5), (3, -0.5), (4.5, 0.25))
      canvas(length: 1cm, {
        // Draw edges
        for (idx, (u, v)) in edges.enumerate() {
          let on-tree = tree-edges.any(t => (t.at(0) == u and t.at(1) == v) or (t.at(0) == v and t.at(1) == u))
          g-edge(verts.at(u), verts.at(v),
            stroke: if on-tree { 2pt + graph-colors.at(0) } else { 1pt + luma(200) })
          let mx = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
          let my = (verts.at(u).at(1) + verts.at(v).at(1)) / 2
          draw.content((mx, my), text(7pt, fill: luma(80))[#weights.at(idx)])
        }
        // Draw vertices
        for (k, pos) in verts.enumerate() {
          let is-terminal = terminals.contains(k)
          let is-steiner = steiner-verts.contains(k)
          g-node(pos, name: "v" + str(k),
            fill: if is-terminal { graph-colors.at(0) } else if is-steiner { luma(220) } else { white },
            stroke: if is-steiner { (dash: "dashed", paint: graph-colors.at(0)) } else { 1pt + black },
            label: if is-terminal { text(fill: white)[$v_#k$] } else { [$v_#k$] })
        }
      })
    },
    caption: [Steiner Tree: terminals $R = {#terminals.map(i => $v_#i$).join(", ")}$ (blue), Steiner vertex #steiner-verts.map(i => $v_#i$).join(", ") (dashed). Optimal tree (blue edges) has weight #opt-weight.],
    ) <fig:steiner-tree-example>
    ]
  ]
}

#{
  let x = load-model-example("MinimumSumMulticenter")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges
  let K = x.instance.k
  let opt-cost = x.optimal_value.Valid
  // Pick optimal config = {v2, v5} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let centers = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("MinimumSumMulticenter")[
      Given a graph $G = (V, E)$ with vertex weights $w: V -> ZZ_(>= 0)$, edge lengths $l: E -> ZZ_(>= 0)$, and a positive integer $K <= |V|$, find a set $P subset.eq V$ of $K$ vertices (centers) that minimizes the total weighted distance $sum_(v in V) w(v) dot d(v, P)$, where $d(v, P) = min_(p in P) d(v, p)$ is the shortest-path distance from $v$ to the nearest center in $P$.
    ][
    Also known as the _p-median problem_. This is a classical NP-complete facility location problem from Garey & Johnson (A2 ND51). The goal is to optimally place $K$ service centers (e.g., warehouses, hospitals) to minimize total service cost. NP-completeness was established by Kariv and Hakimi (1979) via transformation from Dominating Set. The problem remains NP-complete even with unit weights and unit edge lengths, but is solvable in polynomial time for fixed $K$ or when $G$ is a tree.

    The best known exact algorithm runs in $O^*(2^n)$ time by brute-force enumeration of all $binom(n, K)$ vertex subsets. Constant-factor approximation algorithms exist: Charikar et al. (1999) gave the first constant-factor result, and the best known ratio is $(2 + epsilon)$ by Cohen-Addad et al. (STOC 2022).

    Variables: $n = |V|$ binary variables, one per vertex. $x_v = 1$ if vertex $v$ is selected as a center. A configuration is valid when exactly $K$ centers are selected and all vertices are reachable from at least one center.

    *Example.* Consider the graph $G$ on #nv vertices with unit weights $w(v) = 1$ and unit edge lengths, edges ${#edges.map(((u, v)) => $(#u, #v)$).join(", ")}$, and $K = #K$. Placing centers at $P = {#centers.map(i => $v_#i$).join(", ")}$ gives distances $d(v_0) = 2$, $d(v_1) = 1$, $d(v_2) = 0$, $d(v_3) = 1$, $d(v_4) = 1$, $d(v_5) = 0$, $d(v_6) = 1$, for a total cost of $2 + 1 + 0 + 1 + 1 + 0 + 1 = #opt-cost$. This is optimal.

    #figure({
      let blue = graph-colors.at(0)
      let gray = luma(200)
      canvas(length: 1cm, {
        import draw: *
        let verts = ((-1.5, 0.8), (0, 1.5), (1.5, 0.8), (1.5, -0.8), (0, -1.5), (-1.5, -0.8), (-2.2, 0))
        for (u, v) in edges {
          g-edge(verts.at(u), verts.at(v), stroke: 1pt + gray)
        }
        for (k, pos) in verts.enumerate() {
          let is-center = centers.any(c => c == k)
          g-node(pos, name: "v" + str(k),
            fill: if is-center { blue } else { white },
            label: if is-center { text(fill: white)[$v_#k$] } else { [$v_#k$] })
        }
      })
    },
    caption: [Minimum Sum Multicenter with $K = #K$ on a #{nv}-vertex graph. Centers #centers.map(i => $v_#i$).join(" and ") (blue) achieve optimal total weighted distance #opt-cost.],
    ) <fig:minimum-sum-multicenter>
    ]
  ]
}

#{
  let x = load-model-example("MinMaxMulticenter")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let K = x.instance.k
  let B = x.instance.bound
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let centers = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("MinMaxMulticenter")[
      Given a graph $G = (V, E)$ with vertex weights $w: V -> ZZ_(>= 0)$, edge lengths $l: E -> ZZ_(>= 0)$, a positive integer $K <= |V|$, and a rational bound $B > 0$, does there exist $S subset.eq V$ with $|S| = K$ such that $max_(v in V) w(v) dot d(v, S) <= B$, where $d(v, S) = min_(s in S) d(v, s)$ is the shortest weighted-path distance from $v$ to the nearest vertex in $S$?
    ][
    Also known as the _vertex p-center problem_ (Garey & Johnson A2 ND50). The goal is to place $K$ facilities so that the worst-case weighted distance from any demand point to its nearest facility is at most $B$. NP-complete even with unit weights and unit edge lengths (Kariv and Hakimi, 1979).

    Closely related to Dominating Set: on unweighted unit-length graphs, a $K$-center with radius $B = 1$ is exactly a dominating set of size $K$. The best known exact algorithm runs in $O^*(1.4969^n)$ via binary search over distance thresholds combined with dominating set computation @vanrooij2011. An optimal 2-approximation exists (Hochbaum and Shmoys, 1985); no $(2 - epsilon)$-approximation is possible unless $P = "NP"$ (Hsu and Nemhauser, 1979).

    Variables: $n = |V|$ binary variables, one per vertex. $x_v = 1$ if vertex $v$ is selected as a center. A configuration is satisfying when exactly $K$ centers are selected and $max_(v in V) w(v) dot d(v, S) <= B$.

    *Example.* Consider the graph $G$ on #nv vertices with unit weights $w(v) = 1$, unit edge lengths, edges ${#edges.map(((u, v)) => $(#u, #v)$).join(", ")}$, $K = #K$, and $B = #B$. Placing centers at $S = {#centers.map(i => $v_#i$).join(", ")}$ gives maximum distance $max_v d(v, S) = 1 <= B$, so this is a feasible solution.

    #figure({
      let blue = graph-colors.at(0)
      let gray = luma(200)
      canvas(length: 1cm, {
        import draw: *
        let verts = ((-1.5, 0.8), (0, 1.5), (1.5, 0.8), (1.5, -0.8), (0, -1.5), (-1.5, -0.8))
        for (u, v) in edges {
          g-edge(verts.at(u), verts.at(v), stroke: 1pt + gray)
        }
        for (k, pos) in verts.enumerate() {
          let is-center = centers.any(c => c == k)
          g-node(pos, name: "v" + str(k),
            fill: if is-center { blue } else { white },
            label: if is-center { text(fill: white)[$v_#k$] } else { [$v_#k$] })
        }
      })
    },
    caption: [Min-Max Multicenter with $K = #K$, $B = #B$ on a #{nv}-vertex graph. Centers #centers.map(i => $v_#i$).join(" and ") (blue) ensure every vertex is within distance $B$ of some center.],
    ) <fig:min-max-multicenter>
    ]
  ]
}

#{
  let x = load-model-example("MultipleCopyFileAllocation")
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let K = x.instance.bound
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let copies = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("MultipleCopyFileAllocation")[
      Given a graph $G = (V, E)$, usage values $u: V -> ZZ_(> 0)$, storage costs $s: V -> ZZ_(> 0)$, and a positive integer $K$, determine whether there exists a subset $V' subset.eq V$ such that
      $sum_(v in V') s(v) + sum_(v in V) u(v) dot d(v, V') <= K,$
      where $d(v, V') = min_(w in V') d_G(v, w)$ is the shortest-path distance from $v$ to the nearest copy vertex.
    ][
    Multiple Copy File Allocation appears in the storage-and-retrieval section of Garey and Johnson (SR6) @garey1979. The model combines two competing costs: each chosen copy vertex incurs a storage charge, while every vertex pays an access cost weighted by its demand and graph distance to the nearest copy. Garey and Johnson record the problem as NP-complete in the strong sense, even when usage and storage costs are uniform @garey1979.

    *Example.* Consider the 6-cycle $C_6$ with uniform usage $u(v) = 10$, uniform storage $s(v) = 1$, and bound $K = #K$. Placing copies at $V' = {#copies.map(i => $v_#i$).join(", ")}$ gives storage cost $1 + 1 + 1 = 3$. The remaining vertices $v_0, v_2, v_4$ are each at distance 1 from the nearest copy, so the access cost is $10 + 10 + 10 = 30$. Thus the total cost is $3 + 30 = 33 <= #K$, so this placement is satisfying. The alternating placement shown below is one symmetric witness.

    #figure({
      let blue = graph-colors.at(0)
      let gray = luma(200)
      canvas(length: 1cm, {
        import draw: *
        let verts = ((0, 1.6), (1.35, 0.8), (1.35, -0.8), (0, -1.6), (-1.35, -0.8), (-1.35, 0.8))
        for (u, v) in edges {
          g-edge(verts.at(u), verts.at(v), stroke: 1pt + gray)
        }
        for (k, pos) in verts.enumerate() {
          let has-copy = copies.any(c => c == k)
          g-node(pos, name: "v" + str(k),
            fill: if has-copy { blue } else { white },
            label: if has-copy { text(fill: white)[$v_#k$] } else { [$v_#k$] })
        }
      })
    },
    caption: [Multiple Copy File Allocation on a 6-cycle. Copy vertices $v_1$, $v_3$, and $v_5$ are shown in blue; every white vertex is one hop from the nearest copy, so the total cost is $33$.],
    ) <fig:multiple-copy-file-allocation>
    ]
  ]
}

== Set Problems

#{
  let x = load-model-example("MaximumSetPacking")
  let sets = x.instance.sets
  let m = sets.len()
  // Compute universe size from all elements
  let all-elems = sets.flatten().dedup()
  let U-size = all-elems.len()
  // Pick optimal config = {S1, S3} (0-indexed: sets 0, 2) to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let selected = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let wP = sol.metric.Valid
  // Format a set as {e1+1, e2+1, ...} (1-indexed)
  let fmt-set(s) = "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  [
    #problem-def("MaximumSetPacking")[
      Given universe $U$, collection $cal(S) = {S_1, ..., S_m}$ with $S_i subset.eq U$, weights $w: cal(S) -> RR$, find $cal(P) subset.eq cal(S)$ maximizing $sum_(S in cal(P)) w(S)$ s.t. $forall S_i, S_j in cal(P): S_i inter S_j = emptyset$.
    ][
    One of Karp's 21 NP-complete problems @karp1972. Generalizes maximum matching (the special case where all sets have size 2, solvable in polynomial time). Applications include resource allocation, VLSI design, and frequency assignment. The optimization version is as hard to approximate as maximum clique. The best known exact algorithm runs in $O^*(2^m)$ by brute-force enumeration over the $m$ sets#footnote[No algorithm improving on brute-force enumeration is known for general weighted set packing.].

    *Example.* Let $U = {1, 2, dots, #U-size}$ and $cal(S) = {#range(m).map(i => $S_#(i + 1)$).join(", ")}$ with #range(m).map(i => $S_#(i + 1) = #fmt-set(sets.at(i))$).join(", "), and unit weights $w(S_i) = 1$. A maximum packing is $cal(P) = {#selected.map(i => $S_#(i + 1)$).join(", ")}$ with $w(cal(P)) = #wP$: $S_#(selected.at(0) + 1) inter S_#(selected.at(1) + 1) = emptyset$. Adding $S_2$ would conflict with both ($S_1 inter S_2 = {2}$, $S_2 inter S_3 = {3}$), and $S_4$ conflicts with $S_3$ ($S_3 inter S_4 = {4}$). The alternative packing ${S_2, S_4}$ also achieves weight #wP.

    #figure(
      canvas(length: 1cm, {
        let elems = range(U-size).map(i => (i, 0))
        // Draw set regions
        for i in range(m) {
          let positions = sets.at(i).map(e => (e, 0))
          let is-selected = selected.contains(i)
          sregion(positions, label: [$S_#(i + 1)$], ..if is-selected { sregion-selected } else { sregion-dimmed })
        }
        for (k, pos) in elems.enumerate() {
          selem(pos, label: [#(k + 1)], fill: black)
        }
      }),
      caption: [Maximum set packing: $cal(P) = {#selected.map(i => $S_#(i + 1)$).join(", ")}$ (blue) are disjoint; #range(m).filter(i => i not in selected).map(i => $S_#(i + 1)$).join(", ") (gray) conflict with the packing.],
    ) <fig:set-packing>
    ]
  ]
}

#{
  let x = load-model-example("MinimumSetCovering")
  let sets = x.instance.sets
  let m = sets.len()
  let U-size = x.instance.universe_size
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let selected = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let wC = sol.metric.Valid
  let fmt-set(s) = "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  [
    #problem-def("MinimumSetCovering")[
      Given universe $U$, collection $cal(S)$ with weights $w: cal(S) -> RR$, find $cal(C) subset.eq cal(S)$ minimizing $sum_(S in cal(C)) w(S)$ s.t. $union.big_(S in cal(C)) S = U$.
    ][
    One of Karp's 21 NP-complete problems @karp1972. Arises in facility location, crew scheduling, and test suite minimization. The greedy algorithm achieves an $O(ln n)$-approximation where $n = |U|$, which is essentially optimal: cannot be approximated within $(1-o(1)) ln n$ unless P = NP. The best known exact algorithm runs in $O^*(2^m)$ by brute-force enumeration over the $m$ sets#footnote[No algorithm improving on brute-force enumeration is known for general weighted set covering.].

    *Example.* Let $U = {1, 2, dots, #U-size}$ and $cal(S) = {#range(m).map(i => $S_#(i + 1)$).join(", ")}$ with #range(m).map(i => $S_#(i + 1) = #fmt-set(sets.at(i))$).join(", "), and unit weights $w(S_i) = 1$. A minimum cover is $cal(C) = {#selected.map(i => $S_#(i + 1)$).join(", ")}$ with $w(cal(C)) = #wC$: $#selected.map(i => $S_#(i + 1)$).join($union$) = {1, 2, dots, #U-size} = U$. No single set covers all of $U$, so at least two sets are required.

    #figure(
      canvas(length: 1cm, {
        let elems = (
          (-1.2, 0.4),
          (-0.5, -0.4),
          (0.3, 0.4),
          (1.0, -0.4),
          (1.7, 0.4),
        )
        sregion((elems.at(0), elems.at(1), elems.at(2)), pad: 0.4, label: [$S_1$], ..if selected.contains(0) { sregion-selected } else { sregion-dimmed })
        sregion((elems.at(1), elems.at(3)), pad: 0.35, label: [$S_2$], ..if selected.contains(1) { sregion-selected } else { sregion-dimmed })
        sregion((elems.at(2), elems.at(3), elems.at(4)), pad: 0.4, label: [$S_3$], ..if selected.contains(2) { sregion-selected } else { sregion-dimmed })
        for (k, pos) in elems.enumerate() {
          selem(pos, label: [#(k + 1)], fill: black)
        }
      }),
      caption: [Minimum set covering: $cal(C) = {#selected.map(i => $S_#(i + 1)$).join(", ")}$ (blue) cover all of $U$; #range(m).filter(i => i not in selected).map(i => $S_#(i + 1)$).join(", ") (gray) #if m - selected.len() == 1 [is] else [are] redundant.],
    ) <fig:set-covering>
    ]
  ]
}

#{
  let x = load-model-example("ConsecutiveSets")
  let m = x.instance.alphabet_size
  let n = x.instance.subsets.len()
  let K = x.instance.bound_k
  let subs = x.instance.subsets
  let sol = x.optimal_config
  let fmt-set(s) = "${" + s.map(e => str(e)).join(", ") + "}$"
  [
    #problem-def("ConsecutiveSets")[
      Given a finite alphabet $Sigma$ of size $m$, a collection $cal(C) = {Sigma_1, Sigma_2, dots, Sigma_n}$ of subsets of $Sigma$, and a positive integer $K$, determine whether there exists a string $w in Sigma^*$ with $|w| lt.eq K$ such that, for each $i$, the elements of $Sigma_i$ occur in a consecutive block of $|Sigma_i|$ symbols of $w$.
    ][
      This problem arises in information retrieval and file organization (SR18 in Garey and Johnson @garey1979). It generalizes the _consecutive ones property_ from binary matrices to a string-based formulation: given subsets of an alphabet, construct the shortest string where each subset's elements appear contiguously. The problem is NP-complete, as shown by #cite(<kou1977>, form: "prose") via reduction from Hamiltonian Path. The circular variant, where blocks may wrap around from the end of $w$ back to its beginning (considering $w w$), is also NP-complete @boothlueker1976. When $K$ equals the number of distinct symbols appearing in the subsets, the problem reduces to testing a binary matrix for the consecutive ones property, which is solvable in linear time using PQ-tree algorithms @boothlueker1976.

      *Example.* Let $Sigma = {0, 1, dots, #(m - 1)}$, $K = #K$, and $cal(C) = {#range(n).map(i => $Sigma_#(i + 1)$).join(", ")}$ with #range(n).map(i => $Sigma_#(i + 1) = #fmt-set(subs.at(i))$).join(", "). A valid string is $w = (#sol.map(e => str(e)).join(", "))$ with $|w| = #sol.len() = K$: $Sigma_1 = {0, 4}$ appears as the block $(0, 4)$ at positions 0--1, $Sigma_2 = {2, 4}$ appears as $(4, 2)$ at positions 1--2, $Sigma_3 = {2, 5}$ appears as $(2, 5)$ at positions 2--3, $Sigma_4 = {1, 5}$ appears as $(5, 1)$ at positions 3--4, and $Sigma_5 = {1, 3}$ appears as $(1, 3)$ at positions 4--5.
    ]
  ]
}

#{
  let x3c = load-model-example("ExactCoverBy3Sets")
  let n = x3c.instance.universe_size
  let q = int(n / 3)
  let subs = x3c.instance.subsets
  let m = subs.len()
  let sol = x3c.optimal_config
  // Format a 0-indexed triple as 1-indexed set notation: {a+1, b+1, c+1}
  let fmt-triple(t) = "${" + t.map(e => str(e + 1)).join(", ") + "}$"
  // Collect indices of selected subsets (1-indexed)
  let selected = sol.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)

  [
    #problem-def("ExactCoverBy3Sets")[
      Given universe $X$ with $|X| = 3q$ and collection $cal(C)$ of 3-element subsets of $X$, does $cal(C)$ contain an exact cover — a subcollection $cal(C)' subset.eq cal(C)$ with $|cal(C)'| = q$ such that every element of $X$ occurs in exactly one member of $cal(C)'$?
    ][
    Shown NP-complete by Karp (1972) via transformation from 3-Dimensional Matching @karp1972. X3C remains NP-complete even when no element appears in more than three subsets, but is solvable in polynomial time when no element appears in more than two subsets. It is one of the most widely used source problems for NP-completeness reductions in Garey & Johnson (A3 SP2), serving as the starting point for proving hardness of problems in scheduling, graph theory, set systems, coding, and number theory. The best known exact algorithm runs in $O^*(2^n)$ via inclusion-exclusion over the $n = |X|$ universe elements; a direct brute-force search over the $m$ subsets gives the weaker $O^*(2^m)$ bound.

    *Example.* Let $X = {1, 2, dots, #n}$ ($q = #q$) and $cal(C) = {S_1, dots, S_#m}$ with #subs.enumerate().map(((i, t)) => $S_#(i + 1) = #fmt-triple(t)$).join(", "). An exact cover is $cal(C)' = {#selected.map(i => $S_#(i + 1)$).join(", ")}$: #selected.map(i => [$S_#(i + 1)$ covers #fmt-triple(subs.at(i))]).join(", "), their union is $X$, and they are pairwise disjoint with $|cal(C)'| = #selected.len() = q$.
    ]
  ]
}

#{
  let x = load-model-example("ComparativeContainment")
  let n = x.instance.universe_size
  let R = x.instance.r_sets
  let S = x.instance.s_sets
  let r-weights = x.instance.r_weights
  let s-weights = x.instance.s_weights
  let selected = x.optimal_config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let satisfiers = ((config: x.optimal_config, metric: x.optimal_value),).map(sol => sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i))
  let contains-selected(family-set) = selected.all(i => family-set.contains(i))
  let r-active = range(R.len()).filter(i => contains-selected(R.at(i)))
  let s-active = range(S.len()).filter(i => contains-selected(S.at(i)))
  let r-total = r-active.map(i => r-weights.at(i)).sum(default: 0)
  let s-total = s-active.map(i => s-weights.at(i)).sum(default: 0)
  let fmt-set(items) = if items.len() == 0 {
    $emptyset$
  } else {
    "${" + items.map(e => str(e + 1)).join(", ") + "}$"
  }
  let left-elems = (
    (-3.1, 0.4),
    (-2.4, -0.4),
    (-1.6, 0.4),
    (-0.9, -0.4),
  )
  let right-elems = (
    (0.9, 0.4),
    (1.6, -0.4),
    (2.4, 0.4),
    (3.1, -0.4),
  )
  [
    #problem-def("ComparativeContainment")[
      Given a finite universe $X$, two set families $cal(R) = {R_1, dots, R_k}$ and $cal(S) = {S_1, dots, S_l}$ over $X$, and positive integer weights $w_R(R_i)$ and $w_S(S_j)$, does there exist a subset $Y subset.eq X$ such that $sum_(Y subset.eq R_i) w_R(R_i) >= sum_(Y subset.eq S_j) w_S(S_j)$?
    ][
    Comparative Containment is the set-system comparison problem SP10 in Garey & Johnson @garey1979. Unlike covering and packing problems, feasibility depends on how the chosen subset $Y$ is nested inside two competing set families: the $cal(R)$ family rewards containment while the $cal(S)$ family penalizes it. The problem remains NP-complete in the unit-weight special case and provides a clean weighted-set comparison primitive for future reduction entries in this catalog.

    A direct exact algorithm enumerates all $2^n$ subsets $Y subset.eq X$ for $n = |X|$ and checks which members of $cal(R)$ and $cal(S)$ contain each candidate. This yields an $O^*(2^n)$ exact algorithm, with the polynomial factor coming from scanning the $k + l$ sets for each subset#footnote[No specialized exact algorithm improving on brute-force enumeration is recorded in the standard references used for this catalog entry.].

    *Example.* Let $X = {1, 2, dots, #n}$, $cal(R) = {#range(R.len()).map(i => $R_#(i + 1)$).join(", ")}$ with #R.enumerate().map(((i, family-set)) => [$R_#(i + 1) = #fmt-set(family-set)$ with $w_R(R_#(i + 1)) = #(r-weights.at(i))$]).join(", "), and $cal(S) = {#range(S.len()).map(i => $S_#(i + 1)$).join(", ")}$ with #S.enumerate().map(((i, family-set)) => [$S_#(i + 1) = #fmt-set(family-set)$ with $w_S(S_#(i + 1)) = #(s-weights.at(i))$]).join(", "). The subset $Y = #fmt-set(selected)$ is satisfying because #r-active.map(i => $R_#(i + 1)$).join(", ") contribute $#r-total$ on the left while #s-active.map(i => $S_#(i + 1)$).join(", ") contribute only $#s-total$ on the right, so $#r-total >= #s-total$. In fact, the satisfying subsets are #satisfiers.map(fmt-set).join(", "), so this instance has exactly #satisfiers.len() satisfying solutions.

    #figure(
      canvas(length: 1cm, {
        import draw: *
        content((-2.0, 1.5), text(8pt)[$cal(R)$])
        content((2.0, 1.5), text(8pt)[$cal(S)$])
        sregion((left-elems.at(0), left-elems.at(1), left-elems.at(2), left-elems.at(3)), pad: 0.5, label: [$R_1$], ..if r-active.contains(0) { sregion-selected } else { sregion-dimmed })
        sregion((left-elems.at(0), left-elems.at(1)), pad: 0.35, label: [$R_2$], ..if r-active.contains(1) { sregion-selected } else { sregion-dimmed })
        sregion((right-elems.at(0), right-elems.at(1), right-elems.at(2), right-elems.at(3)), pad: 0.5, label: [$S_1$], ..if s-active.contains(0) { sregion-selected } else { sregion-dimmed })
        sregion((right-elems.at(2), right-elems.at(3)), pad: 0.35, label: [$S_2$], ..if s-active.contains(1) { sregion-selected } else { sregion-dimmed })
        for (k, pos) in left-elems.enumerate() {
          selem(pos, label: [#(k + 1)], fill: if selected.contains(k) { graph-colors.at(0) } else { black })
        }
        for (k, pos) in right-elems.enumerate() {
          selem(pos, label: [#(k + 1)], fill: if selected.contains(k) { graph-colors.at(0) } else { black })
        }
      }),
      caption: [Comparative containment for $Y = #fmt-set(selected)$: both $R_1$ and $R_2$ contain $Y$, while only $S_1$ does, so the $cal(R)$ side dominates the $cal(S)$ side.]
    ) <fig:comparative-containment>
    ]
  ]
}

#{
  let x = load-model-example("SetBasis")
  let coll = x.instance.collection
  let m = coll.len()
  let U-size = x.instance.universe_size
  let k = x.instance.k
  let sat-count = 1
  let basis = range(k).map(i =>
    range(U-size).filter(j => x.optimal_config.at(i * U-size + j) == 1)
  )
  let fmt-set(s) = "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  [
    #problem-def("SetBasis")[
      Given finite set $S$, collection $cal(C)$ of subsets of $S$, and integer $k$, does there exist a family $cal(B) = {B_1, ..., B_k}$ with each $B_i subset.eq S$ such that for every $C in cal(C)$ there exists $cal(B)_C subset.eq cal(B)$ with $union.big_(B in cal(B)_C) B = C$?
    ][
    The Set Basis problem was shown NP-complete by Stockmeyer @stockmeyer1975setbasis and appears as SP7 in Garey & Johnson @garey1979. It asks for an exact union-based description of a family of sets, unlike Set Cover which only requires covering the underlying universe. Applications include data compression, database schema design, and Boolean function minimization. The library's decision encoding uses $k |S|$ membership bits, so brute-force over those bits gives an $O^*(2^(k |S|))$ exact algorithm#footnote[This is the direct search bound induced by the encoding implemented here; we are not aware of a faster general exact worst-case algorithm for this representation.].

    *Example.* Let $S = {1, 2, 3, 4}$, $k = #k$, and $cal(C) = {#range(m).map(i => $C_#(i + 1)$).join(", ")}$ with #coll.enumerate().map(((i, s)) => $C_#(i + 1) = #fmt-set(s)$).join(", "). The sample basis from the issue is $cal(B) = {#range(k).map(i => $B_#(i + 1)$).join(", ")}$ with #basis.enumerate().map(((i, s)) => $B_#(i + 1) = #fmt-set(s)$).join(", "). Then $C_1 = B_1 union B_2$, $C_2 = B_2 union B_3$, $C_3 = B_1 union B_3$, and $C_4 = B_1 union B_2 union B_3$. The fixture stores one satisfying encoding; other valid encodings exist (e.g., permuting the singleton basis or using the three pair sets $C_1, C_2, C_3$ as a basis).

    #figure(
      canvas(length: 1cm, {
        let elems = ((-0.9, 0.2), (0.0, -0.5), (0.9, 0.2), (1.8, -0.5))
        for i in range(k) {
          let positions = basis.at(i).map(e => elems.at(e))
          sregion(positions, pad: 0.28, label: [$B_#(i + 1)$], ..sregion-selected)
        }
        for (idx, pos) in elems.enumerate() {
          selem(pos, label: [#(idx + 1)], fill: if idx < 3 { black } else { luma(160) })
        }
      }),
      caption: [Set Basis example: the singleton basis $cal(B) = {#range(k).map(i => $B_#(i + 1)$).join(", ")}$ reconstructs every target set in $cal(C)$; element $4$ is unused by the target family.],
    ) <fig:set-basis>
    ]
  ]
}

#{
  let x = load-model-example("PrimeAttributeName")
  let n = x.instance.num_attributes
  let deps = x.instance.dependencies
  let q = x.instance.query_attribute
  let key = x.optimal_config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let num-sat = 2  // candidate keys containing query attribute: {2,3} and {0,3}
  // Format a set as {e0, e1, ...} (0-indexed) — for use in text mode
  let fmt-set(s) = "${" + s.map(e => str(e)).join(", ") + "}$"
  // Format a set for use inside math mode (no $ delimiters)
  let fmt-set-math(s) = "{" + s.map(e => str(e)).join(", ") + "}"
  [
    #problem-def("PrimeAttributeName")[
      Given a set $A = {0, 1, ..., #(n - 1)}$ of attribute names, a collection $F$ of functional dependencies on $A$, and a specified attribute $x in A$, determine whether $x$ is a _prime attribute_ for $chevron.l A, F chevron.r$ --- i.e., whether there exists a candidate key $K$ for $chevron.l A, F chevron.r$ such that $x in K$.

      A _candidate key_ is a minimal subset $K subset.eq A$ whose closure $K^+_F = A$, where the closure $K^+_F$ is the set of all attributes functionally determined by $K$ under $F$.
    ][
    Classical NP-complete problem from relational database theory (Lucchesi and Osborn, 1978; Garey & Johnson SR28). Prime attributes are central to database normalization: Second Normal Form (2NF) requires that no non-prime attribute is partially dependent on any candidate key, and Third Normal Form (3NF) requires that for every non-trivial functional dependency $X arrow Y$, either $X$ is a superkey or $Y$ consists only of prime attributes. The brute-force approach enumerates all $2^n$ subsets of $A$ containing $x$, checking each for the key property; no algorithm significantly improving on this is known for the general problem.

    *Example.* Let $A = {0, 1, ..., #(n - 1)}$ ($n = #n$), query attribute $x = #q$, and $F = {#deps.enumerate().map(((i, d)) => $#fmt-set-math(d.at(0)) arrow #fmt-set-math(d.at(1))$).join(", ")}$. The subset $K = #fmt-set-math(key)$ is a candidate key containing $x = #q$: its closure is $K^+_F = A$ (since $#fmt-set-math(key.sorted()) arrow #fmt-set-math(deps.at(1).at(1))$ by the second FD, yielding all of $A$), and removing either element breaks the superkey property (${#(key.at(0))} arrow.r.not A$ and ${#(key.at(1))} arrow.r.not A$), so $K$ is minimal. Thus attribute #q is prime. There are #num-sat candidate keys containing attribute #q in total.

    #figure(
      canvas(length: 1cm, {
        import draw: *
        // Attribute nodes in two rows
        let positions = (
          (0, 1.2),    // 0: top-left
          (1.5, 1.2),  // 1: top-center
          (3.0, 1.2),  // 2: top-right
          (0, 0),      // 3: bottom-left (query)
          (1.5, 0),    // 4: bottom-center
          (3.0, 0),    // 5: bottom-right
        )
        // Draw attribute nodes
        for (k, pos) in positions.enumerate() {
          let is-key = key.contains(k)
          let is-query = k == q
          g-node(pos, name: "a" + str(k), radius: 0.25,
            fill: if is-key { graph-colors.at(0) } else if is-query { graph-colors.at(1) } else { white },
            label: if is-key or is-query { text(fill: white)[$#k$] } else { [$#k$] })
        }
        // Draw functional dependencies as grouped arrows
        // FD 1: {0,1} -> {2,3,4,5}
        let fd-y-offsets = (0.55, -0.55, -1.15)
        for (fi, (lhs, rhs)) in deps.enumerate() {
          let ly = if fi == 0 { 2.0 } else if fi == 1 { -0.8 } else { 2.5 }
          // Compute LHS and RHS centers
          let lx = lhs.map(a => positions.at(a).at(0)).sum() / lhs.len()
          let rx = rhs.map(a => positions.at(a).at(0)).sum() / rhs.len()
          let mid-x = (lx + rx) / 2
          // Draw arrow from LHS region to RHS region
          let arrow-y = ly
          on-layer(1, {
            content((mid-x, arrow-y),
              text(7pt)[FD#(fi + 1): $#fmt-set-math(lhs) arrow #fmt-set-math(rhs)$],
              fill: white, frame: "rect", padding: 0.06, stroke: none)
          })
        }
      }),
      caption: [Prime Attribute Name instance with $n = #n$ attributes. Candidate key $K = #fmt-set-math(key)$ is highlighted in blue; query attribute $x = #q$ is a member of $K$. The three functional dependencies determine the closure of every subset.],
    ) <fig:prime-attribute-name>
    ]
  ]
}

#{
  let x = load-model-example("MinimumCardinalityKey")
  let n = x.instance.num_attributes
  let deps = x.instance.dependencies
  let m = deps.len()
  let bound = x.instance.bound
  let key-attrs = range(n).filter(i => x.optimal_config.at(i) == 1)
  let fmt-set(s) = "${" + s.map(e => str(e)).join(", ") + "}$"
  let fmt-fd(d) = fmt-set(d.at(0)) + " $arrow.r$ " + fmt-set(d.at(1))
  [
    #problem-def("MinimumCardinalityKey")[
      Given a set $A$ of attribute names, a collection $F$ of functional dependencies (ordered pairs of subsets of $A$), and a positive integer $M$, does there exist a candidate key $K subset.eq A$ with $|K| <= M$, i.e., a minimal subset $K$ such that the closure of $K$ under $F^*$ equals $A$?
    ][
    The Minimum Cardinality Key problem arises in relational database theory, where identifying the smallest candidate key determines the most efficient way to uniquely identify rows in a relation. It was shown NP-complete by Lucchesi and Osborn (1978) @lucchesi1978keys via transformation from Vertex Cover. The problem appears as SR26 in Garey & Johnson (A4) @garey1979. The closure $F^*$ is defined by Armstrong's axioms: reflexivity ($B subset.eq C$ implies $C arrow.r B$), transitivity, and union. The best known exact algorithm is brute-force enumeration of all subsets of $A$, giving $O^*(2^(|A|))$ time#footnote[Lucchesi and Osborn give an output-polynomial algorithm for enumerating all candidate keys, but the number of keys can be exponential.].

    *Example.* Let $A = {0, 1, ..., #(n - 1)}$ ($|A| = #n$) with $M = #bound$ and functional dependencies $F = {#deps.enumerate().map(((i, d)) => fmt-fd(d)).join(", ")}$.
    The candidate key $K = #fmt-set(key-attrs)$ has $|K| = #key-attrs.len() <= #bound$. Its closure: start with ${0, 1}$; apply ${0, 1} arrow.r {2}$ to get ${0, 1, 2}$; apply ${0, 2} arrow.r {3}$ to get ${0, 1, 2, 3}$; apply ${1, 3} arrow.r {4}$ to get ${0, 1, 2, 3, 4}$; apply ${2, 4} arrow.r {5}$ to get $A$. Neither ${0}$ nor ${1}$ alone determines $A$, so $K$ is minimal.
    ]
  ]
}

#{
  let x = load-model-example("TwoDimensionalConsecutiveSets")
  let n = x.instance.alphabet_size
  let subs = x.instance.subsets
  let m = subs.len()
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let config = sol.config
  // Build groups from config: groups.at(g) = list of symbols in group g
  let groups = range(n).map(g => range(n).filter(s => config.at(s) == g))
  // Only non-empty groups
  let nonempty = groups.enumerate().filter(((_, g)) => g.len() > 0)
  let k = nonempty.len()
  let fmt-set(s) = "${" + s.map(e => str(e)).join(", ") + "}$"
  [
    #problem-def("TwoDimensionalConsecutiveSets")[
      Given finite alphabet $Sigma = {0, 1, dots, n - 1}$ and collection $cal(C) = {Sigma_1, dots, Sigma_m}$ of subsets of $Sigma$, determine whether $Sigma$ can be partitioned into disjoint sets $X_1, X_2, dots, X_k$ such that each $X_i$ has at most one element in common with each $Sigma_j$, and for each $Sigma_j in cal(C)$ there is an index $l(j)$ with $Sigma_j subset.eq X_(l(j)) union X_(l(j)+1) union dots.c union X_(l(j)+|Sigma_j|-1)$.
    ][
    This problem generalizes the Consecutive Sets problem (SR18) by requiring not just that each subset's elements appear consecutively in an ordering, but that they be spread across consecutive groups of a partition where each group contributes at most one element per subset. Shown NP-complete by Lipski @lipski1977fct via transformation from Graph 3-Colorability. The problem arises in information storage and retrieval where records must be organized in contiguous blocks. It remains NP-complete if all subsets have at most 5 elements, but is solvable in polynomial time if all subsets have at most 2 elements. The brute-force algorithm assigns each of $n$ symbols to one of up to $n$ groups, giving $O^*(n^n)$ time#footnote[No algorithm improving on brute-force enumeration is known for this problem.].

    *Example.* Let $Sigma = {0, 1, dots, #(n - 1)}$ and $cal(C) = {#range(m).map(i => $Sigma_#(i + 1)$).join(", ")}$ with #subs.enumerate().map(((i, s)) => $Sigma_#(i + 1) = #fmt-set(s)$).join(", "). A valid partition uses $k = #k$ groups: #nonempty.map(((g, elems)) => $X_#(g + 1) = #fmt-set(elems)$).join(", "). Each group intersects every subset in at most one element, and each subset's elements span exactly $|Sigma_j|$ consecutive groups. For instance, $Sigma_1 = {0, 1, 2}$ maps to groups $X_1, X_2, X_3$ (consecutive), and $Sigma_5 = {0, 5}$ maps to groups $X_1, X_2$ (consecutive). Multiple valid partitions exist for this instance, differing only by unused or shifted group labels.

    #figure(
      canvas(length: 1cm, {
        import draw: *
        // Draw groups as labeled columns
        let gw = 1.4
        let gh = 0.45
        for (col, (g, elems)) in nonempty.enumerate() {
          let x0 = col * (gw + 0.3)
          // Group header
          content((x0 + gw / 2, 0.5), $X_#(g + 1)$, anchor: "south")
          // Draw box for the group
          rect((x0, -elems.len() * gh), (x0 + gw, 0),
            stroke: 0.5pt + black, fill: rgb("#e8f0fe"))
          // Elements inside
          for (row, elem) in elems.enumerate() {
            content((x0 + gw / 2, -row * gh - gh / 2), text(size: 9pt, str(elem)))
          }
        }
      }),
      caption: [2-Dimensional Consecutive Sets: partition of $Sigma = {0, dots, 5}$ into #k groups satisfying intersection and consecutiveness constraints for all #m subsets.],
    ) <fig:two-dim-consecutive-sets>
    ]
  ]
}

== Optimization Problems

#{
  let x = load-model-example("SpinGlass")
  let n = spin-num-spins(x.instance)
  let edges = x.instance.graph.edges
  let ne = edges.len()
  // Pick optimal config = (+,-,+,+,-) to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  // Convert config (0=+1, 1=-1) to spin values
  let spins = sol.config.map(v => if v == 0 { 1 } else { -1 })
  let H = sol.metric.Valid
  let spin-str = spins.map(s => if s > 0 { "+" } else { "-" }).join(", ")
  // Count satisfied and frustrated edges
  let sat-count = edges.filter(((u, v)) => spins.at(u) * spins.at(v) < 0).len()
  let frust-count = ne - sat-count
  [
    #problem-def("SpinGlass")[
      Given $n$ spin variables $s_i in {-1, +1}$, pairwise couplings $J_(i j) in RR$, and external fields $h_i in RR$, minimize the Hamiltonian (energy function): $H(bold(s)) = -sum_((i,j)) J_(i j) s_i s_j - sum_i h_i s_i$.
    ][
    The Ising spin glass is the canonical model in statistical mechanics for disordered magnetic systems @barahona1982. Ground-state computation is NP-hard on general interaction graphs but polynomial-time solvable on planar graphs without external field ($h_i = 0$) via reduction to minimum-weight perfect matching. Central to quantum annealing, where hardware natively encodes spin Hamiltonians. The best known general algorithm runs in $O^*(2^n)$ by brute-force enumeration#footnote[On general interaction graphs, no algorithm improving on brute-force enumeration is known.].

    *Example.* Consider $n = #n$ spins on a triangular lattice with uniform antiferromagnetic couplings $J_(i j) = -1$ for all edges and no external field ($h_i = 0$). The Hamiltonian simplifies to $H(bold(s)) = sum_((i,j)) s_i s_j$, which counts parallel pairs minus antiparallel pairs. The lattice contains #ne edges and 3 triangular faces; since each triangle cannot have all three pairs antiparallel, frustration is unavoidable. A ground state is $bold(s) = (#spin-str)$ achieving $H = #H$: #sat-count edges are satisfied (antiparallel) and #frust-count are frustrated (parallel). No configuration can satisfy more than #sat-count of #ne edges.

    #figure(
      canvas(length: 1cm, {
        let h = calc.sqrt(3) / 2
        let pos = ((0, h), (1, h), (2, h), (0.5, 0), (1.5, 0))
        for (u, v) in edges {
          let sat = spins.at(u) * spins.at(v) < 0
          g-edge(pos.at(u), pos.at(v),
            stroke: if sat { 1pt + black } else { (paint: rgb("#cc4444"), thickness: 1.2pt, dash: "dashed") })
        }
        for (k, p) in pos.enumerate() {
          let up = spins.at(k) > 0
          g-node(p, name: "s" + str(k), radius: 0.22,
            fill: if up { graph-colors.at(0) } else { graph-colors.at(1) },
            label: text(fill: white, if up { $+$ } else { $-$ }))
        }
      }),
      caption: [Triangular lattice with $n = #n$ spins and antiferromagnetic couplings ($J = -1$). Ground state $bold(s) = (#spin-str)$ with $H = #H$. Solid edges: satisfied (antiparallel); dashed red: frustrated (parallel).],
    ) <fig:spin-glass>
    ]
  ]
}

#{
  let x = load-model-example("QUBO")
  let n = x.instance.num_vars
  let Q = x.instance.matrix
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let xstar = sol.config
  let fstar = sol.metric.Valid
  // Format the Q matrix as semicolon-separated rows
  let mat-rows = Q.map(row => row.map(v => {
    let vi = int(v)
    if v == vi { str(vi) } else { str(v) }
  }).join(", ")).join("; ")
  // Collect indices where x*_i = 1 (1-indexed)
  let selected = xstar.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => $x_#(i + 1)$)
  let unselected-pairs = ()
  for i in range(n) {
    for j in range(i + 1, n) {
      if Q.at(i).at(j) != 0 and (xstar.at(i) == 0 or xstar.at(j) == 0) {
        unselected-pairs.push($#(int(Q.at(i).at(j))) x_#(i + 1) x_#(j + 1)$)
      }
    }
  }
  [
    #problem-def("QUBO")[
      Given $n$ binary variables $x_i in {0, 1}$, upper-triangular matrix $Q in RR^(n times n)$, minimize $f(bold(x)) = sum_(i=1)^n Q_(i i) x_i + sum_(i < j) Q_(i j) x_i x_j$ (using $x_i^2 = x_i$ for binary variables).
    ][
    Equivalent to the Ising model via the linear substitution $s_i = 2x_i - 1$. The native formulation for quantum annealing hardware (e.g., D-Wave) and a standard target for penalty-method reductions @glover2019. QUBO unifies many combinatorial problems into a single unconstrained binary framework, making it a universal intermediate representation for quantum and classical optimization. The best known general algorithm runs in $O^*(2^n)$ by brute-force enumeration#footnote[QUBO inherits the Ising model's complexity; no algorithm improving on brute-force is known for the general case.].

    *Example.* Consider $n = #n$ with $Q = mat(#mat-rows)$. The objective is $f(bold(x)) = -x_1 - x_2 - x_3 + 2x_1 x_2 + 2x_2 x_3$. Evaluating all $2^#n$ assignments: $f(0,0,0) = 0$, $f(1,0,0) = -1$, $f(0,1,0) = -1$, $f(0,0,1) = -1$, $f(1,1,0) = 0$, $f(0,1,1) = 0$, $f(1,0,1) = -2$, $f(1,1,1) = 1$. The minimum is $f^* = #fstar$ at $bold(x)^* = (#xstar.map(v => str(v)).join(", "))$: selecting #selected.join(" and ") avoids the penalty terms #unselected-pairs.join(" and ").
    ]
  ]
}

#{
  let x = load-model-example("ILP")
  let nv = x.instance.num_vars
  let obj = x.instance.objective
  let constraints = x.instance.constraints
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let xstar = sol.config
  let fstar = sol.metric.Valid
  // Format objective: c1*x1 + c2*x2 + ...
  let fmt-obj = obj.map(((i, c)) => {
    let ci = int(c)
    let sign = if ci < 0 { "-" } else { "+" }
    let abs-c = calc.abs(ci)
    if abs-c == 1 { $#sign x_#(i + 1)$ } else { $#sign #abs-c x_#(i + 1)$ }
  }).join($""$)
  // Format constraint: a1*x1 + a2*x2 <= b
  let fmt-constraint(con) = {
    let lhs = con.terms.map(((i, a)) => {
      let ai = int(a)
      if ai == 1 { $x_#(i + 1)$ } else { $#ai x_#(i + 1)$ }
    }).join($+$)
    let rhs = int(con.rhs)
    $#lhs <= #rhs$
  }
  [
    #problem-def("ILP")[
      Given $n$ variables $bold(x)$ over a domain $cal(D)$ (binary $cal(D) = {0,1}$ or integer $cal(D) = ZZ_(>=0)$), constraint matrix $A in RR^(m times n)$, bounds $bold(b) in RR^m$, and objective $bold(c) in RR^n$, find $bold(x) in cal(D)^n$ minimizing $bold(c)^top bold(x)$ subject to $A bold(x) <= bold(b)$.
    ][
    Integer Linear Programming is a universal modeling framework: virtually every NP-hard combinatorial optimization problem admits an ILP formulation. Relaxing integrality to $bold(x) in RR^n$ yields a linear program solvable in polynomial time, forming the basis of branch-and-bound solvers. When the number of integer variables $n$ is fixed, ILP is solvable in polynomial time by Lenstra's algorithm @lenstra1983 using the geometry of numbers, making it fixed-parameter tractable in $n$. The best known general algorithm achieves $O^*(n^n)$ via an FPT algorithm based on lattice techniques @dadush2012.

    *Example.* Minimize $bold(c)^top bold(x) = #fmt-obj$ subject to #constraints.map(fmt-constraint).join(", "), $#range(nv).map(i => $x_#(i + 1)$).join(",") >= 0$, $bold(x) in ZZ^#nv$. The LP relaxation optimum is $p_1 = (7 slash 3, 8 slash 3) approx (2.33, 2.67)$ with value $approx -27.67$, which is non-integral. Branch-and-bound yields the ILP optimum $bold(x)^* = (#xstar.map(v => str(v)).join(", "))$ with $bold(c)^top bold(x)^* = #fstar$.

#figure(
  canvas(length: 0.8cm, {
    // Axes
    draw.line((-0.3, 0), (5.5, 0), mark: (end: "straight"), stroke: 0.6pt)
    draw.line((0, -0.3), (0, 4.8), mark: (end: "straight"), stroke: 0.6pt)
    draw.content((5.7, -0.15), text(8pt)[$x_1$])
    draw.content((-0.15, 5.0), text(8pt)[$x_2$])
    // Tick marks
    for i in range(1, 6) {
      draw.line((i, -0.08), (i, 0.08), stroke: 0.4pt)
      draw.content((i, -0.35), text(6pt)[#i])
    }
    for i in range(1, 5) {
      draw.line((-0.08, i), (0.08, i), stroke: 0.4pt)
      draw.content((-0.35, i), text(6pt)[#i])
    }
    // Feasible region polygon: (0,0) → (5,0) → (7/3, 8/3) → (0, 4)
    draw.line((0,0), (5,0), (7/3, 8/3), (0, 4), close: true,
      fill: green.lighten(70%), stroke: none)
    // Constraint lines (extending beyond feasible region)
    draw.line((0, 5), (5, 0), stroke: graph-colors.at(0))  // x1 + x2 = 5
    draw.line((0, 4), (5.25, 1), stroke: orange)            // 4x1 + 7x2 = 28
    // Objective function level curve (dashed): -5x1 - 6x2 = -23, i.e. x2 = (23 - 5x1)/6
    draw.line((0, 23/6), (23/5, 0), stroke: (paint: luma(80), dash: "dashed"))
    // Gradient direction arrow
    draw.line((1.5, 2.5), (1.1, 1.9), mark: (end: "straight"), stroke: 1pt + luma(80))
    draw.content((0.7, 1.75), text(6pt, fill: luma(80))[$bold(c)$])
    // Constraint labels
    draw.content((4.3, 1.0), text(6pt, fill: graph-colors.at(0))[$x_1 + x_2 = 5$], anchor: "west")
    draw.content((4.5, 1.7), text(6pt, fill: orange)[$4x_1 + 7x_2 = 28$], anchor: "west")
    draw.content((1.2, 4.3), text(6pt, fill: luma(80))[objective], anchor: "south")
    // Integer lattice points (hollow circles)
    for x1 in range(6) {
      for x2 in range(5) {
        draw.circle((x1, x2), radius: 0.06, fill: none, stroke: 0.4pt + luma(120))
      }
    }
    // LP optimum (fractional, non-integer)
    draw.circle((7/3, 8/3), radius: 0.1, fill: graph-colors.at(1), stroke: none)
    draw.content((7/3 + 0.3, 8/3 + 0.3), text(7pt)[$p_1$])
    // ILP optimum (integer)
    draw.circle((3, 2), radius: 0.1, fill: graph-colors.at(1), stroke: none)
    draw.content((3.3, 2.3), text(7pt)[$bold(x)^*$])
  }),
  caption: [ILP feasible region (green) with constraints $x_1 + x_2 <= 5$ (blue) and $4x_1 + 7x_2 <= 28$ (orange). Hollow circles mark the integer lattice. The LP relaxation optimum $p_1 = (7 slash 3, 8 slash 3)$ is non-integral; the ILP optimum $bold(x)^* = (#xstar.map(v => str(v)).join(", "))$ gives $bold(c)^top bold(x)^* = #fstar$.],
) <fig:ilp-example>
    ]
  ]
}

#{
  let x = load-model-example("QuadraticAssignment")
  let C = x.instance.cost_matrix
  let D = x.instance.distance_matrix
  let n = C.len()
  let m = D.len()
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let fstar = sol.config
  let cost-star = sol.metric.Valid
  // Convert integer matrix to math.mat content
  let to-mat(m) = math.mat(..m.map(row => row.map(v => $#v$)))
  // Compute identity assignment cost
  let id-cost = range(n).fold(0, (acc, i) =>
    range(n).fold(acc, (acc2, j) =>
      if i != j { acc2 + C.at(i).at(j) * D.at(i).at(j) } else { acc2 }
    )
  )
  // Format optimal assignment as 1-indexed
  let fstar-display = fstar.map(v => str(v + 1)).join(", ")
  // Find the highest-flow off-diagonal pair
  let max-flow = 0
  let max-fi = 0
  let max-fj = 0
  for i in range(n) {
    for j in range(i + 1, n) {
      if C.at(i).at(j) > max-flow {
        max-flow = C.at(i).at(j)
        max-fi = i
        max-fj = j
      }
    }
  }
  let assigned-li = fstar.at(max-fi)
  let assigned-lj = fstar.at(max-fj)
  let dist-between = D.at(assigned-li).at(assigned-lj)
  [
    #problem-def("QuadraticAssignment")[
      Given $n$ facilities and $m$ locations ($n <= m$), a flow matrix $C in ZZ^(n times n)$ representing flows between facilities, and a distance matrix $D in ZZ^(m times m)$ representing distances between locations, find an injective assignment $f: {1, dots, n} -> {1, dots, m}$ that minimizes
      $ sum_(i != j) C_(i j) dot D_(f(i), f(j)). $
    ][
    The Quadratic Assignment Problem was introduced by Koopmans and Beckmann (1957) to model the optimal placement of economic activities (facilities) across geographic locations, minimizing total transportation cost weighted by inter-facility flows. It is NP-hard, as shown by Sahni and Gonzalez (1976) via reduction from the Hamiltonian Circuit problem. QAP is widely regarded as one of the hardest combinatorial optimization problems: even moderate instances ($n > 20$) challenge state-of-the-art exact solvers. Best exact approaches use branch-and-bound with Gilmore--Lawler bounds or cutting-plane methods; the best known general algorithm runs in $O^*(n!)$ by exhaustive enumeration of all permutations#footnote[No algorithm significantly improving on brute-force permutation enumeration is known for general QAP.].

    Applications include facility layout planning, keyboard and control panel design, scheduling, VLSI placement, and hospital floor planning. As a special case, when $D$ is a distance matrix on a line (i.e., $D_(k l) = |k - l|$), QAP reduces to the Optimal Linear Arrangement problem.

    *Example.* Consider $n = m = #n$ with flow matrix $C$ and distance matrix $D$:
    $ C = #to-mat(C), quad D = #to-mat(D). $
    The identity assignment $f(i) = i$ gives cost #id-cost. The optimal assignment is $f^* = (#fstar-display)$ with cost #cost-star: it places the heavily interacting facilities $F_#(max-fi + 1)$ and $F_#(max-fj + 1)$ (highest flow $= #max-flow$) at locations $L_#(assigned-li + 1)$ and $L_#(assigned-lj + 1)$ (distance $= #dist-between$).

    #figure(
      canvas(length: 1cm, {
        import draw: *
        let fac-x = 0
        let loc-x = 5
        let ys = range(n).rev()
        // Draw facility nodes
        for i in range(n) {
          circle((fac-x, ys.at(i)), radius: 0.3, fill: graph-colors.at(0), stroke: 0.8pt + graph-colors.at(0), name: "f" + str(i))
          content("f" + str(i), text(fill: white, 8pt)[$F_#(i+1)$])
        }
        // Draw location nodes
        for j in range(m) {
          circle((loc-x, ys.at(j)), radius: 0.3, fill: graph-colors.at(1), stroke: 0.8pt + graph-colors.at(1), name: "l" + str(j))
          content("l" + str(j), text(fill: white, 8pt)[$L_#(j+1)$])
        }
        content((fac-x, n - 0.3), text(9pt, weight: "bold")[Facilities])
        content((loc-x, m - 0.3), text(9pt, weight: "bold")[Locations])
        // Draw optimal assignment arrows
        for (fi, li) in fstar.enumerate() {
          line("f" + str(fi) + ".east", "l" + str(li) + ".west",
            mark: (end: "straight"), stroke: 1.2pt + luma(80))
        }
        // Highlight highest-flow pair
        on-layer(-1, {
          let y0 = calc.min(ys.at(max-fi), ys.at(max-fj)) - 0.55
          let y1 = calc.max(ys.at(max-fi), ys.at(max-fj)) + 0.55
          rect((-0.55, y0), (0.55, y1),
            fill: graph-colors.at(0).transparentize(92%),
            stroke: (dash: "dashed", paint: graph-colors.at(0).transparentize(50%), thickness: 0.6pt))
        })
        content((fac-x, -0.9), text(6pt, fill: luma(100))[flow$(F_#(max-fi + 1), F_#(max-fj + 1)) = #max-flow$])
      }),
      caption: [Optimal assignment $f^* = (#fstar-display)$ for the $#n times #m$ QAP instance. Facilities (blue, left) are assigned to locations (red, right) by arrows. Facilities $F_#(max-fi + 1)$ and $F_#(max-fj + 1)$ (highest flow $= #max-flow$) are assigned to locations $L_#(assigned-li + 1)$ and $L_#(assigned-lj + 1)$ (distance $= #dist-between$). Total cost $= #cost-star$.],
    ) <fig:qap-example>
    ]
  ]
}

#{
  let x = load-model-example("ClosestVectorProblem")
  let basis = x.instance.basis
  let target = x.instance.target
  let bounds = x.instance.bounds
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let dist = sol.metric.Valid
  // Config encodes offset from lower bound; recover actual integer coordinates
  let coords = sol.config.enumerate().map(((i, v)) => v + bounds.at(i).lower)
  // Compute B*x: sum over j of coords[j] * basis[j]
  let dim = basis.at(0).len()
  let bx = range(dim).map(d => coords.enumerate().fold(0.0, (acc, (j, c)) => acc + c * basis.at(j).at(d)))
  // Format basis vectors
  let fmt-vec(v) = $paren.l #v.map(e => str(e)).join(", ") paren.r^top$
  let dist-rounded = calc.round(dist, digits: 3)
  [
    #problem-def("ClosestVectorProblem")[
      Given a lattice basis $bold(B) in RR^(m times n)$ (columns $bold(b)_1, dots, bold(b)_n in RR^m$ spanning lattice $cal(L)(bold(B)) = {bold(B) bold(x) : bold(x) in ZZ^n}$) and target $bold(t) in RR^m$, find $bold(x) in ZZ^n$ minimizing $norm(bold(B) bold(x) - bold(t))_2$.
    ][
      The Closest Vector Problem is a fundamental lattice problem, proven NP-hard by van Emde Boas @vanemde1981. CVP appears in lattice-based cryptography, coding theory, and integer programming @lenstra1983. Kannan's enumeration algorithm @kannan1987 solves CVP in $n^(O(n))$ time; Micciancio and Voulgaris @micciancio2010 improved this to deterministic $O^*(4^n)$ using Voronoi cell computations, and Aggarwal, Dadush, and Stephens-Davidowitz @aggarwal2015 achieved randomized $O^*(2^n)$.

      *Example.* Consider the 2D lattice with basis #range(basis.len()).map(j => $bold(b)_#(j + 1) = #fmt-vec(basis.at(j))$).join(", ") and target $bold(t) = #fmt-vec(target)$. The lattice points near $bold(t)$ include $bold(B)(1, 0)^top = (2, 0)^top$, $bold(B)(0, 1)^top = (1, 2)^top$, and $bold(B)(#coords.map(c => str(c)).join(","))^top = (#bx.map(v => str(int(v))).join(", "))^top$. The closest is $bold(B)(#coords.map(c => str(c)).join(","))^top = (#bx.map(v => str(int(v))).join(", "))^top$ with distance $norm(bold(B)(#coords.map(c => str(c)).join(","))^top - bold(t))_2 approx #dist-rounded$.

      #figure(
        canvas(length: 0.8cm, {
          import draw: *
          for x1 in range(0, 3) {
            for x2 in range(0, 3) {
              let px = x1 * basis.at(0).at(0) + x2 * basis.at(1).at(0)
              let py = x1 * basis.at(0).at(1) + x2 * basis.at(1).at(1)
              let is-closest = (x1 == coords.at(0) and x2 == coords.at(1))
              let nm = "p" + str(x1) + str(x2)
              circle(
                (px, py),
                radius: if is-closest { 0.15 } else { 0.08 },
                fill: if is-closest { graph-colors.at(0) } else { luma(180) },
                stroke: if is-closest { 0.8pt + graph-colors.at(0) } else { 0.4pt + luma(120) },
                name: nm,
              )
            }
          }
          circle((target.at(0), target.at(1)), radius: 0.1, fill: graph-colors.at(1), stroke: none, name: "target")
          content((rel: (0, -0.45), to: "target"), text(7pt)[$bold(t)$])
          line("target", "p" + str(coords.at(0)) + str(coords.at(1)), stroke: (paint: graph-colors.at(0), thickness: 0.8pt, dash: "dashed"))
          line("p00", "p10", mark: (end: "straight"), stroke: 0.8pt + luma(100), name: "b1")
          content((rel: (0, -0.35), to: "b1.mid"), text(7pt)[$bold(b)_1$])
          line("p00", "p01", mark: (end: "straight"), stroke: 0.8pt + luma(100), name: "b2")
          content((rel: (-0.3, 0), to: "b2.mid"), text(7pt)[$bold(b)_2$])
          content((rel: (0.45, 0.3), to: "p" + str(coords.at(0)) + str(coords.at(1))), text(7pt)[$bold(B)(#coords.map(c => str(c)).join(","))^top$])
        }),
        caption: [2D lattice with basis #range(basis.len()).map(j => $bold(b)_#(j + 1) = #fmt-vec(basis.at(j))$).join(", "). Target $bold(t) = #fmt-vec(target)$ (red) and closest lattice point $bold(B)(#coords.map(c => str(c)).join(","))^top = (#bx.map(v => str(int(v))).join(", "))^top$ (blue). Distance $approx #dist-rounded$.],
      ) <fig:cvp-example>
    ]
  ]
}

== Satisfiability Problems

#{
  let x = load-model-example("Satisfiability")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let clauses = x.instance.clauses
  let sol = (config: x.optimal_config, metric: x.optimal_value)  // pick satisfying assignment
  let assign = sol.config
  // Format a literal: positive l -> x_l, negative l -> not x_{|l|}
  let fmt-lit(l) = if l > 0 { $x_#l$ } else { $not x_#(-l)$ }
  // Format a clause as (l1 or l2 or ...)
  let fmt-clause(c) = $paren.l #c.literals.map(fmt-lit).join($or$) paren.r$
  // Evaluate a literal under assignment: positive l -> assign[l-1], negative l -> 1-assign[|l|-1]
  let eval-lit(l) = if l > 0 { assign.at(l - 1) } else { 1 - assign.at(-l - 1) }
  [
    #problem-def("Satisfiability")[
      Given a CNF formula $phi = and.big_(j=1)^m C_j$ with $m$ clauses over $n$ Boolean variables, where each clause $C_j = or.big_i ell_(j i)$ is a disjunction of literals, find an assignment $bold(x) in {0, 1}^n$ such that $phi(bold(x)) = 1$ (all clauses satisfied).
    ][
    The Boolean Satisfiability Problem (SAT) is the first problem proven NP-complete @cook1971. SAT serves as the foundation of NP-completeness theory: showing a new problem NP-hard typically proceeds by reduction from SAT or one of its variants. Despite worst-case hardness, conflict-driven clause learning (CDCL) solvers handle industrial instances with millions of variables. The Strong Exponential Time Hypothesis (SETH) @impagliazzo2001 conjectures that no $O^*((2-epsilon)^n)$ algorithm exists for general CNF-SAT, and the best known algorithm runs in $O^*(2^n)$ by brute-force enumeration#footnote[SETH conjectures this is optimal; no $O^*((2-epsilon)^n)$ algorithm is known.].

    *Example.* Consider $phi = #clauses.map(fmt-clause).join($and$)$ with $n = #n$ variables and $m = #m$ clauses. The assignment $(#range(n).map(i => $x_#(i + 1)$).join(",") ) = (#assign.map(v => str(v)).join(", "))$ satisfies all clauses: #clauses.enumerate().map(((j, c)) => $C_#(j + 1) = paren.l #c.literals.map(l => str(eval-lit(l))).join($or$) paren.r = 1$).join(", "). Hence $phi(#assign.map(v => str(v)).join(", ")) = 1$.
    ]
  ]
}

#{
  let x = load-model-example("NAESatisfiability")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let clauses = x.instance.clauses
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let assign = sol.config
  let complement = assign.map(v => 1 - v)
  let fmt-lit(l) = if l > 0 { $x_#l$ } else { $not x_#(-l)$ }
  let fmt-clause(c) = $paren.l #c.literals.map(fmt-lit).join($or$) paren.r$
  let eval-lit(l) = if l > 0 { assign.at(l - 1) } else { 1 - assign.at(-l - 1) }
  let clause-values(c) = c.literals.map(l => str(eval-lit(l)))
  [
    #problem-def("NAESatisfiability")[
      Given a CNF formula $phi = and.big_(j=1)^m C_j$ with $m$ clauses over $n$ Boolean variables, where each clause $C_j = or.big_i ell_(j i)$ is a disjunction of literals, find an assignment $bold(x) in {0, 1}^n$ such that every clause contains at least one true literal and at least one false literal.
    ][
    Not-All-Equal Satisfiability (NAE-SAT) is a canonical variant in Schaefer's dichotomy theorem @schaefer1978. Unlike ordinary SAT, each clause forbids the all-true and all-false patterns, giving the problem a complement symmetry: if an assignment is NAE-satisfying, then flipping every bit is also NAE-satisfying. This makes NAE-SAT a natural intermediate for cut and partition reductions such as Max-Cut. A straightforward exact algorithm enumerates all $2^n$ assignments; complement symmetry can halve the search space in practice by fixing one variable, but the asymptotic worst-case bound remains $O^*(2^n)$.

    *Example.* Consider $phi = #clauses.map(fmt-clause).join($and$)$ with $n = #n$ variables and $m = #m$ clauses. The assignment $(#range(n).map(i => $x_#(i + 1)$).join(",")) = (#assign.map(v => str(v)).join(", "))$ is NAE-satisfying because each clause evaluates to a tuple containing both $0$ and $1$: #clauses.enumerate().map(((j, c)) => $C_#(j + 1) = paren.l #clause-values(c).join(", ") paren.r$).join(", "). The complementary assignment $(#range(n).map(i => $x_#(i + 1)$).join(",")) = (#complement.map(v => str(v)).join(", "))$ is therefore also NAE-satisfying, illustrating the paired-solution structure characteristic of NAE-SAT.
    ]
  ]
}

#{
  let x = load-model-example("KSatisfiability")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let k = x.instance.clauses.at(0).literals.len()
  let clauses = x.instance.clauses
  // Pick a satisfying assignment
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let assign = sol.config
  let fmt-lit(l) = if l > 0 { $x_#l$ } else { $not x_#(-l)$ }
  let fmt-clause(c) = $paren.l #c.literals.map(fmt-lit).join($or$) paren.r$
  let eval-lit(l) = if l > 0 { assign.at(l - 1) } else { 1 - assign.at(-l - 1) }
  [
    #problem-def("KSatisfiability")[
      SAT with exactly $k$ literals per clause.
    ][
    The restriction of SAT to exactly $k$ literals per clause reveals a sharp complexity transition: 2-SAT is polynomial-time solvable via implication graph SCC decomposition @aspvall1979 in $O(n+m)$, while $k$-SAT for $k >= 3$ is NP-complete. Random $k$-SAT exhibits a satisfiability threshold at clause density $m slash n approx 2^k ln 2$, a key phenomenon in computational phase transitions. The best known algorithm for 3-SAT runs in $O^*(1.307^n)$ via biased-PPSZ @hansen2019. Under SETH, $k$-SAT requires time $O^*(c_k^n)$ with $c_k -> 2$ as $k -> infinity$.

    *Example.* Consider the #{k}-SAT formula $phi = #clauses.map(fmt-clause).join($and$)$ with $n = #n$ variables and $m = #m$ clauses, each containing exactly #k literals. The assignment $(#range(n).map(i => $x_#(i + 1)$).join(",")) = (#assign.map(v => str(v)).join(", "))$ satisfies all clauses: #clauses.enumerate().map(((j, c)) => $C_#(j + 1) = paren.l #c.literals.map(l => str(eval-lit(l))).join($or$) paren.r = 1$).join(", ").
    ]
  ]
}

#{
  let x = load-model-example("CircuitSAT")
  let vars = x.instance.variables
  let gates = x.instance.circuit.assignments
  let g = gates.len()
  // Input variables are those not produced as gate outputs
  let gate-outputs = gates.map(a => a.outputs).flatten()
  let inputs = vars.filter(v => v not in gate-outputs)
  let n = inputs.len()
  // Find satisfying input assignments: extract input variable positions and group optimal configs
  let input-indices = inputs.map(v => vars.position(u => u == v))
  // Collect unique input assignments from optimal configs
  let sat-assigns = ()
  for o in ((config: x.optimal_config, metric: x.optimal_value),) {
    let ia = input-indices.map(i => o.config.at(i))
    if ia not in sat-assigns { sat-assigns.push(ia) }
  }
  [
    #problem-def("CircuitSAT")[
      Given a Boolean circuit $C$ composed of logic gates (AND, OR, NOT, XOR) with $n$ input variables, find an input assignment $bold(x) in {0,1}^n$ such that $C(bold(x)) = 1$.
    ][
    Circuit Satisfiability is the most natural NP-complete problem: the Cook-Levin theorem @cook1971 proves NP-completeness by showing any nondeterministic polynomial-time computation can be encoded as a Boolean circuit. CircuitSAT is strictly more succinct than CNF-SAT, since a circuit with $g$ gates may require an exponentially larger CNF formula without auxiliary variables. The Tseitin transformation reduces CircuitSAT to CNF-SAT with only $O(g)$ clauses by introducing one auxiliary variable per gate. The best known algorithm runs in $O^*(2^n)$ by brute-force enumeration#footnote[No algorithm improving on brute-force is known for general circuits.].

    *Example.* Consider the circuit $C(x_1, x_2) = (x_1 "AND" x_2) "XOR" (x_1 "OR" x_2)$ with $n = #n$ inputs and $g = #g$ gates. Evaluating: $C(0,0) = (0) "XOR" (0) = 0$, $C(0,1) = (0) "XOR" (1) = 1$, $C(1,0) = (0) "XOR" (1) = 1$, $C(1,1) = (1) "XOR" (1) = 0$. The satisfying assignments are #sat-assigns.map(a => $paren.l #a.map(v => str(v)).join(", ") paren.r$).join(" and ") -- precisely the inputs where exactly one variable is true.

    #figure(
      canvas(length: 1cm, {
        gate-and((2, 0.8), name: "and")
        gate-or((2, -0.8), name: "or")
        gate-xor((4.5, 0), name: "xor")
        draw.line("and.out", (3.5, 0.8), (3.5, 0.175), "xor.in0")
        draw.line("or.out", (3.5, -0.8), (3.5, -0.175), "xor.in1")
        draw.line("xor.out", (5.5, 0), mark: (end: ">"))
        draw.content((5.8, 0), text(8pt)[$C$])
        draw.line((0, 0.975), (0.8, 0.975), "and.in0")
        draw.line((0.8, 0.975), (0.8, -0.625), "or.in0")
        draw.circle((0.8, 0.975), radius: 0.04, fill: black, stroke: none)
        draw.line((0, -0.975), (0.5, -0.975), "or.in1")
        draw.line((0.5, -0.975), (0.5, 0.625), "and.in1")
        draw.circle((0.5, -0.975), radius: 0.04, fill: black, stroke: none)
        draw.content((-0.3, 0.975), text(8pt)[$x_1$])
        draw.content((-0.3, -0.975), text(8pt)[$x_2$])
      }),
      caption: [Circuit $C(x_1, x_2) = (x_1 and x_2) xor (x_1 or x_2)$. Junction dots mark where inputs fork to both gates. Satisfying assignments: #sat-assigns.map(a => $paren.l #a.map(v => str(v)).join(", ") paren.r$).join(" and ").],
    ) <fig:circuit-sat>
    ]
  ]
}

#problem-def("ConjunctiveQueryFoldability")[
  Given a finite domain $D$, relation symbols $R_1, dots, R_m$ with fixed arities $d_1, dots, d_m$, a set $X$ of _distinguished_ variables, a set $Y$ of _undistinguished_ variables (with $X inter Y = emptyset$), and two conjunctive queries $Q_1$ and $Q_2$ — each a set of atoms of the form $R_j (t_1, dots, t_(d_j))$ with $t_i in D union X union Y$ — determine whether there exists a substitution $sigma: Y -> D union X union Y$ such that $sigma(Q_1) = Q_2$ as sets of atoms, where $sigma$ fixes all elements of $D union X$.
][
  Conjunctive query foldability is equivalent to conjunctive query containment and was shown NP-complete by Chandra and Merlin (1977) via reduction from Graph 3-Colorability.#footnote[A. K. Chandra and P. M. Merlin, "Optimal implementation of conjunctive queries in relational data bases," _Proc. 9th ACM STOC_, 1977, pp. 77–90.] If $Q_1$ folds into $Q_2$, then $Q_1$ is subsumed by $Q_2$, making $Q_1$ redundant — a key step in query optimization. The brute-force algorithm enumerates all $|D union X union Y|^(|Y|)$ possible substitutions and checks set equality; no general exact algorithm with a better worst-case bound is known.#footnote[No algorithm improving on brute-force substitution enumeration is known for general conjunctive query foldability.]

  *Example.* Let $D = emptyset$, $X = {x}$, $Y = {u, v, a}$, and $R$ a single binary relation. The query $Q_1 = {R(x, u), R(u, v), R(v, x), R(u, u)}$ is a directed triangle $(x, u, v)$ with a self-loop on $u$. The query $Q_2 = {R(x, a), R(a, a), R(a, x)}$ is a "lollipop": a self-loop on $a$ with edges $x -> a$ and $a -> x$. The substitution $sigma: u |-> a,\ v |-> a,\ a |-> a$ maps $Q_1$ to ${R(x, a), R(a, a), R(a, x), R(a, a)} = Q_2$ (as a set), so $Q_1$ folds into $Q_2$.

  #figure(
    canvas(length: 1cm, {
      import draw: *
      // Q1: triangle (x, u, v) with self-loop on u
      // Place x at top-left, u at bottom-left, v at bottom-right
      let px = (-2.5, 0.6)
      let pu = (-3.2, -0.6)
      let pv = (-1.8, -0.6)
      circle(px, radius: 0.22, fill: white, stroke: 0.6pt, name: "x1")
      content("x1", text(8pt)[$x$])
      circle(pu, radius: 0.22, fill: white, stroke: 0.6pt, name: "u")
      content("u", text(8pt)[$u$])
      circle(pv, radius: 0.22, fill: white, stroke: 0.6pt, name: "v")
      content("v", text(8pt)[$v$])
      // edges: x->u, u->v, v->x
      line("x1.south-west", "u.north", mark: (end: "straight", scale: 0.45))
      line("u.east", "v.west", mark: (end: "straight", scale: 0.45))
      line("v.north-west", "x1.south-east", mark: (end: "straight", scale: 0.45))
      // self-loop on u: arc below u
      arc((-3.2, -0.82), radius: 0.22, start: 200deg, stop: 340deg,
        stroke: 0.6pt, mark: (end: "straight", scale: 0.45))
      // Q1 label
      content((-2.5, -1.4), text(8pt)[$Q_1$])

      // Substitution arrow sigma in the middle
      line((-1.1, 0.0), (-0.3, 0.0), mark: (end: "straight", scale: 0.6))
      content((-0.7, 0.2), text(8pt)[$sigma$])

      // Q2: lollipop — x and a, self-loop on a, edges x->a and a->x
      let qx = (0.8, 0.3)
      let qa = (1.8, -0.5)
      circle(qx, radius: 0.22, fill: white, stroke: 0.6pt, name: "x2")
      content("x2", text(8pt)[$x$])
      circle(qa, radius: 0.22, fill: white, stroke: 0.6pt, name: "a")
      content("a", text(8pt)[$a$])
      // edges: x->a and a->x (use slightly bent anchors)
      line("x2.south-east", "a.north-west", mark: (end: "straight", scale: 0.45))
      line("a.north", (1.8, 0.1), "x2.east", mark: (end: "straight", scale: 0.45))
      // self-loop on a
      arc((1.8, -0.72), radius: 0.22, start: 200deg, stop: 340deg,
        stroke: 0.6pt, mark: (end: "straight", scale: 0.45))
      // Q2 label
      content((1.3, -1.4), text(8pt)[$Q_2$])
    }),
    caption: [Conjunctive Query Foldability example. Left: query $Q_1$ — directed triangle $(x, u, v)$ with self-loop on $u$. Right: query $Q_2$ — lollipop with node $a$ having a self-loop and two edges to $x$. The substitution $sigma: u |-> a, v |-> a$ (with $a |-> a$) folds $Q_1$ into $Q_2$.],
  ) <fig:cqf-example>
]

#{
  let x = load-model-example("Factoring")
  let N = x.instance.target
  let mb = x.instance.m
  let nb = x.instance.n
  let sol = x.optimal_config
  // First mb bits encode p, next nb bits encode q
  let p = range(mb).fold(0, (acc, i) => acc + sol.at(i) * calc.pow(2, i)) + 2
  let q = range(nb).fold(0, (acc, i) => acc + sol.at(mb + i) * calc.pow(2, i)) + 2
  [
    #problem-def("Factoring")[
      Given a composite integer $N$ and bit sizes $m, n$, find integers $p in [2, 2^m - 1]$ and $q in [2, 2^n - 1]$ such that $p times q = N$. Here $p$ has $m$ bits and $q$ has $n$ bits.
    ][
    The hardness of integer factorization underpins RSA cryptography and other public-key systems. Unlike most problems in this collection, Factoring is not known to be NP-complete; it lies in NP $inter$ co-NP, suggesting it may be of intermediate complexity. The best classical algorithm is the General Number Field Sieve @lenstra1993 running in sub-exponential time $e^(O(b^(1 slash 3)(log b)^(2 slash 3)))$ where $b$ is the bit length. Shor's algorithm @shor1994 solves Factoring in polynomial time on a quantum computer.

    *Example.* Let $N = #N$ with $m = #mb$ bits and $n = #nb$ bits, so $p in [2, #(calc.pow(2, mb) - 1)]$ and $q in [2, #(calc.pow(2, nb) - 1)]$. The solution is $p = #p$, $q = #q$, since $#p times #q = #N = N$. Note $p = #p$ fits in #mb bits and $q = #q$ fits in #nb bits. The alternative factorization $#q times #p$ requires $m = #nb$, $n = #mb$.
    ]
  ]
}

#{
  let x = load-model-example("QuantifiedBooleanFormulas")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let clauses = x.instance.clauses
  let quantifiers = x.instance.quantifiers
  let fmt-lit(l) = if l > 0 { $u_#l$ } else { $not u_#(-l)$ }
  let fmt-clause(c) = $paren.l #c.literals.map(fmt-lit).join($or$) paren.r$
  let fmt-quant(q, i) = if q == "Exists" { $exists u_#(i + 1)$ } else { $forall u_#(i + 1)$ }
  [
    #problem-def("QuantifiedBooleanFormulas")[
      Given a set $U = {u_1, dots, u_n}$ of Boolean variables and a fully quantified Boolean formula $F = (Q_1 u_1)(Q_2 u_2) dots.c (Q_n u_n) E$, where each $Q_i in {exists, forall}$ is a quantifier and $E$ is a Boolean expression in CNF with $m$ clauses, determine whether $F$ is true.
    ][
    Quantified Boolean Formulas (QBF) is the canonical PSPACE-complete problem, established by #cite(<stockmeyer1973>, form: "prose"). QBF generalizes SAT by adding universal quantifiers ($forall$) alongside existential ones ($exists$), creating a two-player game semantics: the existential player chooses values for $exists$-variables, the universal player for $forall$-variables, and the formula is true iff the existential player has a winning strategy ensuring all clauses are satisfied. This quantifier alternation is the source of PSPACE-hardness and makes QBF the primary source of PSPACE-completeness reductions for combinatorial game problems. The problem remains PSPACE-complete even when $E$ is restricted to 3-CNF (Quantified 3-SAT), but is polynomial-time solvable when each clause has at most 2 literals @schaefer1978. The best known exact algorithm is brute-force game-tree evaluation in $O^*(2^n)$ time. For QBF with $m$ CNF clauses, #cite(<williams2002>, form: "prose") achieves $O^*(1.709^m)$ time.

    *Example.* Consider $F = #quantifiers.enumerate().map(((i, q)) => fmt-quant(q, i)).join($space$) space #clauses.map(fmt-clause).join($and$)$ with $n = #n$ variables and $m = #m$ clauses. The existential player chooses $u_1 = 1$: then $C_1 = (1 or u_2) = 1$ and $C_2 = (1 or not u_2) = 1$ for any value of $u_2$. Hence $F$ is #x.optimal_value --- the existential player has a winning strategy.
    ]
  ]
}

== Specialized Problems

#{
  let x = load-model-example("BMF")
  let mr = x.instance.m
  let nc = x.instance.n
  let k = x.instance.k
  let A = x.instance.matrix
  let dH = x.optimal_value.Valid
  // Decode B and C from optimal config
  // Config layout: B is m*k values, then C is k*n values
  let cfg = x.optimal_config
  let B = range(mr).map(i => range(k).map(j => cfg.at(i * k + j)))
  let C = range(k).map(i => range(nc).map(j => cfg.at(mr * k + i * nc + j)))
  // Convert A from bool to int for display
  let A-int = A.map(row => row.map(v => if v { 1 } else { 0 }))
  // Format matrix as semicolon-separated rows
  let fmt-mat(m) = m.map(row => row.map(v => str(v)).join(", ")).join("; ")
  [
    #problem-def("BMF")[
      Given an $m times n$ boolean matrix $A$ and rank $k$, find boolean matrices $B in {0,1}^(m times k)$ and $C in {0,1}^(k times n)$ minimizing the Hamming distance $d_H (A, B circle.tiny C)$, where the boolean product $(B circle.tiny C)_(i j) = or.big_ell (B_(i ell) and C_(ell j))$.
    ][
    Boolean Matrix Factorization decomposes binary data into interpretable boolean factors, unlike real-valued SVD which loses the discrete structure. NP-hard even to approximate, BMF arises in data mining, text classification, and role-based access control where factors correspond to latent binary features. Practical algorithms use greedy rank-1 extraction or alternating fixed-point methods. The best known exact algorithm runs in $O^*(2^(m k + k n))$ by brute-force search over $B$ and $C$#footnote[No algorithm improving on brute-force enumeration is known for general BMF.].

    *Example.* Let $A = mat(#fmt-mat(A-int))$ and $k = #k$. Set $B = mat(#fmt-mat(B))$ and $C = mat(#fmt-mat(C))$. Then $B circle.tiny C = mat(#fmt-mat(A-int)) = A$, achieving Hamming distance $d_H = #dH$ (exact factorization). The two boolean factors capture overlapping row/column patterns: factor 1 selects rows ${1, 2}$ and columns ${1, 2}$; factor 2 selects rows ${2, 3}$ and columns ${2, 3}$.

    #figure(
      {
        let cell(val, x, y, color) = {
          let f = if val == 1 { color.transparentize(30%) } else { white }
          box(width: 0.45cm, height: 0.45cm, fill: f, stroke: 0.4pt + luma(180),
            align(center + horizon, text(7pt, if val == 1 { [1] } else { [0] })))
        }
        let mat-grid(data, color) = {
          grid(columns: data.at(0).len(), column-gutter: 0pt, row-gutter: 0pt,
            ..data.flatten().enumerate().map(((i, v)) => {
              cell(v, calc.rem(i, data.at(0).len()), int(i / data.at(0).len()), color)
            }))
        }
        set text(8pt)
        align(center, stack(dir: ltr, spacing: 0.3cm,
          [$A =$], mat-grid(A-int, graph-colors.at(0)),
          [$= B circle.tiny C =$],
          mat-grid(B, graph-colors.at(1)),
          [$circle.tiny$],
          mat-grid(C, rgb("#76b7b2")),
        ))
      },
      caption: [Boolean matrix factorization: $A = B circle.tiny C$ with rank $k = #k$. Factor 1 (red) covers the top-left block; factor 2 (teal) covers the bottom-right block.],
    ) <fig:bmf>
    ]
  ]
}

#{
  let x = load-model-example("ConsecutiveBlockMinimization")
  let mat = x.instance.matrix
  let K = x.instance.bound
  let n-rows = mat.len()
  let n-cols = if n-rows > 0 { mat.at(0).len() } else { 0 }
  let perm = x.optimal_config
  // Count blocks under the satisfying permutation
  let total-blocks = 0
  for row in mat {
    let in-block = false
    for p in perm {
      if row.at(p) {
        if not in-block {
          total-blocks += 1
          in-block = true
        }
      } else {
        in-block = false
      }
    }
  }
  [
    #problem-def("ConsecutiveBlockMinimization")[
      Given an $m times n$ binary matrix $A$ and a positive integer $K$, determine whether there exists a permutation of the columns of $A$ such that the resulting matrix has at most $K$ maximal blocks of consecutive 1-entries (summed over all rows). A _block_ is a maximal contiguous run of 1-entries within a single row.
    ][
    Consecutive Block Minimization (SR17 in Garey & Johnson) arises in consecutive file organization for information retrieval systems, where records stored on a linear medium must be arranged so that each query's relevant records form a contiguous segment. Applications also include scheduling, production planning, the glass cutting industry, and data compression. NP-complete by reduction from Hamiltonian Path @kou1977. When $K$ equals the number of non-all-zero rows, the problem reduces to testing the _consecutive ones property_, solvable in polynomial time via PQ-trees @booth1975. A 1.5-approximation is known @haddadi2008. The best known exact algorithm runs in $O^*(n!)$ by brute-force enumeration of all column permutations.

    *Example.* Let $A$ be the #n-rows$times$#n-cols matrix with rows #mat.enumerate().map(((i, row)) => [$r_#i = (#row.map(v => if v {$1$} else {$0$}).join($,$))$]).join(", ") and $K = #K$. The column permutation $pi = (#perm.map(p => str(p)).join(", "))$ yields #total-blocks total blocks, so #total-blocks $<= #K$ and the answer is YES.
    ]
  ]
}

#{
  let x = load-model-example("PaintShop")
  let n-cars = x.instance.num_cars
  let labels = x.instance.car_labels
  let seq-indices = x.instance.sequence_indices
  let is-first = x.instance.is_first
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let assign = sol.config  // color assignment per car
  let num-changes = sol.metric.Valid
  // Build the full sequence of car labels
  let seq-labels = seq-indices.map(i => labels.at(i))
  // Build color sequence: for each position, if is_first[pos] then color = assign[car], else 1-assign[car]
  let color-seq = range(seq-indices.len()).map(pos => {
    let car = seq-indices.at(pos)
    if is-first.at(pos) { assign.at(car) } else { 1 - assign.at(car) }
  })
  [
    #problem-def("PaintShop")[
      Given a sequence of $2n$ positions where each of $n$ cars appears exactly twice, assign a binary color to each car (each car's two occurrences receive opposite colors) to minimize the number of color changes between consecutive positions.
    ][
    NP-hard and APX-hard @epping2004. Arises in automotive manufacturing where color changes between consecutive cars on an assembly line require costly purging of paint nozzles. Each car appears twice in the sequence (two coats), and each car's two occurrences must receive opposite colors (one per side). A natural benchmark for quantum annealing due to its binary structure and industrial relevance. The best known algorithm runs in $O^*(2^n)$ by brute-force enumeration#footnote[No algorithm improving on brute-force is known for general Paint Shop.].

    *Example.* Consider $n = #n-cars$ cars with sequence $(#seq-labels.join(", "))$. Each car gets one occurrence colored 0 and the other colored 1. The assignment #labels.zip(assign).map(((l, c)) => [#l: #c\/#(1 - c)]).join(", ") yields color sequence $(#color-seq.map(c => str(c)).join(", "))$ with #num-changes color changes. The minimum is #num-changes changes.

    #figure(
      {
        let blue = graph-colors.at(0)
        let red = graph-colors.at(1)
        align(center, stack(dir: ltr, spacing: 0pt,
          ..seq-labels.zip(color-seq).enumerate().map(((i, (car, c))) => {
            let fill = if c == 0 { white } else { blue.transparentize(40%) }
            let change = if i > 0 and color-seq.at(i) != color-seq.at(i - 1) {
              place(dx: -0.08cm, dy: 0.55cm, text(6pt, fill: red, weight: "bold")[×])
            }
            stack(dir: ttb, spacing: 0.08cm,
              box(width: 0.55cm, height: 0.55cm, fill: fill, stroke: 0.5pt + luma(120),
                align(center + horizon, text(8pt, weight: "bold", car))),
              text(6pt, fill: luma(100), str(c)),
              change,
            )
          })))
      },
      caption: [Paint Shop: sequence $(#seq-labels.join(", "))$ with optimal coloring. White = color 0, blue = color 1. #num-changes color changes (marked ×).],
    ) <fig:paintshop>
    ]
  ]
}

#{
  let x = load-model-example("BicliqueCover")
  let left-size = x.instance.graph.left_size
  let right-size = x.instance.graph.right_size
  let k = x.instance.k
  let bip-edges = x.instance.graph.edges  // (li, rj) pairs
  let ne = bip-edges.len()
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let total-size = sol.metric.Valid
  [
    #problem-def("BicliqueCover")[
      Given a bipartite graph $G = (L, R, E)$ and integer $k$, find $k$ bicliques $(L_1, R_1), dots, (L_k, R_k)$ that cover all edges ($E subset.eq union.big_i L_i times R_i$) while minimizing the total size $sum_i (|L_i| + |R_i|)$.
    ][
    Biclique Cover is equivalent to factoring the biadjacency matrix $M$ of the bipartite graph as a Boolean sum of rank-1 binary matrices, connecting it to Boolean matrix rank and nondeterministic communication complexity. Applications include data compression, database optimization (covering queries with materialized views), and bioinformatics (gene expression biclustering). NP-hard even for fixed $k >= 2$. The best known algorithm runs in $O^*(2^(|L| + |R|))$ by brute-force enumeration#footnote[No algorithm improving on brute-force enumeration is known for general Biclique Cover.].

    *Example.* Consider $G = (L, R, E)$ with $L = {#range(left-size).map(i => $ell_#(i + 1)$).join(", ")}$, $R = {#range(right-size).map(i => $r_#(i + 1)$).join(", ")}$, and edges $E = {#bip-edges.map(e => $(ell_#(e.at(0) + 1), r_#(e.at(1) + 1))$).join(", ")}$. A biclique cover with $k = #k$: $(L_1, R_1) = ({ell_1}, {r_1, r_2})$ covering edges ${(ell_1, r_1), (ell_1, r_2)}$, and $(L_2, R_2) = ({ell_2}, {r_2, r_3})$ covering ${(ell_2, r_2), (ell_2, r_3)}$. Total size $= (1+2) + (1+2) = #total-size$. Merging into a single biclique is impossible since $(ell_1, r_3) in.not E$.

    #figure(
      canvas(length: 1cm, {
        let lpos = range(left-size).map(i => (0, left-size - 1 - i))
        let rpos = range(right-size).map(i => (2.5, 1.5 - i))
        let bc1 = bip-edges.filter(e => e.at(0) == 0)
        for (li, rj) in bip-edges {
          let is-bc1 = bc1.any(e => e.at(0) == li and e.at(1) == rj)
          let c = if is-bc1 { graph-colors.at(0) } else { rgb("#76b7b2") }
          g-edge(lpos.at(li), rpos.at(rj), stroke: 1.5pt + c)
        }
        for (k, p) in lpos.enumerate() {
          g-node(p, name: "l" + str(k), fill: luma(240), label: $ell_#(k+1)$)
        }
        for (k, p) in rpos.enumerate() {
          g-node(p, name: "r" + str(k), fill: luma(240), label: $r_#(k+1)$)
        }
      }),
      caption: [Biclique cover of a bipartite graph: biclique 1 (blue) $= ({ell_1}, {r_1, r_2})$, biclique 2 (teal) $= ({ell_2}, {r_2, r_3})$. Edge $(ell_1, r_3)$ is absent, preventing a single biclique.],
    ) <fig:biclique-cover>
    ]
  ]
}

#{
  let x = load-model-example("BalancedCompleteBipartiteSubgraph")
  let left-size = x.instance.graph.left_size
  let right-size = x.instance.graph.right_size
  let k = x.instance.k
  let bip-edges = x.instance.graph.edges
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let left-selected = range(left-size).filter(i => sol.config.at(i) == 1)
  let right-selected = range(right-size).filter(i => sol.config.at(left-size + i) == 1)
  let selected-edges = bip-edges.filter(e =>
    left-selected.contains(e.at(0)) and right-selected.contains(e.at(1))
  )
  [
    #problem-def("BalancedCompleteBipartiteSubgraph")[
      Given a bipartite graph $G = (A, B, E)$ and an integer $k$, determine whether there exist subsets $A' subset.eq A$ and $B' subset.eq B$ such that $|A'| = |B'| = k$ and every cross pair is present:
      $A' times B' subset.eq E.$
    ][
    Balanced Complete Bipartite Subgraph is a classical NP-complete bipartite containment problem from Garey and Johnson @garey1979. Unlike Biclique Cover, which asks for a collection of bicliques covering all edges, this problem asks for a _single_ balanced biclique of prescribed size. It arises naturally in biclustering, dense submatrix discovery, and pattern mining on bipartite data. Chen et al. give an exact $O^*(1.3803^n)$ algorithm for dense bipartite graphs, and the registry records that best-known bound in the catalog metadata. A straightforward baseline still enumerates all $k$-subsets of $A$ and $B$ and checks whether they induce a complete bipartite graph, taking $O(binom(|A|, k) dot binom(|B|, k) dot k^2) = O^*(2^(|A| + |B|))$ time.

    *Example.* Consider the bipartite graph with $A = {ell_1, ell_2, ell_3, ell_4}$, $B = {r_1, r_2, r_3, r_4}$, and edges $E = {#bip-edges.map(e => $(ell_#(e.at(0) + 1), r_#(e.at(1) + 1))$).join(", ")}$. For $k = #k$, the selected sets $A' = {#left-selected.map(i => $ell_#(i + 1)$).join(", ")}$ and $B' = {#right-selected.map(i => $r_#(i + 1)$).join(", ")}$ form a balanced complete bipartite subgraph: all #selected-edges.len() required cross edges are present. Vertex $ell_4$ is excluded because $(ell_4, r_3) in.not E$, so any witness using $ell_4$ cannot realize $K_(#k,#k)$.

    #figure(
      canvas(length: 1cm, {
        let lpos = range(left-size).map(i => (0, left-size - 1 - i))
        let rpos = range(right-size).map(i => (2.6, right-size - 1 - i))
        for (li, rj) in bip-edges {
          let selected = selected-edges.any(e => e.at(0) == li and e.at(1) == rj)
          g-edge(
            lpos.at(li),
            rpos.at(rj),
            stroke: if selected { 2pt + graph-colors.at(0) } else { 1pt + luma(180) },
          )
        }
        for (idx, pos) in lpos.enumerate() {
          let selected = left-selected.contains(idx)
          g-node(
            pos,
            name: "bcbs-l" + str(idx),
            fill: if selected { graph-colors.at(0) } else { luma(240) },
            label: if selected {
              text(fill: white)[$ell_#(idx + 1)$]
            } else {
              [$ell_#(idx + 1)$]
            },
          )
        }
        for (idx, pos) in rpos.enumerate() {
          let selected = right-selected.contains(idx)
          g-node(
            pos,
            name: "bcbs-r" + str(idx),
            fill: if selected { graph-colors.at(0) } else { luma(240) },
            label: if selected {
              text(fill: white)[$r_#(idx + 1)$]
            } else {
              [$r_#(idx + 1)$]
            },
          )
        }
      }),
      caption: [Balanced complete bipartite subgraph with $k = #k$: the selected vertices $A' = {#left-selected.map(i => $ell_#(i + 1)$).join(", ")}$ and $B' = {#right-selected.map(i => $r_#(i + 1)$).join(", ")}$ are blue, and the 9 edges of the induced $K_(#k,#k)$ are highlighted. The missing edge $(ell_4, r_3)$ prevents including $ell_4$.],
    ) <fig:balanced-complete-bipartite-subgraph>
    ]
  ]
}

#{
  let x = load-model-example("PartitionIntoTriangles")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let q = int(nv / 3)
  // optimal config groups vertices into triangles: config[i] = triangle index
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let tri-assign = sol.config
  // Group vertices by triangle
  let triangles = range(q).map(t => tri-assign.enumerate().filter(((i, v)) => v == t).map(((i, _)) => i))
  [
    #problem-def("PartitionIntoTriangles")[
      Given a graph $G = (V, E)$ with $|V| = 3q$ for some integer $q$, determine whether the vertices of $G$ can be partitioned into $q$ disjoint triples $V_1, dots, V_q$, each containing exactly 3 vertices, such that for each $V_i = {u_i, v_i, w_i}$, all three edges ${u_i, v_i}$, ${u_i, w_i}$, and ${v_i, w_i}$ belong to $E$.
    ][
      Partition Into Triangles is NP-complete by transformation from 3-Dimensional Matching @garey1979[GT11]. It remains NP-complete on graphs of maximum degree 4, with an exact algorithm running in $O^*(1.0222^n)$ for bounded-degree-4 graphs @vanrooij2013. The general brute-force bound is $O^*(2^n)$#footnote[No algorithm improving on brute-force enumeration is known for general Partition Into Triangles.].

      *Example.* Consider $G$ with $n = #nv$ vertices ($q = #q$) and edges #edges.map(((u, v)) => [${#u, #v}$]).join(", "). The partition #triangles.enumerate().map(((i, tri)) => $V_#(i + 1) = {#tri.map(v => $v_#v$).join(", ")}$).join(", ") is valid: #triangles.enumerate().map(((i, tri)) => [$V_#(i + 1)$ forms a triangle]).join(" and "). The cross-edge ${0, 3}$ is unused. Swapping $v_2$ and $v_3$ yields $V'_1 = {v_0, v_1, v_3}$, which fails because ${1, 3} in.not E$.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let verts = ((0, 1.2), (1, 0), (-1, 0), (3, 1.2), (4, 0), (2, 0))
          let tri1 = triangles.at(0)
          let tri2 = triangles.at(1)
          for (u, v) in edges {
            let is-cross = not (tri1.contains(u) and tri1.contains(v)) and not (tri2.contains(u) and tri2.contains(v))
            g-edge(verts.at(u), verts.at(v),
              stroke: if is-cross { 1pt + luma(180) } else if tri1.contains(u) and tri1.contains(v) { 1.5pt + graph-colors.at(0) } else { 1.5pt + rgb("#76b7b2") })
          }
          for (k, p) in verts.enumerate() {
            let c = if tri1.contains(k) { graph-colors.at(0).lighten(70%) } else { rgb("#76b7b2").lighten(70%) }
            g-node(p, name: "v" + str(k), fill: c, label: $v_#k$)
          }
        }),
        caption: [Partition Into Triangles: #triangles.enumerate().map(((i, tri)) => $V_#(i + 1) = {#tri.map(v => $v_#v$).join(", ")}$).join(" and ") each form a triangle. Cross-edges (gray) are unused.],
      ) <fig:partition-triangles>
    ]
  ]
}

#{
  let x = load-model-example("BinPacking")
  let sizes = x.instance.sizes
  let n = sizes.len()
  let C = x.instance.capacity
  let config = x.optimal_config
  let num-bins = x.optimal_value.Valid
  // Group items by bin
  let bins-contents = range(num-bins).map(b =>
    range(n).filter(i => config.at(i) == b)
  )
  let bin-loads = bins-contents.map(items => items.map(i => sizes.at(i)).sum())
  [
    #problem-def("BinPacking")[
      Given $n$ items with sizes $s_1, dots, s_n in RR^+$ and bin capacity $C > 0$, find an assignment $x: {1, dots, n} -> NN$ minimizing $|{x(i) : i = 1, dots, n}|$ (the number of distinct bins used) subject to $forall j: sum_(i: x(i) = j) s_i lt.eq C$.
    ][
      Bin Packing is one of the classical NP-hard optimization problems @garey1979, with applications in logistics, cutting stock, and cloud resource allocation. The best known exact algorithm runs in $O^*(2^n)$ time via inclusion-exclusion over set partitions @bjorklund2009.

      *Example.* Consider $n = #n$ items with sizes $(#sizes.map(s => str(s)).join(", "))$ and capacity $C = #C$. An optimal packing uses #num-bins bins.

      #figure({
        canvas(length: 1cm, {
          let s = 0.35
          let w = 1.0
          let gap = 0.6
          let item-colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"), rgb("#b07aa1"))
          for bi in range(num-bins) {
            let bx = bi * (w + gap)
            draw.rect((bx, 0), (bx + w, C * s), stroke: 0.8pt + black)
            let y = 0
            for item-idx in bins-contents.at(bi) {
              let sz = sizes.at(item-idx)
              let c = item-colors.at(calc.rem(item-idx, item-colors.len()))
              draw.rect((bx, y), (bx + w, y + sz * s), stroke: 0.4pt, fill: c)
              draw.content((bx + w / 2, y + sz * s / 2), text(8pt, fill: white)[#sz])
              y += sz * s
            }
            draw.content((bx + w / 2, -0.3), text(8pt)[$B_#(bi + 1)$])
          }
          let total-w = (num-bins - 1) * (w + gap) + w
          draw.line((-0.15, C * s), (total-w + 0.15, C * s),
            stroke: (dash: "dashed", paint: luma(150), thickness: 0.5pt))
          draw.content((-0.5, C * s), text(7pt)[$C$])
        })
      },
      caption: [Optimal packing of #n items into #num-bins bins of capacity $C = #C$. Numbers indicate item sizes.],
      ) <fig:binpacking-example>
    ]
  ]
}

#{
  let x = load-model-example("Knapsack")
  let weights = x.instance.weights
  let values = x.instance.values
  let C = x.instance.capacity
  let n = weights.len()
  let config = x.optimal_config
  let opt-val = x.optimal_value.Valid
  let selected = range(n).filter(i => config.at(i) == 1)
  let total-w = selected.map(i => weights.at(i)).sum()
  let total-v = selected.map(i => values.at(i)).sum()
  [
    #problem-def("Knapsack")[
      Given $n$ items with weights $w_0, dots, w_(n-1) in NN$ and values $v_0, dots, v_(n-1) in NN$, and a capacity $C in NN$, find $S subset.eq {0, dots, n - 1}$ maximizing $sum_(i in S) v_i$ subject to $sum_(i in S) w_i lt.eq C$.
    ][
      One of Karp's 21 NP-complete problems @karp1972. Knapsack is only _weakly_ NP-hard: a classical dynamic-programming algorithm runs in $O(n C)$ pseudo-polynomial time, and a fully polynomial-time approximation scheme (FPTAS) achieves $(1 - epsilon)$-optimal value in $O(n^2 slash epsilon)$ time @ibarra1975. The special case $v_i = w_i$ for all $i$ is the Subset Sum problem. Knapsack is also a special case of Integer Linear Programming with a single constraint. The best known exact algorithm is the $O^*(2^(n slash 2))$ meet-in-the-middle approach of Horowitz and Sahni @horowitz1974, which partitions items into two halves and combines sorted sublists.

      *Example.* Let $n = #n$ items with weights $(#weights.map(w => str(w)).join(", "))$, values $(#values.map(v => str(v)).join(", "))$, and capacity $C = #C$. Selecting $S = {#selected.map(i => str(i)).join(", ")}$ gives total weight $#total-w lt.eq C$ and total value $#total-v$, which is optimal.
    ]
  ]
}

#problem-def("PartiallyOrderedKnapsack")[
  Given $n$ items with weights $w_0, dots, w_(n-1) in NN$ and values $v_0, dots, v_(n-1) in NN$, a partial order $prec$ on the items (given by its cover relations), and a capacity $C in NN$, find a downward-closed subset $S subset.eq {0, dots, n - 1}$ (i.e., if $i in S$ and $j prec i$ then $j in S$) maximizing $sum_(i in S) v_i$ subject to $sum_(i in S) w_i lt.eq C$.
][
  Garey and Johnson's problem A6 MP12 @garey1979. Unlike standard Knapsack, the partial order constraint makes the problem _strongly_ NP-complete --- it remains NP-complete even when $w_i = v_i$ for all $i$, so no pseudo-polynomial algorithm exists unless $P = N P$. The problem arises in manufacturing scheduling, project selection, and mining operations. For tree partial orders, Johnson and Niemi @johnson1983 gave pseudo-polynomial $O(n dot C)$ tree DP and an FPTAS. Kolliopoulos and Steiner @kolliopoulos2007 extended the FPTAS to 2-dimensional partial orders with $O(n^4 slash epsilon)$ running time.

  *Example.* Let $n = 6$ items with weights $(2, 3, 4, 1, 2, 3)$, values $(3, 2, 5, 4, 3, 8)$, and capacity $C = 11$. The partial order has cover relations $0 prec 2$, $0 prec 3$, $1 prec 4$, $3 prec 5$, $4 prec 5$. Selecting $S = {0, 1, 3, 4, 5}$ is downward-closed (all predecessors included), has total weight $2 + 3 + 1 + 2 + 3 = 11 lt.eq C$, and total value $3 + 2 + 4 + 3 + 8 = 20$. Adding item 2 would exceed capacity ($15 > 11$).
]

#{
  let x = load-model-example("RectilinearPictureCompression")
  let mat = x.instance.matrix
  let m = mat.len()
  let n = mat.at(0).len()
  let K = x.instance.bound
  // Convert bool matrix to int for display
  let M = mat.map(row => row.map(v => if v { 1 } else { 0 }))
  [
    #problem-def("RectilinearPictureCompression")[
      Given an $m times n$ binary matrix $M$ and a nonnegative integer $K$,
      determine whether there exists a collection of at most $K$
      axis-aligned rectangles that covers precisely the 1-entries of $M$.
      Each rectangle is a quadruple $(a, b, c, d)$ with $a lt.eq b$ and $c lt.eq d$,
      covering entries $M_(i j)$ for $a lt.eq i lt.eq b$ and $c lt.eq j lt.eq d$,
      where every covered entry must satisfy $M_(i j) = 1$.
    ][
    Rectilinear Picture Compression is a classical NP-complete problem from Garey & Johnson (A4 SR25, p.~232) @garey1979. It arises naturally in image compression, DNA microarray design, integrated circuit manufacturing, and access control list minimization. NP-completeness was established by Masek (1978) via transformation from 3SAT. A straightforward exact baseline, including the brute-force solver in this crate, enumerates subsets of the maximal all-1 rectangles. If an instance has $R$ such rectangles, this gives an $O^*(2^R)$ exact search, so the worst-case behavior remains exponential in the instance size.

    *Example.* Let $M = mat(#M.map(row => row.map(v => str(v)).join(", ")).join("; "))$ (a $#m times #n$ matrix) and $K = #K$. The two maximal all-1 rectangles cover rows $0..1$, columns $0..1$ and rows $2..3$, columns $2..3$. Selecting both gives $|{R_1, R_2}| = 2 lt.eq K = #K$ and their union covers all eight 1-entries, so the answer is YES.

    #figure(
      {
        let cell-size = 0.5
        let blue = graph-colors.at(0)
        let teal = rgb("#76b7b2")
        // Rectangle covers: R1 covers rows 0..1, cols 0..1; R2 covers rows 2..3, cols 2..3
        let rect-color(r, c) = {
          if r <= 1 and c <= 1 { blue.transparentize(40%) }
          else if r >= 2 and c >= 2 { teal.transparentize(40%) }
          else { white }
        }
        align(center, grid(
          columns: n,
          column-gutter: 0pt,
          row-gutter: 0pt,
          ..range(m).map(r =>
            range(n).map(c => {
              let val = M.at(r).at(c)
              let fill = if val == 1 { rect-color(r, c) } else { white }
              box(width: cell-size * 1cm, height: cell-size * 1cm,
                fill: fill, stroke: 0.4pt + luma(180),
                align(center + horizon, text(8pt, weight: if val == 1 { "bold" } else { "regular" },
                  if val == 1 { "1" } else { "0" })))
            })
          ).flatten(),
        ))
      },
      caption: [Rectilinear Picture Compression: matrix $M$ covered by two rectangles $R_1$ (blue, top-left $2 times 2$) and $R_2$ (teal, bottom-right $2 times 2$), with $|{R_1, R_2}| = 2 lt.eq K = #K$.],
    ) <fig:rpc>
    ]
  ]
}

#{
  let x = load-model-example("RuralPostman")
  let nv = x.instance.graph.num_vertices
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let ne = edges.len()
  let edge-lengths = x.instance.edge_lengths
  let required = x.instance.required_edges
  let nr = required.len()
  let B = x.instance.bound
  let config = x.optimal_config
  // Selected edges (multiplicity >= 1)
  let selected = range(ne).filter(i => config.at(i) >= 1)
  let total-cost = selected.map(i => config.at(i) * edge-lengths.at(i)).sum()
  [
    #problem-def("RuralPostman")[
      Given an undirected graph $G = (V, E)$ with edge lengths $l: E -> ZZ_(gt.eq 0)$, a subset $E' subset.eq E$ of required edges, and a bound $B in ZZ^+$, determine whether there exists a circuit (closed walk) in $G$ that traverses every edge in $E'$ and has total length at most $B$.
    ][
      The Rural Postman Problem (RPP) is a fundamental NP-complete arc-routing problem @lenstra1976 that generalizes the Chinese Postman Problem. When $E' = E$, the problem reduces to finding an Eulerian circuit with minimum augmentation (polynomial-time solvable via $T$-join matching). For general $E' subset.eq E$, exact algorithms use dynamic programming over subsets of required edges in $O(n^2 dot 2^r)$ time, where $r = |E'|$ and $n = |V|$, analogous to the Held-Karp algorithm for TSP. The problem admits a $3 slash 2$-approximation for metric instances @frederickson1979.

      *Example.* Consider a graph with #nv vertices and #ne edges, where #(ne - 2) outer edges have length 1 and 2 diagonal edges have length 2. The required edges are $E' = {#required.map(i => {let e = edges.at(i); $(v_#(e.at(0)), v_#(e.at(1)))$}).join($,$)}$ with bound $B = #B$. The outer cycle #range(nv).map(i => $v_#i$).join($->$)$-> v_0$ covers all #nr required edges with total length $#total-cost = B$, so the answer is YES.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (
            required: rgb("#e15759"),
            optional: rgb("#4e79a7"),
            unused: luma(200),
          )
          let r = 1.5
          // Place vertices on a hexagon
          let positions = range(nv).map(i => {
            let angle = 90deg - i * 360deg / nv
            (calc.cos(angle) * r, calc.sin(angle) * r)
          })

          // Draw edges
          for (ei, (u, v)) in edges.enumerate() {
            let is-required = required.contains(ei)
            let is-selected = config.at(ei) >= 1
            let col = if is-required { colors.required } else if is-selected { colors.optional } else { colors.unused }
            let thickness = if is-selected { 1.2pt } else { 0.5pt }
            let dash = if not is-selected { "dashed" } else { "solid" }
            line(positions.at(u), positions.at(v), stroke: (paint: col, thickness: thickness, dash: dash), name: "e" + str(ei))
            // Edge length label
            let mid = ((positions.at(u).at(0) + positions.at(v).at(0)) / 2, (positions.at(u).at(1) + positions.at(v).at(1)) / 2)
            content(mid, text(6pt, fill: col)[#edge-lengths.at(ei)], fill: white, frame: "rect", padding: 0.05, stroke: none)
          }

          // Draw vertices
          for (i, pos) in positions.enumerate() {
            circle(pos, radius: 0.18, fill: white, stroke: 0.6pt + black)
            content(pos, text(7pt)[$v_#i$])
          }
        }),
        caption: [Rural Postman instance: #nv vertices, #ne edges, #nr required edges (red, bold). The outer cycle (blue + red edges) has total cost #total-cost $= B$, covering all required edges.],
      ) <fig:rural-postman>
    ]
  ]
}

#{
  let x = load-model-example("SubgraphIsomorphism")
  let nv-host = x.instance.host_graph.num_vertices
  let ne-host = x.instance.host_graph.edges.len()
  let nv-pat = x.instance.pattern_graph.num_vertices
  let ne-pat = x.instance.pattern_graph.edges.len()
  let config = x.optimal_config
  [
    #problem-def("SubgraphIsomorphism")[
      Given graphs $G = (V_1, E_1)$ (host) and $H = (V_2, E_2)$ (pattern), determine whether $G$ contains a subgraph isomorphic to $H$: does there exist an injective function $f: V_2 -> V_1$ such that ${u, v} in E_2 arrow.double {f(u), f(v)} in E_1$?
    ][
      Subgraph Isomorphism (GT48 in Garey & Johnson @garey1979) is NP-complete by transformation from Clique @garey1979. It strictly generalizes Clique (where $H = K_k$) and also contains Hamiltonian Circuit ($H = C_n$) and Hamiltonian Path ($H = P_n$) as special cases. Brute-force enumeration of all injective mappings $f: V_2 -> V_1$ runs in $O(|V_1|^(|V_2|) dot |E_2|)$ time. For fixed-size patterns, the color-coding technique of Alon, Yuster, and Zwick @alon1995 gives a randomized algorithm in $2^(O(|V_2|)) dot |V_1|^(O("tw"(H)))$ time. Practical algorithms include VF2 @cordella2004 and VF2++ @juttner2018.

      *Example.* Host graph $G = K_#nv-host$ (#nv-host vertices, #ne-host edges), pattern $H = K_#nv-pat$ (#nv-pat vertices, #ne-pat edges). The mapping $f = (#range(nv-pat).map(i => $#i arrow.bar #config.at(i)$).join($,$))$ is injective and preserves all #ne-pat pattern edges, confirming a subgraph isomorphism exists.
    ]
  ]
}

#{
  let x = load-model-example("LongestCommonSubsequence")
  let strings = x.instance.strings
  let witness = x.optimal_config
  let fmt-str(s) = "\"" + s.map(c => str(c)).join("") + "\""
  let string-list = strings.map(fmt-str).join(", ")
  let find-embed(target, candidate) = {
    let positions = ()
    let j = 0
    for (i, ch) in target.enumerate() {
      if j < candidate.len() and ch == candidate.at(j) {
        positions.push(i)
        j += 1
      }
    }
    positions
  }
  let embeds = strings.map(s => find-embed(s, witness))
  [
    #problem-def("LongestCommonSubsequence")[
      Given a finite alphabet $Sigma$, a set $R = {r_1, dots, r_m}$ of strings over $Sigma^*$, and a positive integer $K$, determine whether there exists a string $w in Sigma^*$ with $|w| gt.eq K$ such that every string $r_i in R$ contains $w$ as a _subsequence_: there exist indices $1 lt.eq j_1 < j_2 < dots < j_(|w|) lt.eq |r_i|$ with $r_i[j_t] = w[t]$ for all $t$.
    ][
      A classic NP-complete string problem, listed as problem SR10 in Garey and Johnson @garey1979. #cite(<maier1978>, form: "prose") proved NP-completeness, while Garey and Johnson note polynomial-time cases for fixed $K$ or fixed $|R|$. For the special case of two strings, the classical dynamic-programming algorithm of #cite(<wagnerfischer1974>, form: "prose") runs in $O(|r_1| dot |r_2|)$ time. The decision model implemented in this repository fixes the witness length to exactly $K$; this is equivalent to the standard "$|w| gt.eq K$" formulation because any longer common subsequence has a length-$K$ prefix.

      *Example.* Let $Sigma = {0, 1}$ and let the input set $R$ contain the strings #string-list. The witness $w = $ #fmt-str(witness) is a common subsequence of every string in $R$.

      #figure({
        let blue = graph-colors.at(0)
        align(center, stack(dir: ttb, spacing: 0.35cm,
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.2cm, height: 0.45cm, align(center + horizon, text(8pt, "w ="))),
            ..witness.enumerate().map(((i, symbol)) => {
              box(width: 0.48cm, height: 0.48cm, fill: blue.transparentize(70%), stroke: 0.5pt + luma(120),
                align(center + horizon, text(9pt, weight: "bold", str(symbol))))
            }),
          ),
          ..strings.enumerate().map(((ri, s)) => {
            let embed = embeds.at(ri)
            stack(dir: ltr, spacing: 0pt,
              box(width: 1.2cm, height: 0.45cm, align(center + horizon, text(8pt, "r" + str(ri + 1) + " ="))),
              ..s.enumerate().map(((i, symbol)) => {
                let fill = if embed.contains(i) { blue.transparentize(78%) } else { white }
                box(width: 0.48cm, height: 0.48cm, fill: fill, stroke: 0.5pt + luma(120),
                  align(center + horizon, text(9pt, weight: "bold", str(symbol))))
              }),
            )
          }),
        ))
      })

      The highlighted positions show one left-to-right embedding of $w = $ #fmt-str(witness) in each input string, certifying the YES answer for $K = 3$.
    ]
  ]
}

#{
  let x = load-model-example("SubsetSum")
  let sizes = x.instance.sizes
  let target = x.instance.target
  let n = sizes.len()
  let config = x.optimal_config
  let selected = range(n).filter(i => config.at(i) == 1)
  let sel-sizes = selected.map(i => sizes.at(i))
  [
    #problem-def("SubsetSum")[
      Given a finite set $A = {a_0, dots, a_(n-1)}$ with sizes $s(a_i) in ZZ^+$ and a target $B in ZZ^+$, determine whether there exists a subset $A' subset.eq A$ such that $sum_(a in A') s(a) = B$.
    ][
      One of Karp's 21 NP-complete problems @karp1972. Subset Sum is the special case of Knapsack where $v_i = w_i$ for all items and we seek an exact sum rather than an inequality. Though NP-complete, it is only _weakly_ NP-hard: a dynamic-programming algorithm runs in $O(n B)$ pseudo-polynomial time. The best known exact algorithm is the $O^*(2^(n slash 2))$ meet-in-the-middle approach of Horowitz and Sahni @horowitz1974.

      *Example.* Let $A = {#sizes.map(s => str(s)).join(", ")}$ ($n = #n$) and target $B = #target$. Selecting $A' = {#sel-sizes.map(s => str(s)).join(", ")}$ gives sum $#sel-sizes.map(s => str(s)).join(" + ") = #target = B$.
    ]
  ]
}

#problem-def("ResourceConstrainedScheduling")[
  Given a set $T$ of $n$ unit-length tasks, $m$ identical processors, $r$ resources with bounds $B_i$ ($1 <= i <= r$), resource requirements $R_i (t)$ for each task $t$ and resource $i$ ($0 <= R_i (t) <= B_i$), and an overall deadline $D in ZZ^+$, determine whether there exists an $m$-processor schedule $sigma : T -> {0, dots, D-1}$ such that for every time slot $u$, at most $m$ tasks are scheduled at $u$ and $sum_(t : sigma(t) = u) R_i (t) <= B_i$ for each resource $i$.
][
  RESOURCE CONSTRAINED SCHEDULING is problem SS10 in Garey & Johnson's compendium @garey1979. It is NP-complete in the strong sense, even for $r = 1$ resource and $m = 3$ processors, by reduction from 3-PARTITION @garey1979. For $m = 2$ processors with arbitrary $r$, the problem is solvable in polynomial time via bipartite matching. The general case subsumes bin-packing-style constraints across multiple resource dimensions.

  *Example.* Let $n = 6$ tasks, $m = 3$ processors, $r = 1$ resource with $B_1 = 20$, and deadline $D = 2$. Resource requirements: $R_1(t_1) = 6$, $R_1(t_2) = 7$, $R_1(t_3) = 7$, $R_1(t_4) = 6$, $R_1(t_5) = 8$, $R_1(t_6) = 6$. Schedule: slot 0 $arrow.l {t_1, t_2, t_3}$ (3 tasks, resource $= 20$), slot 1 $arrow.l {t_4, t_5, t_6}$ (3 tasks, resource $= 20$). Both constraints satisfied; answer: YES.
]

#problem-def("BoyceCoddNormalFormViolation")[
  *Instance:* A set $A$ of attribute names, a collection $F$ of functional dependencies on $A$, and a subset $A' subset.eq A$.

  *Question:* Is there a subset $X subset.eq A'$ and two attributes $y, z in A' backslash X$ such that $y in X^+$ but $z in.not X^+$, where $X^+$ is the closure of $X$ under $F$?
][
  A relation satisfies _Boyce-Codd Normal Form_ (BCNF) if every non-trivial functional dependency $X arrow.r Y$ has $X$ as a superkey --- that is, $X^+$ = $A'$. This classical NP-complete problem from database theory asks whether the given attribute subset $A'$ violates BCNF. The NP-completeness was established by Beeri and Bernstein (1979) via reduction from Hitting Set. It appears as problem SR29 in Garey and Johnson's compendium (category A4: Storage and Retrieval).
]

#problem-def("SumOfSquaresPartition")[
  Given a finite set $A = {a_0, dots, a_(n-1)}$ with sizes $s(a_i) in ZZ^+$, a positive integer $K lt.eq |A|$ (number of groups), and a positive integer $J$ (bound), determine whether $A$ can be partitioned into $K$ disjoint sets $A_1, dots, A_K$ such that $sum_(i=1)^K (sum_(a in A_i) s(a))^2 lt.eq J$.
][
  Problem SP19 in Garey and Johnson @garey1979. NP-complete in the strong sense, so no pseudo-polynomial time algorithm exists unless $P = "NP"$. For fixed $K$, a dynamic-programming algorithm runs in $O(n S^(K-1))$ pseudo-polynomial time, where $S = sum s(a)$. The problem remains NP-complete when the exponent 2 is replaced by any fixed rational $alpha > 1$. #footnote[No algorithm improving on brute-force $O(K^n)$ enumeration is known for the general case.] The squared objective penalizes imbalanced partitions, connecting it to variance minimization, load balancing, and $k$-means clustering. Sum of Squares Partition generalizes Partition ($K = 2$, $J = S^2 slash 2$).

  *Example.* Let $A = {5, 3, 8, 2, 7, 1}$ ($n = 6$), $K = 3$ groups, and bound $J = 240$. The partition $A_1 = {8, 1}$, $A_2 = {5, 2}$, $A_3 = {3, 7}$ gives group sums $9, 7, 10$ and sum of squares $81 + 49 + 100 = 230 lt.eq 240 = J$. With a tighter bound $J = 225$, the best achievable partition has group sums ${9, 9, 8}$ yielding $81 + 81 + 64 = 226 > 225$, so the answer is NO.
]

#{
  let x = load-model-example("SequencingWithReleaseTimesAndDeadlines")
  let n = x.instance.lengths.len()
  let lengths = x.instance.lengths
  let release = x.instance.release_times
  let deadline = x.instance.deadlines
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  // Decode Lehmer code to permutation
  let available = range(n)
  let perm = ()
  for c in sol.config {
    perm = perm + (available.at(c),)
    available = available.slice(0, c) + available.slice(c + 1)
  }
  // Compute start times by simulating the schedule (build (task_idx, start) pairs)
  let current = 0
  let schedule = ()
  for idx in perm {
    let s = calc.max(current, release.at(idx))
    schedule = schedule + ((idx, s),)
    current = s + lengths.at(idx)
  }
  [
    #problem-def("SequencingWithReleaseTimesAndDeadlines")[
      Given a set $T$ of $n$ tasks and, for each task $t in T$, a processing time $ell(t) in ZZ^+$, a release time $r(t) in ZZ^(>=0)$, and a deadline $d(t) in ZZ^+$, determine whether there exists a one-processor schedule $sigma: T -> ZZ^(>=0)$ such that for all $t in T$: $sigma(t) >= r(t)$, $sigma(t) + ell(t) <= d(t)$, and no two tasks overlap (i.e., $sigma(t) > sigma(t')$ implies $sigma(t) >= sigma(t') + ell(t')$).
    ][
      Problem SS1 in Garey and Johnson's appendix @garey1979, and a fundamental single-machine scheduling feasibility problem. It is strongly NP-complete by reduction from 3-Partition, so no pseudo-polynomial time algorithm exists unless P = NP. The problem becomes polynomial-time solvable when: (1) all task lengths equal 1, (2) preemption is allowed, or (3) all release times are zero. The best known exact algorithm for the general case runs in $O^*(2^n dot n)$ time via dynamic programming on task subsets.

      *Example.* Consider #n tasks:
      #align(center, table(
        columns: n + 1,
        align: center,
        table.header([], ..range(n).map(i => [$t_#(i + 1)$])),
        [$ell(t)$], ..lengths.map(l => [#l]),
        [$r(t)$], ..release.map(r => [#r]),
        [$d(t)$], ..deadline.map(d => [#d]),
      ))
      A feasible schedule: #schedule.map(((idx, s)) => [$sigma(t_#(idx + 1)) = #s$ (runs $[#s, #(s + lengths.at(idx)))$)]).join([, ]). All release and deadline constraints are satisfied with no overlap.
    ]
  ]
}

#problem-def("Partition")[
  Given a finite set $A = {a_0, dots, a_(n-1)}$ with sizes $s(a_i) in ZZ^+$, determine whether there exists a subset $A' subset.eq A$ such that $sum_(a in A') s(a) = sum_(a in A without A') s(a)$.
][
  One of Karp's 21 NP-complete problems @karp1972, listed as SP12 in Garey & Johnson @garey1979. Partition is the special case of Subset Sum where the target equals half the total sum. Though NP-complete, it is only _weakly_ NP-hard: a dynamic-programming algorithm runs in $O(n dot B_"total")$ pseudo-polynomial time, where $B_"total" = sum_i s(a_i)$. The best known exact algorithm is the $O^*(2^(n slash 2))$ meet-in-the-middle approach of Schroeppel and Shamir (1981).

  *Example.* Let $A = {3, 1, 1, 2, 2, 1}$ ($n = 6$, total sum $= 10$). Setting $A' = {3, 2}$ (indices 0, 3) gives sum $3 + 2 = 5 = 10 slash 2$, and $A without A' = {1, 1, 2, 1}$ also sums to 5. Hence a balanced partition exists.
]

#{
  let x = load-model-example("ShortestCommonSupersequence")
  let alpha-size = x.instance.alphabet_size
  let bound = x.instance.bound
  let strings = x.instance.strings
  let nr = strings.len()
  // Alphabet mapping: 0->a, 1->b, 2->c, ...
  let alpha-map = range(alpha-size).map(i => str.from-unicode(97 + i))
  let fmt-str(s) = "\"" + s.map(c => alpha-map.at(c)).join("") + "\""
  // Pick optimal config = [1,0,1,2] = "babc" to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let w = sol.config.map(c => alpha-map.at(c))
  let w-str = fmt-str(sol.config)
  let w-len = w.len()
  // Format input strings
  let r-strs = strings.map(s => fmt-str(s))
  let r-chars = strings.map(s => s.map(c => alpha-map.at(c)))
  // Compute embeddings: for each input string, find positions in w
  let compute-embed(r, w-cfg) = {
    let positions = ()
    let j = 0
    for (i, ch) in w-cfg.enumerate() {
      if j < r.len() and ch == r.at(j) {
        positions.push(i)
        j += 1
      }
    }
    positions
  }
  let embeds = strings.map(s => compute-embed(s, sol.config))
  [
    #problem-def("ShortestCommonSupersequence")[
      Given a finite alphabet $Sigma$, a set $R = {r_1, dots, r_m}$ of strings over $Sigma^*$, and a positive integer $K$, determine whether there exists a string $w in Sigma^*$ with $|w| lt.eq K$ such that every string $r_i in R$ is a _subsequence_ of $w$: there exist indices $1 lt.eq j_1 < j_2 < dots < j_(|r_i|) lt.eq |w|$ with $w[j_k] = r_i [k]$ for all $k$.
    ][
      A classic NP-complete string problem, listed as problem SR8 in Garey and Johnson @garey1979. #cite(<maier1978>, form: "prose") proved NP-completeness; #cite(<raiha1981>, form: "prose") showed the problem remains NP-complete even over a binary alphabet ($|Sigma| = 2$). Note that _subsequence_ (characters may be non-contiguous) differs from _substring_ (contiguous block): the Shortest Common Supersequence asks that each input string can be embedded into $w$ by selecting characters in order but not necessarily adjacently.

      For $|R| = 2$ strings, the problem is solvable in polynomial time via the duality with the Longest Common Subsequence (LCS): if $"LCS"(r_1, r_2)$ has length $ell$, then the shortest common supersequence has length $|r_1| + |r_2| - ell$, computable in $O(|r_1| dot |r_2|)$ time by dynamic programming. For general $|R| = m$, the brute-force search over all strings of length at most $K$ takes $O(|Sigma|^K)$ time. Applications include bioinformatics (reconstructing ancestral sequences from fragments), data compression (representing multiple strings compactly), and scheduling (merging instruction sequences).

      *Example.* Let $Sigma = {#alpha-map.join(", ")}$ and $R = {#r-strs.join(", ")}$. We seek a string $w$ of length at most $K = #bound$ that contains every $r_i$ as a subsequence.

      #figure({
        let r-colors = (graph-colors.at(0), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#e15759"), rgb("#b07aa1"))
        align(center, stack(dir: ttb, spacing: 0.6cm,
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.2cm, height: 0.5cm, align(center + horizon, text(8pt)[$w =$])),
            ..w.enumerate().map(((i, ch)) => {
              // Count how many strings use this position
              let used = range(nr).filter(ri => embeds.at(ri).contains(i)).len()
              let fill = if used >= 2 { r-colors.at(0).transparentize(50%) } else if used == 1 { r-colors.at(0).transparentize(80%) } else { white }
              box(width: 0.55cm, height: 0.55cm, fill: fill, stroke: 0.5pt + luma(120),
                align(center + horizon, text(9pt, weight: "bold", ch)))
            }),
          ),
          ..range(nr).map(ri => {
            let embed = embeds.at(ri)
            let r = r-chars.at(ri)
            let col = r-colors.at(ri)
            stack(dir: ltr, spacing: 0pt,
              box(width: 1.2cm, height: 0.5cm, align(center + horizon, text(8pt, fill: col)[$r_#(ri + 1) =$])),
              ..range(w-len).map(i => {
                let idx = embed.position(j => j == i)
                let ch = if idx != none { r.at(idx) } else { sym.dot.c }
                let c = if idx != none { col } else { luma(200) }
                box(width: 0.55cm, height: 0.55cm,
                  align(center + horizon, text(9pt, fill: c, weight: if idx != none { "bold" } else { "regular" }, ch)))
              }),
            )
          }),
        ))
      },
      caption: [Shortest Common Supersequence: $w = #w-str$ (length #w-len) contains #range(nr).map(ri => [$r_#(ri + 1) = #r-strs.at(ri)$ (positions #embeds.at(ri).map(p => str(p)).join(","))]).join(", ") as subsequences. Dots mark unused positions.],
      ) <fig:scs>

      The supersequence $w = #w-str$ has length #w-len $lt.eq K = #bound$ and contains all #nr input strings as subsequences.
    ]
  ]
}

#{
  let x = load-model-example("StringToStringCorrection")
  let source = x.instance.source
  let target = x.instance.target
  let alpha-size = x.instance.alphabet_size
  let bound-k = x.instance.bound
  let n = source.len()
  // Alphabet mapping: 0->a, 1->b, 2->c, 3->d
  let alpha-map = range(alpha-size).map(i => str.from-unicode(97 + i))
  let fmt-str(s) = s.map(c => alpha-map.at(c)).join("")
  let src-str = fmt-str(source)
  let tgt-str = fmt-str(target)
  // Use solution [8, 5]: swap(2,3) then delete(5)
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  // Trace the operations
  let after-swap = (source.at(0), source.at(1), source.at(3), source.at(2), source.at(4), source.at(5))
  let after-swap-str = after-swap.map(c => alpha-map.at(c)).join("")
  [
    #problem-def("StringToStringCorrection")[
      Given a finite alphabet $Sigma$, a source string $x in Sigma^*$, a target string $y in Sigma^*$, and a positive integer $K$, determine whether $y$ can be derived from $x$ by a sequence of at most $K$ operations, where each operation is either a _single-symbol deletion_ (remove one character at a chosen position) or an _adjacent-symbol interchange_ (swap two neighboring characters).
    ][
      A classical NP-complete problem listed as SR20 in Garey and Johnson @garey1979. #cite(<wagner1975>, form: "prose") proved NP-completeness via transformation from Set Covering. The standard edit distance problem --- allowing insertion, deletion, and substitution --- is solvable in $O(|x| dot |y|)$ time by the Wagner--Fischer dynamic programming algorithm @wagner1974. However, restricting the operation set to only deletions and adjacent swaps makes the problem NP-complete for unbounded alphabets. When only adjacent swaps are allowed (no deletions), the problem reduces to counting inversions and is polynomial @wagner1975.#footnote[No algorithm improving on brute-force is known for the general swap-and-delete variant.]

      *Example.* Let $Sigma = {#alpha-map.join(", ")}$, source $x = #src-str$ (length #n), target $y = #tgt-str$ (length #target.len()), and $K = #bound-k$.

      #figure({
        let blue = graph-colors.at(0)
        let red = rgb("#e15759")
        let cell(ch, highlight: false, strike: false) = {
          let fill = if highlight { blue.transparentize(70%) } else { white }
          box(width: 0.55cm, height: 0.55cm, fill: fill, stroke: 0.5pt + luma(120),
            align(center + horizon, text(9pt, weight: "bold",
              if strike { text(fill: red, [#sym.times]) } else { ch })))
        }
        align(center, stack(dir: ttb, spacing: 0.5cm,
          // Step 0: source
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[$x: quad$])),
            ..source.map(c => cell(alpha-map.at(c))),
          ),
          // Step 1: after swap at positions 2,3
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[swap$(2,3)$: quad])),
            ..range(after-swap.len()).map(i => cell(alpha-map.at(after-swap.at(i)), highlight: after-swap.at(i) != source.at(i))),
          ),
          // Step 2: after delete at position 5
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[del$(5)$: quad])),
            ..target.map(c => cell(alpha-map.at(c))),
            cell([], strike: true),
          ),
          // Result
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[$= y$: quad])),
            ..target.map(c => cell(alpha-map.at(c), highlight: true)),
          ),
        ))
      },
      caption: [String-to-String Correction: transforming $x = #src-str$ into $y = #tgt-str$ with $K = #bound-k$ operations. Step 1 swaps adjacent symbols at positions 2 and 3; step 2 deletes the symbol at position 5.],
      ) <fig:stsc>

      The transformation uses exactly $K = #bound-k$ operations (1 swap + 1 deletion), which is the minimum: a single operation cannot account for both the transposition of two symbols and the removal of one.
    ]
  ]
}

#{
  let x = load-model-example("MinimumFeedbackArcSet")
  let nv = x.instance.graph.num_vertices
  let arcs = x.instance.graph.arcs.map(a => (a.at(0), a.at(1)))
  let na = arcs.len()
  let weights = x.instance.weights
  let config = x.optimal_config
  let opt-val = x.optimal_value.Valid
  let removed = range(na).filter(i => config.at(i) == 1)
  [
    #problem-def("MinimumFeedbackArcSet")[
      Given a directed graph $G = (V, A)$, find a minimum-size subset $A' subset.eq A$ such that $G - A'$ is a directed acyclic graph (DAG). Equivalently, $A'$ must contain at least one arc from every directed cycle in $G$.
    ][
      Feedback Arc Set (FAS) is a classical NP-complete problem from Karp's original list @karp1972 (via transformation from Vertex Cover, as presented in Garey & Johnson GT8). The problem arises in ranking aggregation, sports scheduling, deadlock avoidance, and causal inference. Unlike the undirected analogue (which is trivially polynomial --- the number of non-tree edges in a spanning forest), the directed version is NP-hard due to the richer structure of directed cycles. The best known exact algorithm uses dynamic programming over vertex subsets in $O^*(2^n)$ time, generalizing the Held--Karp TSP technique to vertex ordering problems @bodlaender2012. FAS is fixed-parameter tractable with parameter $k = |A'|$: an $O(4^k dot k! dot n^(O(1)))$ algorithm exists via iterative compression @chen2008. Polynomial-time solvable for planar digraphs via the Lucchesi--Younger theorem @lucchesi1978.

      *Example.* Consider $G$ with $V = {#range(nv).map(v => str(v)).join(", ")}$ and arcs #arcs.map(a => $(#(a.at(0)) arrow #(a.at(1)))$).join($,$). Removing $A' = {#removed.map(i => {let a = arcs.at(i); $(#(a.at(0)) arrow #(a.at(1)))$}).join($,$)}$ (weight #opt-val) breaks all directed cycles, yielding a DAG.
    ]
  ]
}

#{
  let x = load-model-example("MultipleChoiceBranching")
  let nv = graph-num-vertices(x.instance)
  let arcs = x.instance.graph.arcs
  let chosen = x.optimal_config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("MultipleChoiceBranching")[
      Given a directed graph $G = (V, A)$, arc weights $w: A -> ZZ^+$, a partition $A_1, A_2, dots, A_m$ of $A$, and a threshold $K in ZZ^+$, determine whether there exists a subset $A' subset.eq A$ with $sum_(a in A') w(a) >= K$ such that every vertex has in-degree at most one in $(V, A')$, the selected subgraph $(V, A')$ is acyclic, and $|A' inter A_i| <= 1$ for every partition group.
    ][
      Multiple Choice Branching is the directed-graph problem ND11 in Garey & Johnson @garey1979. The partition constraint turns the polynomial-time maximum branching setting into an NP-complete decision problem: Garey and Johnson note that the problem remains NP-complete even when the digraph is strongly connected and all weights are equal, while the special case in which every partition group has size 1 reduces to ordinary maximum branching and becomes polynomial-time solvable @garey1979.

      A conservative exact algorithm enumerates all $2^{|A|}$ arc subsets and checks the partition, in-degree, acyclicity, and threshold constraints in polynomial time. This is the brute-force search space used by the implementation.#footnote[We use the registry complexity bound $O^*(2^{|A|})$ for the full partitioned problem.]

      *Example.* Consider the digraph on $n = #nv$ vertices with arcs $(0 arrow 1), (0 arrow 2), (1 arrow 3), (2 arrow 3), (1 arrow 4), (3 arrow 5), (4 arrow 5), (2 arrow 4)$, partition groups $A_1 = {(0 arrow 1), (0 arrow 2)}$, $A_2 = {(1 arrow 3), (2 arrow 3)}$, $A_3 = {(1 arrow 4), (2 arrow 4)}$, $A_4 = {(3 arrow 5), (4 arrow 5)}$, and threshold $K = 10$. The highlighted selection $A' = {(0 arrow 1), (1 arrow 3), (2 arrow 4), (3 arrow 5)}$ has total weight $3 + 4 + 3 + 3 = 13 >= 10$, uses exactly one arc from each partition group, and gives in-degrees 1 at vertices $1, 3, 4,$ and $5$. Because every selected arc points strictly left-to-right in the drawing, the selected subgraph is acyclic. The figure highlights one satisfying selection for this instance.

      #figure({
        let verts = ((0, 1.6), (1.3, 2.3), (1.3, 0.9), (3.0, 2.3), (3.0, 0.9), (4.6, 1.6))
        canvas(length: 1cm, {
          for (idx, arc) in arcs.enumerate() {
            let (u, v) = arc
            let selected = chosen.contains(idx)
            draw.line(
              verts.at(u),
              verts.at(v),
              stroke: if selected { 2pt + graph-colors.at(0) } else { 0.9pt + luma(180) },
              mark: (end: "straight", scale: if selected { 0.5 } else { 0.4 }),
            )
          }
          for (k, pos) in verts.enumerate() {
            g-node(pos, name: "v" + str(k), label: [$v_#k$])
          }
        })
      },
      caption: [Directed graph for Multiple Choice Branching. Blue arcs show the satisfying branching $(0 arrow 1), (1 arrow 3), (2 arrow 4), (3 arrow 5)$ of total weight 13; gray arcs are available but unselected.],
      ) <fig:mcb-example>
    ]
  ]
}

#{
  let x = load-model-example("FlowShopScheduling")
  let m = x.instance.num_processors
  let task-lengths = x.instance.task_lengths
  let n = task-lengths.len()
  let D = x.instance.deadline
  let lehmer = x.optimal_config
  // Decode Lehmer code to job permutation
  let job-order = {
    let avail = range(n)
    let result = ()
    for c in lehmer {
      result.push(avail.at(c))
      avail = avail.enumerate().filter(((i, v)) => i != c).map(((i, v)) => v)
    }
    result
  }
  // Compute Gantt schedule greedily
  let machine-end = range(m).map(_ => 0)
  let job-end = range(n).map(_ => 0)
  let blocks = ()
  for ji in job-order {
    let lengths = task-lengths.at(ji)
    for mi in range(m) {
      let start = calc.max(machine-end.at(mi), job-end.at(ji))
      let end = start + lengths.at(mi)
      blocks.push((mi, ji, start, end))
      machine-end.at(mi) = end
      job-end.at(ji) = end
    }
  }
  let makespan = calc.max(..job-end)
  [
    #problem-def("FlowShopScheduling")[
      Given $m$ processors and a set $J$ of $n$ jobs, where each job $j in J$ consists of $m$ tasks $t_1 [j], t_2 [j], dots, t_m [j]$ with lengths $ell(t_i [j]) in ZZ^+_0$, and a deadline $D in ZZ^+$, determine whether there exists a permutation schedule $pi$ of the jobs such that all jobs complete by time $D$. Each job must be processed on machines $1, 2, dots, m$ in order, and job $j$ cannot start on machine $i+1$ until its task on machine $i$ is completed.
    ][
      Flow Shop Scheduling is a classical NP-complete problem from Garey & Johnson (A5 SS15), strongly NP-hard for $m >= 3$ @garey1976. For $m = 2$, it is solvable in $O(n log n)$ by Johnson's rule @johnson1954. The problem is fundamental in operations research, manufacturing planning, and VLSI design. When restricted to permutation schedules (same job order on all machines), the search space is $n!$ orderings. The best known exact algorithm for $m = 3$ runs in $O^*(3^n)$ time @shang2018; for general $m$, brute-force over $n!$ permutations gives $O(n! dot m n)$.

      *Example.* Let $m = #m$ machines, $n = #n$ jobs with task lengths:
      #align(center, math.equation([$ell = #math.mat(..task-lengths.map(row => row.map(v => [#v])))$]))
      and deadline $D = #D$. The job order $pi = (#job-order.map(j => $j_#(j + 1)$).join($,$))$ yields makespan $#makespan <= #D$, so a feasible schedule exists.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"))
          let scale = 0.38
          let row-h = 0.6
          let gap = 0.15

          // Machine labels
          for mi in range(m) {
            let y = -mi * (row-h + gap)
            content((-0.8, y), text(8pt, "M" + str(mi + 1)))
          }

          // Draw schedule blocks
          for (mi, ji, s, e) in blocks {
            let x0 = s * scale
            let x1 = e * scale
            let y = -mi * (row-h + gap)
            rect((x0, y - row-h / 2), (x1, y + row-h / 2),
              fill: colors.at(ji).transparentize(30%), stroke: 0.4pt + colors.at(ji))
            content(((x0 + x1) / 2, y), text(6pt, [$j_#(ji + 1)$]))
          }

          // Time axis
          let y-axis = -(m - 1) * (row-h + gap) - row-h / 2 - 0.2
          line((0, y-axis), (makespan * scale, y-axis), stroke: 0.4pt)
          for t in range(calc.ceil(makespan / 5) + 1).map(i => calc.min(i * 5, makespan)) {
            let x = t * scale
            line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
            content((x, y-axis - 0.25), text(6pt, str(t)))
          }
          // Add makespan tick if not already shown
          if calc.rem(makespan, 5) != 0 {
            let x = makespan * scale
            line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
            content((x, y-axis - 0.25), text(6pt, str(makespan)))
          }
          content((makespan * scale / 2, y-axis - 0.5), text(7pt)[$t$])

          // Deadline marker
          let dl-x = D * scale
          line((dl-x, row-h / 2 + 0.1), (dl-x, y-axis), stroke: (paint: red, thickness: 0.8pt, dash: "dashed"))
          content((dl-x, row-h / 2 + 0.25), text(6pt, fill: red)[$D = #D$])
        }),
        caption: [Flow shop schedule for #n jobs on #m machines. Job order $(#job-order.map(j => $j_#(j + 1)$).join($,$))$ achieves makespan #makespan, within deadline $D = #D$ (dashed red line).],
      ) <fig:flowshop>
    ]
  ]
}

#problem-def("StaffScheduling")[
  Given a collection $C$ of binary schedule patterns of length $m$, where each pattern has exactly $k$ ones, a requirement vector $overline(R) in ZZ_(>= 0)^m$, and a worker budget $n in ZZ_(>= 0)$, determine whether there exists a function $f: C -> ZZ_(>= 0)$ such that $sum_(c in C) f(c) <= n$ and $sum_(c in C) f(c) dot c >= overline(R)$ component-wise.
][
  Staff Scheduling is problem SS20 in Garey and Johnson's catalog @garey1979. It models workforce planning with reusable shift templates: each pattern describes the periods covered by one worker, and the multiplicity function $f$ chooses how many workers receive each template. The general problem is NP-complete @garey1979, while the circular-ones special case admits a polynomial-time algorithm via network-flow structure @bartholdi1980. In this codebase the registered baseline enumerates all assignments of $0, dots, n$ workers to each pattern, matching the $(n + 1)^(|C|)$ configuration space exposed by the model.

  *Example.* Consider a 7-day week with $k = 5$ working days per schedule, worker budget $n = 4$, and schedule patterns
  $ c_1 = (1, 1, 1, 1, 1, 0, 0), c_2 = (0, 1, 1, 1, 1, 1, 0), c_3 = (0, 0, 1, 1, 1, 1, 1), c_4 = (1, 0, 0, 1, 1, 1, 1), c_5 = (1, 1, 0, 0, 1, 1, 1) $
  with requirement vector $overline(R) = (2, 2, 2, 3, 3, 2, 1)$. Choosing
  $ f(c_1) = f(c_2) = f(c_3) = f(c_4) = 1 $ and $ f(c_5) = 0 $
  uses exactly 4 workers and yields coverage vector $(2, 2, 3, 4, 4, 3, 2) >= overline(R)$, so the instance is feasible.

  #figure(
    align(center, table(
      columns: 9,
      align: center,
      table.header([Schedule], [Mon], [Tue], [Wed], [Thu], [Fri], [Sat], [Sun], [Workers]),
      [$c_1$], [1], [1], [1], [1], [1], [0], [0], [1],
      [$c_2$], [0], [1], [1], [1], [1], [1], [0], [1],
      [$c_3$], [0], [0], [1], [1], [1], [1], [1], [1],
      [$c_4$], [1], [0], [0], [1], [1], [1], [1], [1],
      [$c_5$], [1], [1], [0], [0], [1], [1], [1], [0],
      [$overline(R)$], [2], [2], [2], [3], [3], [2], [1], [-],
      [Coverage], [2], [2], [3], [4], [4], [3], [2], [4],
    )),
    caption: [Worked Staff Scheduling instance. The last column shows the chosen multiplicities $f(c_i)$; the final row verifies that daily coverage dominates the requirement vector while using 4 workers.],
  ) <fig:staff-scheduling>
]

#{
  let x = load-model-example("TimetableDesign")
  let assignments = x.optimal_config.enumerate().filter(((idx, value)) => value == 1).map(((idx, value)) => (
    calc.floor(idx / (x.instance.num_tasks * x.instance.num_periods)),
    calc.floor(calc.rem(idx, x.instance.num_tasks * x.instance.num_periods) / x.instance.num_periods),
    calc.rem(idx, x.instance.num_periods),
  ))
  let fmt-assignment(entry) = $(c_#(entry.at(0) + 1), t_#(entry.at(1) + 1))$
  let period-0 = assignments.filter(entry => entry.at(2) == 0)
  let period-1 = assignments.filter(entry => entry.at(2) == 1)
  let period-2 = assignments.filter(entry => entry.at(2) == 2)
  [
    #problem-def("TimetableDesign")[
      Given a set $H$ of work periods, a set $C$ of craftsmen, a set $T$ of tasks, availability sets $A_C(c) subset.eq H$ for each craftsman $c in C$, availability sets $A_T(t) subset.eq H$ for each task $t in T$, and exact workload requirements $R: C times T -> ZZ_(>= 0)$, determine whether there exists a function $f: C times T times H -> {0, 1}$ such that:
      $
        f(c, t, h) = 1 => h in A_C(c) inter A_T(t),
      $
      $
        forall c in C, h in H: sum_(t in T) f(c, t, h) <= 1,
      $
      $
        forall t in T, h in H: sum_(c in C) f(c, t, h) <= 1,
      $
      and
      $
        forall c in C, t in T: sum_(h in H) f(c, t, h) = R(c, t).
      $
    ][
      Timetable Design is the classical timetabling feasibility problem catalogued as SS19 in Garey & Johnson @garey1979. Even, Itai, and Shamir showed that it is NP-complete even when there are only three work periods, every task is available in every period, and every requirement is binary @evenItaiShamir1976. The same paper also identifies polynomial-time islands, including cases where each craftsman is available in at most two periods or where all craftsmen and tasks are available in every period @evenItaiShamir1976. The implementation in this repository uses one binary variable for each triple $(c, t, h)$, so the registered baseline explores a configuration space of size $2^(|C| |T| |H|)$.

      *Example.* The canonical instance has three periods $H = {h_1, h_2, h_3}$, five craftsmen, five tasks, and seven nonzero workload requirements. The satisfying timetable stored in the example database assigns #period-0.map(fmt-assignment).join(", ") during $h_1$, #period-1.map(fmt-assignment).join(", ") during $h_2$, and #period-2.map(fmt-assignment).join(", ") during $h_3$. Every listed assignment lies in the corresponding availability intersection $A_C(c) inter A_T(t)$, no craftsman or task appears twice in the same period, and each required pair is scheduled exactly once, so the verifier returns YES.

      #figure(
        align(center, table(
          columns: 2,
          align: center,
          table.header([Period], [Assignments]),
          [$h_1$], [#period-0.map(fmt-assignment).join(", ")],
          [$h_2$], [#period-1.map(fmt-assignment).join(", ")],
          [$h_3$], [#period-2.map(fmt-assignment).join(", ")],
        )),
        caption: [Worked Timetable Design instance derived from the canonical example DB. Each row lists the craftsman-task pairs assigned in one work period.],
      ) <fig:timetable-design>
    ]
  ]
}

#{
  let x = load-model-example("MultiprocessorScheduling")
  let lengths = x.instance.lengths
  let num-processors = x.instance.num_processors
  let deadline = x.instance.deadline
  let assignment = x.optimal_config
  let tasks-by-processor = range(num-processors).map(p =>
    range(lengths.len()).filter(i => assignment.at(i) == p)
  )
  let loads = tasks-by-processor.map(tasks => tasks.map(i => lengths.at(i)).sum())
  let max-x = (num-processors - 1) * 1.8 + 1.0
  [
    #problem-def("MultiprocessorScheduling")[
      Given a finite set $T$ of tasks with processing lengths $ell: T -> ZZ^+$, a number $m in ZZ^+$ of identical processors, and a deadline $D in ZZ^+$, determine whether there exists an assignment $p: T -> {1, dots, m}$ such that for every processor $i in {1, dots, m}$ we have $sum_(t in T: p(t) = i) ell(t) <= D$.
    ][
      Multiprocessor Scheduling is problem SS8 in Garey & Johnson @garey1979. Their original formulation uses start times on identical processors, but because tasks are independent and non-preemptive, any feasible schedule can be packed contiguously on each processor. The model implemented here therefore uses processor-assignment variables, and feasibility reduces to checking that every processor's total load is at most $D$. For fixed $m$, dynamic programming over load vectors gives pseudo-polynomial algorithms; for general $m$, the best known exact algorithm runs in $O^*(2^n)$ time via inclusion-exclusion over set partitions @bjorklund2009.

      *Example.* Let $T = {t_1, dots, t_5}$ with lengths $(4, 5, 3, 2, 6)$, $m = 2$, and $D = 10$. The satisfying assignment $(1, 2, 2, 2, 1)$ places $t_1$ and $t_5$ on processor 1 and $t_2, t_3, t_4$ on processor 2. The verifier computes the processor loads $4 + 6 = 10$ and $5 + 3 + 2 = 10$, so both meet the deadline exactly.

      #figure({
        canvas(length: 1cm, {
          let scale = 0.25
          let width = 1.0
          let gap = 0.8
          let colors = (
            rgb("#4e79a7"),
            rgb("#e15759"),
            rgb("#76b7b2"),
            rgb("#f28e2b"),
            rgb("#59a14f"),
          )

          for p in range(num-processors) {
            let x0 = p * (width + gap)
            draw.rect((x0, 0), (x0 + width, deadline * scale), stroke: 0.8pt + black)
            let y = 0
            for task in tasks-by-processor.at(p) {
              let len = lengths.at(task)
              let col = colors.at(task)
              draw.rect(
                (x0, y),
                (x0 + width, y + len * scale),
                fill: col.transparentize(25%),
                stroke: 0.4pt + col,
              )
              draw.content(
                (x0 + width / 2, y + len * scale / 2),
                text(7pt, fill: white)[$t_#(task + 1)$],
              )
              y += len * scale
            }
            draw.content((x0 + width / 2, -0.3), text(8pt)[$P_#(p + 1)$])
            draw.content((x0 + width / 2, deadline * scale + 0.25), text(7pt)[$L_#(p + 1) = #loads.at(p)$])
          }

          draw.line(
            (-0.15, deadline * scale),
            (max-x + 0.15, deadline * scale),
            stroke: (dash: "dashed", paint: luma(150), thickness: 0.5pt),
          )
          draw.content((-0.45, deadline * scale), text(7pt)[$D$])
        })
      },
      caption: [Canonical Multiprocessor Scheduling instance with 5 tasks on 2 processors. Stacked blocks show the satisfying assignment $(1, 2, 2, 2, 1)$; both processor loads equal the deadline $D = 10$.],
      ) <fig:multiprocessor-scheduling>
    ]
  ]
}

#{
  let x = load-model-example("PrecedenceConstrainedScheduling")
  let n = x.instance.num_tasks
  let m = x.instance.num_processors
  let D = x.instance.deadline
  let precs = x.instance.precedences
  let sigma = x.optimal_config
  // Group tasks by assigned slot
  let tasks-by-slot = range(D).map(s =>
    range(n).filter(i => sigma.at(i) == s)
  )
  [
    #problem-def("PrecedenceConstrainedScheduling")[
      Given a set $T$ of $n$ unit-length tasks, a partial order $prec$ on $T$, a number $m in ZZ^+$ of processors, and a deadline $D in ZZ^+$, determine whether there exists a schedule $sigma: T -> {0, dots, D-1}$ such that (i) for every time slot $t$, at most $m$ tasks are assigned to $t$, and (ii) for every precedence $t_i prec t_j$, we have $sigma(t_j) >= sigma(t_i) + 1$.
    ][
      Precedence Constrained Scheduling is problem SS9 in Garey & Johnson @garey1979. NP-complete via reduction from 3SAT @ullman1975. Remains NP-complete even for $D = 3$ @lenstra1978. Solvable in polynomial time for $m = 2$ by the Coffman--Graham algorithm @coffman1972, for forest-structured precedences @hu1961, and for chordal complement precedences @papadimitriou1979. A subset dynamic programming approach solves the general case in $O(2^n dot n)$ time by enumerating subsets of completed tasks at each time step.

      *Example.* Let $n = #n$ tasks, $m = #m$ processors, $D = #D$. Precedences: #precs.map(p => $t_#(p.at(0)) prec t_#(p.at(1))$).join(", "). A feasible schedule assigns $sigma = (#sigma.map(s => str(s)).join(", "))$: #range(D).map(s => [slot #s has ${#tasks-by-slot.at(s).map(i => $t_#i$).join(", ")}$]).join(", "). All precedences are satisfied and no slot exceeds $m = #m$.
    ]
  ]
}

#{
  let x = load-model-example("SchedulingWithIndividualDeadlines")
  let ntasks = x.instance.num_tasks
  let nproc = x.instance.num_processors
  let deadlines = x.instance.deadlines
  let precs = x.instance.precedences
  let start = x.optimal_config
  let horizon = deadlines.fold(0, (acc, d) => if d > acc { d } else { acc })
  let slot-groups = range(horizon).map(slot => range(ntasks).filter(t => start.at(t) == slot))
  let tight-tasks = range(ntasks).filter(t => start.at(t) + 1 == deadlines.at(t))
  let start-label = start.map(v => str(v)).join(", ")
  let deadline-pairs = deadlines.enumerate().map(((t, d)) => [$d(t_#(t + 1)) = #d$])
  let slot-summaries = slot-groups.enumerate().map(((slot, tasks)) => [slot #slot: #tasks.map(task => $t_#(task + 1)$).join(", ")])
  let tight-task-labels = tight-tasks.map(task => $t_#(task + 1)$)
  [
    #problem-def("SchedulingWithIndividualDeadlines")[
      Given a set $T$ of $n$ unit-length tasks, a number $m in ZZ^+$ of identical processors, a deadline function $d: T -> ZZ^+$, and a partial order $prec.eq$ on $T$, determine whether there exists a schedule $sigma: T -> {0, 1, dots, D - 1}$, where $D = max_(t in T) d(t)$, such that every task meets its own deadline ($sigma(t) + 1 <= d(t)$), every precedence constraint is respected (if $t_i prec.eq t_j$ then $sigma(t_i) + 1 <= sigma(t_j)$), and at most $m$ tasks are scheduled in each time slot.
    ][
      Scheduling With Individual Deadlines is the parallel-machine feasibility problem catalogued as A5 SS11 in Garey & Johnson @garey1979. Garey & Johnson record NP-completeness via reduction from Vertex Cover, and Brucker, Garey, and Johnson sharpen the complexity picture: the problem remains NP-complete for out-tree precedence constraints, but becomes polynomial-time solvable for in-trees @bruckerGareyJohnson1977. The two-processor case is also polynomial-time solvable @garey1979.

      The direct encoding in this library uses one start-time variable per task, with each variable ranging over its allowable deadline window. If $D = max_t d(t)$, exhaustive search over that encoding yields an $O^*(D^n)$ brute-force bound.#footnote[This is the worst-case search bound induced by the implementation's configuration space; deadlines can be smaller on individual tasks, so practical instances may enumerate fewer than $D^n$ assignments.]

      *Example.* Consider $n = #ntasks$ tasks on $m = #nproc$ processors with deadlines #{deadline-pairs.join(", ")} and precedence constraints #{precs.map(p => [$t_#(p.at(0) + 1) prec.eq t_#(p.at(1) + 1)$]).join(", ")}. The sample schedule $sigma = [#start-label]$ assigns #{slot-summaries.join("; ")}. Every slot uses at most #nproc processors, and the tight tasks #{tight-task-labels.join(", ")} finish exactly at their deadlines.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (
            rgb("#4e79a7"),
            rgb("#e15759"),
            rgb("#76b7b2"),
            rgb("#f28e2b"),
            rgb("#59a14f"),
            rgb("#edc948"),
            rgb("#b07aa1"),
          )
          let scale = 1.25
          let row-h = 0.58
          let gap = 0.18

          for lane in range(nproc) {
            let y = -lane * (row-h + gap)
            content((-0.8, y), text(7pt, "P" + str(lane + 1)))
          }

          for (slot, tasks) in slot-groups.enumerate() {
            for (lane, task) in tasks.enumerate() {
              let x0 = slot * scale
              let x1 = (slot + 1) * scale
              let y = -lane * (row-h + gap)
              let color = colors.at(calc.rem(task, colors.len()))
              rect(
                (x0, y - row-h / 2),
                (x1, y + row-h / 2),
                fill: color.transparentize(30%),
                stroke: 0.4pt + color,
              )
              content(((x0 + x1) / 2, y), text(7pt)[$t_#(task + 1)$])
            }
          }

          let y-axis = -(nproc - 1) * (row-h + gap) - row-h / 2 - 0.2
          line((0, y-axis), (horizon * scale, y-axis), stroke: 0.4pt)
          for t in range(horizon + 1) {
            let x = t * scale
            line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
            content((x, y-axis - 0.24), text(6pt, str(t)))
          }
          content((horizon * scale / 2, y-axis - 0.46), text(7pt)[time slot])
        }),
        caption: [A feasible 3-processor schedule for Scheduling With Individual Deadlines. Tasks sharing a column run in the same unit-length time slot; the sample assignment uses slots $0, 1, 2$ and meets every deadline.],
      ) <fig:scheduling-with-individual-deadlines>
    ]
  ]
}
#{
  let x = load-model-example("SequencingWithinIntervals")
  let ntasks = x.instance.lengths.len()
  let release = x.instance.release_times
  let deadline = x.instance.deadlines
  let lengths = x.instance.lengths
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  // Compute start times from config offsets: start_i = release_i + config_i
  let starts = range(ntasks).map(i => release.at(i) + sol.config.at(i))
  let max-t = calc.max(..range(ntasks).map(i => deadline.at(i)))
  [
    #problem-def("SequencingWithinIntervals")[
      Given a finite set $T$ of tasks and, for each $t in T$, a release time $r(t) >= 0$, a deadline $d(t) >= 0$, and a processing length $ell(t) in ZZ^+$ satisfying $r(t) + ell(t) <= d(t)$, determine whether there exists a feasible schedule $sigma: T -> ZZ_(>= 0)$ such that for each $t in T$: (1) $sigma(t) >= r(t)$, (2) $sigma(t) + ell(t) <= d(t)$, and (3) for all $t' in T backslash {t}$, either $sigma(t') + ell(t') <= sigma(t)$ or $sigma(t') >= sigma(t) + ell(t)$.
    ][
      Sequencing Within Intervals is problem SS1 in Garey & Johnson @garey1979, proved NP-complete via reduction from Partition (Theorem 3.8). Each task $t$ must execute non-preemptively during the interval $[r(t), d(t))$, occupying $ell(t)$ consecutive time units on a single machine, and no two tasks may overlap.

      *Example.* Consider #ntasks tasks with overlapping availability windows:
      #align(center, table(
        columns: ntasks + 1,
        align: center,
        table.header([$"Task"$], ..range(ntasks).map(i => [$t_#(i + 1)$])),
        [$r(t)$], ..range(ntasks).map(i => [#release.at(i)]),
        [$d(t)$], ..range(ntasks).map(i => [#deadline.at(i)]),
        [$ell(t)$], ..range(ntasks).map(i => [#lengths.at(i)]),
      ))
      Each task can only start within its window $[r(t), d(t) - ell(t)]$, and the windows overlap, so finding a non-overlapping assignment is non-trivial. One feasible schedule places the tasks at #range(ntasks).map(i => $[#starts.at(i), #(starts.at(i) + lengths.at(i)))$).join($,$):

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#b07aa1"))
          let scale = 0.65
          let row-h = 0.6

          // Single-row Gantt chart
          for i in range(ntasks) {
            let s = starts.at(i)
            let e = s + lengths.at(i)
            let x0 = s * scale
            let x1 = e * scale
            let col = colors.at(i)
            rect((x0, -row-h / 2), (x1, row-h / 2),
              fill: col.transparentize(30%), stroke: 0.4pt + col)
            content(((x0 + x1) / 2, 0), text(6pt, [$t_#(i + 1)$]))
          }

          // Release-time and deadline markers
          for i in range(ntasks) {
            let col = colors.at(i)
            let rx = release.at(i) * scale
            line((rx, -row-h / 2 - 0.05), (rx, -row-h / 2 - 0.18), stroke: 0.5pt + col)
            let dx = deadline.at(i) * scale
            line((dx, row-h / 2 + 0.05), (dx, row-h / 2 + 0.18), stroke: 0.5pt + col)
          }

          content((-0.5, -row-h / 2 - 0.12), text(5pt)[$r$])
          content((-0.5, row-h / 2 + 0.12), text(5pt)[$d$])

          // Time axis
          let y-axis = -row-h / 2 - 0.35
          line((0, y-axis), (max-t * scale, y-axis), stroke: 0.4pt)
          for t in range(max-t + 1) {
            let x = t * scale
            line((x, y-axis), (x, y-axis - 0.08), stroke: 0.4pt)
            if calc.rem(t, 2) == 0 or t == max-t {
              content((x, y-axis - 0.22), text(5pt, str(t)))
            }
          }
          content((max-t * scale / 2, y-axis - 0.45), text(7pt)[$t$])
        }),
        caption: [A feasible schedule for the SWI instance. Ticks below and above mark release times $r$ and deadlines $d$ for each task.],
      ) <fig:swi>
    ]
  ]
}
#{
  let x = load-model-example("MinimumTardinessSequencing")
  let ntasks = x.instance.num_tasks
  let deadlines = x.instance.deadlines
  let precs = x.instance.precedences
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let tardy-count = sol.metric.Valid
  // Decode Lehmer code to permutation (schedule order)
  let lehmer = sol.config
  let schedule = {
    let avail = range(ntasks)
    let result = ()
    for c in lehmer {
      result.push(avail.at(c))
      avail = avail.enumerate().filter(((i, v)) => i != c).map(((i, v)) => v)
    }
    result
  }
  // Compute inverse: task-pos[task] = position
  let task-pos = range(ntasks).map(task => {
    schedule.enumerate().filter(((p, t)) => t == task).at(0).at(0)
  })
  // Identify tardy tasks
  let tardy-tasks = range(ntasks).filter(t => task-pos.at(t) + 1 > deadlines.at(t))
  [
    #problem-def("MinimumTardinessSequencing")[
      Given a set $T$ of $n$ unit-length tasks, a deadline function $d: T -> ZZ^+$, and a partial order $prec.eq$ on $T$, find a one-machine schedule $sigma: T -> {1, 2, dots, n}$ that respects the precedence constraints (if $t_i prec.eq t_j$ then $sigma(t_i) < sigma(t_j)$) and minimizes the number of _tardy_ tasks, i.e., tasks $t$ with $sigma(t) > d(t)$.
    ][
      Minimum Tardiness Sequencing is a classical NP-complete scheduling problem catalogued as SS2 in Garey & Johnson @garey1979. In standard scheduling notation it is written $1 | "prec", p_j = 1 | sum U_j$, where $U_j = 1$ if job $j$ finishes after its deadline and $U_j = 0$ otherwise.

      The problem is NP-complete by reduction from Clique (Theorem 3.10 in @garey1979). When the precedence constraints are empty, the problem becomes solvable in $O(n log n)$ time by Moore's algorithm @moore1968: sort tasks by deadline and greedily schedule each task on time, removing the task with the largest processing time whenever a deadline violation occurs. With arbitrary precedence constraints and unit processing times, the problem remains strongly NP-hard.

      *Example.* Consider $n = #ntasks$ tasks with deadlines $d = (#deadlines.map(v => str(v)).join(", "))$ and precedence constraint #{precs.map(p => [$t_#(p.at(0)) prec.eq t_#(p.at(1))$]).join(", ")}. An optimal schedule places tasks in order $(#schedule.map(t => $t_#t$).join(", "))$, giving #tardy-count tardy #if tardy-count == 1 [task] else [tasks]#{if tardy-tasks.len() > 0 [ ($#{tardy-tasks.map(t => $t_#t$).join(", ")}$ #if tardy-tasks.len() == 1 [finishes] else [finish] after #if tardy-tasks.len() == 1 [its deadline] else [their deadlines])]}.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"))
          let scale = 1.2
          let row-h = 0.6

          // Draw schedule blocks (single machine, unit-length tasks)
          for (pos, task) in schedule.enumerate() {
            let x0 = pos * scale
            let x1 = (pos + 1) * scale
            let is-tardy = tardy-tasks.contains(task)
            let fill = colors.at(calc.rem(task, colors.len())).transparentize(if is-tardy { 70% } else { 30% })
            let stroke-color = colors.at(calc.rem(task, colors.len()))
            rect((x0, -row-h / 2), (x1, row-h / 2),
              fill: fill, stroke: 0.4pt + stroke-color)
            content(((x0 + x1) / 2, 0), text(7pt, $t_#task$))
            // Deadline marker for this task
            let dl = deadlines.at(task)
            if dl <= ntasks {
              let dl-x = dl * scale
              line((dl-x, row-h / 2 + 0.05 + task * 0.12), (dl-x, row-h / 2 + 0.15 + task * 0.12),
                stroke: (paint: if is-tardy { red } else { green.darken(20%) }, thickness: 0.6pt))
            }
          }

          // Time axis
          let y-axis = -row-h / 2 - 0.2
          line((0, y-axis), (ntasks * scale, y-axis), stroke: 0.4pt)
          for t in range(ntasks + 1) {
            let x = t * scale
            line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
            content((x, y-axis - 0.25), text(6pt, str(t + 1)))
          }
          content((ntasks * scale / 2, y-axis - 0.45), text(7pt)[finish time])
        }),
        caption: [Optimal schedule for #ntasks tasks. #if tardy-tasks.len() > 0 [Faded #if tardy-tasks.len() == 1 [block indicates the] else [blocks indicate] tardy #if tardy-tasks.len() == 1 [task] else [tasks] (finish time exceeds deadline).] else [All tasks meet their deadlines.]],
      ) <fig:mts>
    ]
  ]
}

#{
  let x = load-model-example("SequencingToMinimizeWeightedCompletionTime")
  let lengths = x.instance.lengths
  let weights = x.instance.weights
  let precs = x.instance.precedences
  let ntasks = lengths.len()
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let opt = sol.metric.Valid
  let lehmer = sol.config
  let schedule = {
    let avail = range(ntasks)
    let result = ()
    for c in lehmer {
      result.push(avail.at(c))
      avail = avail.enumerate().filter(((i, v)) => i != c).map(((i, v)) => v)
    }
    result
  }
  let starts = ()
  let finishes = ()
  let elapsed = 0
  for task in schedule {
    starts.push(elapsed)
    elapsed += lengths.at(task)
    finishes.push(elapsed)
  }
  let total-time = elapsed
  [
    #problem-def("SequencingToMinimizeWeightedCompletionTime")[
      Given a set $T$ of $n$ tasks, a processing-time function $l: T -> ZZ^+$, a weight function $w: T -> ZZ^+$, and a partial order $prec.eq$ on $T$, find a one-machine schedule minimizing $sum_(t in T) w(t) C(t)$, where $C(t)$ is the completion time of task $t$ and every precedence relation $t_i prec.eq t_j$ requires task $t_i$ to complete before task $t_j$ starts.
    ][
      Sequencing to Minimize Weighted Completion Time is the single-machine precedence-constrained scheduling problem catalogued as SS4 in Garey & Johnson @garey1979, usually written $1 | "prec" | sum w_j C_j$. Lawler showed that arbitrary precedence constraints make the problem NP-complete, while series-parallel precedence orders admit an $O(n log n)$ algorithm @lawler1978. Without precedence constraints, Smith's ratio rule orders jobs by non-increasing $w_j / l_j$ and is optimal @smith1956.

      *Example.* Consider tasks with lengths $l = (#lengths.map(v => str(v)).join(", "))$, weights $w = (#weights.map(v => str(v)).join(", "))$, and precedence constraints #{precs.map(p => [$t_#(p.at(0)) prec.eq t_#(p.at(1))$]).join(", ")}. An optimal schedule is $(#schedule.map(t => $t_#t$).join(", "))$, with completion times $(#finishes.map(v => str(v)).join(", "))$ along the machine timeline and objective value $#opt$.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"))
          let scale = 0.55
          let row-h = 0.7

          for (pos, task) in schedule.enumerate() {
            let x0 = starts.at(pos) * scale
            let x1 = finishes.at(pos) * scale
            let color = colors.at(calc.rem(task, colors.len()))
            rect((x0, -row-h / 2), (x1, row-h / 2),
              fill: color.transparentize(30%), stroke: 0.4pt + color)
            content(((x0 + x1) / 2, 0), text(7pt, $t_#task$))
          }

          let y-axis = -row-h / 2 - 0.22
          line((0, y-axis), (total-time * scale, y-axis), stroke: 0.4pt)
          for t in range(total-time + 1) {
            let x = t * scale
            line((x, y-axis), (x, y-axis - 0.08), stroke: 0.4pt)
            content((x, y-axis - 0.22), text(6pt, str(t)))
          }
          content((total-time * scale / 2, y-axis - 0.45), text(7pt)[time])
        }),
        caption: [Optimal single-machine schedule for the canonical weighted-completion-time instance. Each block width equals the processing time $l_j$.],
      ) <fig:stmwct>
    ]
  ]
}

#{
  let x = load-model-example("SequencingToMinimizeWeightedTardiness")
  let lengths = x.instance.lengths
  let weights = x.instance.weights
  let deadlines = x.instance.deadlines
  let bound = x.instance.bound
  let njobs = lengths.len()
  let lehmer = x.optimal_config
  let schedule = {
    let avail = range(njobs)
    let result = ()
    for c in lehmer {
      result.push(avail.at(c))
      avail = avail.enumerate().filter(((i, v)) => i != c).map(((i, v)) => v)
    }
    result
  }
  let completions = {
    let t = 0
    let result = ()
    for job in schedule {
      t += lengths.at(job)
      result.push(t)
    }
    result
  }
  let tardiness = schedule.enumerate().map(((pos, job)) => calc.max(0, completions.at(pos) - deadlines.at(job)))
  let weighted = schedule.enumerate().map(((pos, job)) => tardiness.at(pos) * weights.at(job))
  let total-weighted = weighted.fold(0, (acc, v) => acc + v)
  let tardy-jobs = schedule.enumerate().filter(((pos, job)) => tardiness.at(pos) > 0).map(((pos, job)) => job)
  [
    #problem-def("SequencingToMinimizeWeightedTardiness")[
      Given a set $J$ of $n$ jobs, processing times $ell_j in ZZ^+$, tardiness weights $w_j in ZZ^+$, deadlines $d_j in ZZ^+$, and a bound $K in ZZ^+$, determine whether there exists a one-machine schedule whose total weighted tardiness
      $sum_(j in J) w_j max(0, C_j - d_j)$
      is at most $K$, where $C_j$ is the completion time of job $j$.
    ][
      Sequencing to Minimize Weighted Tardiness is the classical single-machine scheduling problem $1 || sum w_j T_j$, where $T_j = max(0, C_j - d_j)$. It appears as SS5 in Garey & Johnson @garey1979 and is strongly NP-complete via transformation from 3-Partition, which rules out pseudo-polynomial algorithms in general. When all weights are equal, the special case reduces to ordinary total tardiness and admits a pseudo-polynomial dynamic program @lawler1977. Garey & Johnson also note that the equal-length case is polynomial-time solvable by bipartite matching @garey1979.

      Exact algorithms remain exponential in the worst case. Brute-force over all $n!$ schedules evaluates the implementation's decision encoding in $O(n! dot n)$ time. More refined exact methods include the branch-and-bound algorithm of Potts and Van Wassenhove @potts1985 and the dynamic-programming style exact algorithm of Tanaka, Fujikuma, and Araki @tanaka2009.

      *Example.* Consider the five jobs with processing times $ell = (#lengths.map(v => str(v)).join(", "))$, weights $w = (#weights.map(v => str(v)).join(", "))$, deadlines $d = (#deadlines.map(v => str(v)).join(", "))$, and bound $K = #bound$. The unique satisfying schedule is $(#schedule.map(job => $t_#(job + 1)$).join(", "))$, with completion times $(#completions.map(v => str(v)).join(", "))$. Only job $t_#(tardy-jobs.at(0) + 1)$ is tardy; the per-job weighted tardiness contributions are $(#weighted.map(v => str(v)).join(", "))$, so the total weighted tardiness is $#total-weighted <= K$.

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"))
          let scale = 0.34
          let row-h = 0.7
          let y = 0

          for (pos, job) in schedule.enumerate() {
            let start = if pos == 0 { 0 } else { completions.at(pos - 1) }
            let end = completions.at(pos)
            let is-tardy = tardiness.at(pos) > 0
            let fill = colors.at(calc.rem(job, colors.len())).transparentize(if is-tardy { 70% } else { 30% })
            let stroke = colors.at(calc.rem(job, colors.len()))
            rect((start * scale, y - row-h / 2), (end * scale, y + row-h / 2),
              fill: fill, stroke: 0.4pt + stroke)
            content(((start + end) * scale / 2, y), text(7pt, $t_#(job + 1)$))

            let dl = deadlines.at(job)
            line((dl * scale, y + row-h / 2 + 0.05), (dl * scale, y + row-h / 2 + 0.2),
              stroke: (paint: if is-tardy { red } else { green.darken(20%) }, thickness: 0.6pt))
          }

          let axis-y = -row-h / 2 - 0.25
          line((0, axis-y), (completions.at(completions.len() - 1) * scale, axis-y), stroke: 0.4pt)
          for t in range(completions.at(completions.len() - 1) + 1) {
            let x = t * scale
            line((x, axis-y), (x, axis-y - 0.08), stroke: 0.4pt)
            content((x, axis-y - 0.22), text(6pt, str(t)))
          }
          content((completions.at(completions.len() - 1) * scale / 2, axis-y - 0.42), text(7pt)[time])
        }),
        caption: [Single-machine schedule for the canonical weighted-tardiness example. The faded job is tardy; colored ticks mark the individual deadlines $d_j$.],
      ) <fig:weighted-tardiness>
    ]
  ]
}

#{
  let x = load-model-example("SequencingToMinimizeMaximumCumulativeCost")
  let costs = x.instance.costs
  let precs = x.instance.precedences
  let bound = x.instance.bound
  let ntasks = costs.len()
  let lehmer = x.optimal_config
  let schedule = {
    let avail = range(ntasks)
    let result = ()
    for c in lehmer {
      result.push(avail.at(c))
      avail = avail.enumerate().filter(((i, v)) => i != c).map(((i, v)) => v)
    }
    result
  }
  let prefix-sums = {
    let running = 0
    let result = ()
    for task in schedule {
      running += costs.at(task)
      result.push(running)
    }
    result
  }
  [
    #problem-def("SequencingToMinimizeMaximumCumulativeCost")[
      Given a set $T$ of $n$ tasks, a precedence relation $prec.eq$ on $T$, an integer cost function $c: T -> ZZ$ (negative values represent profits), and a bound $K in ZZ$, determine whether there exists a one-machine schedule $sigma: T -> {1, 2, dots, n}$ that respects the precedence constraints and satisfies
      $sum_(sigma(t') lt.eq sigma(t)) c(t') lt.eq K$
      for every task $t in T$.
    ][
      Sequencing to Minimize Maximum Cumulative Cost is the scheduling problem SS7 in Garey & Johnson @garey1979. It is NP-complete by transformation from Register Sufficiency, even when every task cost is in ${-1, 0, 1}$ @garey1979. The problem models precedence-constrained task systems with resource consumption and release, where a negative cost corresponds to a profit or resource refund accumulated as the schedule proceeds.

      When the precedence constraints form a series-parallel digraph, #cite(<abdelWahabKameda1978>, form: "prose") gave a polynomial-time algorithm running in $O(n^2)$ time. #cite(<monmaSidney1979>, form: "prose") placed the problem in a broader family of sequencing objectives solvable efficiently on series-parallel precedence structures. The implementation here uses Lehmer-code enumeration of task orders, so the direct exact search induced by the model runs in $O(n!)$ time.

      *Example.* Consider $n = #ntasks$ tasks with costs $(#costs.map(c => str(c)).join(", "))$, precedence constraints #{precs.map(p => [$t_#(p.at(0) + 1) prec.eq t_#(p.at(1) + 1)$]).join(", ")}, and bound $K = #bound$. The sample schedule $(#schedule.map(t => $t_#(t + 1)$).join(", "))$ has cumulative sums $(#prefix-sums.map(v => str(v)).join(", "))$, so every prefix stays at or below $K = #bound$.

      #figure(
        {
          let pos = rgb("#f28e2b")
          let neg = rgb("#76b7b2")
          let zero = rgb("#bab0ab")
          align(center, stack(dir: ttb, spacing: 0.35cm,
            stack(dir: ltr, spacing: 0.08cm,
              ..schedule.enumerate().map(((i, task)) => {
                let cost = costs.at(task)
                let fill = if cost > 0 {
                  pos.transparentize(70%)
                } else if cost < 0 {
                  neg.transparentize(65%)
                } else {
                  zero.transparentize(65%)
                }
                stack(dir: ttb, spacing: 0.05cm,
                  box(width: 1.0cm, height: 0.6cm, fill: fill, stroke: 0.4pt + luma(120),
                    align(center + horizon, text(8pt, weight: "bold")[$t_#(task + 1)$])),
                  text(6pt, if cost >= 0 { $+ #cost$ } else { $#cost$ }),
                )
              }),
            ),
            stack(dir: ltr, spacing: 0.08cm,
              ..prefix-sums.map(v => {
                box(width: 1.0cm, align(center + horizon, text(7pt)[$#v$]))
              }),
            ),
            text(7pt, [prefix sums after each scheduled task]),
          ))
        },
        caption: [A satisfying schedule for Sequencing to Minimize Maximum Cumulative Cost. Orange boxes add cost, teal boxes release cost, and the displayed prefix sums $(#prefix-sums.map(v => str(v)).join(", "))$ never exceed $K = #bound$.],
      ) <fig:seq-max-cumulative>
    ]
  ]
}

#problem-def("DirectedTwoCommodityIntegralFlow")[
  Given a directed graph $G = (V, A)$ with arc capacities $c: A -> ZZ^+$, two source-sink pairs $(s_1, t_1)$ and $(s_2, t_2)$, and requirements $R_1, R_2 in ZZ^+$, determine whether there exist two integral flow functions $f_1, f_2: A -> ZZ_(>= 0)$ such that (1) $f_1(a) + f_2(a) <= c(a)$ for all $a in A$, (2) flow $f_i$ is conserved at every vertex except $s_1, s_2, t_1, t_2$, and (3) the net flow into $t_i$ under $f_i$ is at least $R_i$ for $i in {1, 2}$.
][
  Directed Two-Commodity Integral Flow is a fundamental NP-complete problem in multicommodity flow theory, catalogued as ND38 in Garey & Johnson @garey1979. While single-commodity max-flow is solvable in polynomial time and fractional multicommodity flow reduces to linear programming, requiring integral flows with just two commodities makes the problem NP-complete.

  NP-completeness was proved by Even, Itai, and Shamir via reduction from 3-SAT @even1976. The problem remains NP-complete even when all arc capacities are 1 and $R_1 = 1$. No sub-exponential exact algorithm is known; brute-force enumeration over $(C + 1)^(2|A|)$ flow assignments dominates, where $C = max_(a in A) c(a)$.#footnote[No algorithm improving on brute-force is known for Directed Two-Commodity Integral Flow.]

  *Example.* Consider a directed graph with 6 vertices and 8 arcs (all with unit capacity), sources $s_1 = 0$, $s_2 = 1$, sinks $t_1 = 4$, $t_2 = 5$, and requirements $R_1 = R_2 = 1$. Commodity 1 routes along the path $0 -> 2 -> 4$ and commodity 2 along $1 -> 3 -> 5$, satisfying all capacity and conservation constraints.

  #figure(
    canvas(length: 1cm, {
      import draw: *
      let positions = (
        (0, 1),    // 0 = s1
        (0, -1),   // 1 = s2
        (2, 1),    // 2
        (2, -1),   // 3
        (4, 1),    // 4 = t1
        (4, -1),   // 5 = t2
      )
      let labels = ($s_1$, $s_2$, $2$, $3$, $t_1$, $t_2$)
      let arcs = ((0, 2), (0, 3), (1, 2), (1, 3), (2, 4), (2, 5), (3, 4), (3, 5))
      // Commodity 1 path: arcs 0 (0->2) and 4 (2->4)
      let c1-arcs = (0, 4)
      // Commodity 2 path: arcs 3 (1->3) and 7 (3->5)
      let c2-arcs = (3, 7)

      // Draw arcs
      for (idx, (u, v)) in arcs.enumerate() {
        let from = positions.at(u)
        let to = positions.at(v)
        let color = if c1-arcs.contains(idx) { blue } else if c2-arcs.contains(idx) { red } else { gray.darken(20%) }
        let thickness = if c1-arcs.contains(idx) or c2-arcs.contains(idx) { 1.2pt } else { 0.6pt }
        line(from, to, stroke: (paint: color, thickness: thickness), mark: (end: "straight", scale: 0.5))
      }

      // Draw vertices
      for (k, pos) in positions.enumerate() {
        let fill = if k == 0 or k == 4 { blue.lighten(70%) } else if k == 1 or k == 5 { red.lighten(70%) } else { white }
        circle(pos, radius: 0.3, fill: fill, stroke: 0.6pt, name: str(k))
        content(pos, text(8pt, labels.at(k)))
      }
    }),
    caption: [Two-commodity flow: commodity 1 (blue, $s_1 -> 2 -> t_1$) and commodity 2 (red, $s_2 -> 3 -> t_2$).],
  ) <fig:d2cif>
]

#problem-def("AdditionalKey")[
  Given a set $A$ of attribute names, a collection $F$ of functional dependencies on $A$,
  a subset $R subset.eq A$, and a set $K$ of candidate keys for the relational scheme $chevron.l R, F chevron.r$,
  determine whether there exists a subset $R' subset.eq R$ such that $R' in.not K$,
  the closure $R'^+$ under $F$ equals $R$, and no proper subset of $R'$ also has this property.
][
  A classical NP-complete problem from relational database theory @beeri1979.
  Enumerating all candidate keys is necessary to verify Boyce-Codd Normal Form (BCNF),
  and the NP-completeness of Additional Key implies that BCNF testing is intractable in general.
  The best known exact algorithm is brute-force enumeration of all $2^(|R|)$ subsets,
  checking each for the key property via closure computation under Armstrong's axioms.
  #footnote[No algorithm improving on brute-force is known for the Additional Key problem.]

  *Example.* Consider attribute set $A = {0, 1, 2, 3, 4, 5}$ with functional dependencies
  $F = {{0,1} -> {2,3}, {2,3} -> {4,5}, {4,5} -> {0,1}, {0,2} -> {3}, {3,5} -> {1}}$,
  relation $R = A$, and known keys $K = {{0,1}, {2,3}, {4,5}}$.
  The subset ${0,2}$ is an additional key: starting from ${0,2}$, we apply ${0,2} -> {3}$
  to get ${0,2,3}$, then ${2,3} -> {4,5}$ to get ${0,2,3,4,5}$, then ${4,5} -> {0,1}$
  to reach $R^+ = A$. The set ${0,2}$ is minimal (neither ${0}$ nor ${2}$ alone determines $A$)
  and ${0,2} in.not K$, so the answer is YES.
]


#{
  let x = load-model-example("ConjunctiveBooleanQuery")
  let d = x.instance.domain_size
  let nv = x.instance.num_variables
  let rels = x.instance.relations
  let conj = x.instance.conjuncts
  let nr = rels.len()
  let nc = conj.len()
  let assignment = x.optimal_config
  [
    #problem-def("ConjunctiveBooleanQuery")[
      Given a finite domain $D = {0, dots, d - 1}$, a collection of relations $R_0, R_1, dots, R_(m-1)$ where each $R_i$ is a set of $a_i$-tuples with entries from $D$, and a conjunctive Boolean query
      $ Q = (exists y_0, y_1, dots, y_(l-1))(A_0 and A_1 and dots.c and A_(r-1)) $
      where each _atom_ $A_j$ has the form $R_(i_j)(u_0, u_1, dots)$ with every $u$ in ${y_0, dots, y_(l-1)} union D$, determine whether there exists an assignment to the variables that makes $Q$ true --- i.e., the resolved tuple of every atom belongs to its relation.
    ][
      The Conjunctive Boolean Query (CBQ) problem is one of the most fundamental problems in database theory and finite model theory. #cite(<chandra1977>, form: "prose") showed that evaluating conjunctive queries is NP-complete by reduction from the Clique problem. CBQ is equivalent to the Constraint Satisfaction Problem (CSP) and to the homomorphism problem for relational structures; this equivalence connects database query evaluation, constraint programming, and graph theory under a single computational framework @kolaitis1998.

      For queries of bounded _hypertree-width_, evaluation becomes polynomial-time @gottlob2002. The general brute-force algorithm enumerates all $d^l$ variable assignments and checks every atom, running in $O(d^l dot r dot max_i a_i)$ time.#footnote[No substantially faster general algorithm is known for arbitrary conjunctive Boolean queries.]

      *Example.* Let $D = {0, dots, #(d - 1)}$ ($d = #d$), with #nr relations:

      #align(center, grid(
        columns: nr,
        gutter: 1.5em,
        ..range(nr).map(ri => {
          let rel = rels.at(ri)
          let arity = rel.arity
          let header = range(arity).map(j => [$c_#j$])
          table(
            columns: arity + 1,
            align: center,
            inset: (x: 4pt, y: 3pt),
            table.header([$R_#ri$], ..header),
            table.hline(stroke: 0.3pt),
            ..rel.tuples.enumerate().map(((ti, tup)) => {
              let cells = tup.map(v => [#v])
              ([$tau_#ti$], ..cells)
            }).flatten()
          )
        })
      ))

      The query has #nv variables $(y_0, y_1)$ and #nc atoms:
      #{
        let fmt-arg(a) = {
          if "Variable" in a { $y_#(a.Variable)$ }
          else { $#(a.Constant)$ }
        }
        let atoms = conj.enumerate().map(((j, c)) => {
          let ri = c.at(0)
          let args = c.at(1)
          [$A_#j = R_#ri (#args.map(fmt-arg).join($, $))$]
        })
        [$ Q = (exists y_0, y_1)(#atoms.join($ and $)) $]
      }

      Under the assignment $y_0 = #assignment.at(0)$, $y_1 = #assignment.at(1)$: atom $A_0$ resolves to $(#assignment.at(0), 3) in R_0$ (row $tau_0$), atom $A_1$ resolves to $(#assignment.at(1), 3) in R_0$ (row $tau_1$), and atom $A_2$ resolves to $(#assignment.at(0), #assignment.at(1), 5) in R_1$ (row $tau_0$). All three atoms are satisfied, so $Q$ is true.
    ]
  ]
}

#{
  let x = load-model-example("ConsecutiveOnesSubmatrix")
  let A = x.instance.matrix
  let m = A.len()
  let n = A.at(0).len()
  let K = x.instance.bound
  // Convert bool matrix to int for display
  let A-int = A.map(row => row.map(v => if v { 1 } else { 0 }))
  // Use the canonical witness {0, 1, 3}
  let cfg = x.optimal_config
  // Selected column indices
  let selected = cfg.enumerate().filter(((i, v)) => v == 1).map(((i, v)) => i)
  [
    #problem-def("ConsecutiveOnesSubmatrix")[
      Given an $m times n$ binary matrix $A$ and an integer $K$ with $0 <= K <= n$, determine whether there exists a subset of $K$ columns of $A$ whose columns can be permuted so that in each row all 1's occur consecutively (the _consecutive ones property_).
    ][
      The Consecutive Ones Property (C1P) --- that the columns of a binary matrix can be ordered so that all 1's in each row are contiguous --- is fundamental in computational biology (DNA physical mapping), interval graph recognition, and PQ-tree algorithms. Testing whether a full matrix has the C1P is polynomial: Booth and Lueker @booth1976 gave a linear-time PQ-tree algorithm running in $O(m + n + f)$ where $f$ is the number of 1-entries. However, finding the largest column subset with the C1P is NP-complete, proven by Booth @booth1975 via transformation from Hamiltonian Path. This implementation permits the vacuous case $K = 0$, where the empty submatrix is immediately satisfying. The best known exact algorithm is brute-force enumeration of all $binom(n, K)$ column subsets, testing each for the C1P in $O(m + n)$ time#footnote[No algorithm improving on brute-force subset enumeration is known for the general Consecutive Ones Submatrix problem.].

      *Example.* Consider the $#m times #n$ matrix $A = mat(#A-int.map(row => row.map(v => str(v)).join(", ")).join("; "))$ with $K = #K$. Selecting columns $\{#selected.map(i => str(i)).join(", ")\}$ yields a $#m times #K$ submatrix. Under column permutation $[1, 0, 3]$, each row's 1-entries are contiguous: row 1 has $[1, 1, 1]$, row 2 has $[0, 1, 1]$, and row 3 has $[1, 0, 0]$. The full $3 times 4$ matrix does _not_ have the C1P (it contains a Tucker obstruction), but two of the four 3-column subsets do.

      #figure(
        canvas(length: 0.7cm, {
          import draw: *
          let cell-size = 0.9
          let gap = 0.15
          // Draw the original matrix
          for i in range(m) {
            for j in range(n) {
              let val = A-int.at(i).at(j)
              let is-selected = cfg.at(j) == 1
              let f = if val == 1 {
                if is-selected { graph-colors.at(0).transparentize(30%) } else { luma(200) }
              } else { white }
              rect(
                (j * cell-size, -i * cell-size),
                (j * cell-size + cell-size - gap, -i * cell-size - cell-size + gap),
                fill: f,
                stroke: 0.3pt + luma(180),
              )
              content(
                (j * cell-size + (cell-size - gap) / 2, -i * cell-size - (cell-size - gap) / 2),
                text(8pt, str(val)),
              )
            }
          }
          // Column labels
          for j in range(n) {
            content(
              (j * cell-size + (cell-size - gap) / 2, 0.4),
              text(7pt)[$c_#j$],
            )
          }
          // Row labels
          for i in range(m) {
            content(
              (-0.5, -i * cell-size - (cell-size - gap) / 2),
              text(7pt)[$r_#(i + 1)$],
            )
          }
        }),
        caption: [Binary matrix $A$ ($#m times #n$) with $K = #K$. Blue-highlighted columns $\{#selected.map(i => str(i)).join(", ")\}$ form a submatrix with the consecutive ones property under a suitable column permutation. Grey cells are 1-entries in non-selected columns.],
      ) <fig:c1s-example>
    ]
  ]
}

// Completeness check: warn about problem types in JSON but missing from paper
#{
  let json-models = {
    let names = graph-data.nodes.map(n => n.name)
    let unique = ()
    for n in names { if n not in unique { unique.push(n) } }
    unique
  }
  let defined = display-name.keys()
  let missing = json-models.filter(n => n not in defined)
  if missing.len() > 0 {
    block(width: 100%, inset: (x: 1em, y: 0.5em), fill: rgb("#fff3cd"), stroke: (left: 3pt + rgb("#ffc107")))[
      #text(fill: rgb("#856404"), weight: "bold")[Warning: Missing problem definitions for:]
      #text(fill: rgb("#856404"))[ #missing.join(", ")]
    ]
  }
}

= Reductions <sec:reductions>

Each reduction is presented as a *Rule* (with linked problem names and overhead from the graph data), followed by a *Proof* (construction, correctness, variable mapping, solution extraction), and optionally a *Concrete Example* (a small instance with verified solution). Problem names in the rule title link back to their definitions in @sec:problems.

== Trivial Reductions

#let mvc_mis = load-example("MinimumVertexCover", "MaximumIndependentSet")
#let mvc_mis_sol = mvc_mis.solutions.at(0)
#reduction-rule("MinimumVertexCover", "MaximumIndependentSet",
  example: true,
  example-caption: [Petersen graph ($n = 10$): VC $arrow.l.r$ IS],
  extra: [
    Source VC: $C = {#mvc_mis_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => str(i)).join(", ")}$ (size #mvc_mis_sol.source_config.filter(x => x == 1).len()) #h(1em)
    Target IS: $S = {#mvc_mis_sol.target_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => str(i)).join(", ")}$ (size #mvc_mis_sol.target_config.filter(x => x == 1).len()) \
    $|"VC"| + |"IS"| = #graph-num-vertices(mvc_mis.source.instance) = |V|$ #sym.checkmark
  ],
)[
  Vertex cover and independent set are set complements: removing a cover from $V$ leaves vertices with no edges between them (an independent set), and vice versa. Since $|S| + |C| = |V|$ is constant, maximizing one is equivalent to minimizing the other. The reduction preserves the graph and weights unchanged.
][
  _Construction._ Given VC instance $(G, bold(w))$, create IS instance $(G, bold(w))$ with identical graph and weights. Variables correspond one-to-one: vertex $v$ in the source maps to vertex $v$ in the target.

  _Correctness._ ($arrow.r.double$) If $C$ is a vertex cover, then for any $u, v in V backslash C$, the edge $(u, v) in.not E$ (otherwise $C$ would miss it), so $V backslash C$ is independent. ($arrow.l.double$) If $S$ is independent, then for any $(u, v) in E$, at most one endpoint lies in $S$, so $V backslash S$ covers every edge. Since $|S| + |C| = |V|$ is constant, a minimum vertex cover corresponds to a maximum independent set.

  _Solution extraction._ For IS solution $S$, return $C = V backslash S$, i.e.\ flip each variable: $c_v = 1 - s_v$.
]

#let mvc_fvs = load-example("MinimumVertexCover", "MinimumFeedbackVertexSet")
#let mvc_fvs_sol = mvc_fvs.solutions.at(0)
#let mvc_fvs_cover = mvc_fvs_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#let mvc_fvs_fvs = mvc_fvs_sol.target_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#reduction-rule("MinimumVertexCover", "MinimumFeedbackVertexSet",
  example: true,
  example-caption: [7-vertex graph: each source edge becomes a directed 2-cycle],
  extra: [
    Source VC: $C = {#mvc_fvs_cover.map(str).join(", ")}$ (size #mvc_fvs_cover.len()) on a graph with $n = #graph-num-vertices(mvc_fvs.source.instance)$ vertices and $|E| = #graph-num-edges(mvc_fvs.source.instance)$ edges \
    Target FVS: $F = {#mvc_fvs_fvs.map(str).join(", ")}$ (size #mvc_fvs_fvs.len()) on a digraph with the same $n = #graph-num-vertices(mvc_fvs.target.instance)$ vertices and $|A| = #mvc_fvs.target.instance.graph.arcs.len() = 2 |E|$ arcs \
    Canonical witness is preserved exactly: $C = F$ #sym.checkmark
  ],
)[
  Each undirected edge $\{u, v\}$ can be viewed as the directed 2-cycle $u -> v -> u$. Replacing every source edge this way turns the task "hit every edge with a chosen endpoint" into "hit every directed cycle with a chosen vertex." The vertex set, weights, and budget are preserved, so the reduction is size-preserving up to doubling the edge count into arcs.
][
  _Construction._ Given a Minimum Vertex Cover instance $(G = (V, E), bold(w))$, build the directed graph $D = (V, A)$ on the same vertex set, where for every undirected edge $\{u, v\} in E$ we add both arcs $(u, v)$ and $(v, u)$ to $A$. Keep the vertex weights unchanged and reuse the same decision variables $x_v in {0,1}$.

  _Correctness._ ($arrow.r.double$) If $C subset.eq V$ is a vertex cover of $G$, then every source edge $\{u, v\}$ has an endpoint in $C$, so the corresponding 2-cycle $u -> v -> u$ in $D$ is hit by $C$. Any longer directed cycle in $D$ is also made from source edges, so one of its vertices lies in $C$ as well. Therefore removing $C$ destroys all directed cycles, and $C$ is a feedback vertex set of $D$. ($arrow.l.double$) If $F subset.eq V$ is a feedback vertex set of $D$, then for every source edge $\{u, v\}$ the digraph contains the 2-cycle $u -> v -> u$, which must be hit by $F$. Hence at least one of $u, v$ lies in $F$, so $F$ covers every edge of $G$ and is a vertex cover.

  _Solution extraction._ Return the target solution vector unchanged: a selected vertex in the feedback vertex set is selected in the vertex cover, and vice versa.
]

#reduction-rule("MaximumIndependentSet", "MinimumVertexCover")[
  The exact reverse of VC $arrow.r$ IS: complementing an independent set yields a vertex cover. The graph and weights are preserved unchanged, and $|"IS"| + |"VC"| = |V|$ ensures optimality carries over.
][
  _Construction._ Given IS instance $(G, bold(w))$, create VC instance $(G, bold(w))$ with identical graph and weights.

  _Correctness._ ($arrow.r.double$) If $S$ is independent, no edge has both endpoints in $S$, so every edge has at least one endpoint in $V backslash S$, making $V backslash S$ a cover. ($arrow.l.double$) If $C$ is a vertex cover, every edge is incident to some vertex in $C$, so no edge connects two vertices of $V backslash C$, making $V backslash C$ independent.

  _Solution extraction._ For VC solution $C$, return $S = V backslash C$, i.e.\ flip each variable: $s_v = 1 - c_v$.
]

#let gp_mc = load-example("GraphPartitioning", "MaxCut")
#let gp_mc_sol = gp_mc.solutions.at(0)
#let gp_mc_source_edges = gp_mc.source.instance.graph.edges.map(e => (e.at(0), e.at(1)))
#let gp_mc_target_edges = gp_mc.target.instance.graph.edges.map(e => (e.at(0), e.at(1)))
#let gp_mc_weights = gp_mc.target.instance.edge_weights
#let gp_mc_nv = gp_mc.source.instance.graph.num_vertices
#let gp_mc_ne = gp_mc_source_edges.len()
#let gp_mc_penalty = gp_mc_ne + 1
#let gp_mc_side_a = range(gp_mc_nv).filter(i => gp_mc_sol.source_config.at(i) == 0)
#let gp_mc_side_b = range(gp_mc_nv).filter(i => gp_mc_sol.source_config.at(i) == 1)
#let gp_mc_weight_lo = gp_mc_target_edges.enumerate().filter(((i, e)) => gp_mc_weights.at(i) == gp_mc_penalty - 1).map(((i, e)) => e)
#let gp_mc_weight_hi = gp_mc_target_edges.enumerate().filter(((i, e)) => gp_mc_weights.at(i) == gp_mc_penalty).map(((i, e)) => e)
#let gp_mc_source_cross = gp_mc_source_edges.filter(e => gp_mc_sol.source_config.at(e.at(0)) != gp_mc_sol.source_config.at(e.at(1)))
#let gp_mc_cut_lo = gp_mc_target_edges.enumerate().filter(((i, e)) =>
  gp_mc_weights.at(i) == gp_mc_penalty - 1 and
  gp_mc_sol.target_config.at(e.at(0)) != gp_mc_sol.target_config.at(e.at(1))
).map(((i, e)) => e)
#let gp_mc_cut_hi = gp_mc_target_edges.enumerate().filter(((i, e)) =>
  gp_mc_weights.at(i) == gp_mc_penalty and
  gp_mc_sol.target_config.at(e.at(0)) != gp_mc_sol.target_config.at(e.at(1))
).map(((i, e)) => e)
#let gp_mc_cut_value = gp_mc_target_edges.enumerate().filter(((i, e)) =>
  gp_mc_sol.target_config.at(e.at(0)) != gp_mc_sol.target_config.at(e.at(1))
).map(((i, e)) => gp_mc_weights.at(i)).sum(default: 0)
#reduction-rule("GraphPartitioning", "MaxCut",
  example: true,
  example-caption: [6-vertex minimum bisection to weighted Max-Cut],
  extra: [
    Here $m = #gp_mc_ne$, so $P = m + 1 = #gp_mc_penalty$ \
    Weight $#(gp_mc_penalty - 1)$ edges (original edges): {#gp_mc_weight_lo.map(e => $(v_#(e.at(0)), v_#(e.at(1)))$).join(", ")} \
    Weight $#gp_mc_penalty$ edges (non-edges): {#gp_mc_weight_hi.map(e => $(v_#(e.at(0)), v_#(e.at(1)))$).join(", ")} \
    Canonical witness $A = {#gp_mc_side_a.map(i => $v_#i$).join(", ")}$, $B = {#gp_mc_side_b.map(i => $v_#i$).join(", ")}$ cuts source edges {#gp_mc_source_cross.map(e => $(v_#(e.at(0)), v_#(e.at(1)))$).join(", ")} and attains weighted cut $#gp_mc_cut_lo.len() * #(gp_mc_penalty - 1) + #gp_mc_cut_hi.len() * #gp_mc_penalty = #gp_mc_cut_value$ #sym.checkmark
  ],
)[
  @garey1976 Graph Partitioning minimizes cut edges subject to a perfect-balance constraint, while Max-Cut maximizes a weighted cut without any balance constraint. A standard folklore construction in combinatorial optimization removes that constraint by rewarding every cross-pair equally and then subtracting one unit on original edges. The resulting weighted complete graph forces every optimum to be balanced first, and among balanced cuts it exactly minimizes the original bisection width.
][
  _Construction._ Given a Graph Partitioning instance $G = (V, E)$ with $n = |V|$ and $m = |E|$, set $P = m + 1$. Build the complete graph $G' = (V, E')$ on the same vertex set, where $E'$ contains every unordered pair $\{u, v\}$ with $u != v$. Assign weight $w'_(u, v) = P - 1$ when $(u, v) in E$, and $w'_(u, v) = P$ otherwise. For any partition $(A, B)$ of $V$, the weighted cut in $G'$ is
  $ "cut"_(G')(A, B) = P |A| |B| - "cut"_G(A, B). $

  _Correctness._ ($arrow.r.double$) Let $(A, B)$ be a maximum cut of $G'$. If it were unbalanced, then $|A| |B|$ would be at least one smaller than for a balanced partition $(A', B')$. Hence
  $ "cut"_(G')(A', B') - "cut"_(G')(A, B) >= P - ("cut"_G(A', B') - "cut"_G(A, B)) >= P - m > 0, $
  because $0 <= "cut"_G(·, ·) <= m$ and $P = m + 1$. Therefore every maximum cut of $G'$ is balanced. Among balanced partitions, $P |A| |B| = P (n slash 2)^2$ is constant, so maximizing $"cut"_(G')(A, B)$ is equivalent to minimizing $"cut"_G(A, B)$. ($arrow.l.double$) Conversely, every minimum bisection of $G$ is balanced and therefore maximizes $P |A| |B| - "cut"_G(A, B)$ in $G'$.

  _Solution extraction._ Read off the same partition vector on the original vertex set: the Max-Cut bit for vertex $v$ is already the Graph Partitioning bit for $v$.
]

#let mis_clique = load-example("MaximumIndependentSet", "MaximumClique")
#let mis_clique_sol = mis_clique.solutions.at(0)
#reduction-rule("MaximumIndependentSet", "MaximumClique",
  example: true,
  example-caption: [Path graph $P_5$: IS $arrow.r$ Clique via complement],
  extra: [
    Source IS: $S = {#mis_clique_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => str(i)).join(", ")}$ (size #mis_clique_sol.source_config.filter(x => x == 1).len()) #h(1em)
    Target Clique: $C = {#mis_clique_sol.target_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => str(i)).join(", ")}$ (size #mis_clique_sol.target_config.filter(x => x == 1).len()) \
    Source $|E| = #graph-num-edges(mis_clique.source.instance)$, complement $|overline(E)| = #graph-num-edges(mis_clique.target.instance)$ #sym.checkmark
  ],
)[
  An independent set in $G$ is exactly a clique in the complement graph $overline(G)$: vertices with no edges between them in $G$ are pairwise adjacent in $overline(G)$. Both problems maximize total vertex weight, so optimal values are preserved. This is Karp's classical complement graph reduction.
][
  _Construction._ Given IS instance $(G = (V, E), bold(w))$, build $overline(G) = (V, overline(E))$ where $overline(E) = {(u, v) : u != v, (u, v) in.not E}$. Create MaxClique instance $(overline(G), bold(w))$ with the same weights. Variables correspond one-to-one: vertex $v$ in the source maps to vertex $v$ in the target.

  _Correctness._ ($arrow.r.double$) If $S$ is independent in $G$, then for any $u, v in S$, $(u, v) in.not E$, so $(u, v) in overline(E)$ — all pairs in $S$ are adjacent in $overline(G)$, making $S$ a clique. ($arrow.l.double$) If $C$ is a clique in $overline(G)$, then for any $u, v in C$, $(u, v) in overline(E)$, so $(u, v) in.not E$ — no pair in $C$ is adjacent in $G$, making $C$ independent. Weight sums are identical, so optimality is preserved.

  _Solution extraction._ For clique solution $C$ in $overline(G)$, return IS $= C$ (identity mapping: $s_v = c_v$).
]

#reduction-rule("MaximumIndependentSet", "MaximumSetPacking")[
  The key insight is that two vertices are adjacent if and only if they share an edge. By representing each vertex $v$ as the set of its incident edges $S_v$, adjacency becomes set overlap: $S_u inter S_v != emptyset$ iff $(u,v) in E$. Thus an independent set (no two adjacent) maps exactly to a packing (no two overlapping).
][
  _Construction._ Universe $U = E$ (edges, indexed $0, ..., |E|-1$). For each vertex $v$, define $S_v = {e in E : v in e}$ (the set of edge indices incident to $v$), with weight $w(S_v) = w(v)$. Variables correspond one-to-one: vertex $v$ maps to set $S_v$.

  _Correctness._ ($arrow.r.double$) If $I$ is independent, then for any $u, v in I$, edge $(u,v) in.not E$, so $S_u inter S_v = emptyset$ — the sets are mutually disjoint, forming a valid packing. ($arrow.l.double$) If ${S_v : v in P}$ is a packing, then for any $u, v in P$, $S_u inter S_v = emptyset$, meaning $u$ and $v$ share no edge, so $P$ is independent. Weight sums are identical, so optimality is preserved.

  _Solution extraction._ For packing ${S_v : v in P}$, return IS $= P$ (same variable assignment).
]

#reduction-rule("MaximumSetPacking", "MaximumIndependentSet")[
  The _intersection graph_ captures set overlap as adjacency: two sets that share an element become neighbors, so a packing (mutually disjoint sets) corresponds exactly to an independent set (mutually non-adjacent vertices). This is the standard reduction from set packing to independent set.
][
  _Construction._ Build the intersection graph $G' = (V', E')$: create one vertex $v_i$ per set $S_i$ ($i = 1, ..., m$), and add edge $(v_i, v_j)$ iff $S_i inter S_j != emptyset$. Set $w(v_i) = w(S_i)$. Variables correspond one-to-one: set $S_i$ maps to vertex $v_i$.

  _Correctness._ ($arrow.r.double$) If $cal(P)$ is a packing (all sets mutually disjoint), then for any $S_i, S_j in cal(P)$, $S_i inter S_j = emptyset$, so $(v_i, v_j) in.not E'$, meaning ${v_i : S_i in cal(P)}$ is independent. ($arrow.l.double$) If $I subset.eq V'$ is independent, then for any $v_i, v_j in I$, $(v_i, v_j) in.not E'$, so $S_i inter S_j = emptyset$, meaning ${S_i : v_i in I}$ is a valid packing. Weight sums match, so optimality is preserved.

  _Solution extraction._ For IS $I subset.eq V'$, return packing $cal(P) = {S_i : v_i in I}$ (same variable assignment).
]

#reduction-rule("MinimumVertexCover", "MinimumSetCovering")[
  A vertex cover must "hit" every edge; set covering must "hit" every universe element. By making each edge a universe element and each vertex the set of its incident edges, the two covering conditions become identical. This is the canonical embedding of vertex cover as a special case of set covering.
][
  _Construction._ Universe $U = {0, ..., |E|-1}$ (one element per edge). For each vertex $v$, define $S_v = {i : e_i "incident to" v}$ (the indices of edges touching $v$), with weight $w(S_v) = w(v)$. Variables correspond one-to-one: vertex $v$ maps to set $S_v$.

  _Correctness._ ($arrow.r.double$) If $C$ is a vertex cover, every edge $e_i$ has at least one endpoint $v in C$, so $i in S_v$ for some selected set — hence $union.big_(v in C) S_v = U$, a valid covering. ($arrow.l.double$) If ${S_v : v in C}$ covers $U$, then every edge index $i in U$ appears in some $S_v$ with $v in C$, meaning edge $e_i$ is incident to some $v in C$ — hence $C$ is a vertex cover. Weight sums are identical, so optimality is preserved.

  _Solution extraction._ For covering ${S_v : v in C}$, return VC $= C$ (same variable assignment).
]

#reduction-rule("MaximumMatching", "MaximumSetPacking")[
  A matching selects edges that share no endpoints; set packing selects sets that share no elements. By representing each edge as the 2-element set of its endpoints and using vertices as the universe, two edges conflict (share an endpoint) if and only if their sets overlap. This embeds matching as a special case of set packing where every set has size exactly 2.
][
  _Construction._ Universe $U = V$ (vertices, indexed $0, ..., |V|-1$). For each edge $e = (u, v)$, define $S_e = {u, v}$ with weight $w(S_e) = w(e)$. Variables correspond one-to-one: edge $e$ maps to set $S_e$.

  _Correctness._ ($arrow.r.double$) If $M$ is a matching, then for any $e_1, e_2 in M$, the edges share no endpoint, so $S_(e_1) inter S_(e_2) = emptyset$ — the sets are mutually disjoint, forming a valid packing. ($arrow.l.double$) If ${S_e : e in P}$ is a packing, then for any $e_1, e_2 in P$, $S_(e_1) inter S_(e_2) = emptyset$, meaning the edges share no vertex, so $P$ is a valid matching. Weight sums are identical, so optimality is preserved.

  _Solution extraction._ For packing ${S_e : e in P}$, return matching $= P$ (same variable assignment).
]

#reduction-rule("QUBO", "SpinGlass")[
  QUBO uses binary variables $x_i in {0,1}$; the Ising model uses spin variables $s_i in {-1,+1}$. The affine substitution $x_i = (s_i + 1)\/2$ converts between the two encodings. Since every quadratic binary function maps to a quadratic spin function (and vice versa), the two models are polynomially equivalent. This is the reverse of SpinGlass $arrow.r$ QUBO.
][
  _Construction._ Substitute $x_i = (s_i + 1)\/2$ into $f(bold(x)) = sum_(i <= j) Q_(i j) x_i x_j$. For diagonal terms ($i = j$): $Q_(i i) x_i = Q_(i i)(s_i + 1)\/2$, contributing $Q_(i i)\/2$ to $h_i$. For off-diagonal terms ($i < j$): $Q_(i j) x_i x_j = Q_(i j)(s_i + 1)(s_j + 1)\/4$, contributing $Q_(i j)\/4$ to $J_(i j)$, $Q_(i j)\/4$ to both $h_i$ and $h_j$, plus a constant. Collecting terms:
  $ J_(i j) = Q_(i j) / 4, quad h_i = 1/2 (Q_(i i) + sum_(j != i) Q_(i j) / 2) $

  _Correctness._ ($arrow.r.double$) Any binary assignment $bold(x)$ maps to a spin assignment $bold(s)$ with $s_i = 2 x_i - 1$, and the QUBO objective equals the Ising energy up to a global constant. ($arrow.l.double$) Any spin ground state maps back to a binary minimizer via $x_i = (s_i + 1)\/2$. The constant offset does not affect the argmin.

  _Solution extraction._ Convert spins to binary: $x_i = (s_i + 1) \/ 2$, i.e.\ $s_i = +1 arrow.r x_i = 1$, $s_i = -1 arrow.r x_i = 0$.
]

#let sg_qubo = load-example("SpinGlass", "QUBO")
#let sg_qubo_sol = sg_qubo.solutions.at(0)
#reduction-rule("SpinGlass", "QUBO",
  example: true,
  example-caption: [10-spin Ising model on Petersen graph],
  extra: [
    Source: $n = #spin-num-spins(sg_qubo.source.instance)$ spins, $h_i = 0$, couplings $J_(i j) in {plus.minus 1}$ \
    Mapping: $s_i = 2x_i - 1$ converts spins ${-1, +1}$ to binary ${0, 1}$ \
    Canonical ground-state witness: $bold(x) = (#sg_qubo_sol.target_config.map(str).join(", "))$ #sym.checkmark
  ],
)[
  The Ising model and QUBO are both quadratic functions over finite domains: spins ${-1,+1}$ and binary variables ${0,1}$, respectively. The affine map $s_i = 2x_i - 1$ establishes a bijection between the two domains and preserves the quadratic structure. Substituting into the Ising Hamiltonian yields a QUBO objective that differs from the original energy by a constant, so ground states correspond exactly.
][
  _Construction._ Substitute $s_i = 2x_i - 1$ into $H = -sum_(i<j) J_(i j) s_i s_j - sum_i h_i s_i$. Expanding:
  $ s_i s_j = (2x_i - 1)(2x_j - 1) = 4x_i x_j - 2x_i - 2x_j + 1 $
  Collecting terms and using $x_i^2 = x_i$:
  $ Q_(i j) = -4 J_(i j) quad (i < j), quad Q_(i i) = 2 sum_(j != i) J_(i j) - 2 h_i $
  The constant offset $-sum_(i<j) J_(i j) + sum_i h_i$ does not affect the minimizer.

  _Correctness._ ($arrow.r.double$) Any spin configuration $bold(s)$ maps to a unique binary vector $bold(x)$ via $x_i = (s_i + 1)\/2$, and $H_"SG"(bold(s)) = H_"QUBO"(bold(x)) + "const"$, so a ground state of the Ising model maps to a QUBO minimizer. ($arrow.l.double$) Any QUBO minimizer $bold(x)$ maps back to spins $s_i = 2x_i - 1$ with the same energy relationship, so optimality is preserved in both directions.

  _Solution extraction._ Convert binary to spins: $s_i = 2x_i - 1$, i.e.\ $x_i = 1 arrow.r s_i = +1$, $x_i = 0 arrow.r s_i = -1$.
]

#let cvp_qubo = load-example("ClosestVectorProblem", "QUBO")
#let cvp_qubo_sol = cvp_qubo.solutions.at(0)
#{
  let basis = cvp_qubo.source.instance.basis
  let bounds = cvp_qubo.source.instance.bounds
  let target = cvp_qubo.source.instance.target
  let offsets = cvp_qubo_sol.source_config
  let coords = offsets.enumerate().map(((i, off)) => off + bounds.at(i).lower)
  let matrix = cvp_qubo.target.instance.matrix
  let bits = cvp_qubo_sol.target_config
  let lo = bounds.map(b => b.lower)
  let anchor = range(target.len()).map(d => lo.enumerate().fold(0.0, (acc, (i, x)) => acc + x * basis.at(i).at(d)))
  let constant = range(target.len()).fold(0.0, (acc, d) => acc + calc.pow(anchor.at(d) - target.at(d), 2))
  let qubo-value = range(bits.len()).fold(0.0, (acc, i) => acc + if bits.at(i) == 0 { 0.0 } else {
    range(bits.len() - i).fold(0.0, (row-acc, delta) => row-acc + if bits.at(i + delta) == 0 { 0.0 } else { matrix.at(i).at(i + delta) })
  })
  let fmt-vec(v) = $paren.l #v.map(e => str(e)).join(", ") paren.r^top$
  let rounded-constant = calc.round(constant, digits: 2)
  let rounded-qubo = calc.round(qubo-value, digits: 1)
  let rounded-distance-sq = calc.round(qubo-value + constant, digits: 2)
  [
    #reduction-rule("ClosestVectorProblem", "QUBO",
      example: true,
      example-caption: [2D bounded CVP with two 3-bit exact-range encodings],
      extra: [
        *Step 1 -- Source instance.* The canonical CVP example uses basis columns $bold(b)_1 = #fmt-vec(basis.at(0))$ and $bold(b)_2 = #fmt-vec(basis.at(1))$, target $bold(t) = #fmt-vec(target)$, and bounds $x_1, x_2 in [#bounds.at(0).lower, #bounds.at(0).upper]$.

        *Step 2 -- Exact bounded encoding.* Each variable has #bounds.at(0).upper - bounds.at(0).lower + 1 admissible values, so the implementation uses the capped binary basis $(1, 2, 3)$ rather than $(1, 2, 4)$: the first two bits are powers of two, and the last weight is capped so every bit pattern reconstructs an offset in ${0, dots, 6}$. Thus
        $ x_1 = #bounds.at(0).lower + z_0 + 2 z_1 + 3 z_2, quad x_2 = #bounds.at(1).lower + z_3 + 2 z_4 + 3 z_5 $
        giving #cvp_qubo.target.instance.num_vars QUBO variables in total.

        *Step 3 -- Build the QUBO.* For this instance, $G = A^top A = ((4, 2), (2, 5))$ and $h = A^top bold(t) = (5.6, 5.8)^top$. Expanding the shifted quadratic form yields the exported upper-triangular matrix with representative entries $Q_(0,0) = #matrix.at(0).at(0)$, $Q_(0,1) = #matrix.at(0).at(1)$, $Q_(0,2) = #matrix.at(0).at(2)$, $Q_(2,5) = #matrix.at(2).at(5)$, and $Q_(5,5) = #matrix.at(5).at(5)$.

        *Step 4 -- Verify a solution.* The fixture stores the canonical witness $bold(z) = (#bits.map(str).join(", "))$, which extracts to source offsets $bold(c) = (#offsets.map(str).join(", "))$ and actual lattice coordinates $bold(x) = (#coords.map(str).join(", "))$. The QUBO value is $bold(z)^top Q bold(z) = #rounded-qubo$; adding back the dropped constant #rounded-constant yields the original squared distance #(rounded-distance-sq), so the extracted point is the closest lattice vector #sym.checkmark.

        *Multiplicity.* Offset $3$ has two bit encodings ($(0, 0, 1)$ and $(1, 1, 0)$), so the fixture stores one canonical witness even though the QUBO has multiple optimal binary assignments representing the same CVP solution.
      ],
    )[
      A bounded Closest Vector Problem instance already supplies a finite integer box $x_i in [ell_i, u_i]$ for each coefficient. Following the direct quadratic-form reduction of Canale, Qureshi, and Viola @canale2023qubo, encoding each offset $c_i = x_i - ell_i$ with an exact in-range binary basis turns the squared-distance objective into an unconstrained quadratic over binary variables. Unlike penalty-method encodings, no auxiliary feasibility penalty is needed: every bit pattern decodes to a legal coefficient vector by construction.
    ][
      _Construction._ Let $A in ZZ^(m times n)$ be the basis matrix with columns $bold(a)_1, dots, bold(a)_n$, let $bold(t) in RR^m$ be the target, and let $x_i in [ell_i, u_i]$ with range $r_i = u_i - ell_i$. Define $L_i = ceil(log_2(r_i + 1))$ when $r_i > 0$ and omit bits when $r_i = 0$. For each variable, introduce binary variables $z_(i,0), dots, z_(i,L_i-1)$ with exact-range weights
      $ w_(i,p) = 2^p quad (0 <= p < L_i - 1), quad w_(i,L_i-1) = r_i + 1 - 2^(L_i - 1) $
      so that every bit vector represents an offset in ${0, dots, r_i}$. Then
      $ x_i = ell_i + sum_(p=0)^(L_i-1) w_(i,p) z_(i,p) $
      and the total number of QUBO variables is $N = sum_i L_i$, exactly the exported overhead `num_vars = num_encoding_bits`.

      Let $G = A^top A$ and $h = A^top bold(t)$. Writing $bold(x) = bold(ell) + B bold(z)$ for the encoding matrix $B in RR^(n times N)$ gives
      $ norm(A bold(x) - bold(t))_2^2 = bold(z)^top (B^top G B) bold(z) + 2 bold(z)^top B^top (G bold(ell) - h) + "const" $
      where the constant $norm(A bold(ell) - bold(t))_2^2$ is dropped. Therefore the QUBO coefficients are
      $ Q_(u,u) = (B^top G B)_(u,u) + 2 (B^top (G bold(ell) - h))_u, quad Q_(u,v) = 2 (B^top G B)_(u,v) quad (u < v) $
      using the usual upper-triangular convention.

      _Correctness._ ($arrow.r.double$) Every binary vector $bold(z) in {0,1}^N$ decodes to a coefficient vector $bold(x)$ inside the prescribed bounds because each exact-range basis reaches only offsets in ${0, dots, r_i}$. Substituting this decoding into the CVP objective yields $bold(z)^top Q bold(z) + "const"$, so any QUBO minimizer maps to a bounded CVP minimizer. ($arrow.l.double$) Every bounded CVP solution $bold(x)$ has at least one bit encoding for each coordinate offset, hence at least one binary vector $bold(z)$ with the same objective value up to the dropped constant. Thus the minimizers correspond exactly, although several binary witnesses may decode to the same CVP solution.

      _Solution extraction._ For each source variable, sum its selected encoding weights to recover the source configuration offset $c_i = x_i - ell_i$. This is exactly the configuration format expected by the `ClosestVectorProblem` model.
    ]
  ]
}

== Penalty-Method QUBO Reductions <sec:penalty-method>

The _penalty method_ @glover2019 @lucas2014 converts a constrained optimization problem into an unconstrained QUBO by adding quadratic penalty terms. Given an objective $"obj"(bold(x))$ to minimize and constraints $g_k (bold(x)) = 0$, construct:
$ f(bold(x)) = "obj"(bold(x)) + P sum_k g_k (bold(x))^2 $
where $P$ is a penalty weight large enough that any constraint violation costs more than the entire objective range. Since $g_k (bold(x))^2 >= 0$ with equality iff $g_k (bold(x)) = 0$, minimizers of $f$ are feasible and optimal for the original problem. Because binary variables satisfy $x_i^2 = x_i$, the resulting $f$ is a quadratic in $bold(x)$, i.e.\ a QUBO.

#let kc_qubo = load-example("KColoring", "QUBO")
#let kc_qubo_sol = kc_qubo.solutions.at(0)
#reduction-rule("KColoring", "QUBO",
  example: true,
  example-caption: [House graph ($n = 5$, $|E| = 6$, $chi = 3$) with $k = 3$ colors],
  extra: [
    #{
      let hg = house-graph()
      let fills = kc_qubo_sol.source_config.map(c => graph-colors.at(c))
      align(center, canvas(length: 0.8cm, {
        for (u, v) in hg.edges { g-edge(hg.vertices.at(u), hg.vertices.at(v)) }
        for (k, pos) in hg.vertices.enumerate() {
          g-node(pos, name: str(k), fill: fills.at(k), label: str(k))
        }
      }))
    }

    *Step 1 -- Encode each color choice as a binary variable.* A coloring assigns each vertex one of $k$ colors. To express this in binary, introduce $k$ indicator variables per vertex: $x_(v,c) = 1$ means "vertex $v$ gets color $c$." For the house graph with $k = 3$, this gives $n k = 5 times 3 = 15$ QUBO variables:
    $ underbrace(x_(0,0) x_(0,1) x_(0,2), "vertex 0") #h(4pt) underbrace(x_(1,0) x_(1,1) x_(1,2), "vertex 1") #h(4pt) dots.c #h(4pt) underbrace(x_(4,0) x_(4,1) x_(4,2), "vertex 4") $

    *Step 2 -- Penalize invalid color assignments (one-hot constraint).* A valid coloring requires each vertex to have _exactly one_ color, i.e.\ $sum_c x_(v,c) = 1$. The penalty $(1 - sum_c x_(v,c))^2$ equals zero when exactly one variable is 1, and is positive otherwise. Weighted by $P_1 = 1 + n = 6$, this contributes diagonal entries $Q_(v k+c, v k+c) = -6$ and off-diagonal entries $Q_(v k+c_1, v k+c_2) = 12$ between colors of the same vertex. These form the $5 times 5$ diagonal blocks of $Q$.\

    *Step 3 -- Penalize same-color neighbors (edge conflict).* For each edge $(u,v) in E$ and each color $c$, the product $x_(u,c) x_(v,c) = 1$ iff both endpoints receive color $c$ — exactly the coloring conflict we want to forbid. The penalty $P_2 dot x_(u,c) x_(v,c)$ with $P_2 = P_1 slash 2 = 3$ makes such conflicts costly. The house has 6 edges, each contributing 3 color-conflict penalties $arrow.r$ 18 off-diagonal entries of value $3$ in $Q$.\

    *Step 4 -- Verify a solution.* The first valid 3-coloring is $(c_0, ..., c_4) = (#kc_qubo_sol.source_config.map(str).join(", "))$, shown in the figure above. The one-hot encoding is $bold(x) = (#kc_qubo_sol.target_config.map(str).join(", "))$. Check: each 3-bit group has exactly one 1 (valid one-hot #sym.checkmark), and for every edge the two endpoints have different colors (e.g.\ edge $0 dash 1$: colors $#kc_qubo_sol.source_config.at(0), #kc_qubo_sol.source_config.at(1)$ #sym.checkmark).\

    *Multiplicity:* The fixture stores one canonical coloring witness. The house graph has $3! times 3 = 18$ valid colorings overall: the triangle $2 dash 3 dash 4$ forces 3 distinct colors ($3! = 6$ permutations), and for each, the base vertices $0, 1$ have exactly $3$ compatible ordered pairs.
  ],
)[
  The $k$-coloring problem has two requirements: each vertex gets exactly one color, and adjacent vertices get different colors. Both can be expressed as quadratic penalties over binary variables. Introduce $n k$ binary variables $x_(v,c) in {0,1}$ (indexed by $v dot k + c$), where $x_(v,c) = 1$ means vertex $v$ receives color $c$. The first requirement becomes a _one-hot constraint_ penalizing vertices with zero or multiple colors; the second becomes an _edge conflict penalty_ penalizing same-color neighbors. The combined QUBO matrix $Q in RR^(n k times n k)$ encodes both penalties.
][
  _Construction._ Applying the penalty method (@sec:penalty-method), the two requirements translate into two penalty terms:
  $ f(bold(x)) = underbrace(P_1 sum_(v in V) (1 - sum_(c=1)^k x_(v,c))^2, "one-hot: exactly one color per vertex") + underbrace(P_2 sum_((u,v) in E) sum_(c=1)^k x_(u,c) x_(v,c), "edge conflict: neighbors differ") $

  _One-hot expansion._ The constraint $(1 - sum_c x_(v,c))^2$ penalizes any vertex with $!= 1$ active color. Expanding using $x_(v,c)^2 = x_(v,c)$ (binary variables):
  $ (1 - sum_c x_(v,c))^2 = 1 - sum_c x_(v,c) + 2 sum_(c_1 < c_2) x_(v,c_1) x_(v,c_2) $
  Reading off the QUBO coefficients: diagonal $Q_(v k+c, v k+c) = -P_1$ (favors assigning a color) and intra-vertex off-diagonal $Q_(v k+c_1, v k+c_2) = 2 P_1$ for $c_1 < c_2$ (discourages multiple colors).

  _Edge conflict._ For each edge $(u,v)$ and color $c$, the product $x_(u,c) x_(v,c)$ equals 1 iff both endpoints share color $c$. The penalty $P_2 x_(u,c) x_(v,c)$ adds $P_2$ to $Q_(u k+c, v k+c)$ (with appropriate index ordering).

  In our implementation, $P_1 = P = 1 + n$ and $P_2 = P\/2$. The penalty $P_1$ exceeds the number of vertices, ensuring that any constraint violation outweighs any objective gain.

  _Correctness._ ($arrow.r.double$) If $bold(x)$ violates any one-hot constraint (some vertex has 0 or $>= 2$ colors), the penalty $P_1 > n$ exceeds the objective range, so $bold(x)$ is not a minimizer. ($arrow.l.double$) Among valid one-hot encodings, $f$ reduces to the edge conflict term, minimized when no two adjacent vertices share a color — exactly the $k$-coloring objective.

  _Solution extraction._ For each vertex $v$, find $c$ with $x_(v,c) = 1$.
]

#reduction-rule("MaximumSetPacking", "QUBO")[
  Set packing selects mutually disjoint sets of maximum total weight. Two sets conflict if and only if they share a universe element — the same adjacency structure as an independent set on the _intersection graph_. This reduction builds the intersection graph implicitly and applies the IS penalty method directly: each set becomes a QUBO variable, diagonal entries reward selection, and off-diagonal entries penalize pairs of overlapping sets with a penalty large enough to forbid any overlap.
][
  _Construction._ Given sets $S_1, ..., S_m$ with weights $w_1, ..., w_m$, introduce binary variables $x_i in {0,1}$ for each set. Two sets $S_i, S_j$ _conflict_ iff $S_i inter S_j != emptyset$. The packing objective is: maximize $sum_i w_i x_i$ subject to $x_i x_j = 0$ for every conflicting pair. Applying the penalty method (@sec:penalty-method):
  $ f(bold(x)) = -sum_i w_i x_i + P sum_(S_i inter S_j != emptyset, thin i < j) x_i x_j $
  with $P = 1 + sum_i w_i$. The QUBO coefficients are: diagonal $Q_(i i) = -w_i$ (reward for selecting set $S_i$), off-diagonal $Q_(i j) = P$ for each conflicting pair $i < j$ (penalty for overlap).

  _Correctness._ ($arrow.r.double$) If $bold(x)$ encodes a maximum-weight packing, all selected sets are mutually disjoint, so all penalty terms vanish and $f(bold(x)) = -sum_(i in cal(P)) w_i$. Any assignment selecting overlapping sets incurs penalty $P > sum_i w_i$, making it suboptimal. ($arrow.l.double$) Among feasible assignments (no overlapping sets selected), the penalty terms vanish and $f(bold(x)) = -sum_(i in cal(P)) w_i$, minimized exactly when $cal(P)$ is a maximum-weight packing.

  _Solution extraction._ Return $bold(x)$ directly — each $x_i = 1$ indicates set $S_i$ is in the packing.
]

#reduction-rule("KSatisfiability", "QUBO")[
  Each clause in a $k$-SAT formula is falsified by exactly one assignment to its literals. For $k = 2$, this falsifying pattern is a product of two (possibly complemented) binary variables — already quadratic, so each clause maps directly to QUBO terms. For $k = 3$, the falsifying pattern $y_1 y_2 y_3$ is cubic; Rosenberg quadratization replaces the product $y_1 y_2$ with an auxiliary variable $a$, enforced by a penalty that makes $a != y_1 y_2$ suboptimal. The total QUBO counts unsatisfied clauses, so minimizers maximize satisfiability.
][
  *Case $k = 2$.*

  _Construction._ Each 2-literal clause has exactly one falsifying assignment (both literals false). The penalty for that assignment is a quadratic function of $x_i, x_j$:

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

  Summing over all clauses, $f(bold(x)) = sum_j "penalty"_j (bold(x))$ counts falsified clauses.

  _Correctness._ ($arrow.r.double$) Each penalty term is non-negative and equals 1 exactly when its clause is falsified. If $bold(x)$ satisfies all clauses, $f(bold(x)) = 0$. ($arrow.l.double$) Any minimizer of $f$ achieves the fewest falsified clauses, hence maximizes satisfiability.

  *Case $k = 3$ (Rosenberg quadratization).*

  _Construction._ For each clause $(ell_1 or ell_2 or ell_3)$, define complement variables $y_i = overline(ell_i)$ (so $y_i = x_i$ if the literal is negated, $y_i = 1 - x_i$ if positive). The clause is violated when $y_1 y_2 y_3 = 1$. This cubic penalty is reduced to quadratic form by introducing an auxiliary variable $a$ and the substitution $a = y_1 y_2$, enforced via a Rosenberg penalty with weight $M$:
  $ H = a dot y_3 + M (y_1 y_2 - 2 y_1 a - 2 y_2 a + 3a) $
  where $M = 2$ suffices. Each clause adds one auxiliary variable (indices $n, n+1, ..., n+m-1$), so the total QUBO has $n + m$ variables.

  _Correctness._ ($arrow.r.double$) If $a = y_1 y_2$, the Rosenberg penalty term vanishes and $H = y_1 y_2 y_3$ counts the clause violation faithfully. ($arrow.l.double$) If $a != y_1 y_2$, the penalty $M(dots.c) >= 1$ strictly exceeds the clause-counting contribution (at most 1), so any minimizer must have $a = y_1 y_2$ for every clause. Among such assignments, $H$ counts unsatisfied clauses, and minimizers maximize satisfiability.

  _Solution extraction._ Discard auxiliary variables: return $bold(x)[0..n]$.
]

#let ksat_ss = load-example("KSatisfiability", "SubsetSum")
#let ksat_ss_sol = ksat_ss.solutions.at(0)
#reduction-rule("KSatisfiability", "SubsetSum",
  example: true,
  example-caption: [3-SAT with 3 variables and 2 clauses],
  extra: [
    Source: $n = #ksat_ss.source.instance.num_vars$ variables, $m = #sat-num-clauses(ksat_ss.source.instance)$ clauses \
    Target: #subsetsum-num-elements(ksat_ss.target.instance) elements, target $= #ksat_ss.target.instance.target$ \
    Source config: #ksat_ss_sol.source_config #h(1em) Target config: #ksat_ss_sol.target_config
  ],
)[
  Base-10 digit encoding reduction following Sipser @sipser2012[Thm 7.56] and CLRS @cormen2022[§34.5.5]. (Karp @karp1972 established SubsetSum NP-completeness via Exact Cover; this direct 3-SAT construction is a later textbook formulation.) Each integer has $(n + m)$ digits, where the first $n$ positions correspond to variables and the last $m$ to clauses. For variable $x_i$, two integers $y_i, z_i$ encode positive and negative literal occurrences. For clause $C_j$, slack integers $g_j, h_j$ pad the clause digit to exactly 4. Since each clause has at most 3 literals and slacks add at most 2, no digit exceeds 5, so no carries occur.
][
  _Construction._ Given a 3-CNF formula $phi$ with $n$ variables and $m$ clauses, create $2n + 2m$ integers in $(n+m)$-digit base-10 representation:

  (i) _Variable integers_ ($2n$): For each $x_i$, create $y_i$ with $d_i = 1$ and $d_(n+j) = 1$ if $x_i in C_j$, and $z_i$ with $d_i = 1$ and $d_(n+j) = 1$ if $overline(x_i) in C_j$.

  (ii) _Slack integers_ ($2m$): For each clause $C_j$, create $g_j$ with $d_(n+j) = 1$ and $h_j$ with $d_(n+j) = 2$.

  (iii) _Target_ $T$: $d_i = 1$ for $i in [1, n]$ and $d_(n+j) = 4$ for $j in [1, m]$.

  _Correctness._ ($arrow.r.double$) If assignment $alpha$ satisfies $phi$, select $y_i$ when $x_i = top$ and $z_i$ when $x_i = bot$. Variable digits sum to exactly 1 (one of $y_i, z_i$ per variable). Each satisfied clause has 1--3 true literals contributing 1--3 to its digit; slacks $g_j, h_j$ with values 1, 2 can pad any value in ${1, 2, 3}$ to 4. ($arrow.l.double$) Variable digits force exactly one of $y_i, z_i$ per variable, defining a truth assignment. Clause digits reach 4 only if the literal contribution is $>= 1$, meaning each clause is satisfied.

  _Solution extraction._ For each $i$: if $y_i$ is selected ($x_(2i) = 1$), set $x_i = 1$; if $z_i$ is selected ($x_(2i+1) = 1$), set $x_i = 0$.
]

#{
  let ss-cvp = load-example("SubsetSum", "ClosestVectorProblem")
  let ss-cvp-sol = ss-cvp.solutions.at(0)
  let ss-cvp-sizes = ss-cvp.source.instance.sizes
  let ss-cvp-target = ss-cvp.source.instance.target
  let ss-cvp-basis = ss-cvp.target.instance.basis
  let ss-cvp-target-vec = ss-cvp.target.instance.target
  let ss-cvp-n = ss-cvp-sizes.len()
  let ss-cvp-x = ss-cvp-sol.target_config
  let to-mat(m) = math.mat(..m.map(row => row.map(v => $#v$)))
  [
    #reduction-rule("SubsetSum", "ClosestVectorProblem",
      example: true,
      example-caption: [#ss-cvp-n elements, target sum $B = #ss-cvp-target$],
      extra: [
        *Step 1 -- Source instance.* The canonical Subset Sum instance has sizes $(#ss-cvp-sizes.map(str).join(", "))$ and target $B = #ss-cvp-target$.

        *Step 2 -- Build the lattice.* The reduction creates the basis
        $ bold(B) = #to-mat(ss-cvp-basis) $
        together with target $ bold(t) = (#ss-cvp-target-vec.map(str).join(", "))^top $
        and binary bounds $x_i in {0,1}$ for all $#ss-cvp-n$ coordinates.

        *Step 3 -- Verify the canonical witness.* The fixture stores $bold(x) = (#ss-cvp-x.map(str).join(", "))$, which selects sizes $3$ and $8$ and therefore satisfies $3 + 8 = #ss-cvp-target$. Since $bold(B) bold(x) = (1, 0, 0, 1, #ss-cvp-target)^top$, the difference vector is $(0.5, -0.5, -0.5, 0.5, 0)^top$ and the Euclidean distance is $sqrt(#ss-cvp-n / 4) = 1$.

        *Witness semantics.* The example DB stores one canonical minimizer. This source instance also has another satisfying subset, $(1, 1, 1, 0)$, so the reduction has multiple optimal CVP witnesses even though only one is serialized.
      ],
    )[
      Classical lattice embedding for Subset Sum following Lagarias and Odlyzko @lagarias1985, with the $1/2$-target CVP formulation in the style of Coster et al. @coster1992. For an instance with $n$ elements, the reduction produces $n$ basis vectors in ambient dimension $n + 1$: the first $n$ coordinates enforce binary structure and the last coordinate records the subset sum error.
    ][
      _Construction._ Given sizes $s_0, dots, s_(n-1) in ZZ^+$ and target $B in ZZ^+$, define one basis vector per element:
      $ bold(b)_i = bold(e)_i + s_i bold(e)_(n+1) $
      for $i in {0, dots, n-1}$. Equivalently, the basis matrix has columns $bold(b)_0, dots, bold(b)_(n-1)$, so its first $n$ rows form the identity matrix and its last row is $(s_0, dots, s_(n-1))$. Set the target vector to
      $ bold(t) = (1/2, dots, 1/2, B)^top $
      and restrict every CVP variable to $x_i in {0, 1}$.

      _Correctness._ ($arrow.r.double$) If $bold(x) in {0,1}^n$ is a satisfying Subset Sum solution, then $sum_i s_i x_i = B$ and
      $ norm(bold(B) bold(x) - bold(t))_2^2 = sum_(i=0)^(n-1) (x_i - 1/2)^2 + (sum_i s_i x_i - B)^2 = n/4. $
      Hence every satisfying subset becomes a CVP solution at distance $sqrt(n / 4)$. ($arrow.l.double$) Conversely, binary bounds force every CVP candidate to lie in ${0,1}^n$. The first $n$ coordinates always contribute exactly $n/4$ to the squared distance, so a CVP minimizer attains distance $sqrt(n/4)$ if and only if the last coordinate contributes $0$, i.e. $sum_i s_i x_i = B$. When the Subset Sum instance is unsatisfiable, every binary vector has strictly larger distance.

      _Solution extraction._ Return the binary CVP vector unchanged.
    ]
  ]
}

#reduction-rule("ILP", "QUBO")[
  A binary ILP optimizes a linear objective over binary variables subject to linear constraints. The penalty method converts each equality constraint $bold(a)_k^top bold(x) = b_k$ into the quadratic penalty $(bold(a)_k^top bold(x) - b_k)^2$, which is zero if and only if the constraint is satisfied. Inequality constraints are first converted to equalities using binary slack variables with powers-of-two coefficients. The resulting unconstrained quadratic over binary variables is a QUBO whose matrix $Q$ combines the negated objective (as diagonal terms) with the expanded constraint penalties (as a Gram matrix $A^top A$).
][
  _Construction._ First, normalize all constraints to equalities. Inequalities $bold(a)_k^top bold(x) <= b_k$ become $bold(a)_k^top bold(x) + sum_(s=0)^(S_k - 1) 2^s y_(k,s) = b_k$ where $S_k = ceil(log_2 (b_k + 1))$ binary slack bits. For $>=$ constraints, the slack has a negative sign. The extended system is $A' bold(x)' = bold(b)$ with $bold(x)' = (bold(x), bold(y)) in {0,1}^(n')$. For minimization, negate $bold(c)$ to convert to maximization.

  Applying the penalty method (@sec:penalty-method), combine the negated objective with quadratic constraint penalties:
  $ f(bold(x)') = -bold(c')^top bold(x)' + P sum_(k=1)^m (bold(a)'_k^(top) bold(x)' - b_k)^2 $
  where $bold(c)' = (bold(c), bold(0))$ and $P = 1 + ||bold(c)||_1 + ||bold(b)||_1$. Expanding the quadratic penalty:
  $ sum_k (bold(a)'_k^(top) bold(x)' - b_k)^2 = bold(x)'^(top) A'^(top) A' bold(x)' - 2 bold(b)^top A' bold(x)' + ||bold(b)||_2^2 $
  Combining with $-bold(c')^top bold(x)'$ and dropping the constant $||bold(b)||_2^2$:
  $ Q = -"diag"(bold(c)' + 2P bold(b)^top A') + P A'^(top) A' $
  The diagonal contains linear terms (objective plus constraint); the upper triangle of $A'^(top) A'$ gives quadratic cross-terms.

  _Correctness._ ($arrow.r.double$) If $bold(x)'^*$ is an optimal ILP solution, then $A' bold(x)'^* = bold(b)$ and all penalty terms vanish, so $f(bold(x)'^*) = -bold(c')^top bold(x)'^*$. ($arrow.l.double$) If any constraint is violated, $(bold(a)'_k^(top) bold(x)' - b_k)^2 >= 1$ and the penalty $P > ||bold(c)||_1$ exceeds the entire objective range, so $bold(x)'$ cannot be a QUBO minimizer. Among feasible assignments (all penalties zero), $f$ reduces to $-bold(c')^top bold(x)'$, minimized at the ILP optimum.

  _Solution extraction._ Discard slack variables: return $bold(x)' [0..n]$.
]

#let ks_qubo = load-example("Knapsack", "QUBO")
#let ks_qubo_sol = ks_qubo.solutions.at(0)
#let ks_qubo_num_items = ks_qubo.source.instance.weights.len()
#let ks_qubo_num_slack = ks_qubo.target.instance.num_vars - ks_qubo_num_items
#let ks_qubo_penalty = 1 + ks_qubo.source.instance.values.fold(0, (a, b) => a + b)
#let ks_qubo_selected = ks_qubo_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#let ks_qubo_sel_weight = ks_qubo_selected.fold(0, (a, i) => a + ks_qubo.source.instance.weights.at(i))
#let ks_qubo_sel_value = ks_qubo_selected.fold(0, (a, i) => a + ks_qubo.source.instance.values.at(i))
#reduction-rule("Knapsack", "QUBO",
  example: true,
  example-caption: [$n = #ks_qubo_num_items$ items, capacity $C = #ks_qubo.source.instance.capacity$],
  extra: [
    *Step 1 -- Source instance.* The canonical knapsack instance has weights $(#ks_qubo.source.instance.weights.map(str).join(", "))$, values $(#ks_qubo.source.instance.values.map(str).join(", "))$, and capacity $C = #ks_qubo.source.instance.capacity$.

    *Step 2 -- Introduce slack variables.* The inequality $sum_i w_i x_i lt.eq C$ becomes an equality by adding $B = #ks_qubo_num_slack$ binary slack bits that encode unused capacity:
    $ #ks_qubo.source.instance.weights.enumerate().map(((i, w)) => $#w x_#i$).join($+$) + #range(ks_qubo_num_slack).map(j => $#calc.pow(2, j) s_#j$).join($+$) = #ks_qubo.source.instance.capacity $
    This gives $n + B = #ks_qubo_num_items + #ks_qubo_num_slack = #ks_qubo.target.instance.num_vars$ QUBO variables.

    *Step 3 -- Add the penalty objective.* With penalty $P = 1 + sum_i v_i = #ks_qubo_penalty$, the QUBO minimizes
    $ H = -(#ks_qubo.source.instance.values.enumerate().map(((i, v)) => $#v x_#i$).join($+$)) + #ks_qubo_penalty (#ks_qubo.source.instance.weights.enumerate().map(((i, w)) => $#w x_#i$).join($+$) + #range(ks_qubo_num_slack).map(j => $#calc.pow(2, j) s_#j$).join($+$) - #ks_qubo.source.instance.capacity)^2 $
    so any violation of the equality is more expensive than the entire knapsack value range.

    *Step 4 -- Verify a solution.* The QUBO ground state $bold(z) = (#ks_qubo_sol.target_config.map(str).join(", "))$ extracts to the knapsack choice $bold(x) = (#ks_qubo_sol.source_config.map(str).join(", "))$. This selects items $\{#ks_qubo_selected.map(str).join(", ")\}$ with total weight $#ks_qubo_selected.map(i => str(ks_qubo.source.instance.weights.at(i))).join(" + ") = #ks_qubo_sel_weight$ and total value $#ks_qubo_selected.map(i => str(ks_qubo.source.instance.values.at(i))).join(" + ") = #ks_qubo_sel_value$, so the slack bits are all zero and the penalty term vanishes #sym.checkmark.

    *Uniqueness:* The fixture stores one canonical optimal witness. The source optimum is unique because items $\{#ks_qubo_selected.map(str).join(", ")\}$ are the only feasible selection achieving value #ks_qubo_sel_value.
  ],
)[
  For a standard 0-1 Knapsack instance with nonnegative weights, nonnegative values, and nonnegative capacity, the inequality $sum_i w_i x_i lt.eq C$ is converted to equality using binary slack variables that encode the unused capacity. When $C > 0$, one can take $B = floor(log_2 C) + 1$ slack bits; when $C = 0$, a single slack bit also suffices. The penalty method (@sec:penalty-method) combines the negated value objective with a quadratic constraint penalty, producing a QUBO with $n + B$ binary variables.
][
  _Construction._ Given $n$ items with nonnegative weights $w_0, dots, w_(n-1)$, nonnegative values $v_0, dots, v_(n-1)$, and nonnegative capacity $C$, introduce $B = floor(log_2 C) + 1$ binary slack variables $s_0, dots, s_(B-1)$ when $C > 0$ (or one slack bit when $C = 0$) to convert the capacity inequality to equality:
  $ sum_(i=0)^(n-1) w_i x_i + sum_(j=0)^(B-1) 2^j s_j = C $
  Let $a_k$ denote the constraint coefficient of the $k$-th binary variable ($a_k = w_k$ for $k < n$, $a_(n+j) = 2^j$ for $j < B$). The QUBO objective is:
  $ f(bold(z)) = -sum_(i=0)^(n-1) v_i x_i + P (sum_k a_k z_k - C)^2 $
  where $bold(z) = (x_0, dots, x_(n-1), s_0, dots, s_(B-1))$ and $P = 1 + sum_i v_i$. Expanding the quadratic penalty using $z_k^2 = z_k$ (binary):
  $ Q_(k k) = P a_k^2 - 2 P C a_k - [k < n] v_k, quad Q_(i j) = 2 P a_i a_j quad (i < j) $

  _Correctness._ ($arrow.r.double$) If $bold(x)^*$ is a feasible knapsack solution with value $V^*$, then there exist slack values $bold(s)^*$ satisfying the equality constraint (encoding $C - sum w_i x_i^*$ in binary), so $f(bold(z)^*) = -V^*$. ($arrow.l.double$) If the equality constraint is violated, the penalty $(sum a_k z_k - C)^2 gt.eq 1$ contributes at least $P > sum_i v_i$ to the objective. Since all values are nonnegative, every feasible assignment has objective in the range $[-sum_i v_i, 0]$, so that penalty exceeds the entire feasible value range. Among feasible assignments (penalty zero), $f$ reduces to $-sum v_i x_i$, minimized at the knapsack optimum.

  _Solution extraction._ Discard slack variables: return $bold(z)[0..n]$.
]

#let mwc_qubo = load-example("MinimumMultiwayCut", "QUBO")
#let mwc_qubo_sol = mwc_qubo.solutions.at(0)
#let mwc_qubo_edges = mwc_qubo.source.instance.graph.edges.map(e => (e.at(0), e.at(1)))
#let mwc_qubo_weights = mwc_qubo.source.instance.edge_weights
#let mwc_qubo_terminals = mwc_qubo.source.instance.terminals
#let mwc_qubo_n = mwc_qubo.source.instance.graph.num_vertices
#let mwc_qubo_k = mwc_qubo_terminals.len()
#let mwc_qubo_nq = mwc_qubo_n * mwc_qubo_k
#let mwc_qubo_alpha = mwc_qubo_weights.fold(0, (a, w) => a + w) + 1
#let mwc_qubo_cut_indices = mwc_qubo_sol.source_config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
#let mwc_qubo_cut_cost = mwc_qubo_cut_indices.fold(0, (a, i) => a + mwc_qubo_weights.at(i))
#reduction-rule("MinimumMultiwayCut", "QUBO",
  example: true,
  example-caption: [$n = #mwc_qubo_n$ vertices, $k = #mwc_qubo_k$ terminals $T = {#mwc_qubo_terminals.map(str).join(", ")}$, $|E| = #mwc_qubo_edges.len()$ edges],
  extra: [
    *Step 1 -- Source instance.* The canonical graph has $n = #mwc_qubo_n$ vertices, $m = #mwc_qubo_edges.len()$ edges with weights $(#mwc_qubo_weights.map(str).join(", "))$, and $k = #mwc_qubo_k$ terminals $T = {#mwc_qubo_terminals.map(str).join(", ")}$.

    *Step 2 -- Introduce binary variables.* Assign $k = #mwc_qubo_k$ indicator variables per vertex: $x_(u,t) = 1$ means vertex $u$ belongs to terminal $t$'s component. This gives $n k = #mwc_qubo_n times #mwc_qubo_k = #mwc_qubo_nq$ QUBO variables:
    $ underbrace(x_(0,0) x_(0,1) x_(0,2), "vertex 0") #h(4pt) underbrace(x_(1,0) x_(1,1) x_(1,2), "vertex 1") #h(4pt) dots.c #h(4pt) underbrace(x_(4,0) x_(4,1) x_(4,2), "vertex 4") $

    *Step 3 -- Penalty coefficient.* $alpha = 1 + sum_(e in E) w(e) = 1 + #mwc_qubo_weights.map(str).join(" + ") = #mwc_qubo_alpha$.

    *Step 4 -- Build $H_A$ (constraints).* One-hot: diagonal entries $Q_(u k+t, u k+t) = -#mwc_qubo_alpha$, off-diagonal $Q_(u k+s, u k+t) = #(2 * mwc_qubo_alpha)$ within each vertex's group. Terminal pinning: for each terminal vertex $t_i$, the wrong-position diagonal entries $Q_(t_i k+s, t_i k+s) += #mwc_qubo_alpha$ for $s != i$, effectively canceling the one-hot incentive for those positions.\

    *Step 5 -- Build $H_B$ (cut cost).* For each edge $(u,v)$ with weight $w$ and each pair $s != t$, add $w$ to $Q_(u k+s, v k+t)$. For example, edge $(0,1)$ with weight $2$ contributes $2$ to positions $(x_(0,0), x_(1,1))$, $(x_(0,0), x_(1,2))$, $(x_(0,1), x_(1,0))$, $(x_(0,1), x_(1,2))$, $(x_(0,2), x_(1,0))$, and $(x_(0,2), x_(1,1))$.\

    *Step 6 -- Verify a solution.* The QUBO ground state $bold(x) = (#mwc_qubo_sol.target_config.map(str).join(", "))$ decodes to the partition: vertex 0 in component 0, vertices 1--3 in component 1, vertex 4 in component 2. Cut edges: $\{#mwc_qubo_cut_indices.map(i => "(" + str(mwc_qubo_edges.at(i).at(0)) + "," + str(mwc_qubo_edges.at(i).at(1)) + ")").join(", ")\}$ with total weight #mwc_qubo_cut_indices.map(i => str(mwc_qubo_weights.at(i))).join(" + ") $= #mwc_qubo_cut_cost$ #sym.checkmark.
  ],
)[
  The multiway cut problem requires a partition of vertices into $k$ components — one per terminal — minimizing the total weight of edges crossing components. The penalty method (@sec:penalty-method) encodes two constraints as QUBO penalties: (1) each vertex belongs to exactly one component (one-hot), and (2) each terminal is pinned to its own component. The cut-cost Hamiltonian counts edge weight across distinct components. Reference: @Heidari2022.
][
  _Construction._ Given $G = (V, E)$ with $n = |V|$, edge weights $w: E -> RR_(>0)$, and $k$ terminals $T = {t_0, ..., t_(k-1)}$. Introduce $n k$ binary variables $x_(u,t) in {0,1}$ (indexed by $u dot k + t$), where $x_(u,t) = 1$ means vertex $u$ is in terminal $t$'s component. Let $alpha = 1 + sum_(e in E) w(e)$.

  The QUBO Hamiltonian is $H = H_A + H_B$ where:
  $ H_A = alpha (sum_(u in V) (1 - sum_(t=0)^(k-1) x_(u,t))^2 + sum_(i=0)^(k-1) sum_(s != i) x_(t_i, s)) $
  The first term is a _one-hot constraint_ ensuring each vertex is assigned to exactly one component. The second term _pins_ each terminal $t_i$ to position $i$ by penalizing any other assignment. Expanding the one-hot term using $x^2 = x$:
  $ Q_(u k+t, u k+t) = -alpha, quad Q_(u k+s, u k+t) = 2 alpha quad (s < t) $
  Terminal pinning adds $alpha$ to the diagonal $Q_(t_i k+s, t_i k+s)$ for $s != i$, canceling the one-hot incentive.

  The cut-cost Hamiltonian:
  $ H_B = sum_((u,v) in E) sum_(s != t) w(u,v) dot x_(u,s) dot x_(v,t) $
  counts the total weight of edges whose endpoints lie in different components.

  _Correctness._ ($arrow.r.double$) A valid multiway cut with cost $C$ maps to a QUBO solution with $H_A = 0$ (valid partition with correct terminal pinning) and $H_B = C$. ($arrow.l.double$) If $H_A > 0$, the penalty $alpha > sum_e w(e)$ exceeds the entire cut-cost range, so any QUBO minimizer has $H_A = 0$, encoding a valid partition. Among valid partitions, $H_B$ equals the cut cost, and the minimizer achieves the minimum multiway cut.

  _Solution extraction._ For each vertex $u$, find terminal position $t$ with $x_(u,t) = 1$. For each edge $(u,v)$, output 1 (cut) if $u$ and $v$ are in different components, 0 otherwise.
]

#let gp_qubo = load-example("GraphPartitioning", "QUBO")
#let gp_qubo_sol = gp_qubo.solutions.at(0)
#let gp_qubo_edges = gp_qubo.source.instance.graph.edges.map(e => (e.at(0), e.at(1)))
#let gp_qubo_n = gp_qubo.source.instance.graph.num_vertices
#let gp_qubo_m = gp_qubo_edges.len()
#let gp_qubo_penalty = gp_qubo_m + 1
#let gp_qubo_diag = range(0, gp_qubo_n).map(i => gp_qubo.target.instance.matrix.at(i).at(i))
#let gp_qubo_cut_edges = gp_qubo_edges.filter(e => gp_qubo_sol.source_config.at(e.at(0)) != gp_qubo_sol.source_config.at(e.at(1)))
#let gp_qubo_cut_size = gp_qubo_cut_edges.len()
#reduction-rule("GraphPartitioning", "QUBO",
  example: true,
  example-caption: [6-vertex balanced partition instance ($n = #gp_qubo_n$, $|E| = #gp_qubo_m$)],
  extra: [
    *Step 1 -- Binary partition variables.* Introduce one binary variable per vertex: $x_i = 0$ means vertex $i$ is in the left block, $x_i = 1$ means it is in the right block. For the canonical instance, this gives $n = #gp_qubo_n$ QUBO variables:
    $ x_0, x_1, x_2, x_3, x_4, x_5 $

    *Step 2 -- Choose the balance penalty.* The source graph has $m = #gp_qubo_m$ edges, so the construction uses $P = m + 1 = #gp_qubo_penalty$. Any imbalance contributes at least $P$, which is already larger than the maximum possible cut size of any balanced partition.

    *Step 3 -- Fill the QUBO matrix.* The diagonal entries are $Q_(i i) = deg(i) + P(1 - n)$, which evaluates here to $(#gp_qubo_diag.map(str).join(", "))$. For every pair $i < j$, start from $Q_(i j) = 2P = #(2 * gp_qubo_penalty)$, then subtract $2$ when $(i,j)$ is an edge. Hence edge coefficients become $18$ while non-edge coefficients stay $20$; the exported upper-triangular matrix matches the issue example exactly.\

    *Step 4 -- Verify a solution.* The exported witness is $bold(x) = (#gp_qubo_sol.target_config.map(str).join(", "))$, which is also the source partition encoding. The cut edges are #gp_qubo_cut_edges.map(e => "(" + str(e.at(0)) + "," + str(e.at(1)) + ")").join(", "), so the cut size is $#gp_qubo_cut_size$ and the balance penalty vanishes because exactly #(gp_qubo_n / 2) vertices are assigned to the right block #sym.checkmark.
  ],
)[
  Graph Partitioning (minimum bisection) asks for a balanced bipartition minimizing the number of crossing edges. Lucas's Ising formulation @lucas2014 translates directly to a QUBO by combining a cut-counting quadratic objective with a quadratic equality penalty enforcing $sum_i x_i = n / 2$. The reduction uses one binary variable per source vertex, so the QUBO has exactly $n$ variables.
][
  _Construction._ Given an undirected graph $G = (V, E)$ with even $n = |V|$ and $m = |E|$, introduce binary variables $x_i in {0,1}$ for each vertex $i in V$. Interpret $x_i = 0$ as $i in A$ and $x_i = 1$ as $i in B$. The cut objective is:
  $ H_"cut" = sum_((u,v) in E) (x_u + x_v - 2 x_u x_v) $
  because the term equals $1$ exactly when edge $(u,v)$ crosses the partition. To enforce balance, add:
  $ H_"bal" = P (sum_i x_i - n/2)^2 $
  with penalty $P = m + 1$. The QUBO objective is $H = H_"cut" + H_"bal"$.

  Expanding $H_"bal"$ with $x_i^2 = x_i$ gives:
  $ H_"bal" = P (1 - n) sum_i x_i + 2 P sum_(i < j) x_i x_j + P n^2 / 4 $
  so the upper-triangular QUBO coefficients are:
  $ Q_(i i) = deg(i) + P (1 - n) $
  and for $i < j$, $Q_(i j) = 2 P$ for every pair, then subtract $2$ whenever $(i,j) in E$. Equivalently, edge pairs have coefficient $2P - 2$ and non-edge pairs have coefficient $2P$. The additive constant $P n^2 / 4$ does not affect the minimizer.

  _Correctness._ ($arrow.r.double$) If $bold(x)$ encodes a balanced partition, then $sum_i x_i = n/2$ and $H_"bal" = 0$, so the QUBO objective equals the cut size exactly. ($arrow.l.double$) If $bold(x)$ is imbalanced, then $|sum_i x_i - n/2| >= 1$, hence $H_"bal" >= P = m + 1$. Every balanced partition has cut size at most $m$, so any imbalanced assignment has objective strictly larger than at least one balanced assignment. Therefore every QUBO minimizer is balanced, and among balanced assignments minimizing $H$ is identical to minimizing the cut size.

  _Solution extraction._ Return the QUBO bit-vector directly: the same binary assignment already records the source partition.
]

#let qubo_ilp = load-example("QUBO", "ILP")
#let qubo_ilp_sol = qubo_ilp.solutions.at(0)
#reduction-rule("QUBO", "ILP",
  example: true,
  example-caption: [4-variable QUBO with 3 quadratic terms],
  extra: [
    Source: $n = #qubo_ilp.source.instance.num_vars$ binary variables, 3 off-diagonal terms \
    Target: #qubo_ilp.target.instance.num_vars ILP variables ($#qubo_ilp.source.instance.num_vars$ original $+ #(qubo_ilp.target.instance.num_vars - qubo_ilp.source.instance.num_vars)$ auxiliary), #qubo_ilp.target.instance.constraints.len() McCormick constraints \
    Canonical optimal witness: $bold(x) = (#qubo_ilp_sol.source_config.map(str).join(", "))$ #sym.checkmark
  ],
)[
  QUBO minimizes a quadratic form $bold(x)^top Q bold(x)$ over binary variables. Every quadratic term $Q_(i j) x_i x_j$ can be _linearized_ by introducing an auxiliary variable $y_(i j)$ constrained to equal the product $x_i x_j$ via three McCormick inequalities. Diagonal terms $Q_(i i) x_i^2 = Q_(i i) x_i$ are already linear for binary $x_i$. The result is a binary ILP with a linear objective and $3 m$ constraints (where $m$ is the number of non-zero off-diagonal entries), whose minimizer corresponds exactly to the QUBO minimizer.
][
  _Construction._ For $Q in RR^(n times n)$ (upper triangular) with $m$ non-zero off-diagonal entries:

  _Diagonal terms._ For binary $x_i$: $Q_(i i) x_i^2 = Q_(i i) x_i$, which is directly linear.

  _Off-diagonal terms._ For each non-zero $Q_(i j)$ ($i < j$), introduce binary $y_(i j) = x_i dot x_j$ with McCormick constraints:
  $ y_(i j) <= x_i, quad y_(i j) <= x_j, quad y_(i j) >= x_i + x_j - 1 $

  _ILP formulation._ Minimize $sum_i Q_(i i) x_i + sum_(i < j) Q_(i j) y_(i j)$ subject to the McCormick constraints and $x_i, y_(i j) in {0, 1}$.

  _Correctness._ ($arrow.r.double$) For binary $x_i, x_j$, the three McCormick inequalities are tight: $y_(i j) = x_i x_j$ is the unique feasible value. Hence the ILP objective equals $bold(x)^top Q bold(x)$, and any ILP minimizer is a QUBO minimizer. ($arrow.l.double$) Given a QUBO minimizer $bold(x)^*$, setting $y_(i j) = x_i^* x_j^*$ satisfies all constraints and achieves the same objective value.

  _Solution extraction._ Return the first $n$ variables (discard auxiliary $y_(i j)$).
]

#let cs_ilp = load-example("CircuitSAT", "ILP")
#reduction-rule("CircuitSAT", "ILP",
  example: true,
  example-caption: [1-bit full adder to ILP],
  extra: [
    Circuit: #circuit-num-gates(cs_ilp.source.instance) gates (2 XOR, 2 AND, 1 OR), #circuit-num-variables(cs_ilp.source.instance) variables \
    Target: #cs_ilp.target.instance.num_vars ILP variables (circuit vars $+$ auxiliary), trivial objective \
    Canonical feasible witness shown ($2^3$ valid input combinations exist for the full adder) #sym.checkmark
  ],
)[
  Each boolean gate (AND, OR, NOT, XOR) has a truth table that can be captured exactly by a small set of linear inequalities over binary variables. By Tseitin-style flattening, each internal expression node gets an auxiliary ILP variable constrained to match its gate's output, so the conjunction of all gate constraints is feasible if and only if the circuit is satisfiable. The ILP has a trivial objective (minimize 0), making it a pure feasibility problem.
][
  _Construction._ Recursively assign an ILP variable to each expression node. Named circuit variables keep their identity; internal nodes get auxiliary variables.

  _Gate encodings_ (output $c$, inputs $a_1, ..., a_k$, all binary):
  - NOT: $c + a = 1$
  - AND: $c <= a_i$ ($forall i$), $c >= sum a_i - (k - 1)$
  - OR: $c >= a_i$ ($forall i$), $c <= sum a_i$
  - XOR (binary, chained pairwise): $c <= a + b$, $c >= a - b$, $c >= b - a$, $c <= 2 - a - b$

  _Objective._ Minimize $0$ (feasibility problem): any feasible solution satisfies the circuit.

  _Correctness._ ($arrow.r.double$) Each gate encoding is the convex hull of the gate's truth table rows (viewed as binary vectors), so a satisfying circuit assignment satisfies all constraints. ($arrow.l.double$) Any binary feasible solution respects every gate's input-output relation, and since gates are composed in topological order, the full circuit evaluates to true.

  _Solution extraction._ Return values of the named circuit variables.
]

== Non-Trivial Reductions

#let sat_mis = load-example("Satisfiability", "MaximumIndependentSet")
#let sat_mis_sol = sat_mis.solutions.at(0)
#reduction-rule("Satisfiability", "MaximumIndependentSet",
  example: true,
  example-caption: [3-SAT with 5 variables and 7 clauses],
  extra: [
    SAT assignment: $(x_1, ..., x_5) = (#sat_mis_sol.source_config.map(str).join(", "))$ \
    IS graph: #graph-num-vertices(sat_mis.target.instance) vertices ($= 3 times #sat-num-clauses(sat_mis.source.instance)$ literals), #graph-num-edges(sat_mis.target.instance) edges \
    IS of size #sat-num-clauses(sat_mis.source.instance) $= m$: one vertex per clause $arrow.r$ satisfying assignment #sym.checkmark
  ],
)[
  @karp1972 A satisfying assignment must make at least one literal true in every clause, and different clauses cannot assign contradictory values to the same variable. These two requirements map naturally to an independent set problem: _intra-clause cliques_ force exactly one literal per clause to be selected, while _conflict edges_ between complementary literals across clauses enforce consistency. The target IS size equals the number of clauses $m$, so an IS of size $m$ exists iff the formula is satisfiable.
][
  _Construction._ For $phi = and.big_(j=1)^m C_j$ with $C_j = (ell_(j,1) or ... or ell_(j,k_j))$:

  _Vertices:_ For each literal $ell_(j,i)$ in clause $C_j$, create $v_(j,i)$. Total: $|V| = sum_j k_j$.

  _Edges:_ (1) Intra-clause cliques: $E_"clause" = {(v_(j,i), v_(j,i')) : i != i'}$. (2) Conflict edges: $E_"conflict" = {(v_(j,i), v_(j',i')) : j != j', ell_(j,i) = overline(ell_(j',i'))}$.

  _Correctness._ ($arrow.r.double$) A satisfying assignment selects one true literal per clause; these vertices form an IS of size $m$ (no clause edges by selection, no conflict edges by consistency). ($arrow.l.double$) An IS of size $m$ must contain exactly one vertex per clause (by clause cliques); the corresponding literals are consistent (by conflict edges) and satisfy $phi$.

  _Solution extraction._ For $v_(j,i) in S$ with literal $x_k$: set $x_k = 1$; for $overline(x_k)$: set $x_k = 0$.
]

#let sat_kc = load-example("Satisfiability", "KColoring")
#let sat_kc_sol = sat_kc.solutions.at(0)
#reduction-rule("Satisfiability", "KColoring",
  example: true,
  example-caption: [5-variable SAT with 3 unit clauses to 3-coloring],
  extra: [
    SAT assignment: $(x_1, ..., x_5) = (#sat_kc_sol.source_config.map(str).join(", "))$ \
    Construction: 3 base + $2 times #sat_kc.source.instance.num_vars$ variable gadgets + OR-gadgets $arrow.r$ #graph-num-vertices(sat_kc.target.instance) vertices, #graph-num-edges(sat_kc.target.instance) edges \
    Canonical 3-coloring witness shown (the construction also has the expected color-symmetry multiplicity for satisfying assignments) #sym.checkmark
  ],
)[
  @garey1979 A 3-coloring partitions vertices into three classes. The key insight is that three colors suffice to encode Boolean logic: one color represents TRUE, one FALSE, and a third (AUX) serves as a neutral ground. Variable gadgets force each variable's positive and negative literals to receive opposite truth colors, while clause gadgets use an OR-chain that can only receive the TRUE color when at least one input literal is TRUE-colored. Connecting the output of each clause gadget to the FALSE vertex forces it to be TRUE-colored, encoding the requirement that every clause is satisfied.
][
  _Construction._ (1) _Base triangle:_ vertices TRUE, FALSE, AUX, all mutually connected. This fixes three distinct colors and establishes the color semantics. (2) _Variable gadget_ for $x_i$: vertices $"pos"_i$, $"neg"_i$ connected to each other and to AUX. Since $"pos"_i$ and $"neg"_i$ are both adjacent to AUX, neither can receive the AUX color; since they are adjacent to each other, one must be TRUE-colored and the other FALSE-colored. (3) _Clause gadget_ for $(ell_1 or dots or ell_k)$: apply OR-gadgets iteratively --- $o_1 = "OR"(ell_1, ell_2)$, $o_2 = "OR"(o_1, ell_3)$, etc. --- producing final output $o$, then connect $o$ to both FALSE and AUX.

  _OR-gadget$(a, b) arrow.bar o$:_ Introduces five auxiliary vertices with edges arranged so that $o$ can receive the TRUE color iff at least one of $a$, $b$ has the TRUE color. When both inputs have the FALSE color, the gadget's internal constraints force $o$ into the AUX color.

  _Correctness._ ($arrow.r.double$) A satisfying assignment colors $"pos"_i$ as TRUE when $x_i = 1$ and FALSE otherwise. Each clause has at least one TRUE literal, so the OR-chain output receives the TRUE color, which is compatible with edges to FALSE and AUX. ($arrow.l.double$) In any valid 3-coloring, the variable gadgets assign consistent truth values and the clause gadget connections to FALSE force each clause output to be TRUE-colored, meaning at least one literal per clause is TRUE.

  _Solution extraction._ Set $x_i = 1$ iff $"color"("pos"_i) = "color"("TRUE")$.
]

#let sat_ds = load-example("Satisfiability", "MinimumDominatingSet")
#let sat_ds_sol = sat_ds.solutions.at(0)
#reduction-rule("Satisfiability", "MinimumDominatingSet",
  example: true,
  example-caption: [5-variable 7-clause 3-SAT to dominating set],
  extra: [
    SAT assignment: $(x_1, ..., x_5) = (#sat_ds_sol.source_config.map(str).join(", "))$ \
    Vertex structure: $#graph-num-vertices(sat_ds.target.instance) = 3 times #sat_ds.source.instance.num_vars + #sat-num-clauses(sat_ds.source.instance)$ (variable triangles + clause vertices) \
    Dominating set of size $n = #sat_ds.source.instance.num_vars$: one vertex per variable triangle #sym.checkmark
  ],
)[
  @garey1979 Each variable is represented by a triangle whose three vertices correspond to the positive literal, negative literal, and a dummy. Any dominating set must include at least one vertex from each triangle to dominate the dummy. The clause vertices are connected only to the literal vertices that appear in the clause, so a dominating set of minimum size $n$ (one vertex per triangle) dominates all clause vertices iff the chosen literals satisfy every clause.
][
  _Construction._ (1) _Variable triangle_ for $x_i$: vertices $"pos"_i = 3i$, $"neg"_i = 3i+1$, $"dum"_i = 3i+2$ forming a triangle. The dummy vertex $"dum"_i$ is adjacent only to $"pos"_i$ and $"neg"_i$, so it can only be dominated by a vertex from its own triangle. (2) _Clause vertex_ $c_j = 3n+j$ connected to $"pos"_i$ if $x_i in C_j$, to $"neg"_i$ if $overline(x_i) in C_j$.

  _Correctness._ ($arrow.r.double$) Given a satisfying assignment, select $"pos"_i$ if $x_i = 1$, else $"neg"_i$. This dominates all triangle vertices (each triangle has one selected vertex adjacent to both others). Each clause $C_j$ has at least one true literal, so $c_j$ is adjacent to at least one selected vertex. Total size: $n$. ($arrow.l.double$) Any dominating set needs $>= 1$ vertex per triangle (to dominate $"dum"_i$). A set of size $n$ has exactly one per triangle. If $"dum"_i$ is selected, it does not dominate any clause vertex; but it does dominate $"pos"_i$ and $"neg"_i$, which still need to cover clauses. Since $"dum"_i$ has no clause neighbors, we can swap it for $"pos"_i$ or $"neg"_i$ without losing domination of the triangle. After swapping, each clause vertex $c_j$ must be dominated by some $"pos"_i$ or $"neg"_i$, defining a consistent satisfying assignment.

  _Solution extraction._ Set $x_i = 1$ if $"pos"_i$ selected; $x_i = 0$ if $"neg"_i$ selected.
]

#reduction-rule("KSatisfiability", "Satisfiability")[
  Every $k$-SAT instance is already a SAT instance --- clauses happen to have exactly $k$ literals, but SAT places no restriction on clause width. The embedding is the identity.
][
  _Construction._ Variables and clauses are unchanged.

  _Correctness._ ($arrow.r.double$) Any $k$-SAT satisfying assignment satisfies the same clauses under SAT. ($arrow.l.double$) Any SAT satisfying assignment satisfies the same clauses (which all have width $k$). _Solution extraction._ Identity.
]

#let sat_ksat = load-example("Satisfiability", "KSatisfiability")
#let sat_ksat_sol = sat_ksat.solutions.at(0)
#reduction-rule("Satisfiability", "KSatisfiability",
  example: true,
  example-caption: [Mixed-size clauses (sizes 1 to 5) to 3-SAT],
  extra: [
    Source: #sat_ksat.source.instance.num_vars variables, #sat-num-clauses(sat_ksat.source.instance) clauses (sizes 1, 2, 3, 3, 4, 5) \
    Target 3-SAT: $#sat_ksat.target.instance.num_vars = #sat_ksat.source.instance.num_vars + 7$ variables, #sat-num-clauses(sat_ksat.target.instance) clauses (small padded, large split) \
    First solution: $(x_1, ..., x_5) = (#sat_ksat_sol.source_config.map(str).join(", "))$, auxiliary vars are don't-cares #sym.checkmark
  ],
)[
  @cook1971 @garey1979 Clauses shorter than $k$ can be padded with a complementary pair $y, overline(y)$ that is always satisfiable; clauses longer than $k$ can be split into a chain of width-$k$ clauses linked by auxiliary variables that propagate truth values. Both transformations preserve satisfiability while enforcing uniform clause width.
][
  _Construction._

  _Small clauses ($|C| < k$):_ Pad $(ell_1 or dots or ell_r)$ with fresh auxiliary $y$: $(ell_1 or dots or ell_r or y or overline(y) or dots)$ to length $k$. The pair $y, overline(y)$ is a tautology, so the padded clause is satisfiable iff the original is.

  _Large clauses ($|C| > k$):_ Split $(ell_1 or dots or ell_r)$ with auxiliaries $y_1, dots, y_(r-k)$:
  $ (ell_1 or dots or ell_(k-1) or y_1) and (overline(y_1) or ell_k or dots or y_2) and dots and (overline(y_(r-k)) or ell_(r-k+2) or dots or ell_r) $

  _Correctness._ ($arrow.r.double$) If the original clause is satisfied by some literal $ell_j$, set the auxiliary chain so that $y_i = 1$ for all $i$ before $ell_j$'s sub-clause and $y_i = 0$ after. Each sub-clause then contains either a true original literal or a true auxiliary. ($arrow.l.double$) If all sub-clauses are satisfied but every original literal is false, the first clause forces $y_1 = 1$, which forces $y_2 = 1$ (since $overline(y_1)$ is false), and so on until the last clause has $overline(y_(r-k)) = 0$ and all remaining literals false --- a contradiction.

  _Solution extraction._ Discard auxiliary variables; return original variable assignments.
]

#reduction-rule("Satisfiability", "CircuitSAT",
  example: true,
  example-caption: [3-variable SAT formula to boolean circuit],
)[
  CNF is inherently an AND-of-ORs structure, which maps directly to a boolean circuit: each clause becomes an OR gate over its literals, and a final AND gate combines all clause outputs. The circuit is constrained to output _true_, so a satisfying circuit assignment exists iff the original formula is satisfiable.
][
  _Construction._ For $phi = C_1 and dots and C_k$ with $C_i = (ell_(i 1) or dots or ell_(i m_i))$: (1) Create circuit inputs $x_1, dots, x_n$ corresponding to SAT variables. (2) For each clause $C_i$, add an OR gate $g_i$ with inputs from the clause's literals (negated inputs use NOT gates). (3) Add a final AND gate with inputs $g_1, dots, g_k$, constrained to output _true_.

  _Correctness._ ($arrow.r.double$) A satisfying assignment makes at least one literal true in each clause, so each OR gate outputs true and the AND gate outputs true. ($arrow.l.double$) A satisfying circuit assignment has all OR gates true (forced by the AND output constraint), meaning at least one literal per clause is true --- exactly a SAT solution.

  _Solution extraction._ Return the values of the circuit input variables $x_1, dots, x_n$.
]

#let cs_sg = load-example("CircuitSAT", "SpinGlass")
#reduction-rule("CircuitSAT", "SpinGlass",
  example: true,
  example-caption: [1-bit full adder to Ising model],
  extra: [
    Circuit: #circuit-num-gates(cs_sg.source.instance) gates (2 XOR, 2 AND, 1 OR), #circuit-num-variables(cs_sg.source.instance) variables \
    Target: #spin-num-spins(cs_sg.target.instance) spins (each gate allocates I/O + auxiliary spins) \
    Canonical ground-state witness shown ($2^3$ valid input combinations exist for the full adder) #sym.checkmark
  ],
)[
  @whitfield2012 @lucas2014 Each logic gate can be represented as an Ising gadget --- a small set of spins with couplings $J_(i j)$ and fields $h_i$ chosen so that the gadget's ground states correspond exactly to the gate's truth table rows. Composing gadgets for all gates in the circuit yields a spin glass whose ground states encode precisely the satisfying assignments of the circuit. The energy gap between valid and invalid I/O patterns ensures that any global ground state respects every gate's logic.
][
  _Construction._

  _Spin mapping:_ Boolean variables $sigma in {0,1}$ map to Ising spins $s = 2sigma - 1 in {-1, +1}$. Each circuit variable is assigned a unique spin index; gate gadgets reference these indices for their inputs and outputs.

  _Gate gadgets_ (inputs 0,1; output 2; auxiliary 3 for XOR) are listed in @tab:gadgets. For each gate, instantiate the gadget's couplings and fields. The total Hamiltonian is the sum over all gadgets: $H = -sum_(i < j) J_(i j) s_i s_j - sum_i h_i s_i$.

  _Correctness._ ($arrow.r.double$) A satisfying circuit assignment maps to a spin configuration where every gadget is in a ground state (valid I/O), so the total energy is minimized. ($arrow.l.double$) Any global ground state must minimize each gadget's contribution. Since each gadget's ground states match its gate's truth table, the spin configuration encodes a valid circuit evaluation. The output spin is constrained to $+1$ (true), so the circuit is satisfied.

  _Solution extraction._ Map spins back to Boolean: $sigma_i = (s_i + 1) / 2$. Return the circuit input variables.
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

#let fact_cs = load-example("Factoring", "CircuitSAT")
#let fact-decode(config, start, count) = {
  let pow2 = (1, 2, 4, 8, 16, 32)
  range(count).fold(0, (acc, i) => acc + config.at(start + i) * pow2.at(i))
}
#let fact_cs_sol = fact_cs.solutions.at(0)
#let fact-nbf = fact_cs.source.instance.m
#let fact-nbs = fact_cs.source.instance.n
#let fact-p = fact-decode(fact_cs_sol.source_config, 0, fact-nbf)
#let fact-q = fact-decode(fact_cs_sol.source_config, fact-nbf, fact-nbs)
#reduction-rule("Factoring", "CircuitSAT",
  example: true,
  example-caption: [Factor $N = #fact_cs.source.instance.target$],
  extra: [
    Circuit: $#fact-nbf times #fact-nbs$ array multiplier with #circuit-num-gates(fact_cs.target.instance) gates, #circuit-num-variables(fact_cs.target.instance) variables \
    Canonical witness: $#fact-p times #fact-q = #fact_cs.source.instance.target$ #sym.checkmark
  ],
)[
  Integer multiplication can be implemented as a boolean circuit: an $m times n$ array multiplier computes $p times q$ using only AND, XOR, and OR gates arranged in a grid of full adders. Constraining the output bits to match $N$ turns the circuit into a satisfiability problem --- the circuit is satisfiable iff $N = p times q$ for some $p, q$ within the given bit widths. _(Folklore; no canonical reference.)_
][
  _Construction._ Build $m times n$ array multiplier for $p times q$:

  _Full adder $(i,j)$:_ Each cell computes one partial product bit $p_i and q_j$ and adds it to the running sum from previous cells. The sum and carry are: $s_(i,j) + 2c_(i,j) = (p_i and q_j) + s_"prev" + c_"prev"$, implemented via:
  $ a := p_i and q_j, quad t_1 := a xor s_"prev", quad s_(i,j) := t_1 xor c_"prev" $
  $ t_2 := t_1 and c_"prev", quad t_3 := a and s_"prev", quad c_(i,j) := t_2 or t_3 $

  _Output constraint:_ Fix output wires to the binary representation of $N$: $M_k := "bit"_k(N)$ for $k = 1, dots, m+n$.

  _Correctness._ ($arrow.r.double$) If $N = p times q$ with $p < 2^m$ and $q < 2^n$, setting the input bits to the binary representations of $p$ and $q$ produces output bits matching $N$, satisfying all constraints. ($arrow.l.double$) Any satisfying assignment to the circuit computes a valid multiplication (the gates enforce arithmetic correctness), and the output constraint ensures the product equals $N$.

  _Solution extraction._ Read off factor bits: $p = sum_i p_i 2^(i-1)$, $q = sum_j q_j 2^(j-1)$.
]

#let mc_sg = load-example("MaxCut", "SpinGlass")
#let mc_sg_sol = mc_sg.solutions.at(0)
#let mc_sg_cut = mc_sg.source.instance.graph.edges.filter(e => mc_sg_sol.source_config.at(e.at(0)) != mc_sg_sol.source_config.at(e.at(1))).len()
#reduction-rule("MaxCut", "SpinGlass",
  example: true,
  example-caption: [Petersen graph ($n = 10$, unit weights) to Ising],
  extra: [
    Direct 1:1 mapping: vertices $arrow.r$ spins, $J_(i j) = w_(i j) = 1$, $h_i = 0$ \
    Partition: $S = {#mc_sg_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => str(i)).join(", ")}$ vs $overline(S) = {#mc_sg_sol.source_config.enumerate().filter(((i, x)) => x == 0).map(((i, x)) => str(i)).join(", ")}$ \
    Cut value $= #mc_sg_cut$ (canonical witness shown) #sym.checkmark
  ],
)[
  @barahona1982 A maximum cut partitions vertices into two groups to maximize the total weight of edges crossing the partition. In the Ising model, two spins with opposite signs contribute $-J_(i j) s_i s_j = J_(i j)$ to the energy, while same-sign spins contribute $-J_(i j)$. Setting $J_(i j) = w_(i j)$ and $h_i = 0$ makes each cut edge lower the energy by $2 J_(i j)$ relative to an uncut edge, so the Ising ground state corresponds to the maximum cut.
][
  _Construction._ Map each vertex to a spin with $J_(i j) = w_(i j)$ for each edge and $h_i = 0$ (no external field). Spins are $s_i = 2 sigma_i - 1$ where $sigma_i in {0, 1}$ is the partition label.

  _Correctness._ ($arrow.r.double$) A maximum cut assigns $sigma_i in {0,1}$. For cut edges, $s_i s_j = -1$, contributing $-J_(i j)(-1) = +J_(i j)$. For uncut edges, $s_i s_j = +1$, contributing $-J_(i j)$. Maximizing cut weight is equivalent to minimizing $-sum J_(i j) s_i s_j$, the Ising energy. ($arrow.l.double$) An Ising ground state minimizes $-sum J_(i j) s_i s_j$, which is maximized when opposite-sign pairs (cut edges) have the largest possible weights --- exactly the maximum cut.

  _Solution extraction._ Partition $= {i : s_i = +1}$.
]

#let sg_mc = load-example("SpinGlass", "MaxCut")
#let sg_mc_sol = sg_mc.solutions.at(0)
#reduction-rule("SpinGlass", "MaxCut",
  example: true,
  example-caption: [10-spin Ising with alternating $J_(i j) in {plus.minus 1}$],
  extra: [
    All $h_i = 0$: no ancilla needed, direct 1:1 vertex mapping \
    Edge weights $w_(i j) = J_(i j) in {plus.minus 1}$ (alternating couplings) \
    Canonical ground-state witness: partition $S = {#sg_mc_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => str(i)).join(", ")}$ #sym.checkmark
  ],
)[
  @barahona1982 @lucas2014 The Ising Hamiltonian $H = -sum J_(i j) s_i s_j - sum h_i s_i$ has two types of terms. The pairwise couplings $J_(i j)$ map directly to MaxCut edge weights, since minimizing $-J_(i j) s_i s_j$ favors opposite spins (cut edges) when $J_(i j) > 0$. The local fields $h_i$ have no direct MaxCut analogue, but can be absorbed by introducing a single ancilla vertex connected to every spin with weight $h_i$: fixing the ancilla's partition side effectively creates a linear bias on each spin.
][
  _Construction._ If all $h_i = 0$: set $w_(i j) = J_(i j)$ directly (1:1 mapping, no ancilla). If some $h_i != 0$: add ancilla vertex $a$ with edges $w_(i,a) = h_i$ for each spin $i$. The Ising energy $-sum J_(i j) s_i s_j - sum h_i s_i$ equals $-sum J_(i j) s_i s_j - sum h_i s_i s_a$ when $s_a = +1$, which is a pure pairwise Hamiltonian on $n + 1$ spins.

  _Correctness._ ($arrow.r.double$) An Ising ground state assigns spins to minimize $H$. The equivalent MaxCut graph has the same objective (up to a constant), so the spin configuration defines a maximum cut. ($arrow.l.double$) A maximum cut on the constructed graph maximizes $sum_("cut") w_(i j)$, which corresponds to minimizing $-sum J_(i j) s_i s_j - sum h_i s_i s_a$. With $s_a$ fixed, this is the Ising energy, so the cut defines a ground state.

  _Solution extraction._ Without ancilla: partition labels are the spin values directly. With ancilla: if $sigma_a = 1$ (ancilla on the $+1$ side), the spin values are read directly; if $sigma_a = 0$, flip all spins before reading (since the ancilla should represent $s_a = +1$).
]

#reduction-rule("KColoring", "ILP")[
  A $k$-coloring assigns each vertex exactly one of $k$ colors such that adjacent vertices differ. Both requirements are naturally linear: the "exactly one color" condition is an equality constraint on $k$ binary indicators per vertex, and the "neighbors differ" condition bounds each color's indicator sum to at most one per edge. The resulting ILP has $|V| k$ variables and $|V| + |E| k$ constraints with a trivial objective.
][
  _Construction._ For graph $G = (V, E)$ with $k$ colors:

  _Variables:_ Binary $x_(v,c) in {0, 1}$ for each vertex $v in V$ and color $c in {1, ..., k}$. Interpretation: $x_(v,c) = 1$ iff vertex $v$ has color $c$.

  _Constraints:_ (1) Each vertex has exactly one color: $sum_(c=1)^k x_(v,c) = 1$ for all $v in V$. (2) Adjacent vertices have different colors: $x_(u,c) + x_(v,c) <= 1$ for all $(u, v) in E$ and $c in {1, ..., k}$.

  _Objective:_ Feasibility problem (minimize 0).

  _Correctness._ ($arrow.r.double$) A valid $k$-coloring assigns exactly one color per vertex with different colors on adjacent vertices; setting $x_(v,c) = 1$ for the assigned color satisfies all constraints. ($arrow.l.double$) Any feasible ILP solution has exactly one $x_(v,c) = 1$ per vertex; this defines a coloring, and constraint (2) ensures adjacent vertices differ.

  _Solution extraction._ For each vertex $v$, find $c$ with $x_(v,c) = 1$; assign color $c$ to $v$.
]

#reduction-rule("Factoring", "ILP")[
  Integer multiplication $p times q = N$ is a system of bilinear equations over binary factor bits with carry propagation. Each bit-product $p_i q_j$ is a bilinear term that McCormick linearization replaces with an auxiliary variable and three inequalities. The carry-chain equations are already linear, so the full system becomes a binary ILP with $O(m n)$ variables and constraints.
][
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

== ILP Formulations

The following reductions to Integer Linear Programming are straightforward formulations where problem constraints map directly to linear inequalities.

#reduction-rule("MaximumSetPacking", "ILP")[
  Each set is either selected or not, and every universe element may belong to at most one selected set -- an element-based constraint that is directly linear in binary indicator variables.
][
  _Construction._ Variables: $x_i in {0, 1}$ for each set $S_i in cal(S)$. Constraints: $sum_(S_i in.rev e) x_i <= 1$ for each element $e in U$. Objective: maximize $sum_i w_i x_i$.

  _Correctness._ ($arrow.r.double$) A valid packing chooses pairwise disjoint sets, so each element is covered at most once. ($arrow.l.double$) Any feasible binary solution covers each element at most once, hence the chosen sets are pairwise disjoint; the objective maximizes total weight.

  _Solution extraction._ $cal(P) = {S_i : x_i = 1}$.
]

#reduction-rule("MaximumMatching", "ILP")[
  Each edge is either selected or not, and each vertex may be incident to at most one selected edge -- a degree-bound constraint that is directly linear in binary edge indicators.
][
  _Construction._ Variables: $x_e in {0, 1}$ for each $e in E$. Constraints: $sum_(e in.rev v) x_e <= 1$ for each $v in V$. Objective: maximize $sum_e w_e x_e$.

  _Correctness._ ($arrow.r.double$) A matching has at most one edge per vertex, so all degree constraints hold. ($arrow.l.double$) Any feasible solution is a matching by construction; the objective maximizes total weight.

  _Solution extraction._ $M = {e : x_e = 1}$.
]

#reduction-rule("MinimumSetCovering", "ILP")[
  Every universe element must be covered by at least one selected set -- a lower-bound constraint on the sum of indicators for sets containing that element, which is directly linear.
][
  _Construction._ Variables: $x_i in {0, 1}$ for each $S_i in cal(S)$. Constraints: $sum_(S_i in.rev u) x_i >= 1$ for each $u in U$. Objective: minimize $sum_i w_i x_i$.

  _Correctness._ ($arrow.r.double$) A set cover includes at least one set containing each element, satisfying all constraints. ($arrow.l.double$) Any feasible solution covers every element; the objective minimizes total weight.

  _Solution extraction._ $cal(C) = {S_i : x_i = 1}$.
]

#reduction-rule("MinimumDominatingSet", "ILP")[
  Every vertex must be dominated -- either selected itself or adjacent to a selected vertex -- which is a lower-bound constraint on the sum of indicators over its closed neighborhood.
][
  _Construction._ Variables: $x_v in {0, 1}$ for each $v in V$. Constraints: $x_v + sum_(u in N(v)) x_u >= 1$ for each $v in V$ (each vertex dominated). Objective: minimize $sum_v w_v x_v$.

  _Correctness._ ($arrow.r.double$) A dominating set includes a vertex or one of its neighbors for every vertex, satisfying all constraints. ($arrow.l.double$) Any feasible solution dominates every vertex; the objective minimizes total weight.

  _Solution extraction._ $D = {v : x_v = 1}$.
]

#reduction-rule("MinimumFeedbackVertexSet", "ILP")[
  A directed graph is a DAG iff it admits a topological ordering. MTZ-style ordering variables enforce this: for each kept vertex, an integer position variable must increase strictly along every arc. Removed vertices relax the ordering constraints via big-$M$ terms.
][
  _Construction._ Given directed graph $G = (V, A)$ with $n = |V|$, $m = |A|$, and weights $w_v$:

  _Variables:_ Binary $x_v in {0, 1}$ for each $v in V$: $x_v = 1$ iff $v$ is removed. Integer $o_v in {0, dots, n-1}$ for each $v in V$: topological order position. Total: $2n$ variables.

  _Constraints:_ (1) For each arc $(u -> v) in A$: $o_v - o_u >= 1 - n(x_u + x_v)$. When both endpoints are kept ($x_u = x_v = 0$), this forces $o_v > o_u$ (strict topological order). When either is removed, the constraint relaxes to $o_v - o_u >= 1 - n$ (trivially satisfied). (2) Binary bounds: $x_v <= 1$. (3) Order bounds: $o_v <= n - 1$. Total: $m + 2n$ constraints.

  _Objective:_ Minimize $sum_v w_v x_v$.

  _Correctness._ ($arrow.r.double$) If $S$ is a feedback vertex set, then $G[V backslash S]$ is a DAG with a topological ordering. Set $x_v = 1$ for $v in S$, $o_v$ to the topological position for kept vertices, and $o_v = 0$ for removed vertices. All constraints are satisfied. ($arrow.l.double$) If the ILP is feasible with all arc constraints satisfied, no directed cycle can exist among kept vertices: a cycle $v_1 -> dots -> v_k -> v_1$ would require $o_(v_1) < o_(v_2) < dots < o_(v_k) < o_(v_1)$, a contradiction.

  _Solution extraction._ $S = {v : x_v = 1}$.
]

#reduction-rule("MaximumClique", "ILP")[
  A clique requires every pair of selected vertices to be adjacent; equivalently, no two selected vertices may share a _non_-edge. This is the independent set formulation on the complement graph $overline(G)$.
][
  _Construction._ Variables: $x_v in {0, 1}$ for each $v in V$. Constraints: $x_u + x_v <= 1$ for each $(u, v) in.not E$ (non-edges). Objective: maximize $sum_v x_v$.

  _Correctness._ ($arrow.r.double$) In a clique, every pair of selected vertices is adjacent, so no non-edge constraint is violated. ($arrow.l.double$) Any feasible solution selects only mutually adjacent vertices, forming a clique; the objective maximizes its size.

  _Solution extraction._ $K = {v : x_v = 1}$.
]

#let gp_ilp = load-example("GraphPartitioning", "ILP")
#let gp_ilp_sol = gp_ilp.solutions.at(0)
#let gp_n = graph-num-vertices(gp_ilp.source.instance)
#let gp_edges = gp_ilp.source.instance.graph.edges
#let gp_m = gp_edges.len()
#let gp_part_a = range(gp_n).filter(i => gp_ilp_sol.source_config.at(i) == 0)
#let gp_part_b = range(gp_n).filter(i => gp_ilp_sol.source_config.at(i) == 1)
#let gp_crossing = range(gp_m).filter(i => gp_ilp_sol.target_config.at(gp_n + i) == 1)
#let gp_crossing_edges = gp_crossing.map(i => gp_edges.at(i))
#reduction-rule("GraphPartitioning", "ILP",
  example: true,
  example-caption: [Two triangles linked by three crossing edges encoded as a 15-variable ILP.],
  extra: [
    *Step 1 -- Balanced partition variables.* Introduce $x_v in {0,1}$ for each vertex. In the canonical witness, $A = {#gp_part_a.map(str).join(", ")}$ and $B = {#gp_part_b.map(str).join(", ")}$, so $bold(x) = (#gp_ilp_sol.source_config.map(str).join(", "))$.\

    *Step 2 -- Crossing indicators.* Add one binary variable per edge, so the target has $#gp_ilp.target.instance.num_vars$ binary variables and #gp_ilp.target.instance.constraints.len() constraints in total. The three active crossing indicators correspond to edges $\{#gp_crossing_edges.map(e => "(" + str(e.at(0)) + "," + str(e.at(1)) + ")").join(", ")\}$.\

    *Step 3 -- Verify the objective.* The target witness $bold(z) = (#gp_ilp_sol.target_config.map(str).join(", "))$ sets exactly #gp_crossing.len() edge-indicator variables to 1, so the ILP objective equals the bisection width #gp_crossing.len() #sym.checkmark
  ],
)[
  The node-and-edge integer-programming formulation of Chopra and Rao @chopra1993 models a balanced cut with one binary variable per vertex and one binary crossing indicator per edge. A single balance equality enforces the bisection, and two linear inequalities per edge linearize $|x_u - x_v|$ so that the objective can minimize the number of crossing edges directly.
][
  _Construction._ Given graph $G = (V, E)$ with $n = |V|$ and $m = |E|$:

  _Variables._ Binary $x_v in {0, 1}$ for each $v in V$, where $x_v = 1$ means vertex $v$ is placed in side $B$. For each edge $e = (u, v) in E$, binary $y_e in {0, 1}$ indicates whether $e$ crosses the partition. Total: $n + m$ variables.

  _Constraints._ (1) Balance: $sum_(v in V) x_v = n / 2$. If $n$ is odd, the right-hand side is fractional, so the ILP is infeasible exactly when Graph Partitioning has no valid balanced partition. (2) For each edge $e = (u, v)$: $y_e >= x_u - x_v$ and $y_e >= x_v - x_u$. Since $y_e$ is binary and the objective minimizes $sum_e y_e$, these inequalities force $y_e = 1$ exactly for crossing edges. Total: $2m + 1$ constraints.

  _Objective._ Minimize $sum_(e in E) y_e$.

  _Correctness._ ($arrow.r.double$) Given a balanced partition $(A, B)$, set $x_v = 1$ iff $v in B$, and set $y_e = 1$ iff edge $e$ has one endpoint in each side. The balance constraint holds because $|B| = n / 2$, and the linking inequalities hold because $|x_u - x_v| = 1$ exactly on crossing edges. The objective is therefore the cut size. ($arrow.l.double$) Any feasible ILP solution satisfies the balance equation, so exactly half the vertices have $x_v = 1$ when $n$ is even. For each edge, the linking inequalities imply $y_e >= |x_u - x_v|$; minimization therefore chooses $y_e = |x_u - x_v|$, making the objective count precisely the crossing edges of the extracted partition.

  _Solution extraction._ Return the first $n$ variables $(x_v)_(v in V)$ as the Graph Partitioning configuration; the edge-indicator variables are auxiliary.
]

#let ks_ilp = load-example("Knapsack", "ILP")
#let ks_ilp_sol = ks_ilp.solutions.at(0)
#let ks_ilp_selected = ks_ilp_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#let ks_ilp_sel_weight = ks_ilp_selected.fold(0, (a, i) => a + ks_ilp.source.instance.weights.at(i))
#let ks_ilp_sel_value = ks_ilp_selected.fold(0, (a, i) => a + ks_ilp.source.instance.values.at(i))
#reduction-rule("Knapsack", "ILP",
  example: true,
  example-caption: [$n = #ks_ilp.source.instance.weights.len()$ items, capacity $C = #ks_ilp.source.instance.capacity$],
  extra: [
    *Step 1 -- Source instance.* The canonical knapsack instance has weights $(#ks_ilp.source.instance.weights.map(str).join(", "))$, values $(#ks_ilp.source.instance.values.map(str).join(", "))$, and capacity $C = #ks_ilp.source.instance.capacity$.

    *Step 2 -- Build the binary ILP.* Introduce one binary variable per item:
    $#range(ks_ilp.source.instance.weights.len()).map(i => $x_#i$).join(", ") in {0,1}$.
    The objective is
    $ max #ks_ilp.source.instance.values.enumerate().map(((i, v)) => $#v x_#i$).join($+$) $
    subject to the single capacity inequality
    $ #ks_ilp.source.instance.weights.enumerate().map(((i, w)) => $#w x_#i$).join($+$) <= #ks_ilp.source.instance.capacity $.

    *Step 3 -- Verify a solution.* The ILP optimum $bold(x)^* = (#ks_ilp_sol.target_config.map(str).join(", "))$ extracts directly to the knapsack selection $bold(x)^* = (#ks_ilp_sol.source_config.map(str).join(", "))$, choosing items $\{#ks_ilp_selected.map(str).join(", ")\}$. Their total weight is $#ks_ilp_selected.map(i => str(ks_ilp.source.instance.weights.at(i))).join(" + ") = #ks_ilp_sel_weight$ and their total value is $#ks_ilp_selected.map(i => str(ks_ilp.source.instance.values.at(i))).join(" + ") = #ks_ilp_sel_value$ #sym.checkmark.

    *Uniqueness:* The fixture stores one canonical optimal witness. For this instance the optimum is unique: items $\{#ks_ilp_selected.map(str).join(", ")\}$ are the only feasible choice achieving value #ks_ilp_sel_value.
  ],
)[
  A 0-1 Knapsack instance is already a binary Integer Linear Program @papadimitriou-steiglitz1982: each item-selection bit becomes a binary variable, the capacity condition is a single linear inequality, and the value objective is linear. The reduction preserves the number of decision variables exactly, producing an ILP with $n$ variables and one constraint.
][
  _Construction._ Given nonnegative weights $w_0, dots, w_(n-1)$, nonnegative values $v_0, dots, v_(n-1)$, and capacity $C$, introduce binary variables $x_0, dots, x_(n-1) in {0,1}$ where $x_i = 1$ iff item $i$ is selected. Construct the binary ILP:
  $ max sum_(i=0)^(n-1) v_i x_i $
  subject to
  $ sum_(i=0)^(n-1) w_i x_i <= C $
  and $x_i in {0,1}$ for all $i$. The target therefore has exactly $n$ variables and one linear constraint.

  _Correctness._ ($arrow.r.double$) Any feasible knapsack solution $bold(x)$ satisfies $sum_i w_i x_i <= C$, so the same binary vector is feasible for the ILP and attains identical objective value $sum_i v_i x_i$. ($arrow.l.double$) Any feasible binary ILP solution selects exactly the items with $x_i = 1$; the single inequality guarantees the chosen set fits in the knapsack, and the ILP objective equals the knapsack value. Therefore optimal solutions correspond one-to-one and preserve the optimum value.

  _Solution extraction._ Identity: return the binary variable vector $bold(x)$ as the knapsack selection.
]

#reduction-rule("MaximumClique", "MaximumIndependentSet",
  example: true,
  example-caption: [Path graph $P_4$: clique in $G$ maps to independent set in complement $overline(G)$.],
)[
  A clique in $G$ is an independent set in the complement graph $overline(G)$, where $overline(G) = (V, overline(E))$ with $overline(E) = {(u,v) : u != v, (u,v) in.not E}$. This classical reduction @karp1972 preserves vertices and weights; only the edge set changes.
][
  _Construction._ Given MaximumClique instance $(G = (V, E), bold(w))$ with $n = |V|$ and $m = |E|$, create MaximumIndependentSet instance $(overline(G) = (V, overline(E)), bold(w))$ where $overline(E) = {(u,v) : u != v, (u,v) in.not E}$. The complement graph has $n(n-1)/2 - m$ edges. Weights are preserved identically.

  _Correctness._ ($arrow.r.double$) If $S$ is a clique in $G$, then all pairs in $S$ are adjacent in $G$, so no pair in $S$ is adjacent in $overline(G)$, making $S$ an independent set in $overline(G)$. ($arrow.l.double$) If $S$ is an independent set in $overline(G)$, then no pair in $S$ is adjacent in $overline(G)$, so all pairs in $S$ are adjacent in $G$, making $S$ a clique. Since both problems maximize $sum_(v in S) w_v$, optimal values coincide.

  _Solution extraction._ Identity: the configuration is the same in both problems, since vertices are preserved one-to-one.
]

#reduction-rule("BinPacking", "ILP")[
  The assignment-based formulation introduces a binary indicator for each item--bin pair and a binary variable for each bin being open. Assignment constraints ensure each item is placed in exactly one bin; capacity constraints link bin usage to item weights.
][
  _Construction._ Given $n$ items with sizes $s_1, dots, s_n$ and bin capacity $C$:

  _Variables:_ $x_(i j) in {0, 1}$ for $i, j in {0, dots, n-1}$: item $i$ is assigned to bin $j$. $y_j in {0, 1}$: bin $j$ is used. Total: $n^2 + n$ variables.

  _Constraints:_ (1) Assignment: $sum_(j=0)^(n-1) x_(i j) = 1$ for each item $i$ (each item in exactly one bin). (2) Capacity + linking: $sum_(i=0)^(n-1) s_i dot x_(i j) lt.eq C dot y_j$ for each bin $j$ (bin capacity respected; $y_j$ forced to 1 if bin $j$ is used).

  _Objective:_ Minimize $sum_(j=0)^(n-1) y_j$.

  _Correctness._ ($arrow.r.double$) A valid packing assigns each item to exactly one bin (satisfying (1)); each bin's load is at most $C$ and $y_j = 1$ for any used bin (satisfying (2)). ($arrow.l.double$) Any feasible solution assigns each item to one bin by (1), respects capacity by (2), and the objective counts the number of open bins.

  _Solution extraction._ For each item $i$, find the unique $j$ with $x_(i j) = 1$; assign item $i$ to bin $j$.
]

#reduction-rule("SequencingToMinimizeWeightedCompletionTime", "ILP")[
  Completion times are natural integer variables, precedence constraints compare those completion times directly, and one binary order variable per task pair enforces that a single machine cannot overlap two jobs.
][
  _Construction._ For each task $j$, introduce an integer completion-time variable $C_j$. For each unordered pair $i < j$, introduce a binary order variable $y_(i j)$ with $y_(i j) = 1$ meaning task $i$ finishes before task $j$. Let $M = sum_h l_h$.

  _Bounds._ $l_j <= C_j <= M$ for every task $j$, and $y_(i j) in {0, 1}$.

  _Precedence constraints._ If $i prec.eq j$, require $C_j - C_i >= l_j$.

  _Single-machine disjunction._ For every pair $i < j$, require
  $C_j - C_i + M (1 - y_(i j)) >= l_j$
  and
  $C_i - C_j + M y_(i j) >= l_i$.
  Exactly one of the two orderings is therefore active.

  _Objective._ Minimize $sum_j w_j C_j$.

  _Correctness._ ($arrow.r.double$) Any feasible schedule defines completion times and pairwise order values satisfying the bounds, precedence inequalities, and disjunctive machine constraints; its weighted completion time is exactly the ILP objective. ($arrow.l.double$) Any feasible ILP solution assigns a strict order to every task pair and forbids overlap, so the completion times correspond to a valid single-machine schedule that respects all precedences. Minimizing the ILP objective therefore minimizes the original weighted completion-time objective.

  _Solution extraction._ Sort tasks by their completion times $C_j$ and encode that order back into the source schedule representation.
]

#reduction-rule("TravelingSalesman", "ILP",
  example: true,
  example-caption: [Weighted $K_4$: the optimal tour $0 arrow 1 arrow 3 arrow 2 arrow 0$ with cost 80 is found by position-based ILP.],
)[
  A Hamiltonian tour is a permutation of vertices. Position-based encoding assigns each vertex a tour position via binary indicators, with permutation constraints ensuring a valid bijection. The tour cost involves products of position indicators for consecutive positions, which McCormick linearization converts to auxiliary variables with linear constraints.
][
  _Construction._ For graph $G = (V, E)$ with $n = |V|$ and $m = |E|$:

  _Variables:_ Binary $x_(v,k) in {0, 1}$ for each vertex $v in V$ and position $k in {0, ..., n-1}$. Interpretation: $x_(v,k) = 1$ iff vertex $v$ is at position $k$ in the tour.

  _Auxiliary variables:_ For each edge $(u,v) in E$ and position $k$, introduce $y_(u,v,k)$ and $y_(v,u,k)$ to linearize the products $x_(u,k) dot x_(v,(k+1) mod n)$ and $x_(v,k) dot x_(u,(k+1) mod n)$ respectively.

  _Constraints:_ (1) Each vertex has exactly one position: $sum_(k=0)^(n-1) x_(v,k) = 1$ for all $v in V$. (2) Each position has exactly one vertex: $sum_(v in V) x_(v,k) = 1$ for all $k$. (3) Non-edge consecutive prohibition: if ${v,w} in.not E$, then $x_(v,k) + x_(w,(k+1) mod n) <= 1$ for all $k$. (4) McCormick: $y <= x_(v,k)$, $y <= x_(w,(k+1) mod n)$, $y >= x_(v,k) + x_(w,(k+1) mod n) - 1$.

  _Objective:_ Minimize $sum_((u,v) in E) w(u,v) dot sum_k (y_(u,v,k) + y_(v,u,k))$.

  _Correctness._ ($arrow.r.double$) A valid tour defines a permutation matrix $(x_(v,k))$ satisfying constraints (1)--(2); consecutive vertices are adjacent by construction, so (3) holds; McCormick constraints (4) force $y = x_(u,k) x_(v,k+1)$, making the objective equal to the tour cost. ($arrow.l.double$) Any feasible binary solution defines a permutation (by (1)--(2)) where consecutive positions are connected by edges (by (3)), forming a Hamiltonian tour; the linearized objective equals the tour cost.

  _Solution extraction._ For each position $k$, find vertex $v$ with $x_(v,k) = 1$ to recover the tour permutation; then select edges between consecutive positions.
]

#let tsp_qubo = load-example("TravelingSalesman", "QUBO")
#let tsp_qubo_sol = tsp_qubo.solutions.at(0)

#reduction-rule("TravelingSalesman", "QUBO",
  example: true,
  example-caption: [TSP on $K_3$ with weights $w_(01) = 1$, $w_(02) = 2$, $w_(12) = 3$: the QUBO ground state encodes the optimal tour with cost $1 + 2 + 3 = 6$.],
  extra: [
    *Step 1 -- Encode each tour position as a binary variable.* A tour is a permutation of $n$ vertices. Introduce $n^2 = #tsp_qubo.target.instance.num_vars$ binary variables $x_(v,p)$: vertex $v$ is at position $p$.
    $ underbrace(x_(0,0) x_(0,1) x_(0,2), "vertex 0") #h(4pt) underbrace(x_(1,0) x_(1,1) x_(1,2), "vertex 1") #h(4pt) underbrace(x_(2,0) x_(2,1) x_(2,2), "vertex 2") $

    *Step 2 -- Penalize invalid permutations.* The penalty $A = 1 + |w_(01)| + |w_(02)| + |w_(12)| = 1 + 1 + 2 + 3 = 7$ ensures any row/column constraint violation outweighs any tour cost. Row constraints (each vertex at exactly one position) and column constraints (each position has one vertex) contribute diagonal $-7$ and off-diagonal $+14$ within each group.\

    *Step 3 -- Encode edge costs.* For each edge $(u,v)$ and position $p$, the products $x_(u,p) x_(v,(p+1) mod 3)$ and $x_(v,p) x_(u,(p+1) mod 3)$ add the edge weight $w_(u v)$ when vertices $u,v$ are consecutive in the tour. Since $K_3$ is complete, all pairs are edges with their actual weights.\

    *Step 4 -- Verify a solution.* The QUBO ground state $bold(x) = (#tsp_qubo_sol.target_config.map(str).join(", "))$ encodes a valid tour. Reading the permutation: each 3-bit group has exactly one 1 (valid permutation #sym.checkmark). The tour cost equals $w_(01) + w_(02) + w_(12) = 1 + 2 + 3 = 6$.\

    *Multiplicity:* The fixture stores one canonical optimal witness. On $K_3$ with distinct edge weights $1, 2, 3$, every Hamiltonian cycle has cost $1 + 2 + 3 = 6$ (all edges used), and 3 cyclic tours $times$ 2 directions yield $6$ permutation matrices overall.
  ],
)[
  Position-based QUBO encoding @lucas2014 maps a Hamiltonian tour to $n^2$ binary variables $x_(v,p)$, where $x_(v,p) = 1$ iff city $v$ is visited at position $p$. The QUBO Hamiltonian $H = H_A + H_B + H_C$ combines permutation constraints with the distance objective ($n^2$ variables indexed by $v dot n + p$).
][
  _Construction._ For graph $G = (V, E)$ with $n = |V|$ and edge weights $w_(u v)$. Let $A = 1 + sum_((u,v) in E) |w_(u v)|$ be the penalty coefficient.

  _Variables:_ Binary $x_(v,p) in {0, 1}$ for vertex $v in V$ and position $p in {0, dots, n-1}$. QUBO variable index: $v dot n + p$.

  _QUBO matrix:_ (1) Row constraint $H_A = A sum_v (1 - sum_p x_(v,p))^2$: diagonal $Q[v n + p, v n + p] += -A$, off-diagonal $Q[v n + p, v n + p'] += 2A$ for $p < p'$. (2) Column constraint $H_B = A sum_p (1 - sum_v x_(v,p))^2$: symmetric to $H_A$. (3) Distance $H_C = sum_((u,v) in E) w_(u v) sum_p (x_(u,p) x_(v,(p+1) mod n) + x_(v,p) x_(u,(p+1) mod n))$. For non-edges, penalty $A$ replaces $w_(u v)$.

  _Correctness._ ($arrow.r.double$) A valid tour defines a permutation matrix satisfying $H_A = H_B = 0$; the $H_C$ terms sum to the tour cost. ($arrow.l.double$) The minimum-energy state has $H_A = H_B = 0$ (penalty $A$ exceeds any tour cost), so it encodes a valid permutation; $H_C$ equals the tour cost, selecting the shortest tour.

  _Solution extraction._ From QUBO solution $x^*$, for each position $p$ find the unique vertex $v$ with $x^*_(v n + p) = 1$. Map consecutive position pairs to edge indices.
]

#reduction-rule("LongestCommonSubsequence", "ILP")[
  A bounded-witness ILP formulation turns the decision version of LCS into a feasibility problem. Binary variables choose the symbol at each witness position and, for every input string, choose where that witness position is realized. Linear constraints enforce symbol consistency and strictly increasing source positions.
][
  _Construction._ Given alphabet $Sigma$, strings $R = {r_1, dots, r_m}$, and bound $K$:

  _Variables:_ Binary $x_(p, a) in {0, 1}$ for witness position $p in {1, dots, K}$ and symbol $a in Sigma$, with $x_(p, a) = 1$ iff the $p$-th witness symbol equals $a$. For every input string $r_i$, witness position $p$, and source index $j in {1, dots, |r_i|}$, binary $y_(i, p, j) = 1$ iff the $p$-th witness symbol is matched to position $j$ of $r_i$.

  _Constraints:_ (1) Exactly one symbol per witness position: $sum_(a in Sigma) x_(p, a) = 1$ for all $p$. (2) Exactly one matched source position for each $(i, p)$: $sum_(j = 1)^(|r_i|) y_(i, p, j) = 1$. (3) Character consistency: if $r_i[j] = a$, then $y_(i, p, j) lt.eq x_(p, a)$. (4) Strictly increasing matches: for consecutive witness positions $p$ and $p + 1$, forbid $y_(i, p, j') = y_(i, p + 1, j) = 1$ whenever $j' gt.eq j$.

  _Objective:_ Use the zero objective. The target ILP is feasible iff the source LCS instance is a YES instance.

  _Correctness._ ($arrow.r.double$) If a witness $w = w_1 dots w_K$ is a common subsequence of every string, set $x_(p, w_p) = 1$ and choose, in every $r_i$, the positions where that embedding occurs. Constraints (1)--(4) are satisfied, so the ILP is feasible. ($arrow.l.double$) Any feasible ILP solution selects exactly one symbol for each witness position and exactly one realization in each source string. Character consistency ensures the chosen positions spell the same witness string in every input string, and the ordering constraints ensure those positions are strictly increasing. Therefore the extracted witness is a common subsequence of length $K$.

  _Solution extraction._ For each witness position $p$, read the unique symbol $a$ with $x_(p, a) = 1$ and output the resulting length-$K$ string.
]

#reduction-rule("MinimumMultiwayCut", "ILP")[
  The vertex-assignment + edge-cut indicator formulation @chopra1996 introduces binary variables for vertex-to-component membership and edge-cut indicators. Terminal vertices are fixed to their own components, partition constraints ensure every vertex belongs to exactly one component, and linking inequalities force the cut indicator on whenever an edge's endpoints are in different components.
][
  _Construction._ Given graph $G = (V, E, w)$ with $n = |V|$ vertices, $m = |E|$ edges, edge weights $w_e > 0$, and $k$ terminals $T = {t_0, dots, t_(k-1)}$:

  _Variables:_ (1) $y_(i v) in {0, 1}$ for $i in {0, dots, k-1}$, $v in V$: vertex $v$ belongs to the component of terminal $t_i$. (2) $x_e in {0, 1}$ for $e in E$: edge $e$ is in the cut. Total: $k n + m$ variables.

  _Constraints:_ (1) Terminal fixing: $y_(i, t_i) = 1$ for each $i$ (terminal $t_i$ is in its own component); $y_(j, t_i) = 0$ for $j eq.not i$ (each terminal excluded from other components). (2) Partition: $sum_(i=0)^(k-1) y_(i v) = 1$ for each $v in V$ (each vertex in exactly one component). (3) Edge-cut linking: for each edge $e = (u, v)$ and each terminal $i$: $x_e gt.eq y_(i u) - y_(i v)$ and $x_e gt.eq y_(i v) - y_(i u)$ (force $x_e = 1$ when endpoints are in different components). Total: $k^2 + n + 2 k m$ constraints.

  _Objective:_ Minimize $sum_(e in E) w_e dot x_e$.

  _Correctness._ ($arrow.r.double$) A multiway cut $C$ partitions $V$ into $k$ components, one per terminal. Setting $y_(i v) = 1$ iff $v$ is in $t_i$'s component and $x_e = 1$ iff $e in C$ satisfies all constraints: partition by construction, terminal fixing by definition, and linking because any edge with endpoints in different components is in $C$. The objective equals the cut weight. ($arrow.l.double$) Any feasible ILP solution defines a valid partition (by constraint (2)) separating all terminals (by constraint (1)). The linking constraints (3) force $x_e = 1$ for all cross-component edges, so the objective is at least the multiway cut weight; minimization ensures optimality.

  _Solution extraction._ For each edge $e$ at index $"idx"$, read $x_e = x^*_(k n + "idx")$. The source configuration is $"config"[e] = x_e$ (1 = cut, 0 = keep).
]

#let st_ilp = load-example("SteinerTree", "ILP")
#let st_ilp_sol = st_ilp.solutions.at(0)
#let st_edges = st_ilp.source.instance.graph.edges
#let st_weights = st_ilp.source.instance.edge_weights
#let st_terminals = st_ilp.source.instance.terminals
#let st_root = st_terminals.at(0)
#let st_non_root_terminals = range(1, st_terminals.len()).map(i => st_terminals.at(i))
#let st_selected_edge_indices = st_ilp_sol.source_config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
#let st_selected_edges = st_selected_edge_indices.map(i => st_edges.at(i))
#let st_cost = st_selected_edge_indices.map(i => st_weights.at(i)).sum()

#reduction-rule("SteinerTree", "ILP",
  example: true,
  example-caption: [Canonical Steiner tree instance ($n = #st_ilp.source.instance.graph.num_vertices$, $m = #st_edges.len()$, $|T| = #st_terminals.len()$)],
  extra: [
    *Step 1 -- Choose a root and one commodity per remaining terminal.* The canonical source instance has terminals $T = {#st_terminals.map(t => $v_#t$).join(", ")}$. The reduction fixes the first terminal as root $r = v_#st_root$ and creates one flow commodity for each remaining terminal: $v_#st_non_root_terminals.at(0)$ and $v_#st_non_root_terminals.at(1)$.

    *Step 2 -- Count the variables from the source edge order.* The first #st_edges.len() target variables are the edge selectors $bold(y) = (#st_ilp_sol.target_config.slice(0, st_edges.len()).map(str).join(", "))$, one per source edge in the order #st_edges.enumerate().map(((i, e)) => [$e_#i = (#(e.at(0)), #(e.at(1)))$]).join(", "). The remaining #(st_ilp.target.instance.num_vars - st_edges.len()) variables are directed flow indicators: $2 m (|T| - 1) = 2 times #st_edges.len() times #st_non_root_terminals.len() = #(st_ilp.target.instance.num_vars - st_edges.len())$.

    *Step 3 -- Count the constraints commodity-by-commodity.* Each non-root terminal contributes one flow-conservation equality per vertex and two capacity inequalities per source edge. For this fixture that is $#st_ilp.source.instance.graph.num_vertices times #st_non_root_terminals.len() = #(st_ilp.source.instance.graph.num_vertices * st_non_root_terminals.len())$ equalities plus $#(2 * st_edges.len()) times #st_non_root_terminals.len() = #(2 * st_edges.len() * st_non_root_terminals.len())$ inequalities, totaling #st_ilp.target.instance.constraints.len() constraints.

    *Step 4 -- Read the canonical witness pair.* The source witness selects edges ${#st_selected_edges.map(e => $(v_#(e.at(0)), v_#(e.at(1)))$).join(", ")}$, so $bold(y)$ already encodes the Steiner tree. In the target witness, the commodity for $v_2$ routes along $v_0 arrow v_1 arrow v_2$, while the commodity for $v_4$ routes along $v_0 arrow v_1 arrow v_3 arrow v_4$. Every flow 1-entry therefore sits under a selected edge variable #sym.checkmark

    *Step 5 -- Verify the objective end-to-end.* The selected-edge prefix is $bold(y) = (#st_ilp_sol.target_config.slice(0, st_edges.len()).map(str).join(", "))$, matching the source witness $(#st_ilp_sol.source_config.map(str).join(", "))$. The ILP objective is #st_selected_edge_indices.map(i => $#(st_weights.at(i))$).join($+$) $= #st_cost$, exactly the Steiner tree optimum stored in the fixture.

    *Multiplicity:* The fixture stores one canonical witness. Other optimal Steiner trees could yield different feasible ILP witnesses, but every valid witness still exposes the source solution in the first $m$ variables.
  ],
)[
  The rooted multi-commodity flow formulation @wong1984steiner @kochmartin1998steiner introduces one binary selector $y_e$ for each source edge and, for every non-root terminal $t$, one binary flow variable on each directed source edge. Flow conservation sends one unit from the root to each terminal, while the linking inequalities $f^t_(u,v) <= y_e$ ensure that every used flow arc is backed by a selected source edge. The resulting binary ILP has $m + 2 m (k - 1)$ variables and $n (k - 1) + 2 m (k - 1)$ constraints.
][
  _Construction._ Given an undirected weighted graph $G = (V, E, w)$ with strictly positive edge weights, terminals $T = {t_0, dots, t_(k-1)}$, and root $r = t_0$, introduce binary edge selectors $y_e in {0,1}$ for every $e in E$. For each non-root terminal $t in T backslash {r}$ and each directed copy of an undirected edge $(u, v) in E$, introduce a binary flow variable $f^t_(u,v) in {0,1}$. The target objective is
  $ min sum_(e in E) w_e y_e. $
  For every commodity $t$ and vertex $v$, enforce flow conservation:
  $ sum_(u : (u, v) in A) f^t_(u,v) - sum_(u : (v, u) in A) f^t_(v,u) = b_(t,v), $
  where $A$ contains both orientations of every undirected edge, $b_(t,v) = -1$ at the root $v = r$, $b_(t,v) = 1$ at the sink $v = t$, and $b_(t,v) = 0$ otherwise. For every commodity $t$ and undirected edge $e = {u, v}$, add the capacity-linking inequalities
  $ f^t_(u,v) <= y_e quad "and" quad f^t_(v,u) <= y_e. $
  Binary flow variables suffice because any Steiner tree yields a unique simple root-to-terminal path for each commodity, so every commodity can be realized as a 0/1 path indicator.

  _Correctness._ ($arrow.r.double$) If $S subset.eq E$ is a Steiner tree, set $y_e = 1$ exactly for $e in S$. For each non-root terminal $t$, the unique path from $r$ to $t$ inside the tree defines a binary flow assignment satisfying the conservation equations, and every used arc lies on a selected edge, so all linking inequalities hold. The ILP objective equals $sum_(e in S) w_e$. ($arrow.l.double$) Any feasible ILP solution with edge selector set $Y = {e in E : y_e = 1}$ supports one unit of flow from $r$ to every non-root terminal, so the selected edges contain a connected subgraph spanning all terminals. Because all edge weights are strictly positive, any cycle in the selected subgraph has positive total cost; the optimizer therefore never includes redundant edges, so the selected subgraph is already a Steiner tree. Therefore an optimal ILP solution induces a minimum-cost Steiner tree.

  _Variable mapping._ The first $m$ ILP variables are the source-edge indicators $y_0, dots, y_(m-1)$ in source edge order. For terminal $t_p$ with $p in {1, dots, k-1}$, the next block of $2 m$ variables stores the directed arc indicators $f^(t_p)_(u,v)$ and $f^(t_p)_(v,u)$ for each source edge $(u, v)$.

  _Solution extraction._ Read the first $m$ target variables as the source edge-selection vector. Since those coordinates are exactly the $y_e$ variables, the extracted source configuration is valid whenever the selected subgraph is pruned to its Steiner tree witness.

  _Remark._ Zero-weight edges are excluded because they allow degenerate optimal ILP solutions containing redundant cycles at no cost; following the convention of practical solvers (e.g., SCIP-Jack @kochmartin1998steiner), such edges should be contracted before applying the reduction.
]

== Unit Disk Mapping

#reduction-rule("MaximumIndependentSet", "KingsSubgraph")[
  @nguyen2023 The key idea is to represent each vertex of a general graph as a chain of grid nodes (a "copy line") on a King's subgraph, where adjacency is limited to unit-distance neighbors. Edges between vertices in the original graph are encoded by crossing gadgets: when two copy lines cross, the gadget ensures that at most one can be fully selected, mimicking the independence constraint. The overhead from the copy-line structure is a known constant $Delta$, so $"MIS"(G_"grid") = "MIS"(G) + Delta$, and the reduction preserves optimality with at most quadratic blowup.
][
  _Construction (Copy-Line Method)._ Given $G = (V, E)$ with $n = |V|$:

  1. _Vertex ordering:_ Compute a path decomposition of $G$ to obtain vertex order $(v_1, dots, v_n)$. The pathwidth determines the grid height.

  2. _Copy lines:_ For each vertex $v_i$, create an L-shaped "copy line" on the grid:
  $ "CopyLine"(v_i) = {(r, c_i) : r in [r_"start", r_"stop"]} union {(r_i, c) : c in [c_i, c_"stop"]} $
  where positions are determined by the vertex order and edge structure.

  3. _Crossing gadgets:_ When two copy lines cross (corresponding to an edge $(v_i, v_j) in E$), insert a crossing gadget that enforces: at most one of the two lines can be "active" (all vertices selected).

  4. _MIS correspondence:_ Each copy line has MIS contribution $approx |"line"|/2$. The gadgets add overhead $Delta$ such that:
  $ "MIS"(G_"grid") = "MIS"(G) + Delta $

  _Correctness._ ($arrow.r.double$) An IS $S$ in $G$ maps to a grid IS by activating copy lines for vertices in $S$ (selecting alternating grid nodes) and deactivating lines for vertices not in $S$. At each crossing gadget between adjacent vertices $v_i, v_j in S$, at most one line is active, but since $v_i$ and $v_j$ are not both in $S$ (they are independent), no conflict arises. ($arrow.l.double$) A grid MIS determines which copy lines are active (majority of nodes selected). Active lines correspond to an IS in $G$: if two adjacent vertices $v_i, v_j$ were both active, their crossing gadget would prevent both from contributing fully, contradicting optimality.

  _Solution extraction._ For each copy line, check if the majority of its vertices are in the grid MIS. Map back: $v_i in S$ iff copy line $i$ is active.
]

*Example: Petersen Graph.*#footnote[Generated using `cargo run --example export_petersen_mapping` from the accompanying code repository.] The Petersen graph ($n=10$, MIS$=4$) maps to a $30 times 42$ King's subgraph with 219 nodes and overhead $Delta = 89$. Solving MIS on the grid yields $"MIS"(G_"grid") = 4 + 89 = 93$. The weighted and unweighted KSG mappings share identical grid topology (same node positions and edges); only the vertex weights differ. With triangular lattice encoding @nguyen2023, the same graph maps to a $42 times 60$ grid with 395 nodes and overhead $Delta = 375$, giving $"MIS"(G_"tri") = 4 + 375 = 379$.

// Load JSON data
#let petersen = json("static/petersen_source.json")
#let square_weighted = json("static/petersen_square_weighted.json")
#let square_unweighted = json("static/petersen_square_unweighted.json")
#let triangular_mapping = json("static/petersen_triangular.json")

#figure(
  grid(
    columns: 3,
    gutter: 1.5em,
    align(center + horizon)[
      #{
        let pg = petersen-graph()
        canvas(length: 1cm, {
          for (u, v) in pg.edges { g-edge(pg.vertices.at(u), pg.vertices.at(v)) }
          for (k, pos) in pg.vertices.enumerate() {
            g-node(pos, fill: blue, stroke: none)
          }
        })
      }
      (a) Petersen graph
    ],
    align(center + horizon)[
      #draw-grid-graph(square_weighted)
      (b) King's subgraph (weighted)
    ],
    align(center + horizon)[
      #draw-triangular-graph(triangular_mapping)
      (c) Triangular lattice (weighted)
    ],
  ),
  caption: [Unit disk mappings of the Petersen graph. Blue: weight 1, red: weight 2, green: weight 3.],
) <fig:petersen-mapping>

#reduction-rule("MaximumIndependentSet", "TriangularSubgraph")[
  @nguyen2023 The same copy-line principle as the King's subgraph reduction applies, but on a triangular lattice. The triangular geometry offers a denser packing of neighbors (each node has 6 neighbors vs. 8 in the King's grid), which requires redesigned crossing and simplifier gadgets but preserves the same asymptotic overhead. The resulting graph is a unit disk graph under the triangular metric, suitable for hardware architectures based on triangular lattice connectivity.
][
  _Construction._ Same copy-line method as the KSG mapping: vertex ordering via path decomposition, L-shaped copy lines, and crossing gadgets at edge intersections. The gadgets are adapted for the triangular lattice geometry, where adjacency is defined by unit distance under the triangular metric (6 neighbors per interior node instead of 8).

  _Correctness._ ($arrow.r.double$) An IS in $G$ maps to an IS on the triangular grid by the same copy-line activation mechanism. ($arrow.l.double$) A grid MIS maps back to an IS by the copy-line activity rule, with the adapted crossing gadgets enforcing the same independence constraints.

  _Solution extraction._ Same as the KSG mapping: determine copy-line activity by majority vote, then map back to the original graph.

  _Overhead._ Both vertex and edge counts grow as $O(n^2)$ where $n = |V|$, matching the KSG mapping.
]

*Weighted Extension.* For MWIS, copy lines use weighted vertices (weights 1, 2, or 3). Source weights $< 1$ are added to designated "pin" vertices.

*QUBO Mapping.* A QUBO problem $min bold(x)^top Q bold(x)$ maps to weighted MIS on a grid by:
1. Creating copy lines for each variable
2. Using XOR gadgets for couplings: $x_"out" = not(x_1 xor x_2)$
3. Adding weights for linear and quadratic terms

See #link("https://github.com/CodingThrust/problem-reductions/blob/main/examples/export_petersen_mapping.rs")[`export_petersen_mapping.rs`].

// Completeness check: warn about reduction rules in JSON but missing from paper
#context {
  let covered = covered-rules.get()
  let json-edges = {
    let edges = graph-data.edges.map(e => (graph-data.nodes.at(e.source).name, graph-data.nodes.at(e.target).name))
    let unique = ()
    for e in edges {
      if unique.find(u => u.at(0) == e.at(0) and u.at(1) == e.at(1)) == none {
        unique.push(e)
      }
    }
    unique
  }
  let missing = json-edges.filter(e => {
    covered.find(c => c.at(0) == e.at(0) and c.at(1) == e.at(1)) == none
  })
  if missing.len() > 0 {
    block(width: 100%, inset: (x: 1em, y: 0.5em), fill: rgb("#fff3cd"), stroke: (left: 3pt + rgb("#ffc107")))[
      #text(fill: rgb("#856404"), weight: "bold")[Warning: Missing reduction rules:] \
      #for m in missing [
        #text(fill: rgb("#856404"))[- #m.at(0) #sym.arrow.r #m.at(1)] \
      ]
    ]
  }
}

== Resource Estimation from Examples

The following table shows concrete variable overhead for example instances, taken directly from the canonical fixture examples.

#let example-files = (
  (source: "MaximumIndependentSet", target: "MinimumVertexCover"),
  (source: "MinimumVertexCover", target: "MaximumIndependentSet"),
  (
    source: "MaximumIndependentSet",
    target: "MaximumSetPacking",
    source-variant: (graph: "SimpleGraph", weight: "One"),
    target-variant: (weight: "One"),
  ),
  (source: "MaximumMatching", target: "MaximumSetPacking"),
  (source: "MinimumVertexCover", target: "MinimumSetCovering"),
  (source: "MaxCut", target: "SpinGlass"),
  (source: "SpinGlass", target: "MaxCut"),
  (source: "SpinGlass", target: "QUBO"),
  (source: "QUBO", target: "SpinGlass"),
  (source: "ClosestVectorProblem", target: "QUBO"),
  (source: "KColoring", target: "QUBO"),
  (source: "MaximumSetPacking", target: "QUBO"),
  (
    source: "KSatisfiability",
    target: "QUBO",
    source-variant: (k: "K3"),
    target-variant: (weight: "f64"),
  ),
  (source: "ILP", target: "QUBO"),
  (source: "Satisfiability", target: "MaximumIndependentSet"),
  (source: "Satisfiability", target: "KColoring"),
  (source: "Satisfiability", target: "MinimumDominatingSet"),
  (source: "Satisfiability", target: "KSatisfiability"),
  (source: "CircuitSAT", target: "SpinGlass"),
  (source: "Factoring", target: "CircuitSAT"),
  (source: "MaximumSetPacking", target: "ILP"),
  (source: "MaximumMatching", target: "ILP"),
  (source: "KColoring", target: "ILP"),
  (source: "Factoring", target: "ILP"),
  (source: "MinimumSetCovering", target: "ILP"),
  (source: "MinimumDominatingSet", target: "ILP"),
  (source: "MaximumClique", target: "ILP"),
  (source: "TravelingSalesman", target: "ILP"),
)

#let examples = example-files.map(entry => {
  let d = load-example(
    entry.source,
    entry.target,
    source-variant: entry.at("source-variant", default: none),
    target-variant: entry.at("target-variant", default: none),
  )
  (name: example-name(entry.source, entry.target), data: d)
})

#pagebreak()
#bibliography("references.bib", style: "ieee")
