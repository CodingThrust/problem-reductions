---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumSumOfSquares"
labels: model
assignees: ''
---

## Motivation

MINIMUM SUM OF SQUARES (P146) from Garey & Johnson, A3 SP19. Given a finite set of positive integers and bounds K (number of groups) and J, the problem asks whether the set can be partitioned into K groups such that the sum of the squared group sums is at most J. This is NP-complete in the strong sense, making it harder than weakly NP-complete problems like PARTITION. The squared objective penalizes imbalanced partitions, connecting this problem to variance minimization, load balancing, and k-means clustering. It generalizes PARTITION (K=2, J=(S/2)^2 + (S/2)^2) and 3-PARTITION (K=n/3 with cardinality constraints).

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R84 (PARTITION to MINIMUM SUM OF SQUARES)

## Definition

**Name:** `MinimumSumOfSquares`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Minimum Sum of Squares
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP19

**Mathematical definition:**

INSTANCE: Finite set A, a size s(a) in Z^+ for each a in A, positive integers K <= |A| and J.
QUESTION: Can A be partitioned into K disjoint sets A_1, A_2, ..., A_K such that
Sum_{i=1}^{K} (Sum_{a in A_i} s(a))^2 <= J ?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |A| (each element must be assigned to one of K groups)
- **Per-variable domain:** {0, 1, ..., K-1} -- the index of the group to which the element is assigned
- **Meaning:** x_i = g means element a_i is placed in group A_{g+1}. The assignment must cover all elements (every element in exactly one group), and the sum of squared group sums must not exceed J.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `MinimumSumOfSquares`
**Variants:** none (sizes and bounds are plain positive integers)

| Field        | Type        | Description                                                |
|--------------|-------------|------------------------------------------------------------|
| `sizes`      | `Vec<u64>`  | Positive integer size s(a) for each element a in A         |
| `num_groups` | `usize`     | Number of groups K in the partition                        |
| `bound`      | `u64`       | Upper bound J on the sum of squared group sums             |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** NP-complete in the strong sense, so no pseudo-polynomial time algorithm exists unless P = NP. For fixed K, the problem is solvable in pseudo-polynomial time via dynamic programming with complexity O(n * S^(K-1)), where S = Sum s(a). For general K, brute-force enumeration of all K^n partitions with pruning (Stirling numbers of the second kind bound the distinct partitions). The Korf-Schreiber-Moffitt hybrid algorithms (CKK + branch-and-bound, 2018) provide practical improvements for the related multiway number partitioning problem. Asymptotic worst case: O(K^n) for general K and n.

## Specialization

<!-- ⚠️ Unverified: AI-generated specialization -->

- PARTITION is a special case with K = 2 and J = S^2 / 2 (achievable iff a balanced partition exists, since (S/2)^2 + (S/2)^2 = S^2/2 is the minimum).
- 3-PARTITION can be seen as a related problem with K = n/3 and a cardinality constraint (each group has exactly 3 elements).
- The problem remains NP-complete in the strong sense when the exponent 2 is replaced by any fixed rational alpha > 1 (Wong & Yao, 1976).
- Variants replacing the K bound with a bound B on maximum set cardinality or maximum set size are also NP-complete in the strong sense.

## Extra Remark

**Full book text:**

INSTANCE: Finite set A, a size s(a) in Z^+ for each a in A, positive integers K <= |A| and J.
QUESTION: Can A be partitioned into K disjoint sets A_1,A_2,...,A_K such that
Sum_{i=1}^{K} (Sum_{a in A_i} s(a))^2 <= J ?
Reference: Transformation from PARTITION or 3-PARTITION.
Comment: NP-complete in the strong sense. NP-complete in the ordinary sense and solvable in pseudo-polynomial time for any fixed K. Variants in which the bound K on the number of sets is replaced by a bound B on either the maximum set cardinality or the maximum total set size are also NP-complete in the strong sense [Wong and Yao, 1976]. In all these cases, NP-completeness is preserved if the exponent 2 is replaced by any fixed rational alpha > 1.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all partitions of n elements into K groups; compute sum-of-squares for each and check <= J.)
- [x] It can be solved by reducing to integer programming. (Integer variables x_i in {0,...,K-1} for group assignment; auxiliary variables for group sums; quadratic constraint on sum of squares, linearizable via standard techniques.)
- [ ] Other: For fixed K, pseudo-polynomial DP in O(n * S^(K-1)) time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
A = {a_1, a_2, a_3, a_4, a_5, a_6} with sizes s = {5, 3, 8, 2, 7, 1} (n = 6 elements)
Total sum S = 5 + 3 + 8 + 2 + 7 + 1 = 26
K = 3 groups, J = 240.

**Feasible partition:**
A_1 = {a_3, a_6} = {8, 1}, group sum = 9, squared = 81
A_2 = {a_1, a_4} = {5, 2}, group sum = 7, squared = 49
A_3 = {a_2, a_5} = {3, 7}, group sum = 10, squared = 100
Sum of squares = 81 + 49 + 100 = 230 <= 240 = J. YES.

**Infeasible with tighter bound J = 225:**
The balanced partition above gives 230 > 225. The perfectly balanced partition would need group sums (26/3 ~ 8.67), which is impossible with integers. The best achievable is sums {9, 9, 8} giving 81 + 81 + 64 = 226 > 225. So for J = 225 the answer is NO.

Answer: YES for J = 240 (the partition {8,1}, {5,2}, {3,7} with sum-of-squares 230 witnesses feasibility).
