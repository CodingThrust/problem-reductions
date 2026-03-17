---
name: Problem
about: Propose a new problem type
title: "[Model] ChinesePostmanForMixedGraphs"
labels: model
assignees: ''
---

## Motivation

CHINESE POSTMAN FOR MIXED GRAPHS (P101) from Garey & Johnson, A2 ND25. A fundamental problem in combinatorial optimization on mixed graphs (graphs containing both directed arcs and undirected edges). While the Chinese Postman Problem is polynomial-time solvable on purely undirected or purely directed graphs, the mixed case is NP-complete (Papadimitriou, 1976). It is a target in the reduction from 3SAT (R46).

**Associated rules:**
- R46: 3SAT -> CHINESE POSTMAN FOR MIXED GRAPHS (incoming)

<!-- ⚠️ Unverified: AI-collected rule associations -->

## Definition

**Name:** `ChinesePostmanForMixedGraphs`
<!-- ⚠️ Unverified -->
**Canonical name:** CHINESE POSTMAN FOR MIXED GRAPHS (also: Mixed Chinese Postman Problem, MCPP)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND25

**Mathematical definition:**

INSTANCE: Mixed graph G = (V, A, E), where A is a set of directed edges (arcs) and E is a set of undirected edges on V, length l(e) ∈ Z_0+ for each e ∈ A∪E, bound B ∈ Z+.
QUESTION: Is there a cycle in G that includes each directed and undirected edge at least once, traversing directed edges only in the specified direction, and that has total length no more than B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |A| + |E| integer variables — the number of times each arc/edge is traversed (each at least once; arcs only in their specified direction, undirected edges in either direction).
- **Per-variable domain:** {1, 2, 3, ...} — each edge/arc must be traversed at least once, and may be traversed additional times.
- **Meaning:** A satisfying assignment specifies the traversal multiplicity for each arc and the traversal direction/multiplicity for each undirected edge, such that the resulting multigraph is Eulerian (balanced in-/out-degrees at each vertex) and the total cost is ≤ B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `ChinesePostmanForMixedGraphs`
**Variants:** weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices |V| |
| `arcs` | `Vec<(usize, usize, W)>` | Directed edges (u, v, length) in A |
| `edges` | `Vec<(usize, usize, W)>` | Undirected edges {u, v, length} in E |
| `bound` | `W` | The total length bound B |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The cycle must traverse every arc in its specified direction and every undirected edge in at least one direction, possibly traversing some arcs/edges multiple times.
- The problem reduces to finding a minimum-cost set of additional arcs/edges to add to make the mixed graph Eulerian (balanced in-degree equals out-degree at every vertex).
- Polynomial-time solvable when A = ∅ (undirected Chinese Postman, solved by T-join/matching) or E = ∅ (directed Chinese Postman, solved by min-cost flow).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Exact solutions via Integer Linear Programming (ILP) formulations with branch-and-bound. The ILP determines the minimum-cost additional edge/arc duplications to make the mixed graph Eulerian. Can handle instances with up to ~100 edges in practice, but worst-case exponential.
- **NP-completeness:** NP-complete (Papadimitriou, 1976, "On the complexity of edge traversing"). Remains NP-complete even if all edge lengths are equal (unit lengths), G is planar, and the maximum vertex degree is 3.
- **Special cases:** Polynomial-time solvable when A = ∅ (Edmonds and Johnson, 1973) or E = ∅ (standard min-cost flow).
- **References:**
  - C.H. Papadimitriou (1976). "On the complexity of edge traversing." *Journal of the ACM*, 23(3):544–554.
  - J. Edmonds and E.L. Johnson (1973). "Matching, Euler tours, and the Chinese postman." *Mathematical Programming*, 5:88–124.

## Extra Remark

**Full book text:**

INSTANCE: Mixed graph G = (V,A,E), where A is a set of directed edges and E is a set of undirected edges on V, length l(e) ∈ Z0+ for each e ∈ A∪E, bound B ∈ Z+.
QUESTION: Is there a cycle in G that includes each directed and undirected edge at least once, traversing directed edges only in the specified direction, and that has total length no more than B?

Reference: [Papadimitriou, 1976b]. Transformation from 3SAT.
Comment: Remains NP-complete even if all edge lengths are equal, G is planar, and the maximum vertex degree is 3. Can be solved in polynomial time if either A or E is empty (i.e., if G is either a directed or an undirected graph) [Edmonds and Johnson, 1973].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [x] It can be solved by reducing to integer programming — formulate as ILP to find minimum-cost additional edge/arc duplications making the mixed graph Eulerian.
- [x] Other: Branch-and-bound with LP relaxation; for small instances, enumerate possible orientations of undirected edges and solve the resulting directed Chinese Postman problems (each solvable in polynomial time via min-cost flow).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — postman tour within bound):**
Mixed graph G with 6 vertices {0, 1, 2, 3, 4, 5}:
- Arcs A (directed): (0→1, length 2), (2→3, length 1), (4→5, length 3)
- Edges E (undirected): {1,2, length 1}, {3,4, length 2}, {5,0, length 1}
- The graph forms a directed-undirected alternating cycle: 0→1—2→3—4→5—0
- Total length traversing each arc/edge exactly once: 2+1+1+2+3+1 = 10
- Bound B = 10
- Tour: 0→1→2→3→4→5→0, traversing each element once, total = 10 ≤ B
- Answer: YES

**Instance 2 (NO — postman tour exceeds bound):**
Mixed graph G with 6 vertices {0, 1, 2, 3, 4, 5}:
- Arcs A: (0→1, length 1), (1→0, length 1), (2→3, length 1)
- Edges E: {0,2, length 1}, {1,3, length 1}, {3,4, length 5}, {4,5, length 5}, {5,2, length 5}
- Bound B = 10
- Minimum traversal must cover all 3 arcs (cost 3) and all 4 edges (cost 12), giving a base cost of 15 before any required duplications to make the walk Eulerian. Since 15 > 10 = B, no feasible tour exists.
- Answer: NO
