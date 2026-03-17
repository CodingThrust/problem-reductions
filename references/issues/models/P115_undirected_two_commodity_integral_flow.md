---
name: Problem
about: Propose a new problem type
title: "[Model] UndirectedTwoCommodityIntegralFlow"
labels: model
assignees: ''
---

## Motivation

UNDIRECTED TWO-COMMODITY INTEGRAL FLOW (P115) from Garey & Johnson, A2 ND39. An NP-complete problem that extends the directed two-commodity integral flow to undirected graphs, where flow on each edge can go in either direction but each commodity must choose a single direction per edge. Notable for remaining NP-complete with unit capacities, yet becoming polynomial when all capacities are even (a rare parity-dependent complexity dichotomy).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None in current set.
- **As target:** R60: DIRECTED TWO-COMMODITY INTEGRAL FLOW -> UNDIRECTED TWO-COMMODITY INTEGRAL FLOW

## Definition

**Name:** `UndirectedTwoCommodityIntegralFlow`
<!-- ⚠️ Unverified -->
**Canonical name:** UNDIRECTED TWO-COMMODITY INTEGRAL FLOW
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND39

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s_1, s_2, t_1, and t_2, a capacity c(e) ∈ Z^+ for each e ∈ E, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E and i ∈ {1,2}, either f_i((u,v)) = 0 or f_i((v,u)) = 0,
(2) for each {u,v} ∈ E,
    max{f_1((u,v)),f_1((v,u))} + max{f_2((u,v)),f_2((v,u))} ≤ c({u,v}),
(3) for each v ∈ V − {s_1,s_2,t_1,t_2} and i ∈ {1,2}, flow f_i is conserved at v, and
(4) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** 4|E| (for each edge {u,v} and each commodity i in {1,2}: variables f_i((u,v)) and f_i((v,u))).
- **Per-variable domain:** {0, 1, ..., c(e)} for each directed version of edge e. Subject to antisymmetry: for each commodity i, at most one of f_i((u,v)), f_i((v,u)) is nonzero.
- **Meaning:** Each variable represents the integer flow of a specific commodity in a specific direction along an edge. The joint capacity constraint limits the total flow (from both commodities) on each edge. A valid configuration satisfies antisymmetry, joint capacity, conservation for each commodity, and both requirements.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `UndirectedTwoCommodityIntegralFlow`
**Variants:** None (single variant; problem is always on an undirected graph).

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices \|V\| |
| `edges` | `Vec<(usize, usize)>` | Undirected edges {u, v} |
| `capacities` | `Vec<u64>` | Capacity c(e) for each edge |
| `source_1` | `usize` | Source vertex s_1 for commodity 1 |
| `sink_1` | `usize` | Sink vertex t_1 for commodity 1 |
| `source_2` | `usize` | Source vertex s_2 for commodity 2 |
| `sink_2` | `usize` | Sink vertex t_2 for commodity 2 |
| `requirement_1` | `u64` | Flow requirement R_1 for commodity 1 |
| `requirement_2` | `u64` | Flow requirement R_2 for commodity 2 |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- NP-complete even with unit capacities (c(e) = 1 for all e).
- Polynomial when all capacities are even.
- The fractional (non-integral) version is solvable in polynomial time.
- Antisymmetry constraint: each commodity can only send flow in one direction per edge.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The problem is NP-complete (Even, Itai, and Shamir, 1976). Brute-force: for each edge, choose a direction and flow amount for each commodity. With unit capacities, the search space is O(5^|E|) (each edge can carry: nothing, commodity 1 left, commodity 1 right, commodity 2 left, commodity 2 right). No sub-exponential exact algorithm is known.
- **NP-completeness:** Proved by Even, Itai, and Shamir (1976) via reduction from DIRECTED TWO-COMMODITY INTEGRAL FLOW. Remains NP-complete with unit capacities.
- **Polynomial cases:**
  - All capacities even: polynomial (Even, Itai, and Shamir, 1976).
  - Non-integral flows allowed: polynomial (solvable by LP or specialized algorithms).
- **References:**
  - S. Even, A. Itai, A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM J. Comput.* 5, pp. 691-703.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), specified vertices s_1, s_2, t_1, and t_2, a capacity c(e) ∈ Z^+ for each e ∈ E, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E and i ∈ {1,2}, either f_i((u,v)) = 0 or f_i((v,u)) = 0,
(2) for each {u,v} ∈ E,
    max{f_1((u,v)),f_1((v,u))} + max{f_2((u,v)),f_2((v,u))} ≤ c({u,v}),
(3) for each v ∈ V − {s,t} and i ∈ {1,2}, flow f_i is conserved at v, and
(4) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?
Reference: [Even, Itai, and Shamir, 1976]. Transformation from DIRECTED TWO-COMMODITY INTEGRAL FLOW.
Comment: Remains NP-complete even if c(e) = 1 for all e ∈ E. Solvable in polynomial time if c(e) is even for all e ∈ E. Corresponding problem with non-integral flows allowed can be solved in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [x] It can be solved by reducing to integer programming.
- [x] Other: Formulate as an ILP: for each edge {u,v} and commodity i, introduce integer variables f_i_uv, f_i_vu >= 0 with binary direction indicators. Constraints: antisymmetry (at most one direction per commodity per edge), joint capacity, conservation, and requirements. Standard ILP solvers can handle moderate instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES):**
Graph with 6 vertices {0=s_1, 1=s_2, 2, 3, 4=t_1, 5=t_2} and 8 edges (all capacity 1):
- e_0 = {0,2}, e_1 = {0,3}, e_2 = {1,2}, e_3 = {1,3}
- e_4 = {2,4}, e_5 = {2,5}, e_6 = {3,4}, e_7 = {3,5}

Requirements: R_1 = 1, R_2 = 1.

Solution:
- Commodity 1: flow on e_0 (0->2) = 1, flow on e_4 (2->4) = 1. Path: s_1 -> 2 -> t_1.
- Commodity 2: flow on e_3 (1->3) = 1, flow on e_7 (3->5) = 1. Path: s_2 -> 3 -> t_2.
- Joint capacity: e_0: 1+0=1<=1, e_3: 0+1=1<=1, e_4: 1+0=1<=1, e_7: 0+1=1<=1. Others: 0<=1.
- Conservation: vertex 2: commodity 1 in=1, out=1; commodity 2: 0. vertex 3: commodity 1: 0; commodity 2 in=1, out=1.
- Requirements met: R_1=1, R_2=1. Answer: YES.

**Instance 2 (NO):**
Graph with 4 vertices {0=s_1, 1=s_2, 2, 3} where t_1=3, t_2=3, and 3 edges (all capacity 1):
- e_0 = {0,2}, e_1 = {1,2}, e_2 = {2,3}

Requirements: R_1 = 1, R_2 = 1.
- Both commodities need to route 1 unit to vertex 3. Both must use edge e_2={2,3}. Joint capacity on e_2: max flow from commodity 1 + max flow from commodity 2 <= 1. But both need >= 1 unit through this edge. f_1 + f_2 >= 2 > 1 = c(e_2). Answer: NO.

**Instance 3 (YES, even capacities -- polynomial case):**
Same graph as Instance 2 but c(e_2) = 2.
- Commodity 1: e_0 (0->2)=1, e_2 (2->3)=1. Commodity 2: e_1 (1->2)=1, e_2 (2->3)=1.
- Joint capacity on e_2: 1+1=2<=2. Answer: YES (and solvable in polynomial time since c(e_2) is even).
