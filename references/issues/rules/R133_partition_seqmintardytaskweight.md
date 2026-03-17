---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Sequencing to Minimize Tardy Task Weight"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'SEQUENCING TO MINIMIZE TARDY TASK WEIGHT'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Partition
**Target:** Sequencing to Minimize Tardy Task Weight
**Motivation:** Establishes NP-completeness of SEQUENCING TO MINIMIZE TARDY TASK WEIGHT (the weighted number of tardy jobs on a single machine, 1||sum w_j U_j) via a simple reduction from PARTITION. When all tasks share a common deadline equal to half the total processing time and weights equal their lengths, deciding whether total tardy weight can be kept at or below half the total weight is equivalent to finding a balanced partition. This is one of Karp's original 21 NP-complete problems (1972), establishing scheduling with weighted tardiness as NP-hard even in its simplest non-trivial form. The problem generalizes KNAPSACK (common deadline = capacity, lengths = sizes, weights = values).
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.236-237; Karp (1972)

## GJ Source Entry

> [SS3] SEQUENCING TO MINIMIZE TARDY TASK WEIGHT
> INSTANCE: Set T of tasks, for each task t E T a length l(t) E Z+, a weight w(t) E Z+, and a deadline d(t) E Z+, and a positive integer K.
> QUESTION: Is there a one-processor schedule sigma for T such that the sum of w(t), taken over all t E T for which sigma(t) + l(t) > d(t), does not exceed K?
> Reference: [Karp, 1972]. Transformation from PARTITION.
> Comment: Can be solved in pseudo-polynomial time (time polynomial in |T|, sum l(t), and log sum w(t)) [Lawler and Moore, 1969]. Can be solved in polynomial time if weights are "agreeable" (i.e., w(t) < w(t') implies l(t) >= l(t')) [Lawler, 1976c].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION instance with set A = {a_1, ..., a_n} and sizes s(a_i), let B = sum s(a_i). Construct a SEQUENCING TO MINIMIZE TARDY TASK WEIGHT instance as follows:

1. **Tasks:** For each element a_i in A, create a task t_i with:
   - Length: l(t_i) = s(a_i)
   - Weight: w(t_i) = s(a_i) (weight equals length)
   - Deadline: d(t_i) = B/2 (common deadline for all tasks)

   (If B is odd, no balanced partition exists; output a trivially infeasible instance, e.g., d = 0 with K = 0.)

2. **Tardiness weight bound:** Set K = B/2.

3. **Correctness:** In any one-processor schedule (no precedence constraints, no preemption), the tasks form a sequence. Tasks that complete by time B/2 are "on-time"; tasks that complete after B/2 are "tardy". Since total processing time = B and deadline = B/2, exactly B/2 units of work can be completed by the deadline. The total tardy weight = sum of w(t) for tardy tasks. This is at most K = B/2 iff the on-time tasks have total weight (= length) >= B/2, i.e., iff there exists a subset of A summing to at least B/2. Since total = B, this means the on-time tasks sum to exactly B/2 and tardy tasks also sum to B/2 -- a balanced partition.

4. **Solution extraction:** The tasks completed by the deadline form A' (one half of the partition); the tardy tasks form A \ A'.

**Key invariant:** A balanced partition of A exists iff there is a schedule with total tardy weight <= B/2.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |A| = number of elements in PARTITION instance
- B = sum s(a_i) = total element sum

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_tasks`               | `num_elements` (= n)             |
| `common_deadline`         | `total_sum / 2` (= B/2)         |

**Derivation:** Each element maps to one task with the same length and weight. All deadlines are the same constant B/2. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to SEQUENCING TO MINIMIZE TARDY TASK WEIGHT, solve with BruteForce (try all n! permutations), check whether any ordering yields total tardy weight <= K.
- Verify that on-time tasks sum to exactly B/2 and tardy tasks sum to B/2 (a balanced partition).
- Edge cases: odd total sum (expect infeasible), all equal elements (many valid partitions), singleton (n=1, always infeasible as B/2 < s(a_1) = B).
- Verify that with no precedence constraints, the optimal schedule uses Shortest Processing Time (SPT) or similar ordering.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {3, 5, 2, 4, 1, 5} (n = 6 elements)
Total sum B = 3 + 5 + 2 + 4 + 1 + 5 = 20 (even)
Balanced partition: A' = {5, 5} (sum = 10) and A \ A' = {3, 2, 4, 1} (sum = 10).

**Constructed SEQUENCING TO MINIMIZE TARDY TASK WEIGHT instance:**

| Task | Length l | Weight w | Deadline d |
|------|----------|----------|------------|
| t_1  | 3        | 3        | 10         |
| t_2  | 5        | 5        | 10         |
| t_3  | 2        | 2        | 10         |
| t_4  | 4        | 4        | 10         |
| t_5  | 1        | 1        | 10         |
| t_6  | 5        | 5        | 10         |

K = B/2 = 10.

**Schedule (one valid ordering):**
sigma: t_5, t_3, t_1, t_4, t_2, t_6

| Position | Task | Start | Finish | Deadline | Tardy? | Weight if tardy |
|----------|------|-------|--------|----------|--------|-----------------|
| 1        | t_5  | 0     | 1      | 10       | No     | -               |
| 2        | t_3  | 1     | 3      | 10       | No     | -               |
| 3        | t_1  | 3     | 6      | 10       | No     | -               |
| 4        | t_4  | 6     | 10     | 10       | No     | -               |
| 5        | t_2  | 10    | 15     | 10       | Yes    | 5               |
| 6        | t_6  | 15    | 20     | 10       | Yes    | 5               |

On-time tasks: {t_5, t_3, t_1, t_4} with total length = 1 + 2 + 3 + 4 = 10 = B/2
Tardy tasks: {t_2, t_6} with total tardy weight = 5 + 5 = 10 = K

Total tardy weight = 10 <= K = 10.

**Solution extraction:**
On-time tasks: {a_5, a_3, a_1, a_4} = {1, 2, 3, 4} -> A' (sum = 10)
Tardy tasks: {a_2, a_6} = {5, 5} -> A \ A' (sum = 10)
Balanced partition.


## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Lawler and Moore, 1969]**: [`Lawler1969`] Eugene L. Lawler and J. M. Moore (1969). "A functional equation and its application to resource allocation and sequencing problems". *Management Science* 16, pp. 77-84.
- **[Lawler, 1976c]**: [`Lawler1976c`] Eugene L. Lawler (1976). "Sequencing to minimize the weighted number of tardy jobs". *Revue Francaise d'Automatique, Informatique et Recherche Operationnelle, Serie Bleue* 10.5, pp. 27-33.
