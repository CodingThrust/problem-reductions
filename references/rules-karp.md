# Rules in Karp (PLDI 2022)

**Source:** [Karp: A Language for NP Reductions](https://doi.org/10.1145/3519939.3523732), Chenhao Zhang, Jason D. Hartline, Christos Dimoulas, PLDI 2022.

**Repository:** `pkgref/karp/`

Karp is a Racket DSL for programming and random-testing Karp (many-one) reductions between NP decision problems. The paper evaluates expressiveness by implementing 25 reductions from Kleinberg & Tardos's algorithms textbook (5 in-text examples + 20 end-of-chapter exercises). Only one reduction (3-SAT → Independent Set) is included in the open-source repository; the rest are described in the paper's Table 1.

## Implemented Reductions (Table 1)

| # | Source | Target | Key Features |
|---|--------|--------|-------------|
| 1 | 3-SAT | 3D-Matching | Set |
| 2 | 3-SAT | Directed-Hamiltonian-Cycle | Graph, Connectivity |
| 3 | ? | Directed-Edge-Disjoint-Paths | Graph, Connectivity |
| 4 | ? | Fully-Compatible-Configuration | Graph, Mapping |
| 5 | 3-SAT | Graph-Coloring | Graph, Mapping |
| 6 | 3-SAT | Independent-Set | Graph |
| 7 | ? | Low-Diameter-Clustering | Mapping |
| 8 | ? | Plot-Fulfillment | Graph, Path |
| 9 | ? | Winner-Determination-for-Combinatorial-Auctions | Set, Mapping |
| 10 | ? | Diverse-Subset | Set, Mapping |
| 11 | ? | Resource-Reservation | Set |
| 12 | ? | Strongly-Independent-Set | Graph |
| 13 | Independent-Set | Vertex-Cover | Graph |
| 14 | ? | Independent-Set | Graph, Mapping |
| 15 | ? | (a,b)-Skeleton | Set, Graph |
| 16 | ? | 2-Partition | Set |
| 17 | ? | Galactic-Shortest-Path | Mapping, Path |
| 18 | ? | Dominating-Set | Graph |
| 19 | ? | Nearby-Electromagnetic-Observation | Set, Mapping |
| 20 | ? | Feedback-Vertex-Set | Graph, Acyclicity |
| 21 | ? | Hitting-Set | Set |
| 22 | ? | Monotone-Satisfiability | CNF |
| 23 | Vertex-Cover | Set-Cover | Set |
| 24 | ? | Graphical-Steiner-Tree | Graph, Connectivity |
| 25 | ? | Strategic-Advertising | Set |

**Note:** `?` means the source problem is not specified in the extracted table — these are textbook exercises where students choose the source problem. The "Key Features" column indicates which Karp language features (set operations, graph library, mappings, connectivity/acyclicity predicates, CNF library) are needed.

## Detailed Reduction Algorithms

### 1. 3-SAT → Independent Set

**Source:** `pkgref/karp/example/3sat-to-iset.karp` (the only reduction with full source code in the repository)

**Algorithm:**

Given a 3-CNF formula φ with clauses C₁, C₂, ..., Cₘ:

1. **Vertices:** For each clause Cᵢ and each literal l in Cᵢ, create a vertex (l, i). This gives 3m vertices total (3 per clause).

2. **Conflict edges (E₁):** For every pair of vertices (l₁, i) and (l₂, j) where l₁ and l₂ are complementary literals (one is the negation of the other), add an edge. These edges prevent the independent set from containing contradictory assignments.

3. **Clause edges (E₂):** For every clause Cᵢ, add edges between all pairs of vertices created from that clause's literals. This forms a triangle within each clause's 3 vertices, forcing the independent set to pick at most one literal per clause.

4. **Threshold:** Set k = m (the number of clauses).

5. **Result:** The graph G = (V, E₁ ∪ E₂) with threshold k forms the Independent Set instance.

**Correctness argument (via certificate constructions):**

- **Forward (SAT assignment → independent set):** For each clause, pick the vertex corresponding to one satisfied literal. Since one literal per clause is selected and no two complementary literals are chosen, this gives an independent set of size m = k.

- **Backward (independent set → SAT assignment):** For each variable x, if the independent set contains a vertex whose first subscript is the positive literal of x, assign x = true; otherwise assign x = false. The independent set constraints guarantee consistency.

### 2. 3-SAT → Directed-Hamiltonian-Cycle

**Algorithm:** The classic gadget-based construction. For each variable, create a row of vertices that can be traversed left-to-right (true) or right-to-left (false). For each clause, create a triangle gadget. Stitch clause triangles into variable rows so that a Hamiltonian cycle exists iff the formula is satisfiable. The paper notably found a bug in a textbook version of this reduction using Karp's random testing.

### 3. 3-SAT → Graph-Coloring (3-Coloring)

**Algorithm:** Standard reduction using gadgets. Create a truth-setting component for each variable (two vertices connected by an edge, representing x and ¬x, plus a base vertex connected to both). Create a clause-satisfaction gadget for each clause (an OR-gadget subgraph). Connect components so that a valid 3-coloring exists iff the formula is satisfiable.

### 4. Independent-Set → Vertex-Cover

**Algorithm:** The complement reduction. Given a graph G = (V, E) and threshold k for Independent Set, create a Vertex Cover instance with the same graph G and threshold |V| − k. A set S is an independent set of size ≥ k iff V \ S is a vertex cover of size ≤ |V| − k.

### 5. Vertex-Cover → Set-Cover

**Algorithm:** Given a graph G = (V, E) and threshold k:
- Universe E = the set of edges
- For each vertex v, create a set Sᵥ = {edges incident to v}
- Family F = {Sᵥ | v ∈ V}
- Threshold K = k

A vertex cover of size k selects k vertices whose incident edges cover all edges, which directly corresponds to selecting k sets from F that cover the universe E.

### 6. ? → 2-Partition

**Algorithm:** Typically reduced from Subset-Sum. Given a set of integers and a target, construct a set where a partition into two equal-sum halves exists iff the original instance has a solution. The key challenge (noted in the paper's error analysis) is padding values to ensure all numbers are nonnegative.

### 7. ? → Hitting-Set

**Algorithm:** Typically reduced from Vertex-Cover or Set-Cover. Given a collection of sets and a budget k, find a set of ≤ k elements that "hits" (intersects) every set in the collection. This is the dual of Set-Cover.

### 8. ? → Dominating-Set

**Algorithm:** Typically reduced from Vertex-Cover. Given a graph G and threshold k, find a set of ≤ k vertices such that every vertex is either in the set or adjacent to a vertex in the set.

### 9. ? → Feedback-Vertex-Set

**Algorithm:** Typically reduced from Vertex-Cover or 3-SAT. Find a minimum set of vertices whose removal makes the graph acyclic. The paper notes this requires Karp's acyclicity predicates from the graph library.

### 10. ? → Strongly-Independent-Set

**Algorithm:** A variant of Independent Set with additional constraints. The paper's error analysis notes the common mistake of "no consideration for isolated vertices."

### 11. ? → Monotone-Satisfiability

**Algorithm:** Reduced from general SAT or 3-SAT. Monotone SAT restricts clauses to contain only positive or only negative literals. The reduction uses Karp's CNF library features.

### 12. 3-SAT → 3D-Matching

**Algorithm:** The classic reduction. Create element triples encoding variable assignments and clause satisfaction. A perfect 3D matching exists iff the formula is satisfiable. Uses Karp's set operations.

### 13. ? → Graphical-Steiner-Tree

**Algorithm:** Given a graph with weighted edges and a subset of required vertices (terminals), find a minimum-weight tree connecting all terminals. Typically reduced from 3-SAT or Vertex-Cover. Requires graph connectivity predicates.

## Problem Categories (from the paper)

The paper categorizes the 31 NP-hard problems from Kleinberg & Tardos into:

| Category | Count | Examples |
|----------|-------|---------|
| Set Problems | 10 | Set-Cover, Hitting-Set, 3D-Matching |
| Labeled Object Problems | 5 | Interval-Scheduling, Graph-3-Coloring |
| Basic Graph Problems | 6 | Independent-Set, Vertex-Cover, Dominating-Set |
| Path/Connectivity Problems | 7 | Hamiltonian-Cycle, Steiner-Tree, Edge-Disjoint-Paths |
| Beyond Connectivity | 3 | Graph partitioning problems (not expressible) |
| Unsupported Data Types | 4 | String/real-number problems (not expressible) |

## Notes

- The `?` sources in Table 1 are textbook exercises where students choose an appropriate NP-hard source problem
- 7 of 42 exercises cannot be expressed in Karp due to language limitations (unsupported data types, graph partition certificates with variable number of parts)
- Karp's random testing successfully found bugs in 10 intentionally incorrect reductions, including a bug in a published textbook reduction (3-SAT → Directed-Hamiltonian-Cycle)
