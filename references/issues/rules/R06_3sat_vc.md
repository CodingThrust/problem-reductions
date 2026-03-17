---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-SATISFIABILITY (3SAT) to VERTEX COVER"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'VERTEX COVER'
source_in_codebase: true
target_in_codebase: true
---

**Source:** 3-SATISFIABILITY (3SAT)
**Target:** VERTEX COVER
**Motivation:** Establishes NP-completeness of VERTEX COVER via polynomial-time reduction from 3SAT, making VC one of the foundational NP-complete problems from which hundreds of other reductions proceed.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.3, p.54-56

## Reduction Algorithm

> Theorem 3.3 VERTEX COVER is NP-complete.
> Proof: It is easy to see that VC E NP since a nondeterministic algorithm need only guess a subset of vertices and check in polynomial time whether that subset contains at least one endpoint of every edge and has the appropriate size.
>
> We transform 3SAT to VERTEX COVER. Let U = {u1,u2, . . . , un} and C = {c1,c2, . . . , cm} be any instance of 3SAT. We must construct a graph G = (V,E) and a positive integer K <= |V| such that G has a vertex cover of size K or less if and only if C is satisfiable.
>
> As in the previous proof, the construction will be made up of several components. In this case, however, we will have only truth-setting components and satisfaction testing components, augmented by some additional edges for communicating between the various components.
>
> For each variable ui E U, there is a truth-setting component T_i = (V_i,E_i), with V_i = {ui,u-bar_i} and E_i = {{ui,u-bar_i}}, that is, two vertices joined by a single edge. Note that any vertex cover will have to contain at least one of ui and u-bar_i in order to cover the single edge in E_i.
>
> For each clause cj E C, there is a satisfaction testing component S_j = (V'_j,E'_j), consisting of three vertices and three edges joining them to form a triangle:
>
> V'_j = {a1[j],a2[j],a3[j]}
> E'_j = {{a1[j],a2[j]},{a1[j],a3[j]},{a2[j],a3[j]}}
>
> Note that any vertex cover will have to contain at least two vertices from V'_j in order to cover the edges in E'_j.
>
> The only part of the construction that depends on which literals occur in which clauses is the collection of communication edges. These are best viewed from the vantage point of the satisfaction testing components. For each clause cj E C, let the three literals in cj be denoted by xj, yj, and zj. Then the communication edges emanating from S_j are given by:
>
> E''_j = {{a1[j],xj},{a2[j],yj},{a3[j],zj}}
>
> The construction of our instance of VC is completed by setting K = n + 2m and G = (V,E), where
>
> V = (U (i=1 to n) V_i) U (U (j=1 to m) V'_j)
>
> and
>
> E = (U (i=1 to n) E_i) U (U (j=1 to m) E'_j) U (U (j=1 to m) E''_j)
>
> Figure 3.3 shows an example of the graph obtained when U = {u1,u2,u3,u4} and C = {{u1,u-bar_3,u-bar_4},{u-bar_1,u2,u-bar_4}}.
>
> It is easy to see how the construction can be accomplished in polynomial time. All that remains to be shown is that C is satisfiable if and only if G has a vertex cover of size K or less.
>
> First, suppose that V' ⊆ V is a vertex cover for G with |V'| <= K. By our previous remarks, V' must contain at least one vertex from each T_i and at least two vertices from each S_j. Since this gives a total of at least n+2m = K vertices, V' must in fact contain exactly one vertex from each T_i and exactly two vertices from each S_j. Thus we can use the way in which V' intersects each truth-setting component to obtain a truth assignment t: U->{T,F}. We merely set t(ui) = T if ui E V' and t(ui) = F if u-bar_i E V'. To see that this truth assignment satisfies each of the clauses cj E C, consider the three edges in E''_j. Only two of those edges can be covered by vertices from V'_j ∩ V', so one of them must be covered by a vertex from some V_i that belongs to V'. But that implies that the corresponding literal, either ui or u-bar_i, from clause cj is true under the truth assignment t, and hence clause cj is satisfied by t. Because this holds for every cj E C, it follows that t is a satisfying truth assignment for C.
>
> Conversely, suppose that t: U->{T,F} is a satisfying truth assignment for C. The corresponding vertex cover V' includes one vertex from each T_i and two vertices from each S_j. The vertex from T_i in V' is ui if t(ui) = T and is u-bar_i if t(ui) = F. This ensures that at least one of the three edges from each set E''_j is covered, because t satisfies each clause cj. Therefore we need only include in V' the endpoints from S_j of the other two edges in E''_j (which may or may not also be covered by vertices from truth-setting components), and this gives the desired vertex cover.

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a KSatisfiability<K3> instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}, construct a MinimumVertexCover instance (G, K) as follows:

1. **Truth-setting components:** For each variable u_i, add two vertices {u_i, ¬u_i} connected by an edge. This creates n edges among 2n vertices. Exactly one of each pair must be in any minimum vertex cover (encoding the truth assignment).

2. **Clause triangle components:** For each clause c_j = (l_1 ∨ l_2 ∨ l_3), add three fresh vertices {a_1^j, a_2^j, a_3^j} forming a triangle (3 edges). At least two of these must be in any minimum vertex cover.

3. **Communication edges:** For each clause c_j, connect a_1^j → l_1, a_2^j → l_2, a_3^j → l_3, where l_k is the truth-setting vertex corresponding to the literal (u_i or ¬u_i as appropriate). This adds 3m edges.

4. **Cover size parameter:** Set K = n + 2m.

5. **Solution extraction:** Given a vertex cover V' of size K, read off the truth assignment from which truth-setting vertex is included per pair (u_i ∈ V' means u_i = True). The satisfying assignment is guaranteed by the connectivity structure.

**Vertex count:** 2n (truth-setting) + 3m (clause triangles) = 2n + 3m vertices total.
**Edge count:** n (truth-setting pairs) + 3m (triangle edges) + 3m (communication edges) = n + 6m edges total.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `2 * num_vars + 3 * num_clauses` |
| `num_edges` | `num_vars + 6 * num_clauses` |

**Derivation:**
- Vertices: 2 per variable (u_i and ¬u_i) + 3 per clause (triangle vertices) = 2n + 3m
- Edges: 1 per variable (truth-setting edge) + 3 per clause (triangle) + 3 per clause (communication) = n + 3m + 3m = n + 6m

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce KSatisfiability<K3> instance to MinimumVertexCover, solve target with BruteForce, extract truth assignment from which truth-setting vertex is in cover, verify truth assignment satisfies all clauses
- Test with both satisfiable and unsatisfiable 3SAT instances to verify bidirectional correctness
- Check that cover size returned equals exactly n + 2m when satisfiable

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (KSatisfiability<K3>):**
3 variables: u_1, u_2, u_3 (n = 3)
3 clauses (m = 3):
- c_1 = (u_1 ∨ u_2 ∨ u_3)
- c_2 = (¬u_1 ∨ u_2 ∨ ¬u_3)
- c_3 = (¬u_1 ∨ ¬u_2 ∨ u_3)

**Constructed target instance (MinimumVertexCover):**

Vertices (2n + 3m = 6 + 9 = 15 total):
- Truth-setting: {u_1, ¬u_1, u_2, ¬u_2, u_3, ¬u_3} (vertices 0–5)
- Clause c_1 triangle: {a_1^1, a_2^1, a_3^1} (vertices 6, 7, 8)
- Clause c_2 triangle: {a_1^2, a_2^2, a_3^2} (vertices 9, 10, 11)
- Clause c_3 triangle: {a_1^3, a_2^3, a_3^3} (vertices 12, 13, 14)

Edges (n + 6m = 3 + 18 = 21 total):
- Truth-setting edges: {u_1, ¬u_1}, {u_2, ¬u_2}, {u_3, ¬u_3}  — 3 edges
- Triangle c_1: {a_1^1, a_2^1}, {a_1^1, a_3^1}, {a_2^1, a_3^1}  — 3 edges
- Triangle c_2: {a_1^2, a_2^2}, {a_1^2, a_3^2}, {a_2^2, a_3^2}  — 3 edges
- Triangle c_3: {a_1^3, a_2^3}, {a_1^3, a_3^3}, {a_2^3, a_3^3}  — 3 edges
- Communication c_1: {a_1^1, u_1}, {a_2^1, u_2}, {a_3^1, u_3}  — 3 edges
- Communication c_2: {a_1^2, ¬u_1}, {a_2^2, u_2}, {a_3^2, ¬u_3}  — 3 edges
- Communication c_3: {a_1^3, ¬u_1}, {a_2^3, ¬u_2}, {a_3^3, u_3}  — 3 edges

Cover size parameter: K = n + 2m = 3 + 6 = 9

**Solution mapping:**
- Satisfying assignment: u_1 = True, u_2 = True, u_3 = True (satisfies all 3 clauses)
- Truth-setting selection: include u_1, u_2, u_3 in cover (3 vertices)
- Clause triangle selection (include 2 per triangle, excluding the one connected to a satisfied literal):
  - c_1: literal u_1 is satisfied, so exclude a_1^1; include a_2^1, a_3^1
  - c_2: literal u_2 is satisfied, so exclude a_2^2; include a_1^2, a_3^2
  - c_3: literal u_3 is satisfied, so exclude a_3^3; include a_1^3, a_2^3
- Total vertex cover: {u_1, u_2, u_3, a_2^1, a_3^1, a_1^2, a_3^2, a_1^3, a_2^3} — 9 vertices = K ✓
- Verification: all 21 edges are covered by at least one endpoint in the cover ✓
