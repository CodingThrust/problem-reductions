---
name: Problem
about: Propose a new problem type
title: "[Model] UndirectedFlowWithLowerBounds"
labels: model
assignees: ''
---

## Motivation

UNDIRECTED FLOW WITH LOWER BOUNDS (P113) from Garey & Johnson, A2 ND37. A fundamental NP-complete problem notable for being strongly NP-complete even when non-integral flows are allowed. This stands in sharp contrast to directed flow with lower bounds, which is polynomial-time solvable. The problem demonstrates that the combination of undirected edges and lower bounds creates inherent computational hardness.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None in current set.
- **As target:** R58: SATISFIABILITY -> UNDIRECTED FLOW WITH LOWER BOUNDS

## Definition

**Name:** `UndirectedFlowLowerBounds`
<!-- ⚠️ Unverified -->
**Canonical name:** UNDIRECTED FLOW WITH LOWER BOUNDS
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND37

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s and t, capacity c(e) ∈ Z^+ and lower bound l(e) ∈ Z_0^+ for each e ∈ E, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E, either f((u,v)) = 0 or f((v,u)) = 0,
(2) for each e = {u,v} ∈ E, l(e) ≤ max{f((u,v)),f((v,u))} ≤ c(e),
(3) for each v ∈ V − {s,t}, flow is conserved at v, and
(4) the net flow into t is at least R?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** 2|E| (two variables per undirected edge: one for each direction (u,v) and (v,u)).
- **Per-variable domain:** {0, 1, ..., c(e)} for each direction of edge e.
- **Meaning:** For each undirected edge {u,v}, exactly one of f((u,v)) and f((v,u)) is nonzero (antisymmetry constraint). The nonzero value must lie between the lower bound l(e) and the capacity c(e). A valid configuration satisfies flow conservation at all non-terminal vertices and achieves net flow into t of at least R.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `UndirectedFlowLowerBounds`
**Variants:** None (single variant; problem is always on an undirected graph).

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices \|V\| |
| `edges` | `Vec<(usize, usize)>` | Undirected edges {u, v} |
| `capacities` | `Vec<u64>` | Upper bound c(e) for each edge |
| `lower_bounds` | `Vec<u64>` | Lower bound l(e) for each edge |
| `source` | `usize` | Source vertex s |
| `sink` | `usize` | Sink vertex t |
| `requirement` | `u64` | Flow requirement R |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Flow on undirected edges is antisymmetric: for each edge, flow goes in only one direction.
- The lower bound constraint means every edge must carry at least l(e) units of flow (if l(e) > 0).
- Strongly NP-complete even with non-integral flows allowed.
- The directed version (with lower and upper bounds) is polynomial-time solvable.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The problem is strongly NP-complete (Itai, 1977). Brute-force: enumerate over all possible flow directions and values for each edge. With max capacity C and |E| edges, the search space is O((2C)^|E|). No sub-exponential exact algorithm is known.
- **NP-completeness:** Proved by Itai (1977) via reduction from SATISFIABILITY. Strongly NP-complete even with non-integral flows.
- **Contrast with directed case:** For directed graphs, flow with lower and upper bounds can be solved in polynomial time using standard max-flow techniques (Ford and Fulkerson, 1962).
- **References:**
  - A. Itai (1977/1978). "Two commodity flow". *Journal of the ACM* 25(4), pp. 596-611. (Also as Technion technical report, 1977.)
  - L. R. Ford and D. R. Fulkerson (1962). "Flows in Networks". Princeton University Press.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), specified vertices s and t, capacity c(e) ∈ Z^+ and lower bound l(e) ∈ Z_0^+ for each e ∈ E, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E, either f((u,v)) = 0 or f((v,u)) = 0,
(2) for each e = {u,v} ∈ E, l(e) ≤ max{f((u,v)),f((v,u))} ≤ c(e),
(3) for each v ∈ V − {s,t}, flow is conserved at v, and
(4) the net flow into t is at least R?
Reference: [Itai, 1977]. Transformation from SATISFIABILITY.
Comment: Problem is NP-complete in the strong sense, even if non-integral flows are allowed. Corresponding problem for directed graphs can be solved in polynomial time, even if we ask that the total flow be R or less rather than R or more [Ford and Fulkerson, 1962] (see also [Lawler, 1976a]). The analogous DIRECTED M-COMMODITY FLOW WITH LOWER BOUNDS problem is polynomially equivalent to LINEAR PROGRAMMING for all M ≥ 2 if non-integral flows are allowed [Itai, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [x] It can be solved by reducing to integer programming.
- [x] Other: Formulate as an ILP: for each edge {u,v}, introduce variables f_uv, f_vu >= 0 with f_uv * f_vu = 0 (or use binary direction indicators), enforce lower and upper bounds, flow conservation, and requirement R. The bilinear constraint f_uv * f_vu = 0 can be linearized using big-M or binary variables.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES):**
Graph with 6 vertices {0=s, 1, 2, 3, 4, 5=t} and 7 edges:
- e_0 = {0,1}: l=1, c=2
- e_1 = {0,2}: l=1, c=2
- e_2 = {1,3}: l=0, c=2
- e_3 = {2,3}: l=0, c=2
- e_4 = {1,4}: l=1, c=1
- e_5 = {3,5}: l=0, c=3
- e_6 = {4,5}: l=1, c=2

Requirement R = 3.

Solution:
- e_0: flow s->1, f=2. e_1: flow s->2, f=1. e_2: flow 1->3, f=1. e_3: flow 2->3, f=1. e_4: flow 1->4, f=1. e_5: flow 3->t, f=2. e_6: flow 4->t, f=1.
- Lower bounds: e_0: 2>=1, e_1: 1>=1, e_4: 1>=1, e_6: 1>=1. All satisfied.
- Conservation: vertex 1: in=2, out=1+1=2. vertex 2: in=1, out=1. vertex 3: in=1+1=2, out=2. vertex 4: in=1, out=1.
- Net flow into t: 2+1 = 3 >= 3. Answer: YES.

**Instance 2 (NO):**
Graph with 4 vertices {0=s, 1, 2, 3=t} and 4 edges:
- e_0 = {0,1}: l=1, c=1
- e_1 = {0,2}: l=1, c=1
- e_2 = {1,3}: l=1, c=1
- e_3 = {2,3}: l=1, c=1

Requirement R = 2.
All lower bounds are 1, so all edges must carry flow 1. But flow conservation at vertex 1: in=1 (from e_0), out must be 1 (to e_2). Similarly vertex 2: in=1, out=1. Net flow into t = 2. This seems feasible... but the lower bound forces flow on every edge, and all directions must be consistent. Actually this IS feasible with flow s->1->t and s->2->t. Answer: YES.

Change to: e_0 = {0,1}: l=2, c=2, e_1 = {0,2}: l=2, c=2, e_2 = {1,3}: l=1, c=1, e_3 = {2,3}: l=1, c=1, R=2.
- e_0 forces flow 2 into vertex 1, but e_2 can only carry 1 out. Conservation violated. Answer: NO.
