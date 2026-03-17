---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Flow-Shop Scheduling"
labels: rule
assignees: ''
canonical_source_name: '3-PARTITION'
canonical_target_name: 'FLOW-SHOP SCHEDULING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3-Partition
**Target:** Flow-Shop Scheduling
**Motivation:** Establishes that Flow-Shop Scheduling is NP-complete in the strong sense even for a fixed number of processors m = 3, by encoding the strongly NP-complete 3-Partition problem into a 3-machine flow-shop instance where meeting the makespan deadline requires grouping jobs into triples that exactly fill time slots on each machine.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.241

## GJ Source Entry

> [SS15] FLOW-SHOP SCHEDULING
> INSTANCE: Number m E Z+ of processors, set J of jobs, each job j E J consisting of m tasks t_1[j], t_2[j], ..., t_m[j], a length l(t) E Z_0+ for each such task t, and an overall deadline D E Z+.
> QUESTION: Is there a flow-shop schedule for J that meets the overall deadline, where such a schedule is identical to an open-shop schedule with the additional constraint that, for each j E J and 1 <= i < m, σ_{i+1}(j) >= σ_i(j) + l(t_i[j])?
> Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense for m = 3. Solvable in polynomial time for m = 2 [Johnson, 1954]. The same results hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a], although if release times are added in this case, the problem is NP-complete in the strong sense, even for m = 2 [Cho and Sahni, 1978]. If the goal is to meet a bound K on the sum, over all j E J, of σ_m(j) + l(t_m[j]), then the non-preemptive problem is NP-complete in the strong sense even if m = 2 [Garey, Johnson, and Sethi, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3-PARTITION instance: a set A = {a_1, ..., a_{3m}} of 3m positive integers with sizes s(a_i), bound B such that B/4 < s(a_i) < B/2 and sum of all s(a_i) = mB, construct a 3-machine flow-shop instance as follows:

1. **Processors:** Set the number of machines to m_proc = 3.
2. **Jobs:** Create 3m "element jobs" J_e = {j_1, ..., j_{3m}}, one per element a_i. Also create m-1 "separator jobs" J_s = {s_1, ..., s_{m-1}} that enforce grouping. Total jobs: |J| = 3m + (m-1) = 4m - 1.
3. **Task lengths for element jobs:** For each element job j_i:
   - Task on machine 1: l(t_1[j_i]) = s(a_i) (the size of element a_i)
   - Task on machine 2: l(t_2[j_i]) = s(a_i)
   - Task on machine 3: l(t_3[j_i]) = s(a_i)
4. **Task lengths for separator jobs:** For each separator job s_k:
   - Task on machine 1: l(t_1[s_k]) = 0
   - Task on machine 2: l(t_2[s_k]) = L (a large value, e.g., L = mB + 1)
   - Task on machine 3: l(t_3[s_k]) = 0
   The separator jobs on machine 2 create "walls" that force element jobs to be grouped into triples between separators.
5. **Deadline:** D = mB + (m-1)L (just enough time if elements are perfectly partitioned into triples of sum B on each machine, with separator gaps in between on machine 2).

**Correctness:**
- If a valid 3-partition exists (A_1, ..., A_m each of size 3 summing to B), schedule the element jobs in group k in the k-th time slot of length B on each machine, separated by the large separator jobs on machine 2. The makespan exactly meets D.
- If the flow-shop schedule meets deadline D, then the separator jobs on machine 2 force exactly 3 element jobs to fit into each gap of size B, yielding a valid 3-partition.

**Solution extraction:** Given a feasible schedule, read off which element jobs fall between the k-th and (k+1)-th separator on machine 2; these form partition group A_k.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = 3m = number of elements in 3-PARTITION instance
- B = target sum per triple

| Target metric (code name)     | Polynomial (using symbols above)      |
|-------------------------------|---------------------------------------|
| `num_processors`              | 3                                     |
| `num_jobs`                    | `4 * num_elements / 3 - 1`           |
| `deadline`                    | `num_elements / 3 * B + (num_elements / 3 - 1) * L` |

**Derivation:** 3m element jobs + (m-1) separator jobs = 4m - 1 total jobs. Each job has 3 tasks (one per machine). The deadline is linear in mB + (m-1)L. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3-PARTITION instance with 3m = 6 elements (m = 2 triples), reduce to a 3-machine flow-shop instance, solve by brute-force enumeration of all (4m-1)! permutation schedules, verify that a feasible schedule exists iff a valid 3-partition exists.
- Check that the constructed instance has exactly 3 machines, 4m - 1 jobs, and the correct deadline.
- Edge cases: test with an instance where no valid 3-partition exists (expect no feasible schedule), and test with a trivially partitionable instance.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3-PARTITION):**
A = {a_1, a_2, a_3, a_4, a_5, a_6} with sizes s = {5, 7, 8, 6, 9, 5}
B = 20, m = 2 (sum = 40 = 2 * 20). Constraint: B/4 = 5, B/2 = 10. All sizes in (5, 10). (Note: s(a_1) = s(a_6) = 5 equals B/4, so this is a boundary case; for strict inequality, use sizes like {6, 7, 7, 6, 8, 6} with B = 20.)

Using strict example: A = {6, 7, 7, 6, 8, 6}, B = 20, m = 2, sum = 40.
Valid 3-partition: A_1 = {7, 7, 6} (sum=20), A_2 = {6, 8, 6} (sum=20).

**Constructed Flow-Shop instance:**
- Machines: m_proc = 3
- Jobs: 6 element jobs + 1 separator job = 7 jobs
- Element job task lengths (each job has tasks on all 3 machines):

| Job   | Machine 1 | Machine 2 | Machine 3 |
|-------|-----------|-----------|-----------|
| j_1   | 6         | 6         | 6         |
| j_2   | 7         | 7         | 7         |
| j_3   | 7         | 7         | 7         |
| j_4   | 6         | 6         | 6         |
| j_5   | 8         | 8         | 8         |
| j_6   | 6         | 6         | 6         |
| s_1   | 0         | 41        | 0         |

- Deadline D = 2 * 20 + 1 * 41 = 81

**Solution:**
Schedule group 1 (j_2, j_3, j_4) in time slot [0, 20] on each machine, then separator s_1 on machine 2 at [20, 61], then group 2 (j_1, j_5, j_6) in time slot [61, 81] on each machine. All jobs finish by D = 81.

**Solution extraction:**
- Group 1 jobs: j_2, j_3, j_4 -> A_1 = {7, 7, 6} (sum = 20) ✓
- Group 2 jobs: j_1, j_5, j_6 -> A_2 = {6, 8, 6} (sum = 20) ✓


## References

- **[Garey, Johnson, and Sethi, 1976]**: [`Garey1976f`] M. R. Garey and D. S. Johnson and R. Sethi (1976). "The complexity of flowshop and jobshop scheduling". *Mathematics of Operations Research* 1, pp. 117-129.
- **[Johnson, 1954]**: [`Johnson1954`] Selmer M. Johnson (1954). "Optimal two- and three-stage production schedules with setup times included". *Naval Research Logistics Quarterly* 1, pp. 61-68.
- **[Gonzalez and Sahni, 1978a]**: [`Gonzalez1978b`] T. Gonzalez and S. Sahni (1978). "Flowshop and jobshop schedules: complexity and approximation". *Operations Research* 26, pp. 36-52.
- **[Cho and Sahni, 1978]**: [`Cho1978`] Y. Cho and S. Sahni (1978). "Preemptive scheduling of independent jobs with release and due times on open, flow, and job shops".
