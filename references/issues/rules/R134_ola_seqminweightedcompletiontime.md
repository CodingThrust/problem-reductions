---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Optimal Linear Arrangement to Sequencing to Minimize Weighted Completion Time"
labels: rule
assignees: ''
canonical_source_name: 'OPTIMAL LINEAR ARRANGEMENT'
canonical_target_name: 'SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Optimal Linear Arrangement
**Target:** Sequencing to Minimize Weighted Completion Time
**Motivation:** Establishes NP-completeness (in the strong sense) of SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME by reducing from OPTIMAL LINEAR ARRANGEMENT. The key insight is that minimizing total weighted completion time with precedence constraints subsumes minimizing edge-stretch in a linear ordering: for each edge {u,v} in G, a precedence-constrained edge-task forces a scheduling cost proportional to |f(u) - f(v)|. Lawler (1978) showed that the scheduling problem with arbitrary precedence constraints is NP-hard, and that it becomes polynomial for series-parallel orders. This reduction is fundamental to the complexity landscape of single-machine scheduling.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.237; Lawler (1978)

## GJ Source Entry

> [SS4] SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME
> INSTANCE: Set T of tasks, partial order < on T, for each task t E T a length l(t) E Z+ and a weight w(t) E Z+, and a positive integer K.
> QUESTION: Is there a one-processor schedule sigma for T that obeys the precedence constraints and for which the sum, over all t E T, of (sigma(t) + l(t))*w(t) is K or less?
> Reference: [Lawler, 1978]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
> Comment: NP-complete in the strong sense and remains so even if all task lengths are 1 or all task weights are 1. Can be solved in polynomial time for < a "forest" [Horn, 1972], [Adolphson and Hu, 1973], [Garey, 1973], [Sidney, 1975] or if < is "series-parallel" or "generalized series-parallel" [Knuth, 1973], [Lawler, 1978], [Adolphson, 1977], [Monma and Sidney, 1977]. If the partial order < is replaced by individual task deadlines, the resulting problem is NP-complete in the strong sense [Lenstra, 1977], but can be solved in polynomial time if all task weights are equal [Smith, 1956]. If there are individual task release times instead of deadlines, the resulting problem is NP-complete in the strong sense, even if all task weights are 1 [Lenstra, Rinnooy Kan, and Brucker, 1977]. The "preemptive" version of this latter problem is NP-complete in the strong sense [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1978], but is solvable in polynomial time if all weights are equal [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given an OPTIMAL LINEAR ARRANGEMENT instance (G = (V, E), K_OLA), where |V| = n and |E| = m, construct a SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME instance as follows:

1. **Vertex tasks:** For each vertex v in V, create a task t_v with:
   - Length: l(t_v) = 1 (unit processing time)
   - Weight: w(t_v) = 0 (zero weight; vertex tasks do not contribute to the objective)

2. **Edge tasks:** For each edge e = {u, v} in E, create a task t_e with:
   - Length: l(t_e) = 1 (unit processing time)
   - Weight: w(t_e) = 1 (unit weight; each edge task contributes its completion time to the objective)

3. **Precedence constraints:** For each edge e = {u, v} in E, add:
   - t_u < t_e and t_v < t_e (both endpoint vertex-tasks must complete before the edge-task starts)
   - No precedence among vertex-tasks themselves, and no precedence among edge-tasks themselves

4. **Bound:** Set K = K_OLA + C, where C is a constant that accounts for the baseline cost when edges are zero-stretch. Specifically, for a given arrangement f, the total weighted completion time of the corresponding schedule equals the OLA cost sum |f(u)-f(v)| plus a fixed offset depending on n and m. The precise value of K is chosen so that the scheduling objective <= K iff the OLA objective <= K_OLA.

5. **Correctness:** In any valid schedule respecting the precedence constraints, an edge task t_e (for e = {u,v}) must be scheduled after both t_u and t_v. The completion time of t_e is larger when t_u and t_v are far apart in the schedule. The order of vertex-tasks defines a linear arrangement f of V, and the total weighted completion time of edge tasks captures the sum of edge stretches |f(u) - f(v)| (plus a constant). Thus minimizing weighted completion time is equivalent to minimizing the linear arrangement cost.

6. **Solution extraction:** Read off the ordering of vertex-tasks in the optimal schedule to obtain the linear arrangement f: V -> {1,...,n}.

**Key invariant:** G has a linear arrangement with cost <= K_OLA iff the scheduling instance has a valid schedule with total weighted completion time <= K.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_tasks`               | `num_vertices + num_edges`       |

**Derivation:**
- One task per vertex plus one task per edge gives |T| = n + m.
- The precedence constraints form a bipartite partial order with 2m precedence pairs.
- All tasks have unit processing time.
- Construction is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct an OPTIMAL LINEAR ARRANGEMENT instance (G, K_OLA), reduce to SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME, solve the target with BruteForce (try all topological orderings), verify whether any schedule achieves total weighted completion time <= K.
- Extract the vertex-task ordering from the optimal schedule and verify it yields a linear arrangement with cost <= K_OLA.
- Test with a path graph P_4 (4 vertices, 3 edges): the optimal arrangement places vertices in path order, giving cost 3. Verify the reduction matches.
- Test with K_3 (triangle): optimal arrangement cost is 4 (any permutation gives |f(u)-f(v)| sums of 1+2+1=4). Verify the corresponding schedule is optimal.
- Test with a star graph S_4 (center + 3 leaves): center should be in the middle of the schedule.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (OPTIMAL LINEAR ARRANGEMENT):**
Graph G with 5 vertices {0, 1, 2, 3, 4} and 5 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {0,4} (a cycle C_5)
- Optimal arrangement: f(0)=1, f(1)=2, f(4)=3, f(3)=4, f(2)=5
  (or equivalently, ordering: 0, 1, 4, 3, 2)
  Cost = |1-2| + |2-5| + |5-4| + |4-3| + |1-3| = 1 + 3 + 1 + 1 + 2 = 8
- Another arrangement: f(0)=1, f(1)=2, f(2)=3, f(3)=4, f(4)=5
  Cost = |1-2| + |2-3| + |3-4| + |4-5| + |1-5| = 1 + 1 + 1 + 1 + 4 = 8

**Constructed target instance (SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME):**

Tasks (|V| + |E| = 5 + 5 = 10 total), all with unit length l = 1:

| Task   | Type   | Weight w | Notes         |
|--------|--------|----------|---------------|
| t_0    | vertex | 0        | vertex 0      |
| t_1    | vertex | 0        | vertex 1      |
| t_2    | vertex | 0        | vertex 2      |
| t_3    | vertex | 0        | vertex 3      |
| t_4    | vertex | 0        | vertex 4      |
| t_01   | edge   | 1        | edge {0,1}    |
| t_12   | edge   | 1        | edge {1,2}    |
| t_23   | edge   | 1        | edge {2,3}    |
| t_34   | edge   | 1        | edge {3,4}    |
| t_04   | edge   | 1        | edge {0,4}    |

Precedence constraints:
- t_0 < t_01, t_1 < t_01
- t_1 < t_12, t_2 < t_12
- t_2 < t_23, t_3 < t_23
- t_3 < t_34, t_4 < t_34
- t_0 < t_04, t_4 < t_04

**Schedule (from arrangement 0, 1, 2, 3, 4):**

The vertex tasks are scheduled first in arrangement order, then edge tasks are placed as early as possible respecting precedence:

| Position | Task | Start | Finish | Weight | w * finish |
|----------|------|-------|--------|--------|------------|
| 0        | t_0  | 0     | 1      | 0      | 0          |
| 1        | t_1  | 1     | 2      | 0      | 0          |
| 2        | t_01 | 2     | 3      | 1      | 3          |
| 3        | t_2  | 3     | 4      | 0      | 0          |
| 4        | t_12 | 4     | 5      | 1      | 5          |
| 5        | t_3  | 5     | 6      | 0      | 0          |
| 6        | t_23 | 6     | 7      | 1      | 7          |
| 7        | t_4  | 7     | 8      | 0      | 0          |
| 8        | t_34 | 8     | 9      | 1      | 9          |
| 9        | t_04 | 9     | 10     | 1      | 10         |

Total weighted completion time = 0 + 0 + 3 + 0 + 5 + 0 + 7 + 0 + 9 + 10 = 34

Precedence constraints respected: each edge task is scheduled after both its endpoint vertex tasks.

**Solution extraction:**
The vertex-task ordering (t_0, t_1, t_2, t_3, t_4) defines f(0)=1, f(1)=2, f(2)=3, f(3)=4, f(4)=5.
OLA cost = sum |f(u)-f(v)| = |1-2| + |2-3| + |3-4| + |4-5| + |1-5| = 1 + 1 + 1 + 1 + 4 = 8.


## References

- **[Lawler, 1978]**: [`Lawler1978a`] Eugene L. Lawler (1978). "Sequencing jobs to minimize total weighted completion time subject to precedence constraints". *Annals of Discrete Mathematics* 2, pp. 75-90.
- **[Horn, 1972]**: [`Horn1972`] William A. Horn (1972). "Single-machine job sequencing with treelike precedence ordering and linear delay penalties". *SIAM Journal on Applied Mathematics* 23, pp. 189-202.
- **[Adolphson and Hu, 1973]**: [`Adolphson1973`] D. Adolphson and T. C. Hu (1973). "Optimal linear ordering". *SIAM Journal on Applied Mathematics* 25, pp. 403-423.
- **[Garey, 1973]**: [`Garey1973`] M. R. Garey (1973). "Optimal task sequencing with precedence constraints". *Discrete Mathematics* 4, pp. 37-56.
- **[Sidney, 1975]**: [`Sidney1975`] Jeffrey B. Sidney (1975). "Decomposition algorithms for single-machine sequencing with precedence relations and deferral costs". *Operations Research* 23, pp. 283-298.
- **[Knuth, 1973]**: [`Knuth1973`] Donald E. Knuth (1973). "Private communication".
- **[Adolphson, 1977]**: [`Adolphson1977`] D. Adolphson (1977). "Single machine job sequencing with precedence constraints". *SIAM Journal on Computing* 6, pp. 40-54.
- **[Monma and Sidney, 1977]**: [`Monma1977`] Clyde L. Monma and J. B. Sidney (1977). "A general algorithm for optimal job sequencing with series-parallel precedence constraints". School of Operations Research, Cornell University.
- **[Lenstra, 1977]**: [`Lenstra1977`] Jan K. Lenstra (1977). "".
- **[Smith, 1956]**: [`Smith1956`] Wayne E. Smith (1956). "Various optimizers for single-state production". *Naval Research Logistics Quarterly* 3, pp. 59-66.
- **[Lenstra, Rinnooy Kan, and Brucker, 1977]**: [`Lenstra1977a`] Jan K. Lenstra and A. H. G. Rinnooy Kan and Peter Brucker (1977). "Complexity of machine scheduling problems". *Annals of Discrete Mathematics* 1, pp. 343-362.
- **[Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1978]**: [`Labetoulle and Lawler and Lenstra and Rinnooy Kan1978`] Jacques Labetoulle and Eugene L. Lawler and Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Preemptive scheduling of uniform machines".
- **[Graham, Lawler, Lenstra, and Rinnooy Kan, 1978]**: [`Graham1978`] R. L. Graham and E. L. Lawler and J. K. Lenstra and A. H. G. Rinnooy Kan (1978). "Optimization and approximation in deterministic sequencing and scheduling: a survey". *Annals of Discrete Mathematics*.
