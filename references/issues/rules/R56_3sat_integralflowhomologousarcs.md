---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INTEGRAL FLOW WITH HOMOLOGOUS ARCS"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability (3SAT)'
canonical_target_name: 'Integral Flow with Homologous Arcs'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** INTEGRAL FLOW WITH HOMOLOGOUS ARCS
**Motivation:** Establishes NP-completeness of INTEGRAL FLOW WITH HOMOLOGOUS ARCS via polynomial-time reduction from 3SAT. The reduction shows that requiring equal flow on paired ("homologous") arcs makes integer network flow intractable, even with unit capacities.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND35, p.215

## GJ Source Entry

> [ND35] INTEGRAL FLOW WITH HOMOLOGOUS ARCS
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, capacity c(a)∈Z^+ for each a∈A, requirement R∈Z^+, set H⊆A×A of "homologous" pairs of arcs.
> QUESTION: Is there a flow function f: A→Z_0^+ such that
> (1) f(a)≤c(a) for all a∈A,
> (2) for each v∈V−{s,t}, flow is conserved at v,
> (3) for all pairs <a,a'>∈H, f(a)=f(a'), and
> (4) the net flow into t is at least R?
> Reference: [Sahni, 1974]. Transformation from 3SAT.
> Comment: Remains NP-complete if c(a)=1 for all a∈A (by modifying the construction in [Even, Itai, and Shamir, 1976]). Corresponding problem with non-integral flows is polynomially equivalent to LINEAR PROGRAMMING [Itai, 1977].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct an INTEGRAL FLOW WITH HOMOLOGOUS ARCS instance as follows:

1. **Variable gadgets:** For each variable x_i, create a "diamond" subnetwork with two parallel paths from a node u_i to a node v_i. The upper path (arc a_i^T) represents x_i = TRUE, the lower path (arc a_i^F) represents x_i = FALSE. Set capacity 1 on each arc.

2. **Chain the variable gadgets:** Connect s -> u_1, v_1 -> u_2, ..., v_n -> t_0 in series, so that exactly one unit of flow passes through each variable gadget. The path chosen (upper or lower) encodes the truth assignment.

3. **Clause gadgets:** For each clause C_j, create an additional arc from s to t (or a small subnetwork) that requires one unit of flow. This flow must be "validated" by a literal satisfying C_j.

4. **Homologous arc pairs:** For each literal occurrence x_i in clause C_j, create a pair of homologous arcs: one arc in the variable gadget for x_i (the TRUE arc) and one arc in the clause gadget for C_j. The equal-flow constraint ensures that if the literal's truth path carries flow 1, then the clause gadget also receives flow validation. Similarly for negated literals using the FALSE arcs.

5. **Requirement:** Set R = n + m (n units for the assignment path through variable gadgets plus m units for clause satisfaction).

The 3SAT formula is satisfiable if and only if there exists an integral flow of value at least R satisfying all capacity and homologous-arc constraints.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n + m) where n = num_variables, m = num_clauses |
| `num_arcs` | O(n + m + L) where L = total literal occurrences (at most 3m) |
| `num_homologous_pairs` | O(L) = O(m) (one pair per literal occurrence) |
| `max_capacity` | 1 (unit capacities suffice) |
| `requirement` | n + m |

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source 3SAT instance, solve target integral flow with homologous arcs using BruteForce, extract solution, verify on source
- Compare with known results from literature
- Verify that satisfiable 3SAT instances yield flow >= R and unsatisfiable instances do not

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source (3SAT):**
Variables: x_1, x_2, x_3
Clauses:
- C_1 = (x_1 ∨ x_2 ∨ x_3)
- C_2 = (¬x_1 ∨ ¬x_2 ∨ x_3)
- C_3 = (x_1 ∨ ¬x_2 ∨ ¬x_3)

**Constructed Target (Integral Flow with Homologous Arcs):**

Vertices: s, u_1, v_1, u_2, v_2, u_3, v_3, t, plus clause nodes c_1, c_2, c_3.

Arcs and structure:
- Variable chain: s->u_1, u_1->v_1 (TRUE arc a_1^T), u_1->v_1 (FALSE arc a_1^F), v_1->u_2, u_2->v_2 (TRUE arc a_2^T), u_2->v_2 (FALSE arc a_2^F), v_2->u_3, u_3->v_3 (TRUE arc a_3^T), u_3->v_3 (FALSE arc a_3^F), v_3->t.
- Clause arcs: For each clause C_j, an arc from s through c_j to t carrying 1 unit.
- All capacities = 1.

Homologous pairs (linking literals to clauses):
- (a_1^T, clause_1_lit1) — x_1 in C_1
- (a_2^T, clause_1_lit2) — x_2 in C_1
- (a_3^T, clause_1_lit3) — x_3 in C_1
- (a_1^F, clause_2_lit1) — ¬x_1 in C_2
- (a_2^F, clause_2_lit2) — ¬x_2 in C_2
- (a_3^T, clause_2_lit3) — x_3 in C_2
- (a_1^T, clause_3_lit1) — x_1 in C_3
- (a_2^F, clause_3_lit2) — ¬x_2 in C_3
- (a_3^F, clause_3_lit3) — ¬x_3 in C_3

Requirement R = 3 + 3 = 6.

**Solution mapping:**
Assignment x_1=TRUE, x_2=FALSE, x_3=TRUE satisfies all clauses.
- Variable path: flow goes through a_1^T, a_2^F, a_3^T (each with flow 1).
- C_1 satisfied by x_1=TRUE: clause_1_lit1 gets flow 1 (homologous with a_1^T).
- C_2 satisfied by ¬x_2 (x_2=FALSE): clause_2_lit2 gets flow 1 (homologous with a_2^F).
- C_3 satisfied by x_1=TRUE: clause_3_lit1 gets flow 1 (homologous with a_1^T).
- Total flow = 3 (variable chain) + 3 (clauses) = 6 = R.


## References

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262-279.
- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691-703.
- **[Itai, 1977]**: [`Itai1977a`] Alon Itai (1977). "Two commodity flow". Dept. of Computer Science, Technion.
