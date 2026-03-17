---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Open-Shop Scheduling"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'OPEN-SHOP SCHEDULING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Partition
**Target:** Open-Shop Scheduling
**Motivation:** PARTITION asks whether a multiset of integers can be split into two equal-sum halves; OPEN-SHOP SCHEDULING asks whether jobs (each consisting of m tasks, one per processor) can be non-preemptively scheduled to minimize the makespan. The reduction uses m = 3 machines and constructs jobs from the partition elements so that a schedule meeting a tight makespan deadline exists if and only if the partition elements can be split into two equal-sum groups. This establishes NP-completeness of open-shop scheduling for 3 or more machines, while the 2-machine case remains polynomial.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.241

## GJ Source Entry

> [SS14] OPEN-SHOP SCHEDULING
> INSTANCE: Number m E Z+ of processors, set J of jobs, each job j E J consisting of m tasks t_1[j], t_2[j], ..., t_m[j] (with t_i[j] to be executed by processor i), a length l(t) E Z_0+ for each such task t, and an overall deadline D E Z+.
> QUESTION: Is there an open-shop schedule for J that meets the deadline, i.e., a collection of one-processor schedules σ_i: J → Z_0+, 1 <= i <= m, such that σ_i(j) > σ_i(k) implies σ_i(j) >= σ_i(k) + l(t_i[k]), such that for each j E J the intervals [σ_i(j), σ_i(j) + l(t_i[j])) are all disjoint, and such that σ_i(j) + l(t_i[j]) <= D for 1 <= i <= m, 1 <= j <= |J|?
> Reference: [Gonzalez and Sahni, 1976]. Transformation from PARTITION.
> Comment: Remains NP-complete if m = 3, but can be solved in polynomial time if m = 2. NP-complete in the strong sense for m arbitrary [Lenstra, 1977]. The general problem is solvable in polynomial time if "preemptive" schedules are allowed [Gonzalez and Sahni, 1976], even if two distinct release times are allowed [Cho and Sahni, 1978]. The m = 2 preemptive case can be solved in polynomial time even if arbitrary release times are allowed, and the general preemptive case with arbitrary release times and deadlines can be solved by linear programming [Cho and Sahni, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let A = {a_1, ..., a_k} be a PARTITION instance with total sum 2Q (so Q = S/2 is the target half-sum).

Following Gonzalez and Sahni (1976), construct an open-shop instance with m = 3 machines and k + 1 jobs:

1. **Element jobs:** For each element a_j (j = 1, ..., k), create a job J_j with processing times p_{1,j} = p_{2,j} = p_{3,j} = a_j (same length on all three machines).
2. **Special job:** Create one additional job J_{k+1} with p_{1,k+1} = p_{2,k+1} = p_{3,k+1} = Q. This "big" job has processing time Q on each of the three machines.
3. **Deadline:** Set D = 3Q. The total processing time of J_{k+1} alone is 3Q (since it uses each machine for Q time units, and tasks of the same job cannot overlap across machines in the open-shop model). So J_{k+1} takes at least 3Q total elapsed time, making D = 3Q the tightest possible deadline.
4. **Correctness (forward):** If a balanced partition A' (summing to Q) and A \ A' (summing to Q) exists, then:
   - Schedule J_{k+1}'s three tasks consecutively: machine 1 during [0, Q), machine 2 during [Q, 2Q), machine 3 during [2Q, 3Q).
   - The element jobs whose indices are in A' are scheduled in the idle gaps before J_{k+1} on each machine, and those in A \ A' are scheduled in the idle gaps after J_{k+1}, carefully filling exactly Q time units in each gap.
   - All jobs complete by time 3Q = D.
5. **Correctness (backward):** If a schedule with makespan <= 3Q exists, then J_{k+1} occupies each machine for Q consecutive time units, creating exactly two idle blocks of size Q on some machine. The element jobs filling these blocks correspond to a valid partition.
6. **Solution extraction:** Identify which element jobs share time blocks with which side of J_{k+1}'s execution on a given machine; the two groups form the partition.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- k = number of elements in the PARTITION instance
- Q = S/2 = half the total sum

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_jobs`                 | `num_elements + 1` (= k + 1)    |
| `num_machines`             | 3                                |
| `deadline`                 | `3 * total_sum / 2` (= 3Q)      |
| `num_tasks_total`          | `3 * (num_elements + 1)`        |

**Derivation:** Each partition element becomes one job with 3 tasks (one per machine), plus one special job. The number of machines is constant (3). The deadline is 3Q = 3S/2. Construction is O(k).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to OPEN-SHOP SCHEDULING with 3 machines and k+1 jobs, solve by brute-force enumeration of all feasible open-shop schedules (assign start times to each task on each machine, respecting non-overlap and same-job disjointness), verify the makespan <= 3Q iff a balanced partition exists.
- Check that the constructed instance has k+1 jobs, 3 machines, and deadline D = 3Q.
- Edge cases: test with odd total sum (no partition exists, expect makespan > 3Q), k = 2 equal elements (trivial partition), k = 5 with a known partition.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {4, 5, 3, 2, 6} (k = 5 elements)
Total sum S = 20, Q = 10.
Balanced partition: A' = {4, 6} (sum = 10), A \ A' = {5, 3, 2} (sum = 10).

**Constructed OPEN-SHOP SCHEDULING instance:**

m = 3 machines, 6 jobs, deadline D = 30.

| Job   | p_1 (machine 1) | p_2 (machine 2) | p_3 (machine 3) |
|-------|-----------------|-----------------|-----------------|
| J_1   | 4               | 4               | 4               |
| J_2   | 5               | 5               | 5               |
| J_3   | 3               | 3               | 3               |
| J_4   | 2               | 2               | 2               |
| J_5   | 6               | 6               | 6               |
| J_6   | 10              | 10              | 10              |

J_6 is the special job with processing time Q = 10 on each machine.

**Solution (sketch):**
Schedule J_6 as: machine 1 in [0, 10), machine 2 in [10, 20), machine 3 in [20, 30).

On machine 1: J_6 occupies [0, 10). Jobs J_1 and J_5 (from A' = {4, 6}) fill [10, 14) and [14, 20). Jobs J_2, J_3, J_4 fill [20, 25), [25, 28), [28, 30).

On machine 2: J_6 occupies [10, 20). Jobs from A' fill [0, 4) and [4, 10) on machine 2. Jobs from A \ A' fill [20, 25), [25, 28), [28, 30).

On machine 3: J_6 occupies [20, 30). The remaining jobs fill [0, 10) and [10, 20) similarly.

(The exact feasible schedule requires careful assignment ensuring same-job tasks don't overlap. The key insight is that the partition into {4, 6} and {5, 3, 2} enables filling the gaps around J_6.)

All jobs complete by time 30 = D.

**Solution extraction:**
Partition: A' = {a_1, a_5} = {4, 6} (sum = 10) and A \ A' = {a_2, a_3, a_4} = {5, 3, 2} (sum = 10). Balanced partition.


## References

- **[Gonzalez and Sahni, 1976]**: [`Gonzalez1976`] T. Gonzalez and S. Sahni (1976). "Open shop scheduling to minimize finish time". *Journal of the Association for Computing Machinery* 23, pp. 665–679.
- **[Lenstra, 1977]**: [`Lenstra1977`] Jan K. Lenstra (1977). "".
- **[Cho and Sahni, 1978]**: [`Cho1978`] Y. Cho and S. Sahni (1978). "Preemptive scheduling of independent jobs with release and due times on open, flow, and job shops".
