---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to DIRECTED TWO-COMMODITY INTEGRAL FLOW"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability (3SAT)'
canonical_target_name: 'Directed Two-Commodity Integral Flow'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** DIRECTED TWO-COMMODITY INTEGRAL FLOW
**Motivation:** Establishes NP-completeness of DIRECTED TWO-COMMODITY INTEGRAL FLOW via polynomial-time reduction from 3SAT. This is a foundational result showing that even with only two commodities, the integral multicommodity flow problem is intractable, in sharp contrast to the single-commodity case which is polynomial.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND38, p.216

## GJ Source Entry

> [ND38] DIRECTED TWO-COMMODITY INTEGRAL FLOW
> INSTANCE: Directed graph G=(V,A), specified vertices s_1, s_2, t_1, and t_2, capacity c(a)∈Z^+ for each a∈A, requirements R_1,R_2∈Z^+.
> QUESTION: Are there two flow functions f_1,f_2: A→Z_0^+ such that
> (1) for each a∈A, f_1(a)+f_2(a)≤c(a),
> (2) for each v∈V−{s,t} and i∈{1,2}, flow f_i is conserved at v, and
> (3) for i∈{1,2}, the net flow into t_i under flow f_i is at least R_i?
> Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete even if c(a)=1 for all a∈A and R_1=1. Variant in which s_1=s_2, t_1=t_2, and arcs can be restricted to carry only one specified commodity is also NP-complete (follows from [Even, Itai, and Shamir, 1976]). Corresponding M-commodity problem with non-integral flows allowed is polynomially equivalent to LINEAR PROGRAMMING for all M≥2 [Itai, 1977].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a DIRECTED TWO-COMMODITY INTEGRAL FLOW instance as follows (based on Even, Itai, and Shamir, 1976):

1. **Variable lobes:** For each variable x_i, construct a "lobe" gadget consisting of two parallel directed paths from node u_i to node v_i:
   - Upper path (TRUE path): represents x_i = TRUE.
   - Lower path (FALSE path): represents x_i = FALSE.
   All arcs in the lobes have capacity 1.

2. **Variable chain (Commodity 1):** Chain the lobes in series: s_1 -> u_1, v_1 -> u_2, ..., v_n -> t_1. Commodity 1 has requirement R_1 = 1. This single unit of flow traverses each lobe, choosing either the TRUE or FALSE path, thereby encoding a truth assignment.

3. **Clause sinks (Commodity 2):** For each clause C_j, create a dedicated "clause arc" from a clause node to t_2 (or a path segment leading to t_2). Commodity 2 has requirement R_2 = m (one unit per clause).

4. **Literal connections:** For each literal in clause C_j:
   - If x_i appears positively, add an arc from the TRUE path of lobe i to the clause node for C_j.
   - If ¬x_i appears, add an arc from the FALSE path of lobe i to the clause node for C_j.
   These arcs have capacity 1.

5. **Commodity 2 source:** s_2 connects to each clause node (or to the literal connection points) so that commodity 2 can route flow through satisfied literals to validate clauses.

6. **Capacity sharing:** Since both commodities share arc capacities (constraint: f_1(a) + f_2(a) <= c(a) = 1), an arc used by commodity 1 (the assignment) cannot simultaneously be used by commodity 2 (clause validation). This forces commodity 2 to route through the "other" paths consistent with the assignment, verifying clause satisfaction.

The 3SAT formula is satisfiable if and only if both commodities can simultaneously achieve their requirements.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n + m) where n = num_variables, m = num_clauses |
| `num_arcs` | O(n + 3m) (variable lobe arcs + literal connection arcs + clause arcs) |
| `max_capacity` | 1 (unit capacities) |
| `requirement_1` | 1 |
| `requirement_2` | m (num_clauses) |

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source 3SAT instance, solve target directed two-commodity integral flow using BruteForce, extract solution, verify on source
- Compare with known results from literature
- Verify that satisfiable 3SAT instances yield feasible two-commodity flow and unsatisfiable instances do not

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source (3SAT):**
Variables: x_1, x_2, x_3
Clauses:
- C_1 = (x_1 ∨ ¬x_2 ∨ x_3)
- C_2 = (¬x_1 ∨ x_2 ∨ ¬x_3)
- C_3 = (x_1 ∨ x_2 ∨ x_3)

**Constructed Target (Directed Two-Commodity Integral Flow):**

Vertices: s_1, s_2, t_1, t_2, u_1, v_1, u_2, v_2, u_3, v_3, plus intermediate nodes on TRUE/FALSE paths, and clause nodes d_1, d_2, d_3.

Arcs (all capacity 1):
- Commodity 1 chain: s_1->u_1, two paths u_1->...->v_1 (TRUE/FALSE for x_1), v_1->u_2, two paths u_2->...->v_2 (TRUE/FALSE for x_2), v_2->u_3, two paths u_3->...->v_3 (TRUE/FALSE for x_3), v_3->t_1.
- Commodity 2 source arcs: s_2->d_1, s_2->d_2, s_2->d_3.
- Literal connections:
  - C_1: x_1 TRUE path -> d_1, x_2 FALSE path -> d_1, x_3 TRUE path -> d_1.
  - C_2: x_1 FALSE path -> d_2, x_2 TRUE path -> d_2, x_3 FALSE path -> d_2.
  - C_3: x_1 TRUE path -> d_3, x_2 TRUE path -> d_3, x_3 TRUE path -> d_3.
- Clause sink arcs: d_1->t_2, d_2->t_2, d_3->t_2.

Requirements: R_1 = 1, R_2 = 3.

**Solution mapping:**
Assignment x_1=TRUE, x_2=TRUE, x_3=TRUE satisfies all clauses.
- Commodity 1: s_1 -> u_1 -> (TRUE path) -> v_1 -> u_2 -> (TRUE path) -> v_2 -> u_3 -> (TRUE path) -> v_3 -> t_1. Flow = 1.
- Commodity 2:
  - C_1 via x_1 TRUE: s_2 -> d_1 (through x_1 TRUE literal connection, using a different arc not used by commodity 1) -> t_2.
  - C_2 via x_2 TRUE: s_2 -> d_2 (through x_2 TRUE literal connection) -> t_2.
  - C_3 via x_3 TRUE: s_2 -> d_3 (through x_3 TRUE literal connection) -> t_2.
  Flow = 3.
- All capacity constraints satisfied (no arc carries more than 1 total).


## References

- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691-703.
- **[Itai, 1977]**: [`Itai1977a`] Alon Itai (1977). "Two commodity flow". Dept. of Computer Science, Technion.
