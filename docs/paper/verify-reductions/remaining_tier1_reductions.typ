// Remaining Tier 1 Reduction Rules — 56 rules organized by source problem
// From issue #770, both models exist. Excludes the 34 verified in PR #992.

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#align(center)[
  #text(size: 18pt, weight: "bold")[Remaining Tier 1 Reduction Rules]

  #v(0.5em)
  #text(size: 12pt)[56 rules from issue \#770 — both models exist, not yet implemented]

  #v(0.3em)
  #text(size: 10pt, fill: gray)[Excludes the 34 verified reductions in PR \#992]
]

#v(1em)

*Status legend:*
#block(inset: 8pt, stroke: 0.5pt + gray, radius: 3pt, width: 100%)[
  #text(fill: green)[●] Type-incompatible — math verified, needs decision variant (6) \\
  #text(fill: red)[●] Refuted — incorrect construction (8) \\
  #text(fill: orange)[●] Blocked — needs original paper (3) \\
  #text(fill: purple)[●] Known defects in issue description (19) \\
  #text(fill: blue)[●] Not yet verified (20)
]

#v(1em)
#outline(indent: 1.5em, depth: 2)
#pagebreak()


= 3-DIMENSIONAL MATCHING


== #text(fill: orange)[●] 3-DIMENSIONAL MATCHING $arrow.r$ NUMERICAL 3-DIMENSIONAL MATCHING #text(size: 8pt, fill: gray)[(\#390)]


_Status: Blocked (needs original paper)_


```
**Source:** 3-DIMENSIONAL MATCHING **Target:** NUMERICAL 3-DIMENSIONAL MATCHING **Reference:** Garey & Johnson, SP16, p.224 ## Specialization Note This rule's source problem (3-DIMENSIONAL MATCHING / 3DM) is a specialization of SET PACKING (MaximumSetPacking). Implementation should wait until 3DM is available as a codebase model.
```


= 3-SATISFIABILITY


== #text(fill: blue)[●] 3-SATISFIABILITY $arrow.r$ MULTIPLE CHOICE BRANCHING #text(size: 8pt, fill: gray)[(\#243)]


_Status: Not yet verified_


```
--- name: Rule about: Propose a new reduction rule title: "[Rule] 3SAT to MULTIPLE CHOICE BRANCHING" labels: rule assignees: '' canonical_source_name: '3-SATISFIABILITY' canonical_target_name: 'MULTIPLE CHOICE BRANCHING'
```


== #text(fill: blue)[●] 3-SATISFIABILITY $arrow.r$ ACYCLIC PARTITION #text(size: 8pt, fill: gray)[(\#247)]


_Status: Not yet verified_


```
--- name: Rule about: Propose a new reduction rule title: "[Rule] 3SAT to ACYCLIC PARTITION" labels: rule assignees: '' canonical_source_name: '3-SATISFIABILITY' canonical_target_name: 'ACYCLIC PARTITION'
```


== #text(fill: blue)[●] 3-SATISFIABILITY $arrow.r$ CHINESE POSTMAN FOR MIXED GRAPHS #text(size: 8pt, fill: gray)[(\#260)]


_Status: Not yet verified_


```
--- name: Rule about: Propose a new reduction rule title: "[Rule] 3SAT to CHINESE POSTMAN FOR MIXED GRAPHS" labels: rule assignees: '' canonical_source_name: '3-SATISFIABILITY' canonical_target_name: 'CHINESE POSTMAN FOR MIXED GRAPHS'
```


= 3SAT


== #text(fill: blue)[●] 3SAT $arrow.r$ PATH CONSTRAINED NETWORK FLOW #text(size: 8pt, fill: gray)[(\#364)]


_Status: Not yet verified_


```
**Source:** 3SAT **Target:** PATH CONSTRAINED NETWORK FLOW **Motivation:** Establishes NP-completeness of PATH CONSTRAINED NETWORK FLOW via polynomial-time reduction from 3SAT. This result is notable because standard (unconstrained) network flow is polynomial, but restricting flow to travel along specified paths makes the problem NP-complete, even when all capacities equal 1. **Reference:** Garey & Johnson, *Computers and Intractability*, ND34, p.215 ## GJ Source Entry > [ND34] PATH CONSTRAINED NETWORK FLOW > INSTANCE: Directed graph G=(V,A), specified vertices s and t, a capacity c(a)∈Z^+ for...
```


== #text(fill: blue)[●] 3SAT $arrow.r$ INTEGRAL FLOW WITH HOMOLOGOUS ARCS #text(size: 8pt, fill: gray)[(\#365)]


_Status: Not yet verified_


```
**Source:** 3SAT **Target:** INTEGRAL FLOW WITH HOMOLOGOUS ARCS **Motivation:** Establishes NP-completeness of INTEGRAL FLOW WITH HOMOLOGOUS ARCS via polynomial-time reduction from 3SAT. The reduction shows that requiring equal flow on paired ("homologous") arcs makes integer network flow intractable, even with unit capacities. **Reference:** Garey & Johnson, *Computers and Intractability*, ND35, p.215 ## GJ Source Entry > [ND35] INTEGRAL FLOW WITH HOMOLOGOUS ARCS > INSTANCE: Directed graph G=(V,A), specified vertices s and t, capacity c(a)∈Z^+ for each a∈A, requirement R∈Z^+, set H⊆A×A of "ho...
```


== #text(fill: red)[●] 3SAT $arrow.r$ DISJOINT CONNECTING PATHS #text(size: 8pt, fill: gray)[(\#370)]


_Status: Refuted by /verify-reduction_


```
**Source:** KSatisfiability (3SAT) **Target:** DisjointConnectingPaths **Motivation:** Establishes NP-completeness of Disjoint Connecting Paths via polynomial-time reduction from 3SAT. This is a foundational result in network design theory: it shows that even the decision version of vertex-disjoint multi-commodity routing is intractable. The reduction originates from Lynch (1975) and is presented as Exercise 8.23 in Dasgupta, Papadimitriou & Vazirani (DPV). Adding this edge connects the satisfiability cluster to the graph-routing cluster, enabling any problem that reduces to 3SAT to reach Disj...
```


== #text(fill: blue)[●] 3SAT $arrow.r$ MAXIMUM LENGTH-BOUNDED DISJOINT PATHS #text(size: 8pt, fill: gray)[(\#371)]


_Status: Not yet verified_


```
**Source:** 3SAT **Target:** MAXIMUM LENGTH-BOUNDED DISJOINT PATHS **Motivation:** Establishes NP-completeness of MAXIMUM LENGTH-BOUNDED DISJOINT PATHS via polynomial-time reduction from 3SAT. This result by Itai, Perl, and Shiloach (1977/1982) shows that bounding the length of vertex-disjoint s-t paths makes the counting/optimization problem intractable, in contrast to the unbounded case which is solvable by network flow. **Reference:** Garey & Johnson, *Computers and Intractability*, ND41, p.217 ## GJ Source Entry > [ND41] MAXIMUM LENGTH-BOUNDED DISJOINT PATHS > INSTANCE: Graph G=(V,E), spec...
```


== #text(fill: blue)[●] 3SAT $arrow.r$ Rectilinear Picture Compression #text(size: 8pt, fill: gray)[(\#458)]


_Status: Not yet verified_


```
**Source:** 3SAT **Target:** Rectilinear Picture Compression **Motivation:** Establishes NP-completeness of RECTILINEAR PICTURE COMPRESSION via polynomial-time reduction from 3SAT. This reduction connects Boolean satisfiability to a geometric covering problem: it shows that determining the minimum number of axis-aligned rectangles needed to exactly cover the 1-entries of a binary matrix is computationally intractable. The result has implications for image compression, DNA array synthesis, integrated circuit manufacture, and access control list minimization. **Reference:** Garey & Johnson, *Com...
```


== #text(fill: blue)[●] 3SAT $arrow.r$ Consistency of Database Frequency Tables #text(size: 8pt, fill: gray)[(\#468)]


_Status: Not yet verified_


```
**Source:** 3SAT **Target:** Consistency of Database Frequency Tables **Motivation:** Establishes NP-completeness of Consistency of Database Frequency Tables via polynomial-time reduction from 3SAT. This result has practical implications for statistical database security: it shows that no polynomial-time algorithm can determine whether a set of published frequency tables can be used to "compromise" a database by deducing specific attribute values of individual records, unless P = NP. The reduction encodes Boolean variables as attribute values and clauses as frequency table constraints, so that...
```


== #text(fill: blue)[●] 3SAT $arrow.r$ Timetable Design #text(size: 8pt, fill: gray)[(\#486)]


_Status: Not yet verified_


```
**Source:** 3SAT **Target:** Timetable Design **Motivation:** 3SAT asks whether a Boolean formula in 3-CNF is satisfiable; TIMETABLE DESIGN asks whether craftsmen can be assigned to tasks across work periods subject to availability and requirement constraints. Even, Itai, and Shamir (1976) showed that even a very primitive version of the timetable problem is NP-complete via reduction from 3SAT, establishing that all common timetabling problems are intractable. This is the foundational hardness result for university and school scheduling. **Reference:** Garey & Johnson, *Computers and Intractab...
```


== #text(fill: red)[●] 3SAT $arrow.r$ NON-LIVENESS OF FREE CHOICE PETRI NETS #text(size: 8pt, fill: gray)[(\#920)]


_Status: Refuted by /verify-reduction_


```
**Source:** 3SAT (KSatisfiability with K=3) **Target:** NON-LIVENESS OF FREE CHOICE PETRI NETS (NonLivenessFreePetriNet) **Motivation:** Establishes NP-completeness of determining whether a free-choice Petri net can reach a deadlock state. This is a fundamental result in concurrency theory, showing that even the well-structured class of free-choice nets has intractable liveness analysis. The reduction from 3SAT encodes clause satisfaction as token flow through a Petri net, where a satisfying assignment corresponds to a live execution and an unsatisfiable formula forces a deadlock. NP membershi...
```


= CLIQUE


== #text(fill: purple)[●] CLIQUE $arrow.r$ PARTIALLY ORDERED KNAPSACK #text(size: 8pt, fill: gray)[(\#523)]


_Status: Known defects in issue description_


```
**Source:** CLIQUE **Target:** PARTIALLY ORDERED KNAPSACK **Motivation:** Establishes the NP-completeness (in the strong sense) of PARTIALLY ORDERED KNAPSACK by reducing from CLIQUE. The key insight is that the precedence constraints in the knapsack can encode graph structure: vertices and edges of the source graph become items with precedence relations, where selecting an edge-item requires both endpoint vertex-items to be included. The capacity and value parameters are tuned so that achieving the target value requires selecting exactly J vertex-items and all their induced edges, which corres...
```


= Clique


== #text(fill: purple)[●] Clique $arrow.r$ Minimum Tardiness Sequencing #text(size: 8pt, fill: gray)[(\#206)]


_Status: Known defects in issue description_


```
**Source:** MaximumClique **Target:** MinimumTardinessSequencing **Motivation:** Establishes NP-completeness of MinimumTardinessSequencing by encoding J-clique selection as a scheduling problem where meeting an early edge-task deadline forces exactly J vertex-tasks and J(J−1)/2 edge-tasks to be scheduled early — which is only possible if those tasks form a complete J-vertex subgraph. **Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.10, p.73 > **⚠️ On Hold — Decision vs Optimization mismatch** > > This reduction is a **Karp reduction between decision problems**: "Does G...
```


= DIRECTED TWO-COMMODITY INTEGRAL FLOW


== #text(fill: purple)[●] DIRECTED TWO-COMMODITY INTEGRAL FLOW $arrow.r$ UNDIRECTED TWO-COMMODITY INTEGRAL FLOW #text(size: 8pt, fill: gray)[(\#277)]


_Status: Known defects in issue description_


```
No description provided.
```


= ExactCoverBy3Sets


== #text(fill: red)[●] ExactCoverBy3Sets $arrow.r$ BoundedDiameterSpanningTree #text(size: 8pt, fill: gray)[(\#913)]


_Status: Refuted by /verify-reduction_


```
**Source:** ExactCoverBy3Sets (X3C) **Target:** BoundedDiameterSpanningTree **Motivation:** Establishes NP-completeness of BOUNDED DIAMETER SPANNING TREE for any fixed D >= 4 via transformation from X3C. The diameter constraint on spanning trees arises in communication network design where latency (hop count) must be bounded alongside total cost. The reduction shows that simultaneous optimization of weight and diameter is fundamentally hard, even with weights restricted to {1, 2}. **Reference:** Garey & Johnson, *Computers and Intractability*, ND4, p.206 ## GJ Source Entry > [ND4] BOUNDED DIAM...
```


= FEEDBACK EDGE SET


== #text(fill: blue)[●] FEEDBACK EDGE SET $arrow.r$ GROUPING BY SWAPPING #text(size: 8pt, fill: gray)[(\#454)]


_Status: Not yet verified_


```
**Source:** FEEDBACK EDGE SET **Target:** GROUPING BY SWAPPING **Motivation:** Establishes NP-completeness of GROUPING BY SWAPPING via polynomial-time reduction from FEEDBACK EDGE SET. This shows that the problem of sorting a string into grouped blocks (where all occurrences of each symbol are contiguous) using a minimum number of adjacent transpositions is computationally hard, connecting graph cycle structure to string rearrangement complexity. **Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.231 ## GJ Source Entry > [SR21] GROUPING BY SWAPPING > INSTANCE: Fin...
```


= GRAPH 3-COLORABILITY


== #text(fill: red)[●] GRAPH 3-COLORABILITY $arrow.r$ PARTITION INTO FORESTS #text(size: 8pt, fill: gray)[(\#843)]


_Status: Refuted by /verify-reduction_


```
**Source:** GRAPH 3-COLORABILITY **Target:** PARTITION INTO FORESTS **Motivation:** Establishes NP-completeness of PARTITION INTO FORESTS (vertex arboricity decision problem) by showing that any proper 3-coloring of a graph is also a valid partition into 3 forests, and conversely, when the graph is dense enough the only way to partition vertices into few acyclic induced subgraphs is via a proper coloring. **Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT14 ## GJ Source Entry > [GT14] PARTITION INTO FORESTS > INSTANCE: Graph G = (V,E), positive integer K ≤ |V|. > QUESTION:...
```


= Graph 3-Colorability


== #text(fill: blue)[●] Graph 3-Colorability $arrow.r$ Sparse Matrix Compression #text(size: 8pt, fill: gray)[(\#431)]


_Status: Not yet verified_


```
**Source:** Graph 3-Colorability **Target:** Sparse Matrix Compression **Motivation:** Establishes NP-completeness of SPARSE MATRIX COMPRESSION via polynomial-time reduction from GRAPH 3-COLORABILITY. The sparse matrix compression problem arises in practice when compactly storing sparse matrices (e.g., for DFA transition tables) by overlaying rows with compatible non-zero patterns using shift offsets. Even, Lichtenstein, and Shiloach showed the problem is NP-complete, even when the maximum shift is restricted to at most 2 (i.e., K=3). The reduction represents each vertex as a "tile" (a row pat...
```


== #text(fill: purple)[●] Graph 3-Colorability $arrow.r$ Conjunctive Query Foldability #text(size: 8pt, fill: gray)[(\#463)]


_Status: Known defects in issue description_


```
**Source:** Graph 3-Colorability **Target:** Conjunctive Query Foldability **Motivation:** Establishes NP-completeness of CONJUNCTIVE QUERY FOLDABILITY via polynomial-time reduction from GRAPH 3-COLORABILITY. This reduction connects graph coloring to database query optimization: graph 3-colorability is equivalent to the existence of a homomorphism from a graph to K_3, which is precisely the foldability (containment) condition for conjunctive queries. This foundational result by Chandra and Merlin (1977) demonstrates that optimizing conjunctive queries is inherently hard. **Reference:** Garey &...
```


= HAMILTONIAN CIRCUIT


== #text(fill: purple)[●] HAMILTONIAN CIRCUIT $arrow.r$ BOUNDED COMPONENT SPANNING FOREST #text(size: 8pt, fill: gray)[(\#238)]


_Status: Known defects in issue description_


```
--- name: Rule about: Propose a new reduction rule title: "[Rule] HAMILTONIAN CIRCUIT to BOUNDED COMPONENT SPANNING FOREST" labels: rule assignees: '' canonical_source_name: 'HamiltonianCircuit' canonical_target_name: 'BoundedComponentSpanningForest'
```


= Hamiltonian Path


== #text(fill: purple)[●] Hamiltonian Path $arrow.r$ Consecutive Block Minimization #text(size: 8pt, fill: gray)[(\#435)]


_Status: Known defects in issue description_


```
**Source:** Hamiltonian Path **Target:** Consecutive Block Minimization **Motivation:** Establishes NP-completeness of CONSECUTIVE BLOCK MINIMIZATION via polynomial-time reduction from HAMILTONIAN PATH. The key idea is to encode the adjacency structure of the graph as a binary matrix whose column permutation corresponds to a vertex ordering; a Hamiltonian path exists if and only if the columns can be permuted so that each row (representing a vertex's neighborhood) has a small number of consecutive 1-blocks. **Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230 ##...
```


== #text(fill: purple)[●] Hamiltonian Path $arrow.r$ Consecutive Sets #text(size: 8pt, fill: gray)[(\#436)]


_Status: Known defects in issue description_


```
**Source:** Hamiltonian Path **Target:** Consecutive Sets **Motivation:** Establishes NP-completeness of CONSECUTIVE SETS via polynomial-time reduction from HAMILTONIAN PATH. The reduction encodes the graph structure as a collection of subsets of an alphabet (representing vertex neighborhoods), and asks whether a short string can arrange the symbols so that each neighborhood appears as a consecutive block -- which is possible if and only if the vertex ordering corresponds to a Hamiltonian path. **Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230 ## GJ Source En...
```


= HamiltonianPath


== #text(fill: orange)[●] HamiltonianPath $arrow.r$ IsomorphicSpanningTree #text(size: 8pt, fill: gray)[(\#912)]


_Status: Blocked (needs original paper)_


```
**Source:** HamiltonianPath **Target:** IsomorphicSpanningTree **Motivation:** Establishes NP-completeness of ISOMORPHIC SPANNING TREE via a direct embedding from HAMILTONIAN PATH. When the target tree T is a path P_n, the problem IS Hamiltonian Path. This is one of the simplest reductions in the G&J catalog: the graph is unchanged, and only the tree parameter is constructed. The problem remains NP-complete for other tree types including full binary trees and 3-stars (Papadimitriou and Yannakakis 1978). **Reference:** Garey & Johnson, *Computers and Intractability*, ND8, p.207; Papadimitriou a...
```


= KSatisfiability


== #text(fill: purple)[●] KSatisfiability $arrow.r$ MaxCut #text(size: 8pt, fill: gray)[(\#166)]


_Status: Known defects in issue description_


```
**Source:** NAESatisfiability **Target:** MaxCut **Motivation:** Classic NP-completeness reduction connecting Boolean satisfiability to graph partitioning. The Not-All-Equal structure is the key: every satisfied NAE clause contributes exactly 2 triangle edges to the cut, while every unsatisfied clause (all literals equal) contributes 0. This clean characterization establishes MaxCut as NP-hard via NAE-3SAT. **Reference:** [Garey, Johnson & Stockmeyer, "Some simplified NP-complete graph problems," Theoretical Computer Science 1(3), 237–267 (1976).](https://doi.org/10.1016/0304-3975(76)90059-1) ...
```


= MAX CUT


== #text(fill: green)[●] MAX CUT $arrow.r$ OPTIMAL LINEAR ARRANGEMENT #text(size: 8pt, fill: gray)[(\#890)]


_Status: Type-incompatible (math verified, PR #996)_


```
**Source:** MAX CUT **Target:** OPTIMAL LINEAR ARRANGEMENT **Motivation:** Establishes NP-completeness of OPTIMAL LINEAR ARRANGEMENT by reduction from (SIMPLE) MAX CUT. This connects graph partitioning problems to graph layout/ordering problems, showing that minimizing total edge stretch in a linear layout is as hard as finding a maximum cut. **Reference:** Garey & Johnson, *Computers and Intractability*, A1.3, GT42; Garey, Johnson, Stockmeyer 1976 ## GJ Source Entry > GT42 OPTIMAL LINEAR ARRANGEMENT > INSTANCE: Graph G = (V,E), positive integer K. > QUESTION: Is there a one-to-one function f:...
```


= MINIMUM MAXIMAL MATCHING


== #text(fill: red)[●] MINIMUM MAXIMAL MATCHING $arrow.r$ MaximumAchromaticNumber #text(size: 8pt, fill: gray)[(\#846)]


_Status: Refuted by /verify-reduction_


```
**Source:** MINIMUM MAXIMAL MATCHING **Target:** ACHROMATIC NUMBER **Motivation:** This is the NP-completeness proof for Achromatic Number (GT5) in Garey & Johnson, established by Yannakakis and Gavril (1978). The reduction shows that determining whether a graph admits a complete proper coloring with at least K colors is at least as hard as finding a minimum maximal matching. **Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT5 ## GJ Source Entry > [GT5] ACHROMATIC NUMBER > INSTANCE: Graph G = (V,E), positive integer K ≤ |V|. > QUESTION: Does G have achromatic number K or g...
```


== #text(fill: red)[●] MINIMUM MAXIMAL MATCHING $arrow.r$ MinimumMatrixDomination #text(size: 8pt, fill: gray)[(\#847)]


_Status: Refuted by /verify-reduction_


```
**Source:** MINIMUM MAXIMAL MATCHING **Target:** MATRIX DOMINATION **Motivation:** This is the NP-completeness proof for Matrix Domination (MS12) in Garey & Johnson, established by Yannakakis and Gavril (1978). The reduction encodes the edge domination structure of a graph into a binary matrix domination problem. **Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS12 ## GJ Source Entry > [MS12]  MATRIX DOMINATION > INSTANCE:  An n×n matrix M with entries from {0,1}, and a positive integer K. > QUESTION:  Is there a set of K or fewer non-zero entries in M that dominate all oth...
```


= Minimum Cardinality Key


== #text(fill: purple)[●] Minimum Cardinality Key $arrow.r$ Prime Attribute Name #text(size: 8pt, fill: gray)[(\#461)]


_Status: Known defects in issue description_


```
**Source:** Minimum Cardinality Key **Target:** Prime Attribute Name **Motivation:** Establishes NP-completeness of PRIME ATTRIBUTE NAME via polynomial-time reduction from MINIMUM CARDINALITY KEY. This reduction shows that even the simpler-sounding question "does attribute x belong to some candidate key?" is as hard as finding a minimum-size key. The result implies that determining whether a given attribute is prime (i.e., participates in at least one candidate key) is computationally intractable, with direct consequences for database normalization algorithms that need to distinguish prime fro...
```


= MinimumHittingSet


== #text(fill: purple)[●] MinimumHittingSet $arrow.r$ AdditionalKey #text(size: 8pt, fill: gray)[(\#460)]


_Status: Known defects in issue description_


```
**Source:** Hitting Set **Target:** Additional Key **Motivation:** Establishes NP-completeness of ADDITIONAL KEY via polynomial-time reduction from HITTING SET. This reduction shows that determining whether a relational schema admits a candidate key beyond a given set of known keys is computationally intractable. The result has implications for automated database normalization and schema design, since checking completeness of key enumeration is as hard as solving HITTING SET. **Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.232 ## GJ Source Entry > [SR27] ADDITI...
```


== #text(fill: purple)[●] MinimumHittingSet $arrow.r$ BoyceCoddNormalFormViolation #text(size: 8pt, fill: gray)[(\#462)]


_Status: Known defects in issue description_


```
**Source:** Hitting Set **Target:** Boyce-Codd Normal Form Violation **Motivation:** Establishes NP-completeness of BOYCE-CODD NORMAL FORM VIOLATION via polynomial-time reduction from HITTING SET. The reduction encodes the combinatorial structure of hitting a collection of subsets into the problem of finding a subset of attributes that violates the Boyce-Codd normal form condition with respect to a system of functional dependencies, linking classical set-cover-type problems to database schema design questions. **Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.233...
```


= MinimumVertexCover


== #text(fill: blue)[●] MinimumVertexCover $arrow.r$ ShortestCommonSupersequence #text(size: 8pt, fill: gray)[(\#427)]


_Status: Not yet verified_


```
**Source:** MinimumVertexCover **Target:** ShortestCommonSupersequence **Motivation:** Establishes NP-completeness of SHORTEST COMMON SUPERSEQUENCE via polynomial-time reduction from VERTEX COVER. The SCS problem asks for the shortest string containing each input string as a subsequence. Maier (1978) showed this is NP-complete even for alphabets of size 5 by encoding the "at least one endpoint" constraint of vertex cover through subsequence containment requirements. **Reference:** Garey & Johnson, *Computers and Intractability*, SR8, p.228. [Maier, 1978]. ## GJ Source Entry > [SR8] SHORTEST CO...
```


= OPTIMAL LINEAR ARRANGEMENT


== #text(fill: green)[●] OPTIMAL LINEAR ARRANGEMENT $arrow.r$ ROOTED TREE ARRANGEMENT #text(size: 8pt, fill: gray)[(\#888)]


_Status: Type-incompatible (math verified, PR #996)_


```
**Source:** OPTIMAL LINEAR ARRANGEMENT **Target:** ROOTED TREE ARRANGEMENT **Status: Blocked** — witness extraction is not possible with the current architecture (see below). **Motivation:** Establishes NP-completeness of ROOTED TREE ARRANGEMENT by reduction from OPTIMAL LINEAR ARRANGEMENT. Both problems concern arranging graph vertices to minimize total stretch of edges, but the tree arrangement variant embeds vertices into a rooted tree rather than a linear order, generalizing the layout structure. **Reference:** Garey & Johnson, *Computers and Intractability*, A1.3, GT45; Gavril 1977a ## GJ...
```


= Optimal Linear Arrangement


== #text(fill: purple)[●] Optimal Linear Arrangement $arrow.r$ Consecutive Ones Matrix Augmentation #text(size: 8pt, fill: gray)[(\#434)]


_Status: Known defects in issue description_


```
**Source:** Optimal Linear Arrangement **Target:** Consecutive Ones Matrix Augmentation **Motivation:** Establishes NP-completeness of CONSECUTIVE ONES MATRIX AUGMENTATION via polynomial-time reduction from OPTIMAL LINEAR ARRANGEMENT (GT42). The reduction encodes a vertex ordering problem as a matrix augmentation problem: given the vertex-edge incidence matrix of the graph, an optimal linear arrangement with low total edge length corresponds to a small number of 0-to-1 flips needed to achieve the consecutive ones property. **Reference:** Garey & Johnson, *Computers and Intractability*, Appendi...
```


== #text(fill: purple)[●] Optimal Linear Arrangement $arrow.r$ Sequencing to Minimize Weighted Completion Time #text(size: 8pt, fill: gray)[(\#472)]


_Status: Known defects in issue description_


```
**Source:** Optimal Linear Arrangement **Target:** Sequencing to Minimize Weighted Completion Time **Motivation:** Establishes NP-completeness (in the strong sense) of SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME by reducing from OPTIMAL LINEAR ARRANGEMENT. The key insight (Lawler, 1978; Lawler-Queyranne-Schulz-Shmoys, Lemma 4.14) is that the scheduling problem with arbitrary precedence constraints subsumes the linear arrangement problem: vertex jobs have unit processing time and weight proportional to d_max minus their degree, while zero-processing-time edge jobs with weight 2 are constrai...
```


= PARTITION


== #text(fill: purple)[●] PARTITION $arrow.r$ INTEGRAL FLOW WITH MULTIPLIERS #text(size: 8pt, fill: gray)[(\#363)]


_Status: Known defects in issue description_


```
**Source:** PARTITION **Target:** INTEGRAL FLOW WITH MULTIPLIERS **Motivation:** Establishes NP-completeness of INTEGRAL FLOW WITH MULTIPLIERS via polynomial-time reduction from PARTITION. The multipliers make the flow conservation constraints non-standard, which is precisely what encodes the subset-sum structure of PARTITION. Without multipliers (h(v)=1 for all v), the problem reduces to standard max-flow solvable in polynomial time. **Reference:** Garey & Johnson, *Computers and Intractability*, ND33, p.215 ## GJ Source Entry > [ND33] INTEGRAL FLOW WITH MULTIPLIERS > INSTANCE: Directed graph...
```


== #text(fill: green)[●] PARTITION $arrow.r$ K-th LARGEST m-TUPLE #text(size: 8pt, fill: gray)[(\#395)]


_Status: Type-incompatible (math verified, PR #996)_


```
**Source:** PARTITION **Target:** K-th LARGEST m-TUPLE **Motivation:** Establishes NP-hardness of K-th LARGEST m-TUPLE via polynomial-time reduction from PARTITION. The K-th LARGEST m-TUPLE problem generalizes selection in Cartesian products of integer sets, asking whether at least K m-tuples from X_1 × ... × X_m have total size at least B. This reduction, due to Johnson and Mizoguchi (1978), demonstrates that even the threshold-counting version of the Cartesian product selection problem is computationally hard. Like K-th LARGEST SUBSET, this problem is PP-complete and not known to be in NP. *...
```


= Partition


== #text(fill: orange)[●] Partition $arrow.r$ Sequencing with Deadlines and Set-Up Times #text(size: 8pt, fill: gray)[(\#474)]


_Status: Blocked (needs original paper)_


```
**Source:** Partition **Target:** Sequencing with Deadlines and Set-Up Times **Motivation:** PARTITION asks whether a multiset of integers can be split into two equal-sum halves; SEQUENCING WITH DEADLINES AND SET-UP TIMES asks whether tasks from different "compiler" classes can be ordered on a single processor — respecting class-switch set-up times — so that every task meets its deadline. By encoding the two halves of a PARTITION instance as two compiler classes and setting deadlines and set-up times so that a feasible schedule exists only when the classes can be interleaved with balanced tota...
```


= Partition / 3-Partition


== #text(fill: purple)[●] Partition / 3-Partition $arrow.r$ Expected Retrieval Cost #text(size: 8pt, fill: gray)[(\#423)]


_Status: Known defects in issue description_


```
**Source:** Partition / 3-Partition **Target:** Expected Retrieval Cost **Motivation:** Establishes NP-completeness of EXPECTED RETRIEVAL COST by encoding a PARTITION (or 3-PARTITION) instance as a record-allocation problem on a drum-like storage device. The key insight is that the latency cost function on a circular arrangement of m sectors captures the balance constraint of PARTITION: if records are distributed unevenly by probability weight across sectors, the expected rotational latency increases. When m = 2, the problem reduces exactly to deciding whether the records can be split into two...
```


= Register Sufficiency


== #text(fill: red)[●] Register Sufficiency $arrow.r$ Sequencing to Minimize Maximum Cumulative Cost #text(size: 8pt, fill: gray)[(\#475)]


_Status: Refuted by /verify-reduction_


```
**Source:** Register Sufficiency **Target:** Sequencing to Minimize Maximum Cumulative Cost **Motivation:** REGISTER SUFFICIENCY asks whether a DAG (representing a straight-line computation) can be evaluated using at most K registers; SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST asks whether tasks with precedence constraints can be ordered so that the running total of costs never exceeds a bound K. The reduction maps register "live ranges" to cumulative costs: loading a value into a register corresponds to a positive cost (consuming a register), and finishing with a value corresponds to a ne...
```


= SATISFIABILITY


== #text(fill: blue)[●] SATISFIABILITY $arrow.r$ UNDIRECTED FLOW WITH LOWER BOUNDS #text(size: 8pt, fill: gray)[(\#367)]


_Status: Not yet verified_


```
**Source:** SATISFIABILITY **Target:** UNDIRECTED FLOW WITH LOWER BOUNDS **Motivation:** Establishes NP-completeness of UNDIRECTED FLOW WITH LOWER BOUNDS via polynomial-time reduction from SATISFIABILITY. This is notable because directed flow with lower bounds is polynomial-time solvable, while the undirected variant with lower bounds is NP-complete even for a single commodity, even allowing non-integral flows. **Reference:** Garey & Johnson, *Computers and Intractability*, ND37, p.216 ## GJ Source Entry > [ND37] UNDIRECTED FLOW WITH LOWER BOUNDS > INSTANCE: Graph G=(V,E), specified vertices s...
```


= SET COVERING


== #text(fill: blue)[●] SET COVERING $arrow.r$ STRING-TO-STRING CORRECTION #text(size: 8pt, fill: gray)[(\#453)]


_Status: Not yet verified_


```
**Source:** SET COVERING **Target:** STRING-TO-STRING CORRECTION **Motivation:** Establishes NP-completeness of STRING-TO-STRING CORRECTION (with deletion and adjacent-symbol interchange only) via polynomial-time reduction from SET COVERING. This reduction, due to Wagner (1975), shows that the restricted edit distance problem with only swap and delete operations is computationally hard, even though the problem becomes polynomial-time solvable when additional operations (insert, change) are allowed or when only swaps are permitted. **Reference:** Garey & Johnson, *Computers and Intractability*,...
```


= Satisfiability


== #text(fill: blue)[●] Satisfiability $arrow.r$ IntegralFlowHomologousArcs #text(size: 8pt, fill: gray)[(\#732)]


_Status: Not yet verified_


```
## Source Satisfiability ## Target IntegralFlowHomologousArcs (to be implemented — see issue #292) ## Motivation - Establishes NP-hardness of the integer equal flow problem with homologous arcs, following the classical result by Sahni (1974) - Connects IntegralFlowHomologousArcs to the main reduction graph through the Satisfiability chain (reachable from 3-SAT); without this rule, IntegralFlowHomologousArcs is an orphan node - Provides a historically significant reduction demonstrating that network flow problems with equality constraints become NP-hard, in contrast to ordinary max-flow which i...
```


= SchedulingToMinimizeWeightedCompletionTime


== #text(fill: blue)[●] SchedulingToMinimizeWeightedCompletionTime $arrow.r$ ILP #text(size: 8pt, fill: gray)[(\#783)]


_Status: Not yet verified_


```
## Source SchedulingToMinimizeWeightedCompletionTime ## Target ILP (Integer Linear Programming), variant: i32 ## Motivation - Companion rule for #505 — enables ILP solving of scheduling instances - Natural extension of the existing SequencingToMinimizeWeightedCompletionTime → ILP reduction to the multi-processor case - Standard scheduling ILP formulation using assignment + ordering variables with big-M constraints
```


= VERTEX COVER


== #text(fill: green)[●] VERTEX COVER $arrow.r$ HAMILTONIAN CIRCUIT #text(size: 8pt, fill: gray)[(\#198)]


_Status: Type-incompatible (math verified, PR #996)_


```
**Source:** VERTEX COVER **Target:** HAMILTONIAN CIRCUIT **Motivation:** Establishes NP-completeness of HAMILTONIAN CIRCUIT by a gadget-based polynomial-time reduction from VERTEX COVER, enabling downstream reductions to HAMILTONIAN PATH, TSP, and other tour-finding problems. **Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.4, p.56-60 ## Reduction Algorithm > Theorem 3.4 HAMILTONIAN CIRCUIT is NP-complete > Proof: It is easy to see that HC E NP, because a nondeterministic algorithm need only guess an ordering of the vertices and check in polynomial time that all the re...
```


== #text(fill: purple)[●] VERTEX COVER $arrow.r$ MINIMUM CUT INTO BOUNDED SETS #text(size: 8pt, fill: gray)[(\#250)]


_Status: Known defects in issue description_


```
--- name: Rule about: Propose a new reduction rule title: "[Rule] VERTEX COVER to MINIMUM CUT INTO BOUNDED SETS" labels: rule assignees: '' canonical_source_name: 'VERTEX COVER' canonical_target_name: 'MINIMUM CUT INTO BOUNDED SETS'
```


== #text(fill: blue)[●] VERTEX COVER $arrow.r$ MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS #text(size: 8pt, fill: gray)[(\#374)]


_Status: Not yet verified_


```
**Source:** MinimumVertexCover **Target:** MinimumDummyActivitiesPert **Motivation:** Establishes NP-hardness of MinimumDummyActivitiesPert via polynomial-time reduction from MinimumVertexCover. This result by Krishnamoorthy and Deo (1979) shows that constructing an optimal PERT event network (activity-on-arc) with the fewest dummy activities is computationally intractable, motivating the development of heuristic algorithms for project scheduling. **Reference:** Garey & Johnson, *Computers and Intractability*, ND44, p.218 ## GJ Source Entry > [ND44] MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS...
```


== #text(fill: blue)[●] VERTEX COVER $arrow.r$ SET BASIS #text(size: 8pt, fill: gray)[(\#383)]


_Status: Not yet verified_


```
**Source:** VERTEX COVER **Target:** SET BASIS **Motivation:** Establishes NP-completeness of SET BASIS via polynomial-time reduction from VERTEX COVER. The reduction connects graph covering problems to set representation/compression problems, showing that finding a minimum-size collection of "basis" sets from which a given family of sets can be reconstructed via unions is computationally intractable. This result by Stockmeyer (1975) is one of the earliest NP-completeness proofs for set-theoretic problems outside the core Karp reductions. **Reference:** Garey & Johnson, *Computers and Intracta...
```


== #text(fill: purple)[●] VERTEX COVER $arrow.r$ COMPARATIVE CONTAINMENT #text(size: 8pt, fill: gray)[(\#385)]


_Status: Known defects in issue description_


```
**Source:** VERTEX COVER **Target:** COMPARATIVE CONTAINMENT **Motivation:** Establishes NP-completeness of COMPARATIVE CONTAINMENT via polynomial-time reduction from VERTEX COVER. The reduction, due to Plaisted (1976), encodes the vertex cover structure into weighted set containment: each vertex becomes an element of the universe, and edge-coverage constraints are translated into two collections of weighted subsets (R and S) such that a vertex cover of bounded size exists if and only if a subset Y of the universe achieves at least as much R-containment weight as S-containment weight. **Refere...
```


== #text(fill: green)[●] VERTEX COVER $arrow.r$ HAMILTONIAN PATH #text(size: 8pt, fill: gray)[(\#892)]


_Status: Type-incompatible (math verified, PR #996)_


```
**Source:** VERTEX COVER (MinimumVertexCover) **Target:** HAMILTONIAN PATH (HamiltonianPath) **Motivation:** Establishes NP-completeness of HAMILTONIAN PATH by composing the VC→HC reduction (Theorem 3.4) with a simple HC→HP modification. This two-step approach shows the path variant is NP-complete without requiring a fundamentally new gadget construction. The reduction is described in Section 3.1.4 of Garey & Johnson. **Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.1.4, p. 60; A1.3 GT39 ## GJ Source Entry > GT39 HAMILTONIAN PATH > INSTANCE: Graph G = (V,E). > QUESTION...
```


== #text(fill: green)[●] VERTEX COVER $arrow.r$ PARTIAL FEEDBACK EDGE SET #text(size: 8pt, fill: gray)[(\#894)]


_Status: Type-incompatible (math verified, PR #996)_


```
**Source:** VERTEX COVER (MinimumVertexCover) **Target:** PARTIAL FEEDBACK EDGE SET (PartialFeedbackEdgeSet) **Motivation:** Establishes NP-completeness of PARTIAL FEEDBACK EDGE SET (for any fixed cycle length bound L >= 3) by reduction from VERTEX COVER. This connects vertex-based covering to edge-based cycle-hitting, showing that even the restricted problem of hitting short cycles by removing edges is NP-hard. **Reference:** Garey & Johnson, *Computers and Intractability*, A1.1, GT9; Yannakakis 1978b ## GJ Source Entry > GT9 PARTIAL FEEDBACK EDGE SET > INSTANCE: Graph G = (V,E), positive int...
```


= Vertex Cover


== #text(fill: purple)[●] Vertex Cover $arrow.r$ Multiple Copy File Allocation #text(size: 8pt, fill: gray)[(\#425)]


_Status: Known defects in issue description_


```
**Source:** VERTEX COVER **Target:** MULTIPLE COPY FILE ALLOCATION **Motivation:** Establishes NP-completeness (in the strong sense) of MULTIPLE COPY FILE ALLOCATION by reduction from VERTEX COVER. The key insight is that placing file copies at vertices of a graph corresponds to choosing a vertex cover: each vertex in the cover stores a copy (incurring storage cost), and vertices not in the cover must access the nearest copy (incurring usage-weighted distance cost). By setting uniform usage and storage costs, the total cost is minimized exactly when the selected vertices form a minimum vertex ...
```


== #text(fill: blue)[●] Vertex Cover $arrow.r$ Longest Common Subsequence #text(size: 8pt, fill: gray)[(\#429)]


_Status: Not yet verified_


```
**Source:** MinimumVertexCover **Target:** LongestCommonSubsequence **Motivation:** Establishes NP-completeness of LONGEST COMMON SUBSEQUENCE (for an arbitrary number of strings) via polynomial-time reduction from VERTEX COVER. While LCS for two strings is solvable in O(n²) time by dynamic programming, Maier (1978) showed the problem becomes NP-complete for an unbounded number of strings. The reduction uses a vertex-alphabet encoding: each vertex becomes a symbol, each edge yields a constraint string that forbids both endpoints from appearing together in any common subsequence. **Reference:** ...
```


== #text(fill: purple)[●] Vertex Cover $arrow.r$ Minimum Cardinality Key #text(size: 8pt, fill: gray)[(\#459)]


_Status: Known defects in issue description_


```
**Source:** Vertex Cover **Target:** Minimum Cardinality Key **Motivation:** Establishes NP-completeness of MINIMUM CARDINALITY KEY via polynomial-time reduction from VERTEX COVER. This reduction bridges graph theory and relational database theory, showing that finding a minimum-size key for a relational schema (under functional dependencies) is as hard as finding a minimum vertex cover. The result implies that optimizing database key selection is computationally intractable in general. **Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.232 ## GJ Source Entry > [S...
```


== #text(fill: blue)[●] Vertex Cover $arrow.r$ Scheduling with Individual Deadlines #text(size: 8pt, fill: gray)[(\#478)]


_Status: Not yet verified_


```
**Source:** Vertex Cover **Target:** Scheduling with Individual Deadlines **Motivation:** VERTEX COVER asks for a subset of at most K vertices covering all edges; SCHEDULING WITH INDIVIDUAL DEADLINES asks whether unit-length tasks with a partial order and individual deadlines can be scheduled on m processors so every task meets its own deadline. The reduction encodes each graph edge as a precedence constraint and uses the deadline structure to force that at most K "vertex tasks" are scheduled early (before the remaining tasks), which corresponds to selecting a vertex cover. This establishes NP...
```


= X3C


== #text(fill: red)[●] X3C $arrow.r$ ACYCLIC PARTITION #text(size: 8pt, fill: gray)[(\#822)]


_Status: Refuted by /verify-reduction_


```
**Source:** X3C **Target:** ACYCLIC PARTITION **Motivation:** Establishes NP-completeness of ACYCLIC PARTITION via polynomial-time reduction from X3C. The reduction encodes the exact cover constraint into a directed graph partitioning problem where vertex weight bounds force groups of size 3, arc costs penalize splitting related vertices, and the acyclicity constraint encodes the covering requirement. This is the reduction cited in Garey & Johnson (ND15), attributed to their own unpublished work. **Reference:** Garey & Johnson, *Computers and Intractability*, ND15, p.209 ## GJ Source Entry > [...
```
