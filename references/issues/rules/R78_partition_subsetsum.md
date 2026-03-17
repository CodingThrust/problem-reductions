---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to SUBSET SUM"
labels: rule
assignees: ''
canonical_source_name: 'Partition'
canonical_target_name: 'Subset Sum'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** SUBSET SUM
**Motivation:** Establishes NP-completeness of SUBSET SUM via polynomial-time reduction from PARTITION. This is a textbook-canonical reduction: PARTITION is the special case of SUBSET SUM where the target B equals half the total sum. The reduction is essentially an embedding -- the PARTITION instance is directly re-interpreted as a SUBSET SUM instance with B = S/2. Referenced in Karp (1972) and Garey & Johnson (1979, SP13).
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SP13, p.223

## GJ Source Entry

> [SP13] SUBSET SUM
> INSTANCE: Finite set A, size s(a) in Z^+ for each a in A, positive integer B.
> QUESTION: Is there a subset A' <= A such that the sum of the sizes of the elements in A' is exactly B?
> Reference: [Karp, 1972]. Transformation from PARTITION.
> Comment: Solvable in pseudo-polynomial time (see Section 4.2).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a PARTITION instance (finite set A with sizes s(a) in Z^+), construct a SUBSET SUM instance as follows:

1. **Items:** Keep the same set A with the same sizes s(a). That is, the SUBSET SUM item set is identical to the PARTITION element set.
2. **Target:** Set B = S / 2, where S = Sum_{a in A} s(a) is the total sum of all element sizes. If S is odd, the PARTITION instance has no solution; in this case, there is no integer B such that a balanced partition exists, and one can set B = floor(S/2) (the SUBSET SUM instance will have no solution either, since no subset sums to exactly (S-1)/2 + 1/2).
3. **Correctness:** PARTITION asks: "Is there A' <= A with Sum_{a in A'} s(a) = Sum_{a in A\A'} s(a)?" This is equivalent to asking "Is there A' <= A with Sum_{a in A'} s(a) = S/2?", which is exactly the SUBSET SUM question with target B = S/2. The reduction is a trivial embedding.
4. **Solution extraction:** The SUBSET SUM solution A' (the subset summing to B = S/2) is directly one side of the balanced partition. The other side is A \ A'.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |A| = `num_items` of source PARTITION instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_items`                | `num_items` (= n)                |

**Derivation:** The SUBSET SUM instance has exactly the same n items as the PARTITION instance. The target B is computed as S/2, a data parameter. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source PARTITION instance, solve target SUBSET SUM with BruteForce, extract solution, verify on source
- Compare with known results from literature
- Edge cases: test with odd total sum (no solution), test with all-equal elements (always solvable if n is even)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {a_1, a_2, a_3, a_4, a_5, a_6, a_7} with sizes s = {5, 3, 8, 2, 7, 1, 4} (n = 7 elements)
Total sum S = 5 + 3 + 8 + 2 + 7 + 1 + 4 = 30
A balanced partition exists iff a subset summing to S/2 = 15 can be found.

**Constructed SUBSET SUM instance:**
Items: same 7 elements with sizes {5, 3, 8, 2, 7, 1, 4}
Target B = 15.

**Solution:**
Select A' = {a_1, a_3, a_4} = {5, 8, 2} (sum = 5 + 8 + 2 = 15 = B). YES.

**Solution extraction:**
Partition side 1: A' = {5, 8, 2} (sum = 15)
Partition side 2: A \ A' = {3, 7, 1, 4} (sum = 3 + 7 + 1 + 4 = 15)
Balanced partition confirmed.

**Negative case:**
A = {5, 3, 8, 2, 7, 1, 3} (n = 7 elements), total sum S = 29 (odd).
B = floor(29/2) = 14. No subset sums to exactly 14.5, so PARTITION has no solution.
SUBSET SUM with target 14: might or might not have a solution, but that is irrelevant -- the PARTITION answer is NO because S is odd. (The reduction handles odd-sum cases by detecting them upfront.)


## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
