---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MAXIMUM LENGTH-BOUNDED DISJOINT PATHS"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'MAXIMUM LENGTH-BOUNDED DISJOINT PATHS'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** MAXIMUM LENGTH-BOUNDED DISJOINT PATHS
**Motivation:** Establishes NP-completeness of MAXIMUM LENGTH-BOUNDED DISJOINT PATHS via polynomial-time reduction from 3SAT. This result by Itai, Perl, and Shiloach (1977/1982) shows that bounding the length of vertex-disjoint s-t paths makes the counting/optimization problem intractable, in contrast to the unbounded case which is solvable by network flow.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND41, p.217

## GJ Source Entry

> [ND41] MAXIMUM LENGTH-BOUNDED DISJOINT PATHS
> INSTANCE: Graph G=(V,E), specified vertices s and t, positive integers J,K≤|V|.
> QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, none involving more than K edges?
> Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete for all fixed K≥5. Solvable in polynomial time for K≤4. Problem where paths need only be edge-disjoint is NP-complete for all fixed K≥5, polynomially solvable for K≤3, and open for K=4. The same results hold if G is a directed graph and the paths must be directed paths. The problem of finding the maximum number of disjoint paths from s to t, under no length constraint, is solvable in polynomial time by standard network flow techniques in both the vertex-disjoint and edge-disjoint cases.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}, construct a MAXIMUM LENGTH-BOUNDED DISJOINT PATHS instance (G, s, t, J, K) as follows:

1. **Source and sink:** Create two distinguished vertices s (source) and t (sink).

2. **Variable gadgets:** For each variable u_i, create two parallel paths of length K from s to t — a "true path" and a "false path." Each path passes through K-1 intermediate vertices. The path chosen for u_i encodes whether u_i is set to True or False. The two paths share only the endpoints s and t (plus possibly some clause-junction vertices).

3. **Clause enforcement:** For each clause c_j = (l_1 ∨ l_2 ∨ l_3), create an additional path structure connecting s to t that can be completed as a length-K path only if at least one of its literals is satisfied. This is done by inserting "crossing vertices" at specific positions along the variable paths. The clause path borrows a vertex from a satisfied literal's variable path, forcing the variable path to detour and thus become longer than K if the literal is false.

4. **Length bound:** Set K to a specific value (K ≥ 5 for the NP-complete case) that is determined by the construction to ensure that exactly one of the two variable paths (true or false) can stay within length K, while the other is forced to exceed K if a clause borrows its vertex.

5. **Path count:** Set J = n + m (one path per variable plus one per clause). The n variable paths encode the truth assignment; the m clause paths verify that each clause is satisfied.

6. **Correctness:** J vertex-disjoint s-t paths of length ≤ K exist if and only if the 3SAT formula is satisfiable. The length constraint K forces consistency in the truth assignment, and the clause paths can only be routed when at least one literal per clause is true.

7. **Solution extraction:** Given J vertex-disjoint paths of length ≤ K, for each variable u_i, check whether the "true path" or "false path" was used; set u_i accordingly.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)
- K = length bound (fixed constant ≥ 5 in the construction)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(K * (n + m)) — O(n + m) paths each of length O(K) |
| `num_edges` | O(K * (n + m)) — edges along paths plus crossing edges |
| `num_paths_required` (J) | `num_vars + num_clauses` |
| `length_bound` (K) | O(1) — fixed constant ≥ 5 |

**Derivation:**
- Each of the n variable gadgets has 2 paths of O(K) vertices = O(Kn) vertices
- Each of the m clause gadgets has O(K) vertices = O(Km) vertices
- Plus 2 vertices for s and t
- Total vertices: O(K(n + m)) + 2

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce KSatisfiability<K3> instance to MaximumLengthBoundedDisjointPaths, solve target with BruteForce, extract solution, verify truth assignment satisfies all clauses on source
- Compare with known results from literature
- Test with both satisfiable and unsatisfiable 3SAT instances
- Verify that the length bound K is respected by all paths in the solution

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
3 variables: u_1, u_2, u_3 (n = 3)
2 clauses (m = 2):
- c_1 = (u_1 ∨ u_2 ∨ ¬u_3)
- c_2 = (¬u_1 ∨ ¬u_2 ∨ u_3)

**Constructed target instance (MAXIMUM LENGTH-BOUNDED DISJOINT PATHS):**

Parameters: J = n + m = 5 paths required, K = 5 (length bound).

Graph structure:
- Vertices s and t (source and sink)
- For each variable u_i (i = 1,2,3): a true-path and false-path from s to t, each of length 5
  - True path for u_1: s — a_{1,1} — a_{1,2} — a_{1,3} — a_{1,4} — t
  - False path for u_1: s — b_{1,1} — b_{1,2} — b_{1,3} — b_{1,4} — t
  - (Similarly for u_2 and u_3)
- For each clause c_j (j = 1,2): a clause path from s to t that shares crossing vertices with the appropriate literal paths

**Solution mapping:**
- Satisfying assignment: u_1 = True, u_2 = True, u_3 = True
  - c_1: u_1 = True ✓
  - c_2: u_3 = True ✓
- Variable u_1 uses true-path, u_2 uses true-path, u_3 uses true-path
- Clause c_1 borrows a vertex from u_1's false-path (available since u_1 takes true-path)
- Clause c_2 borrows a vertex from u_3's false-path (available since u_3 takes true-path)
- All 5 paths are vertex-disjoint and each has length ≤ 5 ✓


## References

- **[Itai, Perl, and Shiloach, 1977]**: [`Itai1977b`] Alon Itai and Yehoshua Perl and Yossi Shiloach (1977). "The complexity of finding maximum disjoint paths with length constraints". Dept. of Computer Science, Technion.
