---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Job-Shop Scheduling"
labels: rule
assignees: ''
canonical_source_name: '3-PARTITION'
canonical_target_name: 'JOB-SHOP SCHEDULING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3-Partition
**Target:** Job-Shop Scheduling
**Motivation:** Establishes that Job-Shop Scheduling is NP-complete in the strong sense even for only m = 2 processors, by encoding the strongly NP-complete 3-Partition problem. Unlike Flow-Shop Scheduling where all jobs follow the same machine order, in Job-Shop Scheduling each job has its own machine routing. The reduction exploits the routing flexibility to force a partition structure: jobs are designed so that meeting the deadline requires grouping tasks into triples that exactly sum to the partition bound B on each processor.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.242

## GJ Source Entry

> [SS18] JOB-SHOP SCHEDULING
> INSTANCE: Number m E Z+ of processors, set J of jobs, each j E J consisting of an ordered collection of tasks t_k[j], 1 <= k <= n_j, for each such task t a length l(t) E Z_0+ and a processor p(t) E {1,2,...,m}, where p(t_k[j]) ≠ p(t_{k+1}[j]) for all j E J and 1 <= k < n_j, and a deadline D E Z+.
> QUESTION: Is there a job-shop schedule for J that meets the overall deadline, i.e., a collection of one-processor schedules σ_i mapping {t: p(t) = i} into Z_0+, 1 <= i <= m, such that σ_i(t) > σ_i(t') implies σ_i(t) >= σ_i(t') + l(t), such that σ(t_{k+1}[j]) >= σ(t_k[j]) + l(t_k[j]) (where the appropriate subscripts are to be assumed on σ) for all j E J and 1 <= k < n_j, and such that for all j E J σ(t_{n_j}[j]) + l(t_{n_j}[j]) <= D (again assuming the appropriate subscript on σ)?
> Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense for m = 2. Can be solved in polynomial time if m = 2 and n_j <= 2 for all j E J [Jackson, 1956]. NP-complete (in the ordinary sense) if m = 2 and n_j <= 3 for all j E J, or if m = 3 and n_j <= 2 for all j E J [Gonzalez and Sahni, 1978a]. All the above results continue to hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a]. If in the nonpreemptive case all tasks have the same length, the problem is NP-complete for m = 3 and open for m = 2 [Lenstra and Rinnooy Kan, 1978b].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3-PARTITION instance: a set A = {a_1, ..., a_{3m}} of 3m positive integers with sizes s(a_i), bound B such that B/4 < s(a_i) < B/2 and sum of all s(a_i) = mB, construct a Job-Shop Scheduling instance as follows:

1. **Processors:** Set m_proc = 2.
2. **Jobs:** Create 3m "element jobs" and m-1 "separator jobs."
   - Each element job j_i (for element a_i) has tasks alternating between the two processors, with task lengths encoding s(a_i).
   - Each separator job s_k has a single long task on one of the processors (e.g., processor 1), forcing a gap that separates groups of element jobs.
3. **Task structure for element jobs:** Each element job j_i consists of tasks:
   - t_1[j_i] on processor 1 with length s(a_i)
   - t_2[j_i] on processor 2 with length s(a_i)
   (Jobs alternate between processors as required by the job-shop constraint p(t_k) != p(t_{k+1}).)
4. **Task structure for separator jobs:** Each separator job s_k has:
   - A single task on processor 1 with length L (a large value, e.g., L = mB + 1), creating mandatory idle windows on processor 1 that force element tasks into exactly m groups.
5. **Deadline:** D is chosen so that the schedule is feasible iff the element jobs can be partitioned into m groups of 3, each with total processing time B on each processor, fitting into the gaps between separator tasks.

**Correctness:**
- (3-Partition feasible -> Job-Shop feasible): A valid 3-partition {A_1, ..., A_m} yields a schedule where the 3 element jobs in each A_k are scheduled in the k-th gap on both processors, and their total processing time B fits exactly.
- (Job-Shop feasible -> 3-Partition feasible): The separator jobs force exactly m gaps of size B on processor 1, and the size constraints B/4 < s(a_i) < B/2 ensure exactly 3 element jobs fit per gap, yielding a valid 3-partition.

**Solution extraction:** Read off which element jobs are scheduled in each gap between separator tasks.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = 3m = number of elements in 3-PARTITION instance
- B = target sum per triple

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_processors`           | 2                                |
| `num_jobs`                 | `4 * num_elements / 3 - 1`      |
| `max_tasks_per_job`        | 2 (element jobs), 1 (separator)  |
| `deadline`                 | `num_elements / 3 * B + (num_elements / 3 - 1) * L` |

**Derivation:** 3m element jobs (each with 2 tasks) + (m-1) separator jobs (each with 1 task) = 4m - 1 total jobs. The deadline is linear in mB + (m-1)L. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3-PARTITION instance with 3m = 6 elements (m = 2 triples), reduce to a 2-processor job-shop instance, solve by brute-force enumeration of all feasible schedules, verify that a feasible schedule meeting the deadline exists iff a valid 3-partition exists.
- Verify that the constructed instance has exactly 2 processors and 4m - 1 jobs.
- Verify precedence constraints: for each element job, the task on processor 2 starts only after the task on processor 1 completes.
- Edge cases: test with an instance where no valid 3-partition exists (expect no feasible schedule).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3-PARTITION):**
A = {a_1, ..., a_6} with sizes s = {6, 7, 7, 6, 8, 6}, B = 20, m = 2.
Sum = 40 = 2 * 20. All sizes satisfy B/4 = 5 < s(a_i) < B/2 = 10. ✓
Valid 3-partition: A_1 = {7, 7, 6} (sum=20), A_2 = {6, 8, 6} (sum=20).

**Constructed Job-Shop Scheduling instance:**
- Processors: m_proc = 2
- Jobs: 6 element jobs + 1 separator job = 7 jobs

| Job   | Task 1 (processor, length) | Task 2 (processor, length) |
|-------|----------------------------|----------------------------|
| j_1   | (P1, 6)                   | (P2, 6)                   |
| j_2   | (P1, 7)                   | (P2, 7)                   |
| j_3   | (P1, 7)                   | (P2, 7)                   |
| j_4   | (P1, 6)                   | (P2, 6)                   |
| j_5   | (P1, 8)                   | (P2, 8)                   |
| j_6   | (P1, 6)                   | (P2, 6)                   |
| s_1   | (P1, 41)                  | --                         |

- Deadline D = 2 * 20 + 1 * 41 = 81

**Solution:**
- P1 timeline: [0,20] group 1 tasks (j_2:7, j_3:7, j_4:6), [20,61] separator s_1 (length 41), [61,81] group 2 tasks (j_1:6, j_5:8, j_6:6)
- P2 timeline: group 1 tasks start after their P1 tasks complete, fitting within [7, 27] or similar; group 2 tasks on P2 follow similarly.
- All jobs complete by D = 81. ✓

**Solution extraction:**
- Group 1 (first gap): j_2, j_3, j_4 -> A_1 = {7, 7, 6} (sum = 20) ✓
- Group 2 (second gap): j_1, j_5, j_6 -> A_2 = {6, 8, 6} (sum = 20) ✓


## References

- **[Garey, Johnson, and Sethi, 1976]**: [`Garey1976f`] M. R. Garey and D. S. Johnson and R. Sethi (1976). "The complexity of flowshop and jobshop scheduling". *Mathematics of Operations Research* 1, pp. 117-129.
- **[Jackson, 1956]**: [`Jackson1956`] James R. Jackson (1956). "An extension of {Johnson}'s results on job lot scheduling". *Naval Research Logistics Quarterly* 3, pp. 201-203.
- **[Gonzalez and Sahni, 1978a]**: [`Gonzalez1978b`] T. Gonzalez and S. Sahni (1978). "Flowshop and jobshop schedules: complexity and approximation". *Operations Research* 26, pp. 36-52.
- **[Lenstra and Rinnooy Kan, 1978b]**: [`Lenstra1978b`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Computational complexity of discrete optimization problems". *Annals of Discrete Mathematics*.
