---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to MINIMUM SUM OF SQUARES"
labels: rule
assignees: ''
canonical_source_name: 'Partition'
canonical_target_name: 'Minimum Sum of Squares'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** MINIMUM SUM OF SQUARES
**Motivation:** Establishes NP-completeness of MINIMUM SUM OF SQUARES via polynomial-time reduction from PARTITION. The reduction exploits the fact that among all 2-way partitions of a set of integers, the sum of squared group sums is minimized when the two groups have equal sums. Thus a balanced partition exists if and only if the minimum sum of squares equals (S/2)^2 + (S/2)^2 = S^2/2. This links the combinatorial PARTITION problem to a squared-norm minimization objective.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SP19, p.225

## GJ Source Entry

> [SP19] MINIMUM SUM OF SQUARES
> INSTANCE: Finite set A, a size s(a) in Z^+ for each a in A, positive integers K<=|A| and J.
> QUESTION: Can A be partitioned into K disjoint sets A_1,A_2,...,A_K such that
> Sum_{i=1}^{K}(Sum_{a in A_i} s(a))^2 <= J ?
> Reference: Transformation from PARTITION or 3-PARTITION.
> Comment: NP-complete in the strong sense. NP-complete in the ordinary sense and solvable in pseudo-polynomial time for any fixed K. Variants in which the bound K on the number of sets is replaced by a bound B on either the maximum set cardinality or the maximum total set size are also NP-complete in the strong sense [Wong and Yao, 1976]. In all these cases, NP-completeness is preserved if the exponent 2 is replaced by any fixed rational alpha>1.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a PARTITION instance (finite set A with sizes s(a) in Z^+, total sum S = Sum s(a)), construct a MINIMUM SUM OF SQUARES instance as follows:

1. **Items:** Keep the same set A with the same sizes s(a).
2. **Number of groups:** Set K = 2 (partition into two groups).
3. **Bound on sum of squares:** Set J = S^2 / 2. (If S is odd, S^2 / 2 is not an integer; set J = floor(S^2 / 2) and the answer is NO since a balanced partition is impossible.)
4. **Correctness:** Let S_1 and S_2 = S - S_1 be the sums of the two groups. The sum of squares is S_1^2 + (S - S_1)^2. By calculus (or the identity S_1^2 + S_2^2 = (S_1 + S_2)^2 - 2*S_1*S_2 = S^2 - 2*S_1*S_2), this is minimized when S_1 = S_2 = S/2, giving a minimum of S^2/2. So:
   - If PARTITION has a solution (a balanced split with S_1 = S_2 = S/2), then the sum of squares equals S^2/2 = J, so the MINIMUM SUM OF SQUARES answer is YES.
   - If PARTITION has no solution, then S_1 != S_2 for all partitions, so S_1^2 + S_2^2 > S^2/2 = J for all partitions, and the answer is NO.
5. **Solution extraction:** Given the MINIMUM SUM OF SQUARES partition (A_1, A_2) achieving sum-of-squares <= J = S^2/2, the two groups A_1 and A_2 must each sum to S/2, directly yielding the balanced PARTITION solution.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |A| = `num_items` of source PARTITION instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_items`                | `num_items` (= n)                |
| `num_groups`               | 2                                |

**Derivation:** The MINIMUM SUM OF SQUARES instance has exactly the same n items as the PARTITION instance, with K = 2 groups. The bound J = S^2/2 is a data parameter derived from the input sizes. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source PARTITION instance, solve target MINIMUM SUM OF SQUARES with BruteForce, extract solution, verify on source
- Compare with known results from literature
- Verify the algebraic identity: for K = 2 and a balanced partition, sum-of-squares = S^2/2
- Edge cases: test with odd total sum (no balanced partition, sum-of-squares > S^2/2 for all partitions)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {a_1, a_2, a_3, a_4, a_5, a_6} with sizes s = {3, 1, 1, 2, 2, 1} (n = 6 elements)
Total sum S = 3 + 1 + 1 + 2 + 2 + 1 = 10
A balanced partition requires each half to sum to S/2 = 5.

**Constructed MINIMUM SUM OF SQUARES instance:**
Items: same 6 elements with sizes {3, 1, 1, 2, 2, 1}
K = 2 groups
J = S^2 / 2 = 100 / 2 = 50

**Solution:**
A_1 = {a_1, a_4} = {3, 2}, group sum = 5, squared = 25
A_2 = {a_2, a_3, a_5, a_6} = {1, 1, 2, 1}, group sum = 5, squared = 25
Sum of squares = 25 + 25 = 50 <= J = 50. YES.

**Solution extraction:**
Partition side 1: {3, 2} (sum = 5)
Partition side 2: {1, 1, 2, 1} (sum = 5)
Balanced partition confirmed.

**Imbalanced partition for comparison:**
A_1 = {3, 2, 1} (sum = 6), A_2 = {1, 2, 1} (sum = 4)
Sum of squares = 36 + 16 = 52 > 50 = J. Not feasible.

This demonstrates the key mechanism: only balanced partitions achieve sum-of-squares <= S^2/2.


## References

- **[Wong and Yao, 1976]**: [`Wong and Yao1976`] C. K. Wong and A. C. Yao (1976). "A combinatorial optimization problem related to data set allocation". *Revue Francaise d'Automatique, Informatique, Recherche Operationnelle Ser. Bleue* 10(suppl.), pp. 83-95.
