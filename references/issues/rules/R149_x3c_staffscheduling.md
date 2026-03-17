---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to Staff Scheduling"
labels: rule
assignees: ''
canonical_source_name: 'EXACT COVER BY 3-SETS'
canonical_target_name: 'STAFF SCHEDULING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** X3C
**Target:** Staff Scheduling
**Motivation:** EXACT COVER BY 3-SETS (X3C) asks whether a collection of 3-element subsets of a 3q-element set contains an exact cover; STAFF SCHEDULING asks whether at most n workers, each following a fixed binary schedule pattern, can collectively meet shift requirements. This reduction by Garey and Johnson (unpublished, referenced in their 1979 book) embeds X3C into the staff scheduling framework, showing that the combinatorial structure of exact covering directly encodes the difficulty of assigning workers to shifts. The result establishes NP-completeness of staff scheduling even in a simplified form.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.4, p.243

## GJ Source Entry

> [SS20] STAFF SCHEDULING
> INSTANCE: Positive integers m and k, a collection C of m-tuples, each having k 1's and m-k 0's (representing possible worker schedules), a "requirement" m-tuple R-bar of non-negative integers, and a number n of workers.
> QUESTION: Is there a schedule f: C → Z_0+ such that sum_{c-bar E C} f(c-bar) <= n and such that sum_{c-bar E C} f(c-bar)*c-bar >= R-bar?
> Reference: [Garey and Johnson, ——] Transformation from X3C.
> Comment: Solvable in polynomial time if every c-bar E C has the cyclic one's property, i.e., has all its 1's occuring in consecutive positions with position 1 regarded as following position m [Bartholdi, Orlin, and Ratliff, 1977]. (This corresponds to workers who are available only for consecutive hours of the day, or days of the week.)

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given an X3C instance with universe X = {x_1, ..., x_{3q}} and collection S = {S_1, ..., S_p} of 3-element subsets, construct a STAFF SCHEDULING instance as follows:

1. **Time periods (m):** Set m = 3q (one period per element of X). Set k = 3 (each worker schedule has exactly 3 shifts of 1, matching the 3-element subsets).
2. **Worker schedules (C):** For each subset S_j = {x_a, x_b, x_c} in S, create an m-tuple c_j with 1's in positions a, b, c and 0's elsewhere. The collection C = {c_1, ..., c_p}.
3. **Requirements (R-bar):** Set R-bar = (1, 1, ..., 1) — each period requires exactly 1 worker.
4. **Number of workers (n):** Set n = q (an exact cover uses exactly q subsets, covering all 3q elements with 3 elements each).
5. **Correctness:** An exact cover C' ⊆ S with |C'| = q exists if and only if there is a schedule f with f(c_j) = 1 for each S_j in C' and f(c_j) = 0 otherwise, using exactly q workers, such that the sum of scheduled tuples equals (1, 1, ..., 1) >= R-bar.
6. **Solution extraction:** Given a valid schedule f, the exact cover is C' = {S_j : f(c_j) >= 1}. Since each period has requirement 1 and each tuple contributes exactly 3 ones, exactly q tuples must be selected, each exactly once.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- q = |X|/3 (number of triples needed for exact cover)
- p = |S| = number of 3-element subsets in the X3C instance

| Target metric (code name)  | Polynomial (using symbols above)     |
|----------------------------|--------------------------------------|
| `num_periods` (m)          | 3 * q (= \|X\|)                     |
| `shifts_per_schedule` (k)  | 3 (constant)                         |
| `num_schedules` (\|C\|)   | p (= \|S\|)                         |
| `num_workers` (n)          | q                                    |

**Derivation:** Each 3-element subset maps directly to one m-tuple schedule. The number of periods equals the universe size. The number of workers equals the number of triples in an exact cover. Construction is O(p * q).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct an X3C instance, reduce to STAFF SCHEDULING, solve by brute-force enumeration of all subsets of C of size at most n, check if any combination meets R-bar component-wise. Verify the answer matches X3C solvability.
- Check that the constructed instance has m = 3q, k = 3, n = q, R-bar = all-ones, and |C| = p.
- Edge cases: no exact cover exists (expect no valid schedule), trivial cover (q = 1, one triple covering all 3 elements), overlapping subsets that prevent exact cover.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (X3C):**
X = {1, 2, 3, 4, 5, 6} (q = 2, so |X| = 6)
S = {S_1, S_2, S_3, S_4, S_5}:
- S_1 = {1, 2, 3}
- S_2 = {1, 3, 5}
- S_3 = {4, 5, 6}
- S_4 = {2, 4, 6}
- S_5 = {1, 4, 5}

Exact cover: S_1 ∪ S_3 = {1,2,3} ∪ {4,5,6} = X. ✓

**Constructed STAFF SCHEDULING instance:**
m = 6 periods, k = 3 shifts per schedule, n = 2 workers.
R-bar = (1, 1, 1, 1, 1, 1).

| Schedule | Period 1 | Period 2 | Period 3 | Period 4 | Period 5 | Period 6 |
|----------|----------|----------|----------|----------|----------|----------|
| c_1 (S_1) | 1 | 1 | 1 | 0 | 0 | 0 |
| c_2 (S_2) | 1 | 0 | 1 | 0 | 1 | 0 |
| c_3 (S_3) | 0 | 0 | 0 | 1 | 1 | 1 |
| c_4 (S_4) | 0 | 1 | 0 | 1 | 0 | 1 |
| c_5 (S_5) | 1 | 0 | 0 | 1 | 1 | 0 |

**Solution:**
f(c_1) = 1, f(c_3) = 1, f(c_2) = f(c_4) = f(c_5) = 0.
Total workers: f(c_1) + f(c_3) = 2 <= n = 2 ✓
Coverage: c_1 + c_3 = (1,1,1,0,0,0) + (0,0,0,1,1,1) = (1,1,1,1,1,1) >= R-bar ✓

**Solution extraction:**
Exact cover: C' = {S_1, S_3} = {{1,2,3}, {4,5,6}}. ✓


## References

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Bartholdi, Orlin, and Ratliff, 1977]**: [`Bartholdi1977`] J. J. Bartholdi, III and J. B. Orlin and H. D. Ratliff (1977). "Circular ones and cyclic staffing". Stanford University.
