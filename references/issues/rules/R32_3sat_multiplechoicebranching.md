---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MULTIPLE CHOICE BRANCHING"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'MULTIPLE CHOICE BRANCHING'
source_in_codebase: true
target_in_codebase: false
---

**Source:** 3SAT
**Target:** MULTIPLE CHOICE BRANCHING
<!-- ⚠️ Unverified: AI-generated motivation -->
**Motivation:** Establishes NP-completeness of MULTIPLE CHOICE BRANCHING via polynomial-time reduction from 3SAT. This reduction encodes Boolean satisfiability as an arc-selection problem on a directed graph, where variable assignments correspond to choosing arcs from partition groups, clause satisfaction is enforced by the branching (acyclicity + in-degree) constraints, and the weight threshold ensures enough clauses are satisfied. The reduction is notable because removing the partition constraint makes the problem polynomial (maximum weight branching via matroid intersection), showing that the multiple-choice constraint is the source of intractability.
**Reference:** Garey & Johnson, *Computers and Intractability*, ND11, p.208

## GJ Source Entry

> [ND11] MULTIPLE CHOICE BRANCHING
> INSTANCE: Directed graph G=(V,A), a weight w(a)∈Z^+ for each arc a∈A, a partition of A into disjoint sets A_1,A_2,...,A_m, and a positive integer K.
> QUESTION: Is there a subset A'⊆A with ∑_{a∈A'} w(a)≥K such that no two arcs in A' enter the same vertex, A' contains no cycles, and A' contains at most one arc from each of the A_i, 1≤i≤m?
> Reference: [Garey and Johnson, ——]. Transformation from 3SAT.
> Comment: Remains NP-complete even if G is strongly connected and all weights are equal. If all A_i have |A_i|=1, the problem becomes simply that of finding a "maximum weight branching," a 2-matroid intersection problem that can be solved in polynomial time (e.g., see [Tarjan, 1977]). (In a strongly connected graph, a maximum weight branching can be viewed as a maximum weight directed spanning tree.) Similarly, if the graph is symmetric, the problem becomes equivalent to the "multiple choice spanning tree" problem, another 2-matroid intersection problem that can be solved in polynomial time [Suurballe, 1975].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with variables x_1, ..., x_n and clauses C_1, ..., C_p (each clause having exactly 3 literals), construct a MULTIPLE CHOICE BRANCHING instance as follows:

1. **Variable gadgets:** For each variable x_i, create a pair of arcs representing the true and false assignments. These two arcs form a partition group A_i (|A_i| = 2). The "at most one arc from each A_i" constraint forces exactly one truth assignment per variable.

2. **Clause gadgets:** For each clause C_j = (l_1 OR l_2 OR l_3), create a vertex v_j (clause vertex). For each literal l_k in C_j, add an arc from the corresponding variable gadget vertex to v_j. The in-degree constraint ("no two arcs enter the same vertex") interacts with the variable arc choices.

3. **Graph structure:** Create a directed graph where:
   - There is a root vertex r.
   - For each variable x_i, there are vertices representing the positive and negative literal states, with arcs from the root to these vertices.
   - Clause vertices receive arcs from literal vertices corresponding to their literals.
   - Additional arcs connect the structure to ensure the branching (acyclicity) property encodes the dependency structure.

4. **Weights:** Assign weights to arcs such that selecting arcs corresponding to a satisfying assignment yields total weight >= K. Arcs entering clause vertices have weight 1, and K is set to p (the number of clauses), so all clauses must be "reached" by the branching.

5. **Partition groups:** A_1 through A_n correspond to variable choices (true/false arcs). Additional partition groups may encode auxiliary structural constraints.

**Key invariant:** The branching structure (acyclic, in-degree at most 1) enforces that the selected arcs form a forest of in-arborescences. Combined with the partition constraint (one arc per variable group), this forces a consistent truth assignment. The weight threshold K = p ensures every clause vertex is reached by at least one literal arc, corresponding to clause satisfaction.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in the 3SAT instance
- p = number of clauses (= `num_clauses`)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `O(n + p)` (variable, literal, and clause vertices plus root) |
| `num_arcs` | `O(n + 3*p)` (2 arcs per variable gadget + 3 arcs per clause for literals) |
| `num_partition_groups` (m) | `n` (one group per variable, plus possibly auxiliary groups) |
| `threshold` (K) | `p` (number of clauses) |

**Derivation:** Each variable contributes O(1) vertices and 2 arcs (for true/false). Each clause contributes 1 vertex and 3 incoming arcs (one per literal). The total is linear in the formula size.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small 3SAT instance to MULTIPLE CHOICE BRANCHING, solve the target with BruteForce (enumerate branching subsets respecting partition constraints), extract the variable assignments from the selected partition group arcs, verify the extracted assignment satisfies all clauses of the original 3SAT formula.
- Negative test: use an unsatisfiable 3SAT formula (e.g., all 8 clauses on 3 variables forming a contradiction), verify the target MCB instance has no branching meeting the weight threshold.
- Structural checks: verify that the constructed graph has the correct number of vertices, arcs, and partition groups; verify arc weights sum correctly.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability with k=3):**
Variables: x_1, x_2, x_3, x_4
Clauses (6 clauses):
- C_1 = (x_1 OR x_2 OR NOT x_3)
- C_2 = (NOT x_1 OR x_3 OR x_4)
- C_3 = (x_2 OR NOT x_3 OR NOT x_4)
- C_4 = (NOT x_1 OR NOT x_2 OR x_4)
- C_5 = (x_1 OR x_3 OR NOT x_4)
- C_6 = (NOT x_2 OR x_3 OR x_4)

Satisfying assignment: x_1 = T, x_2 = T, x_3 = T, x_4 = T
- C_1: x_1=T -> satisfied
- C_2: x_3=T -> satisfied
- C_3: NOT x_4=F, but x_2=T -> satisfied
- C_4: x_4=T -> satisfied
- C_5: x_1=T -> satisfied
- C_6: x_3=T -> satisfied

**Constructed target instance (MultipleChoiceBranching):**
Directed graph with vertices: root r, literal vertices {p1, n1, p2, n2, p3, n3, p4, n4}, clause vertices {c1, c2, c3, c4, c5, c6}.
Total: 1 + 8 + 6 = 15 vertices.

Arcs (with partition groups):
- Group A_1 (variable x_1): {r -> p1 (w=1), r -> n1 (w=1)} -- choose true or false for x_1
- Group A_2 (variable x_2): {r -> p2 (w=1), r -> n2 (w=1)}
- Group A_3 (variable x_3): {r -> p3 (w=1), r -> n3 (w=1)}
- Group A_4 (variable x_4): {r -> p4 (w=1), r -> n4 (w=1)}

Clause arcs (each in its own singleton group or ungrouped):
- p1 -> c1 (w=1), p2 -> c1 (w=1), n3 -> c1 (w=1) [for C_1]
- n1 -> c2 (w=1), p3 -> c2 (w=1), p4 -> c2 (w=1) [for C_2]
- p2 -> c3 (w=1), n3 -> c3 (w=1), n4 -> c3 (w=1) [for C_3]
- n1 -> c4 (w=1), n2 -> c4 (w=1), p4 -> c4 (w=1) [for C_4]
- p1 -> c5 (w=1), p3 -> c5 (w=1), n4 -> c5 (w=1) [for C_5]
- n2 -> c6 (w=1), p3 -> c6 (w=1), p4 -> c6 (w=1) [for C_6]

K = 6 + 4 = 10 (must select enough arcs to cover all clauses plus variable assignments).

**Solution mapping:**
- Select variable arcs: r->p1 (x_1=T), r->p2 (x_2=T), r->p3 (x_3=T), r->p4 (x_4=T) from groups A_1 through A_4.
- Select clause arcs (one entering each clause vertex, respecting in-degree 1):
  - p1 -> c1 (C_1 satisfied by x_1)
  - p3 -> c2 (C_2 satisfied by x_3)
  - p2 -> c3 (C_3 satisfied by x_2)
  - p4 -> c4 (C_4 satisfied by x_4)
  - p1 -> c5 (C_5 satisfied by x_1) -- but p1 already used for c1! In-degree constraint on c5 is OK (different vertex), but we need p1 to have out-arcs to both c1 and c5; as long as no two arcs ENTER the same vertex, this is fine. c1 entered by one arc, c5 entered by one arc.
  - p3 -> c6 (C_6 satisfied by x_3)
- Total selected arcs: 4 (variable) + 6 (clause) = 10 = K
- Branching check: root r has outgoing arcs to p1, p2, p3, p4; then p1->c1, p1->c5; p2->c3; p3->c2, p3->c6; p4->c4. This forms a forest rooted at r. No cycles. Each clause vertex has in-degree 1.
- Answer: YES


## References

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Tarjan, 1977]**: [`Tarjan1977`] Robert E. Tarjan (1977). "Finding optimum branchings". *Networks* 7, pp. 25–35.
- **[Suurballe, 1975]**: [`Suurballe1975`] James W. Suurballe (1975). "Minimal spanning trees subject to disjoint arc set constraints".
