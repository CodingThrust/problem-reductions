---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedTwoCommodityIntegralFlow"
labels: model
assignees: ''
---

## Motivation

DIRECTED TWO-COMMODITY INTEGRAL FLOW (P114) from Garey & Johnson, A2 ND38. A fundamental NP-complete problem in multicommodity flow theory. While single-commodity max-flow is polynomial and fractional multicommodity flow reduces to linear programming, requiring integral flows with just two commodities makes the problem NP-complete. This is a cornerstone result in the theory of network flows.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** R60: DIRECTED TWO-COMMODITY INTEGRAL FLOW -> UNDIRECTED TWO-COMMODITY INTEGRAL FLOW
- **As target:** R59: 3SAT -> DIRECTED TWO-COMMODITY INTEGRAL FLOW

## Definition

**Name:** `DirectedTwoCommodityIntegralFlow`
<!-- ⚠️ Unverified -->
**Canonical name:** DIRECTED TWO-COMMODITY INTEGRAL FLOW
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND38

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s_1, s_2, t_1, and t_2, capacity c(a) ∈ Z^+ for each a ∈ A, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: A → Z_0^+ such that
(1) for each a ∈ A, f_1(a)+f_2(a) ≤ c(a),
(2) for each v ∈ V − {s_1,s_2,t_1,t_2} and i ∈ {1,2}, flow f_i is conserved at v, and
(3) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** 2|A| (two variables per arc: one for each commodity's flow on that arc).
- **Per-variable domain:** {0, 1, ..., c(a)} for each commodity on arc a. In the unit-capacity case, each commodity's flow on an arc is in {0, 1}, and their sum is at most 1.
- **Meaning:** f_i(a) represents the integer flow of commodity i on arc a. A valid configuration satisfies: (a) joint capacity constraints f_1(a)+f_2(a) <= c(a) on every arc, (b) separate flow conservation for each commodity at non-terminal vertices, and (c) each commodity achieves at least its required flow value.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `DirectedTwoCommodityIntegralFlow`
**Variants:** None (single variant).

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices \|V\| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs (u, v) in the graph |
| `capacities` | `Vec<u64>` | Capacity c(a) for each arc |
| `source_1` | `usize` | Source vertex s_1 for commodity 1 |
| `sink_1` | `usize` | Sink vertex t_1 for commodity 1 |
| `source_2` | `usize` | Source vertex s_2 for commodity 2 |
| `sink_2` | `usize` | Sink vertex t_2 for commodity 2 |
| `requirement_1` | `u64` | Flow requirement R_1 for commodity 1 |
| `requirement_2` | `u64` | Flow requirement R_2 for commodity 2 |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- NP-complete even with unit capacities (c(a) = 1) and R_1 = 1.
- The variant with shared source/sink (s_1=s_2, t_1=t_2) and arc-commodity restrictions is also NP-complete.
- The fractional (non-integral) M-commodity version is polynomially equivalent to LINEAR PROGRAMMING for M >= 2.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The problem is NP-complete (Even, Itai, and Shamir, 1976). Brute-force: enumerate all pairs of integer flow assignments for both commodities. With unit capacities and |A| arcs, the search space is O(3^|A|) (each arc carries flow from commodity 1, commodity 2, or neither). No sub-exponential exact algorithm is known.
- **NP-completeness:** Proved by Even, Itai, and Shamir (1976) via reduction from 3SAT. Remains NP-complete even with unit capacities and R_1 = 1.
- **Special cases:** The single-commodity case (standard max-flow) is solvable in polynomial time. The fractional two-commodity case is also polynomial (equivalent to LP).
- **Related results:** Two-commodity flow (fractional) is polynomially equivalent to linear programming (Itai, 1978).
- **References:**
  - S. Even, A. Itai, A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM J. Comput.* 5, pp. 691-703.
  - A. Itai (1978). "Two commodity flow". *Journal of the ACM* 25(4), pp. 596-611.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), specified vertices s_1, s_2, t_1, and t_2, capacity c(a) ∈ Z^+ for each a ∈ A, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: A → Z_0^+ such that
(1) for each a ∈ A, f_1(a)+f_2(a) ≤ c(a),
(2) for each v ∈ V − {s,t} and i ∈ {1,2}, flow f_i is conserved at v, and
(3) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?
Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if c(a) = 1 for all a ∈ A and R_1 = 1. Variant in which s_1 = s_2, t_1 = t_2, and arcs can be restricted to carry only one specified commodity is also NP-complete (follows from [Even, Itai, and Shamir, 1976]). Corresponding M-commodity problem with non-integral flows allowed is polynomially equivalent to LINEAR PROGRAMMING for all M ≥ 2 [Itai, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [x] It can be solved by reducing to integer programming.
- [x] Other: Formulate as an ILP with variables f_1(a), f_2(a) >= 0 integer for each arc a, constraints f_1(a)+f_2(a) <= c(a), flow conservation for each commodity at non-terminal vertices, and requirement constraints. Standard ILP solvers handle moderate-sized instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES):**
Directed graph with 6 vertices {0=s_1, 1=s_2, 2, 3, 4=t_1, 5=t_2} and 8 arcs (all capacity 1):
- a_0 = (0,2), a_1 = (0,3), a_2 = (1,2), a_3 = (1,3)
- a_4 = (2,4), a_5 = (2,5), a_6 = (3,4), a_7 = (3,5)

Requirements: R_1 = 1, R_2 = 1.

Solution:
- Commodity 1: f_1(a_0)=1, f_1(a_4)=1 (path s_1 -> 2 -> t_1). All other f_1 = 0.
- Commodity 2: f_2(a_3)=1, f_2(a_7)=1 (path s_2 -> 3 -> t_2). All other f_2 = 0.
- Joint capacity: a_0: 1+0=1<=1, a_3: 0+1=1<=1, a_4: 1+0=1<=1, a_7: 0+1=1<=1. All others: 0<=1.
- Conservation: vertex 2: commodity 1 in=1(a_0), out=1(a_4); commodity 2 in=0, out=0. vertex 3: commodity 1 in=0, out=0; commodity 2 in=1(a_3), out=1(a_7).
- Net flow: commodity 1 into t_1 = 1 >= 1; commodity 2 into t_2 = 1 >= 1. Answer: YES.

**Instance 2 (NO):**
Directed graph with 4 vertices {0=s_1, 1=s_2, 2, 3} where t_1=3, t_2=3, and 3 arcs (all capacity 1):
- a_0 = (0,2), a_1 = (1,2), a_2 = (2,3)

Requirements: R_1 = 1, R_2 = 1.
- Both commodities must route 1 unit through vertex 2 to vertex 3. Arc a_2 has capacity 1, so f_1(a_2)+f_2(a_2) <= 1. But both commodities need at least 1 unit into vertex 3, requiring f_1(a_2) + f_2(a_2) >= 2. Contradiction. Answer: NO.
