---
name: Problem
about: Propose a new problem type
title: "[Model] MultipleChoiceBranching"
labels: model
assignees: ''
---

## Motivation

MULTIPLE CHOICE BRANCHING (P87) from Garey & Johnson, A2 ND11. An NP-complete problem on directed graphs combining branching (arborescence) structure with partition constraints on arcs. The problem asks for a high-weight subset of arcs that forms an acyclic, in-degree-at-most-one subgraph (a branching/forest of arborescences) while respecting a partition constraint that at most one arc from each group is selected. Without the partition constraint, the problem reduces to maximum weight branching — a 2-matroid intersection problem solvable in polynomial time (Tarjan, 1977).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R32 (3SAT -> MULTIPLE CHOICE BRANCHING) via Garey and Johnson (unpublished)

## Definition

<!-- ⚠️ Unverified -->
**Name:** `MultipleChoiceBranching`
**Canonical name:** MULTIPLE CHOICE BRANCHING
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND11

**Mathematical definition:**

INSTANCE: Directed graph G = (V, A), a weight w(a) in Z^+ for each arc a in A, a partition of A into disjoint sets A_1, A_2, ..., A_m, and a positive integer K.
QUESTION: Is there a subset A' of A with sum_{a in A'} w(a) >= K such that no two arcs in A' enter the same vertex, A' contains no cycles, and A' contains at most one arc from each of the A_i, 1 <= i <= m?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |A| binary variables (one per arc), indicating whether the arc is selected
- **Per-variable domain:** {0, 1} — 0 means arc not selected, 1 means arc selected
- **Meaning:** Variable x_a = 1 means arc a is included in the subset A'. The constraints are: (1) for each vertex v, at most one arc entering v has x_a = 1 (in-degree constraint), (2) the selected arcs form an acyclic subgraph (no directed cycle), (3) for each partition group A_i, at most one arc has x_a = 1 (multiple choice constraint), and (4) the total weight of selected arcs is at least K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `MultipleChoiceBranching`
**Variants:** weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices |V| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs (u, v) meaning u -> v |
| `weights` | `Vec<W>` | Positive integer weight w(a) for each arc a |
| `partition` | `Vec<Vec<usize>>` | Partition of arc indices into groups A_1, ..., A_m |
| `threshold` | `W` | Weight threshold K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The "no two arcs enter the same vertex" and "no cycles" constraints together define a branching (a forest of in-arborescences).
- The partition constraint adds the "multiple choice" aspect on top of the branching structure.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** General case requires exponential time. A brute-force approach selects at most one arc from each partition group and checks the branching constraints, giving O(product of |A_i|) configurations. The number of branchings in a directed graph can be exponential.
- **NP-completeness:** NP-complete (Garey and Johnson, unpublished). Transformation from 3SAT. Remains NP-complete even if G is strongly connected and all weights are equal.
- **Polynomial special cases:**
  - If all A_i have |A_i| = 1 (no choice constraint), the problem becomes maximum weight branching, solvable in polynomial time via 2-matroid intersection (Tarjan, 1977; Edmonds, 1967).
  - If the graph is symmetric, the problem reduces to the "multiple choice spanning tree" problem, also a 2-matroid intersection problem solvable in polynomial time (Suurballe, 1975).
- **References:**
  - R. E. Tarjan (1977). "Finding optimum branchings." *Networks* 7, pp. 25--35.
  - J. W. Suurballe (1975). "Minimal spanning trees subject to disjoint arc set constraints."
  - J. Edmonds (1967). "Optimum branchings." *Journal of Research of the National Bureau of Standards* 71B, pp. 233--240.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), a weight w(a) in Z^+ for each arc a in A, a partition of A into disjoint sets A_1, A_2, ..., A_m, and a positive integer K.
QUESTION: Is there a subset A' in A with sum_{a in A'} w(a) >= K such that no two arcs in A' enter the same vertex, A' contains no cycles, and A' contains at most one arc from each of the A_i, 1 <= i <= m?

Reference: [Garey and Johnson, --]. Transformation from 3SAT.
Comment: Remains NP-complete even if G is strongly connected and all weights are equal. If all Ai have |Ai| = 1, the problem becomes simply that of finding a "maximum weight branching," a 2-matroid intersection problem that can be solved in polynomial time (e.g., see [Tarjan, 1977]). (In a strongly connected graph, a maximum weight branching can be viewed as a maximum weight directed spanning tree.) Similarly, if the graph is symmetric, the problem becomes equivalent to the "multiple choice spanning tree" problem, another 2-matroid intersection problem that can be solved in polynomial time [Suurballe, 1975].

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all subsets of arcs (at most one per partition group) and check branching + weight constraints.
- [x] It can be solved by reducing to integer programming — binary variables for each arc, constraints for in-degree, acyclicity (via ordering variables), partition groups, and weight threshold.
- [x] Other: For the special case without partition constraints, Edmonds' algorithm (maximum weight branching) runs in O(|V| * |A|) time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — valid branching exists):**
Directed graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 arcs:
- Arcs with weights: a0=(0->1, w=3), a1=(0->2, w=2), a2=(1->3, w=4), a3=(2->3, w=1), a4=(1->4, w=2), a5=(3->5, w=3), a6=(4->5, w=1), a7=(2->4, w=3)
- Partition: A_1 = {a0, a1} (arcs from vertex 0), A_2 = {a2, a3} (arcs to vertex 3), A_3 = {a4, a7} (arcs to vertex 4), A_4 = {a5, a6} (arcs to vertex 5)
- K = 10
- Solution: A' = {a0, a2, a7, a5} = {0->1 (w=3), 1->3 (w=4), 2->4 (w=3), 3->5 (w=3)}
  - Total weight = 3+4+3+3 = 13 >= K=10
  - In-degree check: vertex 1 entered by a0 only, vertex 3 by a2 only, vertex 4 by a7 only, vertex 5 by a5 only -- OK
  - Acyclicity: 0->1->3->5 and 2->4, no cycles -- OK
  - Partition: a0 from A_1, a2 from A_2, a7 from A_3, a5 from A_4 -- at most one per group -- OK
- Answer: YES

**Instance 2 (NO — no valid branching meets threshold):**
Directed graph G with 4 vertices {0, 1, 2, 3} and 4 arcs:
- Arcs with weights: a0=(0->1, w=2), a1=(1->2, w=2), a2=(2->3, w=2), a3=(3->1, w=2)
- Partition: A_1 = {a0, a3} (arcs entering vertex 1), A_2 = {a1, a2}
- K = 6
- From A_1, select at most one: a0 or a3. From A_2, select at most one: a1 or a2.
- Best acyclic branching: a0 + a1 (weight 4), or a0 + a2 (but a2=2->3, need path from 0 through something), or a3 + a2 (but 3->1 and 2->3 form a cycle if both selected... actually no: a3=3->1, a2=2->3. Selected arcs: 3->1 and 2->3. Check cycle: 2->3->1, no cycle back to 2.)
- Max weight achievable = 4 < K=6
- Answer: NO
