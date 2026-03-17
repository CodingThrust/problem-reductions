---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Scheduling to Minimize Weighted Completion Time"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Partition
**Target:** Scheduling to Minimize Weighted Completion Time
**Motivation:** PARTITION asks whether a multiset of integers can be split into two equal-sum halves; SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME asks whether tasks with lengths and weights can be scheduled on m processors so that the total weighted completion time is at most K. By setting m = 2 and choosing weights equal to lengths, a balanced partition of tasks across two processors minimizes the weighted completion cost. This reduction establishes NP-completeness for weighted completion time scheduling even with just two processors, and the problem remains NP-complete whether or not preemption is allowed.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.240-241

## GJ Source Entry

> [SS13] SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME
> INSTANCE: Set T of tasks, number m E Z+ of processors, for each task t E T a length l(t) E Z+ and a weight w(t) E Z+, and a positive integer K.
> QUESTION: Is there an m-processor schedule σ for T such that the sum, over all t E T, of (σ(t) + l(t))*w(t) is no more than K?
> Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from PARTITION.
> Comment: Remains NP-complete for m = 2, and is NP-complete in the strong sense for m arbitrary [Lageweg and Lenstra, 1977]. The problem is solvable in pseudo-polynomial time for fixed m. These results continue to hold if "preemptive" schedules are allowed [McNaughton, 1959]. Can be solved in polynomial time if all lengths are equal (by matching techniques). If instead all weights are equal, it can be solved in polynomial time even for "different speed" processors [Conway, Maxwell, and Miller, 1967] and for "unrelated" processors [Horn, 1973], [Bruno, Coffman, and Sethi, 1974]. The "preemptive" case for different speed processors also can be solved in polynomial time [Gonzalez, 1977]. If precedence constraints are allowed, the original problem is NP-complete in the strong sense even if all weights are equal, m = 2, and the partial order is either an "in-tree" or an "out-tree" [Sethi, 1977a]. If resources are allowed, the same subcases men-tioned under RESOURCE CONSTRAINED SCHEDULING are NP-complete, even for equal weights [Blazewicz, 1977a].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let A = {a_1, ..., a_n} with s(a_i) in Z+ be a PARTITION instance, and let S = sum_{i=1}^{n} s(a_i).

1. **Tasks:** For each element a_i, create a task t_i with length l(t_i) = s(a_i) and weight w(t_i) = s(a_i) (weight equals length).
2. **Processors:** Set m = 2.
3. **Bound K:** Compute the threshold K based on S. On a single processor, if tasks are ordered by Smith's rule (w/l ratio, which is 1 for all tasks here), the weighted completion time is sum_j l(t_pi(j)) * w(t_pi(j)) + cross terms. The bound K is set so that it can be achieved if and only if the two processor loads are each S/2. Specifically, K = sum_{i=1}^{n} s(a_i)^2 + (S/2)^2 ... the exact formula is: for two processors with loads L_1 and L_2 (L_1 + L_2 = S), the minimum weighted completion cost (with w = l, no precedence, optimal ordering on each processor) depends on the load balance. The bound K is chosen as the value achieved when L_1 = L_2 = S/2.
4. **Correctness (forward):** If a balanced partition exists (A' summing to S/2), assign tasks of A' to processor 1 and the rest to processor 2. Each processor has load S/2. Ordering tasks on each processor by shortest-first (Smith's rule with w = l) achieves the minimum weighted completion time, which equals K.
5. **Correctness (backward):** If a schedule achieves weighted completion time at most K, the load must be balanced (L_1 = L_2 = S/2) since any imbalance increases the objective beyond K. This gives a valid partition.
6. **Solution extraction:** The partition is A' = {a_i : t_i assigned to processor 1}.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in PARTITION instance
- S = total sum of element sizes

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_tasks`                | `num_elements` (= n)             |
| `num_processors`           | 2                                |
| `bound_K`                  | O(S^2)                           |

**Derivation:** Each element maps 1:1 to a task with identical length and weight. The number of processors is constant (2). The bound K is a polynomial function of S. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME with m = 2, solve by brute-force enumeration of all 2^n task-to-processor assignments (and optimal ordering on each processor), verify the minimum weighted completion time is at most K iff a balanced partition exists.
- Check that the constructed instance has exactly n tasks, m = 2 processors, and all weights equal lengths.
- Edge cases: test with odd total sum S (no balanced partition exists, expect weighted completion time > K), n = 2 with equal elements (trivial partition), n = 5 with known partition.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {4, 5, 3, 2, 6} (n = 5 elements)
Total sum S = 4 + 5 + 3 + 2 + 6 = 20
A balanced partition exists: A' = {4, 6} (sum = 10) and A \ A' = {5, 3, 2} (sum = 10).

**Constructed SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME instance:**

| Task t_i | Length l(t_i) | Weight w(t_i) |
|----------|--------------|---------------|
| t_1      | 4            | 4             |
| t_2      | 5            | 5             |
| t_3      | 3            | 3             |
| t_4      | 2            | 2             |
| t_5      | 6            | 6             |

Number of processors m = 2.

**Solution:**
Assign {t_1, t_5} to processor 1 (load = 10), {t_2, t_3, t_4} to processor 2 (load = 10).

Processor 1 (ordered by length: t_1 then t_5):
- t_1 completes at time 4, contribution = 4 * 4 = 16
- t_5 completes at time 10, contribution = 10 * 6 = 60
- Processor 1 total = 76

Processor 2 (ordered by length: t_4, t_3, t_2):
- t_4 completes at time 2, contribution = 2 * 2 = 4
- t_3 completes at time 5, contribution = 5 * 3 = 15
- t_2 completes at time 10, contribution = 10 * 5 = 50
- Processor 2 total = 69

Total weighted completion time = 76 + 69 = 145 = K.

**Solution extraction:**
Partition: A' = {a_1, a_5} = {4, 6} (sum = 10) and A \ A' = {a_2, a_3, a_4} = {5, 3, 2} (sum = 10). Balanced partition.


## References

- **[Lenstra, Rinnooy Kan, and Brucker, 1977]**: [`Lenstra1977a`] Jan K. Lenstra and A. H. G. Rinnooy Kan and Peter Brucker (1977). "Complexity of machine scheduling problems". *Annals of Discrete Mathematics* 1, pp. 343–362.
- **[Lageweg and Lenstra, 1977]**: [`Lageweg1977`] B. J. Lageweg and Jan K. Lenstra (1977). "".
- **[McNaughton, 1959]**: [`McNaughton1959`] Robert McNaughton (1959). "Scheduling with deadlines and loss functions". *Management Science* 6, pp. 1–12.
- **[Conway, Maxwell, and Miller, 1967]**: [`Conway1967`] R. W. Conway and W. L. Maxwell and L. W. Miller (1967). "Theory of Scheduling". Addison-Wesley, Reading, MA.
- **[Horn, 1973]**: [`Horn1973`] William A. Horn (1973). "Minimizing average flow time with parallel machines". *Operations Research* 21, pp. 846–847.
- **[Bruno, Coffman, and Sethi, 1974]**: [`Bruno1974`] J. Bruno and E. G. Coffman, Jr and R. Sethi (1974). "Scheduling independent tasks to reduce mean finishing time". *Communications of the ACM* 17, pp. 382–387.
- **[Gonzalez, 1977]**: [`Gonzalez1977`] T. Gonzalez (1977). "Optimal mean finish time preemptive schedules". Computer Science Dept., Pennsylvania State University.
- **[Sethi, 1977a]**: [`Sethi1977a`] R. Sethi (1977). "On the complexity of mean flow time scheduling". *Mathematics of Operations Research* 2, pp. 320–330.
- **[Blazewicz, 1977a]**: [`Blazewicz1977a`] J. Blazewicz (1977). "Mean flow time scheduling under resource constraints". Technical University of Poznan.
