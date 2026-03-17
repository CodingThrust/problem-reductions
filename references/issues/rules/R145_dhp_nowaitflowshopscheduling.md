---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Directed Hamiltonian Path to No-Wait Flow-Shop Scheduling"
labels: rule
assignees: ''
canonical_source_name: 'DIRECTED HAMILTONIAN PATH'
canonical_target_name: 'NO-WAIT FLOW-SHOP SCHEDULING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Directed Hamiltonian Path
**Target:** No-Wait Flow-Shop Scheduling
**Motivation:** Establishes that No-Wait Flow-Shop Scheduling is NP-complete in the strong sense by reducing from Directed Hamiltonian Path. In a no-wait flow shop, each job must proceed through all machines without any waiting between consecutive machines. The key insight is that the no-wait constraint makes the makespan depend only on the sequencing order of jobs (not start times), and the inter-job delays can be encoded as arc weights in a directed graph, turning the scheduling problem into a shortest Hamiltonian path problem on a digraph.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.241-242

## GJ Source Entry

> [SS16] NO-WAIT FLOW-SHOP SCHEDULING
> INSTANCE: (Same as for FLOW-SHOP SCHEDULING).
> QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and has the property that, for each j E J and 1 <= i < m, σ_{i+1}(j) = σ_i(j) + l(t_i[j])?
> Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from DIRECTED HAMILTONIAN PATH.
> Comment: NP-complete in the strong sense for any fixed m >= 4 [Papadimitriou and Kanellakis, 1978]. Solvable in polynomial time for m = 2 [Gilmore and Gomory, 1964]. (However, NP-complete in the strong sense for m = 2 if jobs with no tasks on the first processor are allowed [Sahni and Cho, 1977b].) Open for fixed m = 3. If the goal is to meet a bound K on the sum, over all j E J, of σ_m(j) + l(t_m[j]), then the problem is NP-complete in the strong sense for m arbitrary [Lenstra, Rinnooy Kan, and Brucker, 1977] and open for fixed m >= 2. The analogous "no-wait" versions of OPEN-SHOP SCHEDULING and JOB-SHOP SCHEDULING are NP-complete in the strong sense for m = 2 [Sahni and Cho, 1977b].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a Directed Hamiltonian Path instance: a directed graph G = (V, A) with |V| = n vertices, construct a No-Wait Flow-Shop Scheduling instance as follows:

1. **Machines:** Choose m >= 4 machines (the number must be large enough to encode the graph structure; m = n + 1 suffices for the general construction).
2. **Jobs:** Create one job j_v for each vertex v in V. Total jobs: |J| = n.
3. **Task lengths:** For each job j_v and each machine i (1 <= i <= m), assign task length l(t_i[j_v]) based on the adjacency structure of G. The key idea is to define processing times such that the "delay" d(j_u, j_v) between scheduling job j_u immediately before job j_v equals:
   - d(j_u, j_v) = a small value (e.g., 1) if arc (u, v) in A
   - d(j_u, j_v) = a large value (e.g., n * max_length + 1) if arc (u, v) not in A

   The delay d(j_u, j_v) = max_{i=1}^{m-1} (sum_{k=1}^{i} l(t_k[j_v]) - sum_{k=1}^{i} l(t_k[j_u])) is the minimum gap between the start times of j_u and j_v when j_u is scheduled before j_v in the no-wait regime.

4. **Deadline:** D = sum of all job completion times when sequenced along a Hamiltonian path (using minimum delays for arcs that exist). Specifically, D = (total processing time of one job) + (n-1) * (small delay value), which is achievable iff a Hamiltonian path exists in G.

**Correctness:**
- The no-wait constraint means each job's start time is completely determined by the sequence order and the delay matrix. The total makespan of a permutation (j_{v_1}, j_{v_2}, ..., j_{v_n}) is the processing time of the last job plus the sum of delays d(j_{v_i}, j_{v_{i+1}}) for i = 1, ..., n-1.
- Makespan <= D iff all consecutive pairs in the sequence correspond to arcs in G, i.e., (v_1, v_2, ..., v_n) is a Hamiltonian path in G.

**Solution extraction:** Given a feasible no-wait schedule with makespan <= D, the job ordering gives a Hamiltonian path in G.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |V| = number of vertices in the directed graph

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_processors`           | `num_vertices + 1`               |
| `num_jobs`                 | `num_vertices`                   |
| `deadline`                 | O(n * max_task_length)           |

**Derivation:** Each vertex maps to one job (n jobs). The number of machines is O(n) in the general construction. The task lengths are polynomial in n. Construction is O(n^2) to compute the delay matrix.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small directed graph (5 vertices) with a known Hamiltonian path, reduce to a no-wait flow-shop instance, solve by brute-force enumeration of all n! job permutations, verify that a feasible schedule exists iff a Hamiltonian path exists.
- Verify the delay matrix: for each pair (u, v) with arc in G, the computed delay is small; for non-arcs, the delay is large.
- Edge cases: test with a graph containing no Hamiltonian path (expect no feasible schedule), test with a complete tournament (always has a Hamiltonian path).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Directed Hamiltonian Path):**
G = (V, A) with V = {1, 2, 3, 4, 5} and arcs:
A = {(1,2), (2,3), (3,4), (4,5), (1,3), (2,5), (3,1), (5,2)}

Hamiltonian path: 1 -> 2 -> 3 -> 4 -> 5 (all arcs (1,2), (2,3), (3,4), (4,5) exist).

**Constructed No-Wait Flow-Shop instance (conceptual):**
- Machines: m = 6 (= n + 1 = 5 + 1)
- Jobs: J = {j_1, j_2, j_3, j_4, j_5}
- Task lengths are assigned so that:
  - delay(j_1, j_2) = 1 (arc exists)
  - delay(j_2, j_3) = 1 (arc exists)
  - delay(j_3, j_4) = 1 (arc exists)
  - delay(j_4, j_5) = 1 (arc exists)
  - delay(j_1, j_4) = L (no arc (1,4))
  - delay(j_4, j_1) = L (no arc (4,1))
  - etc. for all non-arc pairs

- Deadline: D = T_job + 4 * 1 = T_job + 4 (where T_job is the completion time of a single job going through all machines)

**Solution:**
Sequence: j_1, j_2, j_3, j_4, j_5. All consecutive delays are 1 (arcs exist). Makespan = T_job + 4 <= D. ✓

**Solution extraction:**
Job ordering j_1, j_2, j_3, j_4, j_5 -> Hamiltonian path 1 -> 2 -> 3 -> 4 -> 5. ✓

**Negative case:**
Remove arc (3,4) from A. Now no Hamiltonian path using vertex sequence ...3...4... can avoid a large delay at that transition. Since all Hamiltonian paths must traverse all vertices, and the only way to reach 4 from 3 would require a large delay, no schedule meets the tight deadline D. ✓


## References

- **[Lenstra, Rinnooy Kan, and Brucker, 1977]**: [`Lenstra1977a`] Jan K. Lenstra and A. H. G. Rinnooy Kan and Peter Brucker (1977). "Complexity of machine scheduling problems". *Annals of Discrete Mathematics* 1, pp. 343-362.
- **[Papadimitriou and Kanellakis, 1978]**: [`Papadimitriou1978e`] Christos H. Papadimitriou and P. C. Kanellakis (1978). "Flowshop scheduling with limited temporary storage".
- **[Gilmore and Gomory, 1964]**: [`Gilmore1964`] P. C. Gilmore and R. E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem". *Operations Research* 12, pp. 655-679.
- **[Sahni and Cho, 1977b]**: [`Sahni1977b`] S. Sahni and Y. Cho (1977). "Complexity of scheduling shops with no wait in process". Computer Science Dept., University of Minnesota.
