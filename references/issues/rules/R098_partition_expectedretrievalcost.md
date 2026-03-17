---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition / 3-Partition to Expected Retrieval Cost"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION / 3-PARTITION'
canonical_target_name: 'EXPECTED RETRIEVAL COST'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Partition / 3-Partition
**Target:** Expected Retrieval Cost
**Motivation:** Establishes NP-completeness of EXPECTED RETRIEVAL COST by encoding a PARTITION (or 3-PARTITION) instance as a record-allocation problem on a drum-like storage device. The key insight is that the latency cost function on a circular arrangement of m sectors captures the balance constraint of PARTITION: if records are distributed unevenly by probability weight across sectors, the expected rotational latency increases. When m = 2, the problem reduces exactly to deciding whether the records can be split into two equal-probability groups, which is PARTITION. For strong NP-completeness (via 3-PARTITION), m sectors with m groups of 3 records each are used.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.1 [SR4], p.227

## GJ Source Entry

> [SR4] EXPECTED RETRIEVAL COST
> INSTANCE: Set R of records, rational probability p(r) ∈ [0,1] for each r ∈ R, with ∑_{r ∈ R} p(r) = 1, number m of sectors, and a positive integer K.
> QUESTION: Is there a partition of R into disjoint subsets R_1, R_2, ..., R_m such that, if p(R_i) = ∑_{r ∈ R_i} p(r) and the "latency cost" d(i,j) is defined to be j−i−1 if 1 ≤ i < j ≤ m and to be m−i+j−1 if 1 ≤ j ≤ i ≤ m, then the sum over all ordered pairs i,j, 1 ≤ i,j ≤ m, of p(R_i)·p(R_j)·d(i,j) is at most K?
> Reference: [Cody and Coffman, 1976]. Transformation from PARTITION, 3-PARTITION.
> Comment: NP-complete in the strong sense. NP-complete and solvable in pseudo-polynomial time for each fixed m ≥ 2.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary (PARTITION → EXPECTED RETRIEVAL COST with m = 2):**

Given a PARTITION instance: a finite set A = {a_1, ..., a_n} with sizes s(a_i) ∈ Z⁺ and total sum B = ∑ s(a_i), construct an Expected Retrieval Cost instance as follows:

1. **Records:** For each element a_i ∈ A, create a record r_i with probability p(r_i) = s(a_i) / B. Since ∑ s(a_i) = B, we have ∑ p(r_i) = 1.

2. **Sectors:** Set m = 2 sectors.

3. **Latency cost:** With m = 2, the circular latency function gives d(1,1) = 0, d(2,2) = 0, d(1,2) = 0 (since j − i − 1 = 2 − 1 − 1 = 0), and d(2,1) = m − i + j − 1 = 2 − 2 + 1 − 1 = 0. Wait — with m = 2 the latency is degenerate. The meaningful reduction uses m ≥ 3 or a more careful encoding.

**Summary (3-PARTITION → EXPECTED RETRIEVAL COST, strong sense):**

Given a 3-PARTITION instance: a set A = {a_1, ..., a_{3m}} of 3m positive integers with total sum m·B, where B/4 < a_i < B/2 for all i (so each group must have exactly 3 elements summing to B), construct an Expected Retrieval Cost instance:

1. **Records:** For each element a_i, create a record r_i with probability p(r_i) = a_i / (m·B). The probabilities sum to 1.

2. **Sectors:** Use m sectors (matching the 3-PARTITION parameter m).

3. **Bound K:** Set K to the expected latency cost that would result if the records could be distributed with each sector having total probability exactly 1/m (i.e., a perfectly balanced allocation). This value can be computed from the latency formula: for a perfectly balanced allocation where p(R_i) = 1/m for all i, the total cost equals (1/m²) · ∑_{i,j} d(i,j).

4. **Correctness (forward):** If a valid 3-partition exists (each group of 3 elements sums to B), then assigning the corresponding records to sectors gives p(R_i) = B/(m·B) = 1/m for each sector. The resulting expected retrieval cost equals K (the balanced cost).

5. **Correctness (reverse):** If the expected retrieval cost is at most K, the allocation must be perfectly balanced (each sector has probability 1/m), because any imbalance strictly increases the quadratic latency cost. This means each sector contains records whose original sizes sum to exactly B, yielding a valid 3-partition.

6. **Solution extraction:** Given a valid record allocation achieving cost ≤ K, the partition groups are G_i = {a_j : r_j ∈ R_i} for i = 1, ..., m.

**Key invariant:** The quadratic nature of the latency cost (products p(R_i)·p(R_j)) is minimized when the probability mass is distributed as evenly as possible across sectors. A cost of exactly K is achievable if and only if a perfectly balanced partition exists.

**Time complexity of reduction:** O(n) to compute probabilities and the bound K.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in the source PARTITION / 3-PARTITION instance
- m = number of groups in the 3-PARTITION instance (n = 3m)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_records`              | `num_elements` (= n = 3m)       |
| `num_sectors`              | `num_groups` (= m = n/3)        |

**Derivation:** Each element of the source instance maps to exactly one record. The number of sectors equals the number of groups in the 3-PARTITION instance. The bound K is computed from the latency formula in O(m²) time.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3-PARTITION instance, reduce to Expected Retrieval Cost, solve target by brute-force enumeration of all partitions of n records into m sectors, verify the allocation achieving cost ≤ K corresponds to a valid 3-partition.
- Test with known YES instance: A = {5, 6, 7, 5, 6, 7} with m = 2, B = 18; valid groups {5,6,7} and {5,6,7} should give a balanced allocation with cost = K.
- Test with known NO instance: A = {1, 1, 1, 10, 10, 10} with m = 2, B = 16.5 (non-integer, so no valid 3-partition); verify no allocation achieves cost ≤ K.
- Verify that the cost function is indeed minimized at balanced allocations by testing with small m values.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3-PARTITION):**
A = {5, 6, 7, 5, 6, 7} (n = 6 elements, m = 2 groups)
B = (5+6+7+5+6+7)/2 = 18, target group sum = 18.
Valid 3-partition: G_1 = {5, 6, 7} (sum = 18) and G_2 = {5, 6, 7} (sum = 18).

**Constructed target instance (ExpectedRetrievalCost):**
- Records: r_1 through r_6 with probabilities:
  - p(r_1) = 5/36, p(r_2) = 6/36 = 1/6, p(r_3) = 7/36
  - p(r_4) = 5/36, p(r_5) = 6/36 = 1/6, p(r_6) = 7/36
  - Sum = 36/36 = 1 ✓
- Sectors: m = 2
- Latency costs: d(1,2) = 2−1−1 = 0, d(2,1) = 2−2+1−1 = 0. With m = 2, all latency costs are 0 — this is the degenerate case.

**Corrected example with m = 3 sectors (n = 9 elements):**

**Source instance (3-PARTITION):**
A = {3, 3, 4, 2, 4, 4, 3, 5, 2} (n = 9 elements, m = 3 groups)
Total sum = 30, B = 10, each group must sum to 10.
Valid 3-partition: G_1 = {3, 3, 4}, G_2 = {2, 4, 4}, G_3 = {3, 5, 2}.

**Constructed target instance (ExpectedRetrievalCost):**
- Records: r_1, ..., r_9 with p(r_i) = a_i/30
  - p(r_1) = 3/30 = 1/10, p(r_2) = 1/10, p(r_3) = 4/30 = 2/15
  - p(r_4) = 2/30 = 1/15, p(r_5) = 2/15, p(r_6) = 2/15
  - p(r_7) = 1/10, p(r_8) = 5/30 = 1/6, p(r_9) = 1/15
  - Sum = 30/30 = 1 ✓
- Sectors: m = 3
- Latency costs (circular, m = 3):
  - d(1,1) = 0, d(1,2) = 0, d(1,3) = 1
  - d(2,1) = 1, d(2,2) = 0, d(2,3) = 0
  - d(3,1) = 0, d(3,2) = 1, d(3,3) = 0
- Bound K: For balanced allocation with p(R_i) = 1/3 for all i:
  K = ∑_{i,j} p(R_i)·p(R_j)·d(i,j) = (1/3)²·[0+0+1+1+0+0+0+1+0] = (1/9)·3 = 1/3.

**Solution mapping:**
- Assign R_1 = {r_1, r_2, r_3} (elements {3,3,4}): p(R_1) = 10/30 = 1/3 ✓
- Assign R_2 = {r_4, r_5, r_6} (elements {2,4,4}): p(R_2) = 10/30 = 1/3 ✓
- Assign R_3 = {r_7, r_8, r_9} (elements {3,5,2}): p(R_3) = 10/30 = 1/3 ✓
- Cost = (1/3)²·3 = 1/3 ≤ K = 1/3 ✓

**Verification:**
- Each sector has probability mass exactly 1/3 → perfectly balanced → minimum latency cost.
- Extracting element groups: G_1 = {3,3,4} sum 10 ✓, G_2 = {2,4,4} sum 10 ✓, G_3 = {3,5,2} sum 10 ✓.


## References

- **[Cody and Coffman, 1976]**: [`Cody1976`] R. A. Cody and E. G. Coffman, Jr (1976). "Record allocation for minimizing expected retrieval costs on drum-like storage devices". *Journal of the Association for Computing Machinery* 23(1), pp. 103-115.
- **[Garey and Johnson, 1978]**: [`Garey1978`] M. R. Garey and D. S. Johnson (1978). "'Strong' NP-completeness results: Motivation, examples, and implications." *Journal of the ACM* 25(3), pp. 499-508.
