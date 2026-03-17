---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Clique to Sequencing to Minimize Tardy Tasks"
labels: rule
assignees: ''
canonical_source_name: 'CLIQUE'
canonical_target_name: 'SEQUENCING TO MINIMIZE TARDY TASKS'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Clique
**Target:** Sequencing to Minimize Tardy Tasks
**Motivation:** Establishes NP-completeness of SEQUENCING TO MINIMIZE TARDY TASKS by encoding a CLIQUE instance into a scheduling problem with precedence constraints. The reduction creates vertex-tasks and edge-tasks with unit processing times, where precedence constraints force each edge-task to be scheduled after both its endpoint vertex-tasks. An early deadline on edge-tasks means that at most K tasks can be tardy, and a counting argument shows that meeting this bound requires exactly J vertex-tasks and C(J,2) edge-tasks to be scheduled early -- which is only possible if those edges form a J-clique. This is the same structural approach as the MINIMUM TARDINESS SEQUENCING reduction in Garey & Johnson Theorem 3.10, adapted for the SS2 formulation with general task lengths and precedence constraints.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.236; see also Theorem 3.10 (p.73) for MINIMUM TARDINESS SEQUENCING

## GJ Source Entry

> [SS2] SEQUENCING TO MINIMIZE TARDY TASKS
> INSTANCE: Set T of tasks, partial order < on T, for each task t E T a length l(t) E Z+ and a deadline d(t) E Z+, and a positive integer K <= |T|.
> QUESTION: Is there a one-processor schedule sigma for T that obeys the precedence constraints, i.e., such that t < t' implies sigma(t) + l(t) < sigma(t'), and such that there are at most K tasks t E T for which sigma(t) + l(t) > d(t)?
> Reference: [Garey and Johnson, 1976c]. Transformation from CLIQUE (see Section 3.2.3).
> Comment: Remains NP-complete even if all task lengths are 1 and < consists only of "chains" (each task has at most one immediate predecessor and at most one immediate successor) [Lenstra, 1977]. The general problem can be solved in polynomial time if K = 0 [Lawler, 1973], or if < is empty [Moore, 1968] [Sidney, 1973]. The < empty case remains polynomially solvable if "agreeable" release times (i.e., r(t) < r(t') implies d(t) <= d(t')) are added [Kise, Ibaraki, and Mine, 1978], but is NP-complete for arbitrary release times (see previous problem).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a CLIQUE instance (G = (V, E), J) where |V| = n and |E| = m, construct a SEQUENCING TO MINIMIZE TARDY TASKS instance as follows:

1. **Task set:** Create one task t_v for each vertex v in V and one task t_e for each edge e in E. Thus |T| = n + m.

2. **Lengths:** Set l(t) = 1 for all tasks (unit processing times).

3. **Deadlines:**
   - For each vertex task t_v: d(t_v) = n + m (a late deadline; vertex tasks are never in danger of being tardy).
   - For each edge task t_e: d(t_e) = J(J+1)/2 (an early "clique selection" deadline).

4. **Precedence constraints:** For each edge e = {u, v} in E, add precedence constraints t_u < t_e and t_v < t_e (both endpoint vertex-tasks must be completed before the edge-task begins).

5. **Tardiness bound:** Set K = m - J(J-1)/2. This is the maximum allowed number of tardy tasks (edge tasks finishing after their deadline).

6. **Correctness:** There is room for J(J+1)/2 unit tasks before the edge-task deadline. For at most K edge tasks to be tardy, at least J(J-1)/2 edge tasks must finish by time J(J+1)/2. The precedence constraints force their endpoint vertex-tasks to also be early. Since J(J-1)/2 edges need at least J vertices, and only J(J+1)/2 - J(J-1)/2 = J vertex-task slots are available before the deadline, exactly J vertex-tasks and J(J-1)/2 edge-tasks must be early, forming a J-clique.

7. **Solution extraction:** Identify the vertex tasks scheduled before time J(J+1)/2. The corresponding vertices form a J-clique in G.

**Key invariant:** G has a J-clique iff there exists a valid schedule with at most K = m - J(J-1)/2 tardy tasks.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G
- J = clique size parameter

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_tasks`               | `num_vertices + num_edges`       |

**Derivation:**
- One task per vertex plus one task per edge gives |T| = n + m.
- The partial order has exactly 2m precedence pairs (two vertex-tasks per edge-task).
- K = m - J(J-1)/2 is derived from source parameters.
- Construction is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a MaximumClique instance (G, J), reduce to SEQUENCING TO MINIMIZE TARDY TASKS, solve the target with BruteForce (try all topological orderings of the partial order), check whether any schedule has at most K tardy tasks.
- Verify the counting argument: in a satisfying schedule, identify the J vertex-tasks and J(J-1)/2 edge-tasks scheduled before time J(J+1)/2, confirm the corresponding subgraph is a J-clique.
- Test with K_4 (complete graph on 4 vertices) and J = 3: should find a valid schedule (any 3-clique works).
- Test with a triangle-free graph (e.g., C_5) and J = 3: should find no valid schedule since no 3-clique exists.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (CLIQUE):**
Graph G with 5 vertices {0, 1, 2, 3, 4} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,3}, {2,4}, {3,4}
- (Vertices 0,1,2 form a triangle; vertices 1,2,3 form a triangle; vertices 2,3,4 form a triangle)
- G contains a 3-clique: e.g., {0, 1, 2}
- Clique parameter: J = 3

**Constructed target instance (SEQUENCING TO MINIMIZE TARDY TASKS):**

Tasks (|V| + |E| = 5 + 7 = 12 total), all with unit length l = 1:

| Task  | Type   | Deadline d | Notes                  |
|-------|--------|------------|------------------------|
| t_0   | vertex | 12         | vertex 0               |
| t_1   | vertex | 12         | vertex 1               |
| t_2   | vertex | 12         | vertex 2               |
| t_3   | vertex | 12         | vertex 3               |
| t_4   | vertex | 12         | vertex 4               |
| t_01  | edge   | 6          | edge {0,1}, d=J(J+1)/2 |
| t_02  | edge   | 6          | edge {0,2}             |
| t_12  | edge   | 6          | edge {1,2}             |
| t_13  | edge   | 6          | edge {1,3}             |
| t_23  | edge   | 6          | edge {2,3}             |
| t_24  | edge   | 6          | edge {2,4}             |
| t_34  | edge   | 6          | edge {3,4}             |

Deadlines: vertex tasks d = 5 + 7 = 12; edge tasks d = 3(4)/2 = 6.
Tardiness bound: K = 7 - 3(2)/2 = 7 - 3 = 4.

Partial order (vertex endpoints must precede edge task):
- t_0 < t_01, t_1 < t_01
- t_0 < t_02, t_2 < t_02
- t_1 < t_12, t_2 < t_12
- t_1 < t_13, t_3 < t_13
- t_2 < t_23, t_3 < t_23
- t_2 < t_24, t_4 < t_24
- t_3 < t_34, t_4 < t_34

**Schedule (from clique {0, 1, 2}):**

Early portion (positions 0-5, before edge deadline 6):

| Position | Task | Finishes at | Deadline | Tardy? |
|----------|------|-------------|----------|--------|
| 0        | t_0  | 1           | 12       | No     |
| 1        | t_1  | 2           | 12       | No     |
| 2        | t_2  | 3           | 12       | No     |
| 3        | t_01 | 4           | 6        | No     |
| 4        | t_02 | 5           | 6        | No     |
| 5        | t_12 | 6           | 6        | No     |

Late portion (positions 6-11):

| Position | Task | Finishes at | Deadline | Tardy? |
|----------|------|-------------|----------|--------|
| 6        | t_3  | 7           | 12       | No     |
| 7        | t_4  | 8           | 12       | No     |
| 8        | t_13 | 9           | 6        | Yes    |
| 9        | t_23 | 10          | 6        | Yes    |
| 10       | t_24 | 11          | 6        | Yes    |
| 11       | t_34 | 12          | 6        | Yes    |

Tardy tasks: {t_13, t_23, t_24, t_34}, count = 4 = K.
Precedence constraints respected: all vertex tasks precede their dependent edge tasks.

**Solution extraction:**
The J = 3 vertex tasks before deadline 6: {t_0, t_1, t_2} -> vertices {0, 1, 2}.
The J(J-1)/2 = 3 edge tasks before deadline 6: {t_01, t_02, t_12} -> edges {0,1}, {0,2}, {1,2}.
These form a complete subgraph (3-clique) on vertices {0, 1, 2}.


## References

- **[Garey and Johnson, 1976c]**: [`Garey1976c`] M. R. Garey and D. S. Johnson (1976). "The complexity of near-optimal graph coloring". *Journal of the Association for Computing Machinery* 23, pp. 43-49.
- **[Lenstra, 1977]**: [`Lenstra1977`] Jan K. Lenstra (1977). "".
- **[Lawler, 1973]**: [`Lawler1973`] Eugene L. Lawler (1973). "Optimal sequencing of a single machine subject to precedence constraints". *Management Science* 19, pp. 544-546.
- **[Moore, 1968]**: [`Moore1968`] J. M. Moore (1968). "An $n$ job, one machine sequencing algorithm for minimizing the number of late jobs". *Management Science* 15, pp. 102-109.
- **[Sidney, 1973]**: [`Sidney1973`] Jeffrey B. Sidney (1973). "An extension of {Moore}'s due date algorithm". In: *Symposium on the Theory of Scheduling and its Applications*. Springer.
- **[Kise, Ibaraki, and Mine, 1978]**: [`Kise1978`] Hiroshi Kise and Toshihide Ibaraki and Hisashi Mine (1978). "A solvable case of the one-machine scheduling problem with ready and due times". *Operations Research* 26, pp. 121-126.
