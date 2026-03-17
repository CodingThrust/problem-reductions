---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Register Sufficiency to Sequencing to Minimize Maximum Cumulative Cost"
labels: rule
assignees: ''
canonical_source_name: 'REGISTER SUFFICIENCY'
canonical_target_name: 'SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Register Sufficiency
**Target:** Sequencing to Minimize Maximum Cumulative Cost
**Motivation:** REGISTER SUFFICIENCY asks whether a DAG (representing a straight-line computation) can be evaluated using at most K registers; SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST asks whether tasks with precedence constraints can be ordered so that the running total of costs never exceeds a bound K. The reduction maps register "live ranges" to cumulative costs: loading a value into a register corresponds to a positive cost (consuming a register), and finishing with a value corresponds to a negative cost (freeing a register). The maximum number of simultaneously live registers equals the maximum cumulative cost, establishing NP-completeness of the scheduling problem.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.238

## GJ Source Entry

> [SS7] SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST
> INSTANCE: Set T of tasks, partial order < on T, a "cost" c(t) E Z for each t E T (if c(t) < 0, it can be viewed as a "profit"), and a constant K E Z.
> QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and which has the property that, for every task t E T, the sum of the costs for all tasks t' with σ(t') <= σ(t) is at most K?
> Reference: [Abdel-Wahab, 1976]. Transformation from REGISTER SUFFICIENCY.
> Comment: Remains NP-complete even if c(t) E {-1,0,1} for all t E T. Can be solved in polynomial time if < is series-parallel [Abdel-Wahab and Kameda, 1978], [Monma and Sidney, 1977].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a REGISTER SUFFICIENCY instance: a directed acyclic graph G = (V, A) with n = |V| vertices and a positive integer K, construct a SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST instance as follows.

1. **Tasks from vertices:** For each vertex v ∈ V, create a task t_v.

2. **Precedence constraints:** The partial order on tasks mirrors the DAG edges: if (u, v) ∈ A (meaning u depends on v, i.e., v must be computed before u can consume it), then t_v < t_u in the schedule (t_v must be scheduled before t_u).

3. **Cost assignment:** For each task t_v, set the cost c(t_v) = 1 − outdeg(v), where outdeg(v) is the out-degree of v in G. The intuition is:
   - When a vertex v is "evaluated," it occupies one register (cost +1).
   - Each of v's successor vertices u that uses v as an input will eventually "consume" that register (each predecessor that is the last to be needed frees one register slot).
   - A vertex with out-degree d effectively needs 1 register to store its result but frees registers as its successors are evaluated. The net cost c(t_v) = 1 − outdeg(v) captures this: leaves (outdeg = 0) cost +1 (they consume a register until their parent is computed), while high-outdegree nodes may have negative cost (freeing more registers than they use).

4. **Bound:** Set the cumulative cost bound to K (the same register bound from the original instance).

5. **Correctness:** The maximum cumulative cost at any point in the schedule equals the maximum number of simultaneously live registers during the corresponding evaluation order. Thus a K-register computation of G exists if and only if the tasks can be sequenced with maximum cumulative cost ≤ K.

6. **Solution extraction:** A feasible schedule σ with max cumulative cost ≤ K directly gives an evaluation order v_{σ^{-1}(1)}, v_{σ^{-1}(2)}, ..., v_{σ^{-1}(n)} that uses at most K registers.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |V| = number of vertices in the DAG (`num_vertices` of source)
- e = |A| = number of arcs in the DAG (`num_arcs` of source)

| Target metric (code name)   | Polynomial (using symbols above) |
|------------------------------|----------------------------------|
| `num_tasks`                  | n                                |
| `num_precedence_constraints` | e                                |
| `max_abs_cost`               | max(1, max_outdegree − 1)        |
| `bound_K`                    | K (same as source)               |

**Derivation:** Each vertex maps to one task; each arc maps to one precedence constraint. Costs are integers in range [1 − max_outdeg, 1]. Construction is O(n + e).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small DAG (e.g., 6–8 vertices), compute register sufficiency bound K, reduce to SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST, enumerate all topological orderings, verify that the minimum maximum cumulative cost equals K.
- Check that costs satisfy c(t_v) = 1 − outdeg(v) and precedence constraints match DAG edges.
- Edge cases: test with a chain DAG (K = 1 register suffices, max cumulative cost = 1), a tree DAG, and a DAG requiring maximum registers.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (REGISTER SUFFICIENCY):**

DAG G = (V, A) with 7 vertices modeling an expression tree:
```
v1 → v3, v1 → v4
v2 → v4, v2 → v5
v3 → v6
v4 → v6
v5 → v7
v6 → v7
```
(Arrows mean "is an input to".) Vertices v1, v2 are inputs (in-degree 0). K = 3.

Out-degrees: v1: 2, v2: 2, v3: 1, v4: 1, v5: 1, v6: 1, v7: 0.

**Constructed SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST instance:**

| Task | Cost c(t) = 1 − outdeg | Predecessors (must be scheduled before) |
|------|------------------------|-----------------------------------------|
| t_1  | 1 − 2 = −1            | (none — input vertex)                   |
| t_2  | 1 − 2 = −1            | (none — input vertex)                   |
| t_3  | 1 − 1 = 0             | t_1                                     |
| t_4  | 1 − 1 = 0             | t_1, t_2                                |
| t_5  | 1 − 1 = 0             | t_2                                     |
| t_6  | 1 − 1 = 0             | t_3, t_4                                |
| t_7  | 1 − 0 = 1             | t_5, t_6                                |

K = 3.

**A feasible schedule (topological order):**
Order: t_1, t_2, t_3, t_4, t_5, t_6, t_7
Cumulative costs: −1, −2, −2, −2, −2, −2, −1

All cumulative costs ≤ K = 3 ✓

Note: In this example the costs are all non-positive except for the final task, so K = 3 is easily satisfied. The NP-hard instances arise from DAGs with many leaves (high positive costs) interleaved with high-outdegree nodes.

**Solution extraction:**
Evaluation order: v1, v2, v3, v4, v5, v6, v7 — uses at most 3 registers ✓


## References

- **[Abdel-Wahab, 1976]**: [`Abdel-Wahab1976`] H. M. Abdel-Wahab (1976). "Scheduling with Applications to Register Allocation and Deadlock Problems". University of Waterloo.
- **[Abdel-Wahab and Kameda, 1978]**: [`Abdel-Wahab1978`] H. M. Abdel-Wahab and T. Kameda (1978). "Scheduling to minimize maximum cumulative cost subject to series-parallel precedence constraints". *Operations Research* 26, pp. 141–158.
- **[Monma and Sidney, 1977]**: [`Monma1977`] Clyde L. Monma and J. B. Sidney (1977). "A general algorithm for optimal job sequencing with series-parallel precedence constraints". School of Operations Research, Cornell University.
