---
name: Problem
about: Propose a new problem type
title: "[Model] Numerical3DimensionalMatching"
labels: model
assignees: ''
---

## Motivation

NUMERICAL 3-DIMENSIONAL MATCHING (P143) from Garey & Johnson, A3 SP16. A strongly NP-complete number-partition problem: given three disjoint sets W, X, Y each of size m with positive integer sizes, can they be partitioned into m triples (one element from each set) such that each triple sums to a given bound B? This is a numerical strengthening of 3-Dimensional Matching (3DM) -- while 3DM asks only about set membership in triples, N3DM additionally requires that sizes within each triple sum to a target. N3DM is a key intermediate problem for establishing strong NP-completeness of scheduling and packing problems.

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R81: 3-DIMENSIONAL MATCHING -> NUMERICAL 3-DIMENSIONAL MATCHING (establishes NP-completeness)
- R82: NUMERICAL 3-DIMENSIONAL MATCHING -> NUMERICAL MATCHING WITH TARGET SUMS
- R146: NUMERICAL 3-DIMENSIONAL MATCHING -> TWO-PROCESSOR FLOW-SHOP WITH BOUNDED BUFFER

## Definition

**Name:** `Numerical3DimensionalMatching`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP16

**Mathematical definition:**

INSTANCE: Disjoint sets W, X, and Y, each containing m elements, a size s(a) in Z^+ for each element a in W union X union Y, and a bound B in Z^+.
QUESTION: Can W union X union Y be partitioned into m disjoint sets A_1, A_2, ..., A_m such that each A_i contains exactly one element from each of W, X, and Y and such that, for 1 <= i <= m, sum_{a in A_i} s(a) = B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 3m (one assignment variable per element, or equivalently m matching variables each selecting one triple)
- **Per-variable domain:** For a matching formulation: variable k in {1, ..., m} selects which triple each element belongs to. Equivalently, m triple-selection variables each choosing one (w_i, x_j, y_l) triple.
- **Meaning:** The matching assigns each element of W, X, Y to exactly one of the m groups. Each group must contain exactly one element from each set, and the sum of the three element sizes in each group must equal B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `Numerical3DimensionalMatching`
**Variants:** none (sizes are positive integers)

| Field      | Type         | Description                                            |
|------------|--------------|--------------------------------------------------------|
| `sizes_w`  | `Vec<u64>`   | Sizes s(w_i) for elements of W (length m)              |
| `sizes_x`  | `Vec<u64>`   | Sizes s(x_j) for elements of X (length m)              |
| `sizes_y`  | `Vec<u64>`   | Sizes s(y_k) for elements of Y (length m)              |
| `bound`    | `u64`        | Target sum B that each triple must achieve             |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** N3DM is NP-complete in the strong sense [Garey and Johnson, 1979], meaning pseudo-polynomial algorithms do not exist unless P = NP. Brute-force: enumerate all m! ways to match W-elements to X-elements, then for each matching check if Y-elements can be assigned to complete valid triples. This gives O(m! * m) time. Dynamic programming on subsets can achieve O*(3^m) by tracking which elements from each set have been used. No known algorithm significantly improves upon exponential-time enumeration.

## Extra Remark

**Full book text:**

INSTANCE: Disjoint sets W, X, and Y, each containing m elements, a size s(a) in Z^+ for each element a in W union X union Y, and a bound B in Z^+.
QUESTION: Can W union X union Y be partitioned into m disjoint sets A_1,A_2,...,A_m such that each A_i contains exactly one element from each of W, X, and Y and such that, for 1 <= i <= m, sum_{a in A_i} s(a) = B?
Reference: [Garey and Johnson, ----]. Transformation from 3DM (see proof of Theorem 4.4).
Comment: NP-complete in the strong sense.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all possible matchings of W, X, Y into triples; check if each triple sums to B.)
- [x] It can be solved by reducing to integer programming. (ILP with binary variables x_{i,j,k} = 1 if (w_i, x_j, y_k) form a triple; constraints: each element used exactly once, and s(w_i) + s(x_j) + s(y_k) = B for each selected triple.)
- [ ] Other: Subset-DP in O*(3^m).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
W = {w_1, w_2, w_3}, X = {x_1, x_2, x_3}, Y = {y_1, y_2, y_3}, m = 3
Sizes: s(w_1) = 5, s(w_2) = 3, s(w_3) = 7, s(x_1) = 4, s(x_2) = 6, s(x_3) = 2, s(y_1) = 6, s(y_2) = 1, s(y_3) = 6
B = 15 (total sum = 5+3+7+4+6+2+6+1+6 = 40; but 3B = 45 != 40, so this won't work.)

Corrected: s(w_1) = 5, s(w_2) = 3, s(w_3) = 7, s(x_1) = 4, s(x_2) = 6, s(x_3) = 2, s(y_1) = 6, s(y_2) = 6, s(y_3) = 6
B = 15, total sum = 5+3+7+4+6+2+6+6+6 = 45 = 3 * 15. ✓

**Valid matching:**
- A_1 = {w_1, x_2, y_3} -> 5 + 6 + 6 = 17 != 15. No good.

Better example:
s(w_1) = 4, s(w_2) = 5, s(w_3) = 3, s(x_1) = 3, s(x_2) = 2, s(x_3) = 4, s(y_1) = 2, s(y_2) = 2, s(y_3) = 2
B = 9, total sum = 4+5+3+3+2+4+2+2+2 = 27 = 3 * 9. ✓
All sizes in Z+. ✓

**Valid matching:**
- A_1 = {w_1, x_2, y_1} -> 4 + 2 + 2 = 8 != 9. No.
- A_1 = {w_1, x_1, y_1} -> 4 + 3 + 2 = 9 ✓
- A_2 = {w_2, x_3, y_2} -> 5 + 4 + 2 = 11 != 9. No.
- A_2 = {w_2, x_2, y_2} -> 5 + 2 + 2 = 9 ✓
- A_3 = {w_3, x_3, y_3} -> 3 + 4 + 2 = 9 ✓

**Final valid example:**
W = {w_1, w_2, w_3}, X = {x_1, x_2, x_3}, Y = {y_1, y_2, y_3}, m = 3
Sizes: s(w_1) = 4, s(w_2) = 5, s(w_3) = 3, s(x_1) = 3, s(x_2) = 2, s(x_3) = 4, s(y_1) = 2, s(y_2) = 2, s(y_3) = 2
B = 9.

Matching:
- A_1 = (w_1, x_1, y_1): 4 + 3 + 2 = 9 ✓
- A_2 = (w_2, x_2, y_2): 5 + 2 + 2 = 9 ✓
- A_3 = (w_3, x_3, y_3): 3 + 4 + 2 = 9 ✓

Answer: YES -- a valid numerical 3-dimensional matching exists.
