---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MAXIMUM FIXED-LENGTH DISJOINT PATHS"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'MAXIMUM FIXED-LENGTH DISJOINT PATHS'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** MAXIMUM FIXED-LENGTH DISJOINT PATHS
**Motivation:** Establishes NP-completeness of MAXIMUM FIXED-LENGTH DISJOINT PATHS via polynomial-time reduction from 3SAT. This result by Itai, Perl, and Shiloach (1977/1982) shows that requiring s-t paths to have exactly K edges (rather than at most K) preserves NP-hardness for K >= 4, tightening the complexity boundary compared to the length-bounded variant.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND42, p.218

## GJ Source Entry

> [ND42] MAXIMUM FIXED-LENGTH DISJOINT PATHS
> INSTANCE: Graph G=(V,E), specified vertices s and t, positive integers J,K≤|V|.
> QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, each involving exactly K edges?
> Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete for fixed K≥4. Solvable in polynomial time for K≤3. Corresponding problem for edge-disjoint paths is NP-complete for fixed K≥4, polynomially solvable for K≤2, and open for K=3. The same results hold for directed graphs and directed paths, except that the arc-disjoint version is polynomially solvable for K≤3 and open for K=4.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}, construct a MAXIMUM FIXED-LENGTH DISJOINT PATHS instance (G, s, t, J, K) as follows:

1. **Source and sink:** Create two distinguished vertices s (source) and t (sink).

2. **Variable gadgets:** For each variable u_i, create two alternative paths of exactly K edges from s to t — a "true path" and a "false path." Each path has exactly K-1 intermediate vertices. The choice of which path to use encodes u_i = True or u_i = False.

3. **Clause enforcement:** For each clause c_j = (l_1 ∨ l_2 ∨ l_3), create a clause-path structure that can form an s-t path of exactly K edges only if at least one of its literals is satisfied. This uses crossing vertices shared with the variable paths: a clause path borrows a vertex from a variable path whose literal is satisfied (meaning the variable uses the other path and frees the crossing vertex).

4. **Fixed length constraint:** Set K ≥ 4 (the threshold for NP-completeness). The exact-length requirement is stricter than the bounded-length case: paths cannot take shortcuts or detours, so the construction must carefully place crossing vertices at positions that preserve the exact path length of K.

5. **Path count:** Set J = n + m. The construction requires J vertex-disjoint s-t paths, each of exactly K edges.

6. **Correctness:** J vertex-disjoint s-t paths of exactly K edges exist if and only if the 3SAT formula is satisfiable. The fixed-length constraint prevents "cheating" by taking alternative routes that would allow unsatisfied clauses to still find paths.

7. **Solution extraction:** Given J vertex-disjoint paths each of length exactly K, determine which path (true or false) each variable u_i uses. Set u_i = True if the true-path is used.

**Key difference from ND41 (length-bounded):** The "exactly K" constraint (vs "at most K") means the construction must be more carefully designed to ensure no alternative paths of different lengths exist. The NP-completeness threshold is K ≥ 4 instead of K ≥ 5.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)
- K = fixed path length (constant ≥ 4 in the construction)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(K * (n + m)) — O(n + m) paths each of exactly K edges |
| `num_edges` | O(K * (n + m)) — edges along paths plus crossing edges |
| `num_paths_required` (J) | `num_vars + num_clauses` |
| `fixed_length` (K) | O(1) — fixed constant ≥ 4 |

**Derivation:**
- Each variable gadget has 2 paths of K-1 intermediate vertices = O(Kn) vertices
- Each clause gadget has O(K) vertices = O(Km) vertices
- Plus 2 vertices for s and t
- Total: O(K(n + m)) + 2

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce KSatisfiability<K3> instance to MaximumFixedLengthDisjointPaths, solve target with BruteForce, extract solution, verify truth assignment satisfies all clauses on source
- Compare with known results from literature
- Test with both satisfiable and unsatisfiable 3SAT instances
- Verify that all paths in the solution have exactly K edges (not just at most K)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
3 variables: u_1, u_2, u_3 (n = 3)
2 clauses (m = 2):
- c_1 = (u_1 ∨ ¬u_2 ∨ u_3)
- c_2 = (¬u_1 ∨ u_2 ∨ ¬u_3)

**Constructed target instance (MAXIMUM FIXED-LENGTH DISJOINT PATHS):**

Parameters: J = n + m = 5 paths required, K = 4 (fixed length).

Graph structure:
- Vertices s and t (source and sink)
- For each variable u_i (i = 1,2,3): two paths of exactly 4 edges from s to t
  - True path for u_1: s — a_{1,1} — a_{1,2} — a_{1,3} — t (4 edges)
  - False path for u_1: s — b_{1,1} — b_{1,2} — b_{1,3} — t (4 edges)
  - (Similarly for u_2 and u_3)
- For each clause c_j (j = 1,2): a clause path from s to t of exactly 4 edges that shares crossing vertices with literal paths

**Solution mapping:**
- Satisfying assignment: u_1 = True, u_2 = False, u_3 = True
  - c_1: u_1 = True ✓
  - c_2: u_2 = False, so ¬u_2 is not in c_2; but u_2 = False means... c_2 = (¬u_1 ∨ u_2 ∨ ¬u_3) = (False ∨ False ∨ False) — not satisfied
- Revised assignment: u_1 = True, u_2 = True, u_3 = True
  - c_1: u_1 = True ✓
  - c_2: u_2 = True ✓
- Variable paths: u_1 true-path, u_2 true-path, u_3 true-path (each exactly 4 edges)
- Clause c_1 path borrows crossing vertex from u_1's false-path (freed), forming exactly 4 edges
- Clause c_2 path borrows crossing vertex from u_2's false-path (freed), forming exactly 4 edges
- All 5 paths are vertex-disjoint, each with exactly 4 edges ✓


## References

- **[Itai, Perl, and Shiloach, 1977]**: [`Itai1977b`] Alon Itai and Yehoshua Perl and Yossi Shiloach (1977). "The complexity of finding maximum disjoint paths with length constraints". Dept. of Computer Science, Technion.
