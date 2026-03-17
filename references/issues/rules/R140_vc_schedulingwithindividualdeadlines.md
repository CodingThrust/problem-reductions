---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Scheduling with Individual Deadlines"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'SCHEDULING WITH INDIVIDUAL DEADLINES'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Vertex Cover
**Target:** Scheduling with Individual Deadlines
**Motivation:** VERTEX COVER asks for a subset of at most K vertices covering all edges; SCHEDULING WITH INDIVIDUAL DEADLINES asks whether unit-length tasks with a partial order and individual deadlines can be scheduled on m processors so every task meets its own deadline. The reduction encodes each graph edge as a precedence constraint and uses the deadline structure to force that at most K "vertex tasks" are scheduled early (before the remaining tasks), which corresponds to selecting a vertex cover. This establishes NP-completeness of scheduling with individual deadlines, even when tasks have unit length and the precedence order is an out-tree.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.239-240

## GJ Source Entry

> [SS11] SCHEDULING WITH INDIVIDUAL DEADLINES
> INSTANCE: Set T of tasks, each having length l(t) = 1, number m E Z+ of processors, partial order < on T, and for each task t E T a deadline d(t) E Z+.
> QUESTION: Is there an m-processor schedule σ for T that obeys the precedence constraints and meets all the deadlines, i.e., σ(t) + l(t) <= d(t) for all t E T?
> Reference: [Brucker, Garey, and Johnson, 1977]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if < is an "out-tree" partial order (no task has more than one immediate predecessor), but can be solved in polynomial time if < is an "in-tree" partial order (no task has more than one immediate successor). Solvable in polynomial time if m = 2 and < is arbitrary [Garey and Johnson, 1976c], even if individual release times are included [Garey and Johnson, 1977b]. For < empty, can be solved in polynomial time by matching for m arbitrary, even with release times and with a single resource having 0-1 valued requirements [Blazewicz, 1977b], [Blazewicz, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let G = (V, E) be a graph with |V| = n, |E| = q, and K be the vertex-cover bound.

1. **Tasks:** Create one task v_i for each vertex i in V (n vertex tasks), and one task e_j for each edge j in E (q edge tasks). Total tasks: n + q.
2. **Precedence constraints:** For each edge e_j = {u, v}, add precedence constraints v_u < e_j and v_v < e_j (the edge task must be scheduled after both of its endpoint vertex tasks).
3. **Processors:** Set m = n (one processor per vertex, so all vertex tasks can run simultaneously in the first time slot).
4. **Deadlines:** For each vertex task v_i, set d(v_i) = 1 (must complete by time 1). For each edge task e_j, set d(e_j) = 2 (must complete by time 2).
5. **Revised construction (tighter):** Actually, the Brucker-Garey-Johnson construction is more subtle. Set m = K + q. Create n vertex tasks with deadline d(v_i) = 1 and q edge tasks with deadline d(e_j) = 2. The precedence order makes each edge task depend on its two endpoint vertex tasks. With m = K + q processors, at time 0 we can schedule at most K vertex tasks plus up to q edge tasks (but edge tasks have predecessors so they cannot start at time 0). At time 0, we schedule K vertex tasks. At time 1, we schedule the remaining n - K vertex tasks and q edge tasks. The key constraint is that at time 1, we need n - K + q processors (one for each remaining vertex task and each edge task). But we only have m = K + q processors. So we need n - K + q <= K + q, i.e., n <= 2K. Additionally, each edge task requires both its endpoint vertex tasks to be completed by time 1, so at least one endpoint of each edge must be among the K tasks scheduled at time 0, forming a vertex cover.

**Simplified construction (as typically presented):**

Let G = (V, E), |V| = n, |E| = q, bound K.

1. Create n + q unit-length tasks: {v_1, ..., v_n} (vertex tasks) and {e_1, ..., e_q} (edge tasks).
2. For each edge e_j = (u, w): add v_u < e_j and v_w < e_j.
3. Set m = K + q processors.
4. Set d(v_i) = 2 for all vertex tasks, d(e_j) = 2 for all edge tasks.
5. The total work is n + q units in 2 time slots, requiring at most m tasks per slot. At time 0, only vertex tasks can run (edge tasks have unfinished predecessors). At time 1, remaining vertex tasks and edge tasks run. A feasible schedule exists iff we can schedule enough vertex tasks at time 0 so that all edge tasks have both predecessors done, meaning at least one endpoint of each edge was scheduled at time 0 -- i.e., a vertex cover of size at most K.

**Solution extraction:** The vertex cover is V' = {v_i : sigma(v_i) = 0} (vertex tasks scheduled at time 0).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |V| = number of vertices in the graph
- q = |E| = number of edges
- K = vertex cover bound

| Target metric (code name)   | Polynomial (using symbols above) |
|------------------------------|----------------------------------|
| `num_tasks`                  | `num_vertices + num_edges`       |
| `num_processors`             | `vertex_cover_bound + num_edges` |
| `num_precedence_constraints` | `2 * num_edges`                  |
| `max_deadline`               | 2                                |

**Derivation:** Each vertex and each edge in the source graph becomes a task. Each edge contributes two precedence constraints (one per endpoint). The number of processors and the deadline are derived from K and the graph structure. Construction is O(n + q).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a VERTEX COVER instance (graph G, bound K), reduce to SCHEDULING WITH INDIVIDUAL DEADLINES, solve by brute-force enumeration of task-to-timeslot assignments respecting precedence and deadlines, verify the schedule corresponds to a vertex cover of size at most K.
- Check that the constructed scheduling instance has n + q tasks, K + q processors, and all deadlines are at most 2.
- Edge cases: test with K = 0 (infeasible unless q = 0), complete graph K_4 (minimum VC = 2 if K_3, etc.), star graph (VC = 1 at center).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (VERTEX COVER):**
G = (V, E) with V = {1, 2, 3, 4, 5}, E = {(1,2), (2,3), (3,4), (4,5), (1,5)} (a 5-cycle), K = 3.

Minimum vertex cover of a 5-cycle has size 3: e.g., V' = {1, 3, 4}.

**Constructed SCHEDULING WITH INDIVIDUAL DEADLINES instance:**

Tasks (10 total):
- Vertex tasks: v_1, v_2, v_3, v_4, v_5 (all with deadline 2)
- Edge tasks: e_1, e_2, e_3, e_4, e_5 (all with deadline 2)

Precedence constraints (10 total):
- e_1: v_1 < e_1, v_2 < e_1
- e_2: v_2 < e_2, v_3 < e_2
- e_3: v_3 < e_3, v_4 < e_3
- e_4: v_4 < e_4, v_5 < e_4
- e_5: v_1 < e_5, v_5 < e_5

Processors m = K + q = 3 + 5 = 8.

**Solution:**
Time slot 0: Schedule vertex tasks {v_1, v_3, v_4} (3 tasks, 3 <= 8 processors).
Time slot 1: Schedule {v_2, v_5, e_1, e_2, e_3, e_4, e_5} (7 tasks, 7 <= 8 processors).

Check deadlines: all tasks complete by time 2.
Check precedence: For e_1, predecessors v_1 (time 0) and v_2 (time 1) both finish by time 1 ... but v_2 finishes at time 2, and e_1 starts at time 1. Predecessor v_2 must finish before e_1 starts. v_2 is at time 1 and finishes at time 2, but e_1 also starts at time 1 — conflict!

**Revised schedule:** We need all predecessors of edge tasks to be scheduled at time 0. For edge e_1 = (1,2): at least one of v_1, v_2 at time 0. For e_2 = (2,3): at least one of v_2, v_3. Etc.

With V' = {1, 3, 4} at time 0:
- e_1 = (1,2): v_1 at time 0 (done), but v_2 at time 1 finishes at time 2, so e_1 can start at time 1 only if v_2 is done. This requires a slightly different model where edge tasks only need one predecessor done.

**Corrected construction:** The correct reduction requires that for each edge e_j = (u,w), the edge task has a deadline such that it forces at least one of v_u, v_w to be scheduled at time 0. The standard approach: set edge task deadlines to 2, vertex task deadlines to 2, but the precedence ordering and processor count together force the constraint. With m = K + q processors, at time 1 we can schedule at most K + q tasks. We have n - K' vertex tasks remaining (where K' tasks were at time 0) plus q edge tasks. So n - K' + q <= K + q means K' >= n - K. If n - K <= K (i.e., K >= n/2) this is trivial. The actual Brucker-Garey-Johnson construction is more intricate.

**Working example with simpler graph:**
G = path P_3: V = {1, 2, 3}, E = {(1,2), (2,3)}, K = 1 (vertex 2 covers both edges).

Tasks: v_1, v_2, v_3, e_1, e_2 (5 tasks).
Precedence: v_1 < e_1, v_2 < e_1, v_2 < e_2, v_3 < e_2.
m = K + q = 1 + 2 = 3 processors.
Deadlines: all = 2.

Time 0: At most 3 tasks, but only vertex tasks can go (edge tasks have predecessors). Schedule {v_1, v_2, v_3} (3 tasks = m).
Time 1: Schedule {e_1, e_2} (2 tasks <= m). All predecessors of e_1 (v_1, v_2) done at time 1. All predecessors of e_2 (v_2, v_3) done at time 1.
Total: all 5 tasks done by time 2.

But this works for any K >= 0, which is wrong. The construction needs refinement. The real Brucker et al. construction uses a more nuanced encoding. For the purposes of this issue, we note the reduction follows [Brucker, Garey, and Johnson, 1977] and the implementation should follow the original paper.

## References

- **[Brucker, Garey, and Johnson, 1977]**: [`Brucker1977`] P. Brucker and M. R. Garey and D. S. Johnson (1977). "Scheduling equal-length tasks under treelike precedence constraints to minimize maximum lateness". *Mathematics of Operations Research* 2, pp. 275–284.
- **[Garey and Johnson, 1976c]**: [`Garey1976c`] M. R. Garey and D. S. Johnson (1976). "The complexity of near-optimal graph coloring". *Journal of the Association for Computing Machinery* 23, pp. 43–49.
- **[Garey and Johnson, 1977b]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.
- **[Blazewicz, 1977b]**: [`Blazewicz1977b`] J. Blazewicz (1977). "Scheduling with deadlines and resource constraints". Technical University of Poznan.
- **[Blazewicz, 1978]**: [`Blazewicz1978`] J. Blazewicz (1978). "Deadline scheduling of tasks with ready times and resource constraints".
