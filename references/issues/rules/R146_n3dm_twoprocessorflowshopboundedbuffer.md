---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Numerical 3-Dimensional Matching to Two-Processor Flow-Shop with Bounded Buffer"
labels: rule
assignees: ''
canonical_source_name: 'NUMERICAL 3-DIMENSIONAL MATCHING'
canonical_target_name: 'TWO-PROCESSOR FLOW-SHOP WITH BOUNDED BUFFER'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Numerical 3-Dimensional Matching
**Target:** Two-Processor Flow-Shop with Bounded Buffer
**Motivation:** Establishes that Two-Processor Flow-Shop Scheduling with a bounded intermediate buffer of any fixed size B >= 1 is NP-complete in the strong sense. The reduction from Numerical 3-Dimensional Matching (N3DM) encodes the requirement that triples of elements must sum to a target value by using the buffer constraint to force jobs into groups that must satisfy numerical conditions, connecting the combinatorial structure of N3DM to the scheduling feasibility.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.242

## GJ Source Entry

> [SS17] TWO-PROCESSOR FLOW-SHOP WITH BOUNDED BUFFER
> INSTANCE: (Same as for FLOW-SHOP SCHEDULING with m = 2, with the addition of a "buffer bound" B E Z_0+.)
> QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and such that, for all u >= 0, the number of jobs j E J for which both σ_1(j) + l(t_1[j]) <= u and σ_2(j) > u does not exceed B?
> Reference: [Papadimitriou and Kanellakis, 1978]. Transformation from NUMERICAL 3-DIMENSIONAL MATCHING.
> Comment: NP-complete in the strong sense for any fixed B, 1 <= B < ∞. Solvable in polynomial time if B = 0 [Gilmore and Gomory, 1964] or if B >= |J| - 1 [Johnson, 1954].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given an N3DM instance: disjoint sets W, X, Y each of size m, with sizes s(a) in Z+ for each element a in W union X union Y, and bound B_target such that each triple must sum to B_target, construct a Two-Processor Flow-Shop with Bounded Buffer instance as follows:

1. **Processors:** m_proc = 2 (fixed).
2. **Buffer bound:** B_buf = 1 (or any desired fixed constant; the reduction works for any fixed B >= 1 by adjusting the encoding).
3. **Jobs:** Create jobs encoding the elements of W, X, and Y. The jobs are designed so that the buffer constraint of size B forces exactly one element from each of W, X, Y to be processed together in each "group."
   - For each element w_i in W: create a job with task lengths encoding s(w_i).
   - For each element x_j in X: create a job with task lengths encoding s(x_j).
   - For each element y_k in Y: create a job with task lengths encoding s(y_k).
   - Additionally, create "separator" or "enforcer" jobs that use the buffer to force the correct grouping structure.
4. **Task lengths:** The processing times on machine 1 and machine 2 are chosen so that:
   - The buffer constraint (at most B_buf jobs can be waiting between machines at any time) forces a specific interleaving structure.
   - Within each group of 3 element jobs, the total processing on machine 1 (or machine 2) corresponds to the sum s(w_i) + s(x_j) + s(y_k), and meeting the deadline requires this sum to equal B_target.
5. **Deadline:** D is set so the schedule is feasible iff the N3DM instance has a solution.

**Correctness:**
- (N3DM feasible -> scheduling feasible): A valid matching {(w_{i_k}, x_{j_k}, y_{l_k})} with each triple summing to B_target yields a job ordering where each group fits within its time window, respecting the buffer constraint.
- (Scheduling feasible -> N3DM feasible): The buffer constraint forces exactly 3 element jobs per group, and the deadline forces each group's sizes to sum to B_target.

**Solution extraction:** Read off the element jobs in each group from the schedule ordering.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- m = number of triples in N3DM (|W| = |X| = |Y| = m)
- n = 3m = total elements

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_processors`           | 2                                |
| `num_jobs`                 | O(m) = O(n)                      |
| `buffer_bound`             | 1 (constant)                     |
| `deadline`                 | O(m * B_target)                  |

**Derivation:** Each element maps to a job (plus possible separator jobs). The number of processors is constant (2). The buffer bound is a small constant. Construction is polynomial in m.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small N3DM instance (m = 2, so 6 elements from W, X, Y), reduce to a 2-processor bounded-buffer flow-shop instance, solve by enumerating all job permutations, verify that a feasible schedule (meeting deadline and buffer constraint) exists iff the N3DM instance has a valid matching.
- Verify buffer constraint: at every time point, count jobs that have finished on machine 1 but not started on machine 2 and confirm this count never exceeds B.
- Edge cases: test with an N3DM instance that has no valid matching (expect no feasible schedule).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Numerical 3-Dimensional Matching):**
W = {w_1, w_2}, X = {x_1, x_2}, Y = {y_1, y_2}, m = 2
Sizes: s(w_1) = 3, s(w_2) = 5, s(x_1) = 4, s(x_2) = 2, s(y_1) = 3, s(y_2) = 3
B_target = 10 (each triple must sum to 10).

Valid matching: (w_1, x_1, y_1) with 3 + 4 + 3 = 10, (w_2, x_2, y_2) with 5 + 2 + 3 = 10. ✓

**Constructed Two-Processor Bounded Buffer Flow-Shop instance (conceptual):**
- Processors: 2
- Buffer bound: B = 1
- Jobs encode the 6 elements plus enforcer jobs to create grouping structure.
- Task lengths on machine 1 and machine 2 encode the element sizes.
- Deadline D chosen so the schedule is tight when triples sum to 10.

**Solution:**
Schedule jobs in groups: first group {w_1-job, x_1-job, y_1-job} then {w_2-job, x_2-job, y_2-job}, with enforcer jobs interleaved to maintain buffer constraint.

**Solution extraction:**
- Group 1: w_1, x_1, y_1 -> sum = 3 + 4 + 3 = 10 ✓
- Group 2: w_2, x_2, y_2 -> sum = 5 + 2 + 3 = 10 ✓


## References

- **[Papadimitriou and Kanellakis, 1978]**: [`Papadimitriou1978e`] Christos H. Papadimitriou and P. C. Kanellakis (1980). "Flowshop scheduling with limited temporary storage". *Journal of the ACM* 27(3), pp. 533-549. (Note: cited as 1978 in GJ, published 1980.)
- **[Gilmore and Gomory, 1964]**: [`Gilmore1964`] P. C. Gilmore and R. E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem". *Operations Research* 12, pp. 655-679.
- **[Johnson, 1954]**: [`Johnson1954`] Selmer M. Johnson (1954). "Optimal two- and three-stage production schedules with setup times included". *Naval Research Logistics Quarterly* 1, pp. 61-68.
