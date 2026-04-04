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

#let metric-value(metric) = {
  if type(metric) == dictionary {
    if "Valid" in metric {
      metric.Valid
    } else if "value" in metric {
      metric.value
    } else {
      metric
    }
  } else {
    metric
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
  "AlgebraicEquationsOverGF2": [Algebraic Equations over GF(2)],
  "Betweenness": [Betweenness],
  "CyclicOrdering": [Cyclic Ordering],
  "AcyclicPartition": [Acyclic Partition],
  "MaximumIndependentSet": [Maximum Independent Set],
  "MaximumLeafSpanningTree": [Maximum Leaf Spanning Tree],
  "MinimumVertexCover": [Minimum Vertex Cover],
  "MaxCut": [Max-Cut],
  "GeneralizedHex": [Generalized Hex],
  "HamiltonianCircuit": [Hamiltonian Circuit],
  "BiconnectivityAugmentation": [Biconnectivity Augmentation],
  "HamiltonianPath": [Hamiltonian Path],
  "HamiltonianPathBetweenTwoVertices": [Hamiltonian Path Between Two Vertices],
  "BoundedDiameterSpanningTree": [Bounded Diameter Spanning Tree],
  "DegreeConstrainedSpanningTree": [Degree-Constrained Spanning Tree],
  "DirectedHamiltonianPath": [Directed Hamiltonian Path],
  "IntegralFlowBundles": [Integral Flow with Bundles],
  "LongestCircuit": [Longest Circuit],
  "LongestPath": [Longest Path],
  "ShortestWeightConstrainedPath": [Shortest Weight-Constrained Path],
  "UndirectedFlowLowerBounds": [Undirected Flow with Lower Bounds],
  "UndirectedTwoCommodityIntegralFlow": [Undirected Two-Commodity Integral Flow],
  "PathConstrainedNetworkFlow": [Path-Constrained Network Flow],
  "LengthBoundedDisjointPaths": [Length-Bounded Disjoint Paths],
  "IsomorphicSpanningTree": [Isomorphic Spanning Tree],
  "Kernel": [Kernel],
  "KthBestSpanningTree": [Kth Best Spanning Tree],
  "MaximumAchromaticNumber": [Maximum Achromatic Number],
  "KColoring": [$k$-Coloring],
  "KClique": [$k$-Clique],
  "MinimumCoveringByCliques": [Minimum Covering by Cliques],
  "MinimumIntersectionGraphBasis": [Minimum Intersection Graph Basis],
  "MinimumDominatingSet": [Minimum Dominating Set],
  "MinimumGeometricConnectedDominatingSet": [Minimum Geometric Connected Dominating Set],
  "MaximumMatching": [Maximum Matching],
  "MinimumMaximalMatching": [Minimum Maximal Matching],
  "BottleneckTravelingSalesman": [Bottleneck Traveling Salesman],
  "TravelingSalesman": [Traveling Salesman],
  "MaximumClique": [Maximum Clique],
  "MaximumSetPacking": [Maximum Set Packing],
  "MinimumHittingSet": [Minimum Hitting Set],
  "MinimumSetCovering": [Minimum Set Covering],
  "ComparativeContainment": [Comparative Containment],
  "SetBasis": [Set Basis],
  "SetSplitting": [Set Splitting],
  "MinimumCardinalityKey": [Minimum Cardinality Key],
  "SpinGlass": [Spin Glass],
  "QUBO": [QUBO],
  "ILP": [Integer Linear Programming],
  "IntegerKnapsack": [Integer Knapsack],
  "Knapsack": [Knapsack],
  "PartiallyOrderedKnapsack": [Partially Ordered Knapsack],
  "Satisfiability": [SAT],
  "NAESatisfiability": [NAE-SAT],
  "KSatisfiability": [$k$-SAT],
  "Maximum2Satisfiability": [Maximum 2-Satisfiability],
  "NonTautology": [Non-Tautology],
  "OneInThreeSatisfiability": [1-in-3 SAT],
  "Planar3Satisfiability": [Planar 3-SAT],
  "CircuitSAT": [CircuitSAT],
  "ConjunctiveQueryFoldability": [Conjunctive Query Foldability],
  "EnsembleComputation": [Ensemble Computation],
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
  "Clustering": [Clustering],
  "CapacityAssignment": [Capacity Assignment],
  "ConsistencyOfDatabaseFrequencyTables": [Consistency of Database Frequency Tables],
  "ClosestVectorProblem": [Closest Vector Problem],
  "ConsecutiveSets": [Consecutive Sets],
  "DisjointConnectingPaths": [Disjoint Connecting Paths],
  "MinimumMultiwayCut": [Minimum Multiway Cut],
  "OptimalLinearArrangement": [Optimal Linear Arrangement],
  "RootedTreeArrangement": [Rooted Tree Arrangement],
  "RuralPostman": [Rural Postman],
  "MixedChinesePostman": [Mixed Chinese Postman],
  "StackerCrane": [Stacker Crane],
  "LongestCommonSubsequence": [Longest Common Subsequence],
  "ExactCoverBy3Sets": [Exact Cover by 3-Sets],
  "ThreeDimensionalMatching": [Three-Dimensional Matching],
  "ThreeMatroidIntersection": [Three-Matroid Intersection],
  "SubsetProduct": [Subset Product],
  "SubsetSum": [Subset Sum],
  "CosineProductIntegration": [Cosine Product Integration],
  "Partition": [Partition],
  "ThreePartition": [3-Partition],
  "DynamicStorageAllocation": [Dynamic Storage Allocation],
  "Numerical3DimensionalMatching": [Numerical 3-Dimensional Matching],
  "NumericalMatchingWithTargetSums": [Numerical Matching with Target Sums],
  "PartialFeedbackEdgeSet": [Partial Feedback Edge Set],
  "MinimumFeedbackArcSet": [Minimum Feedback Arc Set],
  "MinimumFeedbackVertexSet": [Minimum Feedback Vertex Set],
  "ConjunctiveBooleanQuery": [Conjunctive Boolean Query],
  "ConsecutiveBlockMinimization": [Consecutive Block Minimization],
  "ConsecutiveOnesMatrixAugmentation": [Consecutive Ones Matrix Augmentation],
  "ConsecutiveOnesSubmatrix": [Consecutive Ones Submatrix],
  "FeasibleBasisExtension": [Feasible Basis Extension],
  "SparseMatrixCompression": [Sparse Matrix Compression],
  "MinimumMatrixCover": [Minimum Matrix Cover],
  "MinimumMatrixDomination": [Minimum Matrix Domination],
  "MinimumWeightDecoding": [Minimum Weight Decoding],
  "MinimumWeightSolutionToLinearEquations": [Minimum Weight Solution to Linear Equations],
  "DirectedTwoCommodityIntegralFlow": [Directed Two-Commodity Integral Flow],
  "MinimumEdgeCostFlow": [Minimum Edge-Cost Flow],
  "IntegralFlowHomologousArcs": [Integral Flow with Homologous Arcs],
  "IntegralFlowWithMultipliers": [Integral Flow With Multipliers],
  "MinMaxMulticenter": [Min-Max Multicenter],
  "FlowShopScheduling": [Flow Shop Scheduling],
  "JobShopScheduling": [Job-Shop Scheduling],
  "OpenShopScheduling": [Open Shop Scheduling],
  "GroupingBySwapping": [Grouping by Swapping],
  "IntegerExpressionMembership": [Integer Expression Membership],
  "MinimumCutIntoBoundedSets": [Minimum Cut Into Bounded Sets],
  "MinimumDummyActivitiesPert": [Minimum Dummy Activities in PERT Networks],
  "MinimumSumMulticenter": [Minimum Sum Multicenter],
  "MinimumTardinessSequencing": [Minimum Tardiness Sequencing],
  "MonochromaticTriangle": [Monochromatic Triangle],
  "MultipleChoiceBranching": [Multiple Choice Branching],
  "MultipleCopyFileAllocation": [Multiple Copy File Allocation],
  "ExpectedRetrievalCost": [Expected Retrieval Cost],
  "MultiprocessorScheduling": [Multiprocessor Scheduling],
  "NonLivenessFreePetriNet": [Non-Liveness Free Petri Net],
  "ProductionPlanning": [Production Planning],
  "PartitionIntoCliques": [Partition into Cliques],
  "PartitionIntoForests": [Partition into Forests],
  "PartitionIntoPerfectMatchings": [Partition into Perfect Matchings],
  "PartitionIntoPathsOfLength2": [Partition into Paths of Length 2],
  "PartitionIntoTriangles": [Partition Into Triangles],
  "PrecedenceConstrainedScheduling": [Precedence Constrained Scheduling],
  "PreemptiveScheduling": [Preemptive Scheduling],
  "PrimeAttributeName": [Prime Attribute Name],
  "QuadraticAssignment": [Quadratic Assignment],
  "EquilibriumPoint": [Equilibrium Point],
  "QuadraticCongruences": [Quadratic Congruences],
  "QuadraticDiophantineEquations": [Quadratic Diophantine Equations],
  "SimultaneousIncongruences": [Simultaneous Incongruences],
  "QuantifiedBooleanFormulas": [Quantified Boolean Formulas (QBF)],
  "RectilinearPictureCompression": [Rectilinear Picture Compression],
  "FeasibleRegisterAssignment": [Feasible Register Assignment],
  "MinimumRegisterSufficiencyForLoops": [Minimum Register Sufficiency for Loops],
  "MinimumCodeGenerationOneRegister": [Minimum Code Generation (One Register)],
  "MinimumCodeGenerationParallelAssignments": [Minimum Code Generation (Parallel Assignments)],
  "MaximumDomaticNumber": [Maximum Domatic Number],
  "MinimumCapacitatedSpanningTree": [Minimum Capacitated Spanning Tree],
  "MinimumDecisionTree": [Minimum Decision Tree],
  "MinimumDisjunctiveNormalForm": [Minimum Disjunctive Normal Form],
  "MinimumGraphBandwidth": [Minimum Graph Bandwidth],
  "MinimumMetricDimension": [Minimum Metric Dimension],
  "VertexCover": [Vertex Cover],
  "MinimumCodeGenerationUnlimitedRegisters": [Minimum Code Generation (Unlimited Registers)],
  "RegisterSufficiency": [Register Sufficiency],
  "ResourceConstrainedScheduling": [Resource Constrained Scheduling],
  "RootedTreeStorageAssignment": [Rooted Tree Storage Assignment],
  "SchedulingToMinimizeWeightedCompletionTime": [Scheduling to Minimize Weighted Completion Time],
  "SchedulingWithIndividualDeadlines": [Scheduling With Individual Deadlines],
  "SequencingToMinimizeMaximumCumulativeCost": [Sequencing to Minimize Maximum Cumulative Cost],
  "SequencingToMinimizeTardyTaskWeight": [Sequencing to Minimize Tardy Task Weight],
  "SequencingToMinimizeWeightedCompletionTime": [Sequencing to Minimize Weighted Completion Time],
  "SequencingToMinimizeWeightedTardiness": [Sequencing to Minimize Weighted Tardiness],
  "SequencingWithDeadlinesAndSetUpTimes": [Sequencing with Deadlines and Set-Up Times],
  "SequencingWithReleaseTimesAndDeadlines": [Sequencing with Release Times and Deadlines],
  "SequencingWithinIntervals": [Sequencing Within Intervals],
  "ShortestCommonSupersequence": [Shortest Common Supersequence],
  "StaffScheduling": [Staff Scheduling],
  "SteinerTree": [Steiner Tree],
  "SteinerTreeInGraphs": [Steiner Tree in Graphs],
  "MinimumAxiomSet": [Minimum Axiom Set],
  "MinimumExternalMacroDataCompression": [Minimum External Macro Data Compression],
  "MinimumInternalMacroDataCompression": [Minimum Internal Macro Data Compression],
  "MinimumFaultDetectionTestSet": [Minimum Fault Detection Test Set],
  "MinimumWeightAndOrGraph": [Minimum Weight AND/OR Graph],
  "StringToStringCorrection": [String-to-String Correction],
  "StrongConnectivityAugmentation": [Strong Connectivity Augmentation],
  "SubgraphIsomorphism": [Subgraph Isomorphism],
  "SumOfSquaresPartition": [Sum of Squares Partition],
  "TimetableDesign": [Timetable Design],
  "TwoDimensionalConsecutiveSets": [2-Dimensional Consecutive Sets],
  "KthLargestMTuple": [$K$th Largest $m$-Tuple],
  "MaximumLikelihoodRanking": [Maximum Likelihood Ranking],
  "OptimumCommunicationSpanningTree": [Optimum Communication Spanning Tree],
  "SquareTiling": [Square Tiling],
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

// Render a block of pred CLI commands for reproducibility
#let pred-commands(..cmds) = {
  block(
    width: 100%,
    fill: luma(245),
    inset: (x: 0.8em, y: 0.5em),
    radius: 3pt,
    stroke: 0.5pt + luma(200),
  )[
    #cmds.pos().map(c => raw("$ " + c)).join(linebreak())
  ]
}

// Format target problem spec for pred reduce --to (handles empty variant dicts)
#let target-spec(data) = {
  if data.target.variant.len() == 0 { data.target.problem }
  else { data.target.problem + "/" + data.target.variant.values().join("/") }
}

// Format a canonical example's problem spec for pred create --example
#let problem-spec(data) = {
  if data.variant.len() == 0 { data.problem }
  else { data.problem + "/" + data.variant.values().join("/") }
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
  #text(size: 11pt)[Jin-Guo Liu#super[1] #h(1em) Xi-Wei Pan#super[1] #h(1em) Shi-Wen An#super[2]]
  #v(0.3em)
  #text(size: 9pt)[#super[1]Hong Kong University of Science and Technology (Guangzhou) #h(1em) #super[2]Institute of Science Tokyo]
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
  let alpha = metric-value(sol.metric)
  [
    #problem-def("MaximumIndependentSet")[
      Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing $sum_(v in S) w(v)$ such that no two vertices in $S$ are adjacent: $forall u, v in S: (u, v) in.not E$.
    ][
    One of Karp's 21 NP-complete problems @karp1972, MIS appears in wireless network scheduling, register allocation, and coding theory @shannon1956. Solvable in polynomial time on bipartite graphs (König's theorem), interval graphs, chordal graphs, and cographs. The best known algorithm runs in $O^*(1.1996^n)$ time via measure-and-conquer branching @xiao2017. On geometric graphs (King's subgraph, triangular subgraph, unit disk graphs), MIS admits subexponential $O^*(c^sqrt(n))$ algorithms for some constant $c$, via geometric separation @alber2004.

    *Example.* Consider the Petersen graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, and unit weights $w(v) = 1$ for all $v in V$. The graph is 3-regular (every vertex has degree 3). A maximum independent set is $S = {#S.map(i => $v_#i$).join(", ")}$ with $w(S) = sum_(v in S) w(v) = #alpha = alpha(G)$. No two vertices in $S$ share an edge, and no vertex can be added without violating independence.

    #pred-commands(
      "pred create --example MIS -o mis.json",
      "pred solve mis.json",
      "pred evaluate mis.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let wS = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example MVC -o mvc.json",
      "pred solve mvc.json",
      "pred evaluate mvc.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let x = load-model-example("VertexCover")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let k = x.instance.k
  let sol = x.optimal_config
  let cover = sol.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("VertexCover")[
      Given an undirected graph $G = (V, E)$ and a positive integer $k <= |V|$, determine whether there exists a vertex cover of size at most $k$: a subset $V' subset.eq V$ with $|V'| <= k$ such that for each edge ${u, v} in E$, at least one of $u, v$ belongs to $V'$.
    ][
    Vertex Cover is one of Karp's 21 NP-complete problems @karp1972 and the decision version of Minimum Vertex Cover @garey1979. The best known exact algorithm runs in $O^*(1.1996^n)$ time (Chen, Kanj, and Xia, 2010).

    *Example.* Consider a graph on $n = #nv$ vertices and $|E| = #ne$ edges with threshold $k = #k$. The cover $V' = {#cover.map(i => $v_#i$).join(", ")}$ with $|V'| = #cover.len() <= k$ is a valid vertex cover.

    #pred-commands(
      "pred create --example VertexCover -o vc.json",
      "pred solve vc.json",
      "pred evaluate vc.json --config " + sol.map(str).join(","),
    )
    ]
  ]
}

#{
  let x = load-model-example("MaxCut", variant: (graph: "SimpleGraph", weight: "i32"))
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  // Pick optimal config = S={v0, v3} to match figure
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let side-s = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let side-sbar = sol.config.enumerate().filter(((i, v)) => v == 0).map(((i, _)) => i)
  let cut-val = metric-value(sol.metric)
  let cut-edges = edges.filter(e => side-s.contains(e.at(0)) != side-s.contains(e.at(1)))
  let uncut-edges = edges.filter(e => side-s.contains(e.at(0)) == side-s.contains(e.at(1)))
  [
    #problem-def("MaxCut")[
      Given $G = (V, E)$ with weights $w: E -> RR$, find partition $(S, overline(S))$ maximizing $sum_((u,v) in E: u in S, v in overline(S)) w(u, v)$.
    ][
    Max-Cut is NP-hard on general graphs @barahona1982 but polynomial-time solvable on planar graphs. The Goemans-Williamson SDP relaxation achieves a 0.878-approximation ratio @goemans1995, which is optimal assuming the Unique Games Conjecture. The best known exact algorithm runs in $O^*(2^(omega n slash 3))$ time via algebraic 2-CSP techniques @williams2005, where $omega < 2.372$ is the matrix multiplication exponent.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, and unit weights $w(e) = 1$. The partition $S = {#side-s.map(i => $v_#i$).join(", ")}$, $overline(S) = {#side-sbar.map(i => $v_#i$).join(", ")}$ cuts #cut-val of #ne edges: #cut-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", "). #if uncut-edges.len() == 1 [Only the edge #uncut-edges.map(((u, v)) => $(v_#u, v_#v)$).at(0) is uncut (both endpoints in $overline(S)$).] #if uncut-edges.len() > 1 [The edges #uncut-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ") are uncut.] The cut value is $sum w(e) = #cut-val$.

    #pred-commands(
      "pred create --example MaxCut -o maxcut.json",
      "pred solve maxcut.json",
      "pred evaluate maxcut.json --config " + x.optimal_config.map(str).join(","),
    )

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
#problem-def("MinimumCutIntoBoundedSets")[
  Given an undirected graph $G = (V, E)$ with edge weights $w: E -> ZZ^+$, designated vertices $s, t in V$, and a positive integer $B <= |V|$, find a partition of $V$ into disjoint sets $V_1$ and $V_2$ such that $s in V_1$, $t in V_2$, $|V_1| <= B$, $|V_2| <= B$, that minimizes the total cut weight
  $ sum_({u,v} in E: u in V_1, v in V_2) w({u,v}). $
][
Minimum Cut Into Bounded Sets (Garey & Johnson ND17) combines the classical minimum $s$-$t$ cut problem with a balance constraint on partition sizes. Without the balance constraint ($B = |V|$), the problem reduces to standard minimum $s$-$t$ cut, solvable in polynomial time via network flow. Adding the requirement $|V_1| <= B$ and $|V_2| <= B$ makes the problem NP-complete; it remains NP-complete even for $B = |V| slash 2$ and unit edge weights (the minimum bisection problem) @garey1976. Applications include VLSI layout, load balancing, and graph bisection.

The best known exact algorithm is brute-force enumeration of all $2^n$ vertex partitions in $O(2^n)$ time. For the special case of minimum bisection, Cygan et al. @cygan2014 showed fixed-parameter tractability with respect to the cut size. No polynomial-time finite approximation factor exists for balanced graph partition unless $P = N P$ (Andreev and Racke, 2006). Arora, Rao, and Vazirani @arora2009 gave an $O(sqrt(log n))$-approximation for balanced separator.

*Example.* Consider $G$ with 4 vertices and edges $(v_0, v_1)$, $(v_1, v_2)$, $(v_2, v_3)$ with unit weights, $s = v_0$, $t = v_3$, $B = 3$. The optimal partition $V_1 = {v_0, v_1}$, $V_2 = {v_2, v_3}$ gives minimum cut weight $w({v_1, v_2}) = 1$. Both $|V_1| = 2 <= 3$ and $|V_2| = 2 <= 3$.
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

      #pred-commands(
        "pred create --example HC -o hc.json",
        "pred solve hc.json",
        "pred evaluate hc.json --config " + x.optimal_config.map(str).join(","),
      )

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

#{
  let x = load-model-example("LongestCircuit")
  let nv = x.instance.graph.num_vertices
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let ne = edges.len()
  let edge-lengths = x.instance.edge_lengths
  let config = x.optimal_config
  let selected = range(ne).filter(i => config.at(i) == 1)
  let total-length = selected.map(i => edge-lengths.at(i)).sum()
  let cycle-order = (0, 1, 4, 5, 2, 3)
  [
    #problem-def("LongestCircuit")[
      Given an undirected graph $G = (V, E)$ with positive edge lengths $l: E -> ZZ^+$, find a simple circuit $C subset.eq E$ that maximizes $sum_(e in C) l(e)$.
    ][
      Longest Circuit is the optimization version of the classical longest-cycle problem. Hamiltonian Circuit is the special case where every edge has unit length and the optimum equals $|V|$, so Longest Circuit is NP-hard via Karp's original Hamiltonicity result @karp1972. A standard exact baseline uses Held--Karp-style subset dynamic programming in $O(n^2 dot 2^n)$ time @heldkarp1962; unlike Hamiltonicity, the goal here is to find the longest simple cycle rather than specifically a spanning one.

      In the implementation, a configuration selects a subset of edges. A witness is a configuration whose selected edges induce one connected 2-regular subgraph; the objective value is the total selected length.

      *Example.* Consider the canonical 6-vertex instance. The optimal circuit $v_0 arrow v_1 arrow v_4 arrow v_5 arrow v_2 arrow v_3 arrow v_0$ uses edge lengths $3 + 2 + 5 + 1 + 4 + 3 = #total-length$, which is the maximum circuit length. The remaining edges are available but yield shorter circuits.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o longest-circuit.json",
        "pred solve longest-circuit.json",
        "pred evaluate longest-circuit.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (
            selected: graph-colors.at(0),
            unused: luma(200),
          )
          let r = 1.5
          let positions = range(nv).map(i => {
            let angle = 90deg - i * 360deg / nv
            (calc.cos(angle) * r, calc.sin(angle) * r)
          })

          for (ei, (u, v)) in edges.enumerate() {
            let is-selected = config.at(ei) == 1
            let col = if is-selected { colors.selected } else { colors.unused }
            let thickness = if is-selected { 1.3pt } else { 0.5pt }
            let dash = if is-selected { "solid" } else { "dashed" }
            line(positions.at(u), positions.at(v), stroke: (paint: col, thickness: thickness, dash: dash))

            let mid = (
              (positions.at(u).at(0) + positions.at(v).at(0)) / 2,
              (positions.at(u).at(1) + positions.at(v).at(1)) / 2,
            )
            let dx = if ei == 6 { -0.28 } else if ei == 7 { 0.24 } else if ei == 8 { -0.24 } else if ei == 9 { 0.24 } else { 0 }
            let dy = if ei == 6 { 0 } else if ei == 7 { 0.18 } else if ei == 8 { 0.18 } else if ei == 9 { -0.15 } else { 0 }
            content(
              (mid.at(0) + dx, mid.at(1) + dy),
              text(6pt, fill: col)[#edge-lengths.at(ei)],
              fill: white,
              frame: "rect",
              padding: 0.05,
              stroke: none,
            )
          }

          for (i, pos) in positions.enumerate() {
            circle(pos, radius: 0.18, fill: white, stroke: 0.7pt + black)
            content(pos, text(7pt)[$v_#i$])
          }
        }),
        caption: [Longest Circuit instance on #nv vertices. The highlighted cycle $#cycle-order.map(v => $v_#v$).join($arrow$) arrow v_#(cycle-order.at(0))$ has maximum total length #total-length; the remaining edges yield shorter circuits.],
      ) <fig:longest-circuit>
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
  let M = x.instance.max_paths
  let K = x.instance.max_length
  let chosen-verts = (0, 1, 2, 3, 4)
  let chosen-edges = ((0, 1), (1, 4), (0, 2), (2, 4), (0, 3), (3, 4))
  [
    #problem-def("LengthBoundedDisjointPaths")[
      Given an undirected graph $G = (V, E)$, distinct terminals $s, t in V$, and a positive integer $K$, maximize the number of pairwise internally vertex-disjoint paths from $s$ to $t$, each using at most $K$ edges.
    ][
      Length-Bounded Disjoint Paths is the bounded-routing version of the classical disjoint-path problem, with applications in network routing and VLSI where multiple connections must fit simultaneously under quality-of-service limits. Garey & Johnson list it as ND41 and summarize the sharp threshold proved by Itai, Perl, and Shiloach: the problem is NP-complete for every fixed $K >= 5$, polynomial-time solvable for $K <= 4$, and becomes polynomial again when the length bound is removed entirely @garey1979. The implementation here uses $M dot |V|$ binary variables where $M = min(deg(s), deg(t))$ is an upper bound on the number of vertex-disjoint $s$-$t$ paths, so brute-force search over configurations runs in $O^*(2^(M dot |V|))$.

      *Example.* Consider the graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, terminals $s = v_#s$, $t = v_#t$, and $K = #K$. Here $M = #M$ path slots are available. The three paths $P_1 = v_0 arrow v_1 arrow v_4$, $P_2 = v_0 arrow v_2 arrow v_4$, and $P_3 = v_0 arrow v_3 arrow v_4$ are each of length 2 (at most $K = 3$), and their internal vertex sets ${v_1}$, ${v_2}$, and ${v_3}$ are pairwise disjoint. The optimal value is therefore $3$.

      #pred-commands(
        "pred create --example LengthBoundedDisjointPaths -o length-bounded-disjoint-paths.json",
        "pred solve length-bounded-disjoint-paths.json",
        "pred evaluate length-bounded-disjoint-paths.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          let blue = graph-colors.at(0)
          let gray = luma(180)
          let verts = (
            (0, 1),     // v0 = s
            (1.5, 1.8), // v1
            (1.5, 1.0), // v2
            (1.5, 0.2), // v3
            (3.0, 1),   // v4 = t
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
        caption: [An optimal Length-Bounded Disjoint Paths instance with $s = v_0$, $t = v_4$, and $K = 3$. All three vertex-disjoint paths $v_0 arrow v_1 arrow v_4$, $v_0 arrow v_2 arrow v_4$, and $v_0 arrow v_3 arrow v_4$ are highlighted, giving an optimal value of $3$.],
      ) <fig:length-bounded-disjoint-paths>
    ]
  ]
}
#{
  let x = load-model-example("DisjointConnectingPaths")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let chosen-edges = ((0, 1), (1, 3), (2, 4), (4, 5))
  [
    #problem-def("DisjointConnectingPaths")[
      Given an undirected graph $G = (V, E)$ and pairwise disjoint terminal pairs $(s_1, t_1), dots, (s_k, t_k)$, determine whether $G$ contains $k$ mutually vertex-disjoint paths such that path $P_i$ joins $s_i$ to $t_i$ for every $i$.
    ][
      Disjoint Connecting Paths is the classical routing form of the vertex-disjoint paths problem, catalogued as ND40 in Garey & Johnson @garey1979. When the number of terminal pairs $k$ is part of the input, the problem is NP-complete @karp1972. In contrast, for every fixed $k$, Robertson and Seymour give an $O(n^3)$ algorithm @robertsonSeymour1995, and Kawarabayashi, Kobayashi, and Reed later improve the dependence on $n$ to $O(n^2)$ @kawarabayashiKobayashiReed2012. The implementation in this crate uses one binary variable per undirected edge, so brute-force search explores an $O^*(2^|E|)$ configuration space.#footnote[This is the exact-search bound induced by the edge-subset encoding implemented in the codebase; no sharper general exact worst-case bound is claimed here.]

      *Example.* Consider the repaired YES instance with $n = #nv$ vertices, $|E| = #ne$ edges, and terminal pairs $(v_0, v_3)$ and $(v_2, v_5)$. Selecting the edges $v_0v_1$, $v_1v_3$, $v_2v_4$, and $v_4v_5$ yields the two vertex-disjoint paths $v_0 arrow v_1 arrow v_3$ and $v_2 arrow v_4 arrow v_5$, so the instance is satisfying.

      #pred-commands(
        "pred create --example DisjointConnectingPaths -o disjoint-connecting-paths.json",
        "pred solve disjoint-connecting-paths.json",
        "pred evaluate disjoint-connecting-paths.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          let blue = graph-colors.at(0)
          let gray = luma(180)
          let verts = (
            (0, 1.2),
            (1.4, 1.2),
            (0, 0),
            (2.8, 1.2),
            (1.4, 0),
            (2.8, 0),
          )
          let edges = ((0, 1), (1, 3), (0, 2), (1, 4), (2, 4), (3, 5), (4, 5))
          for (u, v) in edges {
            let selected = chosen-edges.any(e =>
              (e.at(0) == u and e.at(1) == v) or (e.at(0) == v and e.at(1) == u)
            )
            g-edge(verts.at(u), verts.at(v),
              stroke: if selected { 2pt + blue } else { 1pt + gray })
          }
          for (k, pos) in verts.enumerate() {
            let terminal = k == 0 or k == 2 or k == 3 or k == 5
            g-node(pos, name: "v" + str(k),
              fill: if terminal { blue } else { white },
              label: if terminal {
                text(fill: white)[
                  #if k == 0 { $s_1$ }
                  else if k == 3 { $t_1$ }
                  else if k == 2 { $s_2$ }
                  else { $t_2$ }
                ]
              } else [
                $v_#k$
              ])
          }
        }),
        caption: [A satisfying Disjoint Connecting Paths instance with terminal pairs $(v_0, v_3)$ and $(v_2, v_5)$. The highlighted edges form the vertex-disjoint paths $v_0 arrow v_1 arrow v_3$ and $v_2 arrow v_4 arrow v_5$.],
      ) <fig:disjoint-connecting-paths>
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

      #pred-commands(
        "pred create --example GeneralizedHex -o generalized-hex.json",
        "pred solve generalized-hex.json",
        "pred evaluate generalized-hex.json --config " + x.optimal_config.map(str).join(","),
      )

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

      #pred-commands(
        "pred create --example HamiltonianPath -o hamiltonian-path.json",
        "pred solve hamiltonian-path.json",
        "pred evaluate hamiltonian-path.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("HamiltonianPathBetweenTwoVertices")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let s = x.instance.source_vertex
  let t = x.instance.target_vertex
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let path = sol.config
  let path-edges = range(path.len() - 1).map(i => (path.at(i), path.at(i + 1)))
  [
    #problem-def("HamiltonianPathBetweenTwoVertices")[
      Given a graph $G = (V, E)$ and two distinguished vertices $s, t in V$, determine whether $G$ contains a _Hamiltonian s–t path_, i.e., a simple path that begins at $s$, ends at $t$, and visits every vertex of $G$ exactly once.
    ][
      A classical NP-complete decision problem from Garey & Johnson (GT39, p. 60), closely related to _Hamiltonian Path_. Fixing the two endpoints of a Hamiltonian path gives a natural and useful specialisation: any Hamiltonian circuit can be converted into a Hamiltonian s–t path by removing the edge $(s, t)$, and conversely a Hamiltonian s–t path plus the edge $(s, t)$ yields a Hamiltonian circuit. The problem remains NP-complete for planar graphs and bipartite graphs.

      The best known exact algorithm is Björklund's randomized $O^*(1.657^n)$ "Determinant Sums" method @bjorklund2014. The classical Held--Karp dynamic programming algorithm solves it in $O(n^2 dot 2^n)$ deterministic time by initialising the DP only at $s$ and accepting only solutions that terminate at $t$ @heldkarp1962.

      Variables: $n = |V|$ values forming a permutation. Position $i$ holds the vertex visited at step $i$. A configuration is satisfying when it forms a valid permutation of all vertices, the first element is $s$, the last element is $t$, and consecutive vertices are adjacent in $G$.

      *Example.* Consider the graph $G$ on #nv vertices with edges ${#edges.map(((u, v)) => $\{#u, #v\}$).join(", ")}$, source $s = #s$, and target $t = #t$. The sequence $[#path.map(v => str(v)).join(", ")]$ is a Hamiltonian $s$–$t$ path: it starts at $s$, ends at $t$, visits every vertex exactly once, and each consecutive pair is adjacent --- #path-edges.map(((u, v)) => $\{#u, #v\}$).join($,$) $in E$.

      #pred-commands(
        "pred create --example HamiltonianPathBetweenTwoVertices -o hpbtv.json",
        "pred solve hpbtv.json",
        "pred evaluate hpbtv.json --config " + x.optimal_config.map(str).join(","),
      )

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
            let is-endpoint = k == s or k == t
            g-node(pos, name: "v" + str(k),
              fill: if is-endpoint { graph-colors.at(1) } else { blue },
              label: text(fill: white)[$v_#k$])
          }
        })
      },
      caption: [Hamiltonian $s$–$t$ path in a #{nv}-vertex graph with $s = v_#s$ and $t = v_#t$ (orange). Blue edges show the path $#path.map(v => $v_#v$).join($arrow$)$.],
      ) <fig:hamiltonian-path-between-two-vertices>
    ]
  ]
}
#{
  let x = load-model-example("DirectedHamiltonianPath")
  let nv = x.instance.graph.num_vertices
  let arcs = x.instance.graph.arcs
  [
    #problem-def("DirectedHamiltonianPath")[
      Given a directed graph $G = (V, A)$, determine whether $G$ contains a _directed Hamiltonian path_, i.e., a simple directed path that visits every vertex exactly once following arc directions.
    ][
      A classical NP-complete decision problem from Garey & Johnson (A2.1 GT39). The directed version is NP-complete even for tournaments and remains hard for most restricted digraph classes.

      The best known exact algorithm runs in $O(n^2 dot 2^n)$ time using Held--Karp style dynamic programming with bitmask DP.

      Variables: A permutation of the $n$ vertices, encoded as a Lehmer code with $"dims" = [n, n-1, dots, 1]$. A configuration is satisfying when every consecutive pair in the decoded permutation forms a directed arc.

      *Example.* Consider the directed graph $G$ on #nv vertices with arcs ${#arcs.map(((u, v)) => $(#u arrow.r #v)$).join(", ")}$. The directed Hamiltonian path $0 arrow.r 1 arrow.r 3 arrow.r 2 arrow.r 4 arrow.r 5$ visits every vertex exactly once with all consecutive pairs being arcs.

      #pred-commands(
        "pred create --example DirectedHamiltonianPath -o dhp.json",
        "pred solve dhp.json",
      )
    ]
  ]
}
#{
  let x = load-model-example("Kernel")
  let nv = x.instance.graph.num_vertices
  let arcs = x.instance.graph.arcs
  [
    #problem-def("Kernel")[
      Given a directed graph $G = (V, A)$, find a _kernel_ $V' subset.eq V$ such that (1) $V'$ is _independent_ — no arc joins any two vertices in $V'$ — and (2) $V'$ is _absorbing_ — every vertex $u in.not V'$ has an arc $(u, v) in A$ for some $v in V'$.
    ][
      A classical graph-theoretic concept introduced by von Neumann and Morgenstern (1944) in the context of game theory. Deciding whether a directed graph has a kernel is NP-complete in general, though every DAG has a unique kernel. Kernels appear in combinatorial game theory, graph coloring (Galvin's theorem), and stable set problems on digraphs.

      Variables: A binary vector of length $|V|$, where $x_v = 1$ iff vertex $v$ is in the kernel.

      *Example.* Consider the directed graph $G$ on #nv vertices with arcs ${#arcs.map(((u, v)) => $(#u arrow.r #v)$).join(", ")}$. The kernel $V' = {0, 3}$ is independent (no arc between 0 and 3) and absorbing (vertex 1 has arc to 3, vertex 2 has arc to 3, vertex 4 has arc to 0).

      #pred-commands(
        "pred create --example Kernel -o kernel.json",
        "pred solve kernel.json",
      )
    ]
  ]
}
#{
  let x = load-model-example("LongestPath")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges
  let lengths = x.instance.edge_lengths
  let s = x.instance.source_vertex
  let t = x.instance.target_vertex
  let path-config = x.optimal_config
  let path-order = (0, 1, 3, 2, 4, 5, 6)
  let path-edges = edges.enumerate().filter(((idx, _)) => path-config.at(idx) == 1).map(((idx, e)) => e)
  [
    #problem-def("LongestPath")[
      Given an undirected graph $G = (V, E)$ with positive edge lengths $l: E -> ZZ^+$ and designated vertices $s, t in V$, find a simple path $P$ from $s$ to $t$ maximizing $sum_(e in P) l(e)$.
    ][
      Longest Path is problem ND29 in Garey & Johnson @garey1979. It bridges weighted routing and Hamiltonicity: when every edge has unit length, the optimum reaches $|V| - 1$ exactly when there is a Hamiltonian path from $s$ to $t$. The implementation catalog records the classical subset-DP exact bound $O(|V| dot 2^|V|)$, in the style of Held--Karp dynamic programming @heldkarp1962. For the parameterized $k$-path version, color-coding gives randomized $2^(O(k)) |V|^(O(1))$ algorithms @alon1995.

      Variables: one binary value per edge. A configuration is valid exactly when the selected edges form a single simple $s$-$t$ path; otherwise the metric is `Invalid`. For valid selections, the metric is the total selected edge length.

      *Example.* Consider the graph on #nv vertices with source $s = v_#s$ and target $t = v_#t$. The highlighted path $#path-order.map(v => $v_#v$).join($arrow$)$ uses edges ${#path-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$, so its total length is $3 + 4 + 1 + 5 + 3 + 4 = 20$. Another valid path, $v_0 arrow v_2 arrow v_4 arrow v_5 arrow v_3 arrow v_1 arrow v_6$, has total length $17$, so the highlighted path is strictly better.

      #pred-commands(
        "pred create --example LongestPath -o longest-path.json",
        "pred solve longest-path.json",
        "pred evaluate longest-path.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure({
        let blue = graph-colors.at(0)
        let gray = luma(200)
        let verts = ((0, 1.2), (1.2, 2.0), (1.2, 0.4), (2.5, 2.0), (2.5, 0.4), (3.8, 1.2), (5.0, 1.2))
        canvas(length: 1cm, {
          import draw: *
          for (idx, (u, v)) in edges.enumerate() {
            let on-path = path-config.at(idx) == 1
            g-edge(verts.at(u), verts.at(v), stroke: if on-path { 2pt + blue } else { 1pt + gray })
            let mx = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
            let my = (verts.at(u).at(1) + verts.at(v).at(1)) / 2
            let dx = if idx == 0 or idx == 2 { 0 } else if idx == 1 or idx == 4 { -0.18 } else if idx == 5 or idx == 6 { 0.18 } else if idx == 8 { 0 } else { 0.16 }
            let dy = if idx == 0 or idx == 2 or idx == 5 or idx == 8 { 0.18 } else if idx == 1 or idx == 4 or idx == 6 { -0.18 } else if idx == 3 { 0 } else { 0.16 }
            draw.content(
              (mx + dx, my + dy),
              text(7pt, fill: luma(80))[#str(int(lengths.at(idx)))]
            )
          }
          for (k, pos) in verts.enumerate() {
            let on-path = path-order.any(v => v == k)
            g-node(pos, name: "v" + str(k),
              fill: if on-path { blue } else { white },
              label: if on-path { text(fill: white)[$v_#k$] } else { [$v_#k$] })
          }
          content((0, 1.55), text(8pt)[$s$])
          content((5.0, 1.55), text(8pt)[$t$])
        })
      },
      caption: [Longest Path instance with edge lengths shown on the edges. The highlighted path from $s = v_0$ to $t = v_6$ has total length 20.],
      ) <fig:longest-path>
    ]
  ]
}
#{
  let x = load-model-example("UndirectedFlowLowerBounds")
  let s = x.instance.source
  let t = x.instance.sink
  let R = x.instance.requirement
  let orientation = x.optimal_config
  let edges = x.instance.graph.edges
  let lower = x.instance.lower_bounds
  let caps = x.instance.capacities
  let witness = (2, 1, 1, 1, 1, 2, 1)
  [
    #problem-def("UndirectedFlowLowerBounds")[
      Given an undirected graph $G = (V, E)$, specified vertices $s, t in V$, lower bounds $l: E -> ZZ_(>= 0)$, upper capacities $c: E -> ZZ^+$ with $l(e) <= c(e)$ for every edge, and a requirement $R in ZZ^+$, determine whether there exists a flow function $f: {(u, v), (v, u): {u, v} in E} -> ZZ_(>= 0)$ such that each edge carries flow in at most one direction, every edge value lies between its lower and upper bound, flow is conserved at every vertex in $V backslash {s, t}$, and the net flow into $t$ is at least $R$.
    ][
      Undirected Flow with Lower Bounds appears as ND37 in Garey and Johnson's catalog @garey1979. Itai proved that even this single-commodity undirected feasibility problem is NP-complete, contrasting sharply with the directed lower-bounded case, which reduces to ordinary max-flow machinery @itai1978.

      The implementation exposes one binary decision per edge rather than raw flow magnitudes. The configuration $(#orientation.map(str).join(", "))$ means "orient every edge exactly as listed in the stored edge order"; once an orientation is fixed, `evaluate()` checks the remaining lower-bounded directed circulation conditions internally. This keeps the explicit search space at $2^m$ for $m = |E|$, matching the registry complexity bound.

      *Example.* The canonical fixture uses source $s = v_#s$, sink $t = v_#t$, requirement $R = #R$, edges ${#edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$, and lower/upper pairs ${#range(edges.len()).map(i => $(#lower.at(i), #caps.at(i))$).join(", ")}$ in that order. Under the all-zero orientation config, a feasible witness sends flows $(#witness.map(str).join(", "))$ along those edges respectively: $2$ on $(v_0, v_1)$, $1$ on $(v_0, v_2)$, $1$ on $(v_1, v_3)$, $1$ on $(v_2, v_3)$, $1$ on $(v_1, v_4)$, $2$ on $(v_3, v_5)$, and $1$ on $(v_4, v_5)$. Every lower bound is satisfied, each nonterminal vertex has equal inflow and outflow, and the sink receives $2 + 1 = 3 >= R$, so the instance evaluates to true. A separate rule issue tracks the natural reduction to ILP; this model PR only documents the standalone verifier.

      #pred-commands(
        "pred create --example UndirectedFlowLowerBounds -o undirected-flow-lower-bounds.json",
        "pred solve undirected-flow-lower-bounds.json",
        "pred evaluate undirected-flow-lower-bounds.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 0.9cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let red = rgb("#e15759")
          let gray = luma(190)
          let verts = ((0, 0), (1.6, 1.2), (1.6, -1.2), (3.4, 0.5), (3.4, -1.5), (5.2, -0.3))
          let labels = (
            [$s = v_0$],
            [$v_1$],
            [$v_2$],
            [$v_3$],
            [$v_4$],
            [$t = v_5$],
          )
          for (u, v) in edges {
            g-edge(verts.at(u), verts.at(v), stroke: 1.8pt + blue)
          }
          for (i, pos) in verts.enumerate() {
            let fill = if i == s { blue } else if i == t { red } else { white }
            let label = if i == s or i == t { text(fill: white)[#labels.at(i)] } else { labels.at(i) }
            g-node(pos, name: "uflb-" + str(i), fill: fill, label: label)
          }
          content((0.75, 0.7), text(7pt, fill: gray)[$f = 2$])
          content((0.75, -0.7), text(7pt, fill: gray)[$f = 1$])
          content((2.45, 1.05), text(7pt, fill: gray)[$f = 1$])
          content((2.45, -0.25), text(7pt, fill: gray)[$f = 1$])
          content((2.45, -1.45), text(7pt, fill: gray)[$f = 1$])
          content((4.35, 0.35), text(7pt, fill: gray)[$f = 2$])
          content((4.35, -1.1), text(7pt, fill: gray)[$f = 1$])
        }),
        caption: [Canonical YES instance for Undirected Flow with Lower Bounds. Blue edges follow the all-zero orientation config, and edge labels show one feasible witness flow.],
      ) <fig:undirected-flow-lower-bounds>
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

      #pred-commands(
        "pred create --example UndirectedTwoCommodityIntegralFlow -o undirected-two-commodity-integral-flow.json",
        "pred solve undirected-two-commodity-integral-flow.json",
        "pred evaluate undirected-two-commodity-integral-flow.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("PathConstrainedNetworkFlow")
  let arcs = x.instance.graph.arcs.map(a => (a.at(0), a.at(1)))
  let requirement = x.instance.requirement
  let p1 = (0, 2, 5, 8)
  let p2 = (0, 3, 6, 8)
  let p5 = (1, 4, 7, 9)
  [
    #problem-def("PathConstrainedNetworkFlow")[
      Given a directed graph $G = (V, A)$, designated vertices $s, t in V$, arc capacities $c: A -> ZZ^+$, a prescribed collection $cal(P)$ of directed simple $s$-$t$ paths, and a requirement $R in ZZ^+$, determine whether there exists an integral path-flow function $g: cal(P) -> ZZ_(>= 0)$ such that $sum_(p in cal(P): a in p) g(p) <= c(a)$ for every arc $a in A$ and $sum_(p in cal(P)) g(p) >= R$.
    ][
      Path-Constrained Network Flow appears as problem ND34 in Garey \& Johnson @garey1979. Unlike ordinary single-commodity flow, the admissible routes are fixed in advance: every unit of flow must be assigned to one of the listed $s$-$t$ paths. This prescribed-path viewpoint is standard in line planning and unsplittable routing, and Büsing and Stiller give a modern published NP-completeness and inapproximability treatment for exactly this integral formulation @busingstiller2011.

      The implementation uses one integer variable per prescribed path, bounded by that path's bottleneck capacity. Exhaustive search over those path-flow variables gives the registered worst-case bound $O^*((C + 1)^(|cal(P)|))$, where $C = max_(a in A) c(a)$. #footnote[This is the brute-force bound induced by the representation used in the library; no sharper general exact algorithm is claimed here for the integral prescribed-path formulation.]

      *Example.* The canonical fixture uses the directed network with arcs $(0,1)$, $(0,2)$, $(1,3)$, $(1,4)$, $(2,4)$, $(3,5)$, $(4,5)$, $(4,6)$, $(5,7)$, and $(6,7)$, capacities $(2,1,1,1,1,1,1,1,2,1)$, source $s = 0$, sink $t = 7$, and required flow $R = #requirement$. The prescribed paths are $p_1 = 0 arrow 1 arrow 3 arrow 5 arrow 7$, $p_2 = 0 arrow 1 arrow 4 arrow 5 arrow 7$, $p_3 = 0 arrow 1 arrow 4 arrow 6 arrow 7$, $p_4 = 0 arrow 2 arrow 4 arrow 5 arrow 7$, and $p_5 = 0 arrow 2 arrow 4 arrow 6 arrow 7$. The fixture's satisfying configuration is $g = (#x.optimal_config.at(0), #x.optimal_config.at(1), #x.optimal_config.at(2), #x.optimal_config.at(3), #x.optimal_config.at(4)) = (1, 1, 0, 0, 1)$, so one unit is sent along $p_1$, one along $p_2$, and one along $p_5$. The shared arcs $(0,1)$ and $(5,7)$ each carry exactly two units of flow, matching their capacity 2, while every other used arc carries one unit. Therefore the total flow into $t$ is $3 = R$, so the instance is feasible.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o path-constrained-network-flow.json",
        "pred solve path-constrained-network-flow.json --solver brute-force",
        "pred evaluate path-constrained-network-flow.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 0.95cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let orange = rgb("#f28e2b")
          let teal = rgb("#76b7b2")
          let gray = luma(185)
          let verts = (
            (0, 0),
            (1.4, 1.2),
            (1.4, -1.2),
            (2.8, 1.9),
            (2.8, 0),
            (4.2, 1.2),
            (4.2, -1.2),
            (5.6, 0),
          )
          for (u, v) in arcs {
            line(
              verts.at(u),
              verts.at(v),
              stroke: 0.8pt + gray,
              mark: (end: "straight", scale: 0.45),
            )
          }
          for idx in p1 {
            let (u, v) = arcs.at(idx)
            line(
              verts.at(u),
              verts.at(v),
              stroke: 1.8pt + blue,
              mark: (end: "straight", scale: 0.5),
            )
          }
          for idx in p2 {
            let (u, v) = arcs.at(idx)
            line(
              verts.at(u),
              verts.at(v),
              stroke: (paint: orange, thickness: 1.7pt, dash: "dashed"),
              mark: (end: "straight", scale: 0.48),
            )
          }
          for idx in p5 {
            let (u, v) = arcs.at(idx)
            line(
              verts.at(u),
              verts.at(v),
              stroke: 1.6pt + teal,
              mark: (end: "straight", scale: 0.46),
            )
          }
          for (i, pos) in verts.enumerate() {
            let fill = if i == 0 or i == 7 { rgb("#e15759").lighten(75%) } else { white }
            g-node(pos, name: "pcnf-" + str(i), fill: fill, label: [$v_#i$])
          }
          content((0.65, 0.78), text(8pt, fill: gray)[$2 / 2$])
          content((4.9, 0.78), text(8pt, fill: gray)[$2 / 2$])
          line((0.2, -2.15), (0.8, -2.15), stroke: 1.8pt + blue, mark: (end: "straight", scale: 0.42))
          content((1.15, -2.15), text(8pt)[$p_1$])
          line((1.95, -2.15), (2.55, -2.15), stroke: (paint: orange, thickness: 1.7pt, dash: "dashed"), mark: (end: "straight", scale: 0.42))
          content((2.9, -2.15), text(8pt)[$p_2$])
          line((3.75, -2.15), (4.35, -2.15), stroke: 1.6pt + teal, mark: (end: "straight", scale: 0.42))
          content((4.7, -2.15), text(8pt)[$p_5$])
        }),
        caption: [Canonical YES instance for Path-Constrained Network Flow. Blue, dashed orange, and teal show the three prescribed paths used by $g = (1, 1, 0, 0, 1)$. The labels $2 / 2$ mark the shared arcs $(0,1)$ and $(5,7)$, whose flow exactly saturates capacity 2.],
      ) <fig:path-constrained-network-flow>
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

      #pred-commands(
        "pred create --example IsomorphicSpanningTree -o isomorphic-spanning-tree.json",
        "pred solve isomorphic-spanning-tree.json",
        "pred evaluate isomorphic-spanning-tree.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let W = x.instance.weight_bound
  let path-config = x.optimal_config
  let path-edges = edges.enumerate().filter(((idx, _)) => path-config.at(idx) == 1).map(((idx, e)) => e)
  let path-order = (0, 2, 3, 5)
  [
    #problem-def("ShortestWeightConstrainedPath")[
      Given an undirected graph $G = (V, E)$ with positive edge lengths $l: E -> ZZ^+$, positive edge weights $w: E -> ZZ^+$, designated vertices $s, t in V$, and a weight bound $W in ZZ^+$, find a simple path $P$ from $s$ to $t$ that minimizes $sum_(e in P) l(e)$ subject to $sum_(e in P) w(e) <= W$.
    ][
      Also called the _restricted shortest path_ or _resource-constrained shortest path_ problem. Garey and Johnson list it as ND30 and show NP-completeness via transformation from Partition @garey1979. The model captures bicriteria routing: one resource measures path length or delay, while the other captures a second consumable budget such as cost, risk, or bandwidth. Because pseudo-polynomial dynamic programming formulations are known @joksch1966, the hardness is weak rather than strong; approximation schemes were later developed by Hassin @hassin1992 and improved by Lorenz and Raz @lorenzraz2001.

      The implementation catalog reports the natural brute-force complexity of the edge-subset encoding used here: with $m = |E|$ binary variables, exhaustive search over all candidate subsets costs $O^*(2^m)$. A configuration is feasible when the selected edges form a single simple $s$-$t$ path whose total weight stays within the bound; the objective is to minimize total length over all such feasible paths.

      *Example.* Consider the graph on #nv vertices with source $s = v_#s$, target $t = v_#t$, and weight bound $W = #W$. Edge labels are written as $(l(e), w(e))$. The highlighted path $#path-order.map(v => $v_#v$).join($arrow$)$ uses edges ${#path-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$, so its total length is $4 + 1 + 4 = 9$ and its total weight is $1 + 3 + 3 = 7 <= #W$. This is the minimum-length feasible path; another weight-feasible path $v_0 arrow v_1 arrow v_4 arrow v_5$ has length $10$.

      #pred-commands(
        "pred create --example ShortestWeightConstrainedPath -o shortest-weight-constrained-path.json",
        "pred solve shortest-weight-constrained-path.json",
        "pred evaluate shortest-weight-constrained-path.json --config " + x.optimal_config.map(str).join(","),
      )

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

    #pred-commands(
      "pred create --example " + problem-spec(x) + " -o kcoloring.json",
      "pred solve kcoloring.json",
      "pred evaluate kcoloring.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let x = load-model-example("MaximumAchromaticNumber")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let num-colors = metric-value(sol.metric)
  let coloring = sol.config
  let color-groups = range(num-colors).map(c => coloring.enumerate().filter(((i, v)) => v == c).map(((i, _)) => i))
  [
    #problem-def("MaximumAchromaticNumber")[
      Given an undirected graph $G = (V, E)$, find a proper vertex coloring $c: V -> {1, dots, k}$ that is _complete_ --- for every pair of distinct colors $i, j$ there exists an edge $(u, v) in E$ with $c(u) = i$ and $c(v) = j$ --- maximizing the number of colors $k$.
    ][
      The achromatic number $psi(G)$ is the largest $k$ such that $G$ admits a complete proper $k$-coloring. It was introduced by Harary and Hedetniemi (1970) and shown NP-hard @garey1979[GT5]. Applications include network partition and information dissemination. Brute-force enumeration runs in $O^*(n^n)$ time.

      *Example.* Consider the 6-cycle $C_6$ with $n = #nv$ vertices and $|E| = #ne$ edges: #edges.map(((u, v)) => [${#u, #v}$]).join(", "). The coloring #range(nv).map(i => $c(v_#i) = #(coloring.at(i) + 1)$).join(", ") uses $#num-colors$ colors. It is proper (no adjacent pair shares a color) and complete: every pair of color classes is connected by at least one edge. Thus $psi(C_6) >= #num-colors$.

      #pred-commands(
        "pred create --example MaximumAchromaticNumber -o achromatic.json",
        "pred solve achromatic.json",
        "pred evaluate achromatic.json --config " + x.optimal_config.map(str).join(","),
      )
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
  let wS = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example MinimumDominatingSet -o minimum-dominating-set.json",
      "pred solve minimum-dominating-set.json",
      "pred evaluate minimum-dominating-set.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let x = load-model-example("MinimumGeometricConnectedDominatingSet")
  let n = x.instance.points.len()
  let B = x.instance.radius
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let S = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let wS = metric-value(sol.metric)
  [
    #problem-def("MinimumGeometricConnectedDominatingSet")[
      Given points $P = {p_1, dots, p_n}$ in $RR^2$ and distance threshold $B > 0$, find $P' subset.eq P$ minimizing $|P'|$ s.t. (1) $forall p in P backslash P': exists q in P'$ with $d(p,q) <= B$ (domination), and (2) the unit-disk graph on $P'$ with radius $B$ is connected.
    ][
    Geometric Connected Dominating Set arises in wireless ad-hoc networks: selected nodes form a connected backbone that covers all other nodes within communication range. The problem is NP-hard and generalizes both dominating set (dropping connectivity) and connected subgraph (dropping domination).

    *Example.* Consider $n = #n$ points arranged in a $4 times 2$ ladder with spacing $3$ and threshold $B = #B$. The bottom row $P' = {#S.map(i => $p_#i$).join(", ")}$ forms a minimum connected dominating set of size $#wS$: each bottom-row point dominates the top-row point directly above it (vertical distance $3 <= #B$), and consecutive bottom-row points are within distance $3 <= #B$ of each other, so $P'$ induces a connected path.

    #pred-commands(
      "pred create --example MinimumGeometricConnectedDominatingSet -o mgcds.json",
      "pred solve mgcds.json",
      "pred evaluate mgcds.json --config " + x.optimal_config.map(str).join(","),
    )
    ]
  ]
}
#{
  let x = load-model-example("MinimumCoveringByCliques")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let num-cliques = metric-value(sol.metric)
  let groups = range(num-cliques).map(c => sol.config.enumerate().filter(((i, v)) => v == c).map(((i, _)) => i))
  [
    #problem-def("MinimumCoveringByCliques")[
      Given an undirected graph $G = (V, E)$, find a collection of cliques $C_1, dots, C_k$ in $G$ such that every edge $e in E$ is contained in at least one $C_i$, and the number of cliques $k$ is minimized.
    ][
      Minimum Covering by Cliques (also called _edge clique cover_) is NP-hard @garey1979[GT59]. Applications include intersection graph recognition and computational biology. The minimum edge clique cover number equals the minimum dimension of a dot-product representation of the graph. Brute-force enumeration runs in $O^*(2^(|E|))$ time.

      *Example.* Consider $G$ with $n = #nv$ vertices and $|E| = #ne$ edges: #edges.map(((u, v)) => [${#u, #v}$]).join(", "). An optimal cover uses $#num-cliques$ cliques: #groups.enumerate().filter(((_, g)) => g.len() > 0).map(((i, g)) => [$C_#(i + 1)$: edges #g.map(j => str(j)).join(", ")]).join("; ").

      #pred-commands(
        "pred create --example MinimumCoveringByCliques -o covering-by-cliques.json",
        "pred solve covering-by-cliques.json",
        "pred evaluate covering-by-cliques.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}
#{
  let x = load-model-example("MinimumIntersectionGraphBasis")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let universe-size = metric-value(sol.metric)
  [
    #problem-def("MinimumIntersectionGraphBasis")[
      Given an undirected graph $G = (V, E)$, find a universe $U$ of minimum cardinality and an assignment of subsets $S_v subset.eq U$ for each vertex $v in V$ such that two vertices $u, v$ are adjacent if and only if $S_u inter S_v eq.not emptyset$. The minimum $|U|$ is the _intersection number_ of $G$.
    ][
      Minimum Intersection Graph Basis is NP-hard @garey1979[GT60]. Every graph is an intersection graph; the intersection number measures how compactly such a representation can be chosen. The intersection number is at most $|E|$. Brute-force enumeration runs in $O^*(|E|^(|E|))$ time.

      *Example.* Consider the path $P_3$ with $n = #nv$ vertices and $|E| = #ne$ edges: #edges.map(((u, v)) => [${#u, #v}$]).join(", "). An optimal representation uses $#universe-size$ elements: $S_0 = {0}$, $S_1 = {0, 1}$, $S_2 = {1}$.

      #pred-commands(
        "pred create --example MinimumIntersectionGraphBasis -o intersection-basis.json",
        "pred solve intersection-basis.json",
        "pred evaluate intersection-basis.json --config " + x.optimal_config.map(str).join(","),
      )
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
  let wM = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example MaximumMatching -o maximum-matching.json",
      "pred solve maximum-matching.json",
      "pred evaluate maximum-matching.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let x = load-model-example("BottleneckTravelingSalesman")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges
  let ew = x.instance.edge_weights
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let tour-edges = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => edges.at(i))
  let bottleneck = metric-value(sol.metric)
  let tour-weights = tour-edges.map(((u, v)) => {
    let idx = edges.position(e => e == (u, v) or e == (v, u))
    int(ew.at(idx))
  })
  let tour-total = tour-weights.sum()
  let tour-order = (0,)
  let remaining = tour-edges
  for _ in range(nv - 1) {
    let curr = tour-order.last()
    let next-edge = remaining.find(e => e.at(0) == curr or e.at(1) == curr)
    let next-v = if next-edge.at(0) == curr { next-edge.at(1) } else { next-edge.at(0) }
    tour-order.push(next-v)
    remaining = remaining.filter(e => e != next-edge)
  }
  let tsp-order = (0, 2, 3, 1, 4)
  let tsp-total = 13
  let tsp-bottleneck = 5
  let weight-labels = edges.map(((u, v)) => {
    let idx = edges.position(e => e == (u, v))
    (u: u, v: v, w: ew.at(idx))
  })
  [
    #problem-def("BottleneckTravelingSalesman")[
      Given an undirected graph $G=(V,E)$ with edge weights $w: E -> RR$, find an edge set $C subset.eq E$ that forms a cycle visiting every vertex exactly once and minimizes $max_(e in C) w(e)$.
    ][
    This min-max variant models routing where the worst leg matters more than the total distance. Garey and Johnson list the threshold decision version as ND24 @garey1979: given a bound $B$, ask whether some Hamiltonian tour has every edge weight at most $B$. The optimization version implemented here subsumes that decision problem. The classical Held--Karp dynamic programming algorithm still yields an exact $O(n^2 dot 2^n)$-time algorithm @heldkarp1962, while Garey and Johnson note the polynomial-time special case of Gilmore and Gomory @gilmore1964.

    *Example.* Consider the complete graph $K_#nv$ with vertices ${#range(nv).map(i => $v_#i$).join(", ")}$ and edge weights #weight-labels.map(l => $w(v_#(l.u), v_#(l.v)) = #(int(l.w))$).join(", "). The unique optimal bottleneck tour is $#tour-order.map(v => $v_#v$).join($arrow$) arrow v_#(tour-order.at(0))$ with edge weights #tour-weights.map(w => str(w)).join(", ") and bottleneck #bottleneck. Its total weight is #tour-total. By contrast, the minimum-total-weight TSP tour $#tsp-order.map(v => $v_#v$).join($arrow$) arrow v_#(tsp-order.at(0))$ has total weight #tsp-total but bottleneck #tsp-bottleneck, because it uses the weight-5 edge $(v_0, v_4)$. Here every other Hamiltonian tour in $K_#nv$ contains a weight-5 edge, so the blue tour is the only one that keeps the maximum edge weight at 4.

    #figure({
      let verts = ((0, 1.8), (1.7, 0.55), (1.05, -1.45), (-1.05, -1.45), (-1.7, 0.55))
      canvas(length: 1cm, {
        for (idx, (u, v)) in edges.enumerate() {
          let on-tour = tour-edges.any(t => (t.at(0) == u and t.at(1) == v) or (t.at(0) == v and t.at(1) == u))
          let on-tsp-only = (u == 0 and v == 4) or (u == 4 and v == 0)
          g-edge(
            verts.at(u),
            verts.at(v),
            stroke: if on-tour {
              2pt + graph-colors.at(0)
            } else if on-tsp-only {
              1.5pt + rgb("#c44e38")
            } else {
              0.8pt + luma(200)
            },
          )
          let mx = (verts.at(u).at(0) + verts.at(v).at(0)) / 2
          let my = (verts.at(u).at(1) + verts.at(v).at(1)) / 2
          let (dx, dy) = if u == 0 and v == 1 {
            (0.16, 0.2)
          } else if u == 0 and v == 2 {
            (0.25, 0.03)
          } else if u == 0 and v == 3 {
            (-0.25, 0.03)
          } else if u == 0 and v == 4 {
            (-0.16, 0.2)
          } else if u == 1 and v == 2 {
            (0.22, -0.05)
          } else if u == 1 and v == 3 {
            (0.12, -0.18)
          } else if u == 1 and v == 4 {
            (0, 0.12)
          } else if u == 2 and v == 3 {
            (0, -0.2)
          } else if u == 2 and v == 4 {
            (-0.12, -0.18)
          } else {
            (-0.22, -0.05)
          }
          draw.content((mx + dx, my + dy), text(7pt, fill: luma(80))[#str(int(ew.at(idx)))])
        }
        for (k, pos) in verts.enumerate() {
          g-node(pos, name: "v" + str(k), fill: graph-colors.at(0), label: text(fill: white)[$v_#k$])
        }
      })
    },
    caption: [The $K_5$ bottleneck-TSP instance. Blue edges form the unique optimal bottleneck tour; the red edge $(v_0, v_4)$ is the weight-5 edge used by the minimum-total-weight TSP tour.],
    ) <fig:k5-btsp>
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
  let tour-cost = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example TSP -o tsp.json",
      "pred solve tsp.json",
      "pred evaluate tsp.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let cost = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example SteinerTree -o steiner-tree.json",
      "pred solve steiner-tree.json",
      "pred evaluate steiner-tree.json --config " + x.optimal_config.map(str).join(","),
    )

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

    #pred-commands(
      "pred create --example StrongConnectivityAugmentation -o strong-connectivity-augmentation.json",
      "pred solve strong-connectivity-augmentation.json",
      "pred evaluate strong-connectivity-augmentation.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let cost = metric-value(sol.metric)
  [
    #problem-def("MinimumMultiwayCut")[
      Given an undirected graph $G=(V,E)$ with edge weights $w: E -> RR_(>0)$ and a set of $k$ terminal vertices $T = {t_1, ..., t_k} subset.eq V$, find a minimum-weight set of edges $C subset.eq E$ such that no two terminals remain in the same connected component of $G' = (V, E backslash C)$.
    ][
    The Minimum Multiway Cut problem generalizes the classical minimum $s$-$t$ cut: for $k=2$ it reduces to max-flow and is solvable in polynomial time, but for $k >= 3$ on general graphs it becomes NP-hard @dahlhaus1994. The problem arises in VLSI design, image segmentation, and network design. A $(2 - 2 slash k)$-approximation is achievable in polynomial time by taking the union of the $k - 1$ cheapest isolating cuts @dahlhaus1994. The best known exact algorithm runs in $O^*(1.84^k)$ time (suppressing polynomial factors) via submodular functions on isolating cuts @cao2013.

    *Example.* Consider a graph with $n = #nv$ vertices, $m = #ne$ edges, and $k = #terminals.len()$ terminals $T = {#terminals.map(t => $#t$).join(", ")}$, with edge weights #edges.zip(weights).map(((e, w)) => $w(#(e.at(0)), #(e.at(1))) = #w$).join(", "). The optimal multiway cut removes edges ${#cut-edges.map(e => $(#(e.at(0)), #(e.at(1)))$).join(", ")}$ with total weight #cut-edge-indices.map(i => $#(weights.at(i))$).join($+$) $= #cost$, placing each terminal in a distinct component.

    #pred-commands(
      "pred create --example MinimumMultiwayCut -o minimum-multiway-cut.json",
      "pred solve minimum-multiway-cut.json",
      "pred evaluate minimum-multiway-cut.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let config = x.optimal_config
  // Compute total cost
  let total-cost = edges.map(e => calc.abs(config.at(e.at(0)) - config.at(e.at(1)))).sum()
  [
    #problem-def("OptimalLinearArrangement")[
      Given an undirected graph $G=(V,E)$, find a bijection $f: V -> {0, 1, dots, |V|-1}$ that minimizes the total edge length $sum_({u,v} in E) |f(u) - f(v)|$.
    ][
      A classical NP-hard optimization problem from Garey & Johnson (GT42) @garey1979, with applications in VLSI design, graph drawing, and sparse matrix reordering. The problem asks for a vertex ordering on a line that minimizes the total "stretch" of all edges.

      NP-hardness was established by Garey, Johnson, and Stockmeyer @gareyJohnsonStockmeyer1976, via reduction from Simple Max Cut. The problem remains NP-hard on bipartite graphs, but is solvable in polynomial time on trees. The best known exact algorithm for general graphs uses dynamic programming over subsets in $O^*(2^n)$ time and space (Held-Karp style), analogous to TSP.

      *Example.* Consider a graph with #nv vertices and #ne edges. The arrangement $f = (#config.map(c => str(c)).join(", "))$ gives total cost $#edges.map(e => $|#config.at(e.at(0)) - #config.at(e.at(1))|$).join($+$) = #total-cost$, which is optimal.

      #pred-commands(
        "pred create --example OptimalLinearArrangement -o optimal-linear-arrangement.json",
        "pred solve optimal-linear-arrangement.json",
        "pred evaluate optimal-linear-arrangement.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}
#{
  let x = load-model-example("RootedTreeArrangement")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let K = x.instance.bound
  [
    #problem-def("RootedTreeArrangement")[
      Given an undirected graph $G = (V, E)$ and a non-negative integer $K$, is there a rooted tree $T = (U, F)$ with $|U| = |V|$ and a bijection $f: V -> U$ such that every edge $\{u, v\} in E$ maps to two nodes lying on a common root-to-leaf path in $T$, and $sum_(\{u, v\} in E) d_T(f(u), f(v)) <= K$?
    ][
      Rooted Tree Arrangement is GT45 in Garey and Johnson @garey1979. It generalizes Optimal Linear Arrangement by allowing the host layout to be any rooted tree rather than a single path. Garey and Johnson cite Gavril's NP-completeness proof via reduction from Optimal Linear Arrangement @gavril1977.

      The connection to Optimal Linear Arrangement is immediate: if the rooted tree is restricted to a chain, the stretch objective becomes the linear-arrangement objective. This explains why the two problems live in the same arrangement family. For tree-oriented ordering problems, Adolphson and Hu give a polynomial-time algorithm for optimal linear ordering on trees @adolphsonHu1973, showing that the difficulty here comes from simultaneously choosing both the rooted-tree topology and the vertex-to-node bijection.

      *Example.* Consider the graph with $n = #nv$ vertices, $|E| = #ne$ edges, and edge set ${#edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$. With bound $K = #K$, the chain tree encoded by parent array $(0, 0, 1, 2)$ and identity mapping $(0, 1, 2, 3)$ is a valid witness: every listed edge lies on the unique root-to-leaf chain, and the total stretch is $1 + 2 + 1 + 1 = 5 <= #K$. Therefore this canonical instance is a YES instance.

      #pred-commands(
        "pred create --example RootedTreeArrangement -o rooted-tree-arrangement.json",
        "pred solve rooted-tree-arrangement.json --solver brute-force",
        "pred evaluate rooted-tree-arrangement.json --config " + x.optimal_config.map(str).join(","),
      )
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

    #pred-commands(
      "pred create --example KClique -o kclique.json",
      "pred solve kclique.json",
      "pred evaluate kclique.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let omega = metric-value(sol.metric)
  // Edges within the clique
  let clique-edges = edges.filter(e => K.contains(e.at(0)) and K.contains(e.at(1)))
  [
    #problem-def("MaximumClique")[
      Given $G = (V, E)$, find $K subset.eq V$ maximizing $|K|$ such that all pairs in $K$ are adjacent: $forall u, v in K: (u, v) in E$. Equivalent to MIS on the complement graph $overline(G)$.
    ][
    Maximum Clique arises in social network analysis (finding tightly-connected communities), bioinformatics (protein interaction clusters), and coding theory. The problem is equivalent to Maximum Independent Set on the complement graph $overline(G)$. The best known algorithm runs in $O^*(1.1996^n)$ via the complement reduction to MIS @xiao2017. Robson's direct backtracking algorithm achieves $O^*(1.1888^n)$ using exponential space @robson2001.

    *Example.* Consider the house graph $G$ with $n = #nv$ vertices and $|E| = #ne$ edges. The triangle $K = {#K.map(i => $v_#i$).join(", ")}$ is a maximum clique of size $omega(G) = #omega$: all three pairs #clique-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ") are edges. No #(omega + 1)-clique exists because vertices $v_0$ and $v_1$ each have degree 2 and are not adjacent to all of ${#K.map(i => $v_#i$).join(", ")}$.

    #pred-commands(
      "pred create --example MaximumClique -o maximum-clique.json",
      "pred solve maximum-clique.json",
      "pred evaluate maximum-clique.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let w-opt = metric-value(opt.metric)
  // Suboptimal maximal IS {v1,v3} with w=2 (hardcoded — no longer in fixture)
  let S-sub = (1, 3)
  let w-sub = 2
  [
    #problem-def("MaximalIS")[
      Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing $sum_(v in S) w(v)$ such that $S$ is independent ($forall u, v in S: (u, v) in.not E$) and maximal (no vertex $u in V backslash S$ can be added to $S$ while maintaining independence).
    ][
    The maximality constraint (no vertex can be added) distinguishes this from MIS, which only requires maximum weight. Every maximum independent set is maximal, but not vice versa. The enumeration bound of $O^*(3^(n slash 3))$ for listing all maximal independent sets @tomita2006 is tight: Moon and Moser @moonmoser1965 showed every $n$-vertex graph has at most $3^(n slash 3)$ maximal independent sets, achieved by disjoint triangles.

    *Example.* Consider the path graph $P_#nv$ with $n = #nv$ vertices, edges $(v_i, v_(i+1))$ for $i = 0, ..., #(ne - 1)$, and unit weights $w(v) = 1$. The set $S = {#S-sub.map(i => $v_#i$).join(", ")}$ is a maximal independent set: no two vertices in $S$ are adjacent, and neither $v_0$ (adjacent to $v_1$), $v_2$ (adjacent to both), nor $v_4$ (adjacent to $v_3$) can be added. However, $S' = {#S-opt.map(i => $v_#i$).join(", ")}$ with $w(S') = #w-opt$ is a strictly larger maximal IS, illustrating that maximality does not imply maximum weight.

    #pred-commands(
      "pred create --example MaximalIS -o maximal-is.json",
      "pred solve maximal-is.json",
      "pred evaluate maximal-is.json --config " + x.optimal_config.map(str).join(","),
    )

    #figure({
      draw-node-highlight(((0, 0), (1, 0), (2, 0), (3, 0), (4, 0)), edges, S-sub)
    },
    caption: [Path $P_#nv$ with maximal IS $S = {#S-sub.map(i => $v_#i$).join(", ")}$ (blue, $w(S) = #w-sub$). $S$ is maximal --- no white vertex can be added --- but not maximum: ${#S-opt.map(i => $v_#i$).join(", ")}$ achieves $w = #w-opt$.],
    ) <fig:path-maximal-is>
    ]
  ]
}

#{
  let x = load-model-example("MinimumMaximalMatching")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let matched-edges = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => edges.at(i))
  let sz = metric-value(sol.metric)
  [
    #problem-def("MinimumMaximalMatching")[
      Given $G = (V, E)$, find $M subset.eq E$ of minimum cardinality such that $M$ is a matching and $M$ is maximal: every $e in E backslash M$ shares an endpoint with some $e' in M$.
    ][
    A maximal matching cannot be extended by any edge, so every non-selected edge must be "blocked" by a selected one. Among all such matchings, the problem seeks one of minimum size. Unlike Maximum Matching (solvable in $O(n^3)$ by Edmonds' algorithm @edmonds1965), Minimum Maximal Matching is NP-hard @yannakakis1980; it can also be viewed as a Minimum Dominating Set in the line graph.

    *Example.* Consider the path graph $P_#nv$ with $n = #nv$ vertices and $#ne$ edges. A minimum maximal matching is $M = {#matched-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$ with $|M| = #sz$. Every unselected edge shares an endpoint with a selected one, so $M$ is maximal.

    #pred-commands(
      "pred create --example MinimumMaximalMatching -o mmm.json",
      "pred solve mmm.json",
      "pred evaluate mmm.json --config " + x.optimal_config.map(str).join(","),
    )

    #figure({
      let vpos = range(nv).map(i => (i.bit-and(0xffff) * 1.5, 0))
      draw-edge-highlight(vpos, edges, matched-edges,
        matched-edges.fold((), (acc, (u, v)) => {
          let r = acc
          if u not in r { r.push(u) }
          if v not in r { r.push(v) }
          r
        }))
    },
    caption: [Path $P_#nv$ with minimum maximal matching $M = {#matched-edges.map(((u, v)) => $(v_#u, v_#v)$).join(", ")}$ (blue, $|M| = #sz$).],
    ) <fig:path-min-maximal-matching>
    ]
  ]
}

#{
  let x = load-model-example("MinimumDummyActivitiesPert")
  let nv = x.instance.graph.num_vertices
  let arcs = x.instance.graph.arcs
  let ne = arcs.len()
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let merged = arcs.enumerate().filter(((i, _)) => sol.config.at(i) == 1).map(((i, arc)) => arc)
  let dummy = arcs.enumerate().filter(((i, _)) => sol.config.at(i) == 0).map(((i, arc)) => arc)
  let opt = metric-value(sol.metric)
  let blue = graph-colors.at(0)
  [
    #problem-def("MinimumDummyActivitiesPert")[
      Given a precedence DAG $G = (V, A)$, find an activity-on-arc PERT event network with one real activity arc for each task $v in V$, minimizing the number of dummy activity arcs, such that for every ordered pair of tasks $(u, v)$ there is a path from the finish event of $u$ to the start event of $v$ if and only if $v$ is reachable from $u$ in $G$.
    ][
    The decision version of minimum dummy activities appears as ND44 in Garey and Johnson's compendium @garey1979. It arises when an activity-on-node precedence DAG must be converted into an activity-on-arc PERT chart: merging compatible finish/start events removes dummy activities, but an over-aggressive merge creates spurious precedence relations between unrelated tasks. The implementation here enumerates, for each direct precedence arc, whether it is realized as an event merge or left as a dummy activity, so brute-force over the $m = #ne$ direct precedences yields a worst-case bound of $O^*(2^m)$. #footnote[No exact algorithm improving on the direct-precedence merge encoding implemented in the codebase is claimed here.]

    *Example.* Consider the canonical precedence DAG on $n = #nv$ tasks with direct precedences #arcs.map(a => $(v_#(a.at(0)), v_#(a.at(1)))$).join(", "). The optimal encoding merges the predecessor-finish/successor-start pairs #merged.map(a => $(v_#(a.at(0)), v_#(a.at(1)))$).join(", "), so those handoffs need no dummy activity at all. The remaining direct precedences #dummy.map(a => $(v_#(a.at(0)), v_#(a.at(1)))$).join(" and ") still require dummy activities, so the optimum is $#opt$. Both unresolved precedences enter $v_3$, and merging either of them would identify unrelated task completions, creating spurious reachability between the two source tasks.

    #pred-commands(
      "pred create --example " + problem-spec(x) + " -o minimum-dummy-activities-pert.json",
      "pred solve minimum-dummy-activities-pert.json --solver brute-force",
      "pred evaluate minimum-dummy-activities-pert.json --config " + x.optimal_config.map(str).join(","),
    )

    #figure({
      let positions = ((0, 1.0), (0, -0.3), (2.0, 1.3), (2.0, 0.35), (2.0, -0.95), (4.0, 1.3))
      canvas(length: 1cm, {
        for (k, pos) in positions.enumerate() {
          g-node(pos, name: "v" + str(k),
            fill: white,
            label: [$v_#k$])
        }
        for arc in dummy {
          let (u, v) = arc
          draw.line("v" + str(u), "v" + str(v),
            stroke: (paint: luma(140), thickness: 1pt, dash: "dashed"),
            mark: (end: "straight", scale: 0.4))
        }
        for arc in merged {
          let (u, v) = arc
          draw.line("v" + str(u), "v" + str(v),
            stroke: 1.7pt + blue,
            mark: (end: "straight", scale: 0.45))
        }
      })
    },
    caption: [Canonical Minimum Dummy Activities in PERT Networks instance. Blue precedence arcs are encoded by merging the predecessor finish event with the successor start event; dashed gray arcs still require dummy activities. The optimal encoding leaves exactly #opt dummy activities.],
    ) <fig:minimum-dummy-activities-pert>
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
  let wS = metric-value(sol.metric)
  [
    #problem-def("MinimumFeedbackVertexSet")[
      Given a directed graph $G = (V, A)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ such that the induced subgraph $G[V backslash S]$ is a directed acyclic graph (DAG).
    ][
    One of Karp's 21 NP-complete problems ("Feedback Node Set") @karp1972. Applications include deadlock detection in operating systems, loop breaking in circuit design, and Bayesian network structure learning. The directed version is strictly harder than undirected FVS: the best known exact algorithm runs in $O^*(1.9977^n)$ @razgon2007, compared to $O^*(1.7548^n)$ for undirected graphs. An $O(log n dot log log n)$-approximation exists @even1998.

    *Example.* Consider the directed graph $G$ with $n = #nv$ vertices, $|A| = #ne$ arcs, and unit weights. The arcs form two overlapping directed cycles: $C_1 = v_0 -> v_1 -> v_2 -> v_0$ and $C_2 = v_0 -> v_3 -> v_4 -> v_1$. The set $S = {#S.map(i => $v_#i$).join(", ")}$ with $w(S) = #wS$ is a minimum feedback vertex set: removing $v_#(S.at(0))$ breaks both cycles, leaving a DAG with topological order $(v_3, v_4, v_1, v_2)$. No 0-vertex set suffices since $C_1$ and $C_2$ overlap only at $v_0$ and $v_1$, and removing $v_1$ alone leaves $C_1' = v_0 -> v_3 -> v_4 -> v_1 -> v_2 -> v_0$.

    #pred-commands(
      "pred create --example MinimumFeedbackVertexSet -o minimum-feedback-vertex-set.json",
      "pred solve minimum-feedback-vertex-set.json",
      "pred evaluate minimum-feedback-vertex-set.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let opt-weight = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example SteinerTreeInGraphs -o steiner-tree-in-graphs.json",
      "pred solve steiner-tree-in-graphs.json",
      "pred evaluate steiner-tree-in-graphs.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let opt-cost = metric-value(x.optimal_value)
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

    #pred-commands(
      "pred create --example MinimumSumMulticenter -o minimum-sum-multicenter.json",
      "pred solve minimum-sum-multicenter.json",
      "pred evaluate minimum-sum-multicenter.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let opt = x.optimal_value
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let centers = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("MinMaxMulticenter")[
      Given a graph $G = (V, E)$ with vertex weights $w: V -> ZZ_(>= 0)$, edge lengths $l: E -> ZZ_(>= 0)$, and a positive integer $K <= |V|$, find $S subset.eq V$ with $|S| = K$ that minimizes $max_(v in V) w(v) dot d(v, S)$, where $d(v, S) = min_(s in S) d(v, s)$ is the shortest weighted-path distance from $v$ to the nearest vertex in $S$.
    ][
    Also known as the _vertex p-center problem_ (Garey & Johnson A2 ND50). The goal is to place $K$ facilities so that the worst-case weighted distance from any demand point to its nearest facility is minimized. NP-hard even with unit weights and unit edge lengths (Kariv and Hakimi, 1979).

    Closely related to Dominating Set: on unweighted unit-length graphs, a $K$-center with optimal radius 1 corresponds to a dominating set of size $K$. The best known exact algorithm runs in $O^*(1.4969^n)$ via binary search over distance thresholds combined with dominating set computation @vanrooij2011. An optimal 2-approximation exists (Hochbaum and Shmoys, 1985); no $(2 - epsilon)$-approximation is possible unless $P = "NP"$ (Hsu and Nemhauser, 1979).

    Variables: $n = |V|$ binary variables, one per vertex. $x_v = 1$ if vertex $v$ is selected as a center. The objective value is $min_(|S| = K) max_(v in V) w(v) dot d(v, S)$; configurations with $|S| != K$ or unreachable vertices evaluate to $bot$ (infeasible).

    *Example.* Consider the graph $G$ on #nv vertices with unit weights $w(v) = 1$, unit edge lengths, edges ${#edges.map(((u, v)) => $(#u, #v)$).join(", ")}$, and $K = #K$. Placing centers at $S = {#centers.map(i => $v_#i$).join(", ")}$ gives maximum distance $max_v d(v, S) = #opt$, which is optimal.

    #pred-commands(
      "pred create --example MinMaxMulticenter -o min-max-multicenter.json",
      "pred solve min-max-multicenter.json",
      "pred evaluate min-max-multicenter.json --config " + x.optimal_config.map(str).join(","),
    )

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
    caption: [Min-Max Multicenter with $K = #K$ on a #{nv}-vertex graph. Centers #centers.map(i => $v_#i$).join(" and ") (blue) achieve optimal maximum distance #opt.],
    ) <fig:min-max-multicenter>
    ]
  ]
}

#{
  let x = load-model-example("MultipleCopyFileAllocation")
  let edges = x.instance.graph.edges.map(e => (e.at(0), e.at(1)))
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let copies = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("MultipleCopyFileAllocation")[
      Given a graph $G = (V, E)$, usage values $u: V -> ZZ_(> 0)$, and storage costs $s: V -> ZZ_(> 0)$, find a subset $V' subset.eq V$ that minimizes
      $sum_(v in V') s(v) + sum_(v in V) u(v) dot d(v, V'),$
      where $d(v, V') = min_(w in V') d_G(v, w)$ is the shortest-path distance from $v$ to the nearest copy vertex.
    ][
    Multiple Copy File Allocation appears in the storage-and-retrieval section of Garey and Johnson (SR6) @garey1979. The model combines two competing costs: each chosen copy vertex incurs a storage charge, while every vertex pays an access cost weighted by its demand and graph distance to the nearest copy. Garey and Johnson record the problem as NP-hard in the strong sense, even when usage and storage costs are uniform @garey1979.

    *Example.* Consider the 6-cycle $C_6$ with uniform usage $u(v) = 10$ and uniform storage $s(v) = 1$. Placing copies at every vertex $V' = {#copies.map(i => $v_#i$).join(", ")}$ gives storage cost $6 dot 1 = 6$ and access cost $0$ (each vertex is distance $0$ from its own copy), for a total cost of $#sol.metric$. This is optimal: removing any copy saves $1$ in storage but adds at least $10$ in access cost for each neighbor that must now reach a more distant copy.

    #pred-commands(
      "pred create --example MultipleCopyFileAllocation -o multiple-copy-file-allocation.json",
      "pred solve multiple-copy-file-allocation.json",
      "pred evaluate multiple-copy-file-allocation.json --config " + x.optimal_config.map(str).join(","),
    )

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
    caption: [Multiple Copy File Allocation on a 6-cycle. All vertices (shown in blue) host copies; total cost is $#sol.metric$.],
    ) <fig:multiple-copy-file-allocation>
    ]
  ]
}

#{
  let x = load-model-example("ExpectedRetrievalCost")
  [
    #problem-def("ExpectedRetrievalCost")[
      Given a set $R = {r_1, dots, r_n}$ of records, access probabilities $p(r) in [0, 1]$ with $sum_(r in R) p(r) = 1$, and a positive integer $m$ of circular storage sectors, find a partition $R_1, dots, R_m$ of $R$ that minimizes
      $sum_(i=1)^m sum_(j=1)^m p(R_i) p(R_j) d(i, j),$
      where $p(R_i) = sum_(r in R_i) p(r)$ and
      $d(i, j) = j - i - 1$ for $1 <= i < j <= m$, while $d(i, j) = m - i + j - 1$ for $1 <= j <= i <= m$.
    ][
    Expected Retrieval Cost is storage-and-retrieval problem SR4 in Garey and Johnson @garey1979. The model abstracts a drum-like storage device with fixed read heads: placing probability mass evenly around the cycle reduces the expected waiting time until the next requested sector rotates under the head. Cody and Coffman introduced the formulation and analyzed exact and heuristic record-allocation algorithms for fixed numbers of sectors @codycoffman1976. Garey and Johnson record that the general decision problem is NP-complete in the strong sense via transformations from Partition and 3-Partition @garey1979. The implementation in this repository uses one $m$-ary variable per record, so the registered exact baseline enumerates $m^n$ assignments.

    *Example.* Take six records with probabilities $(0.2, 0.15, 0.15, 0.2, 0.1, 0.2)$ and three sectors. Assign
    $R_1 = {r_1, r_5}$, $R_2 = {r_2, r_4}$, and $R_3 = {r_3, r_6}$.
    Then the sector masses are $(p(R_1), p(R_2), p(R_3)) = (0.3, 0.35, 0.35)$.
    For $m = 3$, the non-zero latencies are $d(1, 1) = d(2, 2) = d(3, 3) = 2$, $d(1, 3) = d(2, 1) = d(3, 2) = 1$, and the remaining pairs contribute 0. Hence the expected retrieval cost is $1.0025$, which is optimal for this instance.

    #pred-commands(
      "pred create --example ExpectedRetrievalCost -o expected-retrieval-cost.json",
      "pred solve expected-retrieval-cost.json --solver brute-force",
      "pred evaluate expected-retrieval-cost.json --config " + x.optimal_config.map(str).join(","),
    )

    #figure(
      table(
        columns: 3,
        inset: 6pt,
        stroke: 0.5pt + luma(180),
        [Sector], [Records], [Mass],
        [$S_1$], [$r_1, r_5$], [$0.3$],
        [$S_2$], [$r_2, r_4$], [$0.35$],
        [$S_3$], [$r_3, r_6$], [$0.35$],
      ),
      caption: [Expected Retrieval Cost example with cyclic sector order $S_1 -> S_2 -> S_3 -> S_1$. The optimal allocation yields masses $(0.3, 0.35, 0.35)$ and minimum cost $1.0025$.],
    ) <fig:expected-retrieval-cost>
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
  let wP = metric-value(sol.metric)
  // Format a set as {e1+1, e2+1, ...} (1-indexed)
  let fmt-set(s) = "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  [
    #problem-def("MaximumSetPacking")[
      Given universe $U$, collection $cal(S) = {S_1, ..., S_m}$ with $S_i subset.eq U$, weights $w: cal(S) -> RR$, find $cal(P) subset.eq cal(S)$ maximizing $sum_(S in cal(P)) w(S)$ s.t. $forall S_i, S_j in cal(P): S_i inter S_j = emptyset$.
    ][
    One of Karp's 21 NP-complete problems @karp1972. Generalizes maximum matching (the special case where all sets have size 2, solvable in polynomial time). Applications include resource allocation, VLSI design, and frequency assignment. The optimization version is as hard to approximate as maximum clique. The best known exact algorithm runs in $O^*(2^m)$ by brute-force enumeration over the $m$ sets#footnote[No algorithm improving on brute-force enumeration is known for general weighted set packing.].

    *Example.* Let $U = {1, 2, dots, #U-size}$ and $cal(S) = {#range(m).map(i => $S_#(i + 1)$).join(", ")}$ with #range(m).map(i => $S_#(i + 1) = #fmt-set(sets.at(i))$).join(", "), and unit weights $w(S_i) = 1$. A maximum packing is $cal(P) = {#selected.map(i => $S_#(i + 1)$).join(", ")}$ with $w(cal(P)) = #wP$: $S_#(selected.at(0) + 1) inter S_#(selected.at(1) + 1) = emptyset$. Adding $S_2$ would conflict with both ($S_1 inter S_2 = {2}$, $S_2 inter S_3 = {3}$), and $S_4$ conflicts with $S_3$ ($S_3 inter S_4 = {4}$). The alternative packing ${S_2, S_4}$ also achieves weight #wP.

    #pred-commands(
      "pred create --example " + problem-spec(x) + " -o maximum-set-packing.json",
      "pred solve maximum-set-packing.json",
      "pred evaluate maximum-set-packing.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let wC = metric-value(sol.metric)
  let fmt-set(s) = "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  [
    #problem-def("MinimumSetCovering")[
      Given universe $U$, collection $cal(S)$ with weights $w: cal(S) -> RR$, find $cal(C) subset.eq cal(S)$ minimizing $sum_(S in cal(C)) w(S)$ s.t. $union.big_(S in cal(C)) S = U$.
    ][
    One of Karp's 21 NP-complete problems @karp1972. Arises in facility location, crew scheduling, and test suite minimization. The greedy algorithm achieves an $O(ln n)$-approximation where $n = |U|$, which is essentially optimal: cannot be approximated within $(1-o(1)) ln n$ unless P = NP. The best known exact algorithm runs in $O^*(2^m)$ by brute-force enumeration over the $m$ sets#footnote[No algorithm improving on brute-force enumeration is known for general weighted set covering.].

    *Example.* Let $U = {1, 2, dots, #U-size}$ and $cal(S) = {#range(m).map(i => $S_#(i + 1)$).join(", ")}$ with #range(m).map(i => $S_#(i + 1) = #fmt-set(sets.at(i))$).join(", "), and unit weights $w(S_i) = 1$. A minimum cover is $cal(C) = {#selected.map(i => $S_#(i + 1)$).join(", ")}$ with $w(cal(C)) = #wC$: $#selected.map(i => $S_#(i + 1)$).join($union$) = {1, 2, dots, #U-size} = U$. No single set covers all of $U$, so at least two sets are required.

    #pred-commands(
      "pred create --example MinimumSetCovering -o minimum-set-covering.json",
      "pred solve minimum-set-covering.json",
      "pred evaluate minimum-set-covering.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let x = load-model-example("MinimumHittingSet")
  let sets = x.instance.sets
  let m = sets.len()
  let U-size = x.instance.universe_size
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let selected = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let hit-size = metric-value(sol.metric)
  let fmt-set(s) = if s.len() == 0 {
    $emptyset$
  } else {
    "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  }
  let elems = (
    (-2.0, 0.7),
    (-0.9, 1.4),
    (-1.2, -0.4),
    (0.2, 0.1),
    (1.2, 1.0),
    (1.5, -0.9),
  )
  [
    #problem-def("MinimumHittingSet")[
      Given a finite universe $U$ and a collection $cal(S) = {S_1, dots, S_m}$ of subsets of $U$, find a subset $H subset.eq U$ minimizing $|H|$ such that $H inter S_i != emptyset$ for every $i in {1, dots, m}$.
    ][
    Minimum Hitting Set is one of Karp's 21 NP-complete problems @karp1972. It is the incidence-dual of Set Covering: transposing the set-element incidence matrix swaps the choice of sets with the choice of universe elements. Vertex Cover is the special case in which every set has size $2$, so every edge is "hit" by selecting one of its endpoints.

    A direct exact algorithm enumerates all $2^n$ subsets $H subset.eq U$ for $n = |U|$ and checks whether each subset intersects every member of $cal(S)$. This yields an $O^*(2^n)$ exact algorithm#footnote[No exact worst-case algorithm improving on brute-force enumeration over the universe elements is recorded in the standard references used for this catalog entry.].

    *Example.* Let $U = {1, 2, dots, #U-size}$ and $cal(S) = {#range(m).map(i => $S_#(i + 1)$).join(", ")}$ with #range(m).map(i => $S_#(i + 1) = #fmt-set(sets.at(i))$).join(", "). A minimum hitting set is $H = #fmt-set(selected)$ with $|H| = #hit-size$: every set in $cal(S)$ contains at least one of the selected elements. No $2$-element subset of $U$ hits all #m sets, so the optimum is exactly $#hit-size$.

    #pred-commands(
      "pred create --example MinimumHittingSet -o minimum-hitting-set.json",
      "pred solve minimum-hitting-set.json",
      "pred evaluate minimum-hitting-set.json --config " + x.optimal_config.map(str).join(","),
    )

    #figure(
      canvas(length: 1cm, {
        sregion((elems.at(0), elems.at(1), elems.at(2)), pad: 0.45, label: [$S_1$], ..sregion-dimmed)
        sregion((elems.at(0), elems.at(3), elems.at(4)), pad: 0.48, label: [$S_2$], ..sregion-dimmed)
        sregion((elems.at(1), elems.at(3), elems.at(5)), pad: 0.48, label: [$S_3$], ..sregion-dimmed)
        sregion((elems.at(2), elems.at(4), elems.at(5)), pad: 0.48, label: [$S_4$], ..sregion-dimmed)
        sregion((elems.at(0), elems.at(1), elems.at(5)), pad: 0.48, label: [$S_5$], ..sregion-dimmed)
        sregion((elems.at(2), elems.at(3)), pad: 0.34, label: [$S_6$], ..sregion-dimmed)
        sregion((elems.at(1), elems.at(4)), pad: 0.34, label: [$S_7$], ..sregion-dimmed)
        for (k, pos) in elems.enumerate() {
          selem(pos, label: [#(k + 1)], fill: if selected.contains(k) { graph-colors.at(0) } else { black })
        }
      }),
      caption: [Minimum hitting set: the blue elements $#fmt-set(selected)$ intersect every set region $S_1, dots, S_#m$, so they hit the entire collection $cal(S)$.]
    ) <fig:min-hitting-set>
    ]
  ]
}

#{
  let x = load-model-example("SetSplitting")
  let subsets = x.instance.subsets
  let m = subsets.len()
  let n = x.instance.universe_size
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let part0 = sol.config.enumerate().filter(((i, v)) => v == 0).map(((i, _)) => i)
  let part1 = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let fmt-set(s) = "${" + s.map(e => str(e + 1)).join(", ") + "}$"
  let elems = (
    (-1.5, 0.0),
    (0.0, 1.0),
    (0.0, -1.0),
    (1.5, 0.0),
    (-0.5, -1.0),
    (0.5, 1.0),
  )
  [
    #problem-def("SetSplitting")[
      Given a finite universe $U$ and a collection $cal(C) = {C_1, dots, C_m}$ of subsets of $U$ each of size $gt.eq 2$, does there exist a 2-coloring $chi: U -> {0, 1}$ such that every $C_i$ is non-monochromatic — i.e., contains at least one element of each color?
    ][
    One of Garey and Johnson's NP-complete problems (SP4 in @garey1979), shown NP-complete by reduction from 3-SAT. It is equivalent to deciding whether a hypergraph is 2-colorable (also called Property B). The problem is trivially satisfiable when every subset has size exactly 2, reducing to 2-colorability of the corresponding graph; it becomes NP-complete for subsets of size $gt.eq 3$. The best known exact algorithm runs in $O^*(2^n)$ by brute-force enumeration over the $n = |U|$ elements.

    *Example.* Let $U = {1, 2, dots, #n}$ and $cal(C) = {C_1, dots, C_#m}$ with #range(m).map(i => $C_#(i + 1) = #fmt-set(subsets.at(i))$).join(", "). Coloring $S_1 = #fmt-set(part0)$ and $S_2 = #fmt-set(part1)$ splits all subsets: each $C_i$ has at least one element in each part.

    #pred-commands(
      "pred create --example SetSplitting -o set-splitting.json",
      "pred solve set-splitting.json",
      "pred evaluate set-splitting.json --config " + x.optimal_config.map(str).join(","),
    )

    #figure(
      canvas(length: 1cm, {
        for (i, subset) in subsets.enumerate() {
          let positions = subset.map(e => elems.at(e))
          sregion(positions, pad: 0.42, label: [$C_#(i + 1)$], ..sregion-dimmed)
        }
        for (k, pos) in elems.enumerate() {
          selem(pos, label: [#(k + 1)], fill: if part1.contains(k) { graph-colors.at(0) } else { white })
        }
      }),
      caption: [Set splitting: white elements $#fmt-set(part0)$ form $S_1$ and blue elements $#fmt-set(part1)$ form $S_2$; every subset $C_1, dots, C_#m$ contains both colors.]
    ) <fig:set-splitting>
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

      #pred-commands(
        "pred create --example ConsecutiveSets -o consecutive-sets.json",
        "pred solve consecutive-sets.json",
        "pred evaluate consecutive-sets.json --config " + x.optimal_config.map(str).join(","),
      )
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

    #pred-commands(
      "pred create --example ExactCoverBy3Sets -o exact-cover-by-3-sets.json",
      "pred solve exact-cover-by-3-sets.json",
      "pred evaluate exact-cover-by-3-sets.json --config " + x3c.optimal_config.map(str).join(","),
    )
    ]
  ]
}

#{
  let tdm = load-model-example("ThreeDimensionalMatching")
  let q = tdm.instance.universe_size
  let triples = tdm.instance.triples
  let m = triples.len()
  let sol = tdm.optimal_config
  // Format a triple as (w+1, x+1, y+1) using 1-indexed notation
  let fmt-triple(t) = "$(" + str(t.at(0) + 1) + ", " + str(t.at(1) + 1) + ", " + str(t.at(2) + 1) + ")$"
  // Collect indices of selected triples (0-indexed)
  let selected = sol.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)

  [
    #problem-def("ThreeDimensionalMatching")[
      Given disjoint sets $W$, $X$, $Y$ each with $q$ elements and a set $M subset.eq W times X times Y$ of triples, does $M$ contain a _matching_ — a subset $M' subset.eq M$ with $|M'| = q$ such that no two triples in $M'$ agree in any coordinate?
    ][
    Shown NP-complete by Karp (1972) @karp1972. Three-Dimensional Matching (3DM) is one of the six basic NP-complete problems in Garey & Johnson (A3 SP1) and is closely related to Exact Cover by 3-Sets. While X3C asks for a perfect partition of a single universe into disjoint triples, 3DM asks for a system of distinct representatives across three separate dimensions. The problem remains NP-complete even when each element appears in at most three triples. The direct brute-force algorithm runs in $O^*(2^m)$ time where $m = |M|$.

    *Example.* Let $W = X = Y = {1, 2, dots, #q}$ and $M = {t_1, dots, t_#m}$ with #triples.enumerate().map(((i, t)) => $t_#(i + 1) = #fmt-triple(t)$).join(", "). A valid matching is $M' = {#selected.map(i => $t_#(i + 1)$).join(", ")}$: the $W$-coordinates, $X$-coordinates, and $Y$-coordinates are each pairwise distinct, and $|M'| = #selected.len() = q$.

    #pred-commands(
      "pred create --example ThreeDimensionalMatching -o three-dimensional-matching.json",
      "pred solve three-dimensional-matching.json",
      "pred evaluate three-dimensional-matching.json --config " + tdm.optimal_config.map(str).join(","),
    )
    ]
  ]
}

#{
  let tmi = load-model-example("ThreeMatroidIntersection")
  let n = tmi.instance.ground_set_size
  let parts = tmi.instance.partitions
  let K = tmi.instance.bound
  let sol = tmi.optimal_config
  let selected = sol.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let fmt-set(items) = if items.len() == 0 {
    $emptyset$
  } else {
    "${" + items.map(e => str(e)).join(", ") + "}$"
  }
  let fmt-group(g) = "${" + g.map(e => str(e)).join(", ") + "}$"
  [
    #problem-def("ThreeMatroidIntersection")[
      Given three partition matroids $(E, cal(F)_1)$, $(E, cal(F)_2)$, $(E, cal(F)_3)$ on a common ground set $E$ with $|E| = n$, and a positive integer $K <= n$, does there exist a subset $E' subset.eq E$ with $|E'| = K$ that is independent in all three matroids? A partition matroid partitions $E$ into groups; a set $S$ is independent if $|S sect G| <= 1$ for every group $G$.
    ][
    Three-Matroid Intersection is problem SP11 in Garey & Johnson @garey1979 (section A3). While 2-matroid intersection is solvable in polynomial time (Edmonds, 1970) @edmonds1970, the jump to three matroids captures NP-hardness. NP-completeness is established by transformation from Three-Dimensional Matching, where each dimension induces a partition matroid. The restriction to partition matroids suffices for NP-completeness.

    Doron-Arad, Kulik, and Shachnai (2024) @doron2024 showed that brute force essentially cannot be beaten: any algorithm requires $Omega(2^(n - 5 sqrt(n) log n))$ oracle queries. A marginal improvement to $2^(n - Omega(log^2 n))$ exists via Monotone Local Search @fomin2019. The direct brute-force algorithm runs in $O^*(2^n)$ time where $n = |E|$.

    *Example.* Let $E = {0, 1, dots, #(n - 1)}$ with $K = #K$. The three partition matroids have groups: $cal(F)_1$: #parts.at(0).map(fmt-group).join(", "); $cal(F)_2$: #parts.at(1).map(fmt-group).join(", "); $cal(F)_3$: #parts.at(2).map(fmt-group).join(", "). The subset $E' = #fmt-set(selected)$ is a valid common independent set of size $#K$: each matroid has at most one selected element per group.

    #pred-commands(
      "pred create --example ThreeMatroidIntersection -o three-matroid-intersection.json",
      "pred solve three-matroid-intersection.json",
      "pred evaluate three-matroid-intersection.json --config " + tmi.optimal_config.map(str).join(","),
    )
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

    #pred-commands(
      "pred create --example ComparativeContainment -o comparative-containment.json",
      "pred solve comparative-containment.json",
      "pred evaluate comparative-containment.json --config " + x.optimal_config.map(str).join(","),
    )

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

    #pred-commands(
      "pred create --example SetBasis -o set-basis.json",
      "pred solve set-basis.json",
      "pred evaluate set-basis.json --config " + x.optimal_config.map(str).join(","),
    )

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

    #pred-commands(
      "pred create --example PrimeAttributeName -o prime-attribute-name.json",
      "pred solve prime-attribute-name.json",
      "pred evaluate prime-attribute-name.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let key-attrs = range(n).filter(i => x.optimal_config.at(i) == 1)
  let fmt-set(s) = "${" + s.map(e => str(e)).join(", ") + "}$"
  let fmt-fd(d) = fmt-set(d.at(0)) + " $arrow.r$ " + fmt-set(d.at(1))
  [
    #problem-def("MinimumCardinalityKey")[
      Given a set $A$ of attribute names and a collection $F$ of functional dependencies (ordered pairs of subsets of $A$), find a key $K subset.eq A$ of minimum cardinality, i.e., a subset $K$ such that the closure of $K$ under $F^*$ equals $A$ and $|K|$ is minimized.
    ][
    The Minimum Cardinality Key problem arises in relational database theory, where identifying the smallest candidate key determines the most efficient way to uniquely identify rows in a relation. It was shown NP-complete by Lucchesi and Osborn (1978) @lucchesi1978keys via transformation from Vertex Cover. The problem appears as SR26 in Garey & Johnson (A4) @garey1979. The closure $F^*$ is defined by Armstrong's axioms: reflexivity ($B subset.eq C$ implies $C arrow.r B$), transitivity, and union. The best known exact algorithm is brute-force enumeration of all subsets of $A$, giving $O^*(2^(|A|))$ time#footnote[Lucchesi and Osborn give an output-polynomial algorithm for enumerating all candidate keys, but the number of keys can be exponential.].

    *Example.* Let $A = {0, 1, ..., #(n - 1)}$ ($|A| = #n$) with functional dependencies $F = {#deps.enumerate().map(((i, d)) => fmt-fd(d)).join(", ")}$.
    The optimal key $K = #fmt-set(key-attrs)$ has $|K| = #key-attrs.len()$. Its closure: start with ${0, 1}$; apply ${0, 1} arrow.r {2}$ to get ${0, 1, 2}$; apply ${0, 2} arrow.r {3}$ to get ${0, 1, 2, 3}$; apply ${1, 3} arrow.r {4}$ to get ${0, 1, 2, 3, 4}$; apply ${2, 4} arrow.r {5}$ to get $A$. Neither ${0}$ nor ${1}$ alone determines $A$, so $K$ is a minimum-cardinality key.

    #pred-commands(
      "pred create --example MinimumCardinalityKey -o minimum-cardinality-key.json",
      "pred solve minimum-cardinality-key.json",
      "pred evaluate minimum-cardinality-key.json --config " + x.optimal_config.map(str).join(","),
    )
    ]
  ]
}

#{
  let x = load-model-example("RootedTreeStorageAssignment")
  let n = x.instance.universe_size
  let subsets = x.instance.subsets
  let m = subsets.len()
  let K = x.instance.bound
  let config = x.optimal_config
  let edges = config.enumerate().filter(((v, p)) => v != p).map(((v, p)) => (p, v))
  let fmt-set(s) = "${" + s.map(e => str(e)).join(", ") + "}$"
  let highlight-nodes = (0, 2, 4)
  let highlight-edges = ((0, 2), (2, 4))
  [
    #problem-def("RootedTreeStorageAssignment")[
      Given a finite set $X = {0, 1, dots, #(n - 1)}$, a collection $cal(C) = {X_1, dots, X_m}$ of subsets of $X$, and a nonnegative integer $K$, find a directed rooted tree $T = (X, A)$ and supersets $X_i' supset.eq X_i$ such that every $X_i'$ forms a directed path in $T$ and $sum_(i = 1)^m |X_i' backslash X_i| <= K$.
    ][
    Rooted Tree Storage Assignment is the storage-and-retrieval problem SR5 in Garey and Johnson @garey1979. Their catalog credits a reduction from Rooted Tree Arrangement, framing the problem as hierarchical file organization: pick a rooted tree on the records so every request set can be completed to a single root-to-leaf path using only a limited number of extra records. The implementation here uses one parent variable per element of $X$, so the direct exhaustive bound is $|X|^(|X|)$ candidate parent arrays, filtered down to valid rooted trees#footnote[No exact algorithm improving on the direct parent-array search bound is claimed here for the general formulation.].

    *Example.* Let $X = {0, 1, dots, #(n - 1)}$, $K = #K$, and $cal(C) = {#range(m).map(i => $X_#(i + 1)$).join(", ")}$ with #subsets.enumerate().map(((i, s)) => $X_#(i + 1) = #fmt-set(s)$).join(", "). The satisfying parent array $p = (#config.map(str).join(", "))$ encodes the rooted tree with arcs #edges.map(((u, v)) => $(#u, #v)$).join(", "). In this tree, $X_1 = {0, 2}$, $X_2 = {1, 3}$, and $X_4 = {2, 4}$ are already directed paths. The only extension is $X_3 = {0, 4}$, which becomes $X_3' = {0, 2, 4}$ along the path $0 -> 2 -> 4$, so the total extension cost is exactly $1 = K$.

    #pred-commands(
      "pred create --example " + problem-spec(x) + " -o rooted-tree-storage-assignment.json",
      "pred solve rooted-tree-storage-assignment.json --solver brute-force",
      "pred evaluate rooted-tree-storage-assignment.json --config " + x.optimal_config.map(str).join(","),
    )

    #figure(
      canvas(length: 1cm, {
        import draw: *

        let positions = (
          (1.5, 1.8),
          (0.6, 0.9),
          (2.4, 0.9),
          (0.6, 0.0),
          (2.4, 0.0),
        )

        for (u, v) in edges {
          let highlighted = highlight-edges.contains((u, v))
          line(
            positions.at(u),
            positions.at(v),
            stroke: if highlighted { 1.2pt + graph-colors.at(0) } else { 0.8pt + luma(140) },
            mark: (end: "straight", scale: 0.45),
          )
        }

        for (vertex, pos) in positions.enumerate() {
          let highlighted = highlight-nodes.contains(vertex)
          circle(
            pos,
            radius: 0.2,
            fill: if highlighted { graph-colors.at(0) } else { white },
            stroke: 0.6pt + black,
          )
          content(pos, if highlighted { text(fill: white)[$#vertex$] } else { [$#vertex$] })
        }
      }),
      caption: [Rooted Tree Storage Assignment example. The rooted tree encoded by $p = (#config.map(str).join(", "))$ is shown; the blue path $0 -> 2 -> 4$ is the unique extension needed to realize $X_3 = {0, 4}$ within total cost $K = #K$.],
    ) <fig:rooted-tree-storage-assignment>
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

    #pred-commands(
      "pred create --example TwoDimensionalConsecutiveSets -o two-dimensional-consecutive-sets.json",
      "pred solve two-dimensional-consecutive-sets.json",
      "pred evaluate two-dimensional-consecutive-sets.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let H = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example SpinGlass -o spinglass.json",
      "pred solve spinglass.json",
      "pred evaluate spinglass.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let fstar = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example QUBO -o qubo.json",
      "pred solve qubo.json",
      "pred evaluate qubo.json --config " + x.optimal_config.map(str).join(","),
    )
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
  let fstar = metric-value(sol.metric)
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
      Given $n$ variables $bold(x)$ over a domain $cal(D)$ (binary $cal(D) = {0,1}$ or integer $cal(D) = ZZ_(>=0)$), constraint matrix $A in RR^(m times n)$, bounds $bold(b) in RR^m$, and objective $bold(c) in RR^n$, solve
      $
        min quad & bold(c)^top bold(x) \
        "subject to" quad & A bold(x) <= bold(b) \
        & bold(x) in cal(D)^n
      $.
    ][
    Integer Linear Programming is a universal modeling framework: virtually every NP-hard combinatorial optimization problem admits an ILP formulation. Relaxing integrality to $bold(x) in RR^n$ yields a linear program solvable in polynomial time, forming the basis of branch-and-bound solvers. When the number of integer variables $n$ is fixed, ILP is solvable in polynomial time by Lenstra's algorithm @lenstra1983 using the geometry of numbers, making it fixed-parameter tractable in $n$. The best known general algorithm achieves $O^*(n^n)$ via an FPT algorithm based on lattice techniques @dadush2012.

    *Example.* Minimize $bold(c)^top bold(x) = #fmt-obj$ subject to #constraints.map(fmt-constraint).join(", "), $#range(nv).map(i => $x_#(i + 1)$).join(",") >= 0$, $bold(x) in ZZ^#nv$. The LP relaxation optimum is $p_1 = (7 slash 3, 8 slash 3) approx (2.33, 2.67)$ with value $approx -27.67$, which is non-integral. Branch-and-bound yields the ILP optimum $bold(x)^* = (#xstar.map(v => str(v)).join(", "))$ with $bold(c)^top bold(x)^* = #fstar$.

    #pred-commands(
      "pred create --example " + problem-spec(x) + " -o ilp.json",
      "pred solve ilp.json",
      "pred evaluate ilp.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let cost-star = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example QuadraticAssignment -o quadratic-assignment.json",
      "pred solve quadratic-assignment.json",
      "pred evaluate quadratic-assignment.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let x = load-model-example("AlgebraicEquationsOverGF2")
  let n = x.instance.num_variables
  let eqs = x.instance.equations
  let config = x.optimal_config
  [
    #problem-def("AlgebraicEquationsOverGF2")[
      Given $n$ Boolean variables $x_0, dots, x_(n-1)$ and $m$ multilinear polynomials $p_1, dots, p_m$ over $"GF"(2)$, determine whether there exists an assignment $(x_0, dots, x_(n-1)) in {0,1}^n$ such that $p_j (x_0, dots, x_(n-1)) = 0$ for all $j = 1, dots, m$.

      Each polynomial is a sum (XOR) of monomials over $"GF"(2)$. Each monomial is a product (AND) of a subset of the variables; the empty product denotes the constant $1$.
    ][
      Algebraic Equations over $"GF"(2)$ generalises systems of linear equations over $"GF"(2)$ by allowing higher-degree monomials. The problem is NP-complete in general @garey1979. When restricted to degree $<= 1$ (linear equations only), the system reduces to Gaussian elimination over $"GF"(2)$ and is solvable in polynomial time.

      *Example.* Let $n = #n$ with #(eqs.len()) equations. The assignment $(#(config.map(str).join(", ")))$ satisfies all equations.

      #pred-commands(
        "pred create --example AlgebraicEquationsOverGF2 -o agf2.json",
        "pred solve agf2.json --solver brute-force",
        "pred evaluate agf2.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("QuadraticCongruences")
  let a = x.instance.a
  let b = x.instance.b
  let c = x.instance.c
  let config = x.optimal_config
  let xval = config.at(0) + 1
  // Collect all x in {1..c-1} and check x² mod b == a
  let rows = range(1, c).map(xi => {
    let sq = xi * xi
    let rem = calc.rem(sq, b)
    let ok = rem == a
    (xi, sq, rem, ok)
  })
  [
    #problem-def("QuadraticCongruences")[
      Given non-negative integers $a$, $b$, $c$ with $b > 0$ and $a < b$, determine whether there exists a positive integer $x$ with $1 <= x < c$ such that $x^2 equiv a space (mod space b)$.
    ][
      Quadratic Congruences is an NP-complete problem in the setting where $b$ is composite and given in unary (or the factorisation of $b$ is not provided) @garey1979. The problem asks whether $a$ is a _quadratic residue_ modulo $b$ in the range $\{1, dots, c-1\}$. When $b$ is prime, quadratic residuosity can be decided in polynomial time via Euler's criterion or the Legendre symbol, but the general case with composite modulus is believed to be computationally hard without the factorisation of $b$.

      *Example.* Let $a = #a$, $b = #b$, $c = #c$. We test each $x in {1, dots, #(c - 1)}$:

      #pred-commands(
        "pred create --example QuadraticCongruences -o qc.json",
        "pred solve qc.json --solver brute-force",
        "pred evaluate qc.json --config " + config.map(str).join(","),
      )

      #align(center, table(
        columns: 4,
        align: center,
        table.header([$x$], [$x^2$], [$x^2 mod #b$], [Satisfies?]),
        ..rows.map(((xi, sq, rem, ok)) => (
          [$#xi$],
          [$#sq$],
          [$#rem$],
          [#if ok [Yes] else [No]],
        )).flatten(),
      ))

      The instance is satisfiable: $x = #xval$ gives $#xval^2 = #(xval * xval) equiv #a space (mod space #b)$.
    ]
  ]
}

#{
  let x = load-model-example("QuadraticDiophantineEquations")
  let a = x.instance.a
  let b = x.instance.b
  let c = x.instance.c
  let config = x.optimal_config
  let xval = config.at(0) + 1
  let yval = int((c - a * xval * xval) / b)
  // Enumerate all valid x values for the table
  let max-x = calc.floor(calc.sqrt(c / a))
  let rows = range(1, max-x + 1).map(xi => {
    let rem = c - a * xi * xi
    let feasible = rem > 0 and calc.rem(rem, b) == 0
    let yi = if feasible { int(rem / b) } else { none }
    (xi, rem, feasible, yi)
  })
  [
    #problem-def("QuadraticDiophantineEquations")[
      Given positive integers $a$, $b$, $c$, determine whether there exist positive integers $x$, $y$ such that $a x^2 + b y = c$.
    ][
      Quadratic Diophantine equations of the form $a x^2 + b y = c$ form one of the simplest families of mixed-degree Diophantine problems. The variable $y$ is entirely determined by $x$ via $y = (c - a x^2) slash b$, so the decision problem reduces to checking whether any $x in {1, dots, floor(sqrt(c slash a))}$ yields a positive integer $y$. This can be done in $O(sqrt(c))$ time by trial#footnote[No algorithm improving on brute-force trial of all candidate $x$ values is known; the registered complexity `sqrt(c)` reflects this direct enumeration bound.].

      *Example.* Let $a = #a$, $b = #b$, $c = #c$. Then $x$ ranges over $1, dots, #max-x$:

      #pred-commands(
        "pred create --example QuadraticDiophantineEquations -o qde.json",
        "pred solve qde.json --solver brute-force",
        "pred evaluate qde.json --config " + config.map(str).join(","),
      )

      #align(center, table(
        columns: 4,
        align: center,
        table.header([$x$], [$c - a x^2$], [Divisible by $b$?], [$y$]),
        ..rows.map(((xi, rem, ok, yi)) => (
          [$#xi$],
          [$#rem$],
          [#if ok [Yes] else [No]],
          [#if yi != none [$#yi$] else [$dash$]],
        )).flatten(),
      ))

      The instance is satisfiable: $x = #xval, y = #yval$ gives $#a dot #xval^2 + #b dot #yval = #c$.
    ]
  ]
}

#{
  let x = load-model-example("SimultaneousIncongruences")
  let pairs = x.instance.pairs
  let config = x.optimal_config
  let xval = config.at(0)
  // Build table rows: for each pair (a, b), compute x mod b and check ≠ a mod b
  let rows = pairs.map(pair => {
    let a = pair.at(0)
    let b = pair.at(1)
    let r = calc.rem(xval, b)
    let cond = calc.rem(a, b)
    let ok = r != cond
    (a, b, r, cond, ok)
  })
  let moduli = pairs.map(p => p.at(1))
  // lcm(2,3,5,7) = 210 for the canonical example
  let lcm-val = moduli.fold(1, (l, b) => l * b / {
    // gcd via Euclidean algorithm (unrolled for small values)
    let aa = l
    let bb = b
    while bb != 0 {
      let tmp = calc.rem(aa, bb)
      aa = bb
      bb = tmp
    }
    aa
  })
  [
    #problem-def("SimultaneousIncongruences")[
      Given a list of pairs $(a_i, b_i)$ with $b_i > 0$ and $1 <= a_i <= b_i$ for $i = 1, dots, n$, determine whether there exists a non-negative integer $x$ such that $x equiv.not a_i space (mod space b_i)$ for all $i$.
    ][
      Simultaneous Incongruences is an NP-complete problem @garey1979. It asks whether the complement of a system of congruences — a _covering system_ — can be simultaneously avoided. A _covering system_ is a finite collection of congruences $\{a_i space (op("mod") space b_i)\}$ that covers every integer; when the system is a covering system there is no valid $x$ and the instance is a "no" instance. The problem generalises checking whether a given set of congruences is a covering system, which has connections to Erdős's covering conjecture and sieve methods in analytic number theory.

      *Example.* Let $n = #pairs.len()$ with pairs #pairs.map(p => $(#p.at(0), #p.at(1))$).join(", "). The full period is $L = op("lcm")(#moduli.map(str).join(", ")) = #lcm-val$. We test $x = #xval$:

      #pred-commands(
        "pred create --example SimultaneousIncongruences -o si.json",
        "pred solve si.json --solver brute-force",
        "pred evaluate si.json --config " + config.map(str).join(","),
      )

      #align(center, table(
        columns: 5,
        align: center,
        table.header([$a_i$], [$b_i$], [$#xval mod b_i$], [$a_i mod b_i$], [Avoids?]),
        ..rows.map(((a, b, r, cond, ok)) => (
          [$#a$],
          [$#b$],
          [$#r$],
          [$#cond$],
          [#if ok [Yes] else [No]],
        )).flatten(),
      ))

      The instance is satisfiable: $x = #xval$ avoids all congruences.
    ]
  ]
}

#{
  let x = load-model-example("EquilibriumPoint")
  let polynomials = x.instance.polynomials
  let range_sets = x.instance.range_sets
  let config = x.optimal_config
  let n = range_sets.len()
  // Recover assignment: y_i = range_sets[i][config[i]]
  let assignment = range(n).map(i => range_sets.at(i).at(config.at(i)))
  // Helper: evaluate a single affine factor at assignment
  let eval-factor(coeffs) = {
    coeffs.at(0) + range(n).fold(0, (acc, j) => acc + coeffs.at(j + 1) * assignment.at(j))
  }
  // Helper: evaluate F_i (product of all factors)
  let eval-payoff(i) = {
    polynomials.at(i).fold(1, (prod, f) => prod * eval-factor(f))
  }
  [
    #problem-def("EquilibriumPoint")[
      Given $n$ players, finite strategy sets $M_i subset ZZ$, and polynomial payoff functions $F_i : M_1 times dots.c times M_n -> ZZ$ expressed as products of affine factors, determine whether there exists a pure-strategy Nash equilibrium: an assignment $bold(y) = (y_1, dots, y_n)$ with $y_i in M_i$ such that for every player $i$ and every $y'_i in M_i$, $F_i(bold(y)) >= F_i(bold(y)^((-i, y'_i)))$, where $bold(y)^((-i, y'_i))$ is $bold(y)$ with the $i$-th component replaced by $y'_i$.
    ][
      Equilibrium Point (problem AN15 in Garey & Johnson @garey1979) is NP-complete by reduction from 3-SAT due to Sahni @sahni1974. The problem captures a fundamental question in algorithmic game theory: does a multi-player game with polynomial payoffs admit a stable strategy profile from which no player benefits by deviating? The payoff functions are represented as products of affine factors, enabling compact encoding of degree-$k$ polynomials with $O(k n)$ coefficients per player. The problem remains NP-complete even when all strategy sets are binary.

      *Example.* Consider $n = #n$ players with strategy sets $M_i = \{#range_sets.at(0).map(str).join(", ")\}$ for all $i$, and payoff functions:

      #let fmt-factors(factors) = {
        factors.map(coeffs => {
          // Format: a0 + a1*x1 + ... + an*xn, omit zero terms
          let terms = (coeffs.at(0),) + range(n).map(j => coeffs.at(j+1))
          let parts = ()
          if terms.at(0) != 0 { parts.push(str(terms.at(0))) }
          for j in range(n) {
            let c = terms.at(j+1)
            if c == 1 { parts.push("x" + str(j+1)) }
            else if c == -1 { parts.push("-x" + str(j+1)) }
            else if c != 0 { parts.push(str(c) + "x" + str(j+1)) }
          }
          if parts.len() == 0 { parts.push("0") }
          "(" + parts.join(" + ") + ")"
        }).join(sym.dot.op)
      }
      #align(center)[
        $F_1 = #fmt-factors(polynomials.at(0))$,
        $F_2 = #fmt-factors(polynomials.at(1))$,
        $F_3 = #fmt-factors(polynomials.at(2))$
      ]

      The assignment $bold(y) = (#assignment.map(str).join(", "))$ is a Nash equilibrium: $F_i(bold(y)) = #range(n).map(i => str(eval-payoff(i))).join(", ")$ for $i = 1, 2, 3$, and no player can strictly improve their payoff by deviating.

      #pred-commands(
        "pred create --example EquilibriumPoint -o ep.json",
        "pred solve ep.json --solver brute-force",
        "pred evaluate ep.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("ClosestVectorProblem")
  let basis = x.instance.basis
  let target = x.instance.target
  let bounds = x.instance.bounds
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let dist = metric-value(sol.metric)
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

      #pred-commands(
        "pred create --example ClosestVectorProblem -o closest-vector-problem.json",
        "pred solve closest-vector-problem.json",
        "pred evaluate closest-vector-problem.json --config " + x.optimal_config.map(str).join(","),
      )

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

    #pred-commands(
      "pred create --example SAT -o sat.json",
      "pred solve sat.json",
      "pred evaluate sat.json --config " + x.optimal_config.map(str).join(","),
    )
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

    #pred-commands(
      "pred create --example NAESatisfiability -o nae-satisfiability.json",
      "pred solve nae-satisfiability.json",
      "pred evaluate nae-satisfiability.json --config " + x.optimal_config.map(str).join(","),
    )
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

    #pred-commands(
      "pred create --example " + problem-spec(x) + " -o ksat.json",
      "pred solve ksat.json",
      "pred evaluate ksat.json --config " + x.optimal_config.map(str).join(","),
    )
    ]
  ]
}

#{
  let x = load-model-example("Planar3Satisfiability")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let clauses = x.instance.clauses
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let assign = sol.config
  let fmt-lit(l) = if l > 0 { $x_#l$ } else { $not x_#(-l)$ }
  let fmt-clause(c) = $paren.l #c.literals.map(fmt-lit).join($or$) paren.r$
  let eval-lit(l) = if l > 0 { assign.at(l - 1) } else { 1 - assign.at(-l - 1) }
  [
    #problem-def("Planar3Satisfiability")[
      Given a 3-CNF formula $phi = and.big_(j=1)^m C_j$ with $m$ clauses over $n$ Boolean variables, where each clause $C_j$ contains exactly 3 literals, and the variable-clause incidence graph $H(phi)$ is planar, find a satisfying assignment $bold(x) in {0, 1}^n$.
    ][
    Planar 3-SAT is a restricted variant of 3-SAT introduced by Lichtenstein @lichtenstein1982, who proved it NP-complete. The incidence graph $H(phi)$ is bipartite with variable nodes and clause nodes, connected by edges when a variable appears in a clause. Requiring $H(phi)$ to be planar is a strong structural constraint that enables reductions to geometric and planar problems (e.g., rectilinear Steiner tree, planar vertex cover). The best known algorithm shares the 3-SAT bound of $O^*(1.307^n)$ via biased-PPSZ @hansen2019, since any Planar 3-SAT instance is also a valid 3-SAT instance.

    *Example.* Consider $phi = #clauses.map(fmt-clause).join($and$)$ with $n = #n$ variables and $m = #m$ clauses. The assignment $(#range(n).map(i => $x_#(i + 1)$).join(",")) = (#assign.map(v => str(v)).join(", "))$ satisfies all clauses: #clauses.enumerate().map(((j, c)) => $C_#(j + 1) = paren.l #c.literals.map(l => str(eval-lit(l))).join($or$) paren.r = 1$).join(", ").

    #pred-commands(
      "pred create --example Planar3Satisfiability -o planar3sat.json",
      "pred solve planar3sat.json",
      "pred evaluate planar3sat.json --config " + x.optimal_config.map(str).join(","),
    )
    ]
  ]
}

#{
  let x = load-model-example("OneInThreeSatisfiability")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let clauses = x.instance.clauses
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let assign = sol.config
  let fmt-lit(l) = if l > 0 { $x_#l$ } else { $not x_#(-l)$ }
  let fmt-clause(c) = $paren.l #c.literals.map(fmt-lit).join($or$) paren.r$
  let eval-lit(l) = if l > 0 { assign.at(l - 1) } else { 1 - assign.at(-l - 1) }
  let count-true(c) = c.literals.map(eval-lit).sum()
  [
    #problem-def("OneInThreeSatisfiability")[
      Given a CNF formula $phi = and.big_(j=1)^m C_j$ with $m$ clauses over $n$ Boolean variables, where each clause $C_j$ contains exactly 3 literals, find a truth assignment $bold(x) in {0, 1}^n$ such that each clause has _exactly one_ true literal.
    ][
    One-in-Three Satisfiability (1-in-3 SAT) was introduced by Schaefer @schaefer1978 as part of his dichotomy theorem for generalized satisfiability. Unlike standard 3-SAT which requires at least one true literal per clause, 1-in-3 SAT requires exactly one. The problem is NP-complete even for monotone instances (no negations). The best known algorithm runs in $O^*(1.307^n)$ time via biased-PPSZ @hansen2019, since every 1-in-3 SAT instance reduces trivially to 3-SAT.

    *Example.* Consider $phi = #clauses.map(fmt-clause).join($and$)$ with $n = #n$ variables and $m = #m$ clauses. The assignment $(#range(n).map(i => $x_#(i + 1)$).join(",")) = (#assign.map(v => str(v)).join(", "))$ satisfies the 1-in-3 condition: #clauses.enumerate().map(((j, c)) => $C_#(j + 1)$+ " has " + str(count-true(c)) + " true literal").join(", ").

    #pred-commands(
      "pred create --example OneInThreeSatisfiability -o 1in3sat.json",
      "pred solve 1in3sat.json",
      "pred evaluate 1in3sat.json --config " + x.optimal_config.map(str).join(","),
    )
    ]
  ]
}

#{
  let x = load-model-example("Maximum2Satisfiability")
  let n = x.instance.num_vars
  let m = x.instance.clauses.len()
  let clauses = x.instance.clauses
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let assign = sol.config
  let fmt-lit(l) = if l > 0 { $x_#l$ } else { $not x_#(-l)$ }
  let fmt-clause(c) = $paren.l #c.literals.map(fmt-lit).join($or$) paren.r$
  let eval-lit(l) = if l > 0 { assign.at(l - 1) } else { 1 - assign.at(-l - 1) }
  let clause-sat(c) = c.literals.map(eval-lit).any(v => v == 1)
  let sat-count = clauses.filter(clause-sat).len()
  [
    #problem-def("Maximum2Satisfiability")[
      Given a set $U$ of $n$ Boolean variables and a collection $C = {C_1, dots, C_m}$ of $m$ clauses over $U$ with $|C_j| = 2$ for each $j$, find a truth assignment $bold(x) in {0,1}^n$ that maximizes the number of simultaneously satisfied clauses.
    ][
    Maximum 2-Satisfiability (MAX-2-SAT) is one of the fundamental NP-hard optimization problems. While the decision version of 2-SAT is solvable in linear time by implication-graph analysis, the optimization variant---maximizing the number of satisfied clauses---is NP-hard @garey1979. The best known exact algorithm by Williams @williams2005 runs in $O^*(2^(0.7905n))$ time by reducing to a maximum-weight triangle problem and applying fast matrix multiplication.

    *Example.* Consider $m = #m$ clauses over $n = #n$ variables: $#clauses.map(fmt-clause).join($and$)$. The assignment $(#range(n).map(i => $x_#(i + 1)$).join(",")) = (#assign.map(v => str(v)).join(", "))$ satisfies #sat-count out of #m clauses.

    #pred-commands(
      "pred create --example Maximum2Satisfiability -o max2sat.json",
      "pred solve max2sat.json",
      "pred evaluate max2sat.json --config " + x.optimal_config.map(str).join(","),
    )
    ]
  ]
}

#let max2sat_ilp = load-example("Maximum2Satisfiability", "ILP")
#let max2sat_ilp_sol = max2sat_ilp.solutions.at(0)
#reduction-rule("Maximum2Satisfiability", "ILP",
  example: true,
  example-caption: [$n = #max2sat_ilp.source.instance.num_vars$ variables, $m = #max2sat_ilp.source.instance.clauses.len()$ clauses],
  extra: [
    #pred-commands(
      "pred create --example Maximum2Satisfiability -o max2sat.json",
      "pred reduce max2sat.json --to " + target-spec(max2sat_ilp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate max2sat.json --config " + max2sat_ilp_sol.source_config.map(str).join(","),
    )
    *Step 1 -- Source instance.* The canonical MAX-2-SAT instance has $n = #max2sat_ilp.source.instance.num_vars$ Boolean variables and $m = #max2sat_ilp.source.instance.clauses.len()$ clauses.

    *Step 2 -- Build the binary ILP.* Introduce $n$ binary truth variables $y_0, dots, y_(n-1) in {0,1}$ and $m$ binary clause-indicator variables $z_0, dots, z_(m-1) in {0,1}$. The objective is $ max sum_(j=0)^(m-1) z_j $ subject to one constraint per clause $j$: $z_j <= l_1' + l_2'$ where $l_i' = y_i$ for a positive literal and $l_i' = 1 - y_i$ for a negated literal. The resulting ILP has $n + m = #(max2sat_ilp.source.instance.num_vars + max2sat_ilp.source.instance.clauses.len())$ variables and $m = #max2sat_ilp.source.instance.clauses.len()$ constraints.

    *Step 3 -- Verify a solution.* The ILP optimum extracts the first $n$ variables as the truth assignment $bold(y)^* = (#max2sat_ilp_sol.source_config.map(str).join(", "))$, satisfying #max2sat_ilp_sol.source_config.len() source variables #sym.checkmark.
  ],
)[
  A MAX-2-SAT instance maps directly to a binary ILP @garey1979: each Boolean variable becomes a binary decision variable, each clause gets a binary indicator variable, and a single linear inequality per clause links the indicator to its literals. The objective maximizes the sum of clause indicators, so the ILP optimum equals the maximum number of satisfiable clauses.
][
  _Construction._ Given $n$ Boolean variables and $m$ clauses, introduce binary variables $y_0, dots, y_(n-1) in {0,1}$ (truth assignment) and $z_0, dots, z_(m-1) in {0,1}$ (clause indicators). For each clause $C_j$ with literals $ell_1, ell_2$, define $ell_i' = y_i$ if positive and $ell_i' = 1 - y_i$ if negated. Add the constraint $z_j <= ell_1' + ell_2'$, ensuring $z_j = 1$ only when the clause is satisfied. The ILP is:
  $
    max quad & sum_(j=0)^(m-1) z_j \
    "subject to" quad & z_j <= ell_1' + ell_2' quad forall j in {0, dots, m - 1} \
    & y_i in {0, 1} quad forall i in {0, dots, n - 1} \
    & z_j in {0, 1} quad forall j in {0, dots, m - 1}
  $.
  The target has $n + m$ variables and $m$ constraints.

  _Correctness._ ($arrow.r.double$) Any truth assignment $bold(y)$ satisfying $k$ clauses yields a feasible ILP solution by setting $z_j = 1$ iff clause $j$ is satisfied, achieving objective $k$. ($arrow.l.double$) Any feasible ILP solution with $z_j = 1$ has clause $j$ satisfied by the constraint, so the truth assignment satisfies at least $sum z_j$ clauses. Thus optimal values coincide.

  _Solution extraction._ Return the first $n$ components $(y_0, dots, y_(n-1))$ as the truth assignment.
]

#{
  let x = load-model-example("NonTautology")
  let n = x.instance.num_vars
  let m = x.instance.disjuncts.len()
  let disjuncts = x.instance.disjuncts
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let assign = sol.config
  let fmt-lit(l) = if l > 0 { $x_#l$ } else { $not x_#(-l)$ }
  let fmt-disjunct(d) = $paren.l #d.map(fmt-lit).join($and$) paren.r$
  let eval-lit(l) = if l > 0 { assign.at(l - 1) } else { 1 - assign.at(-l - 1) }
  let disjunct-true(d) = d.map(eval-lit).all(v => v == 1)
  [
    #problem-def("NonTautology")[
      Given a Boolean formula in DNF $phi = or.big_(j=1)^m D_j$ with $m$ disjuncts over $n$ Boolean variables, where each disjunct $D_j$ is a conjunction of literals, find a truth assignment $bold(x) in {0, 1}^n$ such that $phi(bold(x)) = 0$ (i.e., every disjunct is false).
    ][
    The Non-Tautology problem asks whether a given DNF formula is _not_ a tautology, by finding a falsifying assignment. A disjunct $D_j = ell_1 and dots and ell_k$ is false when at least one of its literals evaluates to false; the formula is false when all disjuncts are false. The problem is coNP-complete in general and closely related to SAT through De Morgan duality: a DNF formula $phi$ is a tautology iff $not phi$ (a CNF formula) is unsatisfiable.

    *Example.* Consider $phi = #disjuncts.map(fmt-disjunct).join($or$)$ with $n = #n$ variables and $m = #m$ disjuncts. The assignment $(#range(n).map(i => $x_#(i + 1)$).join(",")) = (#assign.map(v => str(v)).join(", "))$ falsifies the formula: #disjuncts.enumerate().map(((j, d)) => $D_#(j + 1)$+ " is " + if disjunct-true(d) { "true" } else { "false" }).join(", ").

    #pred-commands(
      "pred create --example NonTautology -o nontaut.json",
      "pred solve nontaut.json",
      "pred evaluate nontaut.json --config " + x.optimal_config.map(str).join(","),
    )
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

    #pred-commands(
      "pred create --example CircuitSAT -o circuitsat.json",
      "pred solve circuitsat.json",
      "pred evaluate circuitsat.json --config " + x.optimal_config.map(str).join(","),
    )

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

#problem-def("EnsembleComputation")[
  Given a finite set $A$ and a collection $C$ of subsets of $A$, find the minimum number of union operations in a sequence $S = (z_1 <- x_1 union y_1, z_2 <- x_2 union y_2, dots, z_j <- x_j union y_j)$ such that each operand $x_i, y_i$ is either a singleton ${a}$ for some $a in A$ or a previously computed set $z_k$ with $k < i$, the two operands are disjoint for every step, and every target subset $c in C$ is equal to some computed set $z_i$.
][
  Ensemble Computation is problem PO9 in Garey and Johnson @garey1979. It can be viewed as monotone circuit synthesis over set union: each operation introduces one reusable intermediate set, and the objective is to realize all targets in the fewest operations. The original GJ formulation is a decision problem with a budget parameter $J$; this library models the optimization variant that minimizes the sequence length, using $J$ as a search-space bound. The implementation uses $2J$ operand variables with domain size $|A| + J$ and reports the first step at which all targets are produced. The resulting search space yields a straightforward exact upper bound of $(|A| + J)^(2J)$. Järvisalo, Kaski, Koivisto, and Korhonen study SAT encodings for finding efficient ensemble computations in a monotone-circuit setting @jarvisalo2012.

  *Example.* Let $A = {0, 1, 2, 3}$, $C = {{0, 1, 2}, {0, 1, 3}}$, and $J = 4$. A satisfying witness uses three essential unions:
  $z_1 = {0} union {1} = {0, 1}$,
  $z_2 = z_1 union {2} = {0, 1, 2}$, and
  $z_3 = z_1 union {3} = {0, 1, 3}$.
  Thus both target subsets appear among the computed $z_i$ values while staying within the budget.

  #figure(
    canvas(length: 1cm, {
      import draw: *
      let node(pos, label, name, fill) = {
        rect(
          (pos.at(0) - 0.45, pos.at(1) - 0.18),
          (pos.at(0) + 0.45, pos.at(1) + 0.18),
          radius: 0.08,
          fill: fill,
          stroke: 0.5pt + luma(140),
          name: name,
        )
        content(name, text(7pt, label))
      }

      let base = rgb("#4e79a7").transparentize(78%)
      let target = rgb("#59a14f").transparentize(72%)
      let aux = rgb("#f28e2b").transparentize(74%)

      node((0.0, 1.4), [\{0\}], "a0", base)
      node((1.2, 1.4), [\{1\}], "a1", base)
      node((2.4, 1.4), [\{2\}], "a2", base)
      node((3.6, 1.4), [\{3\}], "a3", base)

      node((0.6, 0.6), [$z_1 = \{0,1\}$], "z1", aux)
      node((1.8, -0.2), [$z_2 = \{0,1,2\}$], "z2", target)
      node((3.0, -0.2), [$z_3 = \{0,1,3\}$], "z3", target)

      line("a0.south", "z1.north-west", stroke: 0.5pt + luma(120), mark: (end: "straight", scale: 0.4))
      line("a1.south", "z1.north-east", stroke: 0.5pt + luma(120), mark: (end: "straight", scale: 0.4))
      line("z1.south-west", "z2.north-west", stroke: 0.5pt + luma(120), mark: (end: "straight", scale: 0.4))
      line("a2.south", "z2.north-east", stroke: 0.5pt + luma(120), mark: (end: "straight", scale: 0.4))
      line("z1.south-east", "z3.north-west", stroke: 0.5pt + luma(120), mark: (end: "straight", scale: 0.4))
      line("a3.south", "z3.north-east", stroke: 0.5pt + luma(120), mark: (end: "straight", scale: 0.4))
    }),
    caption: [An ensemble computation for $A = {0,1,2,3}$ and $C = {{0,1,2}, {0,1,3}}$. The intermediate set $z_1 = {0,1}$ is reused to produce both target subsets.],
  ) <fig:ensemble-computation>
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

    #pred-commands(
      "pred create --example Factoring -o factoring.json",
      "pred solve factoring.json",
      "pred evaluate factoring.json --config " + x.optimal_config.map(str).join(","),
    )
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

    #pred-commands(
      "pred create --example QuantifiedBooleanFormulas -o quantified-boolean-formulas.json",
      "pred solve quantified-boolean-formulas.json",
      "pred evaluate quantified-boolean-formulas.json --config " + x.optimal_config.map(str).join(","),
    )
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
  let dH = metric-value(x.optimal_value)
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

    #pred-commands(
      "pred create --example BMF -o bmf.json",
      "pred solve bmf.json",
      "pred evaluate bmf.json --config " + x.optimal_config.map(str).join(","),
    )

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

    #pred-commands(
      "pred create --example ConsecutiveBlockMinimization -o consecutive-block-minimization.json",
      "pred solve consecutive-block-minimization.json",
      "pred evaluate consecutive-block-minimization.json --config " + x.optimal_config.map(str).join(","),
    )
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
  let num-changes = metric-value(sol.metric)
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

    #pred-commands(
      "pred create --example PaintShop -o paint-shop.json",
      "pred solve paint-shop.json",
      "pred evaluate paint-shop.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let total-size = metric-value(sol.metric)
  [
    #problem-def("BicliqueCover")[
      Given a bipartite graph $G = (L, R, E)$ and integer $k$, find $k$ bicliques $(L_1, R_1), dots, (L_k, R_k)$ that cover all edges ($E subset.eq union.big_i L_i times R_i$) while minimizing the total size $sum_i (|L_i| + |R_i|)$.
    ][
    Biclique Cover is equivalent to factoring the biadjacency matrix $M$ of the bipartite graph as a Boolean sum of rank-1 binary matrices, connecting it to Boolean matrix rank and nondeterministic communication complexity. Applications include data compression, database optimization (covering queries with materialized views), and bioinformatics (gene expression biclustering). NP-hard even for fixed $k >= 2$. The best known algorithm runs in $O^*(2^(|L| + |R|))$ by brute-force enumeration#footnote[No algorithm improving on brute-force enumeration is known for general Biclique Cover.].

    *Example.* Consider $G = (L, R, E)$ with $L = {#range(left-size).map(i => $ell_#(i + 1)$).join(", ")}$, $R = {#range(right-size).map(i => $r_#(i + 1)$).join(", ")}$, and edges $E = {#bip-edges.map(e => $(ell_#(e.at(0) + 1), r_#(e.at(1) + 1))$).join(", ")}$. A biclique cover with $k = #k$: $(L_1, R_1) = ({ell_1}, {r_1, r_2})$ covering edges ${(ell_1, r_1), (ell_1, r_2)}$, and $(L_2, R_2) = ({ell_2}, {r_2, r_3})$ covering ${(ell_2, r_2), (ell_2, r_3)}$. Total size $= (1+2) + (1+2) = #total-size$. Merging into a single biclique is impossible since $(ell_1, r_3) in.not E$.

    #pred-commands(
      "pred create --example BicliqueCover -o biclique-cover.json",
      "pred solve biclique-cover.json",
      "pred evaluate biclique-cover.json --config " + x.optimal_config.map(str).join(","),
    )

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

    #pred-commands(
      "pred create --example BalancedCompleteBipartiteSubgraph -o balanced-complete-bipartite-subgraph.json",
      "pred solve balanced-complete-bipartite-subgraph.json",
      "pred evaluate balanced-complete-bipartite-subgraph.json --config " + x.optimal_config.map(str).join(","),
    )

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

      #pred-commands(
        "pred create --example PartitionIntoTriangles -o partition-into-triangles.json",
        "pred solve partition-into-triangles.json",
        "pred evaluate partition-into-triangles.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("PartitionIntoForests")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let K = x.instance.num_forests
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let classes = range(K).map(c => sol.config.enumerate().filter(((i, v)) => v == c).map(((i, _)) => i))
  [
    #problem-def("PartitionIntoForests")[
      Given an undirected graph $G = (V, E)$ and a positive integer $K$, determine whether the vertex set $V$ can be partitioned into $K$ non-empty classes $V_1, dots, V_K$ such that the subgraph $G[V_i]$ induced by each class $V_i$ is a forest (acyclic graph).
    ][
      Partition Into Forests is NP-complete @garey1979[GT18]. The problem asks whether the vertex set can be split into $K$ classes each inducing an acyclic subgraph; it generalises arboricity decomposition (covering all edges with $K$ forests, solvable in polynomial time) to the decision problem where the partition need not cover all edges. The best known exact algorithm uses brute-force enumeration in $O^*(K^n)$ time.

      *Example.* Consider $G$ with $n = #nv$ vertices and edges #edges.map(((u, v)) => [${#u, #v}$]).join(", "). With $K = #K$, the partition #classes.enumerate().map(((i, c)) => $V_#(i + 1) = {#c.map(v => $v_#v$).join(", ")}$).join(", ") is valid: each induced subgraph is acyclic.

      #pred-commands(
        "pred create --example PartitionIntoForests -o partition-into-forests.json",
        "pred solve partition-into-forests.json",
        "pred evaluate partition-into-forests.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("DegreeConstrainedSpanningTree")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let K = x.instance.max_degree
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let selected-edges = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  [
    #problem-def("DegreeConstrainedSpanningTree")[
      Given an undirected graph $G = (V, E)$ and a positive integer $K$, determine whether $G$ contains a spanning tree $T$ in which every vertex has degree at most $K$.
    ][
      Degree-Constrained Spanning Tree is NP-complete @garey1979[ND1]. The problem generalises the Hamiltonian Path problem (set $K = 2$). The best known exact algorithm uses brute-force enumeration in $O^*(2^n)$ time, where $n = |V|$.

      *Example.* Consider $G$ with $n = #nv$ vertices and $m = #ne$ edges #edges.map(((u, v)) => [${#u, #v}$]).join(", "). With $K = #K$, the spanning tree using edges #selected-edges.map(i => [${#edges.at(i).at(0), #edges.at(i).at(1)}$]).join(", ") has maximum vertex degree $<= #K$.

      #pred-commands(
        "pred create --example DegreeConstrainedSpanningTree -o dcst.json",
        "pred solve dcst.json",
        "pred evaluate dcst.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("BoundedDiameterSpanningTree")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let B = x.instance.weight_bound
  let D = x.instance.diameter_bound
  let ew = x.instance.edge_weights
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let selected-edges = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let total-weight = selected-edges.map(i => ew.at(i)).sum()
  [
    #problem-def("BoundedDiameterSpanningTree")[
      Given an undirected graph $G = (V, E)$ with positive edge weights $w: E -> ZZ_(> 0)$, a weight bound $B$, and a diameter bound $D$, determine whether $G$ contains a spanning tree $T$ such that the total weight $sum_(e in T) w(e) <= B$ and the diameter of $T$ (the longest shortest path in number of edges) is at most $D$.
    ][
      Bounded Diameter Spanning Tree is NP-hard @garey1979[ND3]. The problem asks for a spanning tree that simultaneously satisfies a budget constraint on total edge weight and a structural constraint on tree diameter. The best known exact algorithm uses brute-force enumeration in $O^*(n^n)$ time, where $n = |V|$.

      *Example.* Consider $G$ with $n = #nv$ vertices and $m = #ne$ edges #edges.map(((u, v)) => [${#u, #v}$]).join(", ") with edge weights #edges.enumerate().map(((i, (u, v))) => [$w({#u, #v}) = #(ew.at(i))$]).join(", "). With $B = #B$ and $D = #D$, the spanning tree using edges #selected-edges.map(i => [${#edges.at(i).at(0), #edges.at(i).at(1)}$]).join(", ") has total weight $#total-weight <= #B$ and diameter $<= #D$.

      #pred-commands(
        "pred create --example BoundedDiameterSpanningTree -o bdst.json",
        "pred solve bdst.json",
        "pred evaluate bdst.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MaximumLeafSpanningTree")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let tree-edges = sol.config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let leaf-count = sol.metric
  // compute degrees in the tree
  let degrees = range(nv).map(v => tree-edges.map(i => edges.at(i)).filter(((u, w)) => u == v or w == v).len())
  let leaves = range(nv).filter(v => degrees.at(v) == 1)
  [
    #problem-def("MaximumLeafSpanningTree")[
      Given a connected undirected graph $G = (V, E)$, find a spanning tree $T$ of $G$ that maximizes the number of leaves (degree-1 vertices) in $T$.
    ][
      Maximum Leaf Spanning Tree is NP-hard @garey1979[ND2]. The problem has applications in network design and broadcasting, where maximizing the number of leaf nodes reduces the set of internal (relay) nodes. The best known exact algorithm runs in $O^*(1.8966^n)$ time, where $n = |V|$.#footnote[H. Fernau, J. Kneis, D. Kratsch, A. Langer, M. Liedloff, D. Raible, and P. Rossmanith, An exact algorithm for the Maximum Leaf Spanning Tree problem, _Theoretical Computer Science_, 412(45):6290--6302, 2011.]

      *Example.* Consider $G$ with $n = #nv$ vertices and $m = #ne$ edges #edges.map(((u, v)) => [${#u, #v}$]).join(", "). The spanning tree using edges #tree-edges.map(i => [${#edges.at(i).at(0), #edges.at(i).at(1)}$]).join(", ") has #leaf-count leaves (vertices #leaves.map(v => [$#v$]).join(", ")), each with degree 1 in the tree.

      #pred-commands(
        "pred create --example MaximumLeafSpanningTree -o mlst.json",
        "pred solve mlst.json",
        "pred evaluate mlst.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#reduction-rule("MaximumLeafSpanningTree", "ILP")[
  An MLST instance reduces to an integer linear program with $3m + n$ variables and $3n + 2m + 1$ constraints, using a single-commodity flow formulation to enforce spanning-tree connectivity and binary leaf indicators to encode the objective @garey1979.
][
  _Construction._ Given $G = (V, E)$ with $n = |V|$, $m = |E|$, root the flow at vertex $0$. Introduce binary edge selectors $y_e in {0, 1}$ for each $e in E$ ($m$ variables), binary leaf indicators $z_v in {0, 1}$ for each $v in V$ ($n$ variables), and directed flow variables $f_(u v), f_(v u) >= 0$ for each undirected edge ${u, v}$ ($2m$ variables).

  _Constraints:_
  + *Tree cardinality:* $sum_e y_e = n - 1$.
  + *Flow conservation:* for the root, net outflow $= n - 1$; for each non-root vertex $v$, net inflow $= 1$.
  + *Flow--edge linking:* $f_(u v) + f_(v u) <= (n - 1) y_e$ for each edge $e = {u, v}$.
  + *Leaf detection:* $sum_(e in.rev v) y_e + (n - 2) z_v <= n - 1$ for each vertex $v$.
  + *Binary bounds:* $y_e <= 1$, $z_v <= 1$.

  The objective is $max sum_(v in V) z_v$.

  _Correctness._ ($arrow.r.double$) A spanning tree $T$ of $G$ with $ell$ leaves induces a feasible ILP solution: route one unit of flow from the root to every other vertex along the unique tree path, set $y_e = 1$ for tree edges, and set $z_v = 1$ for degree-1 vertices; constraint (4) is tight when $deg_T(v) = 1$ and slack otherwise, achieving objective $ell$. ($arrow.l.double$) Any feasible ILP solution with $sum y_e = n - 1$ and connectivity enforced by the flow yields a spanning tree; constraint (4) forces $z_v = 0$ whenever $deg_T(v) > 1$, and maximization ensures $z_v = 1$ for all leaves.

  _Solution extraction._ Return the edge-selector prefix $(y_0, dots, y_(m-1))$ as the source configuration.
]

#{
  let x = load-model-example("MonochromaticTriangle")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  [
    #problem-def("MonochromaticTriangle")[
      Given an undirected graph $G = (V, E)$, determine whether the edges of $G$ can be 2-colored (each edge assigned color 0 or 1) so that no triangle is monochromatic — that is, for every three mutually adjacent vertices $u, v, w$, the three edges ${u, v}$, ${u, w}$, ${v, w}$ do not all receive the same color.
    ][
      Monochromatic Triangle is closely related to Ramsey theory. By the classical result $R(3, 3) = 6$, the complete graph $K_6$ admits no valid 2-coloring, while $K_5$ does. The problem is NP-complete in general @garey1979[GT6]. The best known exact algorithm uses brute-force enumeration in $O^*(2^m)$ time, where $m = |E|$.

      *Example.* Consider $K_4$ with $n = #nv$ vertices and $m = #ne$ edges. A valid coloring assigns colors #sol.config.enumerate().map(((i, c)) => [edge #i $arrow.r$ #c]).join(", "), so that no triangle has all three edges the same color.

      #pred-commands(
        "pred create --example MonochromaticTriangle -o monochromatic-triangle.json",
        "pred solve monochromatic-triangle.json",
        "pred evaluate monochromatic-triangle.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("PartitionIntoCliques")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let K = x.instance.num_cliques
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let groups = range(K).map(c => sol.config.enumerate().filter(((i, v)) => v == c).map(((i, _)) => i))
  [
    #problem-def("PartitionIntoCliques")[
      Given an undirected graph $G = (V, E)$ and a positive integer $K <= |V|$, determine whether the vertex set $V$ can be partitioned into $k <= K$ groups $V_1, dots, V_k$ such that each group $V_i$ induces a complete subgraph (clique) in $G$.
    ][
      Partition Into Cliques is NP-complete @garey1979[GT15]. The problem is the complement of Graph Coloring: a valid clique cover of $G$ corresponds to a valid coloring of the complement graph $overline(G)$. The best known exact algorithm uses brute-force enumeration in $O^*(2^n)$ time.

      *Example.* Consider $G$ with $n = #nv$ vertices and edges #edges.map(((u, v)) => [${#u, #v}$]).join(", "). With $K = #K$, the partition #groups.enumerate().filter(((_, c)) => c.len() > 0).map(((i, c)) => $V_#(i + 1) = {#c.map(v => $v_#v$).join(", ")}$).join(", ") is valid: each group induces a clique.

      #pred-commands(
        "pred create --example PartitionIntoCliques -o partition-into-cliques.json",
        "pred solve partition-into-cliques.json",
        "pred evaluate partition-into-cliques.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("PartitionIntoPerfectMatchings")
  let nv = graph-num-vertices(x.instance)
  let ne = graph-num-edges(x.instance)
  let edges = x.instance.graph.edges
  let K = x.instance.num_matchings
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let groups = range(K).map(c => sol.config.enumerate().filter(((i, v)) => v == c).map(((i, _)) => i))
  [
    #problem-def("PartitionIntoPerfectMatchings")[
      Given an undirected graph $G = (V, E)$ and a positive integer $K <= |V|$, determine whether the vertex set $V$ can be partitioned into $k <= K$ groups $V_1, dots, V_k$ such that the subgraph induced by each group $V_i$ is a perfect matching: every vertex in $V_i$ has exactly one neighbor within $V_i$. Empty groups are permitted.
    ][
      Partition Into Perfect Matchings is NP-complete @garey1979[GT16]. The problem asks whether the edges of a graph can be decomposed into perfect matchings of vertex-induced subgraphs. The best known exact algorithm uses brute-force enumeration in $O^*(K^n)$ time.

      *Example.* Consider $G$ with $n = #nv$ vertices and edges #edges.map(((u, v)) => [${#u, #v}$]).join(", "). With $K = #K$, the partition #groups.enumerate().filter(((_, c)) => c.len() > 0).map(((i, c)) => $V_#(i + 1) = {#c.map(v => $v_#v$).join(", ")}$).join(", ") is valid: each group induces a perfect matching.

      #pred-commands(
        "pred create --example PartitionIntoPerfectMatchings -o partition-into-perfect-matchings.json",
        "pred solve partition-into-perfect-matchings.json",
        "pred evaluate partition-into-perfect-matchings.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("BinPacking")
  let sizes = x.instance.sizes
  let n = sizes.len()
  let C = x.instance.capacity
  let config = x.optimal_config
  let num-bins = metric-value(x.optimal_value)
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

      #pred-commands(
        "pred create --example BinPacking -o bin-packing.json",
        "pred solve bin-packing.json",
        "pred evaluate bin-packing.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let opt-val = metric-value(x.optimal_value)
  let selected = range(n).filter(i => config.at(i) == 1)
  let total-w = selected.map(i => weights.at(i)).sum()
  let total-v = selected.map(i => values.at(i)).sum()
  [
    #problem-def("Knapsack")[
      Given $n$ items with weights $w_0, dots, w_(n-1) in NN$ and values $v_0, dots, v_(n-1) in NN$, and a capacity $C in NN$, find $S subset.eq {0, dots, n - 1}$ maximizing $sum_(i in S) v_i$ subject to $sum_(i in S) w_i lt.eq C$.
    ][
      One of Karp's 21 NP-complete problems @karp1972. Knapsack is only _weakly_ NP-hard: a classical dynamic-programming algorithm runs in $O(n C)$ pseudo-polynomial time, and a fully polynomial-time approximation scheme (FPTAS) achieves $(1 - epsilon)$-optimal value in $O(n^2 slash epsilon)$ time @ibarra1975. The special case $v_i = w_i$ for all $i$ is the Subset Sum problem. Knapsack is also a special case of Integer Linear Programming with a single constraint. The best known exact algorithm is the $O^*(2^(n slash 2))$ meet-in-the-middle approach of Horowitz and Sahni @horowitz1974, which partitions items into two halves and combines sorted sublists.

      *Example.* Let $n = #n$ items with weights $(#weights.map(w => str(w)).join(", "))$, values $(#values.map(v => str(v)).join(", "))$, and capacity $C = #C$. Selecting $S = {#selected.map(i => str(i)).join(", ")}$ gives total weight $#total-w lt.eq C$ and total value $#total-v$, which is optimal.

      #pred-commands(
        "pred create --example Knapsack -o knapsack.json",
        "pred solve knapsack.json",
        "pred evaluate knapsack.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("IntegerKnapsack")
  let sizes = x.instance.sizes
  let values = x.instance.values
  let B = x.instance.capacity
  let n = sizes.len()
  let config = x.optimal_config
  let opt-val = metric-value(x.optimal_value)
  let total-s = range(n).map(i => config.at(i) * sizes.at(i)).sum()
  let total-v = range(n).map(i => config.at(i) * values.at(i)).sum()
  [
    #problem-def("IntegerKnapsack")[
      Given $n$ items with sizes $s_0, dots, s_(n-1) in ZZ^+$ and values $v_0, dots, v_(n-1) in ZZ^+$, and a capacity $B in NN$, find non-negative integer multiplicities $c_0, dots, c_(n-1) in NN$ maximizing $sum_(i=0)^(n-1) c_i dot v_i$ subject to $sum_(i=0)^(n-1) c_i dot s_i lt.eq B$.
    ][
      The Integer Knapsack (also called the _unbounded knapsack problem_) generalizes the 0-1 Knapsack by allowing each item to be selected more than once. Like 0-1 Knapsack, it admits a pseudo-polynomial $O(n B)$ dynamic-programming algorithm @garey1979. The problem is weakly NP-hard: when item sizes are bounded by a polynomial in $n$, DP runs in polynomial time. The brute-force approach enumerates all multiplicity vectors, giving $O(product_(i=0)^(n-1)(floor.l B slash s_i floor.r + 1))$ configurations.#footnote[No algorithm improving on brute-force enumeration of multiplicity vectors is known for the general Integer Knapsack problem.]

      *Example.* Let $n = #n$ items with sizes $(#sizes.map(s => str(s)).join(", "))$, values $(#values.map(v => str(v)).join(", "))$, and capacity $B = #B$. Setting multiplicities $(#config.map(c => str(c)).join(", "))$ gives total size $#total-s lt.eq B$ and total value $#total-v$, which is optimal.

      #pred-commands(
        "pred create --example IntegerKnapsack -o ik.json",
        "pred solve ik.json",
        "pred evaluate ik.json --config " + x.optimal_config.map(str).join(","),
      )
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

    #pred-commands(
      "pred create --example RectilinearPictureCompression -o rectilinear-picture-compression.json",
      "pred solve rectilinear-picture-compression.json",
      "pred evaluate rectilinear-picture-compression.json --config " + x.optimal_config.map(str).join(","),
    )

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
  let x = load-model-example("FeasibleRegisterAssignment")
  let n = x.instance.num_vertices
  let arcs = x.instance.arcs
  let K = x.instance.num_registers
  let asgn = x.instance.assignment
  let sigma = x.optimal_config
  let order = range(n).map(pos =>
    range(n).find(v => sigma.at(v) == pos)
  )
  [
    #problem-def("FeasibleRegisterAssignment")[
      Given a directed acyclic graph $G = (V, A)$ with $n = |V|$ vertices, where each arc $(v, u) in A$ means vertex $v$ depends on vertex $u$, $K$ registers, and a register assignment $f: V arrow {0, dots, K-1}$, determine whether there exists a topological evaluation ordering such that no register conflict arises: when a vertex $v$ is evaluated and placed in register $f(v)$, no other vertex currently occupying register $f(v)$ still has uncomputed dependents.
    ][
      Feasible Register Assignment is NP-complete @bouchez2006. It is closely related to Register Sufficiency (SS19 in Garey & Johnson @garey1979), but here the register assignment is given and only the scheduling order is sought.

      *Example.* Let $n = #n$ vertices with arcs: #{arcs.map(a => $v_#(a.at(0)) arrow.r v_#(a.at(1))$).join(", ")}. Registers $K = #K$, assignment $f = (#asgn.map(r => str(r)).join(", "))$. The evaluation order $(#order.map(v => $v_#v$).join(", "))$ is feasible.

      #pred-commands(
        "pred create --example FeasibleRegisterAssignment -o feasible-register-assignment.json",
        "pred solve feasible-register-assignment.json --solver brute-force",
        "pred evaluate feasible-register-assignment.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumRegisterSufficiencyForLoops")
  let N = x.instance.loop_length
  let vars = x.instance.variables
  let nv = vars.len()
  let config = x.optimal_config
  let num-regs = config.dedup().len()
  [
    #problem-def("MinimumRegisterSufficiencyForLoops")[
      Given a loop of length $N$ (representing $N$ timesteps arranged in a circle) and a set of $n$ variables, each active during a contiguous circular arc of timesteps specified by $(s_i, l_i)$ covering timesteps ${s_i, s_i + 1, dots, s_i + l_i - 1} mod N$, assign a register $r_i in {0, dots, n-1}$ to each variable minimizing the number of distinct registers used, such that no two variables with overlapping arcs share the same register.
    ][
      Minimum Register Sufficiency for Loops is problem SS20 in Garey & Johnson @garey1979. It is equivalent to minimum coloring of circular arc graphs. NP-complete via reduction from Chromatic Number. No algorithm improving on brute-force $O(n^n)$ enumeration is known for arbitrary circular arc instances.

      *Example.* Let $N = #N$ timesteps and $n = #nv$ variables with arcs: #vars.enumerate().map(((i, v)) => $x_#i: [#(v.at(0)), #(v.at(0)) + #(v.at(1)))$).join(", ") mod $#N$. All pairs of arcs overlap (each arc covers half the circle and any two arcs share at least one timestep), forming a complete conflict graph $K_#nv$. The assignment $(#config.map(str).join(", "))$ uses #num-regs distinct registers, which is optimal.

      #pred-commands(
        "pred create --example MinimumRegisterSufficiencyForLoops -o mrsfl.json",
        "pred solve mrsfl.json --solver brute-force",
        "pred evaluate mrsfl.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("RegisterSufficiency")
  let n = x.instance.num_vertices
  let arcs = x.instance.arcs
  let K = x.instance.bound
  let sigma = x.optimal_config
  // Build evaluation order (position -> vertex)
  let order = range(n).map(pos =>
    range(n).find(v => sigma.at(v) == pos)
  )
  [
    #problem-def("RegisterSufficiency")[
      Given a directed acyclic graph $G = (V, A)$ with $n = |V|$ vertices, where each arc $(v, u) in A$ means vertex $v$ depends on vertex $u$, and a positive integer $K$, determine whether there exists an evaluation ordering $v_(pi(0)), v_(pi(1)), dots, v_(pi(n-1))$ of all vertices such that the computation can be performed using at most $K$ registers. A value must reside in a register from the moment it is computed until all vertices that depend on it have been evaluated.
    ][
      Register Sufficiency is problem SS19 (also A11 PO1) in Garey & Johnson @garey1979. NP-complete via reduction from 3-SAT @sethi1975. Remains NP-complete even when all vertices have out-degree at most 2. For expression trees (DAGs with tree structure), the Sethi--Ullman algorithm solves the problem in $O(n)$ time @sethiUllman1970. For general DAGs, Kessler's dynamic programming over register states runs in $O(n^2 dot 2^n)$ time @kessler1998.

      *Example.* Let $n = #n$ vertices with arcs (dependencies): #{arcs.map(a => $v_#(a.at(0)) arrow.r v_#(a.at(1))$).join(", ")}. Bound $K = #K$. The evaluation order $pi = (#order.map(v => $v_#v$).join(", "))$ achieves a maximum of $#K$ registers.

      #pred-commands(
        "pred create --example RegisterSufficiency -o register-sufficiency.json",
        "pred solve register-sufficiency.json --solver brute-force",
        "pred evaluate register-sufficiency.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumCodeGenerationOneRegister")
  let n = x.instance.num_vertices
  let edges = x.instance.edges
  let num-leaves = x.instance.num_leaves
  let num-internal = n - num-leaves
  let config = x.optimal_config
  // Build evaluation order: position -> internal vertex
  // internal vertices are those with out-degree > 0
  let out-deg = range(n).map(v => edges.filter(e => e.at(0) == v).len())
  let internal = range(n).filter(v => out-deg.at(v) > 0)
  let order = range(num-internal).map(pos =>
    range(num-internal).find(i => config.at(i) == pos)
  ).map(i => internal.at(i))
  [
    #problem-def("MinimumCodeGenerationOneRegister")[
      Given a directed acyclic graph $G = (V, A)$ with maximum out-degree 2 representing an expression DAG, where leaf vertices (out-degree 0) are input values stored in memory, internal vertices are operations, and root vertices (in-degree 0) are the values to compute, find a program of minimum number of instructions for a one-register machine (supporting LOAD, STORE, and OP instructions) that computes all root vertices.
    ][
      Minimum Code Generation on a One-Register Machine is problem A11 PO4 in Garey & Johnson @garey1979. NP-complete via transformation from 3-Satisfiability @brunoSethi1976. Remains NP-complete even when the only vertices with in-degree greater than 1 have arcs only to leaves. For directed forests (expression trees), the Sethi--Ullman algorithm finds an optimal instruction sequence in $O(n)$ time @sethiUllman1970. For general DAGs, brute-force enumeration of all valid evaluation orderings runs in $O^*(2^n)$ time.
      #footnote[No algorithm improving on brute-force is known for general expression DAGs.]

      *Example.* Consider $n = #n$ vertices with arcs: #{edges.map(a => $v_#(a.at(0)) arrow.r v_#(a.at(1))$).join(", ")}. Leaves (out-degree 0): $\{#(range(n).filter(v => out-deg.at(v) == 0).map(v => $v_#v$).join($, $))\}$. The evaluation order $(#order.map(v => $v_#v$).join(", "))$ yields an optimal program of #x.optimal_value instructions.

      #pred-commands(
        "pred create --example MinimumCodeGenerationOneRegister -o mcgor.json",
        "pred solve mcgor.json --solver brute-force",
        "pred evaluate mcgor.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumCodeGenerationUnlimitedRegisters")
  let n = x.instance.num_vertices
  let left-arcs = x.instance.left_arcs
  let right-arcs = x.instance.right_arcs
  // Build out-degree to identify leaves vs internal
  let out-deg = range(n).map(v =>
    left-arcs.filter(e => e.at(0) == v).len() + right-arcs.filter(e => e.at(0) == v).len()
  )
  let internal = range(n).filter(v => out-deg.at(v) > 0)
  let num-internal = internal.len()
  let config = x.optimal_config
  let order = range(num-internal).map(pos =>
    range(num-internal).find(i => config.at(i) == pos)
  ).map(i => internal.at(i))
  [
    #problem-def("MinimumCodeGenerationUnlimitedRegisters")[
      Given a directed acyclic graph $G = (V, A)$ with maximum out-degree 2 representing an expression DAG, and a partition of arcs into left ($L$) and right ($R$) operand sets, find a program of minimum number of instructions for an unlimited-register machine using 2-address instructions (OP and LOAD/copy) that computes all root vertices. The OP instruction computes a vertex and overwrites the left operand's register; a LOAD instruction copies a register to preserve a value before destruction.
    ][
      Minimum Code Generation with Unlimited Registers is problem A11 PO5 in Garey & Johnson @garey1979. NP-complete via transformation from Feedback Vertex Set @ahoJohnsonUllman1977. Remains NP-complete even if the only vertices with in-degree greater than 1 are leaves. Polynomial for forests and when 3-address instructions are allowed. For general DAGs, brute-force enumeration runs in $O^*(2^n)$ time.
      #footnote[No algorithm improving on brute-force is known for general expression DAGs with 2-address instructions.]

      *Example.* Consider $n = #n$ vertices with left arcs $L$: #{left-arcs.map(a => $v_#(a.at(0)) arrow.r v_#(a.at(1))$).join(", ")} and right arcs $R$: #{right-arcs.map(a => $v_#(a.at(0)) arrow.r v_#(a.at(1))$).join(", ")}. Leaves (out-degree 0): $\{#(range(n).filter(v => out-deg.at(v) == 0).map(v => $v_#v$).join($, $))\}$. The evaluation order $(#order.map(v => $v_#v$).join(", "))$ yields an optimal program of #x.optimal_value instructions.

      #pred-commands(
        "pred create --example MinimumCodeGenerationUnlimitedRegisters -o mcgur.json",
        "pred solve mcgur.json --solver brute-force",
        "pred evaluate mcgur.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumCodeGenerationParallelAssignments")
  let nv = x.instance.num_variables
  let assigns = x.instance.assignments
  let m = assigns.len()
  let config = x.optimal_config
  // Build execution order: order[pos] = assignment index
  let order = range(m).map(pos =>
    range(m).find(i => config.at(i) == pos)
  )
  // Count backward deps for the optimal ordering
  [
    #problem-def("MinimumCodeGenerationParallelAssignments")[
      Given a set $V$ of variables and a collection of simultaneous assignments $A_i: v_i arrow.l op(B_i)$ where $v_i in V$ is the target variable and $B_i subset.eq V$ is the set of variables read, find a permutation $pi$ of the assignments that minimizes the number of backward dependencies. A backward dependency occurs when $v_(pi(i)) in B_(pi(j))$ for some $j > i$, i.e., assignment $pi(i)$ overwrites a variable that a later assignment $pi(j)$ still needs to read.
    ][
      Minimum Code Generation for Parallel Assignments is problem A11 PO6 in Garey & Johnson @garey1979. NP-complete via transformation from Feedback Vertex Set @sethi1975. Remains NP-complete even when $|B_i| lt.eq 2$ for all assignments. For general instances, brute-force enumeration of all permutations runs in $O^*(2^m)$ time via dynamic programming over subsets.
      #footnote[No algorithm improving on brute-force is known for general parallel assignment instances.]

      *Example.* Consider $#nv$ variables and $#m$ assignments: #{assigns.enumerate().map(((i, a)) => {
        let target = a.at(0)
        let reads = a.at(1)
        $A_#i: v_#target arrow.l op({#reads.map(r => $v_#r$).join($, $)})$
      }).join(", ")}. The execution order $(#order.map(i => $A_#i$).join(", "))$ yields #x.optimal_value backward dependencies.

      #pred-commands(
        "pred create --example MinimumCodeGenerationParallelAssignments -o mcgpa.json",
        "pred solve mcgpa.json --solver brute-force",
        "pred evaluate mcgpa.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumDecisionTree")
  let n = x.instance.num_objects
  let m = x.instance.num_tests
  let sol = x.optimal_config
  let tepl = x.optimal_value
  [
    #problem-def("MinimumDecisionTree")[
      Given a set $S$ of $n$ objects and $m$ binary tests $T_1, dots, T_m$ where each test maps $S$ to ${0,1}$ and every pair of objects is distinguished by at least one test, find a decision tree using tests from $T$ that identifies each object and minimizes the total external path length (sum of leaf depths).
    ][
    Minimum Decision Tree (MS15) models optimal test sequencing for object identification @garey1979. NP-hard even when each test has at most 3 positive objects (Hyafil and Rivest 1976, via reduction from Exact Cover by 3-Sets). The Sethi--Ullman algorithm solves the tree (forest) case in polynomial time. The brute-force bound is $O^*(m^n)$.

    *Example.* Consider $n = #n$ objects and $m = #m$ tests. The optimal decision tree has total external path length #tepl, achieved by a balanced split at the root.

    #pred-commands(
      "pred create --example MinimumDecisionTree -o mdt.json",
      "pred solve mdt.json",
      "pred evaluate mdt.json --config " + sol.map(str).join(","),
    )
    ]
  ]
}

#{
  let x = load-model-example("MinimumDisjunctiveNormalForm")
  let n = x.instance.num_variables
  let num_pi = x.instance.prime_implicants.len()
  let num_mt = x.instance.minterms.len()
  [
    #problem-def("MinimumDisjunctiveNormalForm")[
      Given $n$ Boolean variables and a Boolean function $f: {0,1}^n -> {0,1}$ specified by its truth table, find a disjunctive normal form (DNF) formula with the minimum number of terms (disjuncts) that is equivalent to $f$.
    ][
    Minimum Disjunctive Normal Form (LO9) is the classic two-level logic minimization problem @garey1979. NP-hard (Masek 1979, via reduction from Minimum Cover). The Quine--McCluskey algorithm enumerates all prime implicants, reducing the problem to minimum set cover. The worst-case number of prime implicants is $Theta(3^n slash sqrt(n))$ (Chandra and Markowsky 1978).

    *Example.* A function on $n = #n$ variables with #num_mt minterms has #num_pi prime implicants. The minimum cover requires #x.optimal_value terms.

    #pred-commands(
      "pred create --example MinimumDisjunctiveNormalForm -o mdnf.json",
      "pred solve mdnf.json",
    )
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
  let config = x.optimal_config
  // Selected edges (multiplicity >= 1)
  let selected = range(ne).filter(i => config.at(i) >= 1)
  let total-cost = selected.map(i => config.at(i) * edge-lengths.at(i)).sum()
  [
    #problem-def("RuralPostman")[
      Given an undirected graph $G = (V, E)$ with edge lengths $l: E -> ZZ_(gt.eq 0)$ and a subset $E' subset.eq E$ of required edges, find a circuit (closed walk) in $G$ that traverses every edge in $E'$ and has minimum total length.
    ][
      The Rural Postman Problem (RPP) is a fundamental NP-complete arc-routing problem @lenstra1976 that generalizes the Chinese Postman Problem. When $E' = E$, the problem reduces to finding an Eulerian circuit with minimum augmentation (polynomial-time solvable via $T$-join matching). For general $E' subset.eq E$, exact algorithms use dynamic programming over subsets of required edges in $O(n^2 dot 2^r)$ time, where $r = |E'|$ and $n = |V|$, analogous to the Held-Karp algorithm for TSP. The problem admits a $3 slash 2$-approximation for metric instances @frederickson1979.

      *Example.* Consider a graph with #nv vertices and #ne edges, where #(ne - 2) outer edges have length 1 and 2 diagonal edges have length 2. The required edges are $E' = {#required.map(i => {let e = edges.at(i); $(v_#(e.at(0)), v_#(e.at(1)))$}).join($,$)}$. The outer cycle #range(nv).map(i => $v_#i$).join($->$)$-> v_0$ covers all #nr required edges with minimum total length #total-cost.

      #pred-commands(
        "pred create --example RuralPostman -o rural-postman.json",
        "pred solve rural-postman.json",
        "pred evaluate rural-postman.json --config " + x.optimal_config.map(str).join(","),
      )

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
        caption: [Rural Postman instance: #nv vertices, #ne edges, #nr required edges (red, bold). The outer cycle (blue + red edges) has minimum total cost #total-cost, covering all required edges.],
      ) <fig:rural-postman>
    ]
  ]
}

#{
  let x = load-model-example("MixedChinesePostman", variant: (weight: "i32"))
  let nv = x.instance.graph.num_vertices
  let arcs = x.instance.graph.arcs
  let edges = x.instance.graph.edges
  let arc-weights = x.instance.arc_weights
  let edge-weights = x.instance.edge_weights
  let config = x.optimal_config
  let oriented = edges.enumerate().map(((i, e)) => if config.at(i) == 0 { e } else { (e.at(1), e.at(0)) })
  let base-cost = arc-weights.sum() + edge-weights.sum()
  let total-cost = x.optimal_value
  [
    #problem-def("MixedChinesePostman")[
      Given a mixed graph $G = (V, A, E)$ with directed arcs $A$, undirected edges $E$, and integer lengths $l(e) >= 0$ for every $e in A union E$, find a closed walk in $G$ that traverses every arc in its prescribed direction and every undirected edge at least once in some direction, minimizing total length.
    ][
      Mixed Chinese Postman is the mixed-graph arc-routing problem ND25 in Garey and Johnson @garey1979. Papadimitriou proved the mixed case NP-complete even when all lengths are 1, the graph is planar, and the maximum degree is 3 @papadimitriou1976edge. In contrast, the pure undirected and pure directed cases are polynomial-time solvable via matching / circulation machinery @edmondsjohnson1973. The implementation here uses one binary variable per undirected edge orientation, so the search space contributes the $2^|E|$ factor visible in the registered exact bound.

      *Example.* Consider the instance on #nv vertices with directed arcs $(v_0, v_1)$, $(v_1, v_2)$, $(v_2, v_3)$, $(v_3, v_0)$ of lengths $2, 3, 1, 4$ and undirected edges $\{v_0, v_2\}$, $\{v_1, v_3\}$, $\{v_0, v_4\}$, $\{v_4, v_2\}$ of lengths $2, 3, 1, 2$. The config $(#config.map(str).join(", "))$ orients those edges as $(v_2, v_0)$, $(v_3, v_1)$, $(v_0, v_4)$, and $(v_4, v_2)$, producing a strongly connected digraph. The base traversal cost is #base-cost, and the minimum balancing cost brings the total to #total-cost.

      #pred-commands(
        "pred create --example MixedChinesePostman/i32 -o mixed-chinese-postman.json",
        "pred solve mixed-chinese-postman.json --solver brute-force",
        "pred evaluate mixed-chinese-postman.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let positions = (
            (-1.25, 0.85),
            (1.25, 0.85),
            (1.25, -0.85),
            (-1.25, -0.85),
            (0.25, 0.0),
          )

          for (idx, (u, v)) in arcs.enumerate() {
            line(
              positions.at(u),
              positions.at(v),
              stroke: 0.8pt + luma(80),
              mark: (end: "straight", scale: 0.45),
            )
            let mid = (
              (positions.at(u).at(0) + positions.at(v).at(0)) / 2,
              (positions.at(u).at(1) + positions.at(v).at(1)) / 2,
            )
            content(
              mid,
              text(6pt, fill: luma(40))[#arc-weights.at(idx)],
              fill: white,
              frame: "rect",
              padding: 0.04,
              stroke: none,
            )
          }

          for (idx, (u, v)) in oriented.enumerate() {
            line(
              positions.at(u),
              positions.at(v),
              stroke: 1.3pt + graph-colors.at(0),
              mark: (end: "straight", scale: 0.5),
            )
            let mid = (
              (positions.at(u).at(0) + positions.at(v).at(0)) / 2,
              (positions.at(u).at(1) + positions.at(v).at(1)) / 2,
            )
            let offset = if idx == 0 { (-0.18, 0.12) } else if idx == 1 { (0.18, 0.12) } else if idx == 2 { (-0.12, -0.1) } else { (0.12, -0.1) }
            content(
              (mid.at(0) + offset.at(0), mid.at(1) + offset.at(1)),
              text(6pt, fill: graph-colors.at(0))[#edge-weights.at(idx)],
              fill: white,
              frame: "rect",
              padding: 0.04,
              stroke: none,
            )
          }

          for (i, pos) in positions.enumerate() {
            circle(pos, radius: 0.18, fill: white, stroke: 0.6pt + black)
            content(pos, text(7pt)[$v_#i$])
          }
        }),
        caption: [Mixed Chinese Postman example. Gray arrows are the original directed arcs, while blue arrows are the chosen orientations of the former undirected edges under config $(#config.map(str).join(", "))$. The optimal walk has total cost #total-cost.],
      ) <fig:mixed-chinese-postman>
    ]
  ]
}

#{
  let x = load-model-example("StackerCrane")
  let arcs = x.instance.arcs.map(a => (a.at(0), a.at(1)))
  let edges = x.instance.edges.map(e => (e.at(0), e.at(1)))
  let config = x.optimal_config
  let positions = (
    (-2.0, 0.9),
    (-2.0, -0.9),
    (0.0, -1.5),
    (2.0, -0.9),
    (0.0, 1.5),
    (2.0, 0.9),
  )
  [
    #problem-def("StackerCrane")[
      Given a mixed graph $G = (V, A, E)$ with directed arcs $A$, undirected edges $E$, and nonnegative lengths $l: A union E -> ZZ_(gt.eq 0)$, find a closed walk in $G$ that traverses every arc in $A$ in its prescribed direction and has minimum total length.
    ][
      Stacker Crane is the mixed-graph arc-routing problem listed as ND26 in Garey and Johnson @garey1979. Frederickson, Hecht, and Kim prove the problem NP-complete via reduction from Hamiltonian Circuit and give the classical $9 slash 5$-approximation for the metric case @frederickson1978routing. The problem stays difficult even on trees @fredericksonguan1993. The standard Held-Karp-style dynamic program over (current vertex, covered-arc subset) runs in $O(|V|^2 dot 2^|A|)$ time#footnote[Included as a straightforward exact dynamic-programming baseline over subsets of required arcs; no sharper exact bound was independently verified while preparing this entry.].

      A configuration is a permutation of the required arcs, interpreted as the order in which those arcs are forced into the tour. The verifier traverses each chosen arc, then inserts the shortest available connector path from that arc's head to the tail of the next arc, wrapping around at the end to close the walk.

      *Example.* The canonical instance has 6 vertices, 5 required arcs, and 7 undirected edges. The optimal configuration $[#config.map(str).join(", ")]$ orders the required arcs as $a_0, a_2, a_1, a_4, a_3$. Traversing those arcs contributes 17 units of required-arc length, and the shortest connector paths contribute $1 + 1 + 1 + 0 + 0 = 3$, so the resulting closed walk has minimum total length $20$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o stacker-crane.json",
        "pred solve stacker-crane.json --solver brute-force",
        "pred evaluate stacker-crane.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let gray = luma(200)

          for (u, v) in edges {
            line(positions.at(u), positions.at(v), stroke: (paint: gray, thickness: 0.7pt))
          }

          for (i, (u, v)) in arcs.enumerate() {
            line(positions.at(u), positions.at(v), stroke: (paint: blue, thickness: 1.7pt))
            let mid = (
              (positions.at(u).at(0) + positions.at(v).at(0)) / 2,
              (positions.at(u).at(1) + positions.at(v).at(1)) / 2,
            )
            content(mid, text(6pt, fill: blue)[$a_#i$], fill: white, frame: "rect", padding: 0.05, stroke: none)
          }

          for (i, pos) in positions.enumerate() {
            circle(pos, radius: 0.18, fill: white, stroke: 0.6pt + black)
            content(pos, text(7pt)[$v_#i$])
          }
        }),
        caption: [Stacker Crane hourglass instance. Required directed arcs are shown in blue and labeled $a_0$ through $a_4$; undirected connector edges are gray. The optimal order $a_0, a_2, a_1, a_4, a_3$ yields minimum total length 20.],
      ) <fig:stacker-crane>
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

      #pred-commands(
        "pred create --example SubgraphIsomorphism -o subgraph-isomorphism.json",
        "pred solve subgraph-isomorphism.json",
        "pred evaluate subgraph-isomorphism.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("GroupingBySwapping")
  let source = x.instance.string
  let alpha-size = x.instance.alphabet_size
  let budget = x.instance.budget
  let config = x.optimal_config
  let alpha-map = range(alpha-size).map(i => str.from-unicode(97 + i))
  let fmt-str(s) = s.map(c => alpha-map.at(c)).join("")
  let source-str = fmt-str(source)
  let step1 = (0, 1, 0, 2, 1, 2)
  let step2 = (0, 0, 1, 2, 1, 2)
  let step3 = (0, 0, 1, 1, 2, 2)
  let step3-str = fmt-str(step3)
  [
    #problem-def("GroupingBySwapping")[
      Given a finite alphabet $Sigma$, a string $x in Sigma^*$, and a positive integer $K$, determine whether there exists a sequence of at most $K$ adjacent symbol interchanges that transforms $x$ into a string $y in Sigma^*$ in which every symbol $a in Sigma$ appears in a single contiguous block. Equivalently, $y$ contains no subsequence $a b a$ with distinct $a, b in Sigma$.
    ][
      Grouping by Swapping is the storage-and-retrieval problem SR21 in Garey and Johnson @garey1979. It asks whether a string can be locally reorganized, using only adjacent transpositions, until equal symbols coalesce into blocks. The implementation in this crate uses a fixed-length swap program with one slot per allowed operation, so the direct brute-force search explores $O(|x|^K)$ configurations.#footnote[This is the exact search bound induced by the fixed-length witness encoding implemented in the codebase; no sharper exact worst-case bound is claimed here.]

      *Example.* Let $Sigma = {#alpha-map.join(", ")}$, $x = #source-str$, and $K = #budget$. The configuration $p = (#config.map(str).join(", "))$ performs adjacent swaps at positions $(2, 3)$, $(1, 2)$, and $(3, 4)$, then uses two trailing no-op slots. The resulting string is $y = #step3-str$, so every symbol now appears in one contiguous block and the verifier returns YES.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o grouping-by-swapping.json",
        "pred solve grouping-by-swapping.json --solver brute-force",
        "pred evaluate grouping-by-swapping.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure({
        let blue = graph-colors.at(0)
        let cell(ch, highlight: false) = {
          let fill = if highlight { blue.transparentize(72%) } else { white }
          box(width: 0.55cm, height: 0.55cm, fill: fill, stroke: 0.5pt + luma(120),
            align(center + horizon, text(9pt, weight: "bold", ch)))
        }
        align(center, stack(dir: ttb, spacing: 0.45cm,
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[$x: quad$])),
            ..source.map(c => cell(alpha-map.at(c))),
          ),
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[swap$(2,3)$: quad])),
            ..step1.enumerate().map(((i, c)) => cell(alpha-map.at(c), highlight: i == 2 or i == 3)),
          ),
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[swap$(1,2)$: quad])),
            ..step2.enumerate().map(((i, c)) => cell(alpha-map.at(c), highlight: i == 1 or i == 2)),
          ),
          stack(dir: ltr, spacing: 0pt,
            box(width: 2.2cm, height: 0.5cm, align(right + horizon, text(8pt)[swap$(3,4)$: quad])),
            ..step3.enumerate().map(((i, c)) => cell(alpha-map.at(c), highlight: i == 3 or i == 4)),
          ),
        ))
      },
      caption: [Grouping by Swapping on $x = #source-str$: three effective adjacent swaps turn the alternating string into $y = #step3-str$. The remaining two slots in $p = (#config.map(str).join(", "))$ are no-ops at position 5.],
      ) <fig:grouping-by-swapping>

      The final row has exactly one block of $a$, one block of $b$, and one block of $c$, so it satisfies the grouping constraint within the allotted budget.
    ]
  ]
}

#{
  let x = load-model-example("LongestCommonSubsequence")
  let strings = x.instance.strings
  let alphabet-size = x.instance.alphabet_size
  // optimal_config includes padding symbols; extract the non-padding prefix
  let witness = x.optimal_config.filter(c => c < alphabet-size)
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
      Given a finite alphabet $Sigma$ and a set $R = {r_1, dots, r_m}$ of strings over $Sigma^*$, find a longest string $w in Sigma^*$ such that every string $r_i in R$ contains $w$ as a _subsequence_: there exist indices $1 lt.eq j_1 < j_2 < dots < j_(|w|) lt.eq |r_i|$ with $r_i[j_t] = w[t]$ for all $t$.
    ][
      A classic NP-hard string problem, listed as problem SR10 in Garey and Johnson @garey1979. #cite(<maier1978>, form: "prose") proved NP-completeness of the decision version, while Garey and Johnson note polynomial-time cases for fixed $|R|$. For the special case of two strings, the classical dynamic-programming algorithm of #cite(<wagnerfischer1974>, form: "prose") runs in $O(|r_1| dot |r_2|)$ time. The optimization model implemented in this repository maximizes the subsequence length directly using a padding-based encoding.

      *Example.* Let $Sigma = {0, 1}$ and let the input set $R$ contain the strings #string-list. The witness $w = $ #fmt-str(witness) is a longest common subsequence of every string in $R$, with $|w| = #witness.len()$.

      #pred-commands(
        "pred create --example LongestCommonSubsequence -o longest-common-subsequence.json",
        "pred solve longest-common-subsequence.json",
        "pred evaluate longest-common-subsequence.json --config " + x.optimal_config.map(str).join(","),
      )

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

      The highlighted positions show one left-to-right embedding of $w = $ #fmt-str(witness) in each input string, certifying that the longest common subsequence has length #witness.len().
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

      #pred-commands(
        "pred create --example SubsetSum -o subset-sum.json",
        "pred solve subset-sum.json",
        "pred evaluate subset-sum.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("SubsetProduct")
  let sizes = x.instance.sizes
  let target = x.instance.target
  let n = sizes.len()
  let config = x.optimal_config
  let selected = range(n).filter(i => config.at(i) == 1)
  let sel-sizes = selected.map(i => sizes.at(i))
  [
    #problem-def("SubsetProduct")[
      Given a finite set $A = {a_0, dots, a_(n-1)}$ with sizes $s(a_i) in ZZ^+$ and a target $B in ZZ^+$, determine whether there exists a subset $A' subset.eq A$ such that $product_(a in A') s(a) = B$.
    ][
      The multiplicative analogue of Subset Sum. Instead of seeking a subset whose elements sum to a target, we seek one whose product equals the target. NP-complete; the best known exact algorithm is an $O^*(2^(n slash 2))$ meet-in-the-middle approach analogous to Horowitz--Sahni for Subset Sum.

      *Example.* Let $A = {#sizes.map(s => str(s)).join(", ")}$ ($n = #n$) and target $B = #target$. Selecting $A' = {#sel-sizes.map(s => str(s)).join(", ")}$ gives product $#sel-sizes.map(s => str(s)).join(" times ") = #target = B$.

      #pred-commands(
        "pred create --example SubsetProduct -o subset-product.json",
        "pred solve subset-product.json",
        "pred evaluate subset-product.json --config " + x.optimal_config.map(str).join(","),
      )
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

#{
  let x = load-model-example("ConsistencyOfDatabaseFrequencyTables")
  let num_objects = x.instance.num_objects
  let num_attrs = x.instance.attribute_domains.len()
  let domains = x.instance.attribute_domains
  let table01 = x.instance.frequency_tables.at(0).counts
  let table12 = x.instance.frequency_tables.at(1).counts
  let config = x.optimal_config
  let value = (object, attr) => config.at(object * num_attrs + attr)
  [
    #problem-def("ConsistencyOfDatabaseFrequencyTables")[
      Given a finite set $V$ of objects, a finite set $A$ of attributes, a domain $D_a$ for each $a in A$, a collection of pairwise frequency tables $f_(a,b): D_a times D_b -> ZZ^(>=0)$ whose entries sum to $|V|$, and a set $K subset.eq V times A times union_(a in A) D_a$ of known triples $(v, a, x)$, determine whether there exist functions $g_a: V -> D_a$ such that $g_a(v) = x$ for every $(v, a, x) in K$ and, for every published table $f_(a,b)$, exactly $f_(a,b)(x, y)$ objects satisfy $(g_a(v), g_b(v)) = (x, y)$.
    ][
      Consistency of Database Frequency Tables is Garey and Johnson's storage-and-retrieval problem SR35 @garey1979. It asks whether released pairwise marginals can come from some hidden microdata table while respecting already known individual attribute values, making it a natural decision problem in statistical disclosure control. The direct witness space implemented in this crate assigns one categorical variable to each object-attribute pair, so exhaustive search runs in $O^*((product_(a in A) |D_a|)^(|V|))$. #footnote[This is the exact search bound induced by the implementation's configuration space; no faster general exact worst-case algorithm is claimed here.]

      *Example.* Let $|V| = #num_objects$ with attributes $a_0, a_1, a_2$ having domain sizes $#domains.at(0)$, $#domains.at(1)$, and $#domains.at(2)$ respectively. Publish the pairwise tables

      #align(center, table(
        columns: 4,
        align: center,
        table.header([$f_(a_0, a_1)$], [$0$], [$1$], [$2$]),
        [$0$], [#table01.at(0).at(0)], [#table01.at(0).at(1)], [#table01.at(0).at(2)],
        [$1$], [#table01.at(1).at(0)], [#table01.at(1).at(1)], [#table01.at(1).at(2)],
      ))

      and

      #align(center, table(
        columns: 3,
        align: center,
        table.header([$f_(a_1, a_2)$], [$0$], [$1$]),
        [$0$], [#table12.at(0).at(0)], [#table12.at(0).at(1)],
        [$1$], [#table12.at(1).at(0)], [#table12.at(1).at(1)],
        [$2$], [#table12.at(2).at(0)], [#table12.at(2).at(1)],
      ))

      together with the known values $K = {(v_0, a_0, 0), (v_3, a_0, 1), (v_1, a_2, 1)}$. One consistent completion is:

      #align(center, table(
        columns: 4,
        align: center,
        table.header([object], [$a_0$], [$a_1$], [$a_2$]),
        [$v_0$], [#value(0, 0)], [#value(0, 1)], [#value(0, 2)],
        [$v_1$], [#value(1, 0)], [#value(1, 1)], [#value(1, 2)],
        [$v_2$], [#value(2, 0)], [#value(2, 1)], [#value(2, 2)],
        [$v_3$], [#value(3, 0)], [#value(3, 1)], [#value(3, 2)],
        [$v_4$], [#value(4, 0)], [#value(4, 1)], [#value(4, 2)],
        [$v_5$], [#value(5, 0)], [#value(5, 1)], [#value(5, 2)],
      ))

      This witness satisfies every published count: in $f_(a_0, a_1)$ each of the six cells appears exactly once, while in $f_(a_1, a_2)$ the five occupied cells have multiplicities $1, 1, 2, 1, 1$ exactly as listed above. It also respects all three known triples, so the answer is YES.

      #pred-commands(
        "pred create --example ConsistencyOfDatabaseFrequencyTables -o consistency-of-database-frequency-tables.json",
        "pred solve consistency-of-database-frequency-tables.json",
        "pred evaluate consistency-of-database-frequency-tables.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#reduction-rule("ConsistencyOfDatabaseFrequencyTables", "ILP")[
  Each object-attribute pair is encoded by a one-hot binary vector over its domain, and each pairwise frequency count becomes a linear equality over McCormick auxiliary variables that linearize the product of two one-hot indicators. Known values are fixed by pinning the corresponding indicator to 1. The resulting ILP is a pure feasibility problem (trivial objective).
][
  _Construction._ Let $V$ be the set of objects, $A$ the set of attributes with domains $D_a$, $cal(T)$ the set of published frequency tables, and $K$ the set of known triples $(v, a, x)$.

  _Variables:_ (1) Binary one-hot indicators $y_(v,a,x) in {0, 1}$ for each object $v in V$, attribute $a in A$, and value $x in D_a$: $y_(v,a,x) = 1$ iff object $v$ takes value $x$ for attribute $a$. (2) Binary auxiliary variables $z_(t,v,x,x') in {0, 1}$ for each table $t in cal(T)$ (with attribute pair $(a, b)$), object $v in V$, and cell $(x, x') in D_a times D_b$: $z_(t,v,x,x') = 1$ iff object $v$ realizes cell $(x, x')$ in table $t$.

  _Constraints:_ (1) One-hot: $sum_(x in D_a) y_(v,a,x) = 1$ for all $v in V$, $a in A$. (2) Known values: $y_(v,a,x) = 1$ for each $(v, a, x) in K$. (3) McCormick linearization for $z_(t,v,x,x') = y_(v,a,x) dot y_(v,b,x')$: $z_(t,v,x,x') lt.eq y_(v,a,x)$, $z_(t,v,x,x') lt.eq y_(v,b,x')$, $z_(t,v,x,x') gt.eq y_(v,a,x) + y_(v,b,x') - 1$. (4) Frequency counts: $sum_(v in V) z_(t,v,x,x') = f_t (x, x')$ for each table $t$ and cell $(x, x')$.

  _Objective:_ Minimize $0$ (feasibility problem).

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(x in D_a) y_(v,a,x) = 1 quad forall v in V, a in A \
    & y_(v,a,x) = 1 quad forall (v, a, x) in K \
    & z_(t,v,x,x') <= y_(v,a,x) quad forall t in cal(T), v in V, (x, x') in D_a times D_b \
    & z_(t,v,x,x') <= y_(v,b,x') quad forall t in cal(T), v in V, (x, x') in D_a times D_b \
    & z_(t,v,x,x') >= y_(v,a,x) + y_(v,b,x') - 1 quad forall t in cal(T), v in V, (x, x') in D_a times D_b \
    & sum_(v in V) z_(t,v,x,x') = f_t(x, x') quad forall t in cal(T), (x, x') in D_a times D_b \
    & y_(v,a,x), z_(t,v,x,x') in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A consistent assignment defines one-hot indicators and their products; all constraints hold by construction, and the frequency equalities match the published counts. ($arrow.l.double$) Any feasible binary solution assigns exactly one value per object-attribute (one-hot), respects known values, and the McCormick constraints force $z_(t,v,x,x') = y_(v,a,x) dot y_(v,b,x')$ for binary variables, so the frequency equalities certify consistency.

  _Solution extraction._ For each object $v$ and attribute $a$, find $x$ with $y_(v,a,x) = 1$; assign value $x$ to $(v, a)$.
]

#problem-def("SumOfSquaresPartition")[
  Given a finite set $A = {a_0, dots, a_(n-1)}$ with sizes $s(a_i) in ZZ^+$ and a positive integer $K lt.eq |A|$ (number of groups), find a partition of $A$ into $K$ disjoint sets $A_1, dots, A_K$ that minimizes $sum_(i=1)^K (sum_(a in A_i) s(a))^2$.
][
  Problem SP19 in Garey and Johnson @garey1979. NP-complete in the strong sense, so no pseudo-polynomial time algorithm exists unless $P = "NP"$. For fixed $K$, a dynamic-programming algorithm runs in $O(n S^(K-1))$ pseudo-polynomial time, where $S = sum s(a)$. The problem remains NP-complete when the exponent 2 is replaced by any fixed rational $alpha > 1$. #footnote[No algorithm improving on brute-force $O(K^n)$ enumeration is known for the general case.] The squared objective penalizes imbalanced partitions, connecting it to variance minimization, load balancing, and $k$-means clustering. Sum of Squares Partition generalizes Partition ($K = 2$, $J = S^2 slash 2$).

  *Example.* Let $A = {5, 3, 8, 2, 7, 1}$ ($n = 6$) and $K = 3$ groups. The partition $A_1 = {8, 1}$, $A_2 = {5, 2}$, $A_3 = {3, 7}$ gives group sums $9, 7, 10$ and sum of squares $81 + 49 + 100 = 230$. The optimal partition has group sums ${9, 9, 8}$ yielding $81 + 81 + 64 = 226$.
]

#{
  let x = load-model-example("ThreePartition")
  let sizes = x.instance.sizes
  let bound = x.instance.bound
  let config = x.optimal_config
  let m = int(sizes.len() / 3)
  // Group elements by their assignment in optimal_config
  let groups = range(m).map(g => {
    let indices = range(sizes.len()).filter(i => config.at(i) == g)
    indices.map(i => sizes.at(i))
  })
  [
    #problem-def("ThreePartition")[
      Given a set $A = {a_0, dots, a_(3m-1)}$ of $3m$ elements, a bound $B in ZZ^+$, and sizes $s(a) in ZZ^+$ such that $B/4 lt s(a) lt B/2$ for every $a in A$ and $sum_(a in A) s(a) = m B$, determine whether $A$ can be partitioned into $m$ disjoint triples $A_1, dots, A_m$ with $sum_(a in A_i) s(a) = B$ for every $i$.
    ][
      3-Partition is Garey and Johnson's strongly NP-complete benchmark SP15 @garey1979. Unlike ordinary Partition, the strict size window forces every feasible block to contain exactly three elements, making the problem the canonical source for strong NP-completeness reductions to scheduling, packing, and layout models. The implementation in this repository uses one group-assignment variable per element, so the exported exact-search baseline is $O^*(3^n)$#footnote[This is the direct worst-case bound induced by the implementation's configuration space and matches the registered catalog expression `3^num_elements`; no sharper general exact bound was independently verified while preparing this entry.].

      *Example.* Let $B = #bound$ and consider the #(sizes.len())-element instance with sizes $(#sizes.map(str).join(", "))$. The witness triples #groups.enumerate().map(((i, g)) => [$A_#(i+1) = {#g.map(str).join(", ")}$]).join([ and ]) both sum to $#bound$, so this instance is satisfiable.

      #pred-commands(
        "pred create --example ThreePartition -o three-partition.json",
        "pred solve three-partition.json",
        "pred evaluate three-partition.json --config " + config.map(str).join(","),
      )

      #align(center, table(
        columns: 3,
        align: center,
        table.header([Triple], [Elements], [Sum]),
        ..groups.enumerate().map(((i, g)) => (
          [$A_#(i+1)$], [$#(g.map(str).join(", "))$], [$#bound$],
        )).flatten(),
      ))
    ]
  ]
}

#{
  let x = load-model-example("Numerical3DimensionalMatching")
  let m = x.instance.sizes_w.len()
  let sw = x.instance.sizes_w
  let sx = x.instance.sizes_x
  let sy = x.instance.sizes_y
  let bound = x.instance.bound
  let config = x.optimal_config
  let x-perm = config.slice(0, m)
  let y-perm = config.slice(m)
  [
    #problem-def("Numerical3DimensionalMatching")[
      Given disjoint sets $W$, $X$, $Y$ each with $m$ elements, positive integer sizes $s(a)$ with $B\/4 < s(a) < B\/2$ for every element, and a bound $B in ZZ^+$ such that $sum s(a) = m B$, determine whether $W union X union Y$ can be partitioned into $m$ triples, each containing one element from $W$, $X$, and $Y$, with each triple summing to exactly $B$.
    ][
      Numerical 3-Dimensional Matching is strongly NP-complete (SP16 in Garey and Johnson @garey1979). The strict size window $B\/4 < s(a) < B\/2$ forces every feasible triple to contain exactly one element from each set. The problem is a key intermediate in strong NP-completeness reductions to bin packing, scheduling, and layout problems. Brute-force enumeration runs in $O^*(m^(2m))$ time.

      *Example.* Let $m = #m$ and $B = #bound$. The sizes are $W = (#sw.map(str).join(", "))$, $X = (#sx.map(str).join(", "))$, $Y = (#sy.map(str).join(", "))$. The matching pairs each $w_i$ with $x_(pi(i))$ and $y_(sigma(i))$: #range(m).map(i => [$w_#i + x_#(x-perm.at(i)) + y_#(y-perm.at(i)) = #(sw.at(i) + sx.at(x-perm.at(i)) + sy.at(y-perm.at(i)))$]).join(", "), all equal to $B$.

      #pred-commands(
        "pred create --example Numerical3DimensionalMatching -o n3dm.json",
        "pred solve n3dm.json",
        "pred evaluate n3dm.json --config " + config.map(str).join(","),
      )
    ]
  ]
}
#{
  let x = load-model-example("NumericalMatchingWithTargetSums")
  let m = x.instance.sizes_x.len()
  let sx = x.instance.sizes_x
  let sy = x.instance.sizes_y
  let targets = x.instance.targets
  let config = x.optimal_config
  [
    #problem-def("NumericalMatchingWithTargetSums")[
      Given two disjoint sets $X$ and $Y$ each with $m$ elements, integer sizes $s(x_i)$ for $x_i in X$ and $s(y_j)$ for $y_j in Y$, and a multiset of $m$ target values $B_1, dots, B_m$, determine whether $X union Y$ can be partitioned into $m$ pairs, each containing one element from $X$ and one from $Y$, such that the multiset of pair sums ${s(x_i) + s(y_(pi(i)))}$ equals the target multiset.
    ][
      Numerical Matching with Target Sums is NP-complete in the strong sense (SP17 in Garey and Johnson @garey1979). It generalizes bipartite perfect matching by imposing sum constraints on each pair. Brute-force enumeration runs in $O^*(2^m)$ time by trying all $m!$ permutations.

      *Example.* Let $m = #m$, $X = (#sx.map(str).join(", "))$, $Y = (#sy.map(str).join(", "))$, targets $= (#targets.map(str).join(", "))$. The matching $pi = (#config.map(str).join(", "))$ yields sums #range(m).map(i => [$#(sx.at(i)) + #(sy.at(config.at(i))) = #(sx.at(i) + sy.at(config.at(i)))$]).join(", "), which as a multiset equals the targets.

      #pred-commands(
        "pred create --example NumericalMatchingWithTargetSums -o nmts.json",
        "pred solve nmts.json",
        "pred evaluate nmts.json --config " + config.map(str).join(","),
      )
    ]
  ]
}
#reduction-rule("NumericalMatchingWithTargetSums", "ILP",
  example: true,
  example-caption: [Numerical Matching with Target Sums to ILP via compatible-triple assignment variables.],
)[
  Introduce a binary variable $z_(i,j,k) in {0,1}$ for each _compatible triple_ $(i,j,k)$ where $s(x_i) + s(y_j) = B_k$. The constraints ensure a perfect matching: $sum_(j,k) z_(i,j,k) = 1$ for each $i$ (every $x_i$ matched once), $sum_(i,k) z_(i,j,k) = 1$ for each $j$ (every $y_j$ matched once), $sum_(i,j) z_(i,j,k) = 1$ for each $k$ (every target used once). The objective is trivial (minimize 0), since this is a feasibility problem.
][
  _Correctness._ By construction, variables are only created for triples satisfying $s(x_i) + s(y_j) = B_k$. The three families of equality constraints enforce that the assignment is a bijection on $X$, $Y$, and the target indices. Any feasible ILP solution therefore defines a permutation $pi$ with $s(x_i) + s(y_(pi(i)))$ matching a distinct target, and conversely any valid matching maps to a feasible binary assignment. The number of variables is at most $m^3$ (all triples compatible), and the number of constraints is $3m$.
]
#{
  let x = load-model-example("NonLivenessFreePetriNet")
  let np = x.instance.num_places
  let nt = x.instance.num_transitions
  let config = x.optimal_config
  [
    #problem-def("NonLivenessFreePetriNet")[
      Given a free-choice Petri net $P = (S, T, F, M_0)$ with $|S|$ places, $|T|$ transitions, flow relation $F$, and initial marking $M_0$, determine whether $P$ is _not live_: does there exist a transition $t in T$ and a marking $M$ reachable from $M_0$ such that $t$ can never fire again from $M$?
    ][
      Non-Liveness of free-choice Petri nets is NP-complete (Garey and Johnson @garey1979). A Petri net is _free-choice_ if every two transitions sharing an input place have identical presets. The implementation explores the bounded reachability graph (capped at the initial token sum per place) and checks whether any transition becomes permanently dead.

      *Example.* A chain net with $#np$ places and $#nt$ transitions: $t_0$ moves a token from $s_0$ to $s_1$, $t_1$ from $s_1$ to $s_2$, $t_2$ from $s_2$ to $s_3$. Starting from $M_0 = (1, 0, 0, 0)$, after all transitions fire once the net reaches deadlock at $(0, 0, 0, 1)$ and all transitions are permanently dead. The witness configuration $(#config.map(str).join(", "))$ confirms all transitions are globally dead.

      #pred-commands(
        "pred create --example NonLivenessFreePetriNet -o petri.json",
        "pred solve petri.json --solver brute-force",
        "pred evaluate petri.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumAxiomSet")
  let ns = x.instance.num_sentences
  let ts = x.instance.true_sentences
  let nt = ts.len()
  let imps = x.instance.implications
  let config = x.optimal_config
  let selected = range(nt).filter(i => config.at(i) == 1)
  let sel-labels = selected.map(i => str(ts.at(i)))
  [
    #problem-def("MinimumAxiomSet")[
      Given a finite set of sentences $S = {s_0, dots, s_(n-1)}$, a subset $T subset.eq S$ of true sentences, and a set of implications ${(A_j, c_j)}$ where each $A_j subset.eq S$ and $c_j in S$, find a smallest subset $S_0 subset.eq T$ such that the deductive closure of $S_0$ under the implications equals $T$. That is, starting from $S_0$, repeatedly applying every rule "if all sentences in $A_j$ hold then $c_j$ holds" until no new sentences are added must yield exactly $T$.
    ][
      Minimum Axiom Set is problem LO6 in Garey and Johnson @garey1979. The problem models finding a minimal set of assumptions (axioms) from which all truths in a theory can be derived. It generalises set cover: when every implication has a single antecedent, the problem reduces to finding a minimum dominating set in the implication graph. No algorithm improving on brute-force ($O(2^(|T|))$) is known for the general case.#footnote[No algorithm improving on brute-force is known for the general Minimum Axiom Set problem.]

      *Example.* Let $S = {s_0, dots, s_7}$ ($n = #ns$) with $T = S$ (all sentences true) and implications
      #imps.map(imp => {
        let ante = imp.at(0).map(str).join(", ")
        let cons = str(imp.at(1))
        [$({#ante} arrow.r #cons)$]
      }).join(", "). Selecting axioms $S_0 = {#sel-labels.join(", ")}$ generates the full deductive closure $T$ in three rounds: first $\{0,1\} arrow.r \{2,3,4,5\}$, then $\{2,4\},\{3,5\} arrow.r \{6,7\}$, then $\{6,7\} arrow.r \{0,1\}$ (already present). The optimal value is $#x.optimal_value$.

      #pred-commands(
        "pred create --example MinimumAxiomSet -o axiom.json",
        "pred solve axiom.json --solver brute-force",
        "pred evaluate axiom.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("Betweenness")
  let n = x.instance.num_elements
  let triples = x.instance.triples
  let config = x.optimal_config
  [
    #problem-def("Betweenness")[
      Given a finite set $A = {a_0, dots, a_(n-1)}$ of $n$ elements and a collection $C$ of ordered triples $(a, b, c)$, determine whether there exists a linear ordering $f: A -> {0, dots, n-1}$ (a bijection) such that for every $(a, b, c) in C$, either $f(a) < f(b) < f(c)$ or $f(c) < f(b) < f(a)$ — that is, $b$ is _between_ $a$ and $c$ in the ordering.
    ][
      Betweenness is problem MS1 in Garey and Johnson @garey1979. It arises in seriation, archaeological sequencing, and DNA physical mapping. The problem is NP-complete even when restricted to dense constraint sets. The implementation represents a solution as a permutation $f$ where $f(i)$ is the position assigned to element $i$.

      *Example.* Consider $n = #n$ elements with triples #triples.map(t => [$(#t.at(0), #t.at(1), #t.at(2))$]).join(", "). The witness ordering $f = (#config.map(str).join(", "))$ (the identity permutation) satisfies all constraints: each middle element of every triple lies between the other two in the ordering.

      #pred-commands(
        "pred create --example Betweenness -o betweenness.json",
        "pred solve betweenness.json",
        "pred evaluate betweenness.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("CyclicOrdering")
  let n = x.instance.num_elements
  let triples = x.instance.triples
  let config = x.optimal_config
  [
    #problem-def("CyclicOrdering")[
      Given a finite set $A = {a_0, dots, a_(n-1)}$ of $n$ elements and a collection $C$ of ordered triples $(a, b, c)$, determine whether there exists a permutation $f: A -> {0, dots, n-1}$ (a bijection) such that for every $(a, b, c) in C$, the values $f(a)$, $f(b)$, $f(c)$ appear in cyclic order — i.e., $f(a) < f(b) < f(c)$ or $f(b) < f(c) < f(a)$ or $f(c) < f(a) < f(b)$.
    ][
      Cyclic Ordering is problem MS2 in Garey and Johnson @garey1979. It is closely related to Betweenness (MS1) but enforces a cyclic rather than linear ordering constraint. The problem is NP-complete. The implementation represents a solution as a permutation $f$ where $f(i)$ is the position assigned to element $i$.

      *Example.* Consider $n = #n$ elements with triples #triples.map(t => [$(#t.at(0), #t.at(1), #t.at(2))$]).join(", "). The witness ordering $f = (#config.map(str).join(", "))$ satisfies all constraints: each triple's elements appear in cyclic order under $f$.

      #pred-commands(
        "pred create --example CyclicOrdering -o cyclic_ordering.json",
        "pred solve cyclic_ordering.json",
        "pred evaluate cyclic_ordering.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("Clustering")
  let n = x.instance.distances.len()
  let K = x.instance.num_clusters
  let B = x.instance.diameter_bound
  let config = x.optimal_config
  [
    #problem-def("Clustering")[
      Given a set of $n$ elements with a symmetric distance function $d: binom(A,2) -> NN$ (represented as a matrix with zero diagonal), a cluster count bound $K$, and a diameter bound $B$, determine whether there exists a partition of the elements into at most $K$ non-empty clusters such that for every cluster $C$, all pairwise distances within $C$ satisfy $d(i,j) <= B$.
    ][
      Clustering is a fundamental problem in unsupervised learning and data analysis. The variant considered here is the diameter-bounded formulation, which is NP-complete. No algorithm improving on brute-force ($K^n$ enumeration) is known for the general case.#footnote[No algorithm improving on brute-force is known for general diameter-bounded clustering.]

      *Example.* Consider $n = #n$ elements with $K = #K$ clusters and diameter bound $B = #B$. The distance matrix has two tight groups ${0,1,2}$ and ${3,4,5}$ with intra-group distance 1 and inter-group distance 3. The witness assignment $(#config.map(str).join(", "))$ partitions elements into clusters ${0,1,2}$ and ${3,4,5}$; each cluster has maximum pairwise distance $1 <= #B$.

      #pred-commands(
        "pred create --example Clustering -o clustering.json",
        "pred solve clustering.json",
        "pred evaluate clustering.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("DynamicStorageAllocation")
  let items = x.instance.items
  let D = x.instance.memory_size
  let config = x.optimal_config
  let n = items.len()
  [
    #problem-def("DynamicStorageAllocation")[
      Given $n$ items, each with arrival time $r(a)$, departure time $d(a)$, and size $s(a)$, and a storage size $D$, determine whether there exists a starting address $sigma(a) in {0, dots, D - s(a)}$ for each item $a$ such that for every pair of items $a, a'$ with overlapping time intervals ($r(a) < d(a')$ and $r(a') < d(a)$), the memory intervals $[sigma(a), sigma(a) + s(a) - 1]$ and $[sigma(a'), sigma(a') + s(a') - 1]$ are disjoint.
    ][
      Dynamic Storage Allocation is Garey and Johnson's SR2 @garey1979 and models memory allocation for processes with known lifetimes. It generalises strip-packing and bin-packing with time constraints. The implementation encodes each item's starting address as a single variable with domain $D - s(a) + 1$.

      *Example.* Let $D = #D$ and consider #n items with $(r, d, s)$ tuples #items.map(t => $(#t.at(0), #t.at(1), #t.at(2))$).join(", "). The witness assignment $sigma = (#config.map(str).join(", "))$ places every item within $[0, #(D - 1)]$ and ensures no two time-overlapping items share memory cells.

      #pred-commands(
        "pred create --example DynamicStorageAllocation -o dynamic-storage-allocation.json",
        "pred solve dynamic-storage-allocation.json",
        "pred evaluate dynamic-storage-allocation.json --config " + config.map(str).join(","),
      )

      #align(center, table(
        columns: 5,
        align: center,
        table.header([Item], [Arrival], [Departure], [Size], [$sigma$]),
        ..items.enumerate().map(((i, t)) => (
          [$a_#i$], [$#t.at(0)$], [$#t.at(1)$], [$#t.at(2)$], [$#(config.at(i))$],
        )).flatten(),
      ))
    ]
  ]
}

#{
  let x = load-model-example("KthLargestMTuple")
  let sets = x.instance.sets
  let k = x.instance.k
  let bound = x.instance.bound
  let config = x.optimal_config
  let m = sets.len()
  // Count qualifying tuples by enumerating the Cartesian product
  let total = sets.fold(1, (acc, s) => acc * s.len())
  [
    #problem-def("KthLargestMTuple")[
      Given $m$ finite sets $X_1, dots, X_m$ of positive integers, a bound $B in ZZ^+$, and a threshold $K in ZZ^+$, count the number of distinct $m$-tuples $(x_1, dots, x_m) in X_1 times dots.c times X_m$ satisfying $sum_(i=1)^m x_i >= B$. The answer is _yes_ iff this count is at least $K$.
    ][
      The $K$th Largest $m$-Tuple problem is MP10 in Garey and Johnson's appendix @garey1979. It is _not known to be in NP_, because a "yes" certificate may need to exhibit $K$ qualifying tuples and $K$ can be exponentially large. The problem is PP-complete under polynomial-time Turing reductions @haase2016, though the special case $m = 2$, $K = 1$ is NP-complete via reduction from Subset Sum. In the general case, the only known exact approach is brute-force enumeration of all $product_(i=1)^m |X_i|$ tuples, so the registered catalog complexity is `total_tuples * num_sets`#footnote[No algorithm improving on brute-force is known for the general $K$th Largest $m$-Tuple problem.].

      *Example.* Let $m = #m$, $B = #bound$, and $K = #k$ with sets #sets.enumerate().map(((i, s)) => [$X_#(i+1) = {#s.map(str).join(", ")}$]).join([, ]). The Cartesian product has $#total$ tuples. For instance, the tuple $(#config.enumerate().map(((i, c)) => str(sets.at(i).at(c))).join(", "))$ has sum $#config.enumerate().map(((i, c)) => sets.at(i).at(c)).sum() >= #bound$, contributing 1 to the count. In total, #k of the #total tuples satisfy the bound, so the answer is _yes_ (count $= K$).

      #pred-commands(
        "pred create --example KthLargestMTuple -o kth-largest-m-tuple.json",
        "pred solve kth-largest-m-tuple.json --solver brute-force",
        "pred evaluate kth-largest-m-tuple.json --config " + config.map(str).join(","),
      )
    ]
  ]
}

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

      #pred-commands(
        "pred create --example SequencingWithReleaseTimesAndDeadlines -o sequencing-with-release-times-and-deadlines.json",
        "pred solve sequencing-with-release-times-and-deadlines.json",
        "pred evaluate sequencing-with-release-times-and-deadlines.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#problem-def("Partition")[
  Given a finite set $A = {a_0, dots, a_(n-1)}$ with sizes $s(a_i) in ZZ^+$, determine whether there exists a subset $A' subset.eq A$ such that $sum_(a in A') s(a) = sum_(a in A without A') s(a)$.
][
  One of Karp's 21 NP-complete problems @karp1972, listed as SP12 in Garey & Johnson @garey1979. Partition is the special case of Subset Sum where the target equals half the total sum. Though NP-complete, it is only _weakly_ NP-hard: a dynamic-programming algorithm runs in $O(n dot B_"total")$ pseudo-polynomial time, where $B_"total" = sum_i s(a_i)$. The best known exact algorithm is the $O^*(2^(n slash 2))$ meet-in-the-middle approach of Schroeppel and Shamir (1981).

  *Example.* Let $A = {3, 1, 1, 2, 2, 1}$ ($n = 6$, total sum $= 10$). Setting $A' = {3, 2}$ (indices 0, 3) gives sum $3 + 2 = 5 = 10 slash 2$, and $A without A' = {1, 1, 2, 1}$ also sums to 5. Hence a balanced partition exists.
]

#problem-def("CosineProductIntegration")[
  Given a sequence of integers $(a_1, a_2, dots, a_n)$, determine whether there exists a sign assignment $epsilon in {-1, +1}^n$ such that $sum_(i=1)^n epsilon_i a_i = 0$.
][
  Garey & Johnson problem A7/AN14. The original formulation asks whether $integral_0^(2 pi) product_(i=1)^n cos(a_i theta) d theta = 0$; by expanding each cosine as $(e^(i a_i theta) + e^(-i a_i theta)) slash 2$ via Euler's formula and integrating, the integral equals $(2 pi slash 2^n)$ times the number of sign assignments $epsilon$ with $sum epsilon_i a_i = 0$. Hence the integral is nonzero if and only if a balanced sign assignment exists, making this equivalent to a generalisation of Partition to signed integers. NP-complete by reduction from Partition @plaisted1976. Solvable in pseudo-polynomial time via dynamic programming on achievable partial sums.

  *Example.* Let $(a_1, a_2, a_3) = (2, 3, 5)$. The sign assignment $(+1, +1, -1)$ gives $2 + 3 - 5 = 0$, so the integral is nonzero.
]

#{
  let x = load-model-example("ShortestCommonSupersequence")
  let alpha-size = x.instance.alphabet_size
  let max-length = x.instance.max_length
  let strings = x.instance.strings
  let nr = strings.len()
  // Alphabet mapping: 0->a, 1->b, 2->c, ...
  let alpha-map = range(alpha-size).map(i => str.from-unicode(97 + i))
  let fmt-str(s) = "\"" + s.map(c => alpha-map.at(c)).join("") + "\""
  // Optimal config includes padding; extract non-padding prefix
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let w-cfg = sol.config.filter(c => c < alpha-size)
  let w = w-cfg.map(c => alpha-map.at(c))
  let w-str = fmt-str(w-cfg)
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
  let embeds = strings.map(s => compute-embed(s, w-cfg))
  [
    #problem-def("ShortestCommonSupersequence")[
      Given a finite alphabet $Sigma$ and a set $R = {r_1, dots, r_m}$ of strings over $Sigma^*$, find a string $w in Sigma^*$ of minimum length such that every string $r_i in R$ is a _subsequence_ of $w$: there exist indices $1 lt.eq j_1 < j_2 < dots < j_(|r_i|) lt.eq |w|$ with $w[j_k] = r_i [k]$ for all $k$.
    ][
      A classic NP-hard string problem, listed as problem SR8 in Garey and Johnson @garey1979. #cite(<maier1978>, form: "prose") proved NP-completeness of the decision version; #cite(<raiha1981>, form: "prose") showed the problem remains NP-complete even over a binary alphabet ($|Sigma| = 2$). Note that _subsequence_ (characters may be non-contiguous) differs from _substring_ (contiguous block): the Shortest Common Supersequence asks that each input string can be embedded into $w$ by selecting characters in order but not necessarily adjacently.

      For $|R| = 2$ strings, the problem is solvable in polynomial time via the duality with the Longest Common Subsequence (LCS): if $"LCS"(r_1, r_2)$ has length $ell$, then the shortest common supersequence has length $|r_1| + |r_2| - ell$, computable in $O(|r_1| dot |r_2|)$ time by dynamic programming. For general $|R| = m$, the brute-force search explores all candidate supersequences up to the maximum possible length $sum_i |r_i|$. Applications include bioinformatics (reconstructing ancestral sequences from fragments), data compression (representing multiple strings compactly), and scheduling (merging instruction sequences).

      *Example.* Let $Sigma = {#alpha-map.join(", ")}$ and $R = {#r-strs.join(", ")}$. We seek the shortest string $w$ that contains every $r_i$ as a subsequence.

      #pred-commands(
        "pred create --example ShortestCommonSupersequence -o shortest-common-supersequence.json",
        "pred solve shortest-common-supersequence.json",
        "pred evaluate shortest-common-supersequence.json --config " + x.optimal_config.map(str).join(","),
      )

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

      The optimal supersequence $w = #w-str$ has length #w-len and contains all #nr input strings as subsequences.
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

      #pred-commands(
        "pred create --example StringToStringCorrection -o string-to-string-correction.json",
        "pred solve string-to-string-correction.json",
        "pred evaluate string-to-string-correction.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("MinimumExternalMacroDataCompression")
  let alpha-size = x.instance.alphabet_size
  let s = x.instance.string
  let n = s.len()
  let h = x.instance.pointer_cost
  let alpha-map = range(alpha-size).map(i => str.from-unicode(97 + i))
  let s-str = s.map(c => alpha-map.at(c)).join("")
  let opt-val = metric-value(x.optimal_value)
  [
    #problem-def("MinimumExternalMacroDataCompression")[
      Given a finite alphabet $Sigma$ of size $k$, a string $s in Sigma^*$ of length $n$, and a pointer cost $h in ZZ^+$, find a dictionary string $D in Sigma^*$ and a compressed string $C in (Sigma union {p_1, dots, p_n})^*$, where each $p_i$ is a pointer referencing a contiguous substring of $D$, such that $s$ can be obtained from $C$ by replacing every pointer with its referenced substring, minimizing the total cost $|D| + |C| + (h - 1) times$ (number of pointer occurrences in $C$).
    ][
      A classical NP-hard data compression problem, listed as SR22 in Garey and Johnson @garey1979. The macro model of data compression was introduced by #cite(<storer1977>, form: "prose"), who proved NP-completeness via transformation from Vertex Cover. #cite(<storer1982>, form: "prose") provided a comprehensive analysis of the macro compression framework, showing that NP-completeness persists even when $h$ is any fixed integer $gt.eq 2$, when the alphabet has $gt.eq 3$ symbols, and when $D$ contains no pointers (the "external" variant). The LZ-family of practical compression algorithms (LZ77, LZSS, LZ78) are restricted forms of this general macro model. The related Smallest Grammar Problem is APX-hard @charikar2005.#footnote[No algorithm improving on brute-force enumeration is known for optimal external macro compression.]

      *Example.* Let $Sigma = {#alpha-map.join(", ")}$ and $s = #s-str$ (length #n) with pointer cost $h = #h$.

      #pred-commands(
        "pred create --example MinimumExternalMacroDataCompression -o min-emdc.json",
        "pred solve min-emdc.json",
        "pred evaluate min-emdc.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure({
        let blue = graph-colors.at(0)
        let green = graph-colors.at(1)
        let cell(ch, highlight: false, ptr: false) = {
          let fill = if ptr { green.transparentize(70%) } else if highlight { blue.transparentize(70%) } else { white }
          box(width: 0.5cm, height: 0.55cm, fill: fill, stroke: 0.5pt + luma(120),
            align(center + horizon, text(8pt, weight: "bold", ch)))
        }
        let ptr-cell(label) = {
          box(width: 1.5cm, height: 0.55cm, fill: green.transparentize(70%), stroke: 0.5pt + luma(120),
            align(center + horizon, text(7pt, weight: "bold", label)))
        }
        // D = first 6 symbols of s (one copy of the pattern)
        let d-len = alpha-size
        let d-syms = s.slice(0, d-len)
        // C = 3 pointers, each referencing D[0..6]
        let num-ptrs = calc.div-euclid(n, d-len)
        align(center, stack(dir: ttb, spacing: 0.5cm,
          // Source string
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.5cm, height: 0.5cm, align(right + horizon, text(8pt)[$s: quad$])),
            ..s.map(c => cell(alpha-map.at(c))),
          ),
          // Dictionary D
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.5cm, height: 0.5cm, align(right + horizon, text(8pt)[$D: quad$])),
            ..d-syms.map(c => cell(alpha-map.at(c), highlight: true)),
          ),
          // Compressed string C = 3 pointers
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.5cm, height: 0.5cm, align(right + horizon, text(8pt)[$C: quad$])),
            ..range(num-ptrs).map(_ => ptr-cell[$arrow.r D[0..#d-len]$]),
          ),
        ))
      },
      caption: [Minimum External Macro Data Compression: with $s = #s-str$ (length #n) and pointer cost $h = #h$, the optimal compression stores $D = #s-str.slice(0, alpha-size)$ (#alpha-size symbols) and uses #calc.div-euclid(n, alpha-size) pointers in $C$, achieving cost $#alpha-size + #calc.div-euclid(n, alpha-size) + (#h - 1) times #calc.div-euclid(n, alpha-size) = #opt-val$ vs.~uncompressed cost #n.],
      ) <fig:emdc>

      This instance has a repeating pattern of length #alpha-size, allowing the dictionary $D$ to store one copy and the compressed string $C$ to reference it via pointers. Each pointer costs $h = #h$ (the pointer symbol itself plus $h - 1 = #(h - 1)$ extra), so the total cost is $|D| + |C| + (h - 1) times |"pointers"| = #alpha-size + #calc.div-euclid(n, alpha-size) + #(h - 1) times #calc.div-euclid(n, alpha-size) = #opt-val$, saving $#(n - int(opt-val))$ over the uncompressed cost of #n.
    ]
  ]
}

#reduction-rule("MinimumExternalMacroDataCompression", "ILP")[
  The compression problem decomposes into a dictionary selection (which symbols appear at which positions in $D$) and a string partitioning (which segments of $s$ are literals vs.~pointers). Both are naturally expressed with binary variables and linear constraints. The partition structure is modeled as a flow on a DAG whose nodes are string positions and whose arcs are candidate segments.
][
  _Construction._ For alphabet $Sigma$ of size $k$, string $s$ of length $n$, and pointer cost $h$:

  _Variables:_ (1) Binary $d_(j,c) in {0,1}$ for each dictionary position $j in {0, dots, n-1}$ and symbol $c in Sigma$: $d_(j,c) = 1$ iff $D[j] = c$. (2) Binary $u_j in {0,1}$: $u_j = 1$ iff dictionary position $j$ is used. (3) Binary $ell_i in {0,1}$ for each string position $i$: $ell_i = 1$ iff position $i$ is covered by a literal. (4) Binary $p_(i,lambda,delta) in {0,1}$ for each valid triple $(i, lambda, delta)$ with $i + lambda <= n$ and $delta + lambda <= n$: $p_(i,lambda,delta) = 1$ iff positions $[i, i + lambda)$ are covered by a pointer referencing $D[delta .. delta + lambda)$.

  _Constraints:_ (1) Dictionary one-hot: $sum_(c in Sigma) d_(j,c) <= 1$ for all $j$. (2) Linking: $d_(j,c) <= u_j$ for all $j, c$. (3) Contiguity: $u_(j+1) <= u_j$ for all $j < n - 1$. (4) Partition flow: the segments form a partition of ${0, dots, n-1}$ via flow conservation on nodes $0, dots, n$. (5) Pointer matching: $p_(i,lambda,delta) <= d_(delta+r, s[i+r])$ for all offsets $r in {0, dots, lambda - 1}$.

  _Objective:_ Minimize $sum_j u_j + sum_i ell_i + h sum_(i,lambda,delta) p_(i,lambda,delta)$.

  _Correctness._ ($arrow.r.double$) An optimal $(D, C)$ pair determines a feasible ILP assignment: set $d_(j,c) = 1$ for each symbol in $D$, $u_j = 1$ for used positions, and activate the corresponding literal or pointer variables for each $C$-slot. The partition flow is satisfied by construction. ($arrow.l.double$) Any feasible ILP solution defines a valid dictionary (one-hot + contiguity) and a valid partition of $s$ into literal and pointer segments (flow conservation + matching), with cost equal to the objective.

  _Solution extraction._ Read $D$ from the $d_(j,c)$ indicators. Walk through the active segments (via $ell_i$ and $p_(i,lambda,delta)$) to reconstruct $C$.
]

#{
  let x = load-model-example("MinimumInternalMacroDataCompression")
  let alpha-size = x.instance.alphabet_size
  let s = x.instance.string
  let n = s.len()
  let h = x.instance.pointer_cost
  let alpha-map = range(alpha-size).map(i => str.from-unicode(97 + i))
  let s-str = s.map(c => alpha-map.at(c)).join("")
  let opt-val = metric-value(x.optimal_value)
  [
    #problem-def("MinimumInternalMacroDataCompression")[
      Given a finite alphabet $Sigma$ of size $k$, a string $s in Sigma^*$ of length $n$, and a pointer cost $h in ZZ^+$, find a single compressed string $C in (Sigma union {"pointers"})^*$ such that $s$ can be obtained from $C$ by resolving all pointer references _within $C$ itself_ (left-to-right), minimizing the total cost $|C| + (h - 1) times$ (number of pointer occurrences in $C$).
    ][
      A classical NP-hard data compression problem, listed as SR23 in Garey and Johnson @garey1979. Unlike the external variant (@def:MinimumExternalMacroDataCompression), there is no separate dictionary --- the compressed string $C$ serves as both dictionary and output, with pointers referencing substrings within $C$ itself. #cite(<storer1977>, form: "prose") proved NP-completeness via transformation from Vertex Cover. #cite(<storer1982>, form: "prose") showed that NP-completeness persists even when $h$ is any fixed integer $gt.eq 2$. The internal macro model is closely related to the smallest grammar problem, which is APX-hard @charikar2005.#footnote[No algorithm improving on brute-force enumeration is known for optimal internal macro compression.]

      *Example.* Let $Sigma = {#alpha-map.join(", ")}$ and $s = #s-str$ (length #n) with pointer cost $h = #h$.

      #pred-commands(
        "pred create --example MinimumInternalMacroDataCompression -o min-imdc.json",
        "pred solve min-imdc.json",
        "pred evaluate min-imdc.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure({
        let blue = graph-colors.at(0)
        let green = graph-colors.at(1)
        let cell(ch, highlight: false, ptr: false) = {
          let fill = if ptr { green.transparentize(70%) } else if highlight { blue.transparentize(70%) } else { white }
          box(width: 0.5cm, height: 0.55cm, fill: fill, stroke: 0.5pt + luma(120),
            align(center + horizon, text(8pt, weight: "bold", ch)))
        }
        let ptr-cell(label) = {
          box(width: 1.5cm, height: 0.55cm, fill: green.transparentize(70%), stroke: 0.5pt + luma(120),
            align(center + horizon, text(7pt, weight: "bold", label)))
        }
        // Source string
        // C = [a, b, c, ptr(0), ptr(0), EOS, ...]
        align(center, stack(dir: ttb, spacing: 0.5cm,
          // Source string
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.5cm, height: 0.5cm, align(right + horizon, text(8pt)[$s: quad$])),
            ..s.map(c => cell(alpha-map.at(c))),
          ),
          // Compressed string C
          stack(dir: ltr, spacing: 0pt,
            box(width: 1.5cm, height: 0.5cm, align(right + horizon, text(8pt)[$C: quad$])),
            ..range(alpha-size).map(c => cell(alpha-map.at(c), highlight: true)),
            ptr-cell[$arrow.r C[0..#alpha-size]$],
            ptr-cell[$arrow.r C[0..#alpha-size]$],
          ),
        ))
      },
      caption: [Minimum Internal Macro Data Compression: with $s = #s-str$ (length #n) and pointer cost $h = #h$, the optimal self-referencing compression $C$ starts with #alpha-size literals, then uses 2 pointers back to $C[0..#alpha-size]$, achieving cost $5 + (#h - 1) times 2 = #opt-val$ vs.~uncompressed cost #n.],
      ) <fig:imdc>

      The compressed string $C$ has #alpha-size literal symbols followed by 2 pointers, each referencing $C[0..#alpha-size]$ to copy "#alpha-map.join("")". Each pointer costs $h = #h$ (the pointer token plus $h - 1 = #(h - 1)$ extra), so the total cost is $|C| + (h - 1) times |"pointers"| = 5 + #(h - 1) times 2 = #opt-val$, saving $#(n - int(opt-val))$ over the uncompressed cost of #n.
    ]
  ]
}

#reduction-rule("MinimumInternalMacroDataCompression", "ILP")[
  The self-referencing compression problem is formulated as a binary ILP. Since there is no separate dictionary, only the string partition structure needs to be modeled. The partition is expressed as a flow on a DAG whose nodes are string positions and whose arcs are candidate segments.
][
  _Construction._ For alphabet $Sigma$ of size $k$, string $s$ of length $n$, and pointer cost $h$:

  _Variables:_ (1) Binary $ell_i in {0,1}$ for each string position $i in {0, dots, n-1}$: $ell_i = 1$ iff position $i$ is covered by a literal. (2) Binary $p_(i,lambda,r) in {0,1}$ for each valid triple $(i, lambda, r)$ where $r + lambda <= i$ and $s[r..r+lambda) = s[i..i+lambda)$: $p_(i,lambda,r) = 1$ iff positions $[i, i+lambda)$ are covered by a pointer referencing the decoded output starting at source position $r$.

  _Constraints:_ Partition flow: the segments form a partition of ${0, dots, n-1}$ via flow conservation on nodes $0, dots, n$. The string-matching constraint ($s[r..r+lambda) = s[i..i+lambda)$) and the precedence constraint ($r + lambda <= i$) are enforced structurally by only generating valid triples.

  _Objective:_ Minimize $sum_i ell_i + h sum_(i,lambda,r) p_(i,lambda,r)$.

  _Correctness._ ($arrow.r.double$) An optimal compressed string $C$ determines a feasible ILP assignment: activate the literal or pointer variable for each segment in the partition. The flow is satisfied by construction. ($arrow.l.double$) Any feasible ILP solution defines a valid partition of $s$ into literal and pointer segments with cost equal to the objective.

  _Solution extraction._ Walk through the active segments (via $ell_i$ and $p_(i,lambda,r)$) to reconstruct $C$, mapping source reference positions to compressed-string positions.
]

#{
  let x = load-model-example("MinimumWeightAndOrGraph")
  let n = x.instance.num_vertices
  let arcs = x.instance.arcs
  let src = x.instance.source
  let gates = x.instance.gate_types
  let ws = x.instance.arc_weights
  let cfg = x.optimal_config
  let sel-arcs = range(arcs.len()).filter(i => cfg.at(i) == 1)
  let total = sel-arcs.map(i => ws.at(i)).sum()
  [
    #problem-def("MinimumWeightAndOrGraph")[
      Given a directed acyclic graph $G = (V, A)$ with $n = |V|$ vertices, a source vertex $s in V$, a gate-type function $g: V arrow {"AND", "OR", "leaf"}$, and arc weights $w: A arrow ZZ$, find a solution subgraph $S subset.eq A$ of minimum total weight $sum_(a in S) w(a)$. A solution subgraph is valid when: (1) the source is solved, (2) for each solved AND-gate vertex $v$, all outgoing arcs from $v$ are in $S$, (3) for each solved OR-gate vertex $v$, at least one outgoing arc from $v$ is in $S$, and (4) leaf vertices are trivially solved. A vertex $v != s$ is solved if there exists an arc $(u, v) in S$ with $u$ solved.
    ][
      AND/OR graphs generalize search trees and game trees and arise in AI planning, logic programming, and design-space exploration. Dynamic-programming algorithms on tree-structured AND/OR graphs run in linear time, but the general DAG case requires exponential enumeration.#footnote[No algorithm improving on brute-force enumeration of all $2^(|A|)$ arc subsets is known for general AND/OR DAGs.]

      *Example.* Consider $n = #n$ vertices with source $v_#src$ (AND gate). Vertices $v_1, v_2$ are OR gates; $v_3, v_4, v_5, v_6$ are leaves. Arcs with weights: #{range(arcs.len()).map(i => {
        let a = arcs.at(i)
        $v_#(a.at(0)) arrow.r v_#(a.at(1)) (#(ws.at(i)))$
      }).join(", ")}. Since $v_0$ is AND, both outgoing arcs must be selected (cost $#(ws.at(0)) + #(ws.at(1)) = #(ws.at(0) + ws.at(1))$). For OR gates, pick the cheapest outgoing arc: $v_1 arrow.r v_4$ (cost #(ws.at(3))) and $v_2 arrow.r v_6$ (cost #(ws.at(5))). Total weight: $#total$.

      #pred-commands(
        "pred create --example MinimumWeightAndOrGraph -o mwaog.json",
        "pred solve mwaog.json --solver brute-force",
        "pred evaluate mwaog.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumFaultDetectionTestSet")
  let n = x.instance.num_vertices
  let arcs = x.instance.arcs
  let inputs = x.instance.inputs
  let outputs = x.instance.outputs
  let ni = inputs.len()
  let no = outputs.len()
  let cfg = x.optimal_config
  let sel-pairs = range(cfg.len()).filter(i => cfg.at(i) == 1)
  let count = sel-pairs.len()
  [
    #problem-def("MinimumFaultDetectionTestSet")[
      Given a directed acyclic graph $G = (V, A)$ with $n = |V|$ vertices, designated input vertices $I subset.eq V$, and designated output vertices $O subset.eq V$, find the minimum number of input-output pairs $(i, o) in I times O$ such that the union of their coverage sets covers all vertices $V$. For a pair $(i, o)$, the coverage set is the set of vertices reachable from $i$ that can also reach $o$.
    ][
      Fault detection test sets arise in hardware testing: each input-output path through a circuit's DAG representation can detect faults at the vertices it traverses, and the goal is to find the fewest test paths that collectively exercise every component. The problem generalises Set Cover over a structured family of subsets induced by DAG reachability.#footnote[No algorithm improving on brute-force enumeration of all $2^(|I| dot |O|)$ input-output pair subsets is known for the general case.]

      *Example.* Consider $n = #n$ vertices with inputs $I = {#inputs.map(str).join(", ")}$ and outputs $O = {#outputs.map(str).join(", ")}$. Arcs: #{arcs.map(a => $#(a.at(0)) arrow.r #(a.at(1))$).join(", ")}. Selecting pair $(#(inputs.at(0)), #(outputs.at(0)))$ covers ${0, 2, 3, 5}$, and pair $(#(inputs.at(1)), #(outputs.at(1)))$ covers ${1, 3, 4, 6}$. Their union is all $#n$ vertices, giving an optimal count of $#count$.

      #pred-commands(
        "pred create --example MinimumFaultDetectionTestSet -o mfdts.json",
        "pred solve mfdts.json --solver brute-force",
        "pred evaluate mfdts.json --config " + x.optimal_config.map(str).join(","),
      )
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
  let opt-val = metric-value(x.optimal_value)
  let removed = range(na).filter(i => config.at(i) == 1)
  [
    #problem-def("MinimumFeedbackArcSet")[
      Given a directed graph $G = (V, A)$, find a minimum-size subset $A' subset.eq A$ such that $G - A'$ is a directed acyclic graph (DAG). Equivalently, $A'$ must contain at least one arc from every directed cycle in $G$.
    ][
      Feedback Arc Set (FAS) is a classical NP-complete problem from Karp's original list @karp1972 (via transformation from Vertex Cover, as presented in Garey & Johnson GT8). The problem arises in ranking aggregation, sports scheduling, deadlock avoidance, and causal inference. Unlike the undirected analogue (which is trivially polynomial --- the number of non-tree edges in a spanning forest), the directed version is NP-hard due to the richer structure of directed cycles. The best known exact algorithm uses dynamic programming over vertex subsets in $O^*(2^n)$ time, generalizing the Held--Karp TSP technique to vertex ordering problems @bodlaender2012. FAS is fixed-parameter tractable with parameter $k = |A'|$: an $O(4^k dot k! dot n^(O(1)))$ algorithm exists via iterative compression @chen2008. Polynomial-time solvable for planar digraphs via the Lucchesi--Younger theorem @lucchesi1978.

      *Example.* Consider $G$ with $V = {#range(nv).map(v => str(v)).join(", ")}$ and arcs #arcs.map(a => $(#(a.at(0)) arrow #(a.at(1)))$).join($,$). Removing $A' = {#removed.map(i => {let a = arcs.at(i); $(#(a.at(0)) arrow #(a.at(1)))$}).join($,$)}$ (weight #opt-val) breaks all directed cycles, yielding a DAG.

      #pred-commands(
        "pred create --example MinimumFeedbackArcSet -o minimum-feedback-arc-set.json",
        "pred solve minimum-feedback-arc-set.json",
        "pred evaluate minimum-feedback-arc-set.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("PartialFeedbackEdgeSet")
  let nv = graph-num-vertices(x.instance)
  let edges = x.instance.graph.edges
  let ne = edges.len()
  let K = x.instance.budget
  let L = x.instance.max_cycle_length
  let config = x.optimal_config
  let removed-indices = config.enumerate().filter(((i, v)) => v == 1).map(((i, _)) => i)
  let removed-edges = removed-indices.map(i => edges.at(i))
  let blue = graph-colors.at(0)
  let gray = luma(180)
  [
    #problem-def("PartialFeedbackEdgeSet")[
      Given an undirected graph $G = (V, E)$, a budget $K in ZZ_(>= 0)$, and a cycle-length bound $L in ZZ_(>= 0)$, determine whether there exists a subset $E' subset.eq E$ with $|E'| <= K$ such that every simple cycle in $G$ of length at most $L$ contains at least one edge of $E'$.
    ][
      Partial Feedback Edge Set is the bounded-cycle edge-deletion problem GT9 in Garey and Johnson @garey1979. Bounding the cycle length is what makes the problem hard: hitting only the short cycles is NP-complete, whereas the unrestricted undirected feedback-edge-set problem is polynomial-time solvable by reducing to a spanning forest. The implementation here uses one binary variable per edge, so brute-force search explores $O^*(2^|E|)$ candidate edge subsets.#footnote[No sharper general exact worst-case bound is claimed here.]

      *Example.* Consider the graph $G$ with $n = #nv$ vertices, $|E| = #ne$ edges, budget $K = #K$, and length bound $L = #L$. Removing
      $E' = {#removed-edges.map(e => [$\{v_#(e.at(0)), v_#(e.at(1))\}$]).join(", ")}$
      hits the triangles $(v_0, v_1, v_2)$, $(v_0, v_2, v_3)$, $(v_2, v_3, v_4)$, and $(v_3, v_4, v_5)$, together with the 4-cycles $(v_0, v_1, v_2, v_3)$, $(v_0, v_2, v_4, v_3)$, and $(v_2, v_3, v_5, v_4)$. Hence every cycle of length at most 4 is hit. Brute-force search on this instance finds exactly five satisfying 3-edge deletions and none of size 2, so the displayed configuration certifies a YES-instance.

      #pred-commands(
        "pred create --example PartialFeedbackEdgeSet -o partial-feedback-edge-set.json",
        "pred solve partial-feedback-edge-set.json",
        "pred evaluate partial-feedback-edge-set.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          let verts = (
            (0, 1.4),
            (1.2, 2.4),
            (1.9, 1.0),
            (3.3, 1.4),
            (4.5, 2.4),
            (4.5, 0.4),
          )
          for edge in edges {
            let (u, v) = edge
            let selected = removed-edges.any(e =>
              (e.at(0) == u and e.at(1) == v) or (e.at(0) == v and e.at(1) == u)
            )
            g-edge(
              verts.at(u),
              verts.at(v),
              stroke: if selected { 2pt + blue } else { 1pt + gray },
            )
          }
          for (idx, pos) in verts.enumerate() {
            g-node(pos, name: "v" + str(idx), label: [$v_#idx$])
          }
        }),
        caption: [Partial Feedback Edge Set example with $K = 3$ and $L = 4$. Blue edges $\{v_0, v_2\}$, $\{v_2, v_3\}$, and $\{v_3, v_4\}$ form a satisfying edge set that hits every cycle of length at most 4.],
      ) <fig:partial-feedback-edge-set>
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

      #pred-commands(
        "pred create --example MultipleChoiceBranching -o multiple-choice-branching.json",
        "pred solve multiple-choice-branching.json",
        "pred evaluate multiple-choice-branching.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("AcyclicPartition")
  let nv = x.instance.graph.num_vertices
  let arcs = x.instance.graph.arcs.map(a => (a.at(0), a.at(1)))
  let weights = x.instance.vertex_weights
  let config = x.optimal_config
  let B = x.instance.weight_bound
  let K = x.instance.cost_bound
  let part0 = range(nv).filter(v => config.at(v) == 0)
  let part1 = range(nv).filter(v => config.at(v) == 1)
  let part2 = range(nv).filter(v => config.at(v) == 2)
  let part0w = part0.map(v => weights.at(v)).sum(default: 0)
  let part1w = part1.map(v => weights.at(v)).sum(default: 0)
  let part2w = part2.map(v => weights.at(v)).sum(default: 0)
  let cross-arcs = arcs.filter(a => config.at(a.at(0)) != config.at(a.at(1)))
  [
    #problem-def("AcyclicPartition")[
      Given a directed graph $G = (V, A)$ with vertex weights $w: V -> ZZ^+$, arc costs $c: A -> ZZ^+$, and bounds $B, K in ZZ^+$, determine whether there exists a partition $V = V_1 ∪ dots ∪ V_m$ such that every part satisfies $sum_(v in V_i) w(v) <= B$, the total cost of arcs crossing between different parts is at most $K$, and the quotient digraph on the parts is acyclic.
    ][
      Acyclic Partition is the directed partitioning problem ND15 in Garey & Johnson @garey1979. Unlike ordinary graph partitioning, the goal is not merely to minimize the cut: the partition must preserve a global topological order after every part is contracted to a super-node. This makes the model a natural abstraction for DAG-aware task clustering in compiler scheduling, parallel execution pipelines, and automatic differentiation systems where coarse-grained blocks must still communicate without creating cyclic dependencies.

      The implementation uses the natural witness encoding in which each of the $n = #nv$ vertices chooses one of at most $n$ part labels, so direct brute-force search explores $n^n$ assignments.#footnote[Many labelings represent the same unordered partition, but the full configuration space exposed to the solver is still $n^n$.]

      *Example.* Consider the six-vertex digraph in the figure with vertex weights $w = (#weights.map(w => str(w)).join(", "))$, part bound $B = #B$, and cut-cost bound $K = #K$. The witness $V_0 = {#part0.map(v => $v_#v$).join(", ")}$, $V_1 = {#part1.map(v => $v_#v$).join(", ")}$, $V_2 = {#part2.map(v => $v_#v$).join(", ")}$ has part weights $#part0w$, $#part1w$, and $#part2w$, so every part respects the weight cap. Exactly #cross-arcs.len() arcs cross between different parts, namely #cross-arcs.map(a => $(v_#(a.at(0)) arrow v_#(a.at(1)))$).join($,$), so the total crossing cost is $#cross-arcs.len() <= K$. These crossings induce quotient arcs $V_0 arrow V_1$, $V_0 arrow V_2$, and $V_1 arrow V_2$, which form a DAG; hence this instance is a YES-instance.

      #figure({
        let verts = ((0, 1.6), (1.4, 2.4), (1.4, 0.8), (3.2, 2.4), (3.2, 0.8), (4.8, 1.6))
        canvas(length: 1cm, {
          for arc in arcs {
            let (u, v) = arc
            let crossing = config.at(u) != config.at(v)
            draw.line(
              verts.at(u),
              verts.at(v),
              stroke: if crossing { 1.3pt + black } else { 0.9pt + luma(170) },
              mark: (end: "straight", scale: if crossing { 0.5 } else { 0.4 }),
            )
          }
          for (v, pos) in verts.enumerate() {
            let color = graph-colors.at(config.at(v))
            g-node(
              pos,
              name: "v" + str(v),
              fill: color,
              label: text(fill: white)[$v_#v$],
            )
          }
        })
      },
      caption: [A YES witness for Acyclic Partition. Node colors indicate the parts $V_0$, $V_1$, and $V_2$. Black arcs cross parts and define the quotient DAG $V_0 arrow V_1$, $V_0 arrow V_2$, $V_1 arrow V_2$; gray arcs stay inside a part and therefore do not contribute to the quotient graph.],
      ) <fig:acyclic-partition>
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

      #pred-commands(
        "pred create --example FlowShopScheduling -o flow-shop-scheduling.json",
        "pred solve flow-shop-scheduling.json",
        "pred evaluate flow-shop-scheduling.json --config " + x.optimal_config.map(str).join(","),
      )

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

#{
  let x = load-model-example("JobShopScheduling")
  let jobs = x.instance.jobs
  let m = x.instance.num_processors
  let n = jobs.len()
  let lehmer = x.optimal_config

  // Flatten tasks: build per-machine task lists and lengths
  let task-lengths = ()
  let task-job = ()      // which job each flat task belongs to
  let task-index = ()    // which task within the job
  let machine-tasks = range(m).map(_ => ())
  let tid = 0
  for (ji, job) in jobs.enumerate() {
    for (ki, op) in job.enumerate() {
      let (mi, len) = op
      task-lengths.push(len)
      task-job.push(ji)
      task-index.push(ki)
      machine-tasks.at(mi).push(tid)
      tid += 1
    }
  }
  let T = task-lengths.len()

  // Decode per-machine Lehmer codes into machine orders
  let offset = 0
  let machine-orders = ()
  for mi in range(m) {
    let mt = machine-tasks.at(mi)
    let k = mt.len()
    let seg = lehmer.slice(offset, offset + k)
    let avail = range(k)
    let order = ()
    for c in seg {
      order.push(mt.at(avail.at(c)))
      avail = avail.enumerate().filter(((i, v)) => i != c).map(((i, v)) => v)
    }
    machine-orders.push(order)
    offset += k
  }

  // Build DAG edges (job precedence + machine order)
  let successors = range(T).map(_ => ())
  let indegree = range(T).map(_ => 0)
  // Job precedence edges
  let job-task-start = 0
  for job in jobs {
    for i in range(job.len() - 1) {
      let u = job-task-start + i
      let v = job-task-start + i + 1
      successors.at(u).push(v)
      indegree.at(v) += 1
    }
    job-task-start += job.len()
  }
  // Machine order edges
  for order in machine-orders {
    for i in range(order.len() - 1) {
      let u = order.at(i)
      let v = order.at(i + 1)
      successors.at(u).push(v)
      indegree.at(v) += 1
    }
  }

  // Topological sort + longest-path to compute start times
  let start-times = range(T).map(_ => 0)
  let queue = ()
  for t in range(T) {
    if indegree.at(t) == 0 { queue.push(t) }
  }
  while queue.len() > 0 {
    let u = queue.remove(0)
    let finish = start-times.at(u) + task-lengths.at(u)
    for v in successors.at(u) {
      if finish > start-times.at(v) { start-times.at(v) = finish }
      indegree.at(v) -= 1
      if indegree.at(v) == 0 { queue.push(v) }
    }
  }

  // Build Gantt blocks: (machine, job, task-within-job, start, end)
  let blocks = ()
  for t in range(T) {
    let (mi, _len) = jobs.at(task-job.at(t)).at(task-index.at(t))
    blocks.push((mi, task-job.at(t), task-index.at(t), start-times.at(t), start-times.at(t) + task-lengths.at(t)))
  }
  let makespan = calc.max(..range(T).map(t => start-times.at(t) + task-lengths.at(t)))
  [
    #problem-def("JobShopScheduling")[
      Given a positive integer $m$, a set $J$ of jobs, where each job $j in J$ consists of an ordered list of tasks $t_1[j], dots, t_(n_j)[j]$ with processor assignments $p(t_k[j]) in {1, dots, m}$, processing lengths $ell(t_k[j]) in ZZ^+_0$, and consecutive-processor constraint $p(t_k[j]) != p(t_(k+1)[j])$, find start times $sigma(t_k[j]) in ZZ^+_0$ such that tasks sharing a processor do not overlap, each job respects $sigma(t_(k+1)[j]) >= sigma(t_k[j]) + ell(t_k[j])$, and the makespan $max_(j in J) (sigma(t_(n_j)[j]) + ell(t_(n_j)[j]))$ is minimized.
    ][
      Job-Shop Scheduling is the classical disjunctive scheduling problem SS18 in Garey & Johnson; Garey, Johnson, and Sethi proved it strongly NP-hard already for two machines @garey1976. Unlike Flow Shop Scheduling, each job carries its own machine route, so the difficulty lies in choosing a compatible relative order on every machine and then finding the schedule with minimum makespan. This implementation follows the original Garey-Johnson formulation, including the requirement that consecutive tasks of the same job use different processors, and evaluates a witness by orienting the machine-order edges and propagating longest paths through the resulting precedence DAG. The registered baseline therefore exposes a factorial upper bound over task orders#footnote[The auto-generated complexity table records the concrete upper bound used by the Rust implementation; no sharper exact bound is cited here.].

      *Example.* The canonical fixture has #m machines and #n jobs
      $
        #for (ji, job) in jobs.enumerate() {
          $J_#(ji+1) = (#job.map(((mi, len)) => $(M_#(mi+1), #len)$).join($,$))$
          if ji < n - 1 [$,$] else [.]
        }
      $
      The witness stored in the example DB orders the six tasks on $M_1$ as $(J_1^1, J_2^2, J_3^1, J_4^2, J_5^1, J_5^3)$ and the six tasks on $M_2$ as $(J_2^1, J_4^1, J_1^2, J_3^2, J_5^2, J_2^3)$. Taking the earliest schedule consistent with those machine orders yields the Gantt chart in @fig:jobshop, whose makespan is $#makespan$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o job-shop-scheduling.json",
        "pred solve job-shop-scheduling.json --solver brute-force",
        "pred evaluate job-shop-scheduling.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"))
          let scale = 0.38
          let row-h = 0.6
          let gap = 0.15

          for mi in range(m) {
            let y = -mi * (row-h + gap)
            content((-0.8, y), text(8pt, "M" + str(mi + 1)))
          }

          for block in blocks {
            let (mi, ji, ti, s, e) = block
            let x0 = s * scale
            let x1 = e * scale
            let y = -mi * (row-h + gap)
            rect(
              (x0, y - row-h / 2),
              (x1, y + row-h / 2),
              fill: colors.at(ji).transparentize(30%),
              stroke: 0.4pt + colors.at(ji),
            )
            content(((x0 + x1) / 2, y), text(6pt, "j" + str(ji + 1) + "." + str(ti + 1)))
          }

          let y-axis = -(m - 1) * (row-h + gap) - row-h / 2 - 0.2
          line((0, y-axis), (makespan * scale, y-axis), stroke: 0.4pt)
          for t in range(calc.ceil(makespan / 5) + 1).map(i => calc.min(i * 5, makespan)) {
            let x = t * scale
            line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
            content((x, y-axis - 0.25), text(6pt, str(t)))
          }
          if calc.rem(makespan, 5) != 0 {
            let x = makespan * scale
            line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
            content((x, y-axis - 0.25), text(6pt, str(makespan)))
          }
          content((makespan * scale / 2, y-axis - 0.5), text(7pt)[$t$])
        }),
        caption: [Job-shop schedule induced by the canonical machine-order witness. The optimal makespan is #makespan.],
      ) <fig:jobshop>
    ]
  ]
}

#{
  let x = load-model-example("OpenShopScheduling")
  let p = x.instance.processing_times
  let m = x.instance.num_machines
  let n = p.len()
  let cfg = x.optimal_config
  // Decode per-machine orderings: cfg[i*n..(i+1)*n] is machine i's job order
  let orders = range(m).map(i => cfg.slice(i * n, (i + 1) * n))

  // Greedy simulation to compute start times
  let machine-avail = range(m).map(_ => 0)
  let job-avail = range(n).map(_ => 0)
  let next-on = range(m).map(_ => 0)
  let start-times = range(n).map(_ => range(m).map(_ => 0))
  let finish-times = range(n).map(_ => range(m).map(_ => 0))

  let total-tasks = n * m
  let scheduled = 0
  while scheduled < total-tasks {
    // Find machine with earliest next start
    let best-start = 999999
    let best-machine = -1
    for i in range(m) {
      if next-on.at(i) < n {
        let j = orders.at(i).at(next-on.at(i))
        let s = calc.max(machine-avail.at(i), job-avail.at(j))
        if s < best-start or (s == best-start and (best-machine == -1 or i < best-machine)) {
          best-start = s
          best-machine = i
        }
      }
    }
    let i = best-machine
    let j = orders.at(i).at(next-on.at(i))
    let s = calc.max(machine-avail.at(i), job-avail.at(j))
    let f = s + p.at(j).at(i)
    start-times.at(j).at(i) = s
    finish-times.at(j).at(i) = f
    machine-avail.at(i) = f
    job-avail.at(j) = f
    next-on.at(i) += 1
    scheduled += 1
  }

  let makespan = calc.max(..range(n).map(j => calc.max(..range(m).map(i => finish-times.at(j).at(i)))))

  [
    #problem-def("OpenShopScheduling")[
      Given $m$ machines and a set $J$ of $n$ jobs, where each job $j in J$ has one task per machine $i$ with processing time $p(j, i) in ZZ^+_0$, find a non-preemptive schedule minimizing the *makespan* $max_(j,i)(sigma(j, i) + p(j, i))$, subject to:
      1. *Machine constraint:* Each machine processes at most one job at a time.
      2. *Job constraint:* Each job occupies at most one machine at a time.
      Unlike flow-shop or job-shop scheduling, there is no prescribed order for a job's tasks across machines.
    ][
      Open Shop Scheduling is problem SS14 in Garey and Johnson's catalog @garey1979 (decision version: does a schedule exist with makespan $<= D$?). NP-completeness for $m >= 3$ machines was established by Gonzalez and Sahni via reduction from Partition @gonzalez1976. The problem is solvable in polynomial time for $m = 2$ and also for the preemptive variant with any $m$ @gonzalez1976. This codebase evaluates a candidate schedule by simulating a greedy active schedule: for each step, the machine with the earliest feasible next-job start is processed next. The configuration encodes one permutation of jobs per machine (direct indices), giving $(n!)^m$ candidate orderings.

      *Example.* Let $m = #m$ machines and $n = #n$ jobs with processing times
      #align(center, math.equation([$P = #math.mat(..p.map(row => row.map(v => [#v])))$]))
      The canonical optimal orderings are:
      #align(center, table(
        columns: 2,
        align: (left, left),
        table.header([Machine], [Job order]),
        ..range(m).map(i => ([M#(i+1)], orders.at(i).map(j => [$J_#(j+1)$]).join[$,$])).flatten()
      ))
      giving the Gantt chart in @fig:openshop and makespan *#makespan*.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o open-shop-scheduling.json",
        "pred solve open-shop-scheduling.json",
        "pred evaluate open-shop-scheduling.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"), rgb("#f28e2b"), rgb("#59a14f"), rgb("#b07aa1"))
          let scale = 0.55
          let row-h = 0.6
          let gap = 0.15

          for mi in range(m) {
            let y = -mi * (row-h + gap)
            content((-0.8, y), text(8pt, "M" + str(mi + 1)))
          }

          for j in range(n) {
            for i in range(m) {
              let s = start-times.at(j).at(i)
              let f = finish-times.at(j).at(i)
              let x0 = s * scale
              let x1 = f * scale
              let y = -i * (row-h + gap)
              rect((x0, y - row-h / 2), (x1, y + row-h / 2),
                fill: colors.at(j).transparentize(30%), stroke: 0.4pt + colors.at(j))
              content(((x0 + x1) / 2, y), text(6pt, [$J_#(j + 1)$]))
            }
          }

          let y-axis = -(m - 1) * (row-h + gap) - row-h / 2 - 0.2
          line((0, y-axis), (makespan * scale, y-axis), stroke: 0.4pt)
          for t in range(makespan + 1) {
            let x = t * scale
            line((x, y-axis), (x, y-axis - 0.1), stroke: 0.4pt)
            content((x, y-axis - 0.25), text(6pt, str(t)))
          }
          content((makespan * scale / 2, y-axis - 0.5), text(7pt)[$t$])
        }),
        caption: [Open-shop schedule for #n jobs on #m machines. Optimal makespan is #makespan. Each color represents one job; no two tasks of the same job overlap in time.],
      ) <fig:openshop>
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

      #pred-commands(
        "pred create --example TimetableDesign -o timetable-design.json",
        "pred solve timetable-design.json",
        "pred evaluate timetable-design.json --config " + x.optimal_config.map(str).join(","),
      )

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

      #pred-commands(
        "pred create --example MultiprocessorScheduling -o multiprocessor-scheduling.json",
        "pred solve multiprocessor-scheduling.json",
        "pred evaluate multiprocessor-scheduling.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("ProductionPlanning")
  let n = x.instance.num_periods
  let demands = x.instance.demands
  let capacities = x.instance.capacities
  let setup-costs = x.instance.setup_costs
  let production-costs = x.instance.production_costs
  let inventory-costs = x.instance.inventory_costs
  let bound = x.instance.cost_bound
  let plan = x.optimal_config
  let prefix-production = range(n).map(i => plan.slice(0, i + 1).sum())
  let prefix-demand = range(n).map(i => demands.slice(0, i + 1).sum())
  let inventory = range(n).map(i => prefix-production.at(i) - prefix-demand.at(i))
  let production-total = range(n).map(i => production-costs.at(i) * plan.at(i)).sum()
  let inventory-total = range(n).map(i => inventory-costs.at(i) * inventory.at(i)).sum()
  let setup-total = range(n).filter(i => plan.at(i) > 0).map(i => setup-costs.at(i)).sum()
  [
    #problem-def("ProductionPlanning")[
      Given a positive integer $n$, period demands $r_1, dots, r_n in ZZ_(>= 0)$, production capacities $c_1, dots, c_n in ZZ_(>= 0)$, setup costs $b_1, dots, b_n in ZZ_(>= 0)$, per-unit production costs $p_1, dots, p_n in ZZ_(>= 0)$, per-unit inventory costs $h_1, dots, h_n in ZZ_(>= 0)$, and a bound $B in ZZ_(>= 0)$, determine whether there exist production quantities $x_1, dots, x_n$ such that $0 <= x_i <= c_i$ for every period $i$, the inventory prefix $I_i = sum_(j=1)^i (x_j - r_j)$ satisfies $I_i >= 0$ for every $i$, and $sum_(i=1)^n (p_i x_i + h_i I_i) + sum_(i: x_i > 0) b_i <= B$.
    ][
      Production Planning is the lot-sizing feasibility problem SS21 in Garey & Johnson @garey1979. Florian, Lenstra, and Rinnooy Kan show that the general problem is NP-complete even under strong restrictions, while also giving pseudo-polynomial dynamic-programming algorithms for capacitated variants @florianLenstraRinnooyKan1980. The implementation in this repository uses one bounded integer variable per period, so the registered exact baseline explores the direct witness space $product_i (c_i + 1)$; under the uniform-capacity bound $C = max_i c_i$, this becomes $O^*((C + 1)^n)$#footnote[This is the search bound induced by the configuration space exposed by the implementation, not a literature-best exact algorithm claim.].

      *Example.* Consider the canonical instance with #n periods, demands $(#demands.map(str).join(", "))$, capacities $(#capacities.map(str).join(", "))$, setup costs $(#setup-costs.map(str).join(", "))$, production costs $(#production-costs.map(str).join(", "))$, inventory costs $(#inventory-costs.map(str).join(", "))$, and budget $B = #bound$. The satisfying production plan $x = (#plan.map(str).join(", "))$ yields prefix inventories $(#inventory.map(str).join(", "))$. The verifier therefore accepts, and its cost breakdown is $#production-total + #inventory-total + #setup-total = #(production-total + inventory-total + setup-total) <= #bound$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o production-planning.json",
        "pred solve production-planning.json --solver brute-force",
        "pred evaluate production-planning.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure({
        table(
          columns: n + 1,
          align: center,
          inset: 4pt,
          table.header([*Period*], ..range(n).map(i => [#(i + 1)])),
          [$r_i$], ..range(n).map(i => [#demands.at(i)]),
          [$c_i$], ..range(n).map(i => [#capacities.at(i)]),
          [$b_i$], ..range(n).map(i => [#setup-costs.at(i)]),
          [$p_i$], ..range(n).map(i => [#production-costs.at(i)]),
          [$h_i$], ..range(n).map(i => [#inventory-costs.at(i)]),
          [$x_i$], ..range(n).map(i => [#plan.at(i)]),
          [$I_i$], ..range(n).map(i => [#inventory.at(i)]),
        )
      },
      caption: [Canonical Production Planning instance from the example DB. The documented plan meets every prefix-demand constraint and stays within the budget $B = #bound$.],
      ) <fig:production-planning>
    ]
  ]
}

#{
  let x = load-model-example("CapacityAssignment")
  [
    #problem-def("CapacityAssignment")[
      Given a finite set $C$ of communication links, an ordered set $M subset ZZ_(> 0)$ of capacities, cost and delay functions $g: C times M -> ZZ_(>= 0)$ and $d: C times M -> ZZ_(>= 0)$ such that for every $c in C$ and $i < j$ in the order of $M$ we have $g(c, i) <= g(c, j)$ and $d(c, i) >= d(c, j)$, and a delay budget $J in ZZ_(>= 0)$, find an assignment $sigma: C -> M$ minimizing $sum_(c in C) g(c, sigma(c))$ subject to $sum_(c in C) d(c, sigma(c)) <= J$.
    ][
      Capacity Assignment is the bicriteria communication-network design problem SR7 in Garey & Johnson @garey1979. The original NP-completeness proof, via reduction from Subset Sum, is due to Van Sickle and Chandy @vansicklechandy1977. The model captures discrete provisioning of communication links, where upgrading a link increases installation cost but decreases delay. The direct witness encoding implemented in this repository yields an $O^*(|M|^(|C|))$ exact algorithm by brute-force enumeration#footnote[No algorithm improving on brute-force enumeration is known for the exact witness encoding used in this repository.]. Garey and Johnson also note a pseudo-polynomial dynamic-programming formulation when the budgets are small @garey1979.

      *Example.* Let $C = {c_1, c_2, c_3}$, $M = {1, 2, 3}$, and $J = 12$. With cost rows $(1, 3, 6)$, $(2, 4, 7)$, $(1, 2, 5)$ and delay rows $(8, 4, 1)$, $(7, 3, 1)$, $(6, 3, 1)$, the optimal assignment is $sigma = (2, 2, 2)$ with total cost $3 + 4 + 2 = 9$ and total delay $4 + 3 + 3 = 10 <= 12$. For contrast, $sigma = (1, 1, 1)$ has total delay $8 + 7 + 6 = 21 > 12$ and is therefore infeasible.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o capacity-assignment.json",
        "pred solve capacity-assignment.json --solver brute-force",
        "pred evaluate capacity-assignment.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure({
        table(
          columns: (auto, auto, auto),
          inset: 4pt,
          align: left,
          table.header([*Link*], [*Cost row*], [*Delay row*]),
          [$c_1$], [$(1, 3, 6)$], [$(8, 4, 1)$],
          [$c_2$], [$(2, 4, 7)$], [$(7, 3, 1)$],
          [$c_3$], [$(1, 2, 5)$], [$(6, 3, 1)$],
        )
      },
      caption: [Canonical Capacity Assignment instance with delay budget $J = 12$. Each row lists the cost-delay trade-off for one communication link.],
      ) <fig:capacity-assignment>
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

      #pred-commands(
        "pred create --example PrecedenceConstrainedScheduling -o precedence-constrained-scheduling.json",
        "pred solve precedence-constrained-scheduling.json",
        "pred evaluate precedence-constrained-scheduling.json --config " + x.optimal_config.map(str).join(","),
      )
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

      #pred-commands(
        "pred create --example SchedulingWithIndividualDeadlines -o scheduling-with-individual-deadlines.json",
        "pred solve scheduling-with-individual-deadlines.json",
        "pred evaluate scheduling-with-individual-deadlines.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("PreemptiveScheduling")
  let n = x.instance.lengths.len()
  let m = x.instance.num_processors
  let lengths = x.instance.lengths
  let precs = x.instance.precedences
  let d_max = lengths.fold(0, (acc, l) => acc + l)
  let cfg = x.optimal_config
  // For each task t, collect active time slots from the flat binary config
  let active-slots = range(n).map(t =>
    range(d_max).filter(u => cfg.at(t * d_max + u) == 1)
  )
  let makespan = x.optimal_value
  [
    #problem-def("PreemptiveScheduling")[
      Given a set $T$ of $n$ tasks with processing lengths $ell: T -> ZZ^+$, a number $m in ZZ^+$ of identical processors, and a set of precedence constraints $prec$ on $T$, find a preemptive schedule that minimizes the makespan.

      A preemptive schedule assigns each task $t$ a (possibly non-contiguous) set $S(t) subset.eq {0, 1, dots, D_"max" - 1}$ of unit time slots, where $D_"max" = sum_t ell(t)$, such that $|S(t)| = ell(t)$ for all $t$, at most $m$ tasks are active at each slot, and for every precedence $(t_i prec t_j)$, the last slot of $t_i$ precedes the first slot of $t_j$.

      The makespan is $max_{t in T} (max S(t) + 1)$.
    ][
      Preemptive Scheduling is problem A5 SS6 in Garey & Johnson @garey1979. NP-complete in general; the special case without precedences ($m$ arbitrary) is solvable in polynomial time (McNaughton's wrap-around algorithm), and the preemptive open-shop variant is also polynomial. The configuration representation is a binary vector of length $n dot D_"max"$ encoding per-slot assignments.

      *Example.* Let $n = #n$ tasks with lengths $(#lengths.map(str).join(", "))$, $m = #m$ processors, and precedences #{precs.map(p => $t_#(p.at(0)) prec t_#(p.at(1))$).join(", ")}. Optimal makespan: $#makespan$. Schedule: #range(n).map(t => [$t_#t$ at slots $[#active-slots.at(t).map(str).join(", ")]$]).join("; ").

      #pred-commands(
        "pred create --example PreemptiveScheduling -o preemptive-scheduling.json",
        "pred solve preemptive-scheduling.json",
        "pred evaluate preemptive-scheduling.json --config " + cfg.map(str).join(","),
      )
    ]
  ]
}
#{
  let x = load-model-example("SchedulingToMinimizeWeightedCompletionTime")
  let ntasks = x.instance.lengths.len()
  let m = x.instance.num_processors
  let lengths = x.instance.lengths
  let weights = x.instance.weights
  let sigma = x.optimal_config
  // Group tasks by processor
  let tasks-by-proc = range(m).map(p =>
    range(ntasks).filter(i => sigma.at(i) == p)
  )
  [
    #problem-def("SchedulingToMinimizeWeightedCompletionTime")[
      Given a finite set $T$ of tasks with processing lengths $ell: T -> ZZ^+$ and weights $w: T -> ZZ^+$, and a number $m in ZZ^+$ of identical processors, find an assignment $p: T -> {1, dots, m}$ that minimizes the total weighted completion time $sum_(t in T) w(t) dot C(t)$, where on each processor tasks are ordered by Smith's rule (non-decreasing $ell(t) "/" w(t)$ ratio) and $C(t)$ is the completion time of task $t$ (i.e., the cumulative processing time up to and including $t$ on its assigned processor).
    ][
      Scheduling to Minimize Weighted Completion Time is problem A5 SS13 in Garey & Johnson @garey1979. NP-complete for $m = 2$ by reduction from Partition @lenstra1977, and NP-complete in the strong sense for arbitrary $m$. For a fixed assignment of tasks to processors, Smith's rule gives the optimal ordering on each processor, reducing the search space to $m^n$ processor assignments @smith1956. The problem is solvable in polynomial time when all lengths are equal or when all weights are equal @conway1967 @horn1973.

      *Example.* Let $T = {t_1, dots, t_#ntasks}$ with lengths $(#lengths.map(str).join(", "))$, weights $(#weights.map(str).join(", "))$, and $m = #m$ processors. The optimal assignment $(#sigma.map(v => str(v + 1)).join(", "))$ achieves total weighted completion time #x.optimal_value:
      #for p in range(m) [
        - Processor #(p + 1): ${#tasks-by-proc.at(p).map(i => $t_#(i + 1)$).join(", ")}$#if tasks-by-proc.at(p).len() > 0 {
          let proc-tasks = tasks-by-proc.at(p)
          let elapsed = 0
          let contributions = ()
          for t in proc-tasks {
            elapsed = elapsed + lengths.at(t)
            contributions.push($#elapsed times #(weights.at(t)) = #(elapsed * weights.at(t))$)
          }
          [ -- contributions: #contributions.join(", ")]
        }
      ]

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o scheduling-wct.json",
        "pred solve scheduling-wct.json --solver brute-force",
        "pred evaluate scheduling-wct.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure({
        canvas(length: 1cm, {
          import draw: *
          let scale = 0.2
          let width = 1.2
          let gap = 0.8
          let colors = (
            rgb("#4e79a7"),
            rgb("#e15759"),
            rgb("#76b7b2"),
            rgb("#f28e2b"),
            rgb("#59a14f"),
          )

          for p in range(m) {
            let x0 = p * (width + gap)
            let max-time = tasks-by-proc.at(p).fold(0, (acc, t) => acc + lengths.at(t))
            rect((x0, 0), (x0 + width, max-time * scale), stroke: 0.8pt + black)
            let y = 0
            for task in tasks-by-proc.at(p) {
              let len = lengths.at(task)
              let col = colors.at(task)
              rect(
                (x0, y),
                (x0 + width, y + len * scale),
                fill: col.transparentize(25%),
                stroke: 0.4pt + col,
              )
              content(
                (x0 + width / 2, y + len * scale / 2),
                text(7pt, fill: white)[$t_#(task + 1)$],
              )
              y += len * scale
            }
            content((x0 + width / 2, -0.3), text(8pt)[$P_#(p + 1)$])
          }
        })
      },
      caption: [Canonical Scheduling to Minimize Weighted Completion Time instance with #ntasks tasks on #m processors. Tasks are ordered on each processor by Smith's rule.],
      ) <fig:scheduling-wct>
    ]
  ]
}

// Reduction: SchedulingToMinimizeWeightedCompletionTime -> ILP
#reduction-rule("SchedulingToMinimizeWeightedCompletionTime", "ILP",
  example: false,
)[
  This $O(n^2 m)$ reduction constructs an ILP with binary assignment variables $x_(t,p)$, integer completion-time variables $C_t$, and binary ordering variables $y_(i,j)$ for task pairs. Big-M disjunctive constraints enforce non-overlapping execution on shared processors.
][
  _Construction._ Let $n = |T|$ and $m$ be the number of processors. Create $n m$ binary assignment variables $x_(t,p) in {0, 1}$ (task $t$ on processor $p$), $n$ integer completion-time variables $C_t$, and $n(n-1)/2$ binary ordering variables $y_(i,j)$ for $i < j$. The constraints are:
  (1) Assignment: $sum_p x_(t,p) = 1$ for each $t$.
  (2) Completion bounds: $C_t >= ell(t)$ for each $t$.
  (3) Disjunctive: for each pair $(i,j)$ with $i < j$ and each processor $p$, big-M constraints ensure that if both tasks are on processor $p$, one must complete before the other starts.
  The objective minimizes $sum_t w(t) dot C_t$.

  _Correctness._ ($arrow.r.double$) Any valid schedule gives a feasible ILP solution with the same objective. ($arrow.l.double$) Any ILP solution encodes a valid assignment and non-overlapping schedule.

  _Solution extraction._ For each task $t$, find the processor $p$ with $x_(t,p) = 1$.
]

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

      #pred-commands(
        "pred create --example SequencingWithinIntervals -o sequencing-within-intervals.json",
        "pred solve sequencing-within-intervals.json",
        "pred evaluate sequencing-within-intervals.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let x = load-model-example("MinimumTardinessSequencing", variant: (weight: "One"))
  let ntasks = x.instance.lengths.len()
  let deadlines = x.instance.deadlines
  let precs = x.instance.precedences
  let sol = (config: x.optimal_config, metric: x.optimal_value)
  let tardy-count = metric-value(sol.metric)
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

      #pred-commands(
        "pred create --example MinimumTardinessSequencing -o minimum-tardiness-sequencing.json",
        "pred solve minimum-tardiness-sequencing.json",
        "pred evaluate minimum-tardiness-sequencing.json --config " + x.optimal_config.map(str).join(","),
      )

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
  let opt = metric-value(sol.metric)
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

      #pred-commands(
        "pred create --example SequencingToMinimizeWeightedCompletionTime -o sequencing-to-minimize-weighted-completion-time.json",
        "pred solve sequencing-to-minimize-weighted-completion-time.json",
        "pred evaluate sequencing-to-minimize-weighted-completion-time.json --config " + x.optimal_config.map(str).join(","),
      )

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

      #pred-commands(
        "pred create --example SequencingToMinimizeWeightedTardiness -o sequencing-to-minimize-weighted-tardiness.json",
        "pred solve sequencing-to-minimize-weighted-tardiness.json",
        "pred evaluate sequencing-to-minimize-weighted-tardiness.json --config " + x.optimal_config.map(str).join(","),
      )

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
      Given a set $T$ of $n$ tasks, a precedence relation $prec.eq$ on $T$, and an integer cost function $c: T -> ZZ$ (negative values represent profits), find a one-machine schedule $sigma: T -> {1, 2, dots, n}$ that respects the precedence constraints and minimizes the maximum cumulative cost
      $min_sigma max_(t in T) sum_(sigma(t') lt.eq sigma(t)) c(t').$
    ][
      Sequencing to Minimize Maximum Cumulative Cost is the scheduling problem SS7 in Garey & Johnson @garey1979. It is NP-complete by transformation from Register Sufficiency, even when every task cost is in ${-1, 0, 1}$ @garey1979. The problem models precedence-constrained task systems with resource consumption and release, where a negative cost corresponds to a profit or resource refund accumulated as the schedule proceeds.

      When the precedence constraints form a series-parallel digraph, #cite(<abdelWahabKameda1978>, form: "prose") gave a polynomial-time algorithm running in $O(n^2)$ time. #cite(<monmaSidney1979>, form: "prose") placed the problem in a broader family of sequencing objectives solvable efficiently on series-parallel precedence structures. The implementation here uses Lehmer-code enumeration of task orders, so the direct exact search induced by the model runs in $O(n!)$ time.

      *Example.* Consider $n = #ntasks$ tasks with costs $(#costs.map(c => str(c)).join(", "))$ and precedence constraints #{precs.map(p => [$t_#(p.at(0) + 1) prec.eq t_#(p.at(1) + 1)$]).join(", ")}. The optimal schedule $(#schedule.map(t => $t_#(t + 1)$).join(", "))$ has cumulative sums $(#prefix-sums.map(v => str(v)).join(", "))$, achieving a maximum cumulative cost of $#x.optimal_value$.

      #pred-commands(
        "pred create --example SequencingToMinimizeMaximumCumulativeCost -o sequencing-to-minimize-maximum-cumulative-cost.json",
        "pred solve sequencing-to-minimize-maximum-cumulative-cost.json",
        "pred evaluate sequencing-to-minimize-maximum-cumulative-cost.json --config " + x.optimal_config.map(str).join(","),
      )

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
        caption: [An optimal schedule for Sequencing to Minimize Maximum Cumulative Cost. Orange boxes add cost, teal boxes release cost, and the displayed prefix sums $(#prefix-sums.map(v => str(v)).join(", "))$ achieve a maximum of $#calc.max(..prefix-sums)$.],
      ) <fig:seq-max-cumulative>
    ]
  ]
}

#{
  let x = load-model-example("SequencingToMinimizeTardyTaskWeight")
  let lengths = x.instance.lengths
  let weights = x.instance.weights
  let deadlines = x.instance.deadlines
  let ntasks = lengths.len()
  let schedule = x.optimal_config
  let completions = {
    let t = 0
    let result = ()
    for task in schedule {
      t += lengths.at(task)
      result.push(t)
    }
    result
  }
  let tardy-weights = schedule.enumerate().map(((pos, task)) => {
    if completions.at(pos) > deadlines.at(task) { weights.at(task) } else { 0 }
  })
  [
    #problem-def("SequencingToMinimizeTardyTaskWeight")[
      Given a set $T$ of $n$ tasks, a processing-time function $ell: T -> ZZ^+$, a weight function $w: T -> ZZ^+$, and a deadline function $d: T -> ZZ^+$, find a one-machine schedule that minimizes
      $sum_(t in T) w(t) U(t),$
      where $U(t) = 1$ if the completion time $C(t) > d(t)$ (task $t$ is tardy) and $U(t) = 0$ otherwise.
    ][
      Sequencing to Minimize Tardy Task Weight is problem SS8 in Garey & Johnson @garey1979, usually written $1 || sum w_j U_j$. The unweighted variant $1 || sum U_j$ (minimize number of tardy tasks) is solvable in $O(n log n)$ by Moore's algorithm @moore1968, but the weighted version is NP-complete. The problem is closely related to $1 || sum w_j T_j$ (Sequencing to Minimize Weighted Tardiness) but differs in using a 0/1 indicator $U_j$ instead of the actual tardiness $T_j = max(0, C_j - d_j)$.

      Configurations are direct permutation encodings: the config vector $(sigma_0, dots, sigma_{n-1})$ specifies which task occupies each position, i.e., $sigma_p$ is the index of the task scheduled at position $p$. A configuration is valid iff it is a permutation of $\{0, dots, n-1\}$.

      *Example.* Consider $n = #ntasks$ tasks with lengths $ell = (#lengths.map(v => str(v)).join(", "))$, weights $w = (#weights.map(v => str(v)).join(", "))$, and deadlines $d = (#deadlines.map(v => str(v)).join(", "))$. The optimal schedule $(#schedule.map(t => $t_(#(t + 1))$).join(", "))$ achieves completion times $(#completions.map(v => str(v)).join(", "))$. Tasks with completion time exceeding their deadline contribute weights $(#tardy-weights.map(v => str(v)).join(", "))$, giving total tardy weight $#x.optimal_value$.

      #pred-commands(
        "pred create --example SequencingToMinimizeTardyTaskWeight -o sequencing-to-minimize-tardy-task-weight.json",
        "pred solve sequencing-to-minimize-tardy-task-weight.json",
        "pred evaluate sequencing-to-minimize-tardy-task-weight.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("SequencingWithDeadlinesAndSetUpTimes")
  let lengths = x.instance.lengths
  let deadlines = x.instance.deadlines
  let compilers = x.instance.compilers
  let setup_times = x.instance.setup_times
  let ntasks = lengths.len()
  let schedule = x.optimal_config
  let completions = {
    let t = 0
    let prev_compiler = none
    let result = ()
    for task in schedule {
      if prev_compiler != none and prev_compiler != compilers.at(task) {
        t += setup_times.at(compilers.at(task))
      }
      t += lengths.at(task)
      result.push(t)
      prev_compiler = compilers.at(task)
    }
    result
  }
  [
    #problem-def("SequencingWithDeadlinesAndSetUpTimes")[
      Given a set $T$ of $n$ tasks, a processing-time function $ell: T -> ZZ^+$, a deadline function $d: T -> ZZ^+$, a compiler assignment $k: T -> C$ for a finite set $C$ of compilers, and a setup-time function $s: C -> ZZ_(>=0)$, determine whether there exists a single-machine schedule such that every task $t$ completes by its deadline $d(t)$, where an additional setup time $s(k(t))$ is charged before $t$ whenever $k(t) != k(t')$ for the immediately preceding task $t'$.
    ][
      Sequencing with Deadlines and Set-Up Times is problem SS14 in Garey & Johnson @garey1979, usually written $1 | s_(i j) | "feasibility"$. The problem is NP-complete even when all setup times are equal. It generalises Sequencing with Release Times and Deadlines (SS13) by replacing release-time windows with compiler-switch penalties.

      Configurations are direct permutation encodings: the config vector $(sigma_0, dots, sigma_(n-1))$ specifies which task occupies each position, and a configuration is valid iff it is a permutation of $\{0, dots, n-1\}$.

      *Example.* Consider $n = #ntasks$ tasks with lengths $ell = (#lengths.map(v => str(v)).join(", "))$, deadlines $d = (#deadlines.map(v => str(v)).join(", "))$, compilers $k = (#compilers.map(v => str(v)).join(", "))$, and setup times $s = (#setup_times.map(v => str(v)).join(", "))$. The schedule $(#schedule.map(t => $t_(#(t + 1))$).join(", "))$ achieves completion times $(#completions.map(v => str(v)).join(", "))$; every task meets its deadline, so the instance is feasible.

      #pred-commands(
        "pred create --example SequencingWithDeadlinesAndSetUpTimes -o sequencing-with-deadlines-and-set-up-times.json",
        "pred solve sequencing-with-deadlines-and-set-up-times.json",
        "pred evaluate sequencing-with-deadlines-and-set-up-times.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("IntegralFlowHomologousArcs")
  let arcs = x.instance.graph.arcs
  let sol = x.optimal_config
  let source = x.instance.source
  let sink = x.instance.sink
  let requirement = x.instance.requirement
  [
    #problem-def("IntegralFlowHomologousArcs")[
      Given a directed graph $G = (V, A)$ with source $s in V$, sink $t in V$, arc capacities $c: A -> ZZ^+$, requirement $R in ZZ^+$, and a set $H subset.eq A times A$ of homologous arc pairs, determine whether there exists an integral flow function $f: A -> ZZ_(>= 0)$ such that $f(a) <= c(a)$ for every $a in A$, flow is conserved at every vertex in $V backslash {s, t}$, $f(a) = f(a')$ for every $(a, a') in H$, and the net flow into $t$ is at least $R$.
    ][
      Integral Flow with Homologous Arcs is the single-commodity equality-constrained flow problem listed as ND35 in Garey & Johnson @garey1979. Their catalog records the NP-completeness result attributed to Sahni and notes that the unit-capacity restriction remains hard, while the corresponding non-integral relaxation is polynomial-time equivalent to linear programming @garey1979.

      The implementation uses one integer variable per arc, so exhaustive search over the induced configuration space runs in $O((C + 1)^m)$ for $m = |A|$ and $C = max_(a in A) c(a)$#footnote[This is the exact search bound induced by the implemented per-arc domains $f(a) in {0, dots, c(a)}$. In the unit-capacity special case, it simplifies to $O(2^m)$.].

      *Example.* The canonical fixture instance has source $s = v_#source$, sink $t = v_#sink$, unit capacities on all eight arcs, requirement $R = #requirement$, and homologous pairs $(a_2, a_5)$ and $(a_4, a_3)$. The stored satisfying configuration routes one unit along $0 -> 1 -> 3 -> 5$ and one unit along $0 -> 2 -> 4 -> 5$. Thus the paired arcs $(1,3)$ and $(2,4)$ both carry 1, while $(1,4)$ and $(2,3)$ both carry 0. Every nonterminal vertex has equal inflow and outflow, and the sink receives two units of flow, so the verifier returns true.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o integral-flow-homologous-arcs.json",
        "pred solve integral-flow-homologous-arcs.json --solver brute-force",
        "pred evaluate integral-flow-homologous-arcs.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let orange = rgb("#f28e2b")
          let red = rgb("#e15759")
          let gray = luma(185)
          let positions = (
            (0, 0),
            (1.6, 1.1),
            (1.6, -1.1),
            (3.2, 1.1),
            (3.2, -1.1),
            (4.8, 0),
          )
          let labels = (
            [$s = v_0$],
            [$v_1$],
            [$v_2$],
            [$v_3$],
            [$v_4$],
            [$t = v_5$],
          )
          for (idx, (u, v)) in arcs.enumerate() {
            let stroke = if idx == 3 or idx == 4 {
              (paint: orange, thickness: 1.3pt, dash: "dashed")
            } else if sol.at(idx) == 1 {
              (paint: blue, thickness: 1.8pt)
            } else {
              (paint: gray, thickness: 0.7pt)
            }
            line(
              positions.at(u),
              positions.at(v),
              stroke: stroke,
              mark: (end: "straight", scale: 0.5),
            )
          }
          for (i, pos) in positions.enumerate() {
            let fill = if i == source { blue } else if i == sink { red } else { white }
            g-node(
              pos,
              name: "ifha-" + str(i),
              fill: fill,
              label: if i == source or i == sink {
                text(fill: white)[#labels.at(i)]
              } else {
                labels.at(i)
              },
            )
          }
          content((2.4, 1.55), text(8pt, fill: blue)[$f(a_2) = f(a_5) = 1$])
          content((2.4, -1.55), text(8pt, fill: orange)[$f(a_4) = f(a_3) = 0$])
        }),
        caption: [Canonical YES instance for Integral Flow with Homologous Arcs. Solid blue arcs carry the satisfying integral flow; dashed orange arcs form the second homologous pair, constrained to equal zero.],
      ) <fig:integral-flow-homologous-arcs>
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

#problem-def("MinimumEdgeCostFlow")[
  Given a directed graph $G = (V, A)$ with arc capacities $c: A -> ZZ^+$, arc prices $p: A -> ZZ$, a source vertex $s$, a sink vertex $t$, and a flow requirement $R in ZZ^+$, find an integral flow $f: A -> ZZ_(>= 0)$ of value at least $R$ that minimizes the total edge cost $sum_(a in A: f(a) > 0) p(a)$ — the sum of prices of arcs carrying nonzero flow.
][
  Minimum Edge-Cost Flow is an NP-hard network design problem that arises in telecommunications and logistics, where there is a fixed cost (price) for activating each link, independent of the actual traffic volume. Unlike the classical minimum-cost flow problem (where cost is proportional to flow), the edge-cost variant introduces a combinatorial selection aspect: choosing which arcs to activate. The problem is closely related to fixed-charge network flow problems @garey1979.

  The brute-force bound $(C + 1)^(|A|)$ arises from enumerating all possible integral flow vectors, where $C = max_(a in A) c(a)$.#footnote[No sub-exponential exact algorithm is known for Minimum Edge-Cost Flow.]

  *Example.* Consider a directed graph with 5 vertices, source $s = 0$, sink $t = 4$, requirement $R = 3$, and 6 arcs with capacities $c(a) = 2$ for all arcs. The prices are $p(0,1) = 3$, $p(0,2) = 1$, $p(0,3) = 2$, and all arcs entering the sink have price 0. The optimal flow routes 1 unit via vertex 2 and 2 units via vertex 3, activating 4 arcs for a total edge cost of $1 + 2 + 0 + 0 = 3$.

  #figure(
    canvas(length: 1cm, {
      import draw: *
      let positions = (
        (0, 0),    // 0 = s
        (2, 1.5),  // 1
        (2, 0),    // 2
        (2, -1.5), // 3
        (4, 0),    // 4 = t
      )
      let labels = ($s$, $1$, $2$, $3$, $t$)
      let arcs = ((0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4))
      let prices = (3, 1, 2, 0, 0, 0)
      // Optimal arcs: (0,2), (0,3), (2,4), (3,4) — indices 1, 2, 4, 5
      let opt-arcs = (1, 2, 4, 5)

      // Draw arcs
      for (idx, (u, v)) in arcs.enumerate() {
        let from = positions.at(u)
        let to = positions.at(v)
        let is-opt = opt-arcs.contains(idx)
        let color = if is-opt { blue } else { gray.darken(20%) }
        let thickness = if is-opt { 1.2pt } else { 0.6pt }
        line(from, to, stroke: (paint: color, thickness: thickness), mark: (end: "straight", scale: 0.5), name: "arc" + str(idx))
        // Price label
        content("arc" + str(idx) + ".mid", text(7pt, $#prices.at(idx)$), fill: white, frame: "rect", padding: 0.06, stroke: none)
      }

      // Draw vertices
      for (k, pos) in positions.enumerate() {
        let fill = if k == 0 or k == 4 { blue.lighten(70%) } else { white }
        circle(pos, radius: 0.3, fill: fill, stroke: 0.6pt, name: str(k))
        content(pos, text(8pt, labels.at(k)))
      }
    }),
    caption: [Minimum Edge-Cost Flow: optimal flow (blue) routes via vertices 2 and 3, total edge cost 3. Arc labels show prices.],
  ) <fig:mecf>
]

#{
  let x = load-model-example("IntegralFlowBundles")
  let source = x.instance.source
  let sink = x.instance.sink
  [
    #problem-def("IntegralFlowBundles")[
      Given a directed graph $G = (V, A)$, specified vertices $s, t in V$, a family of arc bundles $I_1, dots, I_k subset.eq A$ whose union covers $A$, positive bundle capacities $c_1, dots, c_k$, and a requirement $R in ZZ^+$, determine whether there exists an integral flow $f: A -> ZZ_(>= 0)$ such that (1) $sum_(a in I_j) f(a) <= c_j$ for every bundle $j$, (2) flow is conserved at every vertex in $V backslash {s, t}$, and (3) the net flow into $t$ is at least $R$.
    ][
      Integral Flow with Bundles is the shared-capacity single-commodity flow problem listed as ND36 in Garey \& Johnson @garey1979. Sahni introduced it as one of a family of computationally related network problems and showed that the bundled-capacity variant is NP-complete even in a very sparse unit-capacity regime @sahni1974.

      The implementation keeps one non-negative integer variable per directed arc. Unlike ordinary max-flow, the usable range of an arc is not determined by an intrinsic per-arc capacity; it is bounded instead by the smallest bundle capacity among the bundles that contain that arc. The registered $O(2^m)$ catalog bound therefore reflects the unit-capacity case with $m = |A|$, which is exactly the regime highlighted by Garey \& Johnson and Sahni.#footnote[No exact worst-case algorithm improving on brute-force is claimed here for the bundled-capacity formulation.]

      *Example.* The canonical YES instance has source $s = v_#source$, sink $t = v_#sink$, and arcs $(0,1)$, $(0,2)$, $(1,3)$, $(2,3)$, $(1,2)$, $(2,1)$. The three bundles are $I_1 = {(0,1), (0,2)}$, $I_2 = {(1,3), (2,1)}$, and $I_3 = {(2,3), (1,2)}$, each with capacity 1. Sending one unit along the path $0 -> 1 -> 3$ yields the flow vector $(1, 0, 1, 0, 0, 0)$: bundle $I_1$ contributes $1 + 0 = 1$, bundle $I_2$ contributes $1 + 0 = 1$, bundle $I_3$ contributes $0 + 0 = 0$, and the only nonterminal vertices $v_1, v_2$ satisfy conservation. If the requirement is raised from $R = 1$ to $R = 2$, the same gadget becomes infeasible because $I_1$ caps the total outflow leaving the source at one unit.

      #pred-commands(
        "pred create --example IntegralFlowBundles -o integral-flow-bundles.json",
        "pred solve integral-flow-bundles.json",
        "pred evaluate integral-flow-bundles.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 1cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let orange = rgb("#f28e2b")
          let teal = rgb("#76b7b2")
          let gray = luma(185)
          let positions = (
            (0, 0),
            (2.2, 1.3),
            (2.2, -1.3),
            (4.4, 0),
          )

          line(positions.at(0), positions.at(1), stroke: (paint: blue, thickness: 2pt), mark: (end: "straight", scale: 0.5))
          line(positions.at(0), positions.at(2), stroke: (paint: blue.lighten(35%), thickness: 1.0pt), mark: (end: "straight", scale: 0.5))
          line(positions.at(1), positions.at(3), stroke: (paint: orange, thickness: 2pt), mark: (end: "straight", scale: 0.5))
          line(positions.at(2), positions.at(3), stroke: (paint: teal, thickness: 1.0pt), mark: (end: "straight", scale: 0.5))
          line((2.0, 1.0), (3.0, 0.0), (2.0, -1.0), stroke: (paint: teal, thickness: 1.0pt), mark: (end: "straight", scale: 0.5))
          line((2.4, -1.0), (1.4, 0.0), (2.4, 1.0), stroke: (paint: orange, thickness: 1.0pt), mark: (end: "straight", scale: 0.5))

          for (i, pos) in positions.enumerate() {
            let fill = if i == source { blue } else if i == sink { rgb("#e15759") } else { white }
            g-node(pos, name: "ifb-" + str(i), fill: fill, label: if i == source or i == sink { text(fill: white)[$v_#i$] } else { [$v_#i$] })
          }

          content((1.0, 1.0), text(8pt, fill: blue)[$I_1, c = 1$])
          content((3.3, 1.0), text(8pt, fill: orange)[$I_2, c = 1$])
          content((3.3, -1.0), text(8pt, fill: teal)[$I_3, c = 1$])
          content((2.2, 1.8), text(8pt)[$f(0,1) = 1$])
          content((3.4, 1.55), text(8pt)[$f(1,3) = 1$])
        }),
        caption: [Canonical YES instance for Integral Flow with Bundles. Thick blue/orange arcs carry the satisfying flow $0 -> 1 -> 3$, while the lighter arcs show the two unused alternatives coupled into bundles $I_1$, $I_2$, and $I_3$.],
      ) <fig:integral-flow-bundles>
    ]
  ]
}

#{
  let x = load-model-example("IntegralFlowWithMultipliers")
  let config = x.optimal_config
  [
    #problem-def("IntegralFlowWithMultipliers")[
      Given a directed graph $G = (V, A)$, distinguished vertices $s, t in V$, arc capacities $c: A -> ZZ^+$, vertex multipliers $h: V backslash {s, t} -> ZZ^+$, and a requirement $R in ZZ^+$, determine whether there exists an integral flow function $f: A -> ZZ_(>= 0)$ such that (1) $f(a) <= c(a)$ for every $a in A$, (2) for each nonterminal vertex $v in V backslash {s, t}$, the value $h(v)$ times the total inflow into $v$ equals the total outflow from $v$, and (3) the net flow into $t$ is at least $R$.
    ][
      Integral Flow With Multipliers is Garey and Johnson's gain/loss network problem ND33 @garey1979. Sahni includes the same integral vertex-multiplier formulation among his computationally related problems, where partition-style reductions show that adding discrete gain factors destroys the ordinary max-flow structure @sahni1974. The key wrinkle is that conservation is no longer symmetric: one unit entering a vertex may force several units to leave, so the feasible integral solutions behave more like multiplicative gadgets than classical flow balances.

      When every multiplier equals $1$, the model collapses to ordinary single-commodity max flow and becomes polynomial-time solvable by the standard network-flow machinery summarized in Garey and Johnson @garey1979. Jewell studies a different continuous flow-with-gains model in which gain factors live on arcs and the flow may be fractional @jewell1962. That continuous relaxation remains polynomially tractable, so it should not be conflated with the NP-complete integral vertex-multiplier decision problem catalogued here. In this implementation the witness stores one bounded integer variable per arc, giving the direct exact-search bound $O((C + 1)^m)$ where $m = |A|$ and $C = max_(a in A) c(a)$.

      *Example.* The canonical fixture encodes the Partition multiset ${2, 3, 4, 5, 6, 4}$ using source $s = v_0$, sink $t = v_7$, six unit-capacity arcs out of $s$, six sink arcs with capacities $(2, 3, 4, 5, 6, 4)$, and multipliers $(2, 3, 4, 5, 6, 4)$ on the intermediate vertices. Setting the source arcs to $v_1$, $v_3$, and $v_5$ to $1$ forces outgoing sink arcs of $2$, $4$, and $6$, respectively. The sink therefore receives net inflow $2 + 4 + 6 = 12$, exactly meeting the requirement $R = 12$.

      #pred-commands(
        "pred create --example IntegralFlowWithMultipliers -o integral-flow-with-multipliers.json",
        "pred solve integral-flow-with-multipliers.json --solver brute-force",
        "pred evaluate integral-flow-with-multipliers.json --config " + config.map(str).join(","),
      )

      #figure(
        canvas(length: 0.9cm, {
          import draw: *
          let blue = graph-colors.at(0)
          let gray = luma(180)
          let source = (0, 0)
          let sink = (6, 0)
          let mids = (
            (2.4, 2.5),
            (2.4, 1.5),
            (2.4, 0.5),
            (2.4, -0.5),
            (2.4, -1.5),
            (2.4, -2.5),
          )
          let labels = (
            [$v_1, h = 2$],
            [$v_2, h = 3$],
            [$v_3, h = 4$],
            [$v_4, h = 5$],
            [$v_5, h = 6$],
            [$v_6, h = 4$],
          )
          let active = (0, 2, 4)

          for (i, pos) in mids.enumerate() {
            let chosen = active.contains(i)
            let color = if chosen { blue } else { gray }
            let thickness = if chosen { 1.3pt } else { 0.6pt }
            line(source, pos, stroke: (paint: color, thickness: thickness), mark: (end: "straight", scale: 0.45))
            line(pos, sink, stroke: (paint: color, thickness: thickness), mark: (end: "straight", scale: 0.45))
            circle(pos, radius: 0.22, fill: if chosen { blue.lighten(75%) } else { white }, stroke: 0.6pt)
            content((pos.at(0) + 0.85, pos.at(1)), text(6.5pt, labels.at(i)))
          }

          circle(source, radius: 0.24, fill: blue.lighten(75%), stroke: 0.6pt)
          circle(sink, radius: 0.24, fill: blue.lighten(75%), stroke: 0.6pt)
          content(source, text(7pt, [$s = v_0$]))
          content(sink, text(7pt, [$t = v_7$]))
        }),
        caption: [Integral Flow With Multipliers: the blue branches send one unit from $s$ into $v_1$, $v_3$, and $v_5$, forcing sink inflow $2 + 4 + 6 = 12$ at $t$.],
      ) <fig:ifwm>
    ]
  ]
}

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

      #pred-commands(
        "pred create --example ConjunctiveBooleanQuery -o conjunctive-boolean-query.json",
        "pred solve conjunctive-boolean-query.json",
        "pred evaluate conjunctive-boolean-query.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("ConsecutiveOnesMatrixAugmentation")
  let A = x.instance.matrix
  let m = A.len()
  let n = if m > 0 { A.at(0).len() } else { 0 }
  let K = x.instance.bound
  let perm = x.optimal_config
  let A-int = A.map(row => row.map(v => if v { 1 } else { 0 }))
  let reordered = A.map(row => perm.map(c => if row.at(c) { 1 } else { 0 }))
  let total-flips = 0
  for row in reordered {
    let first = none
    let last = none
    let count = 0
    for (j, value) in row.enumerate() {
      if value == 1 {
        if first == none {
          first = j
        }
        last = j
        count += 1
      }
    }
    if first != none and last != none {
      total-flips += last - first + 1 - count
    }
  }
  [
    #problem-def("ConsecutiveOnesMatrixAugmentation")[
      Given an $m times n$ binary matrix $A$ and a nonnegative integer $K$, determine whether there exists a matrix $A'$, obtained from $A$ by changing at most $K$ zero entries to one, such that some permutation of the columns of $A'$ has the consecutive ones property.
    ][
      Consecutive Ones Matrix Augmentation is problem SR16 in Garey & Johnson @garey1979. It asks whether a binary matrix can be repaired by a bounded number of augmenting flips so that every row's 1-entries become contiguous after reordering the columns. This setting appears in information retrieval and DNA physical mapping, where matrices close to the consecutive ones property can still encode useful interval structure. Booth and Lueker showed that testing whether a matrix already has the consecutive ones property is polynomial-time via PQ-trees @booth1976, but allowing bounded augmentation makes the decision problem NP-complete @booth1975. The direct exhaustive search tries all $n!$ column permutations and, for each one, computes the minimum augmentation cost by filling the holes between the first and last 1 in every row#footnote[No algorithm improving on brute-force permutation enumeration is known for the general problem in this repository's supported setting.].

      *Example.* Consider the $#m times #n$ matrix $A = mat(#A-int.map(row => row.map(v => str(v)).join(", ")).join("; "))$ with $K = #K$. Under the permutation $pi = (#perm.map(p => str(p)).join(", "))$, the reordered rows are #reordered.enumerate().map(((i, row)) => [$r_#(i + 1) = (#row.map(v => str(v)).join(", "))$]).join(", "). The first row becomes $(1, 0, 1, 0, 1)$, so filling the two interior gaps yields $(1, 1, 1, 1, 1)$. The other three rows already have consecutive 1-entries under the same order, so the total augmentation cost is #total-flips and #total-flips $<= #K$, making the instance satisfiable.

      #pred-commands(
        "pred create --example ConsecutiveOnesMatrixAugmentation -o consecutive-ones-matrix-augmentation.json",
        "pred solve consecutive-ones-matrix-augmentation.json --solver brute-force",
        "pred evaluate consecutive-ones-matrix-augmentation.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        align(center, math.equation([$A = #math.mat(..A-int.map(row => row.map(v => [#v])))$])),
        caption: [The canonical $#m times #n$ example matrix for Consecutive Ones Matrix Augmentation. The permutation $pi = (#perm.map(p => str(p)).join(", "))$ makes only the first row need augmentation, and exactly two zero-to-one flips suffice.],
      ) <fig:coma-example>
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

      #pred-commands(
        "pred create --example ConsecutiveOnesSubmatrix -o consecutive-ones-submatrix.json",
        "pred solve consecutive-ones-submatrix.json",
        "pred evaluate consecutive-ones-submatrix.json --config " + x.optimal_config.map(str).join(","),
      )

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

#{
  let x = load-model-example("SparseMatrixCompression")
  let A = x.instance.matrix
  let m = A.len()
  let n = if m > 0 { A.at(0).len() } else { 0 }
  let K = x.instance.bound_k
  let cfg = x.optimal_config
  let shifts = cfg.map(v => v + 1)
  let storage = (4, 1, 2, 3, 1, 0)
  let A-int = A.map(row => row.map(v => if v { 1 } else { 0 }))
  let row-colors = (
    graph-colors.at(0),
    rgb("#f28e2b"),
    rgb("#76b7b2"),
    rgb("#e15759"),
  )
  [
    #problem-def("SparseMatrixCompression")[
      Given an $m times n$ binary matrix $A$ and a positive integer $K$, determine whether there exist a shift function $s: \{1, dots, m\} -> \{1, dots, K\}$ and a storage vector $b in \{0, 1, dots, m\}^{n + K}$ such that, for every row $i$ and column $j$, $A_(i j) = 1$ if and only if $b_(s(i) + j - 1) = i$.
    ][
      Sparse Matrix Compression appears as problem SR13 in Garey and Johnson @garey1979. It models row-overlay compression for sparse lookup tables: rows may share storage positions only when their shifted 1-entries never demand different row labels from the same slot. The implementation in this crate searches over row shifts only, then reconstructs the implied storage vector internally. This yields the direct exact bound $O(K^m dot m dot n)$ for $m$ rows and $n$ columns.#footnote[The storage vector is not enumerated as part of the configuration space. Once the shifts are fixed, every occupied slot is forced by the 1-entries of the shifted rows.]

      *Example.* Let $A = mat(#A-int.map(row => row.map(v => str(v)).join(", ")).join("; "))$ and $K = #K$. The stored config $(#cfg.map(str).join(", "))$ encodes the one-based shifts $s = (#shifts.map(str).join(", "))$. These shifts place the four row supports at positions $\{2, 5\}$, $\{3\}$, $\{4\}$, and $\{1\}$ respectively, so the supports are pairwise disjoint. The implied overlay vector is therefore $b = (#storage.map(str).join(", "))$, and this is the unique satisfying shift assignment among the $2^4 = 16$ configs in the canonical fixture.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o sparse-matrix-compression.json",
        "pred solve sparse-matrix-compression.json",
        "pred evaluate sparse-matrix-compression.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 0.7cm, {
          import draw: *
          let cell-size = 0.9
          let gap = 0.08
          let storage-x = 6.2

          for i in range(m) {
            for j in range(n) {
              let val = A-int.at(i).at(j)
              let fill = if val == 1 {
                row-colors.at(i).transparentize(30%)
              } else {
                white
              }
              rect(
                (j * cell-size, -i * cell-size),
                (j * cell-size + cell-size - gap, -i * cell-size - cell-size + gap),
                fill: fill,
                stroke: 0.3pt + luma(180),
              )
              content(
                (j * cell-size + (cell-size - gap) / 2, -i * cell-size - (cell-size - gap) / 2),
                text(8pt, str(val)),
              )
            }
            content(
              (-0.55, -i * cell-size - (cell-size - gap) / 2),
              text(7pt)[$r_#(i + 1)$],
            )
            content(
              (4.6, -i * cell-size - (cell-size - gap) / 2),
              text(7pt)[$s_#(i + 1) = #shifts.at(i)$],
            )
          }

          for j in range(n) {
            content(
              (j * cell-size + (cell-size - gap) / 2, 0.45),
              text(7pt)[$c_#(j + 1)$],
            )
          }

          content((5.45, -1.35), text(8pt, weight: "bold")[overlay])

          for j in range(storage.len()) {
            let label = storage.at(j)
            let fill = if label == 0 {
              white
            } else {
              row-colors.at(label - 1).transparentize(30%)
            }
            rect(
              (storage-x + j * cell-size, -1.5 * cell-size),
              (storage-x + j * cell-size + cell-size - gap, -2.5 * cell-size + gap),
              fill: fill,
              stroke: 0.3pt + luma(180),
            )
            content(
              (storage-x + j * cell-size + (cell-size - gap) / 2, -2.0 * cell-size + gap / 2),
              text(8pt, str(label)),
            )
            content(
              (storage-x + j * cell-size + (cell-size - gap) / 2, -0.8 * cell-size),
              text(7pt)[$b_#(j + 1)$],
            )
          }
        }),
        caption: [Canonical Sparse Matrix Compression YES instance. Row-colored 1-entries on the left are shifted into the overlay vector on the right, producing $b = (4, 1, 2, 3, 1, 0)$.],
      ) <fig:sparse-matrix-compression>
    ]
  ]
}

#{
  let x = load-model-example("MinimumMatrixCover")
  let A = x.instance.matrix
  let n = A.len()
  let cfg = x.optimal_config
  let signs = cfg.map(v => if v == 0 { $-1$ } else { $+1$ })
  [
    #problem-def("MinimumMatrixCover")[
      Given an $n times n$ nonnegative integer matrix $A$, find a function $f: \{1, dots, n\} -> \{-1, +1\}$ minimizing $sum_(i,j) a_(i j) dot f(i) dot f(j)$.
    ][
      Minimum Matrix Cover asks for a sign assignment to rows (equivalently columns) of a square matrix that minimizes the resulting quadratic form. Each binary variable $x_i in \{0, 1\}$ encodes a sign $f(i) = 2 x_i - 1$. Since $f(i)^2 = 1$, diagonal entries contribute a constant $sum_i a_(i i)$; the optimization depends only on off-diagonal structure. The brute-force complexity is $O(2^n)$ where $n$ is the matrix dimension.#footnote[No algorithm improving on brute-force enumeration of all $2^n$ sign assignments is known for the general case.]

      *Example.* Let $A$ be the #(n)$times$#(n) symmetric matrix with zero diagonal shown below. The optimal config $(#cfg.map(str).join(", "))$ assigns signs $(#signs.join(", "))$, yielding value $= #x.optimal_value$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o mmc.json",
        "pred solve mmc.json",
        "pred evaluate mmc.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumMatrixDomination")
  let M = x.instance.matrix
  let m = M.len()
  let n = if m > 0 { M.at(0).len() } else { 0 }
  let ones = x.instance.ones
  let cfg = x.optimal_config
  let selected-ones = ones.enumerate().filter(((k, _)) => cfg.at(k) == 1).map(((_, pos)) => pos)
  [
    #problem-def("MinimumMatrixDomination")[
      Given an $m times n$ binary matrix $M$, find a minimum-cardinality subset $C$ of 1-entries such that every 1-entry not in $C$ shares a row or column with some entry in $C$.
    ][
      Minimum Matrix Domination is a matrix analogue of the dominating set problem. Each binary variable corresponds to a 1-entry in row-major order; the evaluator checks that every unselected 1-entry shares a row or column with at least one selected entry. The brute-force complexity is $O(2^k)$ where $k$ is the number of 1-entries.

      *Example.* Let $M$ be the #(m)$times$#(n) adjacency matrix of $P_6$ (the path on 6 vertices), which has #(ones.len()) non-zero entries. The optimal config $(#cfg.map(str).join(", "))$ selects entries at positions #selected-ones.map(((r, c)) => [(#r, #c)]).join(", "), yielding value $= #x.optimal_value$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o mmd.json",
        "pred solve mmd.json",
        "pred evaluate mmd.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("MinimumWeightDecoding")
  let H = x.instance.matrix
  let s = x.instance.target
  let n = H.len()
  let m = if n > 0 { H.at(0).len() } else { 0 }
  let cfg = x.optimal_config
  let wt = x.optimal_value
  [
    #problem-def("MinimumWeightDecoding")[
      Given an $n times m$ binary parity-check matrix $H$ and a binary syndrome vector $s in {0,1}^n$, find a binary vector $x in {0,1}^m$ minimizing the Hamming weight $|x| = sum_(j=0)^(m-1) x_j$ subject to $H x equiv s (mod space 2)$.
    ][
      Minimum Weight Decoding is a fundamental problem in coding theory. Given a linear code with parity-check matrix $H$, the task is to find the minimum-weight error pattern consistent with a received syndrome. The problem is equivalent to finding the closest codeword to a received word and is central to the hardness of decoding random linear codes.

      The best known algorithms for general instances use information set decoding techniques, achieving $O(2^(0.0494 n))$ where $n$ is the block length.

      *Example.* Let $H$ be the #(n)$times$#(m) binary matrix and $s = (#s.map(v => if v { "1" } else { "0" }).join(", "))$. The optimal config $(#cfg.map(str).join(", "))$ has Hamming weight $#wt$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o mwd.json",
        "pred solve mwd.json",
        "pred evaluate mwd.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-example("MinimumWeightDecoding", "ILP")
  let src = x.source
  let tgt = x.target
  [
    #reduction-rule("MinimumWeightDecoding", "ILP",
      example: true,
      example-caption: [Minimum Weight Decoding to ILP ($#src.instance.matrix.len()$ rows, $#src.instance.matrix.at(0).len()$ columns)],
    )[
      The GF(2) constraint $H x equiv s (mod space 2)$ is linearized by introducing integer slack variables: for each row $i$, $sum_j H_(i j) x_j - 2 k_i = s_i$ where $k_i >= 0$ is an integer. Binary bounds $x_j <= 1$ are added, and the objective minimizes $sum x_j$.
    ][
      _Construction._ Given $H in {0,1}^(n times m)$ and $s in {0,1}^n$, create an ILP with $m + n$ variables: $x_0, dots, x_(m-1)$ (binary) and $k_0, dots, k_(n-1)$ (non-negative integer). Add $n$ equality constraints $sum_j H_(i j) x_j - 2 k_i = s_i$ and $m$ binary bounds $x_j <= 1$. The objective is $min sum_(j=0)^(m-1) x_j$.

      _Correctness._ ($arrow.r.double$) If $x^*$ is feasible for the source, then $H x^* equiv s (mod space 2)$, so $sum_j H_(i j) x^*_j = s_i + 2 k_i$ for some $k_i >= 0$. Setting these $k_i$ values gives a feasible ILP solution with the same objective. ($arrow.l.double$) If $(x^*, k^*)$ is feasible for the ILP, then $sum_j H_(i j) x^*_j = s_i + 2 k^*_i$ implies $sum_j H_(i j) x^*_j equiv s_i (mod space 2)$ for all $i$, and $x^*_j in {0, 1}$ by the binary bounds.

      _Solution extraction._ Take the first $m$ variables as the source configuration.
    ]
  ]
}

#{
  let x = load-model-example("MinimumWeightSolutionToLinearEquations")
  let A = x.instance.matrix
  let b = x.instance.rhs
  let n = A.len()
  let m = if n > 0 { A.at(0).len() } else { 0 }
  let cfg = x.optimal_config
  let selected-cols = cfg.enumerate().filter(((_, v)) => v == 1).map(((j, _)) => j)
  [
    #problem-def("MinimumWeightSolutionToLinearEquations")[
      Given an $n times m$ integer matrix $A$ and an integer vector $b in ZZ^n$, find a rational vector $y in QQ^m$ satisfying $A y = b$ that minimizes $||y||_0$ (the number of non-zero entries of $y$).
    ][
      Minimum Weight Solution to Linear Equations is a sparsity-seeking variant of solving linear systems. Each binary variable $x_j$ indicates whether the $j$-th component of $y$ may be non-zero; the evaluator forms the restricted submatrix $A'$ from the selected columns and checks whether $b$ lies in its column space via integer Gaussian elimination (using i128 arithmetic for exact rational consistency). If the restricted system $A' y' = b$ is consistent, the value is the number of selected columns; otherwise the configuration is infeasible.

      *Example.* Let $A$ be the #(n)$times$#(m) matrix $mat(#A.at(0).map(str).join(", "); #A.at(1).map(str).join(", "))$ with $b = (#b.map(str).join(", "))$. The optimal config $(#cfg.map(str).join(", "))$ selects columns #selected-cols.map(str).join(", "), yielding value $= #x.optimal_value$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o mwsle.json",
        "pred solve mwsle.json",
        "pred evaluate mwsle.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("IntegerExpressionMembership")
  [
    #problem-def("IntegerExpressionMembership")[
      Given a recursive integer expression $e$ over union ($union$) and Minkowski sum ($+$) operations on singleton positive integers, and a target $K in NN^+$, determine whether $K in "eval"(e)$, where $"eval"("Atom"(n)) = \{n\}$, $"eval"(F union G) = "eval"(F) union "eval"(G)$, and $"eval"(F + G) = \{m + n : m in "eval"(F), n in "eval"(G)\}$.
    ][
      Integer Expression Membership asks whether a specific integer is reachable by selecting one branch at each union node in a recursive expression tree. Each configuration assigns a binary choice (left or right) at every union node in depth-first order; with all unions resolved, the expression reduces to a chain of sums and atoms that evaluates to a single integer.#footnote[No algorithm improving on brute-force enumeration of all $2^u$ union-branch combinations (where $u$ is the number of union nodes) is known for the general case.]

      *Example.* Consider $e = (1 union 4) + (3 union 6) + (2 union 5)$ with $K = 12$. There are $u = 3$ union nodes, giving $2^3 = 8$ configurations. The reachable set is $\{6, 9, 12, 15\}$. Since $12 in \{6, 9, 12, 15\}$, the answer is YES. One witness: choose right ($4$), right ($6$), left ($2$) at the three union nodes, giving $4 + 6 + 2 = 12 = K$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o iem.json",
        "pred solve iem.json --solver brute-force",
        "pred evaluate iem.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 0.7cm, {
          import draw: *
          // Draw expression tree: Sum( Sum( Union(1,4), Union(3,6) ), Union(2,5) )
          let node-r = 0.45
          let level-gap = 1.5
          let spread = 2.0
          // Root: Sum (level 0)
          circle((0, 0), radius: node-r, name: "root", fill: rgb("#e8f0fe"))
          content("root", text(8pt, $+$))
          // Left child: Sum (level 1)
          circle((-spread, -level-gap), radius: node-r, name: "lsum", fill: rgb("#e8f0fe"))
          content("lsum", text(8pt, $+$))
          line("root.south-west", "lsum.north", mark: (end: "straight"))
          // Right child: Union (level 1)
          circle((spread, -level-gap), radius: node-r, name: "ru", fill: rgb("#fce8e6"))
          content("ru", text(8pt, $union$))
          line("root.south-east", "ru.north", mark: (end: "straight"))
          // Left-Left: Union (level 2)
          circle((-spread - 1.2, -2 * level-gap), radius: node-r, name: "llu", fill: rgb("#fce8e6"))
          content("llu", text(8pt, $union$))
          line("lsum.south-west", "llu.north", mark: (end: "straight"))
          // Left-Right: Union (level 2)
          circle((-spread + 1.2, -2 * level-gap), radius: node-r, name: "lru", fill: rgb("#fce8e6"))
          content("lru", text(8pt, $union$))
          line("lsum.south-east", "lru.north", mark: (end: "straight"))
          // Leaves (level 3)
          let leaf-spread = 0.7
          for (parent, vals, xoff) in (
            ("llu", (1, 4), -spread - 1.2),
            ("lru", (3, 6), -spread + 1.2),
            ("ru", (2, 5), spread),
          ) {
            circle((xoff - leaf-spread, -3 * level-gap), radius: node-r, name: parent + "l", fill: rgb("#e6f4ea"))
            content(parent + "l", text(8pt, str(vals.at(0))))
            line(parent + ".south-west", (parent + "l") + ".north", mark: (end: "straight"))
            circle((xoff + leaf-spread, -3 * level-gap), radius: node-r, name: parent + "r", fill: rgb("#e6f4ea"))
            content(parent + "r", text(8pt, str(vals.at(1))))
            line(parent + ".south-east", (parent + "r") + ".north", mark: (end: "straight"))
          }
        }),
        caption: [Expression tree $e = (1 union 4) + (3 union 6) + (2 union 5)$ with target $K = 12$. Blue nodes are sums ($+$), red nodes are unions ($union$), green leaves are atoms. Choosing right at the first two unions and left at the third yields $4 + 6 + 2 = 12$.],
      ) <fig:integer-expression-membership>
    ]
  ]
}

#{
  let x = load-model-example("FeasibleBasisExtension")
  let A = x.instance.matrix
  let m = A.len()
  let n = A.at(0).len()
  let rhs = x.instance.rhs
  let S = x.instance.required_columns
  let cfg = x.optimal_config
  // Free column indices (those not in S)
  let free-cols = range(n).filter(j => j not in S)
  // Selected free columns from config
  let selected = cfg.enumerate().filter(((i, v)) => v == 1).map(((i, v)) => free-cols.at(i))
  // Full basis: required + selected
  let basis = S + selected
  [
    #problem-def("FeasibleBasisExtension")[
      Given an $m times n$ integer matrix $A$ with $m < n$, a column vector $overline(a) in bb(Z)^m$, and a subset $S$ of column indices with $|S| < m$, determine whether there exists a _feasible basis_ $B$ --- a set of $m$ column indices including $S$ --- such that the $m times m$ submatrix $A_B$ is nonsingular and $A_B^(-1) overline(a) >= 0$ (componentwise).
    ][
      The Feasible Basis Extension problem arises in linear programming theory and the study of simplex method pivoting rules. It was shown NP-complete by Murty @Murty1972 via a reduction from Hamiltonian Circuit, establishing that determining whether a partial basis can be extended to a feasible one is computationally intractable in general. The problem is closely related to the question of whether a given linear program has a feasible basic solution containing specified variables. The best known exact algorithm is brute-force enumeration of all $binom(n - |S|, m - |S|)$ candidate extensions, testing each for nonsingularity and non-negativity of the solution in $O(m^3)$ time.#footnote[No algorithm improving on brute-force enumeration is known for the general Feasible Basis Extension problem.]

      *Example.* Consider the $#m times #n$ matrix $A = mat(#A.map(row => row.map(v => str(v)).join(", ")).join("; "))$ with $overline(a) = (#rhs.map(str).join(", "))^top$ and required columns $S = \{#S.map(str).join(", ")\}$. We need $#(m - S.len())$ additional column from the free set $\{#free-cols.map(str).join(", ")\}$. Selecting column #selected.at(0) gives basis $B = \{#basis.map(str).join(", ")\}$, which yields $A_B^(-1) overline(a) = (4, 5, 3)^top >= 0$. Column 4 makes $A_B$ singular, and column 5 produces a negative component.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o feasible-basis-extension.json",
        "pred solve feasible-basis-extension.json --solver brute-force",
        "pred evaluate feasible-basis-extension.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 0.7cm, {
          import draw: *
          let cell-size = 0.9
          let gap = 0.15
          // Draw the matrix
          for i in range(m) {
            for j in range(n) {
              let val = A.at(i).at(j)
              let is-basis = j in basis
              let is-required = j in S
              let f = if is-required {
                graph-colors.at(1).transparentize(50%)
              } else if is-basis {
                graph-colors.at(0).transparentize(30%)
              } else {
                white
              }
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
            let label-color = if j in S { graph-colors.at(1) } else if j in basis { graph-colors.at(0) } else { black }
            content(
              (j * cell-size + (cell-size - gap) / 2, 0.4),
              text(7pt, fill: label-color)[$c_#j$],
            )
          }
          // Row labels
          for i in range(m) {
            content(
              (-0.5, -i * cell-size - (cell-size - gap) / 2),
              text(7pt)[$r_#(i + 1)$],
            )
          }
          // RHS vector
          let rhs-x = n * cell-size + 0.6
          content((rhs-x + (cell-size - gap) / 2, 0.4), text(7pt, weight: "bold")[$overline(a)$])
          for i in range(m) {
            rect(
              (rhs-x, -i * cell-size),
              (rhs-x + cell-size - gap, -i * cell-size - cell-size + gap),
              fill: luma(240),
              stroke: 0.3pt + luma(180),
            )
            content(
              (rhs-x + (cell-size - gap) / 2, -i * cell-size - (cell-size - gap) / 2),
              text(8pt, str(rhs.at(i))),
            )
          }
        }),
        caption: [Feasible Basis Extension instance ($#m times #n$). Orange columns are required ($S = \{#S.map(str).join(", ")\}$), blue column is the selected extension. Together they form a nonsingular basis with non-negative solution.],
      ) <fig:feasible-basis-extension>
    ]
  ]
}

#{
  let x = load-model-example("MaximumLikelihoodRanking")
  let A = x.instance.matrix
  let n = A.len()
  let config = x.optimal_config
  let opt-val = metric-value(x.optimal_value)
  // Compute disagreement pairs for the optimal solution
  let disagreement-pairs = ()
  for a in range(n) {
    for b in range(n) {
      if a != b and config.at(a) > config.at(b) {
        disagreement-pairs.push((a, b, A.at(a).at(b)))
      }
    }
  }
  let total-cost = disagreement-pairs.map(p => p.at(2)).sum()
  [
    #problem-def("MaximumLikelihoodRanking")[
      Given an $n times n$ comparison matrix $A$ with $a_(i i) = 0$ for all $i$, find a permutation $pi$ of ${0, dots, n - 1}$ minimizing the _disagreement cost_ $sum_(i > j) a_(pi(i), pi(j))$, where $pi$ maps rank positions to items.
    ][
      The Maximum Likelihood Ranking problem arises in voting theory and tournament aggregation: given pairwise comparison counts between $n$ alternatives, find a linear ordering that best explains the observed data @garey1979. The problem is equivalent to finding a minimum-weight linear extension of a tournament. When $a_(i j) + a_(j i) = c$ for all $i != j$, the problem is also known as the _Kemeny ranking_ or _minimum feedback arc set in tournaments_. The best known exact algorithm uses dynamic programming over subsets (Held--Karp style), running in $O(n^2 dot 2^n)$ time.#footnote[No algorithm with a significantly better exponential base than $2^n$ is known for the general Maximum Likelihood Ranking problem.]

      *Example.* Consider $n = #n$ items with comparison matrix $A$, where rows and columns index items $0, dots, #(n - 1)$. Ranking all items by identity ($pi(i) = i$) yields disagreement cost $#disagreement-pairs.map(p => str(p.at(2))).join(" + ") = #total-cost$, which is optimal.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o mlr.json",
        "pred solve mlr.json --solver brute-force",
        "pred evaluate mlr.json --config " + x.optimal_config.map(str).join(","),
      )

      #figure(
        canvas(length: 0.7cm, {
          import draw: *
          let cell = 0.9
          let gap = 0.12
          for i in range(n) {
            for j in range(n) {
              let val = A.at(i).at(j)
              let is-disagree = i != j and config.at(i) > config.at(j)
              let f = if i == j { luma(230) } else if is-disagree { graph-colors.at(0).transparentize(40%) } else { white }
              rect(
                (j * cell, -i * cell),
                (j * cell + cell - gap, -i * cell - cell + gap),
                fill: f,
                stroke: 0.3pt + luma(180),
              )
              content(
                (j * cell + (cell - gap) / 2, -i * cell - (cell - gap) / 2),
                text(8pt, str(val)),
              )
            }
          }
          // Column labels
          for j in range(n) {
            content(
              (j * cell + (cell - gap) / 2, 0.35),
              text(7pt)[$#j$],
            )
          }
          // Row labels
          for i in range(n) {
            content(
              (-0.35, -i * cell - (cell - gap) / 2),
              text(7pt)[$#i$],
            )
          }
        }),
        caption: [Comparison matrix $A$ ($n = #n$). Highlighted cells are disagreement entries under the identity ranking.],
      )
    ]
  ]
}

#{
  let x = load-model-example("OptimumCommunicationSpanningTree")
  let n = x.instance.num_vertices
  let W = x.instance.edge_weights
  let R = x.instance.requirements
  let config = x.optimal_config
  let opt-val = metric-value(x.optimal_value)
  // Build edge list and selected edges
  let edges = ()
  let edge-idx = 0
  for i in range(n) {
    for j in range(i + 1, n) {
      edges.push((i, j, config.at(edge-idx)))
      edge-idx += 1
    }
  }
  let tree-edges = edges.filter(e => e.at(2) == 1)
  [
    #problem-def("OptimumCommunicationSpanningTree")[
      Given a complete graph $K_n$ with edge weights $w(e) >= 0$ and communication requirements $r(u, v) >= 0$ for each vertex pair, find a spanning tree $T$ minimizing the total communication cost $sum_(u < v) r(u, v) dot W_T (u, v)$, where $W_T (u, v)$ is the sum of edge weights on the unique path from $u$ to $v$ in $T$.
    ][
      The Optimum Communication Spanning Tree problem (ND7 in @garey1979) models communication network design where edge weights represent link costs and requirements represent traffic demands between vertex pairs. NP-hard even when all requirements are equal (reducing to the Minimum Routing Cost Spanning Tree). Polynomial when all edge weights are equal, solved by the Gomory-Hu tree. The best known exact approach enumerates all $n^(n-2)$ labeled spanning trees.

      *Example.* Consider $K_#n$ with edge weight matrix $W$ and requirement matrix $R$. The optimal spanning tree uses edges ${#tree-edges.map(e => $(#e.at(0), #e.at(1))$).join(", ")}$ with total communication cost $#opt-val$.

      #pred-commands(
        "pred create --example " + problem-spec(x) + " -o ocst.json",
        "pred solve ocst.json --solver brute-force",
        "pred evaluate ocst.json --config " + x.optimal_config.map(str).join(","),
      )
    ]
  ]
}

#{
  let x = load-model-example("SquareTiling")
  let nc = x.instance.num_colors
  let tiles = x.instance.tiles
  let n = x.instance.grid_size
  let config = x.optimal_config
  [
    #problem-def("SquareTiling")[
      Given a set $C$ of colors, a collection $T subset.eq C^4$ of tile types (where $angle.l a, b, c, d angle.r$ denotes a tile whose top, right, bottom, and left sides are colored $a, b, c, d$ respectively), and a positive integer $N$, determine whether there exists an assignment of a tile $f(i,j) in T$ to each grid cell $(i,j)$, $0 <= i,j < N$, such that (1) if $f(i,j) = angle.l a,b,c,d angle.r$ and $f(i+1,j) = angle.l a',b',c',d' angle.r$ then $c = a'$ (bottom of upper tile matches top of lower tile), and (2) if $f(i,j) = angle.l a,b,c,d angle.r$ and $f(i,j+1) = angle.l a',b',c',d' angle.r$ then $b = d'$ (right of left tile matches left of right tile). Tiles may be reused but not rotated or reflected.
    ][
      Square Tiling (also known as Bounded Wang Tiling) is problem GP13 in Garey and Johnson @garey1979. It was shown NP-complete via transformation from Directed Hamiltonian Path. The infinite variant (tiling the entire plane) is famously undecidable (Berger, 1966). The best known exact approach enumerates all $|T|^(N^2)$ assignments.

      *Example.* Consider $|C| = #nc$ colors, $|T| = #tiles.len()$ tiles, and grid size $N = #n$. The tiles are #tiles.enumerate().map(((i, t)) => [$t_#i = angle.l #t.at(0), #t.at(1), #t.at(2), #t.at(3) angle.r$]).join(", "). The witness assignment $(#config.map(str).join(", "))$ places $t_#config.at(0), t_#config.at(1)$ in row 0 and $t_#config.at(2), t_#config.at(3)$ in row 1, satisfying all edge-color constraints.

      #pred-commands(
        "pred create --example SquareTiling -o square_tiling.json",
        "pred solve square_tiling.json",
        "pred evaluate square_tiling.json --config " + config.map(str).join(","),
      )
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
    #pred-commands(
      "pred create --example MVC -o mvc.json",
      "pred reduce mvc.json --to " + target-spec(mvc_mis) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate mvc.json --config " + mvc_mis_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example MVC -o mvc.json",
      "pred reduce mvc.json --to " + target-spec(mvc_fvs) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate mvc.json --config " + mvc_fvs_sol.source_config.map(str).join(","),
    )
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


#let mis_clique = load-example("MaximumIndependentSet", "MaximumClique")
#let mis_clique_sol = mis_clique.solutions.at(0)
#reduction-rule("MaximumIndependentSet", "MaximumClique",
  example: true,
  example-caption: [Path graph $P_5$: IS $arrow.r$ Clique via complement],
  extra: [
    #pred-commands(
      "pred create --example MIS -o mis.json",
      "pred reduce mis.json --to " + target-spec(mis_clique) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate mis.json --config " + mis_clique_sol.source_config.map(str).join(","),
    )
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

#reduction-rule("MinimumVertexCover", "EnsembleComputation")[
  This $O(|V| + |E|)$ reduction @garey1979 encodes the unit-weight vertex-cover problem as an ensemble-computation minimization over disjoint unions. A fresh element $a_0$ is introduced, and each edge becomes a 3-element target subset. The minimum sequence length equals $K^* + |E|$, where $K^*$ is the minimum vertex cover size.
][
  _Construction._ Given a unit-weight VC instance $G = (V, E)$, let $a_0$ be a fresh element not in $V$. Set the universe $A = V union {a_0}$ with $|A| = |V| + 1$. For each edge ${u, v} in E$, add the subset ${a_0, u, v}$ to the collection $C$. Set the search-space bound $J = |V| + |E|$.

  _Correctness._ ($arrow.r.double$) If $C'$ is a vertex cover of size $K$, label its elements $v_1, dots, v_K$ and the edges $e_1, dots, e_m$. Since $C'$ covers every edge, each $e_j = {u_j, v_(r[j])}$ where $v_(r[j]) in C'$. The sequence of $K + m$ operations $z_i = {a_0} union {v_i}$ for $i = 1, dots, K$ followed by $z_(K+j) = {u_j} union z_(r[j])$ for $j = 1, dots, m$ produces every target subset in exactly $K + |E|$ steps. ($arrow.l.double$) An exchange argument (Garey & Johnson, PO9) shows that any minimum-length sequence can be normalized to use only ${a_0} union {u}$ and ${v} union z_k$ forms. Each edge contributes exactly one operation of the second form, so the number of first-form operations equals the sequence length minus $|E|$. Since the first-form vertices must cover all edges, the minimum sequence length is $K^* + |E|$.

  _Solution extraction._ From an optimal witness, collect all vertices appearing as singleton operands (indices $< |V|$). In a minimum-length normalized sequence, exactly the $K^*$ cover vertices appear as ${a_0}$-paired singletons.
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
    #pred-commands(
      "pred create --example SpinGlass -o spinglass.json",
      "pred reduce spinglass.json --to " + target-spec(sg_qubo) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate spinglass.json --config " + sg_qubo_sol.source_config.map(str).join(","),
    )
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
        #pred-commands(
          "pred create --example CVP -o cvp.json",
          "pred reduce cvp.json --to " + target-spec(cvp_qubo) + " -o bundle.json",
          "pred solve bundle.json",
          "pred evaluate cvp.json --config " + cvp_qubo_sol.source_config.map(str).join(","),
        )
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
    #pred-commands(
      "pred create --example " + problem-spec(kc_qubo.source) + " -o kcoloring.json",
      "pred reduce kcoloring.json --to " + target-spec(kc_qubo) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate kcoloring.json --config " + kc_qubo_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example " + problem-spec(ksat_ss.source) + " -o ksat.json",
      "pred reduce ksat.json --to " + target-spec(ksat_ss) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate ksat.json --config " + ksat_ss_sol.source_config.map(str).join(","),
    )
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
        #pred-commands(
          "pred create --example SubsetSum -o subsetsum.json",
          "pred reduce subsetsum.json --to " + target-spec(ss-cvp) + " -o bundle.json",
          "pred solve bundle.json",
          "pred evaluate subsetsum.json --config " + ss-cvp-sol.source_config.map(str).join(","),
        )
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

#{
  let ss-ca = load-example("SubsetSum", "CapacityAssignment")
  let ss-ca-sol = ss-ca.solutions.at(0)
  let ss-ca-sizes = ss-ca.source.instance.sizes.map(int)
  let ss-ca-target = int(ss-ca.source.instance.target)
  let ss-ca-n = ss-ca-sizes.len()
  let ss-ca-S = ss-ca-sizes.fold(0, (a, b) => a + b)
  let ss-ca-J = ss-ca-S - ss-ca-target
  let ss-ca-selected = ss-ca-sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
  let ss-ca-selected-sizes = ss-ca-selected.map(i => ss-ca-sizes.at(i))
  let ss-ca-selected-sum = ss-ca-selected-sizes.fold(0, (a, b) => a + b)
  let ss-ca-not-selected = ss-ca-sol.source_config.enumerate().filter(((i, x)) => x == 0).map(((i, x)) => i)
  let ss-ca-delay-sum = ss-ca-not-selected.map(i => ss-ca-sizes.at(i)).fold(0, (a, b) => a + b)
  [
    #reduction-rule("SubsetSum", "CapacityAssignment",
      example: true,
      example-caption: [#ss-ca-n elements, target sum $B = #ss-ca-target$],
      extra: [
        #pred-commands(
          "pred create --example SubsetSum -o subsetsum.json",
          "pred reduce subsetsum.json --to " + target-spec(ss-ca) + " -o bundle.json",
          "pred solve bundle.json",
          "pred evaluate subsetsum.json --config " + ss-ca-sol.source_config.map(str).join(","),
        )
        *Step 1 -- Source instance.* The canonical Subset Sum instance has sizes $(#ss-ca-sizes.map(str).join(", "))$ and target $B = #ss-ca-target$. The total sum is $S = #ss-ca-S$.

        *Step 2 -- Build the Capacity Assignment instance.* The reduction creates #ss-ca-n communication links with two capacities ${1, 2}$. For each link $c_i$ with element value $a_i$: cost row $(0, a_i)$ and delay row $(a_i, 0)$. The delay budget is $J = S - B = #ss-ca-S - #ss-ca-target = #ss-ca-J$.

        *Step 3 -- Verify the canonical witness.* The fixture stores source config $(#ss-ca-sol.source_config.map(str).join(", "))$, selecting elements at indices $#ss-ca-selected.map(str).join(", ")$ with values $#ss-ca-selected-sizes.map(str).join(" + ") = #ss-ca-selected-sum = B$. In the target, these links get high capacity (index 1) with total cost $#ss-ca-selected-sum$ and the remaining links get low capacity (index 0) with total delay $#ss-ca-delay-sum <= #ss-ca-J = J$.

        *Witness semantics.* The example DB stores one canonical witness. Other subsets summing to $B$ would also yield valid witnesses.
      ],
    )[
      This $O(n)$ reduction from Subset Sum to Capacity Assignment follows the original NP-completeness proof of Van Sickle and Chandy @vansicklechandy1977 (GJ SR7 @garey1979). Each element becomes a communication link with two capacity levels; the cost/delay duality encodes complementary subset selection.
    ][
      _Construction._ Given sizes $a_1, dots, a_n in ZZ^+$ and target $B$, let $S = sum_(i=1)^n a_i$. Create $n$ communication links with capacity set $M = {1, 2}$. For each link $c_i$:
      - Cost: $g(c_i, 1) = 0$, $g(c_i, 2) = a_i$ (non-decreasing since $0 <= a_i$).
      - Delay: $d(c_i, 1) = a_i$, $d(c_i, 2) = 0$ (non-increasing since $a_i >= 0$).
      Set the delay budget $J = S - B$.

      _Correctness._ For any assignment $sigma$, the total cost is $sum_(i: sigma(c_i)=2) a_i$ and the total delay is $sum_(i: sigma(c_i)=1) a_i$. Since every element contributes to exactly one of these sums, cost $+$ delay $= S$.

      ($arrow.r.double$) If $A' subset.eq A$ sums to $B$, assign $sigma(c_i) = 2$ for $a_i in A'$ and $sigma(c_i) = 1$ otherwise. Total cost $= B$, total delay $= S - B = J$.

      ($arrow.l.double$) The delay constraint forces delay $<= S - B$, so cost $>= S - (S - B) = B$. If the optimal cost equals $B$, the high-capacity links form a subset summing to exactly $B$. If no such subset exists, the minimum cost is strictly greater than $B$.

      _Solution extraction._ Return the target configuration unchanged: capacity index 1 (high) for link $c_i$ means element $a_i$ is selected.
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

#let part_cpi = load-example("Partition", "CosineProductIntegration")
#let part_cpi_sol = part_cpi.solutions.at(0)
#let part_cpi_sizes = part_cpi.source.instance.sizes
#let part_cpi_n = part_cpi_sizes.len()
#let part_cpi_coeffs = part_cpi.target.instance.coefficients
#reduction-rule("Partition", "CosineProductIntegration",
  example: true,
  example-caption: [#part_cpi_n elements],
)[
  This $O(n)$ identity reduction casts each positive integer size $s_i$ to the corresponding integer coefficient $a_i = s_i$. A balanced partition (two subsets of equal sum) exists if and only if a balanced sign assignment ($sum epsilon_i a_i = 0$) exists, because assigning element $i$ to subset $A'$ corresponds to $epsilon_i = -1$ and to $A without A'$ corresponds to $epsilon_i = +1$. Reference: Plaisted (1976) @plaisted1976.
][
  _Construction._ Given Partition sizes $s_0, dots, s_(n-1) in ZZ^+$, set the CosineProductIntegration coefficients to $a_i = s_i$ for each $i in {0, dots, n-1}$.

  _Correctness._ ($arrow.r.double$) If a balanced partition exists with subset $A'$ having $sum_(a in A') s(a) = S slash 2$, then the sign assignment $epsilon_i = -1$ for $i in A'$ and $epsilon_i = +1$ otherwise gives $sum epsilon_i a_i = S - 2 dot S slash 2 = 0$. ($arrow.l.double$) If a balanced sign assignment exists with $sum epsilon_i a_i = 0$, the elements with $epsilon_i = -1$ form a subset summing to $S slash 2$, which is a valid partition.

  _Solution extraction._ Return the same binary vector: $x_i = 1$ (element in second subset) corresponds to $epsilon_i = -1$ (negative sign).
]

#let part_ks = load-example("Partition", "Knapsack")
#let part_ks_sol = part_ks.solutions.at(0)
#let part_ks_sizes = part_ks.source.instance.sizes
#let part_ks_n = part_ks_sizes.len()
#let part_ks_total = part_ks_sizes.fold(0, (a, b) => a + b)
#let part_ks_capacity = part_ks.target.instance.capacity
#let part_ks_selected = part_ks_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#let part_ks_selected_sizes = part_ks_selected.map(i => part_ks_sizes.at(i))
#let part_ks_selected_sum = part_ks_selected_sizes.fold(0, (a, b) => a + b)
#reduction-rule("Partition", "Knapsack",
  example: true,
  example-caption: [#part_ks_n elements, total sum $S = #part_ks_total$],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(part_ks.source) + " -o partition.json",
      "pred reduce partition.json --to " + target-spec(part_ks) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate partition.json --config " + part_ks_sol.source_config.map(str).join(","),
    )

    *Step 1 -- Source instance.* The canonical Partition instance has sizes $(#part_ks_sizes.map(str).join(", "))$ with total sum $S = #part_ks_total$, so a balanced witness must hit exactly $S / 2 = #part_ks_capacity$.

    *Step 2 -- Build the knapsack instance.* The reduction copies each size into both the weight and the value list, producing weights $(#part_ks.target.instance.weights.map(str).join(", "))$, values $(#part_ks.target.instance.values.map(str).join(", "))$, and capacity $C = #part_ks_capacity$. No auxiliary variables are introduced, so the target has the same $#part_ks_n$ binary coordinates as the source.

    *Step 3 -- Verify the canonical witness.* The serialized witness uses the same binary vector on both sides, $bold(x) = (#part_ks_sol.source_config.map(str).join(", "))$. It selects elements at indices $\{#part_ks_selected.map(str).join(", ")\}$ with sizes $(#part_ks_selected_sizes.map(str).join(", "))$, so the chosen subset has total weight and value $#part_ks_selected_sum = #part_ks_capacity$. Hence the knapsack solution saturates the capacity and certifies a balanced partition.

    *Witness semantics.* The example DB stores one canonical balanced subset. This instance has multiple balanced partitions because several different subsets sum to $#part_ks_capacity$, but one witness is enough to demonstrate the reduction.
  ],
)[
  This $O(n)$ reduction#footnote[The linear-time bound follows from a single pass that copies the source sizes into item weights and values.] @garey1979[MP9] constructs a 0-1 Knapsack instance by copying each Partition size into both the item weight and item value and setting the capacity to half the total size sum. For $n$ source elements it produces $n$ knapsack items.
][
  _Construction._ Given positive sizes $s_0, dots, s_(n-1)$ with total sum $S = sum_(i=0)^(n-1) s_i$, create one knapsack item per element and set
  $ w_i = s_i, quad v_i = s_i $
  for every $i in {0, dots, n-1}$. Set the knapsack capacity to
  $ C = floor(S / 2). $
  Every feasible knapsack solution is therefore a subset of the original elements, and because $w_i = v_i$, its objective value equals the same subset sum.

  _Correctness._ ($arrow.r.double$) If the Partition instance is satisfiable, some subset $A'$ has sum $S / 2$. In particular $S$ is even, so $C = S / 2$, and selecting exactly the corresponding knapsack items is feasible with value $S / 2$. No feasible knapsack solution can have value larger than $C$, because value equals weight for every item and total weight is bounded by $C$. Thus the knapsack optimum is exactly $S / 2$. ($arrow.l.double$) If the knapsack optimum is $S / 2$, then the optimum is an integer and hence $S$ must be even. The selected items have total value $S / 2$, so they also have total weight $S / 2$ because $w_i = v_i$ itemwise. Those items therefore form a subset of the original multiset whose complement has the same sum, giving a valid balanced partition.

  _Solution extraction._ Return the same binary selection vector on the original elements: item $i$ is selected in the knapsack witness if and only if element $i$ belongs to the extracted partition subset.
]

#let part_ss = load-example("Partition", "SubsetSum")
#let part_ss_sol = part_ss.solutions.at(0)
#let part_ss_sizes = part_ss.source.instance.sizes
#let part_ss_n = part_ss_sizes.len()
#let part_ss_total = part_ss_sizes.fold(0, (a, b) => a + b)
#let part_ss_target = part_ss_total / 2
#let part_ss_selected = part_ss_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#let part_ss_selected_sizes = part_ss_selected.map(i => part_ss_sizes.at(i))
#let part_ss_selected_sum = part_ss_selected_sizes.fold(0, (a, b) => a + b)
#reduction-rule("Partition", "SubsetSum",
  example: true,
  example-caption: [#part_ss_n elements, total sum $S = #part_ss_total$],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(part_ss.source) + " -o partition.json",
      "pred reduce partition.json --to " + target-spec(part_ss) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate partition.json --config " + part_ss_sol.source_config.map(str).join(","),
    )

    *Step 1 -- Source instance.* The canonical Partition instance has sizes $(#part_ss_sizes.map(str).join(", "))$ with total sum $S = #part_ss_total$, so a balanced witness must hit exactly $S / 2 = #part_ss_target$.

    *Step 2 -- Build the Subset Sum instance.* The reduction copies the sizes directly: $(#part_ss_sizes.map(str).join(", "))$, and sets the target $B = S / 2 = #part_ss_target$. The number of binary variables is unchanged ($n = #part_ss_n$).

    *Step 3 -- Verify the canonical witness.* The serialized witness uses the same binary vector on both sides, $bold(x) = (#part_ss_sol.source_config.map(str).join(", "))$. It selects elements at indices $\{#part_ss_selected.map(str).join(", ")\}$ with sizes $(#part_ss_selected_sizes.map(str).join(", "))$, so the chosen subset sums to $#part_ss_selected_sum = #part_ss_target = B$ #sym.checkmark.

    *Witness semantics.* The example DB stores one canonical balanced subset. Multiple subsets may sum to $B$, but one witness suffices to demonstrate the reduction.
  ],
)[
  This $O(n)$ reduction @garey1979[SP13] @karp1972 embeds a Partition instance into Subset Sum by copying the element sizes and setting the target to half the total sum. For $n$ source elements it produces $n$ Subset Sum items.
][
  _Construction._ Given positive sizes $s_0, dots, s_(n-1)$ with total sum $S = sum_(i=0)^(n-1) s_i$, construct a Subset Sum instance with the same sizes and target
  $ B = S / 2. $
  If $S$ is odd, return a trivially infeasible Subset Sum instance (empty sizes, target $= 1$).

  _Correctness._ ($arrow.r.double$) If the Partition instance is satisfiable, some subset $A'$ has sum $S / 2 = B$, so the Subset Sum instance is satisfiable. ($arrow.l.double$) If the Subset Sum instance is satisfiable, some subset sums to $B = S / 2$, so its complement sums to $S - S / 2 = S / 2$, giving a balanced partition. When $S$ is odd, $S / 2$ is not an integer and no subset of positive integers can sum to it; the trivially infeasible target instance correctly reflects this.

  _Solution extraction._ Return the same binary selection vector: element $i$ is in the partition subset if and only if it is selected in the Subset Sum witness.
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
    #pred-commands(
      "pred create --example Knapsack -o knapsack.json",
      "pred reduce knapsack.json --to " + target-spec(ks_qubo) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate knapsack.json --config " + ks_qubo_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example MinimumMultiwayCut -o minimummultiwaycut.json",
      "pred reduce minimummultiwaycut.json --to " + target-spec(mwc_qubo) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate minimummultiwaycut.json --config " + mwc_qubo_sol.source_config.map(str).join(","),
    )
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


#let qubo_ilp = load-example("QUBO", "ILP")
#let qubo_ilp_sol = qubo_ilp.solutions.at(0)
#reduction-rule("QUBO", "ILP",
  example: true,
  example-caption: [4-variable QUBO with 3 quadratic terms],
  extra: [
    #pred-commands(
      "pred create --example QUBO -o qubo.json",
      "pred reduce qubo.json --to " + target-spec(qubo_ilp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate qubo.json --config " + qubo_ilp_sol.source_config.map(str).join(","),
    )
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

  The ILP is:
  $
    min quad & sum_i Q_(i i) x_i + sum_(i < j) Q_(i j) y_(i j) \
    "subject to" quad & y_(i j) <= x_i quad forall i < j, Q_(i j) != 0 \
    & y_(i j) <= x_j quad forall i < j, Q_(i j) != 0 \
    & y_(i j) >= x_i + x_j - 1 quad forall i < j, Q_(i j) != 0 \
    & x_i, y_(i j) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) For binary $x_i, x_j$, the three McCormick inequalities are tight: $y_(i j) = x_i x_j$ is the unique feasible value. Hence the ILP objective equals $bold(x)^top Q bold(x)$, and any ILP minimizer is a QUBO minimizer. ($arrow.l.double$) Given a QUBO minimizer $bold(x)^*$, setting $y_(i j) = x_i^* x_j^*$ satisfies all constraints and achieves the same objective value.

  _Solution extraction._ Return the first $n$ variables (discard auxiliary $y_(i j)$).
]

#let cs_ilp = load-example("CircuitSAT", "ILP")
#let cs_ilp_sol = cs_ilp.solutions.at(0)
#reduction-rule("CircuitSAT", "ILP",
  example: true,
  example-caption: [1-bit full adder to ILP],
  extra: [
    #pred-commands(
      "pred create --example CircuitSAT -o circuitsat.json",
      "pred reduce circuitsat.json --to " + target-spec(cs_ilp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate circuitsat.json --config " + cs_ilp_sol.source_config.map(str).join(","),
    )
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

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & c + a = 1 quad "for each NOT gate" \
    & c <= a_i quad forall i quad "for each AND gate" \
    & c >= sum_i a_i - (k - 1) quad "for each AND gate" \
    & c >= a_i quad forall i quad "for each OR gate" \
    & c <= sum_i a_i quad "for each OR gate" \
    & c <= a + b quad "for each XOR gate" \
    & c >= a - b quad "for each XOR gate" \
    & c >= b - a quad "for each XOR gate" \
    & c <= 2 - a - b quad "for each XOR gate" \
    & "all gate and input variables are binary"
  $.

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
    #pred-commands(
      "pred create --example SAT -o sat.json",
      "pred reduce sat.json --to " + target-spec(sat_mis) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate sat.json --config " + sat_mis_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example SAT -o sat.json",
      "pred reduce sat.json --to " + target-spec(sat_kc) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate sat.json --config " + sat_kc_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example SAT -o sat.json",
      "pred reduce sat.json --to " + target-spec(sat_ds) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate sat.json --config " + sat_ds_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example SAT -o sat.json",
      "pred reduce sat.json --to " + target-spec(sat_ksat) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate sat.json --config " + sat_ksat_sol.source_config.map(str).join(","),
    )
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

#let sat_cs = load-example("Satisfiability", "CircuitSAT")
#let sat_cs_sol = sat_cs.solutions.at(0)
#reduction-rule("Satisfiability", "CircuitSAT",
  example: true,
  example-caption: [3-variable SAT formula to boolean circuit],
  extra: [
    #pred-commands(
      "pred create --example SAT -o sat.json",
      "pred reduce sat.json --to " + target-spec(sat_cs) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate sat.json --config " + sat_cs_sol.source_config.map(str).join(","),
    )
  ],
)[
  CNF is inherently an AND-of-ORs structure, which maps directly to a boolean circuit: each clause becomes an OR gate over its literals, and a final AND gate combines all clause outputs. The circuit is constrained to output _true_, so a satisfying circuit assignment exists iff the original formula is satisfiable.
][
  _Construction._ For $phi = C_1 and dots and C_k$ with $C_i = (ell_(i 1) or dots or ell_(i m_i))$: (1) Create circuit inputs $x_1, dots, x_n$ corresponding to SAT variables. (2) For each clause $C_i$, add an OR gate $g_i$ with inputs from the clause's literals (negated inputs use NOT gates). (3) Add a final AND gate with inputs $g_1, dots, g_k$, constrained to output _true_.

  _Correctness._ ($arrow.r.double$) A satisfying assignment makes at least one literal true in each clause, so each OR gate outputs true and the AND gate outputs true. ($arrow.l.double$) A satisfying circuit assignment has all OR gates true (forced by the AND output constraint), meaning at least one literal per clause is true --- exactly a SAT solution.

  _Solution extraction._ Return the values of the circuit input variables $x_1, dots, x_n$.
]

#let cs_sg = load-example("CircuitSAT", "SpinGlass")
#let cs_sg_sol = cs_sg.solutions.at(0)
#reduction-rule("CircuitSAT", "SpinGlass",
  example: true,
  example-caption: [1-bit full adder to Ising model],
  extra: [
    #pred-commands(
      "pred create --example CircuitSAT -o circuitsat.json",
      "pred reduce circuitsat.json --to " + target-spec(cs_sg) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate circuitsat.json --config " + cs_sg_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example Factoring -o factoring.json",
      "pred reduce factoring.json --to " + target-spec(fact_cs) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate factoring.json --config " + fact_cs_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example MaxCut -o maxcut.json",
      "pred reduce maxcut.json --to " + target-spec(mc_sg) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate maxcut.json --config " + mc_sg_sol.source_config.map(str).join(","),
    )
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
    #pred-commands(
      "pred create --example SpinGlass -o spinglass.json",
      "pred reduce spinglass.json --to " + target-spec(sg_mc) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate spinglass.json --config " + sg_mc_sol.source_config.map(str).join(","),
    )
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

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(c=1)^k x_(v,c) = 1 quad forall v in V \
    & x_(u,c) + x_(v,c) <= 1 quad forall (u, v) in E, c in {1, dots, k} \
    & x_(v,c) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A valid $k$-coloring assigns exactly one color per vertex with different colors on adjacent vertices; setting $x_(v,c) = 1$ for the assigned color satisfies all constraints. ($arrow.l.double$) Any feasible ILP solution has exactly one $x_(v,c) = 1$ per vertex; this defines a coloring, and constraint (2) ensures adjacent vertices differ.

  _Solution extraction._ For each vertex $v$, find $c$ with $x_(v,c) = 1$; assign color $c$ to $v$.
]

#reduction-rule("KColoring", "TwoDimensionalConsecutiveSets")[
  @lipski1977fct A graph 3-coloring can be encoded as a partition problem on an alphabet. Each edge becomes a size-3 subset containing the two endpoint symbols plus a unique dummy symbol, and a valid 3-coloring corresponds to partitioning the alphabet into 3 groups where each edge-subset spans exactly 3 consecutive groups with one element per group. The reduction uses $n + m$ alphabet symbols and $m$ subsets for a graph with $n$ vertices and $m$ edges.
][
  _Construction._ Given $G = (V, E)$ with $|V| = n$ and $|E| = m$, build alphabet $Sigma = V union {d_e : e in E}$ of size $n + m$. For each edge $e = {u, v} in E$, define subset $Sigma_e = {u, v, d_e}$. The collection is $cal(C) = {Sigma_e : e in E}$ with $|cal(C)| = m$ subsets, each of size 3.

  _Correctness._ ($arrow.r.double$) Given a valid 3-coloring $chi: V arrow {1, 2, 3}$, define partition groups $X_c = {v in V : chi(v) = c}$ for $c in {1, 2, 3}$. For each edge $e = {u, v}$, assign dummy $d_e$ to the unique third color $c^* in {1, 2, 3} backslash {chi(u), chi(v)}$ (which exists since $chi(u) != chi(v)$). Then $Sigma_e = {u, v, d_e}$ has its three elements in three distinct groups ${X_(chi(u)), X_(chi(v)), X_(c^*)} = {X_1, X_2, X_3}$, which are consecutive with one element per group. ($arrow.l.double$) If a valid partition into $k$ groups exists, each size-3 subset ${u, v, d_e}$ must occupy 3 distinct consecutive groups. In particular, $u$ and $v$ are in different groups. Mapping groups to colors gives a valid 3-coloring.

  _Solution extraction._ The first $n$ symbols in the target configuration correspond to the graph vertices. Their group assignments, compressed to $0, 1, 2$, yield the 3-coloring.
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

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & z_(i j) <= p_i quad forall i, j \
    & z_(i j) <= q_j quad forall i, j \
    & z_(i j) >= p_i + q_j - 1 quad forall i, j \
    & sum_(i+j=k) z_(i j) + c_(k-1) = N_k + 2 c_k quad forall k in {0, dots, m + n - 1} \
    & c_(m+n-1) = 0 \
    & p_i, q_j, z_(i j) in {0, 1}, c_k in ZZ_(>=0)
  $.

  _Correctness._ The McCormick constraints enforce $z_(i j) = p_i dot q_j$ for binary variables. The bit equations encode $p times q = N$ via carry propagation, matching array multiplier semantics.

  _Solution extraction._ Read $p = sum_i p_i 2^i$ and $q = sum_j q_j 2^j$ from the binary variables.
]

== ILP Formulations

The following reductions to Integer Linear Programming are straightforward formulations where problem constraints map directly to linear inequalities.

#reduction-rule("MaximumSetPacking", "ILP")[
  Each set is either selected or not, and every universe element may belong to at most one selected set -- an element-based constraint that is directly linear in binary indicator variables.
][
  _Construction._ Variables: $x_i in {0, 1}$ for each set $S_i in cal(S)$. The ILP is:
  $
    max quad & sum_i w_i x_i \
    "subject to" quad & sum_(S_i in.rev e) x_i <= 1 quad forall e in U \
    & x_i in {0, 1} quad forall i
  $.

  _Correctness._ ($arrow.r.double$) A valid packing chooses pairwise disjoint sets, so each element is covered at most once. ($arrow.l.double$) Any feasible binary solution covers each element at most once, hence the chosen sets are pairwise disjoint; the objective maximizes total weight.

  _Solution extraction._ $cal(P) = {S_i : x_i = 1}$.
]

#reduction-rule("MaximumMatching", "ILP")[
  Each edge is either selected or not, and each vertex may be incident to at most one selected edge -- a degree-bound constraint that is directly linear in binary edge indicators.
][
  _Construction._ Variables: $x_e in {0, 1}$ for each $e in E$. The ILP is:
  $
    max quad & sum_e w_e x_e \
    "subject to" quad & sum_(e in.rev v) x_e <= 1 quad forall v in V \
    & x_e in {0, 1} quad forall e in E
  $.

  _Correctness._ ($arrow.r.double$) A matching has at most one edge per vertex, so all degree constraints hold. ($arrow.l.double$) Any feasible solution is a matching by construction; the objective maximizes total weight.

  _Solution extraction._ $M = {e : x_e = 1}$.
]

#reduction-rule("MinimumSetCovering", "ILP")[
  Every universe element must be covered by at least one selected set -- a lower-bound constraint on the sum of indicators for sets containing that element, which is directly linear.
][
  _Construction._ Variables: $x_i in {0, 1}$ for each $S_i in cal(S)$. The ILP is:
  $
    min quad & sum_i w_i x_i \
    "subject to" quad & sum_(S_i in.rev u) x_i >= 1 quad forall u in U \
    & x_i in {0, 1} quad forall i
  $.

  _Correctness._ ($arrow.r.double$) A set cover includes at least one set containing each element, satisfying all constraints. ($arrow.l.double$) Any feasible solution covers every element; the objective minimizes total weight.

  _Solution extraction._ $cal(C) = {S_i : x_i = 1}$.
]

#reduction-rule("MinimumDominatingSet", "ILP")[
  Every vertex must be dominated -- either selected itself or adjacent to a selected vertex -- which is a lower-bound constraint on the sum of indicators over its closed neighborhood.
][
  _Construction._ Variables: $x_v in {0, 1}$ for each $v in V$. The ILP is:
  $
    min quad & sum_v w_v x_v \
    "subject to" quad & x_v + sum_(u in N(v)) x_u >= 1 quad forall v in V \
    & x_v in {0, 1} quad forall v in V
  $.

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

  The ILP is:
  $
    min quad & sum_v w_v x_v \
    "subject to" quad & o_v - o_u >= 1 - n (x_u + x_v) quad forall (u -> v) in A \
    & x_v in {0, 1}, o_v in {0, dots, n - 1} quad forall v in V
  $.

  _Correctness._ ($arrow.r.double$) If $S$ is a feedback vertex set, then $G[V backslash S]$ is a DAG with a topological ordering. Set $x_v = 1$ for $v in S$, $o_v$ to the topological position for kept vertices, and $o_v = 0$ for removed vertices. All constraints are satisfied. ($arrow.l.double$) If the ILP is feasible with all arc constraints satisfied, no directed cycle can exist among kept vertices: a cycle $v_1 -> dots -> v_k -> v_1$ would require $o_(v_1) < o_(v_2) < dots < o_(v_k) < o_(v_1)$, a contradiction.

  _Solution extraction._ $S = {v : x_v = 1}$.
]

#reduction-rule("MaximumClique", "ILP")[
  A clique requires every pair of selected vertices to be adjacent; equivalently, no two selected vertices may share a _non_-edge. This is the independent set formulation on the complement graph $overline(G)$.
][
  _Construction._ Variables: $x_v in {0, 1}$ for each $v in V$. The ILP is:
  $
    max quad & sum_v x_v \
    "subject to" quad & x_u + x_v <= 1 quad forall (u, v) in.not E \
    & x_v in {0, 1} quad forall v in V
  $.

  _Correctness._ ($arrow.r.double$) In a clique, every pair of selected vertices is adjacent, so no non-edge constraint is violated. ($arrow.l.double$) Any feasible solution selects only mutually adjacent vertices, forming a clique; the objective maximizes its size.

  _Solution extraction._ $K = {v : x_v = 1}$.
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
    #pred-commands(
      "pred create --example Knapsack -o knapsack.json",
      "pred reduce knapsack.json --to " + target-spec(ks_ilp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate knapsack.json --config " + ks_ilp_sol.source_config.map(str).join(","),
    )
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
  _Construction._ Given nonnegative weights $w_0, dots, w_(n-1)$, nonnegative values $v_0, dots, v_(n-1)$, and capacity $C$, introduce binary variables $x_0, dots, x_(n-1) in {0,1}$ where $x_i = 1$ iff item $i$ is selected. The ILP is:
  $
    max quad & sum_(i=0)^(n-1) v_i x_i \
    "subject to" quad & sum_(i=0)^(n-1) w_i x_i <= C \
    & x_i in {0, 1} quad forall i in {0, dots, n - 1}
  $.
  The target therefore has exactly $n$ variables and one linear constraint.

  _Correctness._ ($arrow.r.double$) Any feasible knapsack solution $bold(x)$ satisfies $sum_i w_i x_i <= C$, so the same binary vector is feasible for the ILP and attains identical objective value $sum_i v_i x_i$. ($arrow.l.double$) Any feasible binary ILP solution selects exactly the items with $x_i = 1$; the single inequality guarantees the chosen set fits in the knapsack, and the ILP objective equals the knapsack value. Therefore optimal solutions correspond one-to-one and preserve the optimum value.

  _Solution extraction._ Identity: return the binary variable vector $bold(x)$ as the knapsack selection.
]

#let clique_mis = load-example("MaximumClique", "MaximumIndependentSet")
#let clique_mis_sol = clique_mis.solutions.at(0)
#reduction-rule("MaximumClique", "MaximumIndependentSet",
  example: true,
  example-caption: [Path graph $P_4$: clique in $G$ maps to independent set in complement $overline(G)$.],
  extra: [
    #pred-commands(
      "pred create --example MaximumClique -o maximumclique.json",
      "pred reduce maximumclique.json --to " + target-spec(clique_mis) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate maximumclique.json --config " + clique_mis_sol.source_config.map(str).join(","),
    )
  ],
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

  The ILP is:
  $
    min quad & sum_(j=0)^(n-1) y_j \
    "subject to" quad & sum_(j=0)^(n-1) x_(i j) = 1 quad forall i in {0, dots, n - 1} \
    & sum_(i=0)^(n-1) s_i x_(i j) <= C y_j quad forall j in {0, dots, n - 1} \
    & x_(i j), y_j in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A valid packing assigns each item to exactly one bin (satisfying (1)); each bin's load is at most $C$ and $y_j = 1$ for any used bin (satisfying (2)). ($arrow.l.double$) Any feasible solution assigns each item to one bin by (1), respects capacity by (2), and the objective counts the number of open bins.

  _Solution extraction._ For each item $i$, find the unique $j$ with $x_(i j) = 1$; assign item $i$ to bin $j$.
]

#reduction-rule("IntegralFlowBundles", "ILP")[
  The feasibility conditions are already linear: one integer variable per arc, one inequality per bundle, one conservation equality per nonterminal vertex, and one lower bound on sink inflow.
][
  _Construction._ Given Integral Flow with Bundles instance $(G = (V, A), s, t, (I_j, c_j)_(j=1)^k, R)$ with arc set $A = {a_0, dots, a_(m-1)}$, create one non-negative integer variable $x_i$ for each arc $a_i$. The ILP therefore has $m$ variables.

  _Bundle constraints._ For every bundle $I_j$, add
  $sum_(a_i in I_j) x_i <= c_j$.

  _Flow conservation._ For every nonterminal vertex $v in V backslash {s, t}$, add
  $sum_(a_i = (u, v) in A) x_i - sum_(a_i = (v, w) in A) x_i = 0$.

  _Requirement constraint._ Add the sink inflow lower bound
  $sum_(a_i = (u, t) in A) x_i - sum_(a_i = (t, w) in A) x_i >= R$.

  _Objective._ Minimize 0. The target is a pure feasibility ILP, so any constant objective works.

  The ILP is:
  $
    "find" quad & (x_i)_(i = 0)^(m - 1) \
    "subject to" quad & sum_(a_i in I_j) x_i <= c_j quad forall j in {1, dots, k} \
    & sum_(a_i = (u, v) in A) x_i - sum_(a_i = (v, w) in A) x_i = 0 quad forall v in V backslash {s, t} \
    & sum_(a_i = (u, t) in A) x_i - sum_(a_i = (t, w) in A) x_i >= R \
    & x_i in ZZ_(>=0) quad forall i in {0, dots, m - 1}
  $.

  _Correctness._ ($arrow.r.double$) Any satisfying bundled flow assigns a non-negative integer to each arc, satisfies every bundle inequality by definition, satisfies every nonterminal conservation equality, and yields sink inflow at least $R$, so it is a feasible ILP solution. ($arrow.l.double$) Any feasible ILP solution gives non-negative integral arc values obeying the same bundle, conservation, and sink-inflow constraints, hence it is a satisfying solution to the original Integral Flow with Bundles instance.

  _Solution extraction._ Identity: read the ILP vector $(x_0, dots, x_(m-1))$ directly as the arc-flow vector of the source problem.
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

  The ILP is:
  $
    min quad & sum_j w_j C_j \
    "subject to" quad & l_j <= C_j <= M quad forall j \
    & C_j - C_i >= l_j quad forall i prec.eq j \
    & C_j - C_i + M (1 - y_(i j)) >= l_j quad forall i < j \
    & C_i - C_j + M y_(i j) >= l_i quad forall i < j \
    & y_(i j) in {0, 1}, C_j in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any feasible schedule defines completion times and pairwise order values satisfying the bounds, precedence inequalities, and disjunctive machine constraints; its weighted completion time is exactly the ILP objective. ($arrow.l.double$) Any feasible ILP solution assigns a strict order to every task pair and forbids overlap, so the completion times correspond to a valid single-machine schedule that respects all precedences. Minimizing the ILP objective therefore minimizes the original weighted completion-time objective.

  _Solution extraction._ Sort tasks by their completion times $C_j$ and encode that order back into the source schedule representation.
]

#let hc_tsp = load-example("HamiltonianCircuit", "TravelingSalesman")
#let hc_tsp_sol = hc_tsp.solutions.at(0)
#let hc_tsp_n = graph-num-vertices(hc_tsp.source.instance)
#let hc_tsp_source_edges = hc_tsp.source.instance.graph.edges
#let hc_tsp_target_edges = hc_tsp.target.instance.graph.edges
#let hc_tsp_target_weights = hc_tsp.target.instance.edge_weights
#let hc_tsp_weight_one = hc_tsp_target_edges.enumerate().filter(((i, _)) => hc_tsp_target_weights.at(i) == 1).map(((i, e)) => (e.at(0), e.at(1)))
#let hc_tsp_weight_two = hc_tsp_target_edges.enumerate().filter(((i, _)) => hc_tsp_target_weights.at(i) == 2).map(((i, e)) => (e.at(0), e.at(1)))
#let hc_tsp_selected_edges = hc_tsp_target_edges.enumerate().filter(((i, _)) => hc_tsp_sol.target_config.at(i) == 1).map(((i, e)) => (e.at(0), e.at(1)))
#reduction-rule("HamiltonianCircuit", "TravelingSalesman",
  example: true,
  example-caption: [Cycle graph on $#hc_tsp_n$ vertices to weighted $K_#hc_tsp_n$],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(hc_tsp.source) + " -o hc.json",
      "pred reduce hc.json --to " + target-spec(hc_tsp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate hc.json --config " + hc_tsp_sol.source_config.map(str).join(","),
    )

    *Step 1 -- Start from the source graph.* The canonical source fixture is the cycle on vertices ${0, 1, 2, 3}$ with edges #hc_tsp_source_edges.map(e => $(#e.at(0), #e.at(1))$).join(", "). The stored Hamiltonian-circuit witness is the permutation $[#hc_tsp_sol.source_config.map(str).join(", ")]$.\ 

    *Step 2 -- Complete the graph and encode adjacency by weights.* The target keeps the same $#hc_tsp_n$ vertices but adds the missing diagonals, so it becomes $K_#hc_tsp_n$ with $#graph-num-edges(hc_tsp.target.instance)$ undirected edges. The original cycle edges #hc_tsp_weight_one.map(e => $(#e.at(0), #e.at(1))$).join(", ") receive weight 1, while the diagonals #hc_tsp_weight_two.map(e => $(#e.at(0), #e.at(1))$).join(", ") receive weight 2.\ 

    *Step 3 -- Verify the canonical witness.* The stored target configuration $[#hc_tsp_sol.target_config.map(str).join(", ")]$ selects the tour edges #hc_tsp_selected_edges.map(e => $(#e.at(0), #e.at(1))$).join(", "). Its total cost is $1 + 1 + 1 + 1 = #hc_tsp_n$, so every chosen edge is a weight-1 source edge, and traversing the selected cycle recovers the Hamiltonian circuit $[#hc_tsp_sol.source_config.map(str).join(", ")]$.\ 

    *Multiplicity:* The fixture stores one canonical witness. For the 4-cycle there are $4 times 2 = 8$ Hamiltonian-circuit permutations (choice of start vertex and direction), but they all induce the same undirected target edge set.
  ],
)[
  @garey1979 This $O(n^2)$ reduction constructs the complete graph on the same vertex set and uses edge weights to distinguish source edges from non-edges: weight 1 means "present in the source" and weight 2 means "missing in the source" ($n (n - 1) / 2$ target edges).
][
  _Construction._ Given a Hamiltonian Circuit instance $G = (V, E)$ with $n = |V|$, construct the complete graph $K_n$ on the same vertex set. For each pair $u < v$, set $w(u, v) = 1$ if $(u, v) in E$ and $w(u, v) = 2$ otherwise. The target TSP instance asks for a minimum-weight Hamiltonian cycle in this weighted complete graph.

  _Correctness._ ($arrow.r.double$) If $G$ has a Hamiltonian circuit $v_0, v_1, dots, v_(n-1), v_0$, then the same cycle exists in $K_n$. Every chosen edge belongs to $E$, so each edge has weight 1 and the resulting TSP tour has total cost $n$. ($arrow.l.double$) Every TSP tour on $n$ vertices uses exactly $n$ edges, and every target edge has weight at least 1, so any tour has cost at least $n$. If the optimum cost is exactly $n$, every selected edge must therefore have weight 1. Those edges are precisely edges of $G$, so the optimal TSP tour is already a Hamiltonian circuit in the source graph.

  _Solution extraction._ Read the selected TSP edges, traverse the unique degree-2 cycle they form, and return the resulting vertex permutation as the source Hamiltonian-circuit witness.
]

#let tsp_ilp = load-example("TravelingSalesman", "ILP")
#let tsp_ilp_sol = tsp_ilp.solutions.at(0)
#reduction-rule("TravelingSalesman", "ILP",
  example: true,
  example-caption: [Weighted $K_4$: the optimal tour $0 arrow 1 arrow 3 arrow 2 arrow 0$ with cost 80 is found by position-based ILP.],
  extra: [
    #pred-commands(
      "pred create --example TSP -o tsp.json",
      "pred reduce tsp.json --to " + target-spec(tsp_ilp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate tsp.json --config " + tsp_ilp_sol.source_config.map(str).join(","),
    )
  ],
)[
  A Hamiltonian tour is a permutation of vertices. Position-based encoding assigns each vertex a tour position via binary indicators, with permutation constraints ensuring a valid bijection. The tour cost involves products of position indicators for consecutive positions, which McCormick linearization converts to auxiliary variables with linear constraints.
][
  _Construction._ For graph $G = (V, E)$ with $n = |V|$ and $m = |E|$:

  _Variables:_ Binary $x_(v,k) in {0, 1}$ for each vertex $v in V$ and position $k in {0, ..., n-1}$. Interpretation: $x_(v,k) = 1$ iff vertex $v$ is at position $k$ in the tour.

  _Auxiliary variables:_ For each edge $(u,v) in E$ and position $k$, introduce $y_(u,v,k)$ and $y_(v,u,k)$ to linearize the products $x_(u,k) dot x_(v,(k+1) mod n)$ and $x_(v,k) dot x_(u,(k+1) mod n)$ respectively.

  _Constraints:_ (1) Each vertex has exactly one position: $sum_(k=0)^(n-1) x_(v,k) = 1$ for all $v in V$. (2) Each position has exactly one vertex: $sum_(v in V) x_(v,k) = 1$ for all $k$. (3) Non-edge consecutive prohibition: if ${v,w} in.not E$, then $x_(v,k) + x_(w,(k+1) mod n) <= 1$ for all $k$. (4) McCormick: $y <= x_(v,k)$, $y <= x_(w,(k+1) mod n)$, $y >= x_(v,k) + x_(w,(k+1) mod n) - 1$.

  _Objective:_ Minimize $sum_((u,v) in E) w(u,v) dot sum_k (y_(u,v,k) + y_(v,u,k))$.

  The ILP is:
  $
    min quad & sum_((u,v) in E) w(u,v) sum_k (y_(u,v,k) + y_(v,u,k)) \
    "subject to" quad & sum_(k=0)^(n-1) x_(v,k) = 1 quad forall v in V \
    & sum_(v in V) x_(v,k) = 1 quad forall k in {0, dots, n - 1} \
    & x_(v,k) + x_(w,(k+1) mod n) <= 1 quad forall {v, w} in.not E, k in {0, dots, n - 1} \
    & y_(u,v,k) <= x_(u,k) quad forall (u, v) in E, k in {0, dots, n - 1} \
    & y_(u,v,k) <= x_(v,(k+1) mod n) quad forall (u, v) in E, k in {0, dots, n - 1} \
    & y_(u,v,k) >= x_(u,k) + x_(v,(k+1) mod n) - 1 quad forall (u, v) in E, k in {0, dots, n - 1} \
    & x_(v,k), y_(u,v,k) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A valid tour defines a permutation matrix $(x_(v,k))$ satisfying constraints (1)--(2); consecutive vertices are adjacent by construction, so (3) holds; McCormick constraints (4) force $y = x_(u,k) x_(v,k+1)$, making the objective equal to the tour cost. ($arrow.l.double$) Any feasible binary solution defines a permutation (by (1)--(2)) where consecutive positions are connected by edges (by (3)), forming a Hamiltonian tour; the linearized objective equals the tour cost.

  _Solution extraction._ For each position $k$, find vertex $v$ with $x_(v,k) = 1$ to recover the tour permutation; then select edges between consecutive positions.
]

#let tsp_qubo = load-example("TravelingSalesman", "QUBO")
#let tsp_qubo_sol = tsp_qubo.solutions.at(0)

#let lp_ilp = load-example("LongestPath", "ILP")
#let lp_ilp_sol = lp_ilp.solutions.at(0)
#reduction-rule("LongestPath", "ILP",
  example: true,
  example-caption: [The 3-vertex path $0 arrow 1 arrow 2$ encoded as a 7-variable ILP with optimum 5.],
  extra: [
    #pred-commands(
      "pred create --example LongestPath -o longest-path.json",
      "pred reduce longest-path.json --to " + target-spec(lp_ilp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate longest-path.json --config " + lp_ilp_sol.source_config.map(str).join(","),
    )
    *Step 1 -- Orient each undirected edge.* The canonical witness has two source edges, so the reduction creates four directed-arc variables. The optimal witness sets $x_(0,1) = 1$ and $x_(1,2) = 1$, leaving the reverse directions at 0.\ 

    *Step 2 -- Add order variables.* The target has #lp_ilp.target.instance.num_vars variables and #lp_ilp.target.instance.constraints.len() constraints in total. The order block $bold(o) = (#lp_ilp_sol.target_config.slice(4, 7).map(str).join(", "))$ certifies the increasing path positions $0 < 1 < 2$.\ 

    *Step 3 -- Check the objective.* The target witness $bold(z) = (#lp_ilp_sol.target_config.map(str).join(", "))$ selects lengths $2$ and $3$, so the ILP objective is $5$, matching the source optimum. #sym.checkmark
  ],
)[
  A simple $s$-$t$ path can be represented as one unit of directed flow from $s$ to $t$ on oriented copies of the undirected edges. Integer order variables then force the selected arcs to move strictly forward, which forbids detached directed cycles.
][
  _Construction._ For graph $G = (V, E)$ with $n = |V|$ and $m = |E|$:

  _Variables:_ For each undirected edge ${u, v} in E$, introduce two binary arc variables $x_(u,v), x_(v,u) in {0, 1}$. Interpretation: $x_(u,v) = 1$ iff the path traverses edge ${u, v}$ from $u$ to $v$. For each vertex $v in V$, add an integer order variable $o_v in {0, dots, n-1}$. Total: $2m + n$ variables.

  _Constraints:_ (1) Flow balance: $sum_(w : {v,w} in E) x_(v,w) - sum_(u : {u,v} in E) x_(u,v) = 1$ at the source, equals $-1$ at the target, and equals $0$ at every other vertex. (2) Degree bounds: every vertex has at most one selected outgoing arc and at most one selected incoming arc. (3) Edge exclusivity: $x_(u,v) + x_(v,u) <= 1$ for each undirected edge. (4) Ordering: for every oriented edge $u -> v$, $o_v - o_u >= 1 - n(1 - x_(u,v))$. (5) Anchor the path at the source with $o_s = 0$.

  _Objective._ Maximize $sum_({u,v} in E) l({u,v}) dot (x_(u,v) + x_(v,u))$.

  The ILP is:
  $
    max quad & sum_({u,v} in E) l({u,v}) (x_(u,v) + x_(v,u)) \
    "subject to" quad & sum_(w : {v,w} in E) x_(v,w) - sum_(u : {u,v} in E) x_(u,v) = b_v quad forall v in V \
    & sum_(w : {v,w} in E) x_(v,w) <= 1 quad forall v in V \
    & sum_(u : {u,v} in E) x_(u,v) <= 1 quad forall v in V \
    & x_(u,v) + x_(v,u) <= 1 quad forall {u, v} in E \
    & o_v - o_u >= 1 - n (1 - x_(u,v)) quad forall u -> v \
    & o_s = 0 \
    & x_(u,v) in {0, 1}, o_v in {0, dots, n - 1}
  $,
  where $b_s = 1$, $b_t = -1$, and $b_v = 0$ otherwise.

  _Correctness._ ($arrow.r.double$) Any simple $s$-$t$ path can be oriented from $s$ to $t$, giving exactly one outgoing arc at $s$, one incoming arc at $t$, balanced flow at every internal vertex, and strictly increasing order values along the path. ($arrow.l.double$) Any feasible ILP solution satisfies the flow equations and degree bounds, so the selected arcs form vertex-disjoint directed paths and cycles. The ordering inequalities make every selected arc increase the order value by at least 1, so directed cycles are impossible. The only remaining positive-flow component is therefore a single directed $s$-$t$ path, whose objective is exactly the total selected edge length.

  _Solution extraction._ For each undirected edge ${u, v}$, select it in the source configuration iff either $x_(u,v)$ or $x_(v,u)$ is 1.
]

#reduction-rule("TravelingSalesman", "QUBO",
  example: true,
  example-caption: [TSP on $K_3$ with weights $w_(01) = 1$, $w_(02) = 2$, $w_(12) = 3$: the QUBO ground state encodes the optimal tour with cost $1 + 2 + 3 = 6$.],
  extra: [
    #pred-commands(
      "pred create --example TSP -o tsp.json",
      "pred reduce tsp.json --to " + target-spec(tsp_qubo) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate tsp.json --config " + tsp_qubo_sol.source_config.map(str).join(","),
    )
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

#let lcs_mis = load-example("LongestCommonSubsequence", "MaximumIndependentSet")
#let lcs_mis_sol = lcs_mis.solutions.at(0)
#reduction-rule("LongestCommonSubsequence", "MaximumIndependentSet",
  example: true,
  example-caption: [LCS of two strings over a 3-symbol alphabet],
  extra: [
    #pred-commands(
      "pred create --example LCS -o lcs.json",
      "pred reduce lcs.json --to " + target-spec(lcs_mis) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate lcs.json --config " + lcs_mis_sol.source_config.map(str).join(","),
    )
    Source LCS: config $(#lcs_mis_sol.source_config.map(str).join(", "))$ \
    Target MIS: $S = {#lcs_mis_sol.target_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => str(i)).join(", ")}$ (size #lcs_mis_sol.target_config.filter(x => x == 1).len()) \
    MIS size $=$ LCS length $= #lcs_mis_sol.target_config.filter(x => x == 1).len()$ #sym.checkmark
  ],
)[
  A match-node construction transforms a $k$-string LCS instance into a Maximum Independent Set problem on a conflict graph. Each vertex represents a $k$-tuple of positions (one per string) that all share the same character, and edges connect pairs that cannot coexist in any valid common subsequence. The MIS of this graph equals the LCS length.
][
  _Construction._ Given $k$ strings $s_1, dots, s_k$ over alphabet $Sigma$ (size $|Sigma|$):

  _Vertices:_ For each character $c in Sigma$, create a vertex for every $k$-tuple $(p_1, dots, p_k)$ where $s_i [p_i] = c$ for all $i$. The total vertex count equals $sum_(c in Sigma) product_(i=1)^k "count"(c, s_i)$.

  _Edges:_ Two vertices $u = (a_1, dots, a_k)$ and $v = (b_1, dots, b_k)$ are connected if they _conflict_ --- they cannot both appear in a valid common subsequence. A conflict occurs when the position differences are not consistently ordered: $not (forall i: a_i < b_i)$ and $not (forall i: a_i > b_i)$.

  _Correctness._ ($arrow.r.double$) A common subsequence of length $ell$ selects $ell$ match nodes whose positions are strictly increasing in every string, so no two are adjacent --- forming an independent set of size $ell$. ($arrow.l.double$) An independent set of size $ell$ consists of $ell$ mutually non-conflicting match nodes, meaning their positions are consistently ordered across all strings. Sorting by any string's position yields a valid common subsequence of length $ell$.

  _Solution extraction._ Sort the selected vertices by position in $s_1$. Read off the characters to obtain the common subsequence, then pad to `max_length` with the padding symbol.
]

#reduction-rule("LongestCommonSubsequence", "ILP")[
  An optimization ILP formulation maximizes the length of a common subsequence. Binary variables choose a symbol (or padding) at each witness position. Match variables link active positions to source string indices, and the objective maximizes the number of non-padding positions.
][
  _Construction._ Given alphabet $Sigma$ (size $k$), strings $R = {r_1, dots, r_m}$, and maximum length $L = min_i |r_i|$:

  _Variables:_ Binary $x_(p, a) in {0, 1}$ for witness position $p in {1, dots, L}$ and symbol $a in Sigma union {bot}$ (where $bot$ is the padding symbol), with $x_(p, a) = 1$ iff position $p$ holds symbol $a$. For every input string $r_i$, witness position $p$, and source index $j in {1, dots, |r_i|}$, binary $y_(i, p, j) = 1$ iff position $p$ is matched to index $j$ of $r_i$.

  _Constraints:_ (1) Exactly one symbol (including padding) per position: $sum_(a in Sigma union {bot}) x_(p, a) = 1$ for all $p$. (2) Contiguity: $x_(p+1, bot) gt.eq x_(p, bot)$ for consecutive positions. (3) Conditional matching: $sum_(j=1)^(|r_i|) y_(i, p, j) + x_(p, bot) = 1$ for each $(i, p)$, so active positions select exactly one match and padding positions select none. (4) Character consistency: $y_(i, p, j) lt.eq x_(p, r_i[j])$. (5) Strictly increasing matches: for consecutive positions $p$ and $p + 1$, forbid $y_(i, p, j') = y_(i, p+1, j) = 1$ whenever $j' gt.eq j$.

  _Objective:_ Maximize $sum_p sum_(a in Sigma) x_(p, a)$ (the number of non-padding positions).

  The ILP is:
  $
    "maximize" quad & sum_p sum_(a in Sigma) x_(p, a) \
    "subject to" quad & sum_(a in Sigma union {bot}) x_(p, a) = 1 quad forall p in {1, dots, L} \
    & x_(p+1, bot) >= x_(p, bot) quad forall p \
    & sum_(j = 1)^(|r_i|) y_(i, p, j) + x_(p, bot) = 1 quad forall i, p \
    & y_(i, p, j) <= x_(p, r_i [j]) quad forall i, p, j \
    & y_(i, p, j') + y_(i, p + 1, j) <= 1 quad forall i, p, j' >= j \
    & x_(p, a), y_(i, p, j) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Given an optimal common subsequence $w$ of length $ell$, set $x_(p, w_p) = 1$ for $p lt.eq ell$ and $x_(p, bot) = 1$ for $p > ell$. For active positions, choose the embedding indices in each source string. All constraints are satisfied and the objective equals $ell$. ($arrow.l.double$) Any optimal ILP solution selects contiguous non-padding positions followed by padding. The active prefix, together with character consistency and ordering constraints, forms a valid common subsequence whose length equals the objective value.

  _Solution extraction._ For each position $p$, read the selected symbol $a$ (which may be $bot$). The resulting length-$L$ vector with padding is the source configuration.
]

#reduction-rule("MinimumMultiwayCut", "ILP")[
  The vertex-assignment + edge-cut indicator formulation @chopra1996 introduces binary variables for vertex-to-component membership and edge-cut indicators. Terminal vertices are fixed to their own components, partition constraints ensure every vertex belongs to exactly one component, and linking inequalities force the cut indicator on whenever an edge's endpoints are in different components.
][
  _Construction._ Given graph $G = (V, E, w)$ with $n = |V|$ vertices, $m = |E|$ edges, edge weights $w_e > 0$, and $k$ terminals $T = {t_0, dots, t_(k-1)}$:

  _Variables:_ (1) $y_(i v) in {0, 1}$ for $i in {0, dots, k-1}$, $v in V$: vertex $v$ belongs to the component of terminal $t_i$. (2) $x_e in {0, 1}$ for $e in E$: edge $e$ is in the cut. Total: $k n + m$ variables.

  _Constraints:_ (1) Terminal fixing: $y_(i, t_i) = 1$ for each $i$ (terminal $t_i$ is in its own component); $y_(j, t_i) = 0$ for $j eq.not i$ (each terminal excluded from other components). (2) Partition: $sum_(i=0)^(k-1) y_(i v) = 1$ for each $v in V$ (each vertex in exactly one component). (3) Edge-cut linking: for each edge $e = (u, v)$ and each terminal $i$: $x_e gt.eq y_(i u) - y_(i v)$ and $x_e gt.eq y_(i v) - y_(i u)$ (force $x_e = 1$ when endpoints are in different components). Total: $k^2 + n + 2 k m$ constraints.

  _Objective:_ Minimize $sum_(e in E) w_e dot x_e$.

  The ILP is:
  $
    min quad & sum_(e in E) w_e x_e \
    "subject to" quad & y_(i, t_i) = 1 quad forall i in {0, dots, k - 1} \
    & y_(j, t_i) = 0 quad forall i != j \
    & sum_(i=0)^(k-1) y_(i v) = 1 quad forall v in V \
    & x_e >= y_(i u) - y_(i v) quad forall e = (u, v) in E, i in {0, dots, k - 1} \
    & x_e >= y_(i v) - y_(i u) quad forall e = (u, v) in E, i in {0, dots, k - 1} \
    & x_e, y_(i v) in {0, 1}
  $.

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
    #pred-commands(
      "pred create --example SteinerTree -o steinertree.json",
      "pred reduce steinertree.json --to " + target-spec(st_ilp) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate steinertree.json --config " + st_ilp_sol.source_config.map(str).join(","),
    )
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

  The ILP is:
  $
    min quad & sum_(e in E) w_e y_e \
    "subject to" quad & sum_(u : (u, v) in A) f^t_(u,v) - sum_(u : (v, u) in A) f^t_(v,u) = b_(t,v) quad forall t in T backslash {r}, v in V \
    & f^t_(u,v) <= y_e quad forall t in T backslash {r}, e = {u, v} in E \
    & f^t_(v,u) <= y_e quad forall t in T backslash {r}, e = {u, v} in E \
    & y_e, f^t_(u,v) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) If $S subset.eq E$ is a Steiner tree, set $y_e = 1$ exactly for $e in S$. For each non-root terminal $t$, the unique path from $r$ to $t$ inside the tree defines a binary flow assignment satisfying the conservation equations, and every used arc lies on a selected edge, so all linking inequalities hold. The ILP objective equals $sum_(e in S) w_e$. ($arrow.l.double$) Any feasible ILP solution with edge selector set $Y = {e in E : y_e = 1}$ supports one unit of flow from $r$ to every non-root terminal, so the selected edges contain a connected subgraph spanning all terminals. Because all edge weights are strictly positive, any cycle in the selected subgraph has positive total cost; the optimizer therefore never includes redundant edges, so the selected subgraph is already a Steiner tree. Therefore an optimal ILP solution induces a minimum-cost Steiner tree.

  _Variable mapping._ The first $m$ ILP variables are the source-edge indicators $y_0, dots, y_(m-1)$ in source edge order. For terminal $t_p$ with $p in {1, dots, k-1}$, the next block of $2 m$ variables stores the directed arc indicators $f^(t_p)_(u,v)$ and $f^(t_p)_(v,u)$ for each source edge $(u, v)$.

  _Solution extraction._ Read the first $m$ target variables as the source edge-selection vector. Since those coordinates are exactly the $y_e$ variables, the extracted source configuration is valid whenever the selected subgraph is pruned to its Steiner tree witness.

  _Remark._ Zero-weight edges are excluded because they allow degenerate optimal ILP solutions containing redundant cycles at no cost; following the convention of practical solvers (e.g., SCIP-Jack @kochmartin1998steiner), such edges should be contracted before applying the reduction.
]

#let mvc_hs = load-example("MinimumVertexCover", "MinimumHittingSet")
#let mvc_hs_sol = mvc_hs.solutions.at(0)
#let mvc_hs_cover = mvc_hs_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#let mvc_hs_hit = mvc_hs_sol.target_config.enumerate().filter(((i, x)) => x == 1).map(((i, x)) => i)
#reduction-rule("MinimumVertexCover", "MinimumHittingSet",
  example: true,
  example-caption: [Unit-weight VC to Hitting Set ($n = #graph-num-vertices(mvc_hs.source.instance)$, $|E| = #graph-num-edges(mvc_hs.source.instance)$)],
  extra: [
    #pred-commands(
      "pred create --example 'MVC {weight: One}' -o mvc.json",
      "pred reduce mvc.json --to " + target-spec(mvc_hs) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate mvc.json --config " + mvc_hs_sol.source_config.map(str).join(","),
    )
    Source VC: $C = {#mvc_hs_cover.map(str).join(", ")}$ (size #mvc_hs_cover.len()) #h(1em)
    Target HS: $H = {#mvc_hs_hit.map(str).join(", ")}$ (size #mvc_hs_hit.len()) \
    The hitting set $H$ is identical to the vertex cover $C$ because the universe elements are the vertices and the subsets are the edges.
  ],
)[
  Vertex Cover is the special case of Hitting Set where every set has exactly two elements @garey1979. Given a unit-weight VC instance $G = (V, E)$, let the universe $U = V$ and define one 2-element subset ${u, v}$ per edge $(u, v) in E$. The budget is unchanged.
][
  _Construction._ Given unit-weight VC instance $(G, k)$ with $G = (V, E)$, construct Hitting Set instance $(U, cal(S), k)$:
  - Universe: $U = V$ with $|U| = |V|$ elements.
  - Collection: $cal(S) = {{u, v} : (u, v) in E}$ with $|cal(S)| = |E|$ subsets, each of size 2.
  - Budget: $k' = k$ (unchanged).

  _Correctness._ ($arrow.r.double$) If $C subset.eq V$ is a vertex cover, then for every edge $(u, v) in E$, at least one of $u, v$ lies in $C$, so $C$ intersects the subset ${u, v} in cal(S)$. Hence $C$ is a hitting set.
  ($arrow.l.double$) If $H subset.eq U$ hits every subset ${u, v} in cal(S)$, then for every edge $(u, v) in E$, $H$ contains $u$ or $v$, so $H$ is a vertex cover.

  Since both problems minimise cardinality (unit weights), an optimal vertex cover of size $k$ corresponds to an optimal hitting set of the same size.

  _Solution extraction._ The hitting set $H$ is directly the vertex cover: $c_v = h_v$ for each $v in V$.
]

#reduction-rule("MinimumHittingSet", "ILP")[
  Each set must contain at least one selected element -- a standard set-covering constraint on the element indicators.
][
  _Construction._ Variables: $x_e in {0, 1}$ for each element $e in U$. The ILP is:
  $
    min quad & sum_e x_e \
    "subject to" quad & sum_(e in S) x_e >= 1 quad forall S in cal(S) \
    & x_e in {0, 1} quad forall e in U
  $.

  _Correctness._ ($arrow.r.double$) A hitting set includes at least one element from each set. ($arrow.l.double$) Any feasible solution hits every set.

  _Solution extraction._ $H = {e : x_e = 1}$.
]

#reduction-rule("ExactCoverBy3Sets", "ILP")[
  Each element must be covered by exactly one triple, and the number of selected triples must equal $|U|\/3$.
][
  _Construction._ Variables: $x_j in {0, 1}$ for each triple $T_j$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(j : e in T_j) x_j = 1 quad forall e in U \
    & sum_j x_j = |U| / 3 \
    & x_j in {0, 1} quad forall j
  $.

  _Correctness._ The equality constraints force each element to appear in exactly one selected triple, which is the definition of an exact cover.

  _Solution extraction._ $cal(C) = {T_j : x_j = 1}$.
]

#reduction-rule("NAESatisfiability", "ILP")[
  Each clause must have at least one true and at least one false literal, encoded as a pair of linear inequalities per clause.
][
  _Construction._ Variables: $x_i in {0, 1}$ per Boolean variable. For each clause $C$ with literals $l_1, dots, l_k$, substitute $l_i = x_i$ for positive and $l_i = 1 - x_i$ for negative literals. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_i "coeff"_(C,i) x_i >= 1 - "neg"(C) quad "for each clause" C \
    & sum_i "coeff"_(C,i) x_i <= |C| - 1 - "neg"(C) quad "for each clause" C \
    & x_i in {0, 1} quad forall i
  $.

  _Correctness._ The two constraints per clause jointly enforce the not-all-equal condition.

  _Solution extraction._ Direct: $x_i = 1$ iff variable $i$ is true.
]

#reduction-rule("SetSplitting", "ILP")[
  Each subset must contain at least one element of each color, encoded as a pair of linear inequalities per subset.
][
  _Construction._ Variables: $x_i in {0, 1}$ for each $u_i in U$ ($x_i = 1$ means $u_i in S_2$). For each subset $C = {i_1, dots, i_k}$ the ILP enforces:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(j in C) x_j >= 1 quad "for each" C in cal(C) \
    & sum_(j in C) x_j <= |C| - 1 quad "for each" C in cal(C) \
    & x_i in {0, 1} quad forall i
  $.

  _Correctness._ ($arrow.r.double$) A valid splitting has at least one element in $S_2$ ($sum >= 1$) and at least one in $S_1$ ($sum <= |C|-1$) for every $C$. ($arrow.l.double$) Any feasible ILP solution defines a valid 2-coloring.

  _Solution extraction._ $S_2 = {u_i : x_i = 1}$, $S_1 = U without S_2$.
]

#reduction-rule("KClique", "ILP")[
  A $k$-clique requires at least $k$ selected vertices with no non-edge between any pair.
][
  _Construction._ Variables: $x_v in {0, 1}$ for each $v in V$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_v x_v >= k \
    & x_u + x_v <= 1 quad forall (u, v) in.not E \
    & x_v in {0, 1} quad forall v in V
  $.

  _Correctness._ ($arrow.r.double$) A $k$-clique selects $>= k$ mutually adjacent vertices, satisfying all constraints. ($arrow.l.double$) Any feasible solution selects $>= k$ vertices with no non-edge pair, forming a clique of size $>= k$.

  _Solution extraction._ $K = {v : x_v = 1}$.
]

#{
  let kc_bcbs = load-example("KClique", "BalancedCompleteBipartiteSubgraph",
    source-variant: ("graph": "SimpleGraph"))
  let kc_bcbs_sol = kc_bcbs.solutions.at(0)
  let src = kc_bcbs.source.instance
  let tgt = kc_bcbs.target.instance
  let n = src.graph.num_vertices
  let m = src.graph.edges.len()
  let k = src.k
  let ck2 = calc.div-euclid(k * (k - 1), 2)
  let n_prime = n + ck2
  let target_k = n_prime - k
  [
#reduction-rule("KClique", "BalancedCompleteBipartiteSubgraph",
  example: true,
  example-caption: [#n\-vertex graph with $k = #k$: non-incidence gadget construction],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(kc_bcbs.source) + " -o kclique.json",
      "pred reduce kclique.json --to " + target-spec(kc_bcbs) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate kclique.json --config " + kc_bcbs_sol.source_config.map(str).join(","),
    )

    *Step 1 -- Pad the vertex set.* $C(#k, 2) = #ck2$ padding vertices are added, giving $n' = #n + #ck2 = #n_prime$ left vertices (Part $A$).

    *Step 2 -- Build Part $B$.* Part $B$ has $#m$ edge elements (one per original edge) plus $#n - #k = #{n - k}$ padding elements, for $|B| = #tgt.graph.right_size$.

    *Step 3 -- Bipartite edges.* For each $v in A$ and edge element $e_j = {u, w}$, add $(v, e_j)$ iff $v in.not {u, w}$ (non-incidence). All padding elements connect to all left vertices.

    *Step 4 -- Set target parameter.* $K' = n' - k = #n_prime - #k = #target_k$.

    *Step 5 -- Verify a solution.* The #k\-clique is $S = {#kc_bcbs_sol.source_config.enumerate().filter(((i, x)) => x == 1).map(((i, _)) => str(i)).join(", ")}$. The #target_k left vertices NOT in $S$ plus the #ck2 padding vertices form the left side $A'$. The right side $B'$ contains the #ck2 intra-clique edge elements plus #{n - k} padding elements ($|B'| = #target_k$). All $#target_k times #target_k$ cross-edges are present because no $v in A'$ is an endpoint of any selected edge element.

    *Multiplicity:* The fixture stores one canonical witness.
  ],
)[
  This $O(n^2 + n m)$ reduction (Johnson, 1987; Garey and Johnson GT24) constructs a bipartite graph $H = (A union.dot B, F)$ with $|A| = n + binom(k, 2)$ and $|B| = m + n - k$ using a non-incidence encoding. The target biclique size is $K' = n + binom(k, 2) - k$.
][
  _Construction._ Given $k$-Clique instance $(G = (V, E), k)$ with $n = |V|$, $m = |E|$: Let $C = binom(k, 2) = k(k-1)/2$. Add $C$ isolated vertices to $V$, giving $V' = {v_0, ..., v_(n'-1)}$ with $n' = n + C$. Part $A = V'$. Part $B$ has $m$ _edge elements_ ${e_0, ..., e_(m-1)}$ (one per edge of $G$) and $n - k$ _padding elements_ ${w_0, ..., w_(n-k-1)}$. Add bipartite edge $(v, e_j)$ iff $v$ is NOT an endpoint of $e_j$ (non-incidence). Add $(v, w_i)$ for all $v in A$, $w_i in W$ (full padding). Set $K' = n' - k$.

  _Correctness._ ($arrow.r.double$) If $S subset.eq V$ is a $k$-clique, let $A' = V' without S$ ($|A'| = n' - k = K'$) and $B' = E(S) union W$ where $E(S)$ is the set of intra-clique edges. Since $|E(S)| = C$ and $|W| = n - k$, we have $|B'| = K'$. For any $v in A'$ and $e_j in E(S)$: both endpoints of $e_j$ lie in $S$ but $v in.not S$, so $v$ is not an endpoint --- the non-incidence edge exists. For padding elements, all edges exist by construction. ($arrow.l.double$) If $(A', B')$ is a balanced $K'$-biclique, let $S = {v in V : v in.not A'}$ with $|S| = k$. Any edge $e_j$ with an endpoint $u in A'$ cannot be in $B'$ (since $(u, e_j) in.not F$). So $B' inter E subset.eq E(S)$. Since $|B'| = K'$ and $|W| = n - k$, we need $|B' inter E| >= K' - |W| = C$. But $|E(S)| <= binom(k, 2) = C$, so $|E(S)| = C$, meaning $S$ is a $k$-clique.

  _Solution extraction._ For each original vertex $v in {0, ..., n-1}$: $"source"[v] = 1 - "target"[v]$ (vertices NOT selected on the left side form the clique).
]
  ]
}

#reduction-rule("MaximalIS", "ILP")[
  An independent set that is also maximal: no vertex outside the set can be added without violating independence.
][
  _Construction._ Variables: $x_v in {0, 1}$ for each $v in V$. The ILP is:
  $
    max quad & sum_v w_v x_v \
    "subject to" quad & x_u + x_v <= 1 quad forall (u, v) in E \
    & x_v + sum_(u in N(v)) x_u >= 1 quad forall v in V \
    & x_v in {0, 1} quad forall v in V
  $.

  _Correctness._ Independence constraints prevent adjacent selections; maximality constraints ensure every vertex is either selected or has a selected neighbor.

  _Solution extraction._ $I = {v : x_v = 1}$.
]

#reduction-rule("MinimumMaximalMatching", "ILP")[
  Each edge is either selected or not; matching and maximality constraints are both directly linear in binary edge indicators.
][
  _Construction._ Variables: $x_e in {0, 1}$ for each $e in E$. The ILP is:
  $
    min quad & sum_e x_e \
    "subject to" quad & sum_(e in.rev v) x_e <= 1 quad forall v in V \
    & x_j + sum_(i : i ~ j,\ i eq.not j) x_i >= 1 quad forall j in E \
    & x_e in {0, 1} quad forall e in E
  $,
  where $i ~ j$ denotes that edges $i$ and $j$ share an endpoint.

  _Correctness._ Degree constraints enforce the matching property. For each edge $j$, the maximality constraint requires that $j$ itself or at least one adjacent edge is selected, ensuring the matching cannot be extended. ($arrow.r.double$) A minimum maximal matching satisfies both constraints and minimizes cardinality. ($arrow.l.double$) Any feasible solution is a maximal matching; the objective minimizes its size.

  _Solution extraction._ $M = {e : x_e = 1}$.
]

#reduction-rule("PartiallyOrderedKnapsack", "ILP")[
  Standard knapsack with precedence constraints: item $b$ can only be selected if item $a$ is also selected for each precedence $(a, b)$.
][
  _Construction._ Variables: $x_i in {0, 1}$ per item. The ILP is:
  $
    max quad & sum_i v_i x_i \
    "subject to" quad & sum_i w_i x_i <= C \
    & x_b <= x_a quad "for each precedence" (a, b) \
    & x_i in {0, 1} quad forall i
  $.

  _Correctness._ Capacity and precedence constraints are directly linear. Any feasible ILP solution is a valid knapsack packing respecting the partial order.

  _Solution extraction._ Selected items: ${i : x_i = 1}$.
]

#reduction-rule("RectilinearPictureCompression", "ILP")[
  Cover all 1-cells with at most $B$ maximal all-1 rectangles.
][
  _Construction._ Variables: $x_r in {0, 1}$ per maximal rectangle $r$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(r "covers" (i,j)) x_r >= 1 quad forall (i, j) "with source cell" = 1 \
    & sum_r x_r <= B \
    & x_r in {0, 1} quad forall r
  $.

  _Correctness._ Coverage constraints ensure every 1-cell is covered; the cardinality bound limits the number of rectangles.

  _Solution extraction._ Selected rectangles: ${r : x_r = 1}$.
]

#reduction-rule("ShortestWeightConstrainedPath", "ILP")[
  Find a minimum-length $s$-$t$ path subject to a weight budget, using directed arc variables with MTZ ordering $o_v - o_u >= 1 - M (1 - a_(u,v))$ on selected arcs to prevent subtours.
][
  _Construction._ Let $A$ contain both orientations of every undirected edge and let $M = n$. Variables: binary $a_(u,v) in {0, 1}$ for each directed arc $(u, v) in A$, plus integer $o_v in {0, dots, n-1}$ per vertex. The ILP is:
  $
    "minimize" quad & sum_((u,v) in A) l_(u,v) a_(u,v) \
    "subject to" quad & sum_(w : (v, w) in A) a_(v,w) - sum_(u : (u, v) in A) a_(u,v) = b_v quad forall v in V \
    & sum_(w : (v, w) in A) a_(v,w) <= 1 quad forall v in V \
    & sum_(u : (u, v) in A) a_(u,v) <= 1 quad forall v in V \
    & a_(u,v) + a_(v,u) <= 1 quad forall {u, v} in E \
    & o_v - o_u >= 1 - M (1 - a_(u,v)) quad forall (u, v) in A \
    & sum_((u,v) in A) w_(u,v) a_(u,v) <= W \
    & a_(u,v) in {0, 1}, o_v in {0, dots, n - 1}
  $,
  where $b_s = 1$, $b_t = -1$, and $b_v = 0$ otherwise.

  _Correctness._ Flow balance forces an $s$-$t$ path; the MTZ inequalities apply only on selected arcs and therefore eliminate subtours; the weight constraint enforces the budget; the objective minimizes total path length.

  _Solution extraction._ Edge $\{u, v\}$ is selected iff $a_(u,v) + a_(v,u) > 0$.
]

#reduction-rule("MultipleCopyFileAllocation", "ILP")[
  Place file copies at vertices to minimize total storage plus weighted access cost.
][
  _Construction._ Variables: binary $x_v$ (copy at $v$) and $y_(v,u)$ (vertex $v$ served by copy at $u$). The ILP is:
  $
    "minimize" quad & sum_v s_v x_v + sum_(v,u) "usage"_v d(v, u) y_(v,u) \
    "subject to" quad & sum_u y_(v,u) = 1 quad forall v \
    & y_(v,u) <= x_u quad forall v, u \
    & x_v, y_(v,u) in {0, 1}
  $.

  _Correctness._ Assignment constraints ensure each vertex is served by exactly one copy; capacity links prevent assignment to non-copy vertices; the objective linearizes the total cost.

  _Solution extraction._ Copy placement: ${v : x_v = 1}$.
]

#reduction-rule("MinimumSumMulticenter", "ILP")[
  Select $k$ centers and assign each vertex to a center, minimizing the total weighted distance.
][
  _Construction._ Variables: binary $x_j$ (vertex $j$ is center), $y_(i,j)$ (vertex $i$ assigned to center $j$). The ILP is:
  $
    min quad & sum_(i,j) w_i d(i, j) y_(i,j) \
    "subject to" quad & sum_j x_j = k \
    & y_(i,j) <= x_j quad forall i, j \
    & sum_j y_(i,j) = 1 quad forall i \
    & x_j, y_(i,j) in {0, 1}
  $.

  _Correctness._ The assignment structure and cardinality constraint directly encode the $k$-median objective with precomputed shortest-path distances.

  _Solution extraction._ Centers: ${j : x_j = 1}$.
]

#reduction-rule("MinMaxMulticenter", "ILP")[
  Select $k$ centers minimizing the maximum weighted distance from any vertex to its assigned center.
][
  _Construction._ Same assignment structure as MinimumSumMulticenter (binary $x_j$, $y_(i,j)$), plus an integer variable $z$. The ILP is:
  $
    "minimize" quad & z \
    "subject to" quad & sum_j x_j = k \
    & y_(i,j) <= x_j quad forall i, j \
    & sum_j y_(i,j) = 1 quad forall i \
    & sum_j w_i d(i, j) y_(i,j) <= z quad forall i \
    & x_j, y_(i,j) in {0, 1}, z in bb(Z)
  $.

  _Correctness._ Each minimax constraint forces $z$ to be at least the weighted distance from vertex $i$ to its assigned center. Minimizing $z$ yields the optimal maximum weighted distance.

  _Solution extraction._ Centers: ${j : x_j = 1}$.
]

#reduction-rule("MultiprocessorScheduling", "ILP")[
  Assign tasks to processors so that no processor's total load exceeds the deadline.
][
  _Construction._ Variables: binary $x_(j,p)$ (task $j$ on processor $p$), one-hot per task. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_p x_(j,p) = 1 quad forall j \
    & sum_j l_j x_(j,p) <= D quad forall p \
    & x_(j,p) in {0, 1}
  $.

  _Correctness._ One-hot constraints ensure each task is assigned to exactly one processor; load constraints enforce the deadline on every processor.

  _Solution extraction._ Task $j$ goes to processor $arg max_p x_(j,p)$.
]

#reduction-rule("CapacityAssignment", "ILP")[
  Assign a capacity level to each link to minimize total cost subject to a delay budget.
][
  _Construction._ Variables: binary $x_(l,c)$ (link $l$ gets capacity $c$), one-hot per link. The ILP is:
  $
    "minimize" quad & sum_(l,c) "cost"[l][c] x_(l,c) \
    "subject to" quad & sum_c x_(l,c) = 1 quad forall l \
    & sum_(l,c) "delay"[l][c] x_(l,c) <= J \
    & x_(l,c) in {0, 1}
  $.

  _Correctness._ One-hot constraints fix one capacity per link; the delay budget constraint is linear in the indicators; the objective sums the selected costs.

  _Solution extraction._ Link $l$ gets capacity $arg max_c x_(l,c)$.
]

#reduction-rule("ExpectedRetrievalCost", "ILP")[
  Assign records to sectors to minimize expected retrieval cost, using product linearization for the quadratic cost terms.
][
  _Construction._ Variables: binary $x_(r,s)$ (record $r$ in sector $s$), one-hot per record, plus linearization variables $z_((r,s),(r',s')) = x_(r,s) dot x_(r',s')$. The ILP is:
  $
    "minimize" quad & sum d(s,s') p_r p_(r') z_((r,s),(r',s')) \
    "subject to" quad & sum_s x_(r,s) = 1 quad forall r \
    & z_((r,s),(r',s')) <= x_(r,s) quad forall r, s, r', s' \
    & z_((r,s),(r',s')) <= x_(r',s') quad forall r, s, r', s' \
    & z_((r,s),(r',s')) >= x_(r,s) + x_(r',s') - 1 quad forall r, s, r', s' \
    & x_(r,s), z_((r,s),(r',s')) in {0, 1}
  $.

  _Correctness._ McCormick constraints force $z$ to equal the product of binary indicators, linearizing the quadratic cost. The ILP objective directly encodes the expected retrieval cost.

  _Solution extraction._ Record $r$ goes to sector $arg max_s x_(r,s)$.
]

#reduction-rule("PartitionIntoTriangles", "ILP")[
  Partition vertices into groups of 3 such that each group forms a triangle in the graph.
][
  _Construction._ Variables: binary $x_(v,g)$ (vertex $v$ in group $g$), one-hot per vertex, $q = n\/3$ groups. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_g x_(v,g) = 1 quad forall v \
    & sum_v x_(v,g) = 3 quad forall g in {1, dots, q} \
    & x_(u,g) + x_(v,g) <= 1 quad forall g in {1, dots, q}, (u, v) in.not E \
    & x_(v,g) in {0, 1}
  $.

  _Correctness._ Size-3 groups with no non-edge pair within any group forces each group to be a triangle.

  _Solution extraction._ Vertex $v$ goes to group $arg max_g x_(v,g)$.
]

#reduction-rule("PartitionIntoPathsOfLength2", "ILP")[
  Partition vertices into groups of 3 such that each group induces a path of length 2 (at least 2 edges within the group).
][
  _Construction._ Variables: binary $x_(v,g)$ plus product linearization variables $z_((u,v),g) = x_(u,g) dot x_(v,g)$ for edges $(u, v)$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_g x_(v,g) = 1 quad forall v \
    & sum_v x_(v,g) = 3 quad forall g \
    & sum_((u,v) in E) z_((u,v),g) >= 2 quad forall g \
    & z_((u,v),g) <= x_(u,g) quad forall (u, v) in E, g \
    & z_((u,v),g) <= x_(v,g) quad forall (u, v) in E, g \
    & z_((u,v),g) >= x_(u,g) + x_(v,g) - 1 quad forall (u, v) in E, g \
    & x_(v,g), z_((u,v),g) in {0, 1}
  $.

  _Correctness._ The edge count constraint ensures connectivity within each group. Combined with group size 3, this forces a path of length 2.

  _Solution extraction._ Vertex $v$ goes to group $arg max_g x_(v,g)$.
]

#let ppl2_bcsf = load-example("PartitionIntoPathsOfLength2", "BoundedComponentSpanningForest")
#let ppl2_bcsf_sol = ppl2_bcsf.solutions.at(0)
#reduction-rule("PartitionIntoPathsOfLength2", "BoundedComponentSpanningForest",
  example: true,
  example-caption: [6-vertex graph ($n = 6$, $q = 2$): two $P_3$ paths],
  extra: [
    #pred-commands(
      "pred create --example PartitionIntoPathsOfLength2 -o ppl2.json",
      "pred reduce ppl2.json --to " + target-spec(ppl2_bcsf) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate ppl2.json --config " + ppl2_bcsf_sol.source_config.map(str).join(","),
    )
    Source PPL2: groups $= {#ppl2_bcsf_sol.source_config.map(str).join(", ")}$ on a graph with $n = #graph-num-vertices(ppl2_bcsf.source.instance)$ vertices and $|E| = #graph-num-edges(ppl2_bcsf.source.instance)$ edges \
    Target BCSF: components $= {#ppl2_bcsf_sol.target_config.map(str).join(", ")}$, $K = #ppl2_bcsf.target.instance.max_components$, $B = #ppl2_bcsf.target.instance.max_weight$ \
    Identity mapping: source and target configs coincide #sym.checkmark
  ],
)[
  This $O(n + m)$ parameter-setting reduction (Hadlock, 1974; Garey and Johnson @garey1979[ND10, p.~208]) constructs a Bounded Component Spanning Forest instance on the same graph with unit vertex weights, $K = |V| slash 3$ components, and weight bound $B = 3$.
][
  _Construction._ Given a Partition into Paths of Length 2 instance on graph $G = (V, E)$ with $|V| = 3q$:
  - Graph: use $G$ unchanged.
  - Vertex weights: $w(v) = 1$ for all $v in V$.
  - Component bound: $K = q = |V| slash 3$.
  - Weight bound: $B = 3$.

  _Correctness._ ($arrow.r.double$) Suppose $V$ has a valid $P_3$-partition $V_1, dots, V_q$ where each $V_t$ induces at least 2 edges. Since a graph on 3 vertices with at least 2 edges is connected, each $V_t$ is a connected component of weight $1 + 1 + 1 = 3 <= B$. There are $q = K$ components. ($arrow.l.double$) Suppose $V$ has a partition into at most $K = q$ connected components each of weight at most $B = 3$. Since all weights are 1, each component has at most 3 vertices. With $3q$ vertices and at most $q$ components, the pigeonhole principle forces exactly $q$ components of exactly 3 vertices each. A connected graph on 3 vertices has at least 2 edges (a path $P_3$ or a triangle $K_3$), satisfying the $P_3$-partition requirement.

  _Solution extraction._ Identity: the component assignment in BCSF is directly a group assignment in PPL2.
]

#reduction-rule("SumOfSquaresPartition", "ILP")[
  Partition elements into groups minimizing $sum_g (sum_(i in g) s_i)^2$.
][
  _Construction._ Variables: binary $x_(i,g)$ (element $i$ in group $g$), plus $z_((i,j),g) = x_(i,g) dot x_(j,g)$. The ILP is:
  $
    "minimize" quad & sum_g sum_(i,j) s_i s_j z_((i,j),g) \
    "subject to" quad & sum_g x_(i,g) = 1 quad forall i \
    & z_((i,j),g) <= x_(i,g) quad forall i, j, g \
    & z_((i,j),g) <= x_(j,g) quad forall i, j, g \
    & z_((i,j),g) >= x_(i,g) + x_(j,g) - 1 quad forall i, j, g \
    & x_(i,g), z_((i,j),g) in {0, 1}
  $.

  _Correctness._ Product linearization captures the quadratic sum-of-squares objective; the ILP minimizes the linearized form directly.

  _Solution extraction._ Element $i$ goes to group $arg max_g x_(i,g)$.
]

#reduction-rule("PrecedenceConstrainedScheduling", "ILP")[
  Assign unit-length tasks to time slots on $m$ processors, respecting precedence constraints and a deadline.
][
  _Construction._ Variables: binary $x_(j,t)$ (task $j$ at time $t$), one-hot per task. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_t x_(j,t) = 1 quad forall j \
    & sum_j x_(j,t) <= m quad forall t \
    & sum_t t x_(j,t) >= sum_t t x_(i,t) + 1 quad "for each precedence" (i, j) \
    & x_(j,t) in {0, 1}
  $.

  _Correctness._ One-hot ensures each task is scheduled once; capacity limits processors per slot; precedence is linearized via weighted time indicators.

  _Solution extraction._ Task $j$ is scheduled at time $arg max_t x_(j,t)$.
]

#reduction-rule("SchedulingWithIndividualDeadlines", "ILP")[
  Schedule unit-length tasks on $m$ processors, each task $j$ must complete before its individual deadline $d_j$.
][
  _Construction._ Variables: binary $x_(j,t)$ (task $j$ at time $t in {0, dots, d_j - 1}$), one-hot per task. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(t = 0)^(d_j - 1) x_(j,t) = 1 quad forall j \
    & sum_j x_(j,t) <= m quad forall t \
    & sum_t t x_(j,t) >= sum_t t x_(i,t) + 1 quad "for each precedence" (i, j) \
    & x_(j,t) in {0, 1}
  $.

  _Correctness._ Per-task deadline is enforced by restricting the time domain of each task's indicator variables.

  _Solution extraction._ Task $j$ is scheduled at time $arg max_t x_(j,t)$.
]

#reduction-rule("PreemptiveScheduling", "ILP")[
  Minimize makespan for preemptive parallel scheduling with variable-length tasks and precedence constraints.
][
  _Construction._ Let $D = sum_t ell(t)$ be the horizon. Variables: binary $x_(t,u) in {0,1}$ (task $t$ processed at slot $u$) for $t in {0, dots, n-1}$, $u in {0, dots, D-1}$; integer $M in {0, dots, D}$ (makespan). The ILP is:
  $
    min quad & M \
    "subject to" quad & sum_u x_(t,u) = ell(t) quad forall t quad "(work)" \
    & sum_t x_(t,u) <= m quad forall u quad "(capacity)" \
    & sum_u u dot x_(j,u) - sum_u u dot x_(i,u) >= 1 quad "for each" (i prec j) quad "(precedence)" \
    & M - (u+1) dot x_(t,u) >= 0 quad forall t, u quad "(makespan)" \
    & x_(t,u) in {0, 1}, quad M in ZZ_(>= 0)
  $.

  _Correctness._ Work constraints enforce each task runs for exactly $ell(t)$ slots. Capacity limits at most $m$ tasks per slot. Precedences are enforced by weighted time indicators. Makespan lower bounds force $M >= u+1$ whenever task $t$ is active at slot $u$.

  _Solution extraction._ Config$[t dot D + u] = x_(t,u)$ for all $t, u$.
]

#reduction-rule("SequencingWithinIntervals", "ILP")[
  Schedule tasks with release times, deadlines, and processing lengths on a single machine without overlap.
][
  _Construction._ Variables: binary $x_(j,t)$ (task $j$ starts at time $t in [r_j, d_j - l_j]$), one-hot per task. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(t = r_j)^(d_j - l_j) x_(j,t) = 1 quad forall j \
    & sum_(j, t : t <= tau < t + l_j) x_(j,t) <= 1 quad forall tau \
    & x_(j,t) in {0, 1}
  $.

  _Correctness._ One-hot ensures each task starts once within its feasible window; non-overlap prevents simultaneous execution.

  _Solution extraction._ Task $j$ starts at time $arg max_t x_(j,t)$; config$[j] = t - r_j$.
]

#reduction-rule("MinimumFeedbackArcSet", "ILP")[
  Remove minimum-weight arcs to make a directed graph acyclic, using MTZ-style ordering to enforce acyclicity among kept arcs.
][
  _Construction._ Variables: binary $y_a in {0, 1}$ per arc ($y_a = 1$ iff removed), integer $o_v in {0, dots, n-1}$ per vertex. The ILP is:
  $
    min quad & sum_a w_a y_a \
    "subject to" quad & o_v - o_u >= 1 - n y_a quad forall a = (u -> v) \
    & y_a in {0, 1} quad forall a \
    & o_v in {0, dots, n - 1} quad forall v
  $.

  _Correctness._ ($arrow.r.double$) Removing a FAS leaves a DAG with a topological ordering satisfying all constraints. ($arrow.l.double$) Among kept arcs, the ordering variables enforce acyclicity: a cycle would require $o_(v_1) < dots < o_(v_k) < o_(v_1)$, a contradiction.

  _Solution extraction._ Removed arcs: ${a : y_a = 1}$.
]

#reduction-rule("UndirectedTwoCommodityIntegralFlow", "ILP")[
  Route two commodities on an undirected graph with shared edge capacities, using direction indicators to enforce anti-parallel flow constraints.
][
  _Construction._ Variables: integer flow variables $f^k_(u,v), f^k_(v,u)$ per edge per commodity ($k in {1, 2}$), plus binary direction indicators $d^k_e$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & f^k_(u,v) <= "cap"_e d^k_e quad forall e = {u, v} in E, k in {1, 2} \
    & f^k_(v,u) <= "cap"_e (1 - d^k_e) quad forall e = {u, v} in E, k in {1, 2} \
    & sum_(k=1)^2 (f^k_(u,v) + f^k_(v,u)) <= "cap"_e quad forall e = {u, v} in E \
    & sum_(w) f^k_(v,w) - sum_(u) f^k_(u,v) = b^k_v quad forall k in {1, 2}, v in V \
    & d^k_e in {0, 1}, f^k_(u,v) in ZZ_(>=0)
  $.

  _Correctness._ Direction indicators linearize the capacity-sharing constraint; flow conservation and demand constraints ensure valid multi-commodity flow.

  _Solution extraction._ Flow variables (first $4|E|$ variables).
]

#reduction-rule("DirectedTwoCommodityIntegralFlow", "ILP")[
  Route two commodities on a directed graph with shared arc capacities.
][
  _Construction._ Variables: integer $f^1_a, f^2_a >= 0$ per arc $a$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & f^1_a + f^2_a <= "cap"(a) quad forall a in A \
    & sum_(a in delta^+(v)) f^k_a - sum_(a in delta^-(v)) f^k_a = b^k_v quad forall k in {1, 2}, v in V \
    & sum_(a in delta^-(t_k)) f^k_a - sum_(a in delta^+(t_k)) f^k_a >= R_k quad forall k in {1, 2} \
    & f^1_a, f^2_a in ZZ_(>=0) quad forall a in A
  $.

  _Correctness._ Joint capacity and conservation constraints directly encode the two-commodity flow problem.

  _Solution extraction._ Direct: $2|A|$ flow variables.
]

#reduction-rule("UndirectedFlowLowerBounds", "ILP")[
  Find a feasible single-commodity flow on an undirected graph with both upper and lower capacity bounds per edge.
][
  _Construction._ Variables: integer $f_(u,v), f_(v,u) >= 0$ per edge, plus direction indicator $z_e in {0, 1}$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & f_(u,v) <= "cap"_e z_e quad forall e = {u, v} in E \
    & f_(v,u) <= "cap"_e (1 - z_e) quad forall e = {u, v} in E \
    & f_(u,v) >= "lower"_e z_e quad forall e = {u, v} in E \
    & f_(v,u) >= "lower"_e (1 - z_e) quad forall e = {u, v} in E \
    & sum_(a in delta^+(v)) f_a - sum_(a in delta^-(v)) f_a = b_v quad forall v in V \
    & sum_(a in delta^-(t)) f_a - sum_(a in delta^+(t)) f_a >= R \
    & z_e in {0, 1}, f_(u,v) in ZZ_(>=0)
  $.

  _Correctness._ Direction indicators force flow in one direction per edge; bounds enforce both upper and lower capacity limits.

  _Solution extraction._ Edge orientations: $z_e$ values.
]

// Flow-based

#reduction-rule("IntegralFlowHomologousArcs", "ILP")[
  Use one integer flow variable per arc, with standard conservation plus equality constraints on every homologous pair.
][
  _Construction._ Variables: integer $f_a >= 0$ per arc $a in A$. The ILP is:
  $
    "find" quad & (f_a)_(a in A) \
    "subject to" quad & f_a <= c_a quad forall a in A \
    & sum_(a in delta^-(v)) f_a = sum_(a in delta^+(v)) f_a quad forall v in V backslash {s, t} \
    & f_a = f_b quad forall (a, b) \
    & sum_(a in delta^-(t)) f_a - sum_(a in delta^+(t)) f_a >= R \
    & f_a in ZZ_(>=0) quad forall a in A
  $.

  _Correctness._ ($arrow.r.double$) Any feasible integral flow already satisfies the capacity, conservation, equality, and sink-demand constraints. ($arrow.l.double$) Any feasible ILP assignment is exactly an integral arc-flow meeting the homologous-pair and requirement conditions.

  _Solution extraction._ Output the arc-flow vector $(f_a)_(a in A)$ in the source arc order.
]

#reduction-rule("IntegralFlowWithMultipliers", "ILP")[
  The source constraints are linear after writing one integer flow variable per arc and enforcing multiplier-scaled conservation at each non-terminal.
][
  _Construction._ Variables: integer $f_a >= 0$ per arc $a = (u -> v)$. The ILP is:
  $
    "find" quad & (f_a)_(a in A) \
    "subject to" quad & f_a <= c_a quad forall a in A \
    & sum_(a in delta^+(v)) f_a = h(v) sum_(a in delta^-(v)) f_a quad forall v in V backslash {s, t} \
    & sum_(a in delta^-(t)) f_a - sum_(a in delta^+(t)) f_a >= R \
    & f_a in ZZ_(>=0) quad forall a in A
  $.

  _Correctness._ ($arrow.r.double$) A valid multiplier flow satisfies these linear equalities and inequalities by definition. ($arrow.l.double$) Any feasible ILP solution gives an integral arc flow whose non-terminal outflow equals the prescribed multiple of its inflow and whose sink inflow meets the requirement.

  _Solution extraction._ Output the arc-flow vector $(f_a)_(a in A)$.
]

#reduction-rule("PathConstrainedNetworkFlow", "ILP")[
  Because flow may use only the prescribed $s$-$t$ paths, it suffices to assign an integer amount to each allowed path and aggregate those loads on every arc.
][
  _Construction._ Let $P_1, dots, P_q$ be the prescribed paths. Variables: integer $f_i >= 0$ for each path $P_i$. The ILP is:
  $
    "find" quad & (f_i)_(i = 1)^q \
    "subject to" quad & sum_(i : a in P_i) f_i <= c_a quad forall a in A \
    & sum_i f_i >= R \
    & f_i in ZZ_(>=0) quad forall i in {1, dots, q}
  $.

  _Correctness._ ($arrow.r.double$) Any valid path-flow assignment respects every arc capacity and delivers at least $R$ units in total. ($arrow.l.double$) Any feasible ILP solution assigns integral flow only to the prescribed paths, and the aggregated arc loads satisfy the network capacities.

  _Solution extraction._ Output the path-flow vector $(f_1, dots, f_q)$.
]

#reduction-rule("DisjointConnectingPaths", "ILP")[
  Route one unit of flow for each terminal pair on an oriented copy of the graph, and forbid internal vertices from carrying more than one commodity.
][
  _Construction._ For terminal pairs $(s_k, t_k)$, variables: binary $f^k_(u,v)$ on each orientation of each edge and integer order variables $h^k_v$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(w) f^k_(s_k,w) - sum_(u) f^k_(u,s_k) = 1 quad forall k \
    & sum_(u) f^k_(u,t_k) - sum_(w) f^k_(t_k,w) = 1 quad forall k \
    & sum_(w) f^k_(v,w) - sum_(u) f^k_(u,v) = 0 quad forall k, v in V backslash {s_k, t_k} \
    & f^k_(u,v) + f^k_(v,u) <= 1 quad forall {u, v} in E, k \
    & sum_k sum_(w in N(v)) f^k_(v,w) <= 1 quad forall "non-terminal" v \
    & h^k_v >= h^k_u + 1 - M (1 - f^k_(u,v)) quad forall k, u -> v \
    & f^k_(u,v) in {0, 1}, h^k_v in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) A family of pairwise internally vertex-disjoint connecting paths orients each path from its source to its sink and satisfies all constraints. ($arrow.l.double$) The conservation, disjointness, and ordering constraints force each commodity to trace one simple path, and different commodities can intersect only at terminals.

  _Solution extraction._ Mark an edge selected in the source config iff some orientation of that edge carries flow for some commodity.
]

#reduction-rule("LengthBoundedDisjointPaths", "ILP")[
  Use one unit-flow commodity for each requested path and add hop variables so every chosen path has at most the source bound $K$ edges.
][
  _Construction._ Variables: binary $f^k_(u,v)$ on each orientation of each edge for each path slot $k$, plus integer hop variables $h^k_v in {0, dots, K}$, where $K$ is the path-length bound and $M = K + 1$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(w) f^k_(s,w) - sum_(u) f^k_(u,s) = 1 quad forall k \
    & sum_(u) f^k_(u,t) - sum_(w) f^k_(t,w) = 1 quad forall k \
    & sum_(w) f^k_(v,w) - sum_(u) f^k_(u,v) = 0 quad forall k, v in V backslash {s, t} \
    & f^k_(u,v) + f^k_(v,u) <= 1 quad forall {u, v} in E, k \
    & sum_k sum_(w in N(v)) f^k_(v,w) <= 1 quad forall v in V backslash {s, t} \
    & h^k_s = 0 quad forall k \
    & h^k_v >= h^k_u + 1 - M (1 - f^k_(u,v)) quad forall k, u -> v \
    & h^k_t <= K quad forall k \
    & f^k_(u,v) in {0, 1}, h^k_v in {0, dots, K}
  $.

  _Correctness._ ($arrow.r.double$) A collection of $J$ internally disjoint $s$-$t$ paths of length at most $K$ yields feasible commodity flows and consistent hop labels. ($arrow.l.double$) The flow and hop constraints force each commodity to be a simple $s$-$t$ path, while the vertex-disjointness inequalities match the source requirement.

  _Solution extraction._ For each path slot $k$, set the source vertex-indicator block to 1 exactly on the vertices incident to the commodity-$k$ path, including $s$ and $t$.
]

#reduction-rule("MixedChinesePostman", "ILP")[
  Choose an orientation for every undirected edge, then add integer traversal variables on the available directed arcs to balance the oriented required multigraph within the length bound.
][
  _Construction._ Let $n = |V|$, let the original directed arcs be $A = {a_0, dots, a_(m-1)}$ with $a_i = (alpha_i, beta_i)$, and let the undirected edges be $E = {e_0, dots, e_(q-1)}$ with $e_k = {u_k, v_k}$. Set $R = m + q$. If $R = 0$, return the empty feasible ILP: the empty walk already has length 0. Otherwise form the available directed-arc list
  $A^* = {b_0, dots, b_(L-1)}$ with $L = m + 2 q$,
  where $b_i = a_i$ for $0 <= i < m$, $b_(m + 2 k) = (u_k, v_k)$, and $b_(m + 2 k + 1) = (v_k, u_k)$.
  Write $b_j = ("tail"_j, "head"_j)$ and let $ell_j$ be the corresponding length. Use `ILP<i32>` with binary variables encoded by bounds $0 <= x <= 1$. Order the variables as
  $(d_0, dots, d_(q-1), g_0, dots, g_(L-1), y_0, dots, y_(L-1), z_0, dots, z_(n-1), rho_0, dots, rho_(n-1), s, b_0, dots, b_(n-1), f_0, dots, f_(L-1), h_0, dots, h_(L-1))$,
  so $d_k$ has index $k$, $g_j$ has index $q + j$, $y_j$ has index $q + L + j$, $z_v$ has index $q + 2 L + v$, $rho_v$ has index $q + 2 L + n + v$, $s$ has index $q + 2 L + 2 n$, $b_v$ has index $q + 2 L + 2 n + 1 + v$, $f_j$ has index $q + 2 L + 3 n + 1 + j$, and $h_j$ has index $q + 3 L + 3 n + 1 + j$. There are $q + 4 L + 3 n + 1$ variables in total.

  The orientation bit $d_k in {0, 1}$ means $d_k = 0$ chooses $u_k -> v_k$ and $d_k = 1$ chooses $v_k -> u_k$. Define the required multiplicity on each available arc explicitly by
  $r_i(d) = 1$ for $0 <= i < m$,
  $r_(m + 2 k)(d) = 1 - d_k$,
  and $r_(m + 2 k + 1)(d) = d_k$.
  Thus the two oriented copies of each undirected edge are already linear in the orientation bit.

  Let $G = R (n - 1)$ and $M_"use" = 1 + G$. The variable $g_j in {0, dots, G}$ counts extra traversals of $b_j$ beyond the required multiplicity, so the total multiplicity of $b_j$ is $r_j(d) + g_j$. The bound $G = R (n - 1)$ is exact for this formulation: any closed walk can be shortcut so that between consecutive required traversals it uses a simple connector path of at most $n - 1$ arcs, and there are exactly $R$ such connector segments in the cyclic order of the required traversals.

  The constraints are:
  $sum_(j : "tail"_j = v) (r_j(d) + g_j) - sum_(j : "head"_j = v) (r_j(d) + g_j) = 0$ for every $v in V$;
  $r_j(d) + g_j <= M_"use" y_j$ and $y_j <= r_j(d) + g_j$ for every $j in {0, dots, L - 1}$, so $y_j = 1$ iff arc $b_j$ is used at least once;
  $y_j <= z_"tail"_j$ and $y_j <= z_"head"_j$ for every $j$, and $z_v <= sum_(j : "tail"_j = v " or " "head"_j = v) y_j$ for every vertex $v$, so $z_v = 1$ iff $v$ is incident to some used arc;
  $s = sum_v z_v$;
  $sum_v rho_v = 1$, $rho_v <= z_v$ for every $v$, and the product linearization $b_v <= s$, $b_v <= n rho_v$, $b_v >= s - n (1 - rho_v)$, $b_v >= 0$ for every $v$, so $b_v = s rho_v$ and therefore the unique root chosen by $rho$ supplies $s - 1$ units of connectivity flow;
  $0 <= f_j <= (n - 1) y_j$ and $0 <= h_j <= (n - 1) y_j$ for every available arc $b_j$; here the exact big-$M$ for arc activation is $n - 1$, because at most one unit is demanded by each non-root active vertex;
  $sum_(j : "tail"_j = v) f_j - sum_(j : "head"_j = v) f_j = b_v - z_v$ for every vertex $v$; and
  $sum_(j : "head"_j = v) h_j - sum_(j : "tail"_j = v) h_j = b_v - z_v$ for every vertex $v$.
  The $f$-flow makes every active vertex reachable from the chosen root, and the $h$-flow makes the root reachable from every active vertex on the same used support.
  Finally impose the length bound
  $sum_(j = 0)^(L - 1) ell_j (r_j(d) + g_j) <= B$.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(j : "tail"_j = v) (r_j(d) + g_j) - sum_(j : "head"_j = v) (r_j(d) + g_j) = 0 quad forall v in V \
    & r_j(d) + g_j <= M_"use" y_j, y_j <= r_j(d) + g_j quad forall j in {0, dots, L - 1} \
    & y_j <= z_"tail"_j, y_j <= z_"head"_j quad forall j \
    & z_v <= sum_(j : "tail"_j = v " or " "head"_j = v) y_j quad forall v in V \
    & s = sum_v z_v; sum_v rho_v = 1; rho_v <= z_v quad forall v in V \
    & "the standard product linearization enforces" b_v = s rho_v quad forall v in V \
    & 0 <= f_j, h_j <= (n - 1) y_j quad forall j in {0, dots, L - 1} \
    & "forward and reverse root-flow conservation hold on the used support" \
    & sum_(j = 0)^(L - 1) ell_j (r_j(d) + g_j) <= B \
    & d_k, y_j, z_v, rho_v in {0, 1}; g_j in {0, dots, G}; f_j, h_j in {0, dots, n - 1}; s, b_v in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) From any feasible mixed-postman tour, set $d_k$ from the direction in which edge $e_k$ is first required, let $g_j$ be the number of extra copies of $b_j$ beyond the required multiplicity, and let $y_j$ mark the positive-support arcs. The tour itself visits exactly the active vertices, so some active vertex can be chosen as the root. Taking one outgoing spanning arborescence and one incoming spanning arborescence of the used Eulerian digraph gives feasible $f$- and $h$-flows. The walk length is exactly $sum_j ell_j (r_j(d) + g_j)$, hence the ILP is feasible.
  ($arrow.l.double$) A feasible ILP solution chooses one direction for every undirected edge, and the balance equations make the directed multigraph with multiplicities $r_j(d) + g_j$ Eulerian. The two root-flow systems imply that the positive-support digraph on the active vertices is strongly connected. Therefore the used multigraph admits an Euler tour, and its total length is exactly the bounded linear form above, so the source instance is a YES-instance.

  _Solution extraction._ Return the orientation bits $d_e$ in the source edge order.
]

#reduction-rule("RuralPostman", "ILP")[
  Use one traversal-multiplicity variable per edge, together with activation and connectivity constraints, to encode an Eulerian connected subgraph covering all required edges.
][
  _Construction._ If $E' = emptyset$, the empty circuit already satisfies the source instance whenever $B >= 0$, so use the empty ILP. Otherwise fix a root vertex $r$ incident to some required edge and let $n = |V|$. Variables: integer $t_e in {0, 1, 2}$ and parity variables $q_v$, binary edge-activation flags $y_e$, binary vertex-activity flags $z_v$, and nonnegative connectivity-flow variables $f_(u,v)$ on both orientations of every edge. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & y_e <= t_e <= 2 y_e quad forall e in E \
    & t_e >= 1 quad forall e in E' \
    & sum_(e : v in e) t_e = 2 q_v quad forall v \
    & y_e <= z_u, y_e <= z_v quad forall e = {u, v} in E \
    & z_v <= sum_(e : v in e) y_e quad forall v in V \
    & f_(u,v) <= (n - 1) y_e, f_(v,u) <= (n - 1) y_e quad forall e = {u, v} in E \
    & sum_(w : {r, w} in E) f_(r,w) - sum_(u : {u, r} in E) f_(u,r) = sum_v z_v - 1 \
    & sum_(u : {u, v} in E) f_(u,v) - sum_(w : {v, w} in E) f_(v,w) = z_v quad forall v in V backslash {r} \
    & sum_e ell_e t_e <= B \
    & y_e, z_v in {0, 1}, t_e in {0, 1, 2}, q_v, f_(u,v) in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any feasible rural-postman circuit uses each edge at most twice, has even degrees, is connected on its positive-support edges, and satisfies the bound. ($arrow.l.double$) A feasible ILP solution defines a connected Eulerian multigraph containing every required edge, hence an Eulerian circuit of total length at most $B$.

  _Solution extraction._ Output the traversal multiplicities $(t_e)_(e in E)$.
]

#reduction-rule("StackerCrane", "ILP")[
  Encode the required-arc order by a one-hot position assignment and charge the shortest connector distance between each consecutive pair of required arcs.
][
  _Construction._ Let the required arcs be $A = {a_0, dots, a_(m-1)}$ with $a_i = ("tail"_i, "head"_i)$. Build the mixed connector graph
  $H = (V, A union {(u, v), (v, u) : {u, v} in E})$,
  where the original required arcs keep their given lengths and each undirected edge contributes both orientations with the same length. Because all lengths are nonnegative, compute the all-pairs connector distances
  $D[u, v] = "dist"_H(u, v)$
  either by running Dijkstra from every source vertex or by Floyd--Warshall on the $n$-vertex graph $H$; this is exactly the graph queried by `mixed_graph_adjacency()` and `shortest_path_length()` in the model. If $D[u, v] = oo$, the pair is impossible and will be forbidden explicitly.

  Use `ILP<bool>`. The binary position variables are $x_(i,p)$ for $i, p in {0, dots, m - 1}$, with index
  $"idx"_x(i, p) = i m + p$.
  The binary McCormick variables are $z_(i,j,p)$ for $i, j, p in {0, dots, m - 1}$, where position $p + 1$ is interpreted cyclically as $(p + 1) mod m$; their indices are
  $"idx"_z(i, j, p) = m^2 + p m^2 + i m + j$.
  There are $m^2 + m^3$ binary variables.

  The constraints are:
  $sum_(p = 0)^(m - 1) x_(i,p) = 1$ for each required arc $i$;
  $sum_(i = 0)^(m - 1) x_(i,p) = 1$ for each position $p$;
  $z_(i,j,p) <= x_(i,p)$, $z_(i,j,p) <= x_(j,(p + 1) mod m)$, and $z_(i,j,p) >= x_(i,p) + x_(j,(p + 1) mod m) - 1$ for all $i, j, p$;
  if $D["head"_i, "tail"_j] = oo$, then either set $z_(i,j,p) = 0$ for all $p$ or, equivalently, impose $x_(i,p) + x_(j,(p + 1) mod m) <= 1$ for all $p$;
  and finally
  $sum_(i = 0)^(m - 1) ell_i + sum_(p = 0)^(m - 1) sum_(i = 0)^(m - 1) sum_(j = 0)^(m - 1) D["head"_i, "tail"_j] z_(i,j,p) <= B$.
  The first term is the total length of the required traversals, and the second term charges exactly one connector distance for each consecutive pair in the cyclic order.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(p = 0)^(m - 1) x_(i,p) = 1 quad forall i in {0, dots, m - 1} \
    & sum_(i = 0)^(m - 1) x_(i,p) = 1 quad forall p in {0, dots, m - 1} \
    & z_(i,j,p) <= x_(i,p), z_(i,j,p) <= x_(j,(p + 1) mod m) quad forall i, j, p \
    & z_(i,j,p) >= x_(i,p) + x_(j,(p + 1) mod m) - 1 quad forall i, j, p \
    & z_(i,j,p) = 0 quad "whenever" D["head"_i, "tail"_j] = oo \
    & sum_(i = 0)^(m - 1) ell_i + sum_(p = 0)^(m - 1) sum_(i = 0)^(m - 1) sum_(j = 0)^(m - 1) D["head"_i, "tail"_j] z_(i,j,p) <= B \
    & x_(i,p), z_(i,j,p) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any feasible Stacker Crane permutation determines a one-hot assignment and consecutive-pair indicators whose connector costs equal the route length. ($arrow.l.double$) Any feasible ILP solution yields a permutation of the required arcs, and the linearized connector term is exactly the sum of shortest paths between consecutive arcs.

  _Solution extraction._ Decode the permutation by taking, for each position $p$, the unique arc $a$ with $x_(a,p) = 1$.
]

#reduction-rule("SteinerTreeInGraphs", "ILP")[
  Select edges and certify terminal connectivity by sending one unit of flow from a root terminal to every other terminal through the selected subgraph.
][
  _Construction._ Fix a root terminal $r in R$. Variables: binary $y_(u,v)$ for each undirected edge $\{u,v\}$ and nonnegative flow variables $f^t_(u,v)$ on each directed edge orientation for every terminal $t in R backslash {r}$. The ILP is:
  $
    min quad & sum_({u,v} in E) w_(u,v) y_(u,v) \
    "subject to" quad & sum_(u) f^t_(u,v) - sum_(w) f^t_(v,w) = b_(t,v) quad forall t in R backslash {r}, v in V \
    & f^t_(u,v) <= y_(u,v) quad forall {u, v} in E, t in R backslash {r} \
    & f^t_(v,u) <= y_(u,v) quad forall {u, v} in E, t in R backslash {r} \
    & y_(u,v) in {0, 1}, f^t_(u,v) in ZZ_(>=0)
  $,
  where $b_(t,v) = -1$ if $v = r$, $b_(t,v) = 1$ if $v = t$, and $b_(t,v) = 0$ otherwise.

  _Correctness._ ($arrow.r.double$) A Steiner tree supports a unit flow from the root to every other terminal using exactly its selected edges, with the same total weight. ($arrow.l.double$) Any feasible ILP solution selects a connected subgraph spanning all terminals, and with nonnegative edge weights an optimum solution is a minimum-weight Steiner tree.

  _Solution extraction._ Output the binary edge-selection vector $(y_e)_(e in E)$.
]

// Scheduling

#reduction-rule("FlowShopScheduling", "ILP")[
  Order the jobs with pairwise precedence bits and completion-time variables on every machine; the deadline becomes a makespan bound.
][
  _Construction._ Let $q in {1, dots, m}$ index the machines, let $p_(j,q) = ell(t_q [j])$ be the processing time of job $j$ on machine $q$, and let $M = D + max_(j, q) p_(j,q)$. Variables: binary $y_(i,j)$ with $y_(i,j) = 1$ iff job $i$ precedes job $j$, and integer completion times $C_(j,q)$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & y_(i,j) + y_(j,i) = 1 quad forall i != j \
    & C_(j,1) >= p_(j,1) quad forall j \
    & C_(j,q + 1) >= C_(j,q) + p_(j,q + 1) quad forall j, q in {1, dots, m - 1} \
    & C_(j,q) >= C_(i,q) + p_(j,q) - M (1 - y_(i,j)) quad forall i != j, q in {1, dots, m} \
    & C_(j,m) <= D quad forall j \
    & y_(i,j) in {0, 1}, C_(j,q) in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any feasible flow-shop permutation induces a total order and completion times satisfying the machine and deadline constraints. ($arrow.l.double$) Any feasible ILP solution defines one common order of the jobs on all machines, and the resulting schedule completes by the deadline.

  _Solution extraction._ Sort the jobs by their final-machine completion times $C_(j,m)$ and convert that permutation to Lehmer code.
]

#reduction-rule("OpenShopScheduling", "ILP")[
  Binary ordering variables and integer start times encode the disjunctive non-overlap constraints for both machines and jobs; the makespan is the minimized objective.
][
  _Construction._ Let $M = sum_(j,i) p(j,i)$ be the big-$M$ constant (an upper bound on the makespan). For each pair $j < k$ and each machine $i$, let $x_{j k i} in {0,1}$ with $x_{j k i} = 1$ iff job $j$ precedes job $k$ on machine $i$. For each job $j$ and pair of machines $i < i'$, let $y_{j i i'} in {0,1}$ with $y_{j i i'} = 1$ iff machine $i$ is processed before machine $i'$ for job $j$. Let $s_{j,i} in ZZ_{>=0}$ be the start time of job $j$ on machine $i$, and $C$ be the integer makespan variable. The ILP is:
  $
    min quad & C \
    "subject to" quad
    & s_(k,i) - s_(j,i) - M x_(j k i) >= p(j, i) - M quad forall j < k, i \
    & s_(j,i) - s_(k,i) + M x_(j k i) >= p(k, i) quad forall j < k, i \
    & s_(j,i') - s_(j,i) - M y_(j i i') >= p(j, i) - M quad forall j, i < i' \
    & s_(j,i) - s_(j,i') + M y_(j i i') >= p(j, i') quad forall j, i < i' \
    & C - s_(j,i) >= p(j, i) quad forall j, i \
    & x_(j k i), y_(j i i') in {0,1},; s_(j,i), C in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any feasible open-shop schedule with the given permutations $sigma_i$ induces valid ordering bits $x_{j k i}$ and $y_{j i i'}$ and start times satisfying all non-overlap constraints. ($arrow.l.double$) Any feasible ILP solution defines non-overlapping start times for all tasks, respecting both machine and job constraints.

  _Solution extraction._ For each machine $i$, sort jobs by their ILP start times $s_{j,i}$ to recover the per-machine permutation; output the concatenation of these $m$ direct-index permutations.
]

#reduction-rule("MinimumTardinessSequencing", "ILP")[
  A position-assignment ILP captures the permutation, the precedence constraints, and a binary tardy indicator for each unit-length task.
][
  _Construction._ Variables: binary $x_(j,p)$ placing task $j$ in position $p in {0, dots, n-1}$ and binary tardy indicators $u_j$, where $M = n$. The ILP is:
  $
    min quad & sum_j u_j \
    "subject to" quad & sum_p x_(j,p) = 1 quad forall j \
    & sum_j x_(j,p) = 1 quad forall p \
    & sum_p p x_(i,p) + 1 <= sum_p p x_(j,p) quad "for each precedence" (i, j) \
    & sum_p (p + 1) x_(j,p) - d_j <= M u_j quad forall j \
    & x_(j,p), u_j in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any feasible schedule gives a permutation and tardy bits with objective equal to the number of tardy tasks. ($arrow.l.double$) Any feasible ILP assignment decodes to a precedence-respecting permutation, and each $u_j$ is forced to record whether task $j$ misses its deadline.

  _Solution extraction._ Decode the permutation from $x_(j,p)$ and encode it as Lehmer code.
]

#reduction-rule("ResourceConstrainedScheduling", "ILP")[
  The source witness is already a time-slot assignment, so a standard time-indexed ILP suffices.
][
  _Construction._ Variables: binary $x_(j,t)$ with $x_(j,t) = 1$ iff task $j$ is run in slot $t in {0, dots, D - 1}$, where $r_(j,q) = R_q(t_j)$ denotes the amount of resource $q$ consumed by task $j$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_t x_(j,t) = 1 quad forall j \
    & sum_j x_(j,t) <= m quad forall t \
    & sum_j r_(j,q) x_(j,t) <= B_q quad forall q, t \
    & x_(j,t) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any feasible schedule chooses one slot per task while respecting processor and resource capacities in every period. ($arrow.l.double$) Any feasible ILP solution directly gives such a slot assignment.

  _Solution extraction._ Task $j$ is assigned to the unique slot $t$ with $x_(j,t) = 1$.
]

#reduction-rule("SequencingToMinimizeMaximumCumulativeCost", "ILP")[
  Assign each task to one position in the permutation and bound the running cumulative cost at every prefix.
][
  _Construction._ Variables: binary $x_(j,p)$ with $x_(j,p) = 1$ iff task $j$ is scheduled in position $p$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_p x_(j,p) = 1 quad forall j \
    & sum_j x_(j,p) = 1 quad forall p \
    & sum_p p x_(i,p) + 1 <= sum_p p x_(j,p) quad "for each precedence" (i, j) \
    & sum_j sum_(p in {0, dots, q}) c_j x_(j,p) <= K quad forall q \
    & x_(j,p) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A feasible permutation satisfies the precedence constraints and keeps every prefix sum at most $K$. ($arrow.l.double$) Any feasible ILP assignment is a permutation whose cumulative cost after each prefix is exactly the linear expression being bounded.

  _Solution extraction._ Decode the position assignment and convert the resulting permutation to Lehmer code.
]

#reduction-rule("SequencingToMinimizeTardyTaskWeight", "ILP")[
  Place each task in exactly one schedule position with a binary tardy indicator forced on whenever the completion time at that position exceeds the task's deadline.
][
  _Construction._ Variables: binary $x_(j,p)$ with $x_(j,p) = 1$ iff task $j$ occupies position $p$, and binary tardy indicator $u_j$. Let $M = sum_j ell_j$. The ILP is:
  $
    "minimize" quad & sum_j w_j u_j \
    "subject to" quad & sum_p x_(j,p) = 1 quad forall j \
    & sum_j x_(j,p) = 1 quad forall p \
    & M x_(j,p) + sum_(p' < p) sum_(j') ell_(j') x_(j',p') - M u_j <= d_j - ell_j + M quad forall j, p \
    & x_(j,p) in {0, 1}, u_j in {0, 1}
  $.
  The third family of constraints enforces: if task $j$ is at position $p$ (so $x_(j,p) = 1$), then its completion time $ell_j + sum_(p' < p) sum_(j') ell_(j') x_(j',p')$ exceeds $d_j$ only when $u_j = 1$.

  _Correctness._ ($arrow.r.double$) Any schedule induces completion times; for each tardy task the big-$M$ constraint forces $u_j = 1$, so the objective counts exactly the total tardy weight. ($arrow.l.double$) Any feasible ILP assignment is a valid permutation (by the assignment constraints) and the tardy indicators agree with the actual completion times.

  _Solution extraction._ Read the unique position $p$ with $x_(j,p) = 1$ for each task $j$ to recover the schedule permutation.
]

#reduction-rule("SequencingWithDeadlinesAndSetUpTimes", "ILP")[
  Assign tasks to positions with switch-detection auxiliaries that gate per-compiler setup costs into the deadline constraints.
][
  _Construction._ Let $n$ be the number of tasks. Variables: binary $x_(j,p)$ with $x_(j,p) = 1$ iff task $j$ occupies position $p$; binary $"sw"_p$ for $p >= 1$ indicating a compiler switch before position $p$; binary $a_(j,p) = x_(j,p) dot "sw"_p$ (linearised product). Let $M = sum_j ell_j + max_c s(c) dot (n-1)$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_p x_(j,p) = 1 quad forall j \
    & sum_j x_(j,p) = 1 quad forall p \
    & x_(j,p) + x_(j',p-1) - "sw"_p <= 1 quad forall p >= 1, j, j' : k(j) != k(j') \
    & a_(j,p) <= x_(j,p), quad a_(j,p) <= "sw"_p, quad x_(j,p) + "sw"_p - a_(j,p) <= 1 quad forall j, p >= 1 \
    & M x_(j,p) + sum_(p' < p) sum_(j') ell_(j') x_(j',p') + sum_(p'=1)^(p) sum_(j') s(k(j')) a_(j',p') <= d_j - ell_j + M quad forall j, p \
    & x_(j,p), "sw"_p, a_(j,p) in {0, 1}
  $.
  The switch-detection row forces $"sw"_p = 1$ whenever the tasks at positions $p-1$ and $p$ use different compilers. The $a_(j,p)$ linearisation then routes the correct per-compiler setup time into the completion-time bound for each position.

  _Correctness._ ($arrow.r.double$) Any feasible schedule assigns each task to a position; the switch indicator equals one exactly when consecutive compilers differ, and the deadline constraint is satisfied by hypothesis. ($arrow.l.double$) Any feasible ILP solution is a valid permutation and the deadline bound ensures each task finishes on time accounting for all setup penalties.

  _Solution extraction._ Read the unique position $p$ with $x_(j,p) = 1$ for each task $j$ to recover the schedule permutation.
]

#reduction-rule("SequencingToMinimizeWeightedTardiness", "ILP")[
  Encode the single-machine order with pairwise precedence bits and completion times, then linearize the weighted tardiness bound with nonnegative tardiness variables.
][
  _Construction._ Variables: binary $y_(i,j)$ with $y_(i,j) = 1$ iff job $i$ precedes job $j$, integer completion times $C_j$, and nonnegative tardiness variables $T_j$, where $M = sum_j ell_j$ is a valid schedule-horizon bound. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & y_(i,j) + y_(j,i) = 1 quad forall i != j \
    & C_j >= ell_j quad forall j \
    & C_j >= C_i + ell_j - M (1 - y_(i,j)) quad forall i != j \
    & T_j >= C_j - d_j quad forall j \
    & T_j >= 0 quad forall j \
    & sum_j w_j T_j <= K \
    & y_(i,j) in {0, 1}, C_j in ZZ_(>=0), T_j in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any job order induces completion times and tardiness values satisfying the bound exactly when the source instance is feasible. ($arrow.l.double$) Any feasible ILP solution yields a single-machine order whose weighted tardiness equals the encoded linear objective term.

  _Solution extraction._ Sort the jobs by $C_j$ and encode that permutation as Lehmer code.
]

#reduction-rule("SequencingWithReleaseTimesAndDeadlines", "ILP")[
  A time-indexed formulation captures the admissible start window of each task and forbids overlap on the single machine.
][
  _Construction._ Variables: binary $x_(j,t)$ with $x_(j,t) = 1$ iff task $j$ starts at time $t$, where $p_j = ell(t_j)$ is the processing time (length) of task $j$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(t = r_j)^(d_j - p_j) x_(j,t) = 1 quad forall j \
    & sum_(j, t : t <= tau < t + p_j) x_(j,t) <= 1 quad forall tau \
    & x_(j,t) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any feasible non-preemptive schedule chooses one valid start time per task and never overlaps two active jobs. ($arrow.l.double$) Any feasible ILP solution gives exactly such a start-time assignment, so executing the jobs in increasing start order solves the source instance.

  _Solution extraction._ Read each task's chosen start time, sort the tasks by that order, and encode the resulting permutation as Lehmer code.
]

#reduction-rule("TimetableDesign", "ILP")[
  The source witness is a binary craftsman-task-period incidence table, and all feasibility conditions are already linear.
][
  _Construction._ Variables: binary $x_(c,t,h)$ with $x_(c,t,h) = 1$ iff craftsman $c$ works on task $t$ in period $h$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & x_(c,t,h) = 0 quad "whenever either side is unavailable" \
    & sum_t x_(c,t,h) <= 1 quad forall c, h \
    & sum_c x_(c,t,h) <= 1 quad forall t, h \
    & sum_h x_(c,t,h) = r_(c,t) quad forall c, t \
    & x_(c,t,h) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any valid timetable satisfies availability, exclusivity, and exact requirement counts. ($arrow.l.double$) Any feasible ILP solution is exactly such a timetable because the variable layout matches the source configuration.

  _Solution extraction._ Output the flattened binary array $(x_(c,t,h))$ in source order.
]

// Position/Assignment

#reduction-rule("HamiltonianPath", "ILP")[
  Place each vertex in exactly one path position and use auxiliary variables for consecutive pairs so only graph edges may appear between adjacent positions.
][
  _Construction._ Variables: binary $x_(v,p)$ with $x_(v,p) = 1$ iff vertex $v$ is placed at position $p$, and binary $z_((u,v),p)$ linearizing $x_(u,p) x_(v,p+1)$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_p x_(v,p) = 1 quad forall v \
    & sum_v x_(v,p) = 1 quad forall p \
    & z_((u,v),p) <= x_(u,p) quad forall (u, v), p \
    & z_((u,v),p) <= x_(v,p+1) quad forall (u, v), p \
    & z_((u,v),p) >= x_(u,p) + x_(v,p+1) - 1 quad forall (u, v), p \
    & sum_((u,v) in E) z_((u,v),p) = 1 quad forall p \
    & x_(v,p), z_((u,v),p) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A Hamiltonian path defines a permutation of the vertices and therefore a feasible assignment matrix with one admissible graph edge between every consecutive pair. ($arrow.l.double$) Any feasible ILP solution is a vertex permutation whose consecutive pairs are graph edges, hence a Hamiltonian path.

  _Solution extraction._ For each position $p$, output the unique vertex $v$ with $x_(v,p) = 1$.
]

#reduction-rule("DirectedHamiltonianPath", "ILP")[
  Assign each vertex to exactly one path position and forbid non-arc pairs at consecutive positions.
][
  _Construction._ Variables: binary $x_(v,k)$ with $x_(v,k) = 1$ iff vertex $v$ is placed at position $k$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_k x_(v,k) = 1 quad forall v \
    & sum_v x_(v,k) = 1 quad forall k \
    & x_(v,k) + x_(w,k+1) <= 1 quad forall k, (v, w) in.not A \
    & x_(v,k) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A directed Hamiltonian path yields a permutation where every consecutive pair is a directed arc; the arc-exclusion constraints are satisfied by definition. ($arrow.l.double$) Any feasible ILP solution defines a vertex permutation whose consecutive pairs are all directed arcs, hence a directed Hamiltonian path.

  _Solution extraction._ For each position $k$, decode the unique vertex $v$ with $x_(v,k) = 1$ to recover the permutation; convert to Lehmer code for the source configuration.
]

#reduction-rule("BottleneckTravelingSalesman", "ILP")[
  Use a cyclic position assignment for the tour and a bottleneck variable that dominates the weight of every chosen tour edge.
][
  _Construction._ Variables: binary $x_(v,p)$ for city-position assignment, binary $z_((u,v),p)$ for consecutive tour edges, and integer bottleneck variable $b$. The ILP is:
  $
    min quad & b \
    "subject to" quad & sum_p x_(v,p) = 1 quad forall v \
    & sum_v x_(v,p) = 1 quad forall p \
    & z_((u,v),p) <= x_(u,p) quad forall (u, v), p \
    & z_((u,v),p) <= x_(v,(p+1) mod n) quad forall (u, v), p \
    & z_((u,v),p) >= x_(u,p) + x_(v,(p+1) mod n) - 1 quad forall (u, v), p \
    & sum_((u,v) in E) z_((u,v),p) = 1 quad forall p \
    & b >= w_(u,v) z_((u,v),p) quad forall (u, v), p \
    & x_(v,p), z_((u,v),p) in {0, 1}, b in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any Hamiltonian tour yields a feasible assignment and sets $b$ to the maximum selected edge weight. ($arrow.l.double$) Any feasible ILP solution encodes a Hamiltonian cycle, and the minimax constraints force $b$ to equal its bottleneck edge weight.

  _Solution extraction._ Mark an edge selected in the source config iff it appears between two consecutive positions in the decoded cycle.
]

#let hc_lc = load-example("HamiltonianCircuit", "LongestCircuit")
#let hc_lc_sol = hc_lc.solutions.at(0)
#let hc_lc_n = graph-num-vertices(hc_lc.source.instance)
#let hc_lc_source_edges = hc_lc.source.instance.graph.edges
#let hc_lc_target_edges = hc_lc.target.instance.graph.edges
#let hc_lc_target_weights = hc_lc.target.instance.edge_lengths
#let hc_lc_selected_edges = hc_lc_target_edges.enumerate().filter(((i, _)) => hc_lc_sol.target_config.at(i) == 1).map(((i, e)) => (e.at(0), e.at(1)))
#reduction-rule("HamiltonianCircuit", "LongestCircuit",
  example: true,
  example-caption: [Cycle graph on $#hc_lc_n$ vertices with unit edge lengths],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(hc_lc.source) + " -o hc.json",
      "pred reduce hc.json --to " + target-spec(hc_lc) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate hc.json --config " + hc_lc_sol.source_config.map(str).join(","),
    )

    *Step 1 -- Start from the source graph.* The canonical source fixture is the cycle on vertices ${0, 1, dots, #(hc_lc_n - 1)}$ with $#hc_lc_source_edges.len()$ edges. The stored Hamiltonian-circuit witness is the permutation $[#hc_lc_sol.source_config.map(str).join(", ")]$.\

    *Step 2 -- Assign unit edge lengths.* The target keeps the same $#hc_lc_n$ vertices and $#hc_lc_target_edges.len()$ edges. Every edge receives length $1$, so the edge-length vector is $[#hc_lc_target_weights.map(str).join(", ")]$.\

    *Step 3 -- Verify the canonical witness.* The stored target configuration $[#hc_lc_sol.target_config.map(str).join(", ")]$ selects the edges #hc_lc_selected_edges.map(e => $(#e.at(0), #e.at(1))$).join(", "). The total circuit length is $#hc_lc_selected_edges.len() times 1 = #hc_lc_n = n$, confirming a Hamiltonian circuit. Traversing the selected edges recovers the vertex permutation $[#hc_lc_sol.source_config.map(str).join(", ")]$.\

    *Multiplicity:* The fixture stores one canonical witness. For the $#hc_lc_n$-cycle there are $#hc_lc_n times 2 = #(hc_lc_n * 2)$ directed Hamiltonian circuits (choice of start vertex and direction), but they all select the same undirected edge set.
  ],
)[
  @garey1979 This $O(m)$ reduction copies the graph unchanged and assigns unit weight to every edge ($n$ target vertices, $m$ target edges). A Hamiltonian circuit exists iff the optimal circuit length equals $n$.
][
  _Construction._ Given a Hamiltonian Circuit instance $G = (V, E)$ with $n = |V|$ and $m = |E|$, construct a Longest Circuit instance on the same graph $G' = G$ with edge lengths $l(e) = 1$ for every $e in E$.

  _Correctness._ ($arrow.r.double$) If $G$ has a Hamiltonian circuit $v_0, v_1, dots, v_(n-1), v_0$, then this circuit uses $n$ edges each of length 1, giving total length $n$. Since a simple circuit on $n$ vertices can use at most $n$ edges, this is optimal. ($arrow.l.double$) If the longest circuit in $G'$ has length $n$, it uses $n$ unit-weight edges and therefore visits $n$ distinct vertices, i.e., every vertex exactly once. This circuit is therefore a Hamiltonian circuit in $G$.

  _Solution extraction._ Read the selected target edges, traverse the unique degree-2 cycle they form, and return the resulting vertex permutation as the source Hamiltonian-circuit witness.
]

#reduction-rule("LongestCircuit", "ILP")[
  A direct cycle-selection ILP uses binary edge variables, degree constraints, and a connectivity witness to force exactly one simple circuit of length at least the bound.
][
  _Construction._ Variables: binary $y_e$ for edges, binary $s_v$ indicating whether vertex $v$ lies on the circuit, and root-flow variables on selected edges. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(e : v in e) y_e = 2 s_v quad forall v \
    & sum_e y_e >= 3 \
    & sum_e l_e y_e >= K \
    & "root-flow connectivity constraints hold on the selected edges" \
    & y_e, s_v in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A simple circuit has degree 2 at each used vertex, is connected, and meets the length bound $K$. ($arrow.l.double$) The degree and connectivity constraints force the selected edges to form exactly one simple circuit, and the final inequality enforces the required total length.

  _Solution extraction._ Output the binary edge-selection vector $(y_e)_(e in E)$.
]

#reduction-rule("QuadraticAssignment", "ILP")[
  Assign each facility to exactly one location, enforce injectivity, and linearize every quadratic cost term with McCormick products.
][
  _Construction._ Variables: binary $x_(i,p)$ with $x_(i,p) = 1$ iff facility $i$ is placed at location $p$, and binary $z_((i,p),(j,q))$ for the products $x_(i,p) x_(j,q)$. The ILP is:
  $
    min quad & sum_(i != j) sum_(p,q) c_(i,j) d_(p,q) z_((i,p),(j,q)) \
    "subject to" quad & sum_p x_(i,p) = 1 quad forall i \
    & sum_i x_(i,p) <= 1 quad forall p \
    & z_((i,p),(j,q)) <= x_(i,p) quad forall i, p, j, q \
    & z_((i,p),(j,q)) <= x_(j,q) quad forall i, p, j, q \
    & z_((i,p),(j,q)) >= x_(i,p) + x_(j,q) - 1 quad forall i, p, j, q \
    & x_(i,p), z_((i,p),(j,q)) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any injective facility placement gives a feasible ILP assignment with exactly the same quadratic cost. ($arrow.l.double$) Any feasible ILP solution decodes to an injective facility-to-location map, and the linearized objective equals the source objective term by term.

  _Solution extraction._ For each facility $i$, output the unique location $p$ with $x_(i,p) = 1$.
]

#reduction-rule("OptimalLinearArrangement", "ILP")[
  Assign each vertex to one position and use absolute-value auxiliaries to measure the length of every edge in the arrangement.
][
  _Construction._ Variables: binary $x_(v,p)$ with $x_(v,p) = 1$ iff vertex $v$ gets position $p$, integer position variables $p_v = sum_p p x_(v,p)$, and nonnegative $z_(u,v)$ per edge $\{u,v\}$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_p x_(v,p) = 1 quad forall v \
    & sum_v x_(v,p) = 1 quad forall p \
    & z_(u,v) >= p_u - p_v quad forall {u, v} in E \
    & z_(u,v) >= p_v - p_u quad forall {u, v} in E \
    & sum_({u,v} in E) z_(u,v) <= K \
    & x_(v,p) in {0, 1}, z_(u,v) in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any valid linear arrangement satisfies the permutation constraints and gives edge lengths $|p_u - p_v|$ within the bound. ($arrow.l.double$) Any feasible ILP solution is a bijection from vertices to positions, and the auxiliary variables exactly upper-bound the edge lengths, so the total arrangement cost is at most $K$.

  _Solution extraction._ For each vertex $v$, output its decoded position $p_v$.
]

#reduction-rule("SubgraphIsomorphism", "ILP")[
  Choose an injective image of every pattern vertex in the host graph and forbid any mapped pattern edge from landing on a host non-edge.
][
  _Construction._ Variables: binary $x_(v,u)$ with $x_(v,u) = 1$ iff pattern vertex $v$ maps to host vertex $u$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_u x_(v,u) = 1 quad forall v \
    & sum_v x_(v,u) <= 1 quad forall u \
    & x_(v,u) + x_(w,u') <= 1 quad forall {v, w} in E_"pat", {u, u'} in.not E_"host" \
    & x_(v,u') + x_(w,u) <= 1 quad forall {v, w} in E_"pat", {u, u'} in.not E_"host" \
    & x_(v,u) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any injective edge-preserving embedding satisfies the assignment and non-edge constraints. ($arrow.l.double$) Any feasible ILP solution is an injective vertex map, and the non-edge inequalities ensure every pattern edge is sent to a host edge.

  _Solution extraction._ For each pattern vertex $v$, output the unique host vertex $u$ with $x_(v,u) = 1$.
]

// Graph structure

#reduction-rule("AcyclicPartition", "ILP")[
  Assign every vertex to one partition class, bound the weight and crossing cost of those classes, and impose a topological order on the quotient digraph.
][
  _Construction._ Let $n = |V|$ and let the directed arcs be $A = {a_0, dots, a_(m-1)}$ with $a_t = (u_t -> v_t)$. The source witness already allows every vertex to choose one label in ${0, dots, n - 1}$, so the ILP uses exactly the same label range. Use `ILP<i32>` with variable order
  $(x_(v,c))_(v,c), (s_(t,c))_(t,c), (y_t)_t, (o_c)_c, (p_v)_v$.
  The indices are
  $"idx"_x(v,c) = v n + c$,
  $"idx"_s(t,c) = n^2 + t n + c$,
  $"idx"_y(t) = n^2 + m n + t$,
  $"idx"_o(c) = n^2 + m n + m + c$,
  and $ "idx"_p(v) = n^2 + m n + m + n + v$.
  There are $n^2 + m n + m + 2 n$ variables.

  Here $x_(v,c) in {0, 1}$ means vertex $v$ is assigned to class $c$, $s_(t,c) in {0, 1}$ means both endpoints of arc $a_t$ lie in class $c$, $y_t in {0, 1}$ marks that arc $a_t$ crosses between two different classes, $o_c in {0, dots, n - 1}$ is the order assigned to class $c$, and $p_v in {0, dots, n - 1}$ copies the order of the class chosen by vertex $v$.

  The constraints are:
  $sum_(c = 0)^(n - 1) x_(v,c) = 1$ for every vertex $v$;
  $sum_v w_v x_(v,c) <= B$ for every class $c$;
  $s_(t,c) <= x_(u_t,c)$, $s_(t,c) <= x_(v_t,c)$, and $s_(t,c) >= x_(u_t,c) + x_(v_t,c) - 1$ for every arc $a_t$ and class $c$;
  $y_t + sum_(c = 0)^(n - 1) s_(t,c) = 1$ for every arc $a_t$, so $y_t = 1$ exactly for crossing arcs;
  $sum_(t = 0)^(m - 1) "cost"(a_t) y_t <= K$;
  $0 <= o_c <= n - 1$ and $0 <= p_v <= n - 1$ for all classes $c$ and vertices $v$;
  $p_v - o_c <= (n - 1) (1 - x_(v,c))$ and $o_c - p_v <= (n - 1) (1 - x_(v,c))$ for all $v, c$, so $p_v = o_c$ whenever $x_(v,c) = 1$;
  and for every arc $a_t = (u_t -> v_t)$,
  $p_(v_t) - p_(u_t) >= 1 - n sum_(c = 0)^(n - 1) s_(t,c)$.
  The exact big-$M$ here is $M = n$: if $u_t$ and $v_t$ lie in the same class, then $sum_c s_(t,c) = 1$ and the right-hand side is $1 - n = -(n - 1)$, which is precisely the smallest possible difference between two order variables in ${0, dots, n - 1}$. If the arc crosses between two distinct classes, then $sum_c s_(t,c) = 0$ and the inequality becomes $p_(v_t) - p_(u_t) >= 1$. For the realized classes $c$ and $d$ of the endpoints, this is exactly the requested form
  $o_d - o_c >= 1 - M sum_h s_(t,h)$.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(c = 0)^(n - 1) x_(v,c) = 1 quad forall v in V \
    & sum_v w_v x_(v,c) <= B quad forall c in {0, dots, n - 1} \
    & s_(t,c) <= x_(u_t,c), s_(t,c) <= x_(v_t,c) quad forall t, c \
    & s_(t,c) >= x_(u_t,c) + x_(v_t,c) - 1 quad forall t, c \
    & y_t + sum_(c = 0)^(n - 1) s_(t,c) = 1 quad forall t in {0, dots, m - 1} \
    & sum_(t = 0)^(m - 1) "cost"(a_t) y_t <= K \
    & p_v - o_c <= (n - 1) (1 - x_(v,c)), o_c - p_v <= (n - 1) (1 - x_(v,c)) quad forall v, c \
    & p_(v_t) - p_(u_t) >= 1 - n sum_(c = 0)^(n - 1) s_(t,c) quad forall t in {0, dots, m - 1} \
    & x_(v,c), s_(t,c), y_t in {0, 1}; o_c, p_v in {0, dots, n - 1}
  $.

  _Correctness._ ($arrow.r.double$) Any valid acyclic partition gives a class assignment whose quotient arcs respect some topological ordering, with the same class weights and crossing cost. ($arrow.l.double$) Any feasible ILP solution partitions the vertices, keeps every class within the weight bound, charges exactly the inter-class arcs, and the order variables force the quotient digraph to be acyclic.

  _Solution extraction._ For each vertex $v$, output the unique class label $c$ with $x_(v,c) = 1$.
]

#reduction-rule("BalancedCompleteBipartiteSubgraph", "ILP")[
  Choose exactly $k$ vertices on each side of the bipartite graph and forbid any selected left-right pair that is not an edge.
][
  _Construction._ Let $L$ and $R$ be the bipartition. Variables: binary $x_l$ for $l in L$ and $y_r$ for $r in R$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(l in L) x_l = k \
    & sum_(r in R) y_r = k \
    & x_l + y_r <= 1 quad forall (l, r) in.not E \
    & x_l, y_r in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A balanced complete bipartite subgraph of size $k + k$ satisfies the cardinality constraints and has no selected non-edge pair. ($arrow.l.double$) Any feasible ILP solution selects $k$ left vertices and $k$ right vertices with every cross-pair present, hence a balanced biclique.

  _Solution extraction._ Output the concatenated left/right binary selection vector.
]

#reduction-rule("BicliqueCover", "ILP")[
  Use $k$ candidate bicliques, assign vertices to any of them, force every graph edge to be covered by some common biclique, and minimize the total membership size.
][
  _Construction._ Variables: binary $x_(l,b)$ for left vertices, binary $y_(r,b)$ for right vertices, and binary $z_((l,r),b)$ linearizing $x_(l,b) y_(r,b)$. The ILP is:
  $
    min quad & sum_(l,b) x_(l,b) + sum_(r,b) y_(r,b) \
    "subject to" quad & z_((l,r),b) <= x_(l,b) quad forall l, r, b \
    & z_((l,r),b) <= y_(r,b) quad forall l, r, b \
    & z_((l,r),b) >= x_(l,b) + y_(r,b) - 1 quad forall l, r, b \
    & sum_b z_((l,r),b) >= 1 quad forall (l, r) in E \
    & x_(l,b) + y_(r,b) <= 1 quad forall (l, r) in.not E, b \
    & x_(l,b), y_(r,b), z_((l,r),b) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any valid $k$-biclique cover assigns each covered edge to a biclique containing both endpoints, with objective equal to the total biclique size. ($arrow.l.double$) Any feasible ILP solution defines $k$ complete bipartite subgraphs whose union covers every edge, and the objective is exactly the source objective.

  _Solution extraction._ Output the flattened vertex-by-biclique membership bits and discard the coverage auxiliaries.
]

#reduction-rule("BiconnectivityAugmentation", "ILP")[
  Select candidate edges under the budget and, for every deleted vertex, certify that the remaining augmented graph stays connected by a flow witness.
][
  _Construction._ Let the base graph edges be $E = {e_0, dots, e_(m-1)}$ with $e_i = {u_i, v_i}$, and let the candidate edges be $F = {f_0, dots, f_(p-1)}$ with $f_j = {s_j, t_j}$. If $n = |V| <= 1$, return the empty feasible ILP, since every 0- or 1-vertex graph is already biconnected in the model. Otherwise fix, for each deleted vertex $q$, the surviving root
  $r_q = 0$ if $q != 0$, and $r_0 = 1$.
  This choice is explicit and valid because $n >= 2$.

  Use `ILP<i32>`. The candidate-selection bits are $y_j in {0, 1}$ with index $j$. For the connectivity witnesses, allocate the full $(q, t)$ commodity grid with $q, t in {0, dots, n - 1}$, even though the commodities with $t = q$ or $t = r_q$ will be pinned to 0. For each base edge $e_i$ and orientation flag $eta in {0, 1}$, let $eta = 0$ mean $u_i -> v_i$ and $eta = 1$ mean $v_i -> u_i$; define binary flow variables $f^(q,t)_(i,eta)$ with index
  $p + (((q n + t) m + i) 2 + eta)$.
  For each candidate edge $f_j$ and orientation flag $eta in {0, 1}$, let $eta = 0$ mean $s_j -> t_j$ and $eta = 1$ mean $t_j -> s_j$; define binary flow variables $g^(q,t)_(j,eta)$ with index
  $p + 2 m n^2 + (((q n + t) p + j) 2 + eta)$.
  There are $p + 2 n^2 (m + p)$ variables in total.

  The constraints are:
  $sum_(j = 0)^(p - 1) w_j y_j <= B$;
  for every deleted vertex $q$ and target $t$, if $t = q$ or $t = r_q$, set all $f^(q,t)_(i,eta)$ and $g^(q,t)_(j,eta)$ equal to 0;
  if the deleted vertex $q$ is incident to base edge $e_i$ or candidate edge $f_j$, set the corresponding directed flow variables for that $(q,t)$ to 0, because they do not exist in $G - q$;
  for each candidate edge variable, the exact activation big-$M$ is 1:
  $g^(q,t)_(j,eta) <= y_j$ for every $q, t, j, eta$;
  and for every valid pair $(q,t)$ with $t in.not {q, r_q}$ and every surviving vertex $v != q$, impose flow conservation
  $sum_"out of v" (f^(q,t) + g^(q,t)) - sum_"into v" (f^(q,t) + g^(q,t)) = 1$
  when $v = r_q$,
  $= -1$ when $v = t$,
  and $= 0$ otherwise.
  The sums range over both orientations of all base and candidate edges that avoid $q$. Since every commodity carries exactly one unit, binary flows are sufficient.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(j = 0)^(p - 1) w_j y_j <= B \
    & f^(q,t)_(i,eta) = 0 quad "whenever" t in {q, r_q} "or" e_i "is incident to" q \
    & g^(q,t)_(j,eta) = 0 quad "whenever" t in {q, r_q} "or" f_j "is incident to" q \
    & g^(q,t)_(j,eta) <= y_j quad forall q, t in {0, dots, n - 1}, j in {0, dots, p - 1}, eta in {0, 1} \
    & "for each valid pair" (q,t) ", unit-flow conservation from" r_q "to" t "holds in" G - q \
    & y_j, f^(q,t)_(i,eta), g^(q,t)_(j,eta) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) If the chosen augmentation makes the graph biconnected, then every vertex-deleted graph is connected and therefore supports the required flows. ($arrow.l.double$) If the ILP is feasible, then removing any single vertex leaves a connected graph, which is exactly the definition of biconnectivity for the augmented graph.

  _Solution extraction._ Output the binary selection vector of candidate edges.
]

#reduction-rule("BoundedComponentSpanningForest", "ILP")[
  Assign every vertex to one of at most $K$ components, bound each component's total weight, and certify connectivity inside each used component by a flow witness.
][
  _Construction._ Let $n = |V|$, let the graph edges be $E = {e_0, dots, e_(m-1)}$ with $e_i = {u_i, v_i}$, and let the allowed component labels be $c in {0, dots, K - 1}$. Use `ILP<i32>` with variables ordered as
  $(x_(v,c))_(v,c), (u_c)_c, (r_(v,c))_(v,c), (s_c)_c, (b_(v,c))_(v,c), (f_(i,eta,c))_(i,eta,c)$.
  Their indices are
  $"idx"_x(v,c) = v K + c$,
  $"idx"_u(c) = n K + c$,
  $"idx"_r(v,c) = n K + K + v K + c$,
  $"idx"_s(c) = 2 n K + K + c$,
  $"idx"_b(v,c) = 2 n K + 2 K + v K + c$,
  and, with $eta = 0$ meaning $u_i -> v_i$ and $eta = 1$ meaning $v_i -> u_i$,
  $"idx"_f(i, eta, c) = 3 n K + 2 K + (i 2 + eta) K + c$.
  There are $3 n K + 2 K + 2 m K$ variables.

  Here $x_(v,c) in {0, 1}$ means vertex $v$ is placed in component $c$, $u_c in {0, 1}$ says that component $c$ is nonempty, $r_(v,c) in {0, 1}$ chooses the root of nonempty component $c$, $s_c in {0, dots, n}$ is its size, $b_(v,c) in {0, dots, n}$ linearizes the product $s_c r_(v,c)$, and $f_(i,eta,c) in {0, dots, n - 1}$ is the root-flow on the chosen component edges.

  The constraints are:
  $sum_(c = 0)^(K - 1) x_(v,c) = 1$ for every vertex $v$;
  $sum_v w_v x_(v,c) <= B$ for every component label $c$;
  $s_c = sum_v x_(v,c)$ for every $c$;
  $u_c <= s_c$ and $s_c <= n u_c$ for every $c$, so $u_c = 1$ iff the component is nonempty;
  $sum_v r_(v,c) = u_c$ and $r_(v,c) <= x_(v,c)$ for every $c$ and $v$, which chooses exactly one root in every nonempty component;
  the product linearization $b_(v,c) <= s_c$, $b_(v,c) <= n r_(v,c)$, $b_(v,c) >= s_c - n (1 - r_(v,c))$, $b_(v,c) >= 0$ for every $v, c$, so $b_(v,c) = s_c r_(v,c)$; the exact big-$M$ here is $n$;
  for every edge $e_i = {u_i, v_i}$, orientation flag $eta in {0, 1}$, and component $c$,
  $0 <= f_(i,eta,c) <= (n - 1) x_(u_i,c)$ and $0 <= f_(i,eta,c) <= (n - 1) x_(v_i,c)$.
  The exact capacity big-$M$ is $n - 1$: a component of size $s_c$ needs to route at most $s_c - 1 <= n - 1$ units across any oriented edge of a spanning tree.
  Finally, for every vertex $v$ and component $c$,
  $sum_"out of v in c" f - sum_"into v in c" f = b_(v,c) - x_(v,c)$.
  If $v$ is the chosen root of component $c$, then the right-hand side is $s_c - 1$; every other assigned vertex consumes one unit; unassigned vertices have right-hand side 0.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(c = 0)^(K - 1) x_(v,c) = 1 quad forall v in V \
    & sum_v w_v x_(v,c) <= B quad forall c in {0, dots, K - 1} \
    & s_c = sum_v x_(v,c) quad forall c in {0, dots, K - 1} \
    & u_c <= s_c <= n u_c quad forall c in {0, dots, K - 1} \
    & sum_v r_(v,c) = u_c, r_(v,c) <= x_(v,c) quad forall v, c \
    & "the standard product linearization enforces" b_(v,c) = s_c r_(v,c) quad forall v, c \
    & 0 <= f_(i,eta,c) <= (n - 1) x_(u_i,c), 0 <= f_(i,eta,c) <= (n - 1) x_(v_i,c) quad forall i, eta, c \
    & sum_"out of v in c" f - sum_"into v in c" f = b_(v,c) - x_(v,c) quad forall v, c \
    & x_(v,c), u_c, r_(v,c) in {0, 1}; s_c in {0, dots, n}; b_(v,c), f_(i,eta,c) in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any valid bounded-component partition assigns each component a connected supporting subgraph and respects the weight bound. ($arrow.l.double$) Any feasible ILP solution partitions the vertices into at most $K$ connected sets, each of total weight at most $B$, exactly as required by the source problem.

  _Solution extraction._ For each vertex $v$, output the unique component label $c$ with $x_(v,c) = 1$.
]

#reduction-rule("MinimumCutIntoBoundedSets", "ILP")[
  A binary side variable for each vertex, together with cut indicators on the edges, directly linearizes the bounded two-way cut conditions.
][
  _Construction._ Variables: binary $x_v$ with $x_v = 1$ iff $v$ is placed on the sink side, and binary $y_e$ for edges. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & x_s = 0 \
    & x_t = 1 \
    & sum_v x_v <= B \
    & sum_v (1 - x_v) <= B \
    & y_e >= x_u - x_v quad forall e = {u, v} in E \
    & y_e >= x_v - x_u quad forall e = {u, v} in E \
    & sum_e w_e y_e <= K \
    & x_v, y_e in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any feasible bounded cut determines a 0/1 side assignment, and the edge indicators are 1 exactly on the cut edges. ($arrow.l.double$) Any feasible ILP solution partitions the vertices into two bounded sets with $s$ and $t$ separated and total cut weight at most $K$.

  _Solution extraction._ Output the partition bit-vector $(x_v)_(v in V)$.
]

#reduction-rule("StrongConnectivityAugmentation", "ILP")[
  Select candidate arcs under the budget and certify strong connectivity by sending flow both from a root to every vertex and back again.
][
  _Construction._ Let the base arcs be $A = {a_0, dots, a_(m-1)}$ with $a_i = (u_i, v_i)$, let the candidate arcs be $C = {c_0, dots, c_(p-1)}$ with $c_j = (s_j, t_j)$, and, when $n = |V| >= 1$, fix the root to be vertex $r = 0$. If $n <= 1$, return the empty feasible ILP. Use `ILP<i32>` with variables ordered as
  $(y_j)_j, (f^t_i)_(t,i), (bar(f)^t_j)_(t,j), (g^t_i)_(t,i), (bar(g)^t_j)_(t,j)$,
  where $f^t$ is the forward root-to-$t$ flow on base arcs, $bar(f)^t$ is the forward flow on candidate arcs, $g^t$ is the backward $t$-to-root flow on base arcs, and $bar(g)^t$ is the backward flow on candidate arcs.
  The indices are
  $"idx"_y(j) = j$,
  $"idx"_(f^t_i) = p + t m + i$,
  $"idx"_(bar(f)^t_j) = p + n m + t p + j$,
  $"idx"_(g^t_i) = p + n (m + p) + t m + i$,
  and $ "idx"_(bar(g)^t_j) = p + n (2 m + p) + t p + j$.
  There are $p + 2 n (m + p)$ variables.

  The constraints are:
  $sum_(j = 0)^(p - 1) w_j y_j <= B$;
  for the dummy commodity $t = r$, set all four flow blocks $f^r_i$, $bar(f)^r_j$, $g^r_i$, $bar(g)^r_j$ to 0;
  for every candidate arc and target vertex, use the exact activation big-$M = 1$:
  $bar(f)^t_j <= y_j$ and $bar(g)^t_j <= y_j$ for all $t, j$;
  for every $t != r$ and every vertex $v$,
  $sum_"out of v" (f^t + bar(f)^t) - sum_"into v" (f^t + bar(f)^t) = 1$
  when $v = r$,
  $= -1$ when $v = t$,
  and $= 0$ otherwise;
  and
  $sum_"out of v" (g^t + bar(g)^t) - sum_"into v" (g^t + bar(g)^t) = 1$
  when $v = t$,
  $= -1$ when $v = r$,
  and $= 0$ otherwise.
  All flow variables are binary, because each commodity carries a single unit.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(j = 0)^(p - 1) w_j y_j <= B \
    & f^r_i = 0, bar(f)^r_j = 0, g^r_i = 0, bar(g)^r_j = 0 \
    & bar(f)^t_j <= y_j, bar(g)^t_j <= y_j quad forall t in {0, dots, n - 1}, j in {0, dots, p - 1} \
    & "root-to-target unit-flow conservation holds on" f^t, bar(f)^t quad forall t != r \
    & "target-to-root unit-flow conservation holds on" g^t, bar(g)^t quad forall t != r \
    & y_j, f^t_i, bar(f)^t_j, g^t_i, bar(g)^t_j in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A strongly connected augmentation provides both directions of reachability between the root and every other vertex, hence all required flows. ($arrow.l.double$) If those flows exist for every vertex, then every vertex is reachable from the root and can reach the root, so the augmented digraph is strongly connected.

  _Solution extraction._ Output the binary candidate-arc selection vector $(y_a)$.
]

// Matrix/encoding

#reduction-rule("BMF", "ILP")[
  Split the witness into binary factor matrices $B$ and $C$, reconstruct their Boolean product with McCormick auxiliaries, and minimize the Hamming distance to the target matrix.
][
  _Construction._ Variables: binary $b_(i,r)$, binary $c_(r,j)$, binary $p_(i,r,j)$ linearizing $b_(i,r) c_(r,j)$, binary $w_(i,j)$ for the reconstructed entry, and nonnegative error variables $e_(i,j)$. The ILP is:
  $
    min quad & sum_(i,j) e_(i,j) \
    "subject to" quad & p_(i,r,j) <= b_(i,r) quad forall i, r, j \
    & p_(i,r,j) <= c_(r,j) quad forall i, r, j \
    & p_(i,r,j) >= b_(i,r) + c_(r,j) - 1 quad forall i, r, j \
    & w_(i,j) >= p_(i,r,j) quad forall i, r, j \
    & w_(i,j) <= sum_r p_(i,r,j) quad forall i, j \
    & e_(i,j) >= A_(i,j) - w_(i,j) quad forall i, j \
    & e_(i,j) >= w_(i,j) - A_(i,j) quad forall i, j \
    & b_(i,r), c_(r,j), p_(i,r,j), w_(i,j) in {0, 1}, e_(i,j) in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any choice of factor matrices induces the same Boolean product and Hamming error in the ILP. ($arrow.l.double$) Any feasible ILP assignment determines factor matrices $B$ and $C$, and the linearization forces the objective to equal the Hamming distance between $A$ and $B dot C$.

  _Solution extraction._ Output the flattened bits of $B$ followed by the flattened bits of $C$, discarding the reconstruction auxiliaries.
]

#reduction-rule("ConsecutiveBlockMinimization", "ILP")[
  Permute the columns with a one-hot assignment and count row-wise block starts by detecting each 0-to-1 transition after permutation.
][
  _Construction._ Variables: binary $x_(c,p)$ with $x_(c,p) = 1$ iff column $c$ goes to position $p$, binary $a_(r,p)$ for the value seen by row $r$ at position $p$, and binary block-start indicators $b_(r,p)$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_p x_(c,p) = 1 quad forall c \
    & sum_c x_(c,p) = 1 quad forall p \
    & a_(r,p) = sum_c A_(r,c) x_(c,p) quad forall r, p \
    & b_(r,0) = a_(r,0) quad forall r \
    & b_(r,p) >= a_(r,p) - a_(r,p-1) quad forall r, p > 0 \
    & sum_(r,p) b_(r,p) <= K \
    & x_(c,p), a_(r,p), b_(r,p) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any column permutation determines exactly one block-start variable for each maximal run of 1s in every row. ($arrow.l.double$) A feasible ILP solution is a column permutation whose counted block starts sum to at most $K$, which is precisely the source criterion.

  _Solution extraction._ Decode the column permutation from $x_(c,p)$.
]

#reduction-rule("ConsecutiveOnesMatrixAugmentation", "ILP")[
  Choose a column permutation and, for each row, choose the interval that will become its consecutive block of 1s; flips are needed only for zeros inside that interval.
][
  _Construction._ Let the matrix have $m$ rows and $n$ columns, and let $A_(r,c) in {0, 1}$ be the given entry. For each row define the constant
  $beta_r = 1$ if row $r$ contains at least one 1, and $beta_r = 0$ otherwise.
  Use `ILP<bool>` with variable order
  $(x_(c,p))_(c,p), (a_(r,p))_(r,p), (ell_(r,p))_(r,p), (u_(r,p))_(r,p), (h_(r,p))_(r,p), (f_(r,p))_(r,p)$.
  The indices are
  $"idx"_x(c,p) = c n + p$,
  $"idx"_a(r,p) = n^2 + r n + p$,
  $"idx"_ell(r,p) = n^2 + m n + r n + p$,
  $"idx"_u(r,p) = n^2 + 2 m n + r n + p$,
  $"idx"_h(r,p) = n^2 + 3 m n + r n + p$,
  and $ "idx"_f(r,p) = n^2 + 4 m n + r n + p$.
  There are $n^2 + 5 m n$ binary variables.

  Here $x_(c,p) = 1$ means original column $c$ is placed at position $p$ of the permutation, $a_(r,p)$ is the value seen in row $r$ at permuted position $p$, $ell_(r,p)$ and $u_(r,p)$ choose the left and right interval boundaries of row $r$, $h_(r,p)$ indicates that position $p$ lies inside that chosen interval, and $f_(r,p)$ indicates that row $r$ flips a 0 to a 1 at position $p$.

  The constraints are:
  $sum_p x_(c,p) = 1$ for every column $c$;
  $sum_c x_(c,p) = 1$ for every position $p$;
  $a_(r,p) = sum_c A_(r,c) x_(c,p)$ for every row $r$ and position $p$;
  $sum_p ell_(r,p) = beta_r$ and $sum_p u_(r,p) = beta_r$ for every row $r$;
  $sum_p p ell_(r,p) <= sum_p p u_(r,p) + (n - 1) (1 - beta_r)$ for every row $r$, which forces the left boundary not to exceed the right boundary when the row is nonzero;
  for every row $r$ and position $p$,
  $h_(r,p) <= sum_(q = 0)^p ell_(r,q)$,
  $h_(r,p) <= sum_(q = p)^(n - 1) u_(r,q)$,
  and
  $h_(r,p) >= sum_(q = 0)^p ell_(r,q) + sum_(q = p)^(n - 1) u_(r,q) - 1$;
  $a_(r,p) <= h_(r,p)$ for every $r, p$, so every original 1 lies inside the chosen interval;
  $h_(r,p) <= a_(r,p) + f_(r,p)$, $f_(r,p) <= h_(r,p)$, and $f_(r,p) + a_(r,p) <= 1$ for every $r, p$, so $f_(r,p) = 1$ exactly when the position lies inside the interval but the original matrix has a 0 there;
  and the augmentation budget
  $sum_(r = 0)^(m - 1) sum_(p = 0)^(n - 1) f_(r,p) <= K$.
  These are the exact consecutive-ones constraints: after permutation, row $r$ is 1 exactly on the positions with $h_(r,p) = 1$, and the only modifications charged are the zero-to-one flips recorded by $f$.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_p x_(c,p) = 1 quad forall c \
    & sum_c x_(c,p) = 1 quad forall p \
    & a_(r,p) = sum_c A_(r,c) x_(c,p) quad forall r, p \
    & sum_p ell_(r,p) = beta_r, sum_p u_(r,p) = beta_r quad forall r \
    & sum_p p ell_(r,p) <= sum_p p u_(r,p) + (n - 1) (1 - beta_r) quad forall r \
    & h_(r,p) <= sum_(q = 0)^p ell_(r,q), h_(r,p) <= sum_(q = p)^(n - 1) u_(r,q) quad forall r, p \
    & h_(r,p) >= sum_(q = 0)^p ell_(r,q) + sum_(q = p)^(n - 1) u_(r,q) - 1 quad forall r, p \
    & a_(r,p) <= h_(r,p); h_(r,p) <= a_(r,p) + f_(r,p); f_(r,p) <= h_(r,p); f_(r,p) + a_(r,p) <= 1 quad forall r, p \
    & sum_(r = 0)^(m - 1) sum_(p = 0)^(n - 1) f_(r,p) <= K \
    & x_(c,p), a_(r,p), ell_(r,p), u_(r,p), h_(r,p), f_(r,p) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A feasible augmentation chooses a permutation and flips exactly the zeros lying inside each row's final consecutive-ones interval. ($arrow.l.double$) Any feasible ILP solution yields a permuted matrix whose rows become consecutive-ones after the encoded zero-to-one augmentations, with total augmentation cost at most $K$.

  _Solution extraction._ Decode the column permutation from $x_(c,p)$ and discard the auxiliary flip variables.
]

#reduction-rule("ConsecutiveOnesSubmatrix", "ILP")[
  Select exactly $K$ columns, permute only those selected columns, and require every row to have a single consecutive block within the chosen submatrix.
][
  _Construction._ Variables: binary selection bits $s_c$, binary placement variables $x_(c,p)$ for selected columns, and row-interval auxiliaries as in ConsecutiveOnesMatrixAugmentation. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_c s_c = K \
    & sum_p x_(c,p) = s_c quad forall c \
    & sum_c x_(c,p) = 1 quad forall p in {1, dots, K} \
    & "the selected rows satisfy the consecutive-ones interval constraints" \
    & s_c, x_(c,p) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any feasible column subset with a valid permutation satisfies the selection and interval constraints. ($arrow.l.double$) Any feasible ILP solution chooses exactly $K$ columns whose induced submatrix admits a consecutive-ones permutation.

  _Solution extraction._ Output the column-selection bits $(s_c)_(c = 1)^n$ and ignore the permutation auxiliaries.
]

#reduction-rule("SparseMatrixCompression", "ILP")[
  Assign each row one shift value and forbid any pair of shifted 1-entries from colliding in the storage vector.
][
  _Construction._ Variables: binary $x_(r,g)$ with $x_(r,g) = 1$ iff row $r$ uses shift $g in {0, dots, K - 1}$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_g x_(r,g) = 1 quad forall r \
    & x_(r,g) + x_(s,h) <= 1 quad "whenever" A_(r,i) = A_(s,j) = 1 "and" i + g = j + h \
    & x_(r,g) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) A valid compression chooses one shift per row and never overlays 1-entries from different rows in the same storage position. ($arrow.l.double$) Any feasible ILP solution gives exactly such a collision-free shift assignment, hence a valid storage vector of length $n + K$.

  _Solution extraction._ For each row $r$, output the unique zero-based shift $g$ with $x_(r,g) = 1$.
]

// Sequence/misc

#reduction-rule("ShortestCommonSupersequence", "ILP")[
  Fill the $B$ positions of the supersequence with one-hot symbol variables and match each input string monotonically into those positions.
][
  _Construction._ Variables: binary $x_(p,a)$ with $x_(p,a) = 1$ iff position $p$ carries symbol $a$, and binary matching variables $m_(s,j,p)$ saying that the $j$-th symbol of string $s$ is matched to position $p$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_a x_(p,a) = 1 quad forall p \
    & sum_p m_(s,j,p) = 1 quad forall s, j \
    & m_(s,j,p) <= x_(p,a) quad forall s, j, p " with symbol " a \
    & "matching positions are strictly increasing in j for every string s" \
    & x_(p,a), m_(s,j,p) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any common supersequence of length at most $B$ induces a one-hot symbol assignment and a monotone match of every input string. ($arrow.l.double$) Any feasible ILP solution yields a length-$B$ string into which every source string embeds as a subsequence.

  _Solution extraction._ At each position $p$, output the unique symbol $a$ with $x_(p,a) = 1$.
]

#reduction-rule("StringToStringCorrection", "ILP")[
  A time-expanded ILP chooses one edit operation at each of the $K$ stages and tracks the evolving string state until it matches the target.
][
  _Construction._ Let the source length be $n$, the target length be $m$, and the operation bound be $K$. If $m > n$ or $m < n - K$, return an infeasible empty ILP, because the model rejects such instances before any search. Otherwise use `ILP<bool>`. Track the evolving string by the identities of the original source positions, not only by their symbols: token $i in {0, dots, n - 1}$ carries symbol $x_i$ from the source string.

  The state variables are $z_(t,p,i)$ for $t in {0, dots, K}$, $p in {0, dots, n - 1}$, $i in {0, dots, n - 1}$, where $z_(t,p,i) = 1$ iff token $i$ occupies position $p$ after step $t$. Their indices are
  $"idx"_z(t,p,i) = t n^2 + p n + i$.
  The emptiness bits are $e_(t,p)$ with index
  $"idx"_e(t,p) = (K + 1) n^2 + t n + p$.
  The operation variables are delete bits $d_(t,j)$ for $t in {1, dots, K}$ and $j in {0, dots, n - 1}$ with index
  $"idx"_d(t,j) = (K + 1) (n^2 + n) + (t - 1) n + j$;
  swap bits $s_(t,j)$ for $j in {0, dots, n - 2}$ with index
  $"idx"_s(t,j) = (K + 1) (n^2 + n) + K n + (t - 1) (n - 1) + j$;
  and no-op bits $nu_t$ with index
  $"idx"_nu(t) = (K + 1) (n^2 + n) + K n + K (n - 1) + (t - 1)$.
  There are $(K + 1) (n^2 + n) + K (2 n)$ variables.

  The state validity constraints are:
  $e_(t,p) + sum_i z_(t,p,i) = 1$ for every $t, p$;
  $sum_p z_(t,p,i) <= 1$ for every $t, i$;
  and $e_(t,p) <= e_(t,p + 1)$ for every $t$ and $p < n - 1$, forcing the active string to occupy a prefix and the deleted positions to form a suffix.
  The initial state is fixed by
  $z_(0,p,p) = 1$ for every $p$,
  $z_(0,p,i) = 0$ for every $i != p$,
  and $e_(0,p) = 0$ for every position $p$.

  At each step $t$, choose exactly one operation:
  $sum_(j = 0)^(n - 1) d_(t,j) + sum_(j = 0)^(n - 2) s_(t,j) + nu_t = 1$.
  Delete at position $j$ is legal only when that current position exists, so
  $d_(t,j) <= 1 - e_(t - 1,j)$.
  Swap at position $j$ is legal only when both positions $j$ and $j + 1$ exist, so
  $s_(t,j) <= 1 - e_(t - 1,j)$ and $s_(t,j) <= 1 - e_(t - 1,j + 1)$.

  The state-update equations are conditioned by exact big-$M$ bounds with $M = 1$, because every left-hand side and every referenced right-hand side is binary. For every token $i$, step $t$, and position $p$:
  if no-op is chosen, impose
  $z_(t,p,i) - z_(t - 1,p,i) <= 1 - nu_t$ and $z_(t - 1,p,i) - z_(t,p,i) <= 1 - nu_t$;
  for every delete position $j$, if $p < j$ impose
  $z_(t,p,i) - z_(t - 1,p,i) <= 1 - d_(t,j)$ and $z_(t - 1,p,i) - z_(t,p,i) <= 1 - d_(t,j)$;
  if $j <= p < n - 1$, impose
  $z_(t,p,i) - z_(t - 1,p + 1,i) <= 1 - d_(t,j)$ and $z_(t - 1,p + 1,i) - z_(t,p,i) <= 1 - d_(t,j)$;
  and for the last position impose $z_(t,n - 1,i) <= 1 - d_(t,j)$;
  for every swap position $j$, if $p in.not {j, j + 1}$ impose
  $z_(t,p,i) - z_(t - 1,p,i) <= 1 - s_(t,j)$ and $z_(t - 1,p,i) - z_(t,p,i) <= 1 - s_(t,j)$;
  if $p = j$, impose
  $z_(t,j,i) - z_(t - 1,j + 1,i) <= 1 - s_(t,j)$ and $z_(t - 1,j + 1,i) - z_(t,j,i) <= 1 - s_(t,j)$;
  and if $p = j + 1$, impose
  $z_(t,j + 1,i) - z_(t - 1,j,i) <= 1 - s_(t,j)$ and $z_(t - 1,j,i) - z_(t,j + 1,i) <= 1 - s_(t,j)$.

  Finally force the step-$K$ state to equal the target string:
  $sum_(i : x_i = y_p) z_(K,p,i) = 1$ for every target position $p in {0, dots, m - 1}$,
  and $e_(K,p) = 1$ for every $p in {m, dots, n - 1}$.
  This exactly matches the model semantics, which compare the final working string to the target after $K$ operations.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & e_(t,p) + sum_i z_(t,p,i) = 1 quad forall t in {0, dots, K}, p in {0, dots, n - 1} \
    & sum_p z_(t,p,i) <= 1 quad forall t in {0, dots, K}, i in {0, dots, n - 1} \
    & e_(t,p) <= e_(t,p + 1) quad forall t in {0, dots, K}, p in {0, dots, n - 2} \
    & z_(0,p,p) = 1, z_(0,p,i) = 0 quad forall i != p, e_(0,p) = 0 quad forall p \
    & sum_(j = 0)^(n - 1) d_(t,j) + sum_(j = 0)^(n - 2) s_(t,j) + nu_t = 1 quad forall t in {1, dots, K} \
    & d_(t,j) <= 1 - e_(t - 1,j) quad forall t, j \
    & s_(t,j) <= 1 - e_(t - 1,j), s_(t,j) <= 1 - e_(t - 1,j + 1) quad forall t, j \
    & "the exact M = 1 state-update equations enforce no-op, delete, and adjacent-swap transitions" \
    & sum_(i : x_i = y_p) z_(K,p,i) = 1 quad forall p in {0, dots, m - 1} \
    & e_(K,p) = 1 quad forall p in {m, dots, n - 1} \
    & z_(t,p,i), e_(t,p), d_(t,j), s_(t,j), nu_t in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any valid length-$K$ edit script yields a feasible sequence of operations and states ending at the target. ($arrow.l.double$) Any feasible ILP solution traces a legal sequence of deletes, adjacent swaps, and no-ops whose final string is the target.

  _Solution extraction._ For each step $t$, compute the current length
  $ell_(t - 1) = sum_(p = 0)^(n - 1) (1 - e_(t - 1,p))$.
  If $nu_t = 1$, output the source code $2 n$. If $d_(t,j) = 1$, output $j$. If $s_(t,j) = 1$, output $ell_(t - 1) + j$. This is exactly the encoding used by `evaluate()`: deletions use raw positions, swaps are offset by the current length, and no-op is the distinguished value $2 n$.
]

#let ps_qubo = load-example("PaintShop", "QUBO")
#let ps_qubo_sol = ps_qubo.solutions.at(0)
#reduction-rule("PaintShop", "QUBO",
  example: true,
  example-caption: [4 cars, sequence length 8],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(ps_qubo.source) + " -o paintshop.json",
      "pred reduce paintshop.json --to " + target-spec(ps_qubo) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate paintshop.json --config " + ps_qubo_sol.source_config.map(str).join(","),
    )
    #{
      let n = ps_qubo.source.instance.num_cars
      let seq = ps_qubo.source.instance.sequence_indices
      let labels = ps_qubo.source.instance.car_labels
      let is-first = ps_qubo.source.instance.is_first
      let seq-labels = seq.map(i => labels.at(i))
      let coloring = seq.enumerate().map(((pos, car)) => {
        let first-color = ps_qubo_sol.source_config.at(car)
        if is-first.at(pos) { first-color } else { 1 - first-color }
      })
      [*Source:* $n = #n$ cars, sequence $(#seq-labels.join(", "))$. \
      *Parity:* #seq.enumerate().map(((pos, _)) => if is-first.at(pos) { "1st" } else { "2nd" }).join(", ") \
      *Step 1 -- One QUBO variable per car.* Binary variable $x_i in {0,1}$ for each car $i$: $x_i = 0$ means "first occurrence gets color 0, second gets color 1"; $x_i = 1$ reverses. \
      *Step 2 -- Build the $Q$ matrix from adjacent pairs.* For each adjacent pair $(j, j+1)$ in the sequence with distinct cars $a, b$: if both positions have the _same_ parity (both first or both second occurrence), a color switch occurs when $x_a != x_b$, contributing $+1$ to $Q_(a a)$, $+1$ to $Q_(b b)$, and $-2$ to $Q_(a b)$. If they have _different_ parity, a switch occurs when $x_a = x_b$, contributing $-1$ to $Q_(a a)$, $-1$ to $Q_(b b)$, and $+2$ to $Q_(a b)$. \
      *Step 3 -- Verify.* The QUBO solution $bold(x) = (#ps_qubo_sol.target_config.map(str).join(", "))$ yields coloring $(#coloring.map(str).join(", "))$ with #{coloring.windows(2).filter(w => w.at(0) != w.at(1)).len()} color switches #sym.checkmark ]
    }
  ],
)[
  Each car's two occurrences must receive opposite colors, so a single binary variable per car determines the full coloring. Adjacent pairs in the sequence contribute quadratic terms to a QUBO matrix based on their parity (first vs.\ second occurrence), and the QUBO minimum plus a constant offset equals the minimum number of color switches @Streif2021.
][
  _Construction._ Given $n$ cars, each appearing exactly twice in a sequence of length $2n$, introduce binary variables $x_1, ..., x_n$ (one per car). Initialize an $n times n$ upper-triangular matrix $Q$ of zeros. For each adjacent pair of positions $(j, j+1)$ with distinct cars $a, b$:

  - *Same parity* (both first or both second occurrence): a color switch occurs iff $x_a != x_b$. The switch indicator is $x_a + x_b - 2 x_a x_b$, so add $+1$ to $Q_(a a)$, $+1$ to $Q_(b b)$, and $-2$ to $Q_(a b)$.
  - *Different parity* (one first, one second): a color switch occurs iff $x_a = x_b$. The switch indicator is $1 - x_a - x_b + 2 x_a x_b$, so add $-1$ to $Q_(a a)$, $-1$ to $Q_(b b)$, and $+2$ to $Q_(a b)$, with a constant offset of $+1$.

  Adjacent pairs where both positions are the same car always produce a switch (constant term), and are skipped. The QUBO objective is $min bold(x)^top Q bold(x)$; the minimum number of color switches equals the QUBO minimum plus the total constant offset (number of different-parity pairs plus number of same-car pairs).

  _Correctness._ ($arrow.r.double$) Any PaintShop coloring corresponds to a binary assignment $bold(x)$ with the same number of switches (up to the constant offset). ($arrow.l.double$) Any QUBO minimizer $bold(x)$ defines a valid coloring (each car's two occurrences get opposite colors), and the offset-adjusted objective equals the switch count. Since the correspondence is bijective and value-preserving, optimality is preserved.

  _Solution extraction._ The QUBO solution $(x_1, ..., x_n)$ maps directly back: car $i$'s first occurrence gets color $x_i$, second gets $1 - x_i$.
]

#reduction-rule("PaintShop", "ILP")[
  One binary variable per car determines its first color, the second occurrence receives the opposite color automatically, and switch indicators count color changes along the sequence.
][
  _Construction._ Variables: binary $x_i$ for each car $i$, binary color variables $k_p$ for sequence positions, and binary switch indicators $c_p$ for positions $p > 0$. The ILP is:
  $
    min quad & sum_p c_p \
    "subject to" quad & k_p = x_i quad "if p is the first occurrence of car i" \
    & k_p = 1 - x_i quad "otherwise" \
    & c_p >= k_p - k_(p-1) quad forall p > 0 \
    & c_p >= k_(p-1) - k_p quad forall p > 0 \
    & x_i, k_p, c_p in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any first-occurrence coloring determines the whole paint sequence and induces exactly the same number of switches in the ILP. ($arrow.l.double$) Any ILP assignment is already a valid source witness, and the switch variables are forced to count adjacent color changes.

  _Solution extraction._ Output the first-occurrence color bits $(x_i)$.
]

#reduction-rule("IsomorphicSpanningTree", "ILP")[
  A bijection from the tree vertices to the graph vertices is enough: every tree edge must map to a graph edge, which then defines the desired spanning tree.
][
  _Construction._ Variables: binary $x_(u,v)$ with $x_(u,v) = 1$ iff tree vertex $u$ maps to graph vertex $v$. The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_v x_(u,v) = 1 quad forall u \
    & sum_u x_(u,v) = 1 quad forall v \
    & x_(u,v) + x_(w,z) <= 1 quad forall {u, w} in E_"tree", {v, z} in.not E_"graph" \
    & x_(u,z) + x_(w,v) <= 1 quad forall {u, w} in E_"tree", {v, z} in.not E_"graph" \
    & x_(u,v) in {0, 1}
  $.

  _Correctness._ ($arrow.r.double$) Any isomorphism from the given tree to a spanning tree of the graph satisfies the bijection and non-edge constraints. ($arrow.l.double$) Any feasible ILP solution is a bijection that preserves every tree edge, so the image edges form a spanning tree of the graph isomorphic to the source tree.

  _Solution extraction._ For each tree vertex $u$, output the unique graph vertex $v$ with $x_(u,v) = 1$.
]

#let rta_rtsa = load-example("RootedTreeArrangement", "RootedTreeStorageAssignment")
#let rta_rtsa_sol = rta_rtsa.solutions.at(0)
#reduction-rule("RootedTreeArrangement", "RootedTreeStorageAssignment",
  example: true,
  example-caption: [Path graph $P_4$ ($n = 4$, $|E| = 3$, $K = 5$)],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(rta_rtsa.source) + " -o rta.json",
      "pred reduce rta.json --to " + target-spec(rta_rtsa) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate rta.json --config " + rta_rtsa_sol.source_config.map(str).join(","),
    )
    Source: path graph $P_4$ with vertices ${0, 1, 2, 3}$, edges $\{0,1\}, \{1,2\}, \{2,3\}$, and bound $K = 5$. \
    Target: universe $X = {0, 1, 2, 3}$, subsets $\{0,1\}, \{1,2\}, \{2,3\}$, bound $K' = 5 - 3 = 2$. \
    The chain tree $0 arrow 1 arrow 2 arrow 3$ (parent array $(0, 0, 1, 2)$) with identity mapping gives total stretch $1 + 1 + 1 = 3 <= 5$ in the source. In the target, every edge subset is already a parent-child pair, so extension cost is $0 + 0 + 0 = 0 <= 2$ #sym.checkmark
  ],
)[
  This $O(|E|)$ reduction @gavril1977 transforms a graph-embedding arrangement problem into a set-system path-cover problem. Each edge $\{u, v\}$ of the source graph becomes a 2-element required subset $\{u, v\}$ whose elements must lie on a directed path in a rooted tree. The bound adjusts by $K' = K - |E|$, since each edge contributes at least 1 to the arrangement cost but 0 to the extension cost when its endpoints are adjacent in the tree.
][
  _Construction._ Given a Rooted Tree Arrangement instance with graph $G = (V, E)$ and bound $K$, construct a Rooted Tree Storage Assignment instance as follows. Set the universe $X = V$ with $|X| = |V|$ elements. For each edge $\{u, v\} in E$, create a 2-element subset $X_e = \{u, v\}$, yielding a collection $cal(C) = \{X_e : e in E\}$ of $|E|$ subsets. Set the bound $K' = K - |E|$.

  _Correctness._ ($arrow.r.double$) Suppose there exists a rooted tree $T$ on $|V|$ nodes and a bijection $f: V arrow U$ with $sum_(\{u,v\} in E) d_T(f(u), f(v)) <= K$, where every edge pair lies on a common root-to-leaf path. Using $T$ as the storage tree and the identity embedding (since $X = V$), for each edge $e = \{u, v\}$ the extended subset $X'_e$ consists of all nodes on the path from $f(u)$ to $f(v)$, costing $d_T(f(u), f(v)) - 1$ additional elements. The total extension cost is $sum_(e in E) (d_T(f(u), f(v)) - 1) = (sum d_T) - |E| <= K - |E| = K'$.

  ($arrow.l.double$) Suppose there exists a rooted tree $T = (X, A)$ and extended subsets forming directed paths with total extension cost $<= K'$. The same tree $T$ with the identity mapping $f(v) = v$ gives total arrangement stretch $= "extension cost" + |E| <= K' + |E| = K$.

  _Solution extraction._ The target solution is a parent array defining a rooted tree on $X = V$. The source solution is this same parent array concatenated with the identity mapping $f(v) = v$ for all $v in V$.
]

#reduction-rule("RootedTreeStorageAssignment", "ILP")[
  Choose one parent for each non-root element, enforce acyclicity with depth variables, and linearize the path-extension cost of every subset by selecting its top and bottom vertices in the rooted tree.
][
  _Construction._ Let $X = {0, dots, n - 1}$ and let the subset family be $cal(C) = {S_0, dots, S_(m-1)}$. For every subset of size 0 or 1 the model charges extension cost 0 automatically, so only the nontrivial subsets matter. Enumerate them as
  $I = {k_0 < dots < k_(r-1)} = {k : |S_k| >= 2}$.
  Use `ILP<i32>`. The variable blocks are:
  parent indicators $p_(v,u) in {0, 1}$ for all $v, u in X$;
  depths $d_v in {0, dots, n - 1}$;
  ancestor indicators $a_(u,v) in {0, 1}$, where $a_(u,v) = 1$ means $u$ is an ancestor of $v$ (allowing $u = v$);
  auxiliary transitive-closure variables $h_(u,v,w) in {0, 1}$;
  and, for each nontrivial subset gadget $s in {0, dots, r - 1}$ corresponding to original subset $S_(k_s)$, top selectors $t_(s,u)$, bottom selectors $b_(s,v)$, pair selectors $m_(s,u,v)$, endpoint depths $T_s, B_s$, and extension cost $c_s$.

  The indices are
  $"idx"_p(v,u) = v n + u$,
  $"idx"_d(v) = n^2 + v$,
  $"idx"_a(u,v) = n^2 + n + u n + v$,
  $"idx"_h(u,v,w) = 2 n^2 + n + (u n + v) n + w$,
  $"idx"_t(s,u) = n^3 + 2 n^2 + n + s n + u$,
  $"idx"_b(s,v) = n^3 + 2 n^2 + n + r n + s n + v$,
  $"idx"_m(s,u,v) = n^3 + 2 n^2 + n + 2 r n + s n^2 + u n + v$,
  $"idx"_T(s) = n^3 + 2 n^2 + n + 2 r n + r n^2 + s$,
  $"idx"_B(s) = n^3 + 2 n^2 + n + 2 r n + r n^2 + r + s$,
  and $ "idx"_c(s) = n^3 + 2 n^2 + n + 2 r n + r n^2 + 2 r + s$.
  The total number of variables is
  $n^3 + 2 n^2 + n + r (n^2 + 2 n + 3)$.

  The rooted-tree constraints are:
  $sum_(u = 0)^(n - 1) p_(v,u) = 1$ for every vertex $v$;
  $sum_v p_(v,v) = 1$, so exactly one vertex chooses itself as parent and becomes the root;
  $d_v <= (n - 1) (1 - p_(v,v))$ for every $v$, hence the unique root has depth 0;
  and for every ordered pair $u != v$,
  $d_v - d_u >= 1 - n (1 - p_(v,u))$ and $d_v - d_u <= 1 + n (1 - p_(v,u))$.
  The exact big-$M$ here is $M = n$: the expression $d_v - d_u - 1$ ranges from $-n$ to $n - 2$ when both depths lie in ${0, dots, n - 1}$.

  The ancestor relation is defined explicitly by
  $a_(v,v) = 1$ for every $v$,
  $h_(u,v,v) = 0$ for all $u, v$,
  and, for every $u != v$,
  $a_(u,v) = sum_(w = 0)^(n - 1) h_(u,v,w)$.
  The helper variables linearize the recursion "u is an ancestor of v iff u is an ancestor of the unique parent of v":
  $h_(u,v,w) <= p_(v,w)$,
  $h_(u,v,w) <= a_(u,w)$,
  and
  $h_(u,v,w) >= p_(v,w) + a_(u,w) - 1$
  for all $u, v, w$ with $w != v$.

  For each nontrivial subset gadget $s$ corresponding to $S_(k_s)$, choose path endpoints only from the subset itself:
  $sum_(u in S_(k_s)) t_(s,u) = 1$, $t_(s,u) = 0$ for $u in.not S_(k_s)$,
  $sum_(v in S_(k_s)) b_(s,v) = 1$, $b_(s,v) = 0$ for $v in.not S_(k_s)$.
  Linearize the chosen ordered endpoint pair by
  $m_(s,u,v) <= t_(s,u)$,
  $m_(s,u,v) <= b_(s,v)$,
  $m_(s,u,v) >= t_(s,u) + b_(s,v) - 1$.
  Because exactly one top and one bottom are chosen, exactly one pair selector is 1.

  The path condition for subset $S_(k_s)$ is then fully explicit:
  $m_(s,u,v) <= a_(u,v)$ for every $u, v$, so the chosen top is an ancestor of the chosen bottom;
  and for every element $w in S_(k_s)$,
  $m_(s,u,v) <= a_(u,w)$ and $m_(s,u,v) <= a_(w,v)$ for all $u, v$.
  Thus every subset element lies on the ancestor chain from the chosen top to the chosen bottom.

  Bind the endpoint depths to the chosen selectors by exact big-$M$ constraints with $M = n - 1$:
  $T_s - d_u <= (n - 1) (1 - t_(s,u))$ and $d_u - T_s <= (n - 1) (1 - t_(s,u))$ for every $u$;
  $B_s - d_v <= (n - 1) (1 - b_(s,v))$ and $d_v - B_s <= (n - 1) (1 - b_(s,v))$ for every $v$.
  Finally set the extension cost of the subset to the exact path surplus
  $c_s = B_s - T_s + 1 - |S_(k_s)|$,
  require $c_s >= 0$,
  and bound the total cost by
  $sum_(s = 0)^(r - 1) c_s <= K$.
  This matches the model's `subset_extension_cost()`: the top and bottom are the shallowest and deepest members of the subset on the chosen chain, and the path contributes exactly the interior vertices not already present in the subset.

  The ILP is:
  $
    "find" quad & bold(x) \
    "subject to" quad & sum_(u = 0)^(n - 1) p_(v,u) = 1 quad forall v in X \
    & sum_v p_(v,v) = 1, d_v <= (n - 1) (1 - p_(v,v)) quad forall v in X \
    & d_v - d_u >= 1 - n (1 - p_(v,u)), d_v - d_u <= 1 + n (1 - p_(v,u)) quad forall u != v \
    & a_(v,v) = 1, h_(u,v,v) = 0, a_(u,v) = sum_(w = 0)^(n - 1) h_(u,v,w) quad forall u, v in X \
    & h_(u,v,w) <= p_(v,w), h_(u,v,w) <= a_(u,w), h_(u,v,w) >= p_(v,w) + a_(u,w) - 1 quad forall u, v, w in X " with " w != v \
    & sum_(u in S_(k_s)) t_(s,u) = 1, t_(s,u) = 0 quad forall u in.not S_(k_s), s \
    & sum_(v in S_(k_s)) b_(s,v) = 1, b_(s,v) = 0 quad forall v in.not S_(k_s), s \
    & m_(s,u,v) <= t_(s,u), m_(s,u,v) <= b_(s,v), m_(s,u,v) >= t_(s,u) + b_(s,v) - 1 quad forall s, u, v \
    & m_(s,u,v) <= a_(u,v), m_(s,u,v) <= a_(u,w), m_(s,u,v) <= a_(w,v) quad forall s, u, v, w in S_(k_s) \
    & T_s - d_u <= (n - 1) (1 - t_(s,u)), d_u - T_s <= (n - 1) (1 - t_(s,u)) quad forall s, u \
    & B_s - d_v <= (n - 1) (1 - b_(s,v)), d_v - B_s <= (n - 1) (1 - b_(s,v)) quad forall s, v \
    & c_s = B_s - T_s + 1 - |S_(k_s)|, c_s >= 0 quad forall s; sum_s c_s <= K \
    & p_(v,u), a_(u,v), h_(u,v,w), t_(s,u), b_(s,v), m_(s,u,v) in {0, 1} \
    & d_v, T_s, B_s, c_s in ZZ_(>=0)
  $.

  _Correctness._ ($arrow.r.double$) Any rooted tree satisfying all subset-path conditions induces parent, depth, and path-endpoint variables with the same total extension cost. ($arrow.l.double$) Any feasible ILP solution defines a rooted tree in which every subset lies on one ancestor chain, and the encoded path lengths keep the total extension cost within the bound.

  _Solution extraction._ For each vertex $v$, output its unique parent $u$ with $p_(v,u) = 1$.
]

#reduction-rule("MinimumEdgeCostFlow", "ILP")[
  Introduce integer flow variables and binary arc-activation indicators, link them so that an indicator is forced to 1 whenever the corresponding arc carries positive flow, and minimize the total price of activated arcs.
][
  _Construction._ Let $m = |A|$ and $n = |V|$. Use `ILP<i32>` with $2m$ variables: integer flow variables $f_a in {0, dots, c(a)}$ for $a in {0, dots, m - 1}$ and binary activation indicators $y_a in {0, 1}$ for $a in {m, dots, 2m - 1}$.

  Constraints:
  - _Linking:_ $f_a - c(a) dot y_a <= 0$ for each arc $a$ — forces $y_a = 1$ when $f_a > 0$ ($m$ constraints).
  - _Binary bound:_ $y_a <= 1$ for each arc $a$ ($m$ constraints).
  - _Flow conservation:_ $sum_(a "entering" v) f_a - sum_(a "leaving" v) f_a = 0$ for each non-terminal vertex $v in V backslash {s, t}$ ($n - 2$ constraints).
  - _Flow requirement:_ $sum_(a "entering" t) f_a - sum_(a "leaving" t) f_a >= R$ (1 constraint).

  Objective: $min sum_(a=0)^(m-1) p(a) dot y_a$.

  Total: $2m + n - 1$ constraints, $2m$ variables.

  _Correctness._ ($arrow.r.double$) Any feasible flow of value $>= R$ determines the flow variables directly, and the linking constraints force $y_a = 1$ for every arc with positive flow, so the ILP objective equals the edge cost. ($arrow.l.double$) Any feasible ILP solution has $f_a <= c(a) y_a <= c(a)$ and satisfies conservation and the flow requirement. The objective $sum p(a) y_a$ is at least the edge cost of the flow because $y_a >= 1$ whenever $f_a > 0$.

  _Solution extraction._ Output the first $m$ variables $(f_0, dots, f_(m-1))$ as the flow assignment.
]

#{
  let mlr_ilp = load-example("MaximumLikelihoodRanking", "ILP")
  let mlr_ilp_sol = mlr_ilp.solutions.at(0)
  let mlr_n = mlr_ilp.source.instance.matrix.len()
  let mlr_nv = mlr_ilp.target.instance.num_vars
  let mlr_nc = mlr_ilp.target.instance.constraints.len()
  [
    #reduction-rule("MaximumLikelihoodRanking", "ILP",
      example: true,
      example-caption: [$n = #mlr_n$ items, $#mlr_nv$ pairwise variables, $#mlr_nc$ transitivity constraints],
      extra: [
        #pred-commands(
          "pred create --example MaximumLikelihoodRanking -o mlr.json",
          "pred reduce mlr.json --to " + target-spec(mlr_ilp) + " -o bundle.json",
          "pred solve bundle.json",
          "pred evaluate mlr.json --config " + mlr_ilp_sol.source_config.map(str).join(","),
        )
        *Step 1 -- Source instance.* A ranking instance with $n = #mlr_n$ items and comparison matrix $A$.

        *Step 2 -- Build the ILP.* Introduce $binom(#str(mlr_n), 2) = #mlr_nv$ binary variables $x_(i j)$ for each pair $i < j$. Add $#mlr_nc$ transitivity constraints from all $binom(#str(mlr_n), 3)$ triples. The ILP has #mlr_nv variables and #mlr_nc constraints.

        *Step 3 -- Verify.* The ILP optimum extracts to ranking $(#mlr_ilp_sol.source_config.map(str).join(", "))$, which matches the source optimum #sym.checkmark.
      ],
    )[
      Each pair of items $(i, j)$ with $i < j$ gets a binary variable $x_(i j)$ indicating whether $i$ is ranked before $j$. Transitivity constraints enforce a valid linear order, and the objective minimizes the total disagreement cost.
    ][
      _Construction._ Given $n$ items and comparison matrix $A$, introduce $n(n-1)/2$ binary variables $x_(i j)$ for $i < j$. For each triple ${a, b, c}$ with $a < b < c$, add two transitivity constraints:
      $
        x_(a b) + x_(b c) - x_(a c) &<= 1 \
        -x_(a b) - x_(b c) + x_(a c) &<= 0
      $
      The objective is:
      $
        min sum_(i < j) (a_(j i) - a_(i j)) dot x_(i j)
      $
      This yields $n(n-1)/2$ variables and $n(n-1)(n-2)/3$ constraints.

      _Correctness._ ($arrow.r.double$) Any permutation $pi$ defines a consistent tournament: set $x_(i j) = 1$ iff $i$ appears before $j$ in $pi$. The transitivity constraints are satisfied because a linear order has no directed cycles. The ILP objective equals the disagreement cost of $pi$. ($arrow.l.double$) Any feasible binary solution defines a transitive tournament (an acyclic tournament), which corresponds to a unique linear order. The objective equals the disagreement cost of that order.

      _Solution extraction._ For each item $i$, count the number of items ranked before it: $"rank"(i) = sum_(j < i) x_(j i) + sum_(j > i) (1 - x_(i j))$. The resulting rank vector is the permutation.
    ]
  ]
}

#{
  let ocst_ilp = load-example("OptimumCommunicationSpanningTree", "ILP")
  let ocst_ilp_sol = ocst_ilp.solutions.at(0)
  let ocst_n = ocst_ilp.source.instance.num_vertices
  let ocst_nv = ocst_ilp.target.instance.num_vars
  let ocst_nc = ocst_ilp.target.instance.constraints.len()
  [
    #reduction-rule("OptimumCommunicationSpanningTree", "ILP",
      example: true,
      example-caption: [$K_#ocst_n$, #ocst_nv variables, #ocst_nc constraints],
      extra: [
        #pred-commands(
          "pred create --example OptimumCommunicationSpanningTree -o ocst.json",
          "pred reduce ocst.json --to " + target-spec(ocst_ilp) + " -o bundle.json",
          "pred solve bundle.json",
          "pred evaluate ocst.json --config " + ocst_ilp_sol.source_config.map(str).join(","),
        )
        *Step 1 -- Source instance.* $K_#ocst_n$ with edge weight and requirement matrices.

        *Step 2 -- Build the ILP.* Introduce edge selectors and multi-commodity flow variables. The ILP has #ocst_nv variables and #ocst_nc constraints.

        *Step 3 -- Verify.* The ILP optimum extracts to edge selection $(#ocst_ilp_sol.source_config.map(str).join(", "))$, which matches the source optimum #sym.checkmark.
      ],
    )[
      Binary edge selectors determine the spanning tree. Multi-commodity flow variables route one unit of flow for each vertex pair with positive requirement, and the objective minimizes the total weighted communication cost.
    ][
      _Construction._ Given $K_n$ with weights $w(e)$ and requirements $r(u, v)$, let $m = n(n-1)/2$. Introduce binary edge variables $x_e$ for each edge. For each pair $(s, t)$ with $r(s, t) > 0$, add directed flow variables $f^(s t)_(e, "fwd")$ and $f^(s t)_(e, "bwd")$ for each edge $e$.

      Constraints:
      - Tree size: $sum_e x_e = n - 1$
      - Flow conservation: for each commodity $(s, t)$ and each vertex $v$, net inflow equals $-1$ at source $s$, $+1$ at sink $t$, and $0$ elsewhere.
      - Capacity linking: $f^(s t)_(e, "dir") <= x_e$ for each commodity and direction.

      Objective: $min sum_((s,t): r > 0) r(s,t) dot sum_e w(e) dot (f^(s t)_(e, "fwd") + f^(s t)_(e, "bwd")))$

      _Correctness._ ($arrow.r.double$) Any spanning tree defines edge selectors and unique path flows. The objective equals the communication cost. ($arrow.l.double$) Any feasible ILP solution with $n - 1$ selected edges and valid flows corresponds to a connected spanning subgraph (tree), and the flow cost equals the path-weighted communication cost.

      _Solution extraction._ Output the first $m$ variables $(x_0, dots, x_(m-1))$ as the edge selection.
    ]
  ]
}

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
  (source: "MinimumMaximalMatching", target: "ILP"),
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

// === Missing problem-defs and ILP reduction-rules ===

#problem-def("MaximumDomaticNumber")[
  Given an undirected graph $G = (V, E)$, find the maximum $k$ such that $V$ can be partitioned into $k$ disjoint dominating sets $V_1, dots, V_k$ where each $V_i$ dominates all of $V$.
][
  Maximum Domatic Number (GT3) @garey1979. NP-complete for any fixed $k >= 3$ (Garey, Johnson, Tarjan 1976). Polynomial for $k = 2$.

  The best known exact algorithm runs in $O^*(2.695^n)$ (Riege, Rothe, Spakowski, Yamamoto 2007).
]

#reduction-rule("MaximumDomaticNumber", "ILP")[
  Binary assignment variables $x_(v,i) in {0,1}$ for each vertex $v$ and set index $i in {1, dots, n}$, plus binary usage indicators $y_i$. Partition constraints, domination constraints, and linking constraints enforce a valid domatic partition. Maximize $sum_i y_i$.
][
  _Construction._ Introduce $n^2 + n$ binary variables. For each vertex $v$, $sum_i x_(v,i) = 1$ (partition). For each $v$ and $i$, $x_(v,i) + sum_(u in N(v)) x_(u,i) >= y_i$ (domination). For each $i$, $y_i <= sum_v x_(v,i)$ (linking). Maximize $sum_i y_i$.

  _Correctness._ ($arrow.r.double$) Any valid domatic partition gives a feasible ILP assignment with the same number of non-empty sets. ($arrow.l.double$) Any feasible ILP solution encodes a valid domatic partition.

  _Solution extraction._ For each vertex $v$, find $i$ with $x_(v,i) = 1$; set $"config"[v] = i$.
]

#problem-def("MinimumMetricDimension")[
  Given an undirected graph $G = (V, E)$, find a minimum-size resolving set $V' subset.eq V$ such that for every pair of distinct vertices $u, v in V$, there exists $w in V'$ with $d(u, w) != d(v, w)$.
][
  Minimum Metric Dimension (GT61) @garey1979. NP-complete via reduction from 3-Dimensional Matching. Polynomial for trees.

  The best known exact algorithm is brute-force $O^*(2^n)$.
]

#reduction-rule("MinimumMetricDimension", "ILP")[
  Binary variables $z_v in {0,1}$ for each vertex. Precompute all-pairs shortest-path distances. For each pair $(u, v)$ with $u < v$, add constraint $sum_(w: d(u,w) != d(v,w)) z_w >= 1$. Minimize $sum_v z_v$.
][
  _Construction._ $n$ binary variables and $n(n-1)/2$ pair-distinguishing constraints from BFS distances.

  _Correctness._ ($arrow.r.double$) A resolving set satisfies all pair constraints. ($arrow.l.double$) Any feasible solution is a resolving set since every pair is distinguished.

  _Solution extraction._ $z_v = 1 arrow.r "config"[v] = 1$.
]

#problem-def("MinimumGraphBandwidth")[
  Given an undirected graph $G = (V, E)$, find a one-to-one mapping $f: V -> {0, 1, dots, |V|-1}$ that minimizes $max_({u,v} in E) |f(u) - f(v)|$.
][
  Graph Bandwidth (GT40) @garey1979. NP-complete even for trees with maximum degree 3. The brute-force bound is $O^*(n!)$ over all permutations.
]

#reduction-rule("MinimumGraphBandwidth", "ILP")[
  Assignment variables $x_(v,p) in {0,1}$ for vertex $v$ and position $p$, integer position variables, and bandwidth variable $B$. Bijection constraints enforce a valid permutation; edge-stretch constraints enforce $|"pos"(u) - "pos"(v)| <= B$. Minimize $B$.
][
  _Construction._ $n^2 + n + 1$ variables (assignment + position + bandwidth). Bijection: $sum_p x_(v,p) = 1$ and $sum_v x_(v,p) = 1$. Position linking and edge-stretch constraints bound the bandwidth.

  _Correctness._ ($arrow.r.double$) Any valid permutation with bandwidth $B$ gives a feasible ILP with objective $B$. ($arrow.l.double$) Any feasible ILP solution encodes a valid permutation with bandwidth $<= B$.

  _Solution extraction._ For each vertex $v$, find $p$ with $x_(v,p) = 1$; set $"config"[v] = p$.
]

#problem-def("MinimumCapacitatedSpanningTree")[
  Given a weighted graph $G = (V, E)$ with root $v_0$, vertex requirements $r(v)$, and edge capacity $c$, find a spanning tree $T$ rooted at $v_0$ minimizing $sum_(e in T) w(e)$ subject to: for each edge $e$ in $T$, the sum of requirements in the subtree on the non-root side is at most $c$.
][
  Minimum Capacitated Spanning Tree (ND5) @garey1979. NP-hard in the strong sense, even with unit requirements and capacity 3.
]

#reduction-rule("MinimumCapacitatedSpanningTree", "ILP")[
  Binary edge selectors $y_e$ plus directed requirement-flow variables. Flow conservation routes each vertex's requirement to the root; capacity constraints bound flow per edge.
][
  _Construction._ $3m$ variables: $m$ edge selectors + $2m$ directed flow. Tree cardinality, binary bounds, flow conservation (each non-root vertex generates $r(v)$ flow toward root), flow-edge linking, and per-edge capacity bounds ($<= c$). Minimize $sum_e w(e) dot y_e$.

  _Correctness._ ($arrow.r.double$) A feasible capacitated spanning tree induces a valid flow. ($arrow.l.double$) A feasible ILP solution encodes a spanning tree satisfying capacity constraints.

  _Solution extraction._ First $m$ variables (edge selectors) give the source configuration.
]

// MinimumMatrixCover → ILP (problem-def already exists above, just need reduction-rule)
#reduction-rule("MinimumMatrixCover", "ILP")[
  McCormick linearization of the quadratic form: $n$ sign variables $x_i$ plus $n(n-1)/2$ auxiliary variables $y_(i j)$ linearizing products $x_i x_j$. Three McCormick constraints per pair. Minimize the linearized objective.
][
  _Construction._ $n + n(n-1)/2$ binary variables and $3 dot n(n-1)/2$ constraints. For each pair $i < j$: $y_(i j) <= x_i$, $y_(i j) <= x_j$, $y_(i j) >= x_i + x_j - 1$. The objective encodes $sum_(i,j) a_(i j) dot f(i) dot f(j)$ via $f(i) = 2x_i - 1$.

  _Correctness._ ($arrow.r.double$) Any sign assignment $f$ maps to $x_i = (f(i)+1)/2$ with matching quadratic form value. ($arrow.l.double$) McCormick ensures $y_(i j) = x_i x_j$ at optimality.

  _Solution extraction._ First $n$ variables (sign variables) give the source configuration.
]

// === Non-ILP reduction rules (issue #974) ===

#reduction-rule("ILP", "ILP")[
  Integer variables with finite upper bounds are replaced by groups of binary variables using truncated binary encoding. Feasibility-Based Bound Tightening (FBBT) first infers per-variable upper bounds $U_i$ from the constraint system; each integer variable $x_i in {0, dots, U_i}$ is then encoded as $x_i = sum_(j=0)^(K_i - 1) w_(i j) y_(i j)$ where $y_(i j) in {0, 1}$ and the weights are powers of two with a truncated final weight ensuring $sum w_(i j) = U_i$.
][
  _Construction._ Given an ILP over $n$ non-negative integer variables with constraints $A x <= b$ (and/or $>=, =$) and objective $c^top x$:
  (1) Run FBBT to infer upper bounds $U_1, dots, U_n$. If infeasible, return a trivially infeasible binary ILP. If unbounded, fall back to $U_i = 2^(31) - 1$.
  (2) For each $x_i$, compute $K_i = ceil(log_2(U_i + 1))$ and weights $w_(i 0) = 1, w_(i 1) = 2, dots, w_(i, K_i - 2) = 2^(K_i - 2), w_(i, K_i - 1) = U_i - (2^(K_i - 1) - 1)$.
  (3) Substitute $x_i = sum_j w_(i j) y_(i j)$ into every constraint and objective term, expanding each integer coefficient into a sum over binary coefficients.

  _Correctness._ ($arrow.r.double$) Any integer assignment $x_i in {0, dots, U_i}$ has a unique truncated binary representation $y_(i j)$ with $sum w_(i j) y_(i j) = x_i$, preserving all constraints and the objective value. ($arrow.l.double$) Any binary assignment $y_(i j) in {0,1}$ yields $x_i = sum w_(i j) y_(i j) in {0, dots, U_i}$, so the decoded integer assignment satisfies the original constraints.

  _Solution extraction._ For each source variable $x_i$, compute $x_i = sum_(j=0)^(K_i - 1) w_(i j) y_(i j)$ from the binary solution.
]

#reduction-rule("HamiltonianCircuit", "HamiltonianPath")[
  To decide whether $G = (V, E)$ contains a Hamiltonian circuit, we split an arbitrary vertex $v_0$ into two copies and attach a private pendant to each copy, forcing any Hamiltonian path in the expanded graph to enter through one pendant, traverse the original circuit, and exit through the other.
][
  _Construction._ Let $G = (V, E)$ with $n = |V|$ and $m = |E|$. Fix vertex $v_0 = 0$. Introduce three new vertices: a duplicate $v'$ of $v_0$, a pendant $s$, and a pendant $t$. Form $G' = (V', E')$ where $V' = V union {v', s, t}$, and $E'$ contains (i) every original edge of $E$; (ii) an edge ${v', u}$ for each neighbor $u$ of $v_0$ in $G$; (iii) the pendant edge ${s, v_0}$; and (iv) the pendant edge ${t, v'}$. Thus $|V'| = n + 3$ and $|E'| = m + deg(v_0) + 2$.

  _Correctness._ ($arrow.r.double$) Suppose $G$ has a Hamiltonian circuit $C = (v_0, u_1, u_2, dots, u_(n-1), v_0)$. The walk $s, v_0, u_1, u_2, dots, u_(n-1), v', t$ visits every vertex of $G'$ exactly once, so $G'$ has a Hamiltonian path. ($arrow.l.double$) Suppose $G'$ has a Hamiltonian path $P$. The pendants $s$ and $t$ have degree one, so $P$ must start at $s$ and end at $t$ (or vice versa), giving the form $s, v_0, pi_1, dots, pi_(n-1), v', t$. Because ${v', pi_(n-1)} in E'$, vertex $pi_(n-1)$ is a neighbor of $v_0$ in $G$. Hence $v_0, pi_1, dots, pi_(n-1), v_0$ is a Hamiltonian circuit in $G$.

  _Solution extraction._ Orient the path so $s$ is the start and $t$ the end. Drop $s$ and the last two elements $v', t$; the remaining sequence is the Hamiltonian circuit witness.
]

#reduction-rule("KClique", "SubgraphIsomorphism")[
  A $k$-clique in $G$ is precisely a subgraph of $G$ isomorphic to $K_k$. Constructing $K_k$ as the pattern and passing $G$ as the host reduces $k$-clique detection to a single subgraph-isomorphism query.
][
  _Construction._ Given a $k$-clique instance $(G, k)$ with $G = (V, E)$, $|V| = n$, $|E| = m$, build the subgraph-isomorphism instance $(H, P)$ where $H := G$ (host) and $P := K_k$ (pattern, with $k$ vertices and $binom(k, 2)$ edges).

  _Correctness._ ($arrow.r.double$) If $G$ contains a $k$-clique $C = {v_1, dots, v_k}$, the injection $f(i) := v_i$ is a subgraph isomorphism from $K_k$ into $G$. ($arrow.l.double$) If $f: V(K_k) -> V(G)$ is a subgraph isomorphism, the image ${f(0), dots, f(k-1)}$ is a set of $k$ pairwise-adjacent vertices, i.e., a $k$-clique.

  _Solution extraction._ The subgraph-isomorphism solution $c in {0, dots, n-1}^k$ gives the host vertex assigned to each pattern vertex. The $k$-clique indicator $x in {0,1}^n$ sets $x[c[i]] := 1$ for each $i$.
]

#reduction-rule("Partition", "MultiprocessorScheduling")[
  Each element $a_i$ becomes a task of length $a_i$ on $m = 2$ processors with deadline $D = floor(S / 2)$. A balanced partition exists iff a feasible schedule exists.
][
  _Construction._ Let $A = (a_1, dots, a_n)$ with total sum $S = sum_(i=1)^n a_i$. Set task lengths $ell_i = a_i$, number of processors $m = 2$, and deadline $D = floor(S / 2)$.

  _Correctness._ ($arrow.r.double$) If $A' subset.eq A$ has $sum_(i in A') a_i = S/2$, assign tasks in $A'$ to processor 0 and the rest to processor 1; both loads equal $S/2 = D$. ($arrow.l.double$) If a feasible schedule exists with both loads $<= D = floor(S/2)$, since both loads sum to $S$ and each is at most $floor(S/2)$, equality holds, giving a balanced partition.

  _Solution extraction._ The processor assignment $p_i in {0, 1}$ is the partition assignment directly.
]

#reduction-rule("HamiltonianPath", "IsomorphicSpanningTree")[
  A Hamiltonian path in $G$ is a spanning tree isomorphic to the path graph $P_n$. The reduction asks whether $G$ admits a spanning tree isomorphic to $P_n$.
][
  _Construction._ Given $G = (V, E)$ with $|V| = n$, the target instance is $(G, P_n)$ where $P_n$ is the path $0 - 1 - dots - (n-1)$ with $n - 1$ edges.

  _Correctness._ ($arrow.r.double$) A Hamiltonian path $v_0, dots, v_(n-1)$ defines a bijection $phi(i) = v_i$ mapping $P_n$ edges to $G$ edges, giving a spanning tree isomorphic to $P_n$. ($arrow.l.double$) A spanning tree isomorphic to $P_n$ via $phi$ yields the Hamiltonian path $phi(0), phi(1), dots, phi(n-1)$.

  _Solution extraction._ The mapping $c[i]$ (graph vertex assigned to tree vertex $i$) is the Hamiltonian path vertex ordering directly.
]

#reduction-rule("HamiltonianCircuit", "BottleneckTravelingSalesman")[
  Construct a complete weighted graph with weight 1 on edges of $G$ and weight 2 on non-edges. A Hamiltonian tour of bottleneck cost 1 exists iff $G$ has a Hamiltonian circuit.
][
  _Construction._ Let $G = (V, E)$ with $n = |V|$. Build $K_n$ with $w(u, v) = 1$ if ${u,v} in E$, else $w(u, v) = 2$.

  _Correctness._ ($arrow.r.double$) A Hamiltonian circuit in $G$ uses only weight-1 edges, giving bottleneck cost 1. ($arrow.l.double$) A tour with bottleneck $<= 1$ uses only weight-1 edges, which are exactly edges of $G$, so it is a Hamiltonian circuit.

  _Solution extraction._ Recover the vertex cycle order from the tour edge set.
]

#reduction-rule("KClique", "ConjunctiveBooleanQuery")[
  Introduce $k$ existential variables over the vertex set and require the edge relation $R(y_i, y_j)$ for every pair $i < j$. The query is satisfiable iff $G$ contains a $k$-clique.
][
  _Construction._ Given $G = (V, E)$ with $|V| = n$ and parameter $k$, set domain $D = V$, define binary relation $R = {(u,v), (v,u) : {u,v} in E}$, and form the conjunction $phi = and.big_(0 <= i < j < k) R(y_i, y_j)$ with $binom(k, 2)$ conjuncts.

  _Correctness._ ($arrow.r.double$) A $k$-clique ${v_0, dots, v_(k-1)}$ satisfies every conjunct since all pairs are adjacent. ($arrow.l.double$) A satisfying assignment gives $k$ vertices where every pair is in $R$, hence pairwise-adjacent.

  _Solution extraction._ The satisfying tuple $(d_0, dots, d_(k-1))$ gives vertex indices; set $x[d_i] = 1$ for the clique indicator.
]

#reduction-rule("ExactCoverBy3Sets", "StaffScheduling")[
  Each universe element becomes a time period requiring exactly one worker, each 3-element subset becomes a schedule active on those three periods, and the worker budget is $q = |X|/3$. A feasible assignment selects $q$ schedules covering every period exactly once.
][
  _Construction._ Let $(X, cal(C))$ with $|X| = 3q$. Set $m_p = 3q$ periods (one per element), requirement $r[i] = 1$ for all $i$, worker budget $W = q$. Each subset $S_j = {a, b, c}$ defines schedule $sigma_j$ with $sigma_j[i] = 1$ iff $i in S_j$.

  _Correctness._ ($arrow.r.double$) An exact cover $I$ with $|I| = q$ assigns one worker per selected schedule, covering each period exactly once. ($arrow.l.double$) A feasible assignment with $sum_j w_j = q$ and each period covered exactly once gives pairwise-disjoint schedules covering all of $X$.

  _Solution extraction._ Map $w in NN^m$ to $c in {0,1}^m$ by $c[j] = 1$ if $w[j] > 0$.
]

#reduction-rule("Satisfiability", "NAESatisfiability")[
  Introduce a fresh sentinel variable $s$ and append it to every clause. The sentinel forces NAE-SAT to simulate ordinary disjunction.
][
  _Construction._ Given SAT instance $phi$ with variables $x_1, dots, x_n$ and clauses $C_1, dots, C_m$, introduce $s$ (index $n+1$). For each clause $C_j = (ell_1 or dots or ell_k)$, form NAE clause $C'_j = (ell_1, dots, ell_k, s)$.

  _Correctness._ ($arrow.r.double$) Set $s = 0$. A satisfying assignment makes at least one $ell_i = 1$ per clause; with $s = 0$, the clause is neither all-true nor all-false. ($arrow.l.double$) If $beta$ NAE-satisfies $phi'$, let $v = beta(s)$. If $v = 0$, at least one literal per clause is true. If $v = 1$, flip all variables; by NAE symmetry the complement also works.

  _Solution extraction._ If $beta(s) = 0$, return $(beta(x_1), dots, beta(x_n))$. If $beta(s) = 1$, return $(1 - beta(x_1), dots, 1 - beta(x_n))$.
]

#reduction-rule("KSatisfiability", "MinimumVertexCover")[
  Each variable contributes a truth-setting edge; each clause contributes a satisfaction-testing triangle. The formula is satisfiable iff the graph has a vertex cover of size $n + 2m$.
][
  _Construction._ Given 3-CNF $phi$ with $n$ variables and $m$ clauses, construct $G = (V, E)$ with $|V| = 2n + 3m$. For each variable $x_i$: vertices $u_i$ (index $2i$) and $overline(u)_i$ (index $2i+1$) with edge $(u_i, overline(u)_i)$. For each clause $c_j$: triangle vertices $t^j_0, t^j_1, t^j_2$ at indices $2n + 3j, 2n+3j+1, 2n+3j+2$. Communication edges connect each $t^j_k$ to the literal vertex of its $k$-th literal.

  _Correctness._ ($arrow.r.double$) A satisfying assignment selects literal vertices ($n$ total) and two triangle vertices per clause ($2m$ total), covering all edges. ($arrow.l.double$) A cover of size $n + 2m$ must include exactly one literal vertex per variable and two triangle vertices per clause; the uncovered triangle vertex's communication edge forces the corresponding literal to be true.

  _Solution extraction._ For variable $x_i$, set $x_i = 1$ if the cover indicator at position $2i$ is 1.
]

#reduction-rule("Partition", "SequencingWithinIntervals")[
  A unit-length enforcer task pinned at $[floor(S/2), floor(S/2)+1)$ splits the timeline into two blocks. A valid schedule exists iff the elements partition into two equal-sum subsets.
][
  _Construction._ Let $A = {a_1, dots, a_n}$ with $S = sum a_i$ and $h = floor(S/2)$. Create $n+1$ tasks: element tasks with $r_i = 0$, $d_i = S+1$, $p_i = a_i$; enforcer task with $r = h$, $d = h+1$, $p = 1$.

  _Correctness._ ($arrow.r.double$) A balanced partition places one subset's tasks in $[0, h)$ and the other in $[h+1, S+1)$. ($arrow.l.double$) The enforcer at $[h, h+1)$ splits usable time into two blocks of size $h = S/2$; since element tasks fill both blocks exactly, the assignment gives a balanced partition.

  _Solution extraction._ Task $t_i$ starting at time $s_i$: assign to subset 0 if $s_i <= h$, else subset 1.
]

#reduction-rule("MinimumVertexCover", "MinimumFeedbackArcSet")[
  Each vertex $v$ splits into $v^"in"$ and $v^"out"$ joined by an internal arc weighted $w(v)$. Each edge becomes two crossing arcs weighted $M = 1 + sum_v w(v)$. The optimal FAS never includes crossing arcs; selecting internal arcs for cover vertices breaks every cycle.
][
  _Construction._ Given $(G, w)$ with $G = (V, E)$, $n = |V|$. Build directed graph $H$ on $2n$ nodes. Internal arcs $(v^"in", v^"out")$ with weight $w(v)$. For each ${u,v} in E$: crossing arcs $(u^"out", v^"in")$ and $(v^"out", u^"in")$ with weight $M = 1 + sum_(v in V) w(v)$.

  _Correctness._ ($arrow.r.double$) A vertex cover $S$ gives FAS $F = {(v^"in", v^"out") : v in S}$; every cycle through a crossing arc has at least one internal arc in $F$. ($arrow.l.double$) Since $M$ exceeds total internal weight, no crossing arc is in the optimal FAS. For each edge ${u,v}$, the 4-cycle through both internal and crossing arcs forces at least one internal arc into $F$.

  _Solution extraction._ Internal arcs at positions $0, dots, n-1$; the cover is $c[0 : n]$.
]

#reduction-rule("KSatisfiability", "KClique")[
  Assign one vertex per literal position $(j, p)$; connect vertices from different clauses whose literals are not contradictory. A $k$-clique selects one consistent true literal per clause.
][
  _Construction._ Given 3-CNF $phi = C_1 and dots.c and C_m$ over $n$ variables, construct $G = (V, E)$ with $V = {(j, p) : 1 <= j <= m, 1 <= p <= 3}$, $|V| = 3m$. Edge between $(j_1, p_1)$ and $(j_2, p_2)$ iff $j_1 != j_2$ and $ell_(j_1, p_1) != not ell_(j_2, p_2)$. Set $k = m$.

  _Correctness._ ($arrow.r.double$) A satisfying assignment picks one true literal per clause; these vertices form a clique since they span all clauses without contradiction. ($arrow.l.double$) A $k$-clique has exactly one vertex per clause group; the selected literals are pairwise non-contradictory, defining a satisfying assignment.

  _Solution extraction._ For selected vertex $v$: clause $j = floor(v / 3)$, position $p = v mod 3$. If literal $ell_(j,p) = x_i$ set $x_i = 1$; if $ell_(j,p) = not x_i$ set $x_i = 0$.
]

#reduction-rule("HamiltonianCircuit", "BiconnectivityAugmentation")[
  Start with the edgeless graph on $n$ vertices. Price original edges at cost 1 and non-edges at cost 2. A budget-$n$ augmentation is achievable iff $G$ has a Hamiltonian circuit (the only way to biconnect with $n$ weight-1 edges is a Hamiltonian cycle).
][
  _Construction._ Given $G = (V, E)$ with $n = |V|$, let $H = (V, emptyset)$. For every pair ${u, v}$: potential edge with weight 1 if ${u,v} in E$, else weight 2. Budget $B = n$.

  _Correctness._ ($arrow.r.double$) A Hamiltonian circuit selects $n$ weight-1 edges forming a 2-connected cycle, cost $= n$. ($arrow.l.double$) Budget $n$ with $>= n$ edges required (degree $>= 2$) forces exactly $n$ weight-1 edges (all from $E$), which must form a Hamiltonian cycle.

  _Solution extraction._ Walk the unique cycle from vertex 0 through selected edges to recover the circuit order.
]

#reduction-rule("HamiltonianCircuit", "StrongConnectivityAugmentation")[
  Start with the empty digraph on $n$ vertices. Weight-1 candidate arcs correspond to edges of $G$; weight-2 arcs for non-edges. A budget-$n$ augmentation that achieves strong connectivity must select exactly $n$ weight-1 arcs forming a directed Hamiltonian cycle.
][
  _Construction._ Given $G = (V, E)$ with $n = |V|$. Build $D = (V, emptyset)$. For every ordered pair $(u, v)$ with $u != v$: candidate arc with weight 1 if ${u,v} in E$, else weight 2. Budget $B = n$.

  _Correctness._ ($arrow.r.double$) A Hamiltonian circuit gives $n$ directed arcs of weight 1 forming a strongly-connected cycle. ($arrow.l.double$) Strong connectivity needs $>= n$ arcs; budget $n$ forces all weight 1, hence all from $E$, forming a single $n$-cycle.

  _Solution extraction._ Follow unique successors from vertex 0 to recover the Hamiltonian permutation.
]

#reduction-rule("HamiltonianCircuit", "StackerCrane")[
  Each vertex $v_i$ splits into $(v_i^"in", v_i^"out")$ with a mandatory directed arc of length 1. Undirected connector edges of length 1 encode original graph edges. A tour of cost $2n$ exists iff a Hamiltonian circuit exists.
][
  _Construction._ Given $G = (V, E)$ with $n = |V|$. Create $2n$ vertices: $v_i^"in" = 2i$, $v_i^"out" = 2i + 1$. Add $n$ mandatory arcs $(v_i^"in", v_i^"out")$ of length 1. For each ${v_i, v_j} in E$: connector edges $(v_i^"out", v_j^"in")$ and $(v_j^"out", v_i^"in")$ of length 1.

  _Correctness._ ($arrow.r.double$) A Hamiltonian circuit gives a tour serving arcs in circuit order, each inter-arc hop using one connector edge, total cost $2n$. ($arrow.l.double$) Cost $2n$ with $n$ arcs of cost 1 leaves exactly $n$ connector hops of cost 1 each; single-hop connections correspond to edges of $G$.

  _Solution extraction._ The arc service permutation directly encodes the Hamiltonian circuit vertex order.
]

#reduction-rule("HamiltonianCircuit", "RuralPostman")[
  Each vertex splits into two copies connected by a required edge. Original graph edges become connector edges. A minimum-cost tour of cost $2n$ traversing all required edges exists iff $G$ has a Hamiltonian circuit.
][
  _Construction._ Given $G = (V, E)$ with $n = |V|$. Build $H$ with $2n$ vertices: $v_i^a = 2i$, $v_i^b = 2i+1$. Required edges ${v_i^a, v_i^b}$ of weight 1 ($n$ total). For each ${v_i, v_j} in E$: connector edges ${v_i^b, v_j^a}$ and ${v_j^b, v_i^a}$ of weight 1.

  _Correctness._ ($arrow.r.double$) A Hamiltonian circuit $(v_(p_0), dots, v_(p_(n-1)))$ gives a tour traversing required edges and single connector edges, total $2n$. ($arrow.l.double$) Each required edge costs 1 ($n$ total); remaining budget $n$ for connectors forces single-hop connections corresponding to edges of $G$.

  _Solution extraction._ Identify used connector edges and follow the successor map from vertex 0.
]

#reduction-rule("Partition", "ShortestWeightConstrainedPath")[
  A layered digraph with include/exclude edges per element: include edge has length $a_i + 1$ and weight 1; exclude has length 1 and weight $a_i + 1$. A feasible path witnesses a balanced partition.
][
  _Construction._ Build layered graph $v_0, dots, v_n$ with $s = v_0$, $t = v_n$. For each element $a_i$: include edge with length $a_i + 1$, weight 1; exclude edge with length 1, weight $a_i + 1$. Weight bound $W = floor(S/2) + n$.

  _Correctness._ ($arrow.r.double$) A balanced partition uses include edges for one subset and exclude for the other, achieving feasible weight and minimum length. ($arrow.l.double$) The weight constraint forces included elements to sum to at least $ceil(S/2)$; minimizing length forces equality at $floor(S/2)$.

  _Solution extraction._ Element $i$ assigned to subset 0 if include edge selected, subset 1 if exclude edge selected.
]

#reduction-rule("MaximumIndependentSet", "IntegralFlowBundles")[
  Each vertex $v_i$ maps to a flow unit through an intermediate node; edge-bundle constraints cap combined outflow of adjacent pairs at 1, so feasible flow of value $k$ exists iff an independent set of size $>= k$ exists.
][
  _Construction._ Directed graph on $n + 2$ nodes: source $s$, intermediates $w_0, dots, w_(n-1)$, sink $t$. Arcs $a_i^"in" = (s, w_i)$ (index $2i$) and $a_i^"out" = (w_i, t)$ (index $2i+1$). Edge bundles ${a_i^"out", a_j^"out"}$ with capacity 1 for each ${v_i, v_j} in E$. Vertex bundles ${a_i^"in", a_i^"out"}$ with capacity 2. Flow requirement $R = 1$.

  _Correctness._ ($arrow.r.double$) An independent set $S$ gives flow $f_i = 1$ for $v_i in S$; edge bundles satisfied since $S$ is independent. ($arrow.l.double$) Feasible flow gives $f_i in {0,1}$; edge bundles force ${v_i : f_i = 1}$ to be independent.

  _Solution extraction._ Vertex $v_i$ in independent set iff target solution at arc index $2i+1$ is positive.
]

#let hc_qa = load-example("HamiltonianCircuit", "QuadraticAssignment")
#let hc_qa_sol = hc_qa.solutions.at(0)
#reduction-rule("HamiltonianCircuit", "QuadraticAssignment",
  example: true,
  example-caption: [Cycle graph $C_#hc_qa.source.instance.graph.num_vertices$ ($n = #hc_qa.source.instance.graph.num_vertices$, $|E| = #hc_qa.source.instance.graph.edges.len()$)],
  extra: [
    #pred-commands(
      "pred create --example " + problem-spec(hc_qa.source) + " -o hc.json",
      "pred reduce hc.json --to " + target-spec(hc_qa) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate hc.json --config " + hc_qa_sol.source_config.map(str).join(","),
    )

    *Step 1 -- Source instance.* The graph $G$ has $n = #hc_qa.source.instance.graph.num_vertices$ vertices and edges ${#hc_qa.source.instance.graph.edges.map(e => "(" + str(e.at(0)) + "," + str(e.at(1)) + ")").join(", ")}$, forming a cycle $C_#hc_qa.source.instance.graph.num_vertices$. The penalty weight is $omega = n + 1 = #(hc_qa.source.instance.graph.num_vertices + 1)$.

    *Step 2 -- Construction.* The cost matrix $C$ encodes a directed cycle on positions: $c[i][(i+1) mod #hc_qa.source.instance.graph.num_vertices] = 1$, all other entries 0. The distance matrix $D$ encodes graph adjacency: $d[k][l] = 1$ if ${k,l} in E$, $d[k][l] = #(hc_qa.source.instance.graph.num_vertices + 1)$ for non-edges, $d[k][k] = 0$. Both matrices are $#hc_qa.source.instance.graph.num_vertices times #hc_qa.source.instance.graph.num_vertices$, so the QAP has $n = #hc_qa.target.instance.cost_matrix.len()$ facilities and $n = #hc_qa.target.instance.distance_matrix.len()$ locations.

    *Step 3 -- Verify a solution.* The canonical Hamiltonian circuit visits vertices in order $gamma = (#hc_qa_sol.source_config.map(str).join(", "))$. The QAP permutation is the same: $(#hc_qa_sol.target_config.map(str).join(", "))$. The QAP cost is $sum_(i=0)^(n-1) c[i][(i+1) mod n] dot d[gamma(i)][gamma((i+1) mod n)]$. Since $gamma$ maps each position $i$ to vertex $i$, each consecutive pair $(gamma(i), gamma(i+1 mod n))$ is an edge in $G$, contributing $1 dot 1 = 1$. Total cost $= #hc_qa.source.instance.graph.num_vertices = n$ #sym.checkmark

    *Multiplicity:* The fixture stores one canonical witness. The cycle $C_#hc_qa.source.instance.graph.num_vertices$ has $#hc_qa.source.instance.graph.num_vertices$ rotations and 2 reflections, giving $2n = #(2 * hc_qa.source.instance.graph.num_vertices)$ distinct Hamiltonian circuits; the canonical one is the identity permutation.
  ],
)[
  Position-adjacency encoded in cost matrix $C$ (directed cycle on positions), graph-adjacency in distance matrix $D$ (1 for edges, $omega = n+1$ for non-edges). QAP optimum equals $n$ iff a Hamiltonian circuit exists.
][
  _Construction._ Let $G = (V, E)$ with $n = |V|$ and $omega = n + 1$. Cost matrix: $c[i][j] = 1$ if $j equiv i+1 space (mod n)$, else 0. Distance matrix: $d[k][l] = 0$ if $k = l$; 1 if ${k,l} in E$; $omega$ otherwise.

  _Correctness._ ($arrow.r.double$) A Hamiltonian circuit $v_0, dots, v_(n-1)$ with $gamma(i) = v_i$ gives cost $n$ (each consecutive pair is an edge). ($arrow.l.double$) Cost $n$ forces $d[gamma(i), gamma((i+1) mod n)] = 1$ for all $i$, meaning all consecutive pairs are edges.

  _Solution extraction._ The QAP permutation $gamma$ is the Hamiltonian circuit visit order directly.
]

#reduction-rule("HamiltonianPath", "ConsecutiveOnesSubmatrix")[
  The vertex-edge incidence matrix has the consecutive-ones property on a selected subset of $n-1$ columns iff those columns correspond to a Hamiltonian path.
][
  _Construction._ Given $G = (V, E)$ with $|V| = n$, $|E| = m$, build $n times m$ Boolean matrix $A$ with $A_(i,j) = 1$ iff vertex $i$ is an endpoint of edge $j$. Set bound $K = n - 1$.

  _Correctness._ ($arrow.r.double$) A Hamiltonian path's $n-1$ edges, ordered by path position, give each row at most two consecutive ones. ($arrow.l.double$) $n-1$ columns with the C1P property form a subgraph where each vertex has degree $<= 2$; with $n-1$ edges spanning $n$ vertices, this is a Hamiltonian path.

  _Solution extraction._ Selected columns identify edges; walk from a degree-1 vertex to recover the path ordering.
]

#reduction-rule("Partition", "BinPacking")[
  Items with sizes $a_i$ packed into 2 bins of capacity $floor(S/2)$. A valid 2-bin packing exists iff a balanced partition exists.
][
  _Construction._ Set item sizes $s_i = a_i$, bin capacity $C = floor(S/2)$, number of bins $k = 2$.

  _Correctness._ ($arrow.r.double$) A balanced partition with both subsets summing to $S/2$ gives a valid 2-bin packing. ($arrow.l.double$) Both bins summing to $S$ with capacity $floor(S/2)$ forces $S$ even and each bin holding exactly $S/2$.

  _Solution extraction._ Normalize bin labels: element $i$ in subset 0 if $b_i = b_0$, else subset 1.
]

#reduction-rule("ExactCoverBy3Sets", "MaximumSetPacking")[
  The identity map embeds exact cover as set packing: unit-weight 3-element subsets with a $3q$-element universe. A maximum packing of $q$ disjoint sets is an exact cover.
][
  _Construction._ Given $(X, cal(C))$ with $|X| = 3q$, the MaximumSetPacking instance is $(X, cal(C))$ with unit weights.

  _Correctness._ ($arrow.r.double$) An exact cover selects $q$ pairwise-disjoint sets, which is a maximum packing (no packing can exceed $q$ sets of size 3 over $3q$ elements). ($arrow.l.double$) A maximum packing of $q$ disjoint size-3 sets covers all $3q$ elements, hence is an exact cover.

  _Solution extraction._ The selection vector is unchanged.
]

#reduction-rule("NAESatisfiability", "MaxCut")[
  Each variable $x_i$ becomes a literal pair $(v_i^+, v_i^-)$ joined by a heavy edge of weight $M = m+1$. Clause edges of weight 1 connect literal vertices within each clause. The optimal cut separates each literal pair and has at least one literal on each side per clause, encoding NAE-satisfiability.
][
  _Construction._ Given $n$ variables and $m$ clauses. For each variable $x_i$: vertices $v_i^+ = 2(i-1)$ and $v_i^- = 2(i-1)+1$ with variable edge of weight $M = m + 1$. For each clause $C_j$: unit-weight edges between all pairs of literal vertices in $C_j$.

  _Correctness._ ($arrow.r.double$) A NAE-satisfying assignment places true literals on one side, false on the other; variable edges cross ($n dot M$ contribution), and each clause has literals on both sides. ($arrow.l.double$) Weight $M > m$ forces every literal pair to be separated; the remaining cut value requires each clause to have literals on both sides.

  _Solution extraction._ Set $x_i = top$ if $v_i^+$ is in side 1, else $x_i = bot$.
]

#reduction-rule("ThreePartition", "ResourceConstrainedScheduling")[
  Each element becomes a unit-length task requiring $a_i$ units of a shared resource with bound $B$. With 3 processors and deadline $m$, every slot receives exactly 3 tasks summing to $B$.
][
  _Construction._ Given $(S, B)$ with $|S| = 3m$ and $B/4 < a_i < B/2$. Create $3m$ unit-length tasks with resource requirement $r_i = a_i$, $p = 3$ processors, resource bound $B$, deadline $D = m$.

  _Correctness._ ($arrow.r.double$) A valid 3-partition assigns each triple to a time slot; each slot uses exactly $B$ resource units. ($arrow.l.double$) $3m$ tasks in $m$ slots with $p = 3$: every slot has exactly 3 tasks. Resource bound $B$ with total $m B$: each slot sums to exactly $B$. Size constraints prevent fewer or more than 3 elements per slot.

  _Solution extraction._ Task $t_i$ assigned to slot $k$ means element $a_i$ belongs to group $k$.
]

#reduction-rule("ThreePartition", "SequencingWithReleaseTimesAndDeadlines")[
  $m-1$ filler tasks with tight release windows partition the timeline into $m$ slots of width $B$. Element tasks must fill these slots, and size constraints force exactly 3 elements per slot.
][
  _Construction._ Given $(S, B)$ with $|S| = 3m$. Create $3m$ element tasks with $p_i = a_i$, $r_i = 0$, $d_i = H$ where $H = m(B+1) - 1$. Add $m-1$ filler tasks with $p = 1$, $r_j = (j+1)B + j$, $d_j = (j+1)B + j + 1$.

  _Correctness._ ($arrow.r.double$) A valid 3-partition fills each slot of width $B$ with exactly 3 elements; fillers are placed in their tight windows. ($arrow.l.double$) Fillers pinned to unit windows create $m$ slots of width $B$; total element work $m B$ fills all slots exactly; size constraints force 3 elements per slot.

  _Solution extraction._ Decode the Lehmer code, simulate the schedule tracking start times; assign elements to groups by $floor("start" / (B+1))$.
]

#reduction-rule("ThreePartition", "SequencingToMinimizeWeightedTardiness")[
  High-weight filler tasks with tight deadlines force zero-tardiness schedules to leave exactly $m$ slots of width $B$ for element tasks. Size constraints ensure 3 elements per slot.
][
  _Construction._ Given $(S, B)$ with $|S| = 3m$. Horizon $H = m B + (m-1)$, filler weight $W_f = m B + 1$. Element tasks: $p_i = a_i$, $w_i = 1$, $d_i = H$. Filler tasks ($m-1$): $p = 1$, $w = W_f$, $d_j = (j+1)B + (j+1)$. Tardiness bound $K = 0$.

  _Correctness._ ($arrow.r.double$) A valid 3-partition schedules each triple in a slot between fillers, achieving zero tardiness. ($arrow.l.double$) Zero tardiness forces fillers to their tight deadlines, creating $m$ slots of width $B$; element tasks fill them exactly with 3 per slot.

  _Solution extraction._ Decode Lehmer code; scan left to right incrementing group index at each filler.
]

#reduction-rule("ThreePartition", "FlowShopScheduling")[
  On 3 machines, $m-1$ separator jobs with large machine-2 tasks force element jobs into $m$ windows of width $B$. Size constraints ensure 3 elements per window.
][
  _Construction._ Given $(S, B)$ with $|S| = 3m$ and $L = m B + 1$. Element jobs: task lengths $(a_i, a_i, a_i)$ on machines 1, 2, 3. Separator jobs ($m-1$): task lengths $(0, L, 0)$. Deadline $D$ computed from canonical schedule.

  _Correctness._ ($arrow.r.double$) A valid 3-partition interleaves element groups with separators, meeting the deadline. ($arrow.l.double$) Separators on machine 2 create $m$ windows of capacity $B$; element tasks fill exactly; size constraints give 3 per window.

  _Solution extraction._ Decode Lehmer code; walk job sequence incrementing group at each separator.
]

#reduction-rule("ThreePartition", "JobShopScheduling")[
  On 2 machines, $m-1$ long separator jobs on machine 0 force element jobs into $m$ windows of width $B$. Size constraints ensure 3 elements per window.
][
  _Construction._ Given $(S, B)$ with $|S| = 3m$, $L = m B + 1$, $D = m B + (m-1)L$. Element jobs ($3m$): two tasks $(text("machine") 0, a_i)$ then $(text("machine") 1, a_i)$. Separator jobs ($m-1$): single task $(text("machine") 0, L)$.

  _Correctness._ ($arrow.r.double$) A valid 3-partition interleaves element groups with separators on machine 0, achieving makespan $D$. ($arrow.l.double$) Separators of length $L > m B$ create impassable barriers; remaining budget $m B$ split into $m$ windows; size constraints force 3 elements per window.

  _Solution extraction._ Decode machine 0 Lehmer code; walk permutation incrementing group at each separator.
]

#reduction-rule("MaxCut", "MinimumCutIntoBoundedSets")[
  Invert edge weights relative to $w_"max"$ on a complete graph $K_N$ with $N = 2n'$. A minimum balanced bisection in the inverted graph corresponds to a maximum cut in the original.
][
  _Construction._ Given $G = (V, E, w)$ with $n = |V|$. Set $n' = n + (n mod 2)$, $N = 2n'$, $w_"max" = 1 + max_(e in E) w(e)$. Build $K_N$ with $tilde(w)(i,j) = w_"max" - w(i,j)$ for edges in $E$, else $w_"max"$. Designate $s = n'$, $t = n' + 1$, bound $b = n'$.

  _Correctness._ ($arrow.r.double$) A max-cut extended to a balanced bisection gives a feasible target instance. ($arrow.l.double$) Minimizing $tilde(w)$-cut cost is equivalent to maximizing original weight crossing the cut, since $tilde(w) = w_"max" - w$.

  _Solution extraction._ Return the first $n$ entries of the target assignment.
]

#pagebreak()
#bibliography("references.bib", style: "ieee")
