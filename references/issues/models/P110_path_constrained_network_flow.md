---
name: Problem
about: Propose a new problem type
title: "[Model] PathConstrainedNetworkFlow"
labels: model
assignees: ''
---

## Motivation

PATH CONSTRAINED NETWORK FLOW (P110) from Garey & Johnson, A2 ND34. A variant of network flow where flow must be routed along a given collection of specified s-t paths (rather than being decomposed freely). This constraint makes the problem NP-complete even when all arc capacities equal 1. The non-integral version is equivalent to linear programming, but the integrality gap question (whether the best integral flow matches the best rational flow) is itself NP-complete.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in the current rule set.
- **As target:** R55: 3SAT -> PATH CONSTRAINED NETWORK FLOW

## Definition

**Name:** `PathConstrainedNetworkFlow`
<!-- ⚠️ Unverified -->
**Canonical name:** PATH CONSTRAINED NETWORK FLOW (also: Integer Multicommodity Flow on Prescribed Paths, Unsplittable Flow on Paths)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND34

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, a capacity c(a) ∈ Z^+ for each a ∈ A, a collection P of directed paths in G, and a requirement R ∈ Z^+.
QUESTION: Is there a function g: P -> Z_0^+ such that if f: A -> Z_0^+ is the flow function defined by f(a) = Sum_{p in P(a)} g(p), where P(a) is the set of all paths in P containing the arc a, then f is such that
(1) f(a) <= c(a) for all a ∈ A,
(2) for each v ∈ V - {s,t}, flow is conserved at v, and
(3) the net flow into t is at least R?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |P| integer variables (one per path in the collection P), representing the amount of flow sent along each path.
- **Per-variable domain:** {0, 1, ..., M} where M is bounded by the minimum capacity along the path -- the flow routed along path p.
- **Meaning:** The variable assignment encodes the flow decomposition. g(p) is the amount of flow sent along path p. A satisfying assignment has total flow (sum over all paths of g(p)) into t at least R, while respecting all arc capacity constraints.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `PathConstrainedNetworkFlow`
**Variants:** None (directed graph with integer types)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices |V| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs (u, v) in A |
| `source` | `usize` | Index of source vertex s |
| `sink` | `usize` | Index of sink vertex t |
| `capacities` | `Vec<i32>` | Capacity c(a) for each arc |
| `paths` | `Vec<Vec<usize>>` | Collection P of directed s-t paths (each path is a sequence of arc indices) |
| `requirement` | `i32` | Flow requirement R |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Each path in P is a directed path from s to t in G.
- The flow on each arc is the sum of flows on all paths using that arc.
- Flow conservation is automatically satisfied when flow is decomposed into s-t paths.
- The key constraint is integrality of the path flows g(p).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** NP-complete, so no polynomial-time algorithm is known. Brute-force: enumerate all non-negative integer assignments to |P| paths such that arc capacities are satisfied and total flow >= R. This is bounded by O(product of capacity bounds per path).
- **Special cases:** With non-integral flows allowed, equivalent to linear programming (polynomial). With all c(a) = 1, still NP-complete.
- **NP-completeness:** NP-complete by transformation from 3SAT (Promel, 1978; Garey & Johnson, ND34).
- **Related problems:** The integrality gap problem (does the best rational flow exceed the best integral flow?) is also NP-complete.
- **References:**
  - H.J. Promel (1978). Transformation from 3SAT.
  - C. Barnhart, C.A. Hane, P.H. Vance (2000). "Using branch-and-price-and-cut to solve origin-destination integer multicommodity flow problems." *Operations Research*, 48(2):318-326.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, a capacity c(a) ∈ Z^+ for each a ∈ A, a collection P of directed paths in G, and a requirement R ∈ Z^+.
QUESTION: Is there a function g: P -> Z_0^+ such that if f: A -> Z_0^+ is the flow function defined by f(a) = Sum_{p in P(a)} g(p), where P(a) is the set of all paths in P containing the arc a, then f is such that
(1) f(a) <= c(a) for all a ∈ A,
(2) for each v ∈ V - {s,t}, flow is conserved at v, and
(3) the net flow into t is at least R?
Reference: [Promel, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete even if all c(a) = 1. The corresponding problem with non-integral flows is equivalent to LINEAR PROGRAMMING, but the question of whether the best rational flow fails to exceed the best integral flow is NP-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all integer flow assignments on paths and verify capacity and requirement constraints.
- [x] It can be solved by reducing to integer programming -- ILP with variables g(p) for each path, capacity constraints on arcs, and flow requirement.
- [ ] Other: LP relaxation (non-integral) is equivalent to linear programming and solvable in polynomial time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES -- feasible integral flow exists):**
Directed graph with 6 vertices {s=0, 1, 2, 3, 4, t=5}:
- Arcs (with capacities):
  - (0,1) c=1, (0,2) c=1, (1,3) c=1, (2,3) c=1, (2,4) c=1, (3,5) c=1, (4,5) c=1
- Paths in P:
  - p_1: 0 -> 1 -> 3 -> 5
  - p_2: 0 -> 2 -> 3 -> 5
  - p_3: 0 -> 2 -> 4 -> 5
- R = 2

Solution: g(p_1) = 1, g(p_2) = 0, g(p_3) = 1.
- Arc flows: f(0,1)=1, f(0,2)=1, f(1,3)=1, f(2,4)=1, f(3,5)=1, f(4,5)=1, f(2,3)=0.
- All capacities satisfied (all <= 1).
- Net flow into t: 1 + 1 = 2 = R.
- Answer: YES

**Instance 2 (NO -- no feasible integral flow achieving R):**
Same graph, R = 3.
- Paths p_1 and p_2 both use arc (3,5) with capacity 1, so at most one can carry flow.
- Path p_3 uses arc (2,4) and (4,5), independent of arc (3,5).
- But p_2 and p_3 both use arc (0,2) with capacity 1, so at most one of p_2 or p_3 can carry flow.
- Maximum integral flow: g(p_1)=1 + max(g(p_2), g(p_3))=1, total = 2 < 3 = R.
- Answer: NO
