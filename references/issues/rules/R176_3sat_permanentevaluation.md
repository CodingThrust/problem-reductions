---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PERMANENT EVALUATION"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Permanent Evaluation'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** PERMANENT EVALUATION
**Motivation:** Establishes NP-hardness of Permanent Evaluation via polynomial-time reduction from 3SAT. Valiant (1979) proved that computing the permanent of a 0-1 matrix is #P-complete, meaning it is at least as hard as counting the number of satisfying assignments to any Boolean formula. The decision version -- determining whether perm(M) equals a given value K -- is NP-hard (but not known to be in NP). The reduction constructs a bipartite graph from a 3-CNF formula using variable, clause, and XOR gadgets, where the permanent of the biadjacency matrix encodes the count of satisfying assignments.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.252

## GJ Source Entry

> [AN13] PERMANENT EVALUATION (*)
> INSTANCE: An n*n matrix M of 0's and 1's, and a positive integer K <= n!.
> QUESTION: Is the value of the permanent of M equal to K?
> Reference: [Valiant, 1977a]. Transformation from 3SAT.
> Comment: The problem is NP-hard but not known to be in NP, as is the case for the variants in which we ask whether the value of the permanent is "K or less" or "K or more." The problem of computing the value of the permanent of M is #P-complete.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance (equivalently #3SAT) with n variables {x_1, ..., x_n} and m clauses {C_1, ..., C_m}, construct a 0-1 matrix M and an integer K such that perm(M) = K if and only if the formula is satisfiable (or more precisely, K encodes information about the number of satisfying assignments).

The construction uses the equivalence between the permanent of a 0-1 matrix and the number of cycle covers (or perfect matchings in bipartite graphs) in the corresponding directed graph:

1. **Cycle cover characterization:** perm(M) = sum over all permutations sigma of product M[i, sigma(i)]. Equivalently, perm(M) counts the number of cycle covers in the directed graph G where there is an edge from i to j iff M[i,j] = 1.

2. **Variable gadgets:** For each variable x_i, construct a variable gadget -- a small directed graph component with exactly two cycle covers: one corresponding to x_i = TRUE and one to x_i = FALSE. The gadget has a "true chain" and a "false chain" of edges, with the chain length equal to the number of clauses in which x_i appears.

3. **Clause gadgets:** For each clause C_j = (l_1 OR l_2 OR l_3), construct a clause gadget -- a small directed graph component. The clause gadget has the property that it contributes to cycle covers only when at least one of its three input edges is "active" (corresponding to a satisfied literal). Specifically, for each non-empty subset of satisfied literals, the clause gadget has exactly one cycle cover using the corresponding external edges.

4. **XOR gadgets:** For each occurrence of a literal in a clause, an XOR gadget connects the variable gadget to the clause gadget. The XOR gadget is a small graph fragment with 4 vertices that ensures exactly one of its two external edge pairs is used in any cycle cover, contributing a weight of 4 per gadget. This enforces that the variable's truth value is consistently communicated to each clause.

5. **Matrix construction:** The combined directed graph G is converted to its adjacency matrix M (a 0-1 matrix). The order n of M equals the total number of vertices across all gadgets.

6. **Target value K:** Set K = (number of satisfying assignments) * 4^(3m), where the factor 4^(3m) accounts for the weight contributed by the 3m XOR gadgets (each contributes a factor of 4).

7. **Solution extraction:** If perm(M) = K > 0, the formula is satisfiable. If perm(M) = 0 (which would require K = 0), the formula is unsatisfiable. In practice, K is set so that perm(M) = K iff at least one satisfying assignment exists.

**Key property:** Each satisfying assignment contributes exactly 4^(3m) to the permanent (one unit of cycle-cover weight per XOR gadget combination). Non-satisfying assignments contribute 0 (the clause gadgets block the cycle cover). Thus perm(M) = (number of satisfying assignments) * 4^(3m).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in source 3SAT instance
- m = number of clauses in source 3SAT instance
- L = total number of literal occurrences (sum of clause lengths, at most 3m)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `matrix_size` (n in target, dimension of M) | O(n + m + L) = O(n + m) since L <= 3m |
| `num_ones` (number of 1-entries in M) | O(n + m + L) = O(n + m) |

**Derivation:**
- Variable gadgets contribute O(L_i) vertices per variable, where L_i is the number of clause occurrences of x_i. Total: O(L) = O(3m) vertices.
- Clause gadgets contribute O(1) vertices per clause. Total: O(m) vertices.
- XOR gadgets contribute 4 vertices each, with L total gadgets. Total: O(4L) = O(m) vertices.
- Overall matrix dimension: O(n + m) (more precisely, the constant factors give roughly 4L + 3m + something proportional to n, but all terms are O(n + m)).
- Each gadget contributes a constant number of edges (1-entries), so the total number of 1s is also O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Construct a small 3SAT instance (e.g., 2 variables, 2 clauses) and build the matrix M.
- Compute perm(M) by brute force (feasible for small matrices, say 10x10 or smaller).
- Count satisfying assignments of the 3SAT formula by brute force.
- Verify that perm(M) = (number of satisfying assignments) * 4^(3m).
- Test with an unsatisfiable formula and verify perm(M) = 0 (or perm(M) != K for any K encoding satisfiability).
- Check matrix dimensions match the overhead formulas.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
2 variables: x_1, x_2 (n = 2)
2 clauses (m = 2):
- C_1 = (x_1 OR x_2)
- C_2 = (NOT x_1 OR x_2)

**Satisfying assignments:**
- (T, T): C_1 satisfied (x_1), C_2 satisfied (x_2). Yes.
- (T, F): C_1 satisfied (x_1), C_2: NOT x_1 = F, x_2 = F. Not satisfied. No.
- (F, T): C_1 satisfied (x_2), C_2 satisfied (NOT x_1 and x_2). Yes.
- (F, F): C_1 not satisfied. No.

Number of satisfying assignments = 2.

**Constructed matrix:**
The reduction builds a directed graph with:
- 2 variable gadgets (one per variable), each with chain length equal to the number of clause appearances.
  - x_1 appears in C_1 (positive) and C_2 (negative): chain length 2.
  - x_2 appears in C_1 (positive) and C_2 (positive): chain length 2.
- 2 clause gadgets (one per clause), each with 3 vertices.
- 4 XOR gadgets (one per literal occurrence: x_1 in C_1, x_2 in C_1, NOT x_1 in C_2, x_2 in C_2), each with 4 vertices.

Total vertices: approximately 2*2 (variable) + 2*3 (clause) + 4*4 (XOR) = 4 + 6 + 16 = 26.
Matrix M is 26 x 26 (0-1 matrix).

**Expected permanent:**
perm(M) = 2 * 4^(3*2) = 2 * 4^6 = 2 * 4096 = 8192.

Setting K = 8192, we have perm(M) = K, confirming satisfiability.

**Negative example:**
For the unsatisfiable formula (x_1) AND (NOT x_1), we would have 0 satisfying assignments, so perm(M) = 0 * 4^3 = 0 != K for any positive K.


## References

- **[Valiant, 1977a]**: [`Valiant1977a`] Leslie G. Valiant (1977). "The complexity of computing the permanent". Computer Science Department, University of Edinburgh.
- **[Valiant, 1979]**: Leslie G. Valiant (1979). "The Complexity of Computing the Permanent". *Theoretical Computer Science* 8(2), pp. 189-201.
