---
name: Problem
about: Propose a new problem type
title: "[Model] IntegralFlowWithMultipliers"
labels: model
assignees: ''
---

## Motivation

INTEGRAL FLOW WITH MULTIPLIERS (P109) from Garey & Johnson, A2 ND33. A generalization of the standard network flow problem where each intermediate vertex v has a multiplier h(v) that scales the incoming flow before comparing it with the outgoing flow. This small change makes the problem NP-complete (the standard case with all h(v) = 1 is polynomial). Arises in lossy network models where flow is gained or lost at intermediate nodes (e.g., water networks with leakage, currency exchange networks).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in the current rule set.
- **As target:** R54: PARTITION -> INTEGRAL FLOW WITH MULTIPLIERS

## Definition

**Name:** `IntegralFlowWithMultipliers`
<!-- ⚠️ Unverified -->
**Canonical name:** INTEGRAL FLOW WITH MULTIPLIERS (also: Network Flow with Gains, Generalized Flow, Lossy/Gainy Flow)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND33

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, multiplier h(v) ∈ Z^+ for each v ∈ V - {s,t}, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A -> Z_0^+ such that
(1) f(a) <= c(a) for all a ∈ A,
(2) for each v ∈ V - {s,t}, Sum_{(u,v) in A} h(v)*f((u,v)) = Sum_{(v,u) in A} f((v,u)), and
(3) the net flow into t is at least R?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |A| integer variables (one per arc), representing the flow on each arc.
- **Per-variable domain:** {0, 1, ..., c(a)} -- the flow on arc a is a non-negative integer bounded by its capacity.
- **Meaning:** The variable assignment encodes the flow function. A satisfying assignment is a flow f such that capacity constraints, generalized conservation constraints (with multipliers), and the requirement R on net flow into t are all met.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `IntegralFlowWithMultipliers`
**Variants:** None (the directed graph and integer types are fixed)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices |V| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs (u, v) in A |
| `source` | `usize` | Index of source vertex s |
| `sink` | `usize` | Index of sink vertex t |
| `multipliers` | `Vec<i32>` | Multiplier h(v) for each non-terminal vertex |
| `capacities` | `Vec<i32>` | Capacity c(a) for each arc |
| `requirement` | `i32` | Flow requirement R |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Multipliers h(v) only apply to intermediate vertices (not s or t).
- The generalized conservation constraint scales incoming flow by h(v): h(v) * (total in-flow) = total out-flow.
- When h(v) = 1 for all v, this is standard max-flow (polynomial).
- The non-integral version can be solved by linear programming.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The problem is NP-complete in general. Brute-force enumeration of integer flow assignments: O(product of (c(a)+1) for all arcs). No known exact exponential-time improvement over exhaustive search for the general integral case.
- **Special cases:** When all h(v) = 1, solvable in polynomial time via standard max-flow algorithms (e.g., Edmonds-Karp O(|V|*|E|^2), push-relabel O(|V|^2*|E|)). The rational (non-integral) version is solvable via linear programming regardless of multiplier values.
- **NP-completeness:** NP-complete by transformation from PARTITION (Sahni, 1974).
- **References:**
  - S. Sahni (1974). "Computationally related problems." *SIAM Journal on Computing*, 3:262-279.
  - W.S. Jewell (1962). "Optimal flow through networks with gains." *Operations Research*, 10(4):476-499.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, multiplier h(v) ∈ Z^+ for each v ∈ V - {s,t}, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A -> Z_0^+ such that
(1) f(a) <= c(a) for all a ∈ A,
(2) for each v ∈ V - {s,t}, Sum_{(u,v) in A} h(v)*f((u,v)) = Sum_{(v,u) in A} f((v,u)), and
(3) the net flow into t is at least R?
Reference: [Sahni, 1974]. Transformation from PARTITION.
Comment: Can be solved in polynomial time by standard network flow techniques if h(v) = 1 for all v ∈ V - {s,t}. Corresponding problem with non-integral flows allowed can be solved by linear programming.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all integer flow assignments on arcs and verify constraints.
- [x] It can be solved by reducing to integer programming -- ILP with capacity, conservation (with multipliers), and flow requirement constraints.
- [ ] Other: When all h(v) = 1, standard max-flow algorithms apply (Edmonds-Karp, push-relabel).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES -- feasible integral flow exists):**
Directed graph with 8 vertices {s, v_1, v_2, v_3, v_4, v_5, v_6, t}:
- Arcs from s: (s, v_1) c=1, (s, v_2) c=1, (s, v_3) c=1, (s, v_4) c=1, (s, v_5) c=1, (s, v_6) c=1
- Arcs to t: (v_1, t) c=2, (v_2, t) c=3, (v_3, t) c=4, (v_4, t) c=5, (v_5, t) c=6, (v_6, t) c=4
- Multipliers: h(v_1)=2, h(v_2)=3, h(v_3)=4, h(v_4)=5, h(v_5)=6, h(v_6)=4
- R = 12

This encodes PARTITION of {2, 3, 4, 5, 6, 4} (sum = 24, target = 12).

Flow: f(s,v_1)=1, f(v_1,t)=2; f(s,v_3)=1, f(v_3,t)=4; f(s,v_5)=1, f(v_5,t)=6.
All other flows = 0.
- Conservation: h(v_1)*1 = 2 = f(v_1,t). h(v_3)*1 = 4 = f(v_3,t). h(v_5)*1 = 6 = f(v_5,t).
- Net flow into t: 2 + 4 + 6 = 12 = R.
- Answer: YES

**Instance 2 (NO -- no feasible flow):**
Same graph structure but A = {1, 2, 3, 7, 8, 5} with sum = 26 (odd), R = 13.
- h(v_1)=1, h(v_2)=2, h(v_3)=3, h(v_4)=7, h(v_5)=8, h(v_6)=5
- Each f(s,v_i) in {0,1}, so net flow = sum of selected h(v_i).
- No subset of {1,2,3,7,8,5} sums to 13 (subsets: {8,5}=13 works!).
- Actually this is YES. Change to A = {1, 2, 4, 8}, sum=15 (odd), R=7: no subset sums to 7 ({1,2,4}=7 works!).
- For a true NO: A = {1, 2, 6, 9}, sum=18, R=9. Subsets: {9}=9. YES again.
- True NO: A = {1, 1, 3, 6}, sum=11 (odd), R=5. Subsets summing to 5: {1,1,3}=5. YES.
- True NO: A = {1, 5, 7, 11}, sum=24, R=12. {1,11}=12. YES.
- For a definitive NO instance: take odd total sum like A = {1, 2, 4}, sum=7, R=3. Subsets: {1,2}=3. YES.
- A = {3, 5, 7, 11}, sum=26, R=13. {3,5,7}=15 no. {7,11}=18 no. {5,11}=16 no. {3,11}=14 no. {3,7}=10 no. {5,7}=12 no. {3,5}=8 no. None sum to 13.
- Answer: NO
