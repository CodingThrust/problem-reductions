---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to DISJOINT CONNECTING PATHS"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'DISJOINT CONNECTING PATHS'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** DISJOINT CONNECTING PATHS
**Motivation:** Establishes NP-completeness of DISJOINT CONNECTING PATHS via polynomial-time reduction from 3SAT. This is one of the earliest reductions in network design theory, connecting propositional satisfiability to graph routing, and shows that even the decision version of multi-commodity routing is intractable.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND40, p.217

## GJ Source Entry

> [ND40] DISJOINT CONNECTING PATHS
> INSTANCE: Graph G=(V,E), collection of disjoint vertex pairs (s_1,t_1),(s_2,t_2),…,(s_k,t_k).
> QUESTION: Does G contain k mutually vertex-disjoint paths, one connecting s_i and t_i for each i, 1≤i≤k?
> Reference: [Knuth, 1974c], [Karp, 1975a], [Lynch, 1974]. Transformation from 3SAT.
> Comment: Remains NP-complete for planar graphs [Lynch, 1974], [Lynch, 1975]. Complexity is open for any fixed k≥2, but can be solved in polynomial time if k=2 and G is planar or chordal [Perl and Shiloach, 1978]. (A polynomial time algorithm for the general 2 path problem has been announced in [Shiloach, 1978]). The directed version of this problem is also NP-complete in general and solvable in polynomial time when k=2 and G is planar or acyclic [Perl and Shiloach, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}, construct a DISJOINT CONNECTING PATHS instance (G, terminal pairs) as follows:

1. **Variable gadgets:** For each variable u_i, create a "variable path" consisting of a chain of 2m+1 vertices: s_i = v_{i,0}, v_{i,1}, ..., v_{i,2m} = t_i. The terminal pair (s_i, t_i) must be connected by a path. The path can traverse this chain in two ways (using the "upper" or "lower" edges at each junction), encoding the assignment u_i = True or u_i = False.

2. **Clause gadgets:** For each clause c_j, create a terminal pair (s'_j, t'_j) with a clause vertex c_j. The vertex c_j is connected to two specific vertices in the variable chains corresponding to the literals appearing in clause c_j.

3. **Interconnection structure:** For each clause c_j = (l_1 ∨ l_2 ∨ l_3), add edges from s'_j and t'_j to the appropriate vertices on the variable chains. Specifically, if literal l_r appears in clause c_j, then c_j's clause vertex is connected to the junction point on variable chain i at position 2j (corresponding to clause j's slot), where the edge is on the "true side" if l_r = u_i and the "false side" if l_r = ¬u_i.

4. **Terminal pairs:** The instance has n + m terminal pairs total: (s_i, t_i) for i = 1..n (variable pairs) and (s'_j, t'_j) for j = 1..m (clause pairs).

5. **Correctness:** The variable path for u_i must route through either the "true" or "false" side at each junction (corresponding to truth assignment). A clause pair (s'_j, t'_j) can be connected only if at least one literal in c_j is satisfied, because the corresponding variable path leaves a junction vertex available for the clause path to use. Thus k = n + m vertex-disjoint paths exist if and only if the 3SAT formula is satisfiable.

6. **Solution extraction:** Given n + m vertex-disjoint paths, read off the truth assignment from which side of each variable chain is used by the variable path. For each variable u_i: if the variable path takes the "true" route, set u_i = True; otherwise set u_i = False.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n * m) — each variable chain has O(m) vertices, plus O(m) clause gadget vertices |
| `num_edges` | O(n * m) — chain edges plus interconnection edges |
| `num_pairs` | `num_vars + num_clauses` |

**Derivation:**
- Variable chain vertices: (2m + 1) per variable = n(2m + 1)
- Clause gadget vertices: O(m) additional vertices for clause terminals
- Total vertices: O(nm)
- Edges: O(nm) for chains plus O(m) for clause connections

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce KSatisfiability<K3> instance to DisjointConnectingPaths, solve target with BruteForce, extract solution, verify truth assignment satisfies all clauses on source
- Compare with known results from literature
- Test with both satisfiable and unsatisfiable 3SAT instances

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
3 variables: u_1, u_2, u_3 (n = 3)
2 clauses (m = 2):
- c_1 = (u_1 ∨ ¬u_2 ∨ u_3)
- c_2 = (¬u_1 ∨ u_2 ∨ ¬u_3)

**Constructed target instance (DISJOINT CONNECTING PATHS):**

Variable chains (5 vertices each, since 2m+1 = 5):
- Variable u_1 chain: s_1 = v_{1,0} — v_{1,1} — v_{1,2} — v_{1,3} — v_{1,4} = t_1
- Variable u_2 chain: s_2 = v_{2,0} — v_{2,1} — v_{2,2} — v_{2,3} — v_{2,4} = t_2
- Variable u_3 chain: s_3 = v_{3,0} — v_{3,1} — v_{3,2} — v_{3,3} — v_{3,4} = t_3

At each even position (2j for clause c_j), there are "true" and "false" side-vertices:
- Position 2 (clause c_1): true-side and false-side bypass vertices
- Position 4 (clause c_2): true-side and false-side bypass vertices

Clause terminal pairs:
- (s'_1, t'_1) for clause c_1
- (s'_2, t'_2) for clause c_2

Terminal pairs: 3 + 2 = 5 total.

**Solution mapping:**
- Satisfying assignment: u_1 = True, u_2 = False, u_3 = True (c_1: u_1 is True ✓; c_2: ¬u_3 = False, u_2 = False, but ¬u_1 = False — actually c_2: ¬u_1 = False, u_2 = False, ¬u_3 = False — NOT satisfied)
- Revised assignment: u_1 = True, u_2 = True, u_3 = True (c_1: u_1 = True ✓; c_2: u_2 = True ✓)
- Variable path u_1 takes "true" route at both clause junctions
- Variable path u_2 takes "true" route at both clause junctions
- Variable path u_3 takes "true" route at both clause junctions
- Clause c_1 path uses the available junction vertex freed by u_1's true-side route at position 2
- Clause c_2 path uses the available junction vertex freed by u_2's true-side route at position 4
- All 5 paths are vertex-disjoint ✓


## References

- **[Knuth, 1974c]**: [`Knuth1974c`] Donald E. Knuth (1974). "".
- **[Karp, 1975a]**: [`Karp1975a`] Richard M. Karp (1975). "On the complexity of combinatorial problems". *Networks* 5, pp. 45–68.
- **[Lynch, 1974]**: [`Lynch1974`] J. F. Lynch (1974). "The equivalence of theorem proving and the interconnection problem".
- **[Lynch, 1975]**: [`Lynch1975`] James F. Lynch (1975). "The equivalence of theorem proving and the interconnection problem". *ACM SIGDA Newsletter* 5(3).
- **[Perl and Shiloach, 1978]**: [`Perl1978`] Y. Perl and Y. Shiloach (1978). "Finding two disjoint paths between two pairs of vertices in a graph". *Journal of the Association for Computing Machinery* 25, pp. 1–9.
- **[Shiloach, 1978]**: [`Shiloach1978`] Yossi Shiloach (1978). "The two paths problem is polynomial". Computer Science Department, Stanford University.
