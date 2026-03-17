---
name: Problem
about: Propose a new problem type
title: "[Model] IntegralFlowWithBundles"
labels: model
assignees: ''
---

## Motivation

INTEGRAL FLOW WITH BUNDLES (P112) from Garey & Johnson, A2 ND36. A classical NP-complete problem that generalizes standard network flow by partitioning arcs into "bundles" with shared capacity constraints. While standard network flow (with per-arc capacities) is polynomial, the bundle capacity variant is NP-complete even when all bundle capacities are 1 and all bundles have exactly two arcs.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None in current set.
- **As target:** R57: INDEPENDENT SET -> INTEGRAL FLOW WITH BUNDLES

## Definition

**Name:** `IntegralFlowBundles`
<!-- ⚠️ Unverified -->
**Canonical name:** INTEGRAL FLOW WITH BUNDLES
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND36

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, "bundles" I_1,I_2,...,I_k ⊆ A such that ∪_{1 ≤ j ≤ k} I_j = A, bundle capacities c_1,c_2,...,c_k ∈ Z^+, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) for 1 ≤ j ≤ k, Σ_{a ∈ I_j} f(a) ≤ c_j,
(2) for each v ∈ V − {s,t}, flow is conserved at v, and
(3) the net flow into t is at least R?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |A| (one variable per arc in the directed graph).
- **Per-variable domain:** {0, 1, ..., C_max} where C_max = max_j c_j. In the unit-capacity case, domain is {0, 1}.
- **Meaning:** Each variable f(a) represents the integer flow on arc a. A valid configuration satisfies all bundle capacity constraints (sum of flows within each bundle does not exceed its capacity), flow conservation at non-terminal vertices, and achieves net flow into t of at least R.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `IntegralFlowBundles`
**Variants:** None (single variant; problem is always on a directed graph).

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices \|V\| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs (u, v) in the graph |
| `source` | `usize` | Source vertex s |
| `sink` | `usize` | Sink vertex t |
| `bundles` | `Vec<Vec<usize>>` | Bundles I_j, each a list of arc indices |
| `bundle_capacities` | `Vec<u64>` | Capacity c_j for each bundle |
| `requirement` | `u64` | Flow requirement R |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Bundles partition the arc set (every arc belongs to at least one bundle; bundles may overlap).
- NP-complete even with all capacities = 1 and all bundles of size 2.
- The non-integral variant can be solved by linear programming.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The problem is NP-complete (Sahni, 1974). Brute-force enumeration over all integer flow assignments takes O(prod_{a in A} (C_max+1)) time. With unit capacities, O(2^|A|). No sub-exponential exact algorithm is known.
- **NP-completeness:** Proved by Sahni (1974) via reduction from INDEPENDENT SET.
- **Special cases:** With unit capacities and bundles of size 2, the problem is equivalent to finding an independent set in a conflict graph defined by the bundles.
- **References:**
  - S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262-279.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, "bundles" I_1,I_2,...,I_k ⊆ A such that ∪_{1 ≤ j ≤ k} I_j = A, bundle capacities c_1,c_2,...,c_k ∈ Z^+, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) for 1 ≤ j ≤ k, Σ_{a ∈ I_j} f(a) ≤ c_j,
(2) for each v ∈ V − {s,t}, flow is conserved at v, and
(3) the net flow into t is at least R?
Reference: [Sahni, 1974]. Transformation from INDEPENDENT SET.
Comment: Remains NP-complete if all capacities are 1 and all bundles have two arcs. Corresponding problem with non-integral flows allowed can be solved by linear programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [x] It can be solved by reducing to integer programming.
- [x] Other: Formulate as an ILP: integer variables f(a) >= 0 for each arc, constraints sum_{a in I_j} f(a) <= c_j for each bundle, flow conservation at each non-terminal vertex, and objective/constraint that net flow into t >= R.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance (YES):**
Directed graph with 4 vertices {0=s, 1, 2, 3=t} and 6 arcs:
- a_0 = (0,1) -- s to 1
- a_1 = (0,2) -- s to 2
- a_2 = (1,3) -- 1 to t
- a_3 = (2,3) -- 2 to t
- a_4 = (1,2) -- 1 to 2
- a_5 = (2,1) -- 2 to 1

Bundles (all capacity 1):
- I_1 = {a_0, a_1} (capacity 1) -- at most 1 unit leaves s via these arcs combined
- I_2 = {a_2, a_5} (capacity 1) -- bundle linking arc 1->t and 2->1
- I_3 = {a_3, a_4} (capacity 1) -- bundle linking arc 2->t and 1->2

Requirement R = 1.

Solution: f(a_0) = 1, f(a_2) = 1, all others = 0.
- Bundle I_1: f(a_0) + f(a_1) = 1 + 0 = 1 <= 1.
- Bundle I_2: f(a_2) + f(a_5) = 1 + 0 = 1 <= 1.
- Bundle I_3: f(a_3) + f(a_4) = 0 + 0 = 0 <= 1.
- Conservation: vertex 1: in = 1 (a_0), out = 1 (a_2) + 0 (a_4) = 1. vertex 2: in = 0, out = 0.
- Net flow into t: f(a_2) + f(a_3) = 1 >= R = 1. Answer: YES.

**Instance (NO):**
Same graph and bundles but R = 2.
- Bundle I_1 limits total outflow from s to 1, so max flow into the network is 1. Cannot achieve R = 2.
- Answer: NO.
