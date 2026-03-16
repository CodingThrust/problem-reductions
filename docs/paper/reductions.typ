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
#let example-db = json("../../src/example_db/fixtures/examples.json")

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

#let graph-num-vertices(instance) = instance.graph.inner.nodes.len()
#let graph-num-edges(instance) = instance.graph.inner.edges.len()
#let spin-num-spins(instance) = instance.fields.len()
#let sat-num-clauses(instance) = instance.clauses.len()
#let subsetsum-num-elements(instance) = instance.sizes.len()
#let circuit-num-gates(instance) = instance.circuit.assignments.len()
#let circuit-num-variables(instance) = instance.variables.len()

#let example-name(source, target) = lower(source) + "_to_" + lower(target)

#let problem-schemas = json("../src/reductions/problem_schemas.json")

// Problem display names for theorem headers
#let display-name = (
  "MaximumIndependentSet": [Maximum Independent Set],
  "MinimumVertexCover": [Minimum Vertex Cover],
  "MaxCut": [Max-Cut],
  "GraphPartitioning": [Graph Partitioning],
  "HamiltonianPath": [Hamiltonian Path],
  "IsomorphicSpanningTree": [Isomorphic Spanning Tree],
  "KColoring": [$k$-Coloring],
  "MinimumDominatingSet": [Minimum Dominating Set],
  "MaximumMatching": [Maximum Matching],
  "TravelingSalesman": [Traveling Salesman],
  "MaximumClique": [Maximum Clique],
  "MaximumSetPacking": [Maximum Set Packing],
  "MinimumSetCovering": [Minimum Set Covering],
  "ComparativeContainment": [Comparative Containment],
  "SetBasis": [Set Basis],
  "SpinGlass": [Spin Glass],
  "QUBO": [QUBO],
  "ILP": [Integer Linear Programming],
  "Knapsack": [Knapsack],
  "Satisfiability": [SAT],
  "KSatisfiability": [$k$-SAT],
  "CircuitSAT": [CircuitSAT],
  "Factoring": [Factoring],
  "KingsSubgraph": [King's Subgraph MIS],
  "TriangularSubgraph": [Triangular Subgraph MIS],
  "MaximalIS": [Maximal Independent Set],
  "BMF": [Boolean Matrix Factorization],
  "PaintShop": [Paint Shop],
  "BicliqueCover": [Biclique Cover],
  "BinPacking": [Bin Packing],
  "ClosestVectorProblem": [Closest Vector Problem],
  "OptimalLinearArrangement": [Optimal Linear Arrangement],
  "RuralPostman": [Rural Postman],
  "LongestCommonSubsequence": [Longest Common Subsequence],
  "ExactCoverBy3Sets": [Exact Cover by 3-Sets],
  "SubsetSum": [Subset Sum],
  "MinimumFeedbackArcSet": [Minimum Feedback Arc Set],
  "MinimumFeedbackVertexSet": [Minimum Feedback Vertex Set],
  "ShortestCommonSupersequence": [Shortest Common Supersequence],
  "MinimumSumMulticenter": [Minimum Sum Multicenter],
  "SteinerTree": [Steiner Tree],
  "SubgraphIsomorphism": [Subgraph Isomorphism],
  "PartitionIntoTriangles": [Partition Into Triangles],
  "FlowShopScheduling": [Flow Shop Scheduling],
  "MinimumTardinessSequencing": [Minimum Tardiness Sequencing],
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
  // Pick optimal[2] = {v1, v3, v5, v9} to match figure
  let sol = x.optimal.at(2)
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
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // Pick optimal[2] = {v0, v3, v4} to match figure
  let sol = x.optimal.at(2)
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
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // Pick optimal[2] = S={v0, v3} to match figure
  let sol = x.optimal.at(2)
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
#problem-def("GraphPartitioning")[
  Given an undirected graph $G = (V, E)$ with $|V| = n$ (even), find a partition of $V$ into two disjoint sets $A$ and $B$ with $|A| = |B| = n slash 2$ that minimizes the number of edges crossing the partition:
  $ "cut"(A, B) = |{(u, v) in E : u in A, v in B}|. $
][
Graph Partitioning is a core NP-hard problem arising in VLSI design, parallel computing, and scientific simulation, where balanced workload distribution with minimal communication is essential. Closely related to Max-Cut (which _maximizes_ rather than _minimizes_ the cut) and to the Ising Spin Glass model. NP-completeness was proved by Garey, Johnson and Stockmeyer @garey1976. Arora, Rao and Vazirani @arora2009 gave an $O(sqrt(log n))$-approximation algorithm. The best known unconditional exact algorithm is brute-force enumeration of all $binom(n, n slash 2) = O^*(2^n)$ balanced partitions; no faster worst-case algorithm is known. Cygan et al. @cygan2014 showed that Minimum Bisection is fixed-parameter tractable in $O(2^(O(k^3)) dot n^3 log^3 n)$ time parameterized by bisection width $k$. Standard partitioning tools include METIS, KaHIP, and Scotch.

*Example.* Consider the graph $G$ with $n = 6$ vertices and 9 edges: $(v_0, v_1)$, $(v_0, v_2)$, $(v_1, v_2)$, $(v_1, v_3)$, $(v_2, v_3)$, $(v_2, v_4)$, $(v_3, v_4)$, $(v_3, v_5)$, $(v_4, v_5)$. The optimal balanced partition is $A = {v_0, v_1, v_2}$, $B = {v_3, v_4, v_5}$, with cut value 3: the crossing edges are $(v_1, v_3)$, $(v_2, v_3)$, $(v_2, v_4)$. All other balanced partitions yield a cut of at least 3.

#figure(
  canvas(length: 1cm, {
    // 6-vertex layout: two columns of 3
    let verts = (
      (0, 2),     // v0: top-left
      (0, 1),     // v1: mid-left
      (0, 0),     // v2: bottom-left
      (2.5, 2),   // v3: top-right
      (2.5, 1),   // v4: mid-right
      (2.5, 0),   // v5: bottom-right
    )
    let edges = ((0,1),(0,2),(1,2),(1,3),(2,3),(2,4),(3,4),(3,5),(4,5))
    let side-a = (0, 1, 2)
    let cut-edges = edges.filter(e => side-a.contains(e.at(0)) != side-a.contains(e.at(1)))
    // Draw edges
    for (u, v) in edges {
      let crossing = cut-edges.any(e => (e.at(0) == u and e.at(1) == v) or (e.at(0) == v and e.at(1) == u))
      g-edge(verts.at(u), verts.at(v),
        stroke: if crossing { 2pt + graph-colors.at(1) } else { 1pt + luma(180) })
    }
    // Draw partition regions
    import draw: *
    on-layer(-1, {
      rect((-0.5, -0.5), (0.5, 2.5),
        fill: graph-colors.at(0).transparentize(90%),
        stroke: (dash: "dashed", paint: graph-colors.at(0), thickness: 0.8pt))
      content((0, 2.8), text(8pt, fill: graph-colors.at(0))[$A$])
      rect((2.0, -0.5), (3.0, 2.5),
        fill: graph-colors.at(1).transparentize(90%),
        stroke: (dash: "dashed", paint: graph-colors.at(1), thickness: 0.8pt))
      content((2.5, 2.8), text(8pt, fill: graph-colors.at(1))[$B$])
    })
    // Draw nodes
    for (k, pos) in verts.enumerate() {
      let in-a = side-a.contains(k)
      g-node(pos, name: "v" + str(k),
        fill: if in-a { graph-colors.at(0) } else { graph-colors.at(1) },
        label: text(fill: white)[$v_#k$])
    }
  }),
  caption: [Graph with $n = 6$ vertices partitioned into $A = {v_0, v_1, v_2}$ (blue) and $B = {v_3, v_4, v_5}$ (red). The 3 crossing edges $(v_1, v_3)$, $(v_2, v_3)$, $(v_2, v_4)$ are shown in bold red; internal edges are gray.],
) <fig:graph-partitioning>
]
#{
  let x = load-model-example("HamiltonianPath")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // Pick optimal[2] = [0, 2, 4, 3, 1, 5] to match figure
  let sol = x.optimal.at(2)
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
  let x = load-model-example("IsomorphicSpanningTree")
  let g-edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  let t-edges = x.instance.tree.inner.edges.map(e => (e.at(0), e.at(1)))
  let nv = x.instance.graph.inner.nodes.len()
  let nt = x.instance.tree.inner.nodes.len()
  // optimal[0] = identity mapping [0,1,2,3]
  let sol = x.optimal.at(0)
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
  let x = load-model-example("KColoring")
  let nv = graph-num-vertices(x.instance)
  let k = x.instance.num_colors
  // Pick optimal[0] = [0,1,1,0,2] to match figure
  let sol = x.optimal.at(0)
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
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // Pick optimal[0] = {v2, v3} to match figure
  let sol = x.optimal.at(0)
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
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // Pick optimal[4] = config [1,0,0,0,1,0] = edges {(0,1),(2,4)} to match figure
  let sol = x.optimal.at(4)
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
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  let ew = x.instance.edge_weights
  let sol = x.optimal.at(0)
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
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  let weights = x.instance.edge_weights
  let terminals = x.instance.terminals
  let sol = x.optimal.at(0)
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
#problem-def("OptimalLinearArrangement")[
  Given an undirected graph $G=(V,E)$ and a non-negative integer $K$, is there a bijection $f: V -> {0, 1, dots, |V|-1}$ such that $sum_({u,v} in E) |f(u) - f(v)| <= K$?
][
A classical NP-complete decision problem from Garey & Johnson (GT42) @garey1979, with applications in VLSI design, graph drawing, and sparse matrix reordering. The problem asks whether vertices can be placed on a line so that the total "stretch" of all edges is at most $K$.

NP-completeness was established by Garey, Johnson, and Stockmeyer @gareyJohnsonStockmeyer1976, via reduction from Simple Max Cut. The problem remains NP-complete on bipartite graphs, but is solvable in polynomial time on trees. The best known exact algorithm for general graphs uses dynamic programming over subsets in $O^*(2^n)$ time and space (Held-Karp style), analogous to TSP.

*Example.* Consider the path graph $P_3$: vertices ${v_0, v_1, v_2}$ with edges ${v_0, v_1}$ and ${v_1, v_2}$. The identity arrangement $f(v_i) = i$ gives cost $|0-1| + |1-2| = 2$. With bound $K = 2$, this is a YES instance. For a triangle $K_3$ with the same vertex set plus edge ${v_0, v_2}$, any arrangement yields cost 4, so a bound of $K = 3$ gives a NO instance.
]
#{
  let x = load-model-example("MaximumClique")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // optimal[0] = {v2, v3, v4}
  let sol = x.optimal.at(0)
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
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // optimal[0] = {v0,v2,v4} with w=3 (maximum-weight maximal IS)
  let opt = x.optimal.at(0)
  let S-opt = opt.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let w-opt = opt.metric.Valid
  // samples[0] = {v1,v3} with w=2 (suboptimal maximal IS)
  let sub = x.samples.at(0)
  let S-sub = sub.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let w-sub = sub.metric.Valid
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
  let ne = graph-num-edges(x.instance)
  let arcs = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  // Pick optimal[1] = {v0} to match figure
  let sol = x.optimal.at(1)
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

#{
  let x = load-model-example("MinimumSumMulticenter")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  let K = x.instance.k
  let opt-cost = x.optimal.at(2).metric.Valid
  // Pick optimal[2] = {v2, v5} to match figure
  let sol = x.optimal.at(2)
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

== Set Problems

#{
  let x = load-model-example("MaximumSetPacking")
  let sets = x.instance.sets
  let m = sets.len()
  // Compute universe size from all elements
  let all-elems = sets.flatten().dedup()
  let U-size = all-elems.len()
  // Pick optimal[2] = {S1, S3} (0-indexed: sets 0, 2) to match figure
  let sol = x.optimal.at(2)
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
  let sol = x.optimal.at(0)
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
  let x3c = load-model-example("ExactCoverBy3Sets")
  let n = x3c.instance.universe_size
  let q = int(n / 3)
  let subs = x3c.instance.subsets
  let m = subs.len()
  let sol = x3c.optimal.at(0).config
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
  let sample = x.samples.at(0)
  let selected = sample.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let satisfiers = x.optimal.map(sol => sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i))
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
  let sample = x.samples.at(0)
  let sat-count = x.optimal.len()
  let basis = range(k).map(i =>
    range(U-size).filter(j => sample.config.at(i * U-size + j) == 1)
  )
  let fmt-set(s) = "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  [
    #problem-def("SetBasis")[
      Given finite set $S$, collection $cal(C)$ of subsets of $S$, and integer $k$, does there exist a family $cal(B) = {B_1, ..., B_k}$ with each $B_i subset.eq S$ such that for every $C in cal(C)$ there exists $cal(B)_C subset.eq cal(B)$ with $union.big_(B in cal(B)_C) B = C$?
    ][
    The Set Basis problem was shown NP-complete by Stockmeyer @stockmeyer1975setbasis and appears as SP7 in Garey & Johnson @garey1979. It asks for an exact union-based description of a family of sets, unlike Set Cover which only requires covering the underlying universe. Applications include data compression, database schema design, and Boolean function minimization. The library's decision encoding uses $k |S|$ membership bits, so brute-force over those bits gives an $O^*(2^(k |S|))$ exact algorithm#footnote[This is the direct search bound induced by the encoding implemented here; we are not aware of a faster general exact worst-case algorithm for this representation.].

    *Example.* Let $S = {1, 2, 3, 4}$, $k = #k$, and $cal(C) = {#range(m).map(i => $C_#(i + 1)$).join(", ")}$ with #coll.enumerate().map(((i, s)) => $C_#(i + 1) = #fmt-set(s)$).join(", "). The sample basis from the issue is $cal(B) = {#range(k).map(i => $B_#(i + 1)$).join(", ")}$ with #basis.enumerate().map(((i, s)) => $B_#(i + 1) = #fmt-set(s)$).join(", "). Then $C_1 = B_1 union B_2$, $C_2 = B_2 union B_3$, $C_3 = B_1 union B_3$, and $C_4 = B_1 union B_2 union B_3$. There are #sat-count satisfying encodings in total: the singleton basis can be permuted in $3! = 6$ ways, and the three pair sets $C_1, C_2, C_3$ also form a basis with another six row permutations.

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

== Optimization Problems

#{
  let x = load-model-example("SpinGlass")
  let n = spin-num-spins(x.instance)
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  let ne = edges.len()
  // Pick optimal[1] = (+,-,+,+,-) to match figure
  let sol = x.optimal.at(1)
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
  let sol = x.optimal.at(0)
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
  let sol = x.optimal.at(0)
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
  let x = load-model-example("ClosestVectorProblem")
  let basis = x.instance.basis
  let target = x.instance.target
  let bounds = x.instance.bounds
  let sol = x.optimal.at(0)
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
  let sol = x.optimal.at(1)  // pick (1,0,1)
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
  let x = load-model-example("KSatisfiability")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let k = x.instance.clauses.at(0).literals.len()
  let clauses = x.instance.clauses
  // Pick a satisfying assignment
  let sol = x.optimal.at(0)
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
  for o in x.optimal {
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

#{
  let x = load-model-example("Factoring")
  let N = x.instance.target
  let mb = x.instance.m
  let nb = x.instance.n
  let sol = x.optimal.at(0).config
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

== Specialized Problems

#{
  let x = load-model-example("BMF")
  let mr = x.instance.m
  let nc = x.instance.n
  let k = x.instance.k
  let A = x.instance.matrix
  let dH = x.optimal.at(0).metric.Valid
  // Decode B and C from optimal config
  // Config layout: B is m*k values, then C is k*n values
  let cfg = x.optimal.at(0).config
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
  let x = load-model-example("PaintShop")
  let n-cars = x.instance.num_cars
  let labels = x.instance.car_labels
  let seq-indices = x.instance.sequence_indices
  let is-first = x.instance.is_first
  let sol = x.optimal.at(0)
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
  let sol = x.optimal.at(0)
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
  let x = load-model-example("PartitionIntoTriangles")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.inner.edges.map(e => (e.at(0), e.at(1)))
  let q = int(nv / 3)
  // optimal[0] config groups vertices into triangles: config[i] = triangle index
  let sol = x.optimal.at(0)
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

#problem-def("BinPacking")[
  Given $n$ items with sizes $s_1, dots, s_n in RR^+$ and bin capacity $C > 0$, find an assignment $x: {1, dots, n} -> NN$ minimizing $|{x(i) : i = 1, dots, n}|$ (the number of distinct bins used) subject to $forall j: sum_(i: x(i) = j) s_i lt.eq C$.
][
  Bin Packing is one of the classical NP-hard optimization problems @garey1979, with applications in logistics, cutting stock, and cloud resource allocation. The best known exact algorithm runs in $O^*(2^n)$ time via inclusion-exclusion over set partitions @bjorklund2009.

  *Example.* Consider $n = 6$ items with sizes $(6, 6, 5, 5, 4, 4)$ and capacity $C = 10$. The lower bound is $ceil(30 slash 10) = 3$ bins. An optimal packing uses exactly 3 bins: $B_1 = {6, 4}$, $B_2 = {6, 4}$, $B_3 = {5, 5}$, each with total load $10 = C$.

  #figure({
    canvas(length: 1cm, {
      let s = 0.28
      let w = 1.0
      let gap = 0.6
      let bins = ((6, 4), (6, 4), (5, 5))
      let fills = (
        (graph-colors.at(0), graph-colors.at(1)),
        (graph-colors.at(0), graph-colors.at(1)),
        (graph-colors.at(2), graph-colors.at(2)),
      )
      for i in range(3) {
        let x = i * (w + gap)
        draw.rect((x, 0), (x + w, 10 * s), stroke: 0.8pt + black)
        let y = 0
        for j in range(bins.at(i).len()) {
          let sz = bins.at(i).at(j)
          let c = fills.at(i).at(j)
          draw.rect((x, y), (x + w, y + sz * s), stroke: 0.4pt, fill: c)
          draw.content((x + w / 2, y + sz * s / 2), text(8pt, fill: white)[#sz])
          y += sz * s
        }
        draw.content((x + w / 2, -0.3), text(8pt)[$B_#(i + 1)$])
      }
      draw.line((-0.15, 10 * s), (2 * (w + gap) + w + 0.15, 10 * s),
        stroke: (dash: "dashed", paint: luma(150), thickness: 0.5pt))
      draw.content((-0.5, 10 * s), text(7pt)[$C$])
    })
  },
  caption: [Optimal packing of items with sizes $(6, 6, 5, 5, 4, 4)$ into 3 bins of capacity $C = 10$. Numbers indicate item sizes; all bins are fully utilized.],
  ) <fig:binpacking-example>
]

#problem-def("Knapsack")[
  Given $n$ items with weights $w_0, dots, w_(n-1) in NN$ and values $v_0, dots, v_(n-1) in NN$, and a capacity $C in NN$, find $S subset.eq {0, dots, n - 1}$ maximizing $sum_(i in S) v_i$ subject to $sum_(i in S) w_i lt.eq C$.
][
  One of Karp's 21 NP-complete problems @karp1972. Knapsack is only _weakly_ NP-hard: a classical dynamic-programming algorithm runs in $O(n C)$ pseudo-polynomial time, and a fully polynomial-time approximation scheme (FPTAS) achieves $(1 - epsilon)$-optimal value in $O(n^2 slash epsilon)$ time @ibarra1975. The special case $v_i = w_i$ for all $i$ is the Subset Sum problem. Knapsack is also a special case of Integer Linear Programming with a single constraint. The best known exact algorithm is the $O^*(2^(n slash 2))$ meet-in-the-middle approach of Horowitz and Sahni @horowitz1974, which partitions items into two halves and combines sorted sublists.

  *Example.* Let $n = 4$ items with weights $(2, 3, 4, 5)$, values $(3, 4, 5, 7)$, and capacity $C = 7$. Selecting $S = {1, 2}$ (items with weights 3 and 4) gives total weight $3 + 4 = 7 lt.eq C$ and total value $4 + 5 = 9$. Selecting $S = {0, 3}$ (weights 2 and 5) gives weight $2 + 5 = 7 lt.eq C$ and value $3 + 7 = 10$, which is optimal.
]

#problem-def("RuralPostman")[
  Given an undirected graph $G = (V, E)$ with edge lengths $l: E -> ZZ_(gt.eq 0)$, a subset $E' subset.eq E$ of required edges, and a bound $B in ZZ^+$, determine whether there exists a circuit (closed walk) in $G$ that traverses every edge in $E'$ and has total length at most $B$.
][
  The Rural Postman Problem (RPP) is a fundamental NP-complete arc-routing problem @lenstra1976 that generalizes the Chinese Postman Problem. When $E' = E$, the problem reduces to finding an Eulerian circuit with minimum augmentation (polynomial-time solvable via $T$-join matching). For general $E' subset.eq E$, exact algorithms use dynamic programming over subsets of required edges in $O(n^2 dot 2^r)$ time, where $r = |E'|$ and $n = |V|$, analogous to the Held-Karp algorithm for TSP. The problem admits a $3 slash 2$-approximation for metric instances @frederickson1979.

  *Example.* Consider a hexagonal graph with 6 vertices and 8 edges, where all outer edges have length 1 and two diagonal edges have length 2. The required edges are $E' = {(v_0, v_1), (v_2, v_3), (v_4, v_5)}$ with bound $B = 6$. The outer cycle $v_0 -> v_1 -> v_2 -> v_3 -> v_4 -> v_5 -> v_0$ covers all three required edges with total length $6 times 1 = 6 = B$, so the answer is YES.
]

#problem-def("SubgraphIsomorphism")[
  Given graphs $G = (V_1, E_1)$ (host) and $H = (V_2, E_2)$ (pattern), determine whether $G$ contains a subgraph isomorphic to $H$: does there exist an injective function $f: V_2 -> V_1$ such that ${u, v} in E_2 arrow.double {f(u), f(v)} in E_1$?
][
  Subgraph Isomorphism (GT48 in Garey & Johnson @garey1979) is NP-complete by transformation from Clique @garey1979. It strictly generalizes Clique (where $H = K_k$) and also contains Hamiltonian Circuit ($H = C_n$) and Hamiltonian Path ($H = P_n$) as special cases. Brute-force enumeration of all injective mappings $f: V_2 -> V_1$ runs in $O(|V_1|^(|V_2|) dot |E_2|)$ time. For fixed-size patterns, the color-coding technique of Alon, Yuster, and Zwick @alon1995 gives a randomized algorithm in $2^(O(|V_2|)) dot |V_1|^(O("tw"(H)))$ time. Practical algorithms include VF2 @cordella2004 and VF2++ @juttner2018.

  *Example.* Consider host graph $G$ with 7 vertices: a $K_4$ clique on ${0, 1, 2, 3}$ and a triangle on ${4, 5, 6}$ connected via edge $(3, 4)$. Pattern $H = K_4$ with vertices ${a, b, c, d}$. The mapping $f(a) = 0, f(b) = 1, f(c) = 2, f(d) = 3$ preserves all 6 edges of $K_4$, confirming a subgraph isomorphism exists.
]

#problem-def("LongestCommonSubsequence")[
  Given $k$ strings $s_1, dots, s_k$ over a finite alphabet $Sigma$, find a longest string $w$ that is a subsequence of every $s_i$. A string $w$ is a _subsequence_ of $s$ if $w$ can be obtained by deleting zero or more characters from $s$ without changing the order of the remaining characters.
][
  The LCS problem is polynomial-time solvable for $k = 2$ strings via dynamic programming in $O(n_1 n_2)$ time (Wagner & Fischer, 1974), but NP-hard for $k gt.eq 3$ strings @maier1978. It is a foundational problem in bioinformatics (sequence alignment), version control (diff algorithms), and data compression. The problem is listed as SR10 in Garey & Johnson @garey1979.

  *Example.* Let $s_1 = $ `ABAC` and $s_2 = $ `BACA` over $Sigma = {A, B, C}$. The longest common subsequence has length 3, e.g., `BAC`: positions 1, 2, 3 of $s_1$ match positions 0, 1, 2 of $s_2$.
]

#problem-def("SubsetSum")[
  Given a finite set $A = {a_0, dots, a_(n-1)}$ with sizes $s(a_i) in ZZ^+$ and a target $B in ZZ^+$, determine whether there exists a subset $A' subset.eq A$ such that $sum_(a in A') s(a) = B$.
][
  One of Karp's 21 NP-complete problems @karp1972. Subset Sum is the special case of Knapsack where $v_i = w_i$ for all items and we seek an exact sum rather than an inequality. Though NP-complete, it is only _weakly_ NP-hard: a dynamic-programming algorithm runs in $O(n B)$ pseudo-polynomial time. The best known exact algorithm is the $O^*(2^(n slash 2))$ meet-in-the-middle approach of Horowitz and Sahni @horowitz1974.

  *Example.* Let $A = {3, 7, 1, 8, 2, 4}$ ($n = 6$) and target $B = 11$. Selecting $A' = {3, 8}$ gives sum $3 + 8 = 11 = B$. Another solution: $A' = {7, 4}$ with sum $7 + 4 = 11 = B$.
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
  // Pick optimal[1] = [1,0,1,2] = "babc" to match figure
  let sol = x.optimal.at(1)
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

      *Example.* Let $Sigma = {#alpha-map.join(", ")}$ and $R = {#r-strs.join(", ")}$. We seek the shortest string $w$ containing both #r-strs.join(" and ") as subsequences.

      #figure({
        let blue = graph-colors.at(0)
        let teal = rgb("#76b7b2")
        align(center, stack(dir: ttb, spacing: 0.6cm,
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.2cm, height: 0.5cm, align(center + horizon, text(8pt)[$w =$])),
            ..w.enumerate().map(((i, ch)) => {
              let is1 = embeds.at(0).contains(i)
              let is2 = embeds.at(1).contains(i)
              let fill = if is1 and is2 { blue.transparentize(60%) } else if is1 { blue.transparentize(80%) } else if is2 { teal.transparentize(80%) } else { white }
              box(width: 0.55cm, height: 0.55cm, fill: fill, stroke: 0.5pt + luma(120),
                align(center + horizon, text(9pt, weight: "bold", ch)))
            }),
          ),
          ..range(nr).map(ri => {
            let embed = embeds.at(ri)
            let r = r-chars.at(ri)
            let col = if ri == 0 { blue } else { teal }
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
      caption: [Shortest Common Supersequence: $w = #w-str$ (length #w-len) contains #range(nr).map(ri => [$r_#(ri + 1) = #r-strs.at(ri)$ (#if ri == 0 [blue] else [teal], positions #embeds.at(ri).map(p => str(p)).join(","))]).join(" and ") as subsequences. Dots mark unused positions in each embedding.],
      ) <fig:scs>

      The supersequence $w = #w-str$ has length #w-len and contains both input strings as subsequences. This is optimal because $"LCS"(#r-strs.join(", ")) = "ac"$ (length 2), so the shortest common supersequence has length $#strings.at(0).len() + #strings.at(1).len() - 2 = #w-len$.
    ]
  ]
}

#problem-def("MinimumFeedbackArcSet")[
  Given a directed graph $G = (V, A)$, find a minimum-size subset $A' subset.eq A$ such that $G - A'$ is a directed acyclic graph (DAG). Equivalently, $A'$ must contain at least one arc from every directed cycle in $G$.
][
  Feedback Arc Set (FAS) is a classical NP-complete problem from Karp's original list @karp1972 (via transformation from Vertex Cover, as presented in Garey & Johnson GT8). The problem arises in ranking aggregation, sports scheduling, deadlock avoidance, and causal inference. Unlike the undirected analogue (which is trivially polynomial --- the number of non-tree edges in a spanning forest), the directed version is NP-hard due to the richer structure of directed cycles. The best known exact algorithm uses dynamic programming over vertex subsets in $O^*(2^n)$ time, generalizing the Held--Karp TSP technique to vertex ordering problems @bodlaender2012. FAS is fixed-parameter tractable with parameter $k = |A'|$: an $O(4^k dot k! dot n^(O(1)))$ algorithm exists via iterative compression @chen2008. Polynomial-time solvable for planar digraphs via the Lucchesi--Younger theorem @lucchesi1978.

  *Example.* Consider $G$ with $V = {0, 1, 2, 3, 4, 5}$ and arcs $(0 arrow 1), (1 arrow 2), (2 arrow 0), (1 arrow 3), (3 arrow 4), (4 arrow 1), (2 arrow 5), (5 arrow 3), (3 arrow 0)$. This graph contains four directed cycles: $0 arrow 1 arrow 2 arrow 0$, $1 arrow 3 arrow 4 arrow 1$, $0 arrow 1 arrow 3 arrow 0$, and $2 arrow 5 arrow 3 arrow 0 arrow 1 arrow 2$. Removing $A' = {(0 arrow 1), (3 arrow 4)}$ breaks all four cycles (vertex 0 becomes a sink in the residual graph), giving a minimum FAS of size 2.
]

#problem-def("FlowShopScheduling")[
  Given $m$ processors and a set $J$ of $n$ jobs, where each job $j in J$ consists of $m$ tasks $t_1 [j], t_2 [j], dots, t_m [j]$ with lengths $ell(t_i [j]) in ZZ^+_0$, and a deadline $D in ZZ^+$, determine whether there exists a permutation schedule $pi$ of the jobs such that all jobs complete by time $D$. Each job must be processed on machines $1, 2, dots, m$ in order, and job $j$ cannot start on machine $i+1$ until its task on machine $i$ is completed.
][
  Flow Shop Scheduling is a classical NP-complete problem from Garey & Johnson (A5 SS15), strongly NP-hard for $m >= 3$ @garey1976. For $m = 2$, it is solvable in $O(n log n)$ by Johnson's rule @johnson1954. The problem is fundamental in operations research, manufacturing planning, and VLSI design. When restricted to permutation schedules (same job order on all machines), the search space is $n!$ orderings. The best known exact algorithm for $m = 3$ runs in $O^*(3^n)$ time @shang2018; for general $m$, brute-force over $n!$ permutations gives $O(n! dot m n)$.

  *Example.* Let $m = 3$ machines, $n = 5$ jobs with task lengths:
  $ ell = mat(
    3, 4, 2;
    2, 3, 5;
    4, 1, 3;
    1, 5, 4;
    3, 2, 3;
  ) $
  and deadline $D = 25$. The job order $pi = (j_4, j_1, j_5, j_3, j_2)$ (0-indexed: $3, 0, 4, 2, 1$) yields makespan $23 <= 25$, so a feasible schedule exists.

  #figure(
    canvas(length: 1cm, {
      import draw: *
      // Gantt chart for job order [3, 0, 4, 2, 1] on 3 machines
      // Schedule computed greedily:
      // M1: j3[0,1], j0[1,4], j4[4,7], j2[7,11], j1[11,13]
      // M2: j3[1,6], j0[6,10], j4[10,12], j2[12,13], j1[13,16]
      // M3: j3[6,10], j0[10,12], j4[12,15], j2[15,18], j1[18,23]
      let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"))
      let job-names = ("$j_1$", "$j_2$", "$j_3$", "$j_4$", "$j_5$")
      let scale = 0.38
      let row-h = 0.6
      let gap = 0.15

      // Machine labels
      for (mi, label) in ("M1", "M2", "M3").enumerate() {
        let y = -mi * (row-h + gap)
        content((-0.8, y), text(8pt, label))
      }

      // Draw schedule blocks: (machine, job-index, start, end)
      let blocks = (
        (0, 3, 0, 1), (0, 0, 1, 4), (0, 4, 4, 7), (0, 2, 7, 11), (0, 1, 11, 13),
        (1, 3, 1, 6), (1, 0, 6, 10), (1, 4, 10, 12), (1, 2, 12, 13), (1, 1, 13, 16),
        (2, 3, 6, 10), (2, 0, 10, 12), (2, 4, 12, 15), (2, 2, 15, 18), (2, 1, 18, 23),
      )

      for (mi, ji, s, e) in blocks {
        let x0 = s * scale
        let x1 = e * scale
        let y = -mi * (row-h + gap)
        rect((x0, y - row-h / 2), (x1, y + row-h / 2),
          fill: colors.at(ji).transparentize(30%), stroke: 0.4pt + colors.at(ji))
        content(((x0 + x1) / 2, y), text(6pt, job-names.at(ji)))
      }

      // Time axis
      let max-t = 23
      let y-axis = -2 * (row-h + gap) - row-h / 2 - 0.2
      line((0, y-axis), (max-t * scale, y-axis), stroke: 0.4pt)
      for t in (0, 5, 10, 15, 20, 23) {
        let x = t * scale
        line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
        content((x, y-axis - 0.25), text(6pt, str(t)))
      }
      content((max-t * scale / 2, y-axis - 0.5), text(7pt)[$t$])

      // Deadline marker
      let dl-x = 25 * scale
      line((dl-x, row-h / 2 + 0.1), (dl-x, y-axis), stroke: (paint: red, thickness: 0.8pt, dash: "dashed"))
      content((dl-x, row-h / 2 + 0.25), text(6pt, fill: red)[$D = 25$])
    }),
    caption: [Flow shop schedule for 5 jobs on 3 machines. Job order $(j_4, j_1, j_5, j_3, j_2)$ achieves makespan 23, within deadline $D = 25$ (dashed red line).],
  ) <fig:flowshop>
]

#{
  let x = load-model-example("MinimumTardinessSequencing")
  let ntasks = x.instance.num_tasks
  let deadlines = x.instance.deadlines
  let precs = x.instance.precedences
  let sol = x.optimal.at(0)
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

#reduction-rule("MaximumIndependentSet", "MinimumVertexCover")[
  The exact reverse of VC $arrow.r$ IS: complementing an independent set yields a vertex cover. The graph and weights are preserved unchanged, and $|"IS"| + |"VC"| = |V|$ ensures optimality carries over.
][
  _Construction._ Given IS instance $(G, bold(w))$, create VC instance $(G, bold(w))$ with identical graph and weights.

  _Correctness._ ($arrow.r.double$) If $S$ is independent, no edge has both endpoints in $S$, so every edge has at least one endpoint in $V backslash S$, making $V backslash S$ a cover. ($arrow.l.double$) If $C$ is a vertex cover, every edge is incident to some vertex in $C$, so no edge connects two vertices of $V backslash C$, making $V backslash C$ independent.

  _Solution extraction._ For VC solution $C$, return $S = V backslash C$, i.e.\ flip each variable: $s_v = 1 - c_v$.
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
#let mc_sg_cut = mc_sg.source.instance.graph.inner.edges.filter(e => mc_sg_sol.source_config.at(e.at(0)) != mc_sg_sol.source_config.at(e.at(1))).len()
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

#reduction-rule("MaximumClique", "ILP")[
  A clique requires every pair of selected vertices to be adjacent; equivalently, no two selected vertices may share a _non_-edge. This is the independent set formulation on the complement graph $overline(G)$.
][
  _Construction._ Variables: $x_v in {0, 1}$ for each $v in V$. Constraints: $x_u + x_v <= 1$ for each $(u, v) in.not E$ (non-edges). Objective: maximize $sum_v x_v$.

  _Correctness._ ($arrow.r.double$) In a clique, every pair of selected vertices is adjacent, so no non-edge constraint is violated. ($arrow.l.double$) Any feasible solution selects only mutually adjacent vertices, forming a clique; the objective maximizes its size.

  _Solution extraction._ $K = {v : x_v = 1}$.
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
  The match-pair ILP formulation @blum2021 encodes subsequence alignment as a binary optimization. For two strings $s_1$ (length $n_1$) and $s_2$ (length $n_2$), each position pair $(j_1, j_2)$ where $s_1[j_1] = s_2[j_2]$ yields a binary variable. Constraints enforce one-to-one matching and order preservation (no crossings). The objective maximizes the number of matched pairs.
][
  _Construction._ Given strings $s_1$ and $s_2$:

  _Variables:_ Binary $m_(j_1, j_2) in {0, 1}$ for each $(j_1, j_2)$ with $s_1[j_1] = s_2[j_2]$. Interpretation: $m_(j_1, j_2) = 1$ iff position $j_1$ of $s_1$ is matched to position $j_2$ of $s_2$.

  _Constraints:_ (1) Each position in $s_1$ matched at most once: $sum_(j_2 : (j_1, j_2) in M) m_(j_1, j_2) lt.eq 1$ for all $j_1$. (2) Each position in $s_2$ matched at most once: $sum_(j_1 : (j_1, j_2) in M) m_(j_1, j_2) lt.eq 1$ for all $j_2$. (3) No crossings: for $(j_1, j_2), (j'_1, j'_2) in M$ with $j_1 < j'_1$ and $j_2 > j'_2$: $m_(j_1, j_2) + m_(j'_1, j'_2) lt.eq 1$.

  _Objective:_ Maximize $sum_((j_1, j_2) in M) m_(j_1, j_2)$.

  _Correctness._ ($arrow.r.double$) A common subsequence of length $ell$ defines $ell$ matched pairs that are order-preserving (no crossings) and one-to-one, yielding a feasible ILP solution with objective $ell$. ($arrow.l.double$) An ILP solution with objective $ell$ defines $ell$ matched pairs; constraints (1)--(2) ensure one-to-one matching, and constraint (3) ensures order preservation, so the matched characters form a common subsequence of length $ell$.

  _Solution extraction._ Collect pairs $(j_1, j_2)$ with $m_(j_1, j_2) = 1$, sort by $j_1$, and read the characters.
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
