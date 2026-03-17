---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to BIN PACKING"
labels: rule
assignees: ''
canonical_source_name: 'Partition'
canonical_target_name: 'Bin Packing'
source_in_codebase: false
target_in_codebase: true
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** BIN PACKING
**Motivation:** Establishes NP-completeness of BIN PACKING via polynomial-time reduction from PARTITION. This is one of the most natural and well-known reductions in combinatorial optimization: a set of integers can be split into two equal-sum halves if and only if the same integers (as item sizes) can be packed into exactly 2 bins of capacity S/2. BIN PACKING is NP-complete in the strong sense (also via reduction from 3-PARTITION), but this simpler reduction from PARTITION suffices for weak NP-completeness.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SR1, p.226

## GJ Source Entry

> [SR1] BIN PACKING
> INSTANCE: Finite set U of items, a size s(u)∈Z^+ for each u∈U, a positive integer bin capacity B, and a positive integer K.
> QUESTION: Is there a partition of U into disjoint sets U_1,U_2,…,U_K such that the sum of the sizes of the items in each U_i is B or less?
> Reference: Transformation from PARTITION, 3-PARTITION.
> Comment: NP-complete in the strong sense. NP-complete and solvable in pseudo-polynomial time for each fixed K≥2. Solvable in polynomial time for any fixed B by exhaustive search.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION instance A = {a_1, ..., a_n} with sizes s(a_i) ∈ Z^+ and total sum S = Σ s(a_i), construct a BIN PACKING instance as follows:

1. **Items:** For each element a_i ∈ A, create an item u_i with size s(u_i) = s(a_i). The item set U = {u_1, ..., u_n}.
2. **Bin capacity:** Set B = ⌊S/2⌋. (If S is odd, there is no balanced partition, and 2 bins of capacity ⌊S/2⌋ cannot hold all items since ⌊S/2⌋ + ⌊S/2⌋ = S - 1 < S.)
3. **Number of bins:** Set K = 2.

**Correctness:**
- **(PARTITION feasible → BIN PACKING feasible):** If there exists A' ⊆ A with Σ_{a ∈ A'} s(a) = S/2, then place items corresponding to A' in bin 1 and the rest in bin 2. Each bin has total size exactly S/2 = B. ✓
- **(BIN PACKING feasible → PARTITION feasible):** If U can be packed into 2 bins of capacity B = S/2, then the items in bin 1 form a subset with sum ≤ S/2, and items in bin 2 also have sum ≤ S/2. Since the total sum is S, each bin must have sum exactly S/2. The items in bin 1 correspond to a valid partition half A'. ✓
- **(Odd S case):** If S is odd, no balanced partition exists and packing into 2 bins of capacity ⌊S/2⌋ is impossible (total capacity 2⌊S/2⌋ = S - 1 < S). Both answers are NO. ✓

**Note on the codebase BinPacking model:** The codebase's `BinPacking<W>` is an **optimization problem** (minimize number of bins used). The reduction encodes PARTITION feasibility: PARTITION is feasible if and only if the optimal BIN PACKING value is ≤ 2 (i.e., the minimum number of bins is exactly 2, or 1 if all items fit in one bin -- but since S/2 < S for n ≥ 2, we need exactly 2 bins).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |A| = number of elements in the PARTITION instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_items`                | `num_items` (= n)                |

**Derivation:** Each PARTITION element maps to exactly one BIN PACKING item (n items total). The bin capacity B = ⌊S/2⌋ is a scalar data parameter, not a structural size field. K = 2 is a constant. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to BinPacking, solve with BruteForce (`find_best`), verify the optimal number of bins is 2 iff a balanced partition exists (and > 2 otherwise).
- Solution extraction: from the optimal BinPacking assignment, items assigned to bin 0 form one partition half A', items in bin 1 form A \ A'. Verify both halves sum to S/2.
- Edge cases: test with odd total sum (optimal bins > 2, no balanced partition), all-equal elements (trivially partitionable if n is even), single element (no balanced partition, needs 1 bin).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {5, 3, 8, 2, 4, 6} (n = 6 elements)
Total sum S = 5 + 3 + 8 + 2 + 4 + 6 = 28; target half-sum = 14.
A balanced partition exists: A' = {8, 6} (sum = 14), A \ A' = {5, 3, 2, 4} (sum = 14).
(Note: greedy picking largest-first gives {8,5}=13 then stuck -- need to pick {8,6} instead.)

**Constructed BIN PACKING instance:**

| Item i | Size s(u_i) |
|--------|-------------|
| 0      | 5           |
| 1      | 3           |
| 2      | 8           |
| 3      | 2           |
| 4      | 4           |
| 5      | 6           |

Bin capacity B = 14. Number of bins K = 2.

**Optimal solution:**
- Bin 0: items {2, 5} (sizes 8, 6). Total = 14 ≤ 14 ✓
- Bin 1: items {0, 1, 3, 4} (sizes 5, 3, 2, 4). Total = 14 ≤ 14 ✓
- Bins used: 2 ≤ K = 2 ✓ → BIN PACKING is feasible → PARTITION is feasible.

**Solution extraction:**
- Bin 0 items → A' = {8, 6} (sum = 14)
- Bin 1 items → A \ A' = {5, 3, 2, 4} (sum = 14)
- Balanced partition confirmed. ✓

**Negative example:**
A = {7, 3, 5, 2, 4} (n = 5, sum S = 21, odd).
B = ⌊21/2⌋ = 10, K = 2. Total capacity = 20 < 21.
Cannot pack 5 items of total size 21 into 2 bins of capacity 10. → BIN PACKING infeasible → PARTITION infeasible. ✓

## References

- **[Garey & Johnson, 1979]**: [`garey1979`] Michael R. Garey and David S. Johnson (1979). *Computers and Intractability: A Guide to the Theory of NP-Completeness*. W.H. Freeman.
- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
