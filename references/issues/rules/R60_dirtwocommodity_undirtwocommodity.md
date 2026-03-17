---
name: Rule
about: Propose a new reduction rule
title: "[Rule] DIRECTED TWO-COMMODITY INTEGRAL FLOW to UNDIRECTED TWO-COMMODITY INTEGRAL FLOW"
labels: rule
assignees: ''
canonical_source_name: 'Directed Two-Commodity Integral Flow'
canonical_target_name: 'Undirected Two-Commodity Integral Flow'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** DIRECTED TWO-COMMODITY INTEGRAL FLOW
**Target:** UNDIRECTED TWO-COMMODITY INTEGRAL FLOW
**Motivation:** Establishes NP-completeness of UNDIRECTED TWO-COMMODITY INTEGRAL FLOW via polynomial-time reduction from the directed variant. This is the standard directed-to-undirected network transformation applied to two-commodity flow, preserving integrality constraints.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND39, p.217

## GJ Source Entry

> [ND39] UNDIRECTED TWO-COMMODITY INTEGRAL FLOW
> INSTANCE: Graph G=(V,E), specified vertices s_1, s_2, t_1, and t_2, a capacity c(e)∈Z^+ for each e∈E, requirements R_1,R_2∈Z^+.
> QUESTION: Are there two flow functions f_1,f_2: {(u,v),(v,u): {u,v}∈E}→Z_0^+ such that
> (1) for all {u,v}∈E and i∈{1,2}, either f_i((u,v))=0 or f_i((v,u))=0,
> (2) for each {u,v}∈E,
>  max{f_1((u,v)),f_1((v,u))}+max{f_2((u,v)),f_2((v,u))}≤c({u,v}),
> (3) for each v∈V−{s,t} and i∈{1,2}, flow f_i is conserved at v, and
> (4) for i∈{1,2}, the net flow into t_i under flow f_i is at least R_i?
> Reference: [Even, Itai, and Shamir, 1976]. Transformation from DIRECTED TWO-COMMODITY INTEGRAL FLOW.
> Comment: Remains NP-complete even if c(e)=1 for all e∈E. Solvable in polynomial time if c(e) is even for all e∈E. Corresponding problem with non-integral flows allowed can be solved in polynomial time.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a DIRECTED TWO-COMMODITY INTEGRAL FLOW instance on directed graph G=(V,A) with sources s_1, s_2, sinks t_1, t_2, arc capacities c(a), and requirements R_1, R_2, construct an UNDIRECTED TWO-COMMODITY INTEGRAL FLOW instance as follows:

1. **Node splitting:** For each vertex v in V, create two nodes v_in and v_out connected by an undirected edge {v_in, v_out} with capacity equal to the sum of capacities of all arcs incident to v (or a sufficiently large value to not be a bottleneck).

2. **Arc replacement:** Replace each directed arc (u, v) in A with an undirected edge {u_out, v_in} with the same capacity c((u,v)). The direction is enforced by the node-splitting structure: flow entering v_in must exit through v_out.

3. **Alternatively (simpler standard construction):** For each directed arc (u, v) with capacity c, introduce an intermediate node w_{uv} and two undirected edges:
   - {u, w_{uv}} with capacity c
   - {w_{uv}, v} with capacity c
   The two-hop structure through the intermediate node ensures that flow effectively travels in the intended direction (u to v), because routing flow "backwards" (from v through w_{uv} to u) would waste capacity and not contribute to the objective.

4. **Terminal vertices:** Keep s_1, s_2, t_1, t_2 as terminal vertices in the undirected graph.

5. **Requirements:** Keep R_1 and R_2 unchanged.

A feasible directed two-commodity integral flow exists if and only if a feasible undirected two-commodity integral flow exists in the constructed graph with the same requirements.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | \|V\| + \|A\| (original vertices plus one intermediate node per arc) |
| `num_edges` | 2\|A\| (two undirected edges per directed arc) |
| `max_capacity` | max(c(a)) (unchanged) |
| `requirement_1` | R_1 (unchanged) |
| `requirement_2` | R_2 (unchanged) |

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source directed two-commodity flow instance, solve target undirected two-commodity flow using BruteForce, extract solution, verify on source
- Compare with known results from literature
- Verify that feasible directed instances yield feasible undirected instances and vice versa

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source (Directed Two-Commodity Integral Flow):**
Directed graph with 6 vertices {s_1, s_2, a, b, t_1, t_2} and 7 arcs (all capacity 1):
- (s_1, a), (s_1, b), (a, t_1), (b, t_1)
- (s_2, a), (s_2, b), (a, t_2), (b, t_2)

Wait -- let's use a smaller but non-trivial example.

Directed graph with vertices {s_1, s_2, u, v, t_1, t_2} and arcs:
- (s_1, u) cap 1, (u, v) cap 1, (v, t_1) cap 1  -- path for commodity 1
- (s_2, u) cap 1, (u, t_2) cap 1                  -- path for commodity 2
- (s_2, v) cap 1, (v, t_2) cap 1                  -- alternative path for commodity 2
- (s_1, v) cap 1                                    -- alternative for commodity 1

Requirements: R_1 = 1, R_2 = 1.

**Constructed Target (Undirected Two-Commodity Integral Flow):**

For each directed arc (x, y), introduce intermediate node w_{xy} and edges {x, w_{xy}}, {w_{xy}, y} each with same capacity.

Vertices: s_1, s_2, u, v, t_1, t_2, w_{s1u}, w_{uv}, w_{vt1}, w_{s2u}, w_{ut2}, w_{s2v}, w_{vt2}, w_{s1v} (14 vertices).

Edges (each pair from an original arc, capacity 1):
- {s_1, w_{s1u}}, {w_{s1u}, u} -- from (s_1, u)
- {u, w_{uv}}, {w_{uv}, v} -- from (u, v)
- {v, w_{vt1}}, {w_{vt1}, t_1} -- from (v, t_1)
- {s_2, w_{s2u}}, {w_{s2u}, u} -- from (s_2, u)
- {u, w_{ut2}}, {w_{ut2}, t_2} -- from (u, t_2)
- {s_2, w_{s2v}}, {w_{s2v}, v} -- from (s_2, v)
- {v, w_{vt2}}, {w_{vt2}, t_2} -- from (v, t_2)
- {s_1, w_{s1v}}, {w_{s1v}, v} -- from (s_1, v)

Requirements: R_1 = 1, R_2 = 1 (unchanged).

**Solution mapping:**
- Commodity 1 path: s_1 -> w_{s1u} -> u -> w_{uv} -> v -> w_{vt1} -> t_1 (flow 1).
- Commodity 2 path: s_2 -> w_{s2v} -> v -> w_{vt2} -> t_2 (flow 1).
- No capacity conflicts (shared vertex v is fine since different edges are used).
- Both requirements met.


## References

- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691-703.
