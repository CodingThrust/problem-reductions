---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumFeedbackVertexSet"
labels: model
assignees: ''
---

## Motivation

FEEDBACK VERTEX SET (P18) from Garey & Johnson, A1.1 GT7. A classical NP-complete problem (Karp, 1972) that arises in deadlock resolution, circuit testing, and program verification. Removing a feedback vertex set from a directed graph yields a directed acyclic graph (DAG); the problem asks for the smallest such set. It is a key source problem for reductions and has deep algorithmic interest due to its connection to vertex cover on bidirected graphs.

## Definition

**Name:** <!-- ⚠️ Unverified --> `MinimumFeedbackVertexSet`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Directed Feedback Vertex Set (DFVS); also: Minimum Feedback Vertex Set
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT7

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≤ K such that V' contains at least one vertex from every directed cycle in G?

The optimization version (which is the natural Rust model) asks: find the minimum-size set V' ⊆ V such that G − V' is a directed acyclic graph.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V| binary variables (one per vertex)
- **Per-variable domain:** binary {0, 1} — whether vertex v ∈ V is included in the feedback vertex set
- **Meaning:** variable x_v = 1 if vertex v is selected into the FVS. The configuration (x_0, ..., x_{n-1}) encodes a candidate subset V' ⊆ V. The assignment is valid (a valid FVS) if for every directed cycle C in G, at least one vertex of C has x_v = 1.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MinimumFeedbackVertexSet`
**Variants:** graph topology (directed graph type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `DirectedGraph` | The directed graph G = (V, A) on which a feedback vertex set is sought |

**Notes:**
- This is an optimization (minimization) problem: `Metric = SolutionSize<i32>`, implementing `OptimizationProblem`.
- The objective is to minimize |V'|, the number of selected vertices.
- Only directed graphs (digraphs) are relevant; undirected FVS is a separate but also NP-complete problem (also listed in GJ).
- Key getter methods needed: `num_vertices()` (= |V|), `num_arcs()` (= |A|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Karp, 1972; transformation from VERTEX COVER).
- **Best known exact algorithm (directed):** O*(1.9977^n) — Razgon's branch-and-prune algorithm (ICTCS 2007). This was the first algorithm for directed FVS faster than the trivial O*(2^n) bound. It computes the maximum induced DAG (complement of minimum FVS) using measure-and-conquer, where vertices receive fractional weights to track the effective problem size.
- **Best known exact algorithm (undirected):** O*(1.7266^n) — improved measure-and-conquer algorithm (Fomin et al., improved by Xiao & Nagamochi, 2013). Earlier milestone: O*(1.7548^n) (Fomin, Grandoni, Kratsch, Algorithmica 2008).
- **FPT (parameterized by k):** O(4^k · k! · n^{O(1)}) — Directed FVS is fixed-parameter tractable. Chen et al. (2008) gave an O(4^k · k! · n^2) algorithm. FPT solvability of directed FVS was a long-standing open problem resolved by Razgon (2007) and Chen et al. (2008) independently.
- **References:**
  - R.M. Karp (1972). "Reducibility Among Combinatorial Problems." *Complexity of Computer Computations*, pp. 85–103. Plenum Press. Original NP-completeness proof.
  - I. Razgon (2007). "Computing Minimum Directed Feedback Vertex Set in O*(1.9977^n)." *Proceedings of ICTCS 2007*, pp. 70–81. First sub-2^n exact algorithm for directed FVS.
  - J. Chen, Y. Liu, S. Lu, B. O'Sullivan, I. Razgon (2008). "A Fixed-Parameter Algorithm for the Directed Feedback Vertex Set Problem." *Journal of the ACM*, 55(5), Article 21. FPT result.
  - F.V. Fomin, F. Grandoni, D. Kratsch (2008). "A measure and conquer approach for the analysis of exact algorithms." *Journal of the ACM*, 56(5):25. Undirected FVS O*(1.7548^n).

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** (none — FVS is a fundamental problem)
- **Known special cases:** Undirected Feedback Vertex Set (same problem on undirected graphs, also NP-complete), FVS on tournament graphs (solvable in O*(1.6181^n)), FVS on reducible graphs (polynomial-time solvable, Shamir 1977)
- **Restriction:** Polynomial-time solvable for reducible flow graphs (used in compiler optimization)

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≤ K such that V' contains at least one vertex from every directed cycle in G?
Reference: [Karp, 1972]. Transformation from VERTEX COVER.
Comment: Remains NP-complete for digraphs having no in- or out-degree exceeding 2, for planar digraphs with no in- or out-degree exceeding 3 [Garey and Johnson, ——], and for edge digraphs [Gavril, 1977a], but can be solved in polynomial time for reducible graphs [Shamir, 1977]. The corresponding problem for undirected graphs is also NP-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all 2^n subsets V' ⊆ V; for each candidate, run a cycle-detection DFS on G − V' and check if it is acyclic. Total time O(2^n · (n + m)).
- [x] It can be solved by reducing to integer programming. Introduce binary variable x_v for each vertex; minimize ∑x_v subject to: for each directed cycle C in G, ∑_{v ∈ C} x_v ≥ 1. (Exponentially many constraints, but can be handled via iterative constraint generation.)
- [x] Other: Razgon's O*(1.9977^n) exact algorithm; FPT algorithms parameterized by solution size k.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Directed graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 arcs:**
- Arcs: (0→1), (1→2), (2→0), (1→3), (3→4), (4→1), (2→5), (5→3), (3→2)
- (Two interleaved directed cycles sharing vertices 1, 2, 3)

Directed cycles present:
- C_1: 0→1→2→0 (length 3, using vertices {0,1,2})
- C_2: 1→3→4→1 (length 3, using vertices {1,3,4})
- C_3: 1→2→5→3→2→0→1 (longer cycle — note: actually 1→3→2→0→1 via 3→2)
- C_4: 2→5→3→2 (length 3, using vertices {2,3,5})
- (Additional cycles formed by combinations)

**Minimum FVS analysis:**
- Selecting vertex 1 breaks cycles C_1, C_2, and any cycle through vertex 1.
- Selecting vertex 2 breaks cycles C_1, C_4, and cycles through vertex 2.
- Selecting {1, 2} (size 2): vertex 1 breaks C_1 and C_2; vertex 2 breaks C_4. Check C_3 (2→5→3→2 — broken by vertex 2 ✓). All cycles broken.
- Can we do it with 1 vertex? Vertex 1 misses C_4: 2→5→3→2 does not pass through 1. Vertex 2 misses C_2: 1→3→4→1. No single vertex covers all cycles.
- **Minimum FVS = {1, 2}, size = 2.**

**Verification:**
After removing vertices {1, 2}, remaining graph has vertices {0, 3, 4, 5} and arcs: (3→4), (0→?) — no arcs remain from 0, and no arcs enter 0 from remaining vertices. Remaining arcs among {0,3,4,5}: (3→4) only. This is a DAG ✓.

**Budget:** K = 2 → answer is YES (FVS of size ≤ 2 exists). For K = 1 → answer is NO.
