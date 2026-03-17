---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Subset Sum to Capacity Assignment"
labels: rule
assignees: ''
canonical_source_name: 'Subset Sum'
canonical_target_name: 'Capacity Assignment'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Subset Sum
**Target:** Capacity Assignment
**Motivation:** Establishes NP-completeness of CAPACITY ASSIGNMENT via polynomial-time reduction from SUBSET SUM. The Capacity Assignment problem models a bicriteria optimization over communication links where each link must be assigned a capacity from a discrete set, balancing total cost against total delay penalty. The reduction from Subset Sum encodes the subset selection as a capacity choice, mapping the target-sum constraint into the cost/delay budget constraints.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SR7, p.227. [Van Sickle and Chandy, 1977].

## GJ Source Entry

> [SR7] CAPACITY ASSIGNMENT
> INSTANCE: Set C of communication links, set M ⊆ Z+ of capacities, cost function g: C×M → Z+, delay penalty function d: C×M → Z+ such that, for all c ∈ C and i < j ∈ M, g(c,i) ≤ g(c,j) and d(c,i) ≥ d(c,j), and positive integers K and J.
> QUESTION: Is there an assignment σ: C → M such that the total cost ∑_{c ∈ C} g(c,σ(c)) does not exceed K and such that the total delay penalty ∑_{c ∈ C} d(c,σ(c)) does not exceed J?
> Reference: [Van Sickle and Chandy, 1977]. Transformation from SUBSET SUM.
> Comment: Solvable in pseudo-polynomial time.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a SUBSET SUM instance with a set A = {a_1, a_2, ..., a_n} of positive integers and a target sum B, construct a CAPACITY ASSIGNMENT instance as follows:

1. **Communication links:** Create one link c_i for each element a_i in A. So |C| = n.

2. **Capacity set:** M = {1, 2} (two capacities: "low" and "high").

3. **Cost function:** For each link c_i:
   - g(c_i, 1) = 0 (low capacity has zero cost)
   - g(c_i, 2) = a_i (high capacity costs a_i)
   This satisfies g(c_i, 1) ≤ g(c_i, 2) since 0 ≤ a_i.

4. **Delay penalty function:** For each link c_i:
   - d(c_i, 1) = a_i (low capacity incurs delay a_i)
   - d(c_i, 2) = 0 (high capacity has zero delay)
   This satisfies d(c_i, 1) ≥ d(c_i, 2) since a_i ≥ 0.

5. **Cost budget:** K = B.

6. **Delay budget:** J = (∑_{i=1}^{n} a_i) - B.

7. **Correctness (forward):** If A' ⊆ A sums to B, assign σ(c_i) = 2 for a_i ∈ A' and σ(c_i) = 1 for a_i ∉ A'. Total cost = ∑_{a_i ∈ A'} a_i = B = K. Total delay = ∑_{a_i ∉ A'} a_i = (∑ a_i) - B = J.

8. **Correctness (reverse):** If σ is a feasible assignment, let A' = {a_i : σ(c_i) = 2}. Then ∑_{a_i ∈ A'} a_i ≤ K = B (cost constraint) and ∑_{a_i ∉ A'} a_i ≤ J = (∑ a_i) - B (delay constraint). Since ∑_{a_i ∈ A'} a_i + ∑_{a_i ∉ A'} a_i = ∑ a_i, both inequalities together force ∑_{a_i ∈ A'} a_i = B.

**Key invariant:** Choosing capacity 2 for link c_i corresponds to including a_i in the subset. The complementary cost/delay structure forces the subset sum to equal exactly B.

**Time complexity of reduction:** O(n) to construct the instance (plus O(n) to compute ∑ a_i).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in the Subset Sum instance (|A|)
- S = ∑_{i=1}^{n} a_i (sum of all elements)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_links` | `num_elements` |
| `num_capacities` | `2` |
| `cost_budget` | `target_sum` |
| `delay_budget` | `total_sum - target_sum` |

**Derivation:** One link per element, two capacities, cost/delay budgets determined by the target sum and its complement.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a SubsetSum instance to CapacityAssignment, solve target with BruteForce (enumerate all 2^n assignments), extract solution, verify on source
- Test with known YES instance: A = {3, 7, 1, 8, 4, 12}, B = 15, subset {3, 8, 4} sums to 15
- Test with known NO instance: A = {1, 2, 4, 8}, B = 6, no subset sums to 6 (check: {1,2} = 3 != 6, {2,4} = 6 -- actually YES); use A = {1, 5, 11, 5}, B = 12, no subset sums to 12
- Verify cost/delay monotonicity constraints are satisfied

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (SubsetSum):**
Set A = {3, 7, 1, 8, 4, 12}, target B = 15.
- Total sum S = 3 + 7 + 1 + 8 + 4 + 12 = 35
- Subset {3, 8, 4} sums to 15 = B. Answer: YES.

**Constructed target instance (CapacityAssignment):**
- Links: C = {c_1, c_2, c_3, c_4, c_5, c_6} (one per element)
- Capacities: M = {1, 2}
- Cost function g:
  - g(c_1,1)=0, g(c_1,2)=3
  - g(c_2,1)=0, g(c_2,2)=7
  - g(c_3,1)=0, g(c_3,2)=1
  - g(c_4,1)=0, g(c_4,2)=8
  - g(c_5,1)=0, g(c_5,2)=4
  - g(c_6,1)=0, g(c_6,2)=12
- Delay penalty function d:
  - d(c_1,1)=3, d(c_1,2)=0
  - d(c_2,1)=7, d(c_2,2)=0
  - d(c_3,1)=1, d(c_3,2)=0
  - d(c_4,1)=8, d(c_4,2)=0
  - d(c_5,1)=4, d(c_5,2)=0
  - d(c_6,1)=12, d(c_6,2)=0
- Cost budget: K = 15
- Delay budget: J = 35 - 15 = 20

**Solution mapping:**
- Subset {a_1=3, a_4=8, a_5=4} -> assign σ(c_1)=2, σ(c_4)=2, σ(c_5)=2 (high capacity); σ(c_2)=1, σ(c_3)=1, σ(c_6)=1 (low capacity)
- Total cost = g(c_1,2)+g(c_4,2)+g(c_5,2) = 3+8+4 = 15 ≤ K=15 ✓
- Total delay = d(c_2,1)+d(c_3,1)+d(c_6,1) = 7+1+12 = 20 ≤ J=20 ✓

**Verification:**
- Forward: subset {3,8,4} summing to 15 maps to feasible assignment with cost=15 and delay=20
- Reverse: any feasible assignment with cost ≤ 15 and delay ≤ 20 forces the "high capacity" links to sum to exactly 15, recovering a valid subset


## References

- **[Van Sickle and Chandy, 1977]**: [`van Sickle and Chandy1977`] Larry van Sickle and K. Mani Chandy (1977). "The complexity of computer network design problems". Technical report.
