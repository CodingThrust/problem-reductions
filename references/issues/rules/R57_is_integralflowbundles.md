---
name: Rule
about: Propose a new reduction rule
title: "[Rule] INDEPENDENT SET to INTEGRAL FLOW WITH BUNDLES"
labels: rule
assignees: ''
canonical_source_name: 'Maximum Independent Set'
canonical_target_name: 'Integral Flow with Bundles'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** INDEPENDENT SET
**Target:** INTEGRAL FLOW WITH BUNDLES
**Motivation:** Establishes NP-completeness of INTEGRAL FLOW WITH BUNDLES via polynomial-time reduction from INDEPENDENT SET. The reduction encodes adjacency constraints as shared bundle capacities, showing that even simple bundle structures make integer network flow intractable.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND36, p.216

## GJ Source Entry

> [ND36] INTEGRAL FLOW WITH BUNDLES
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, "bundles" I_1,I_2,···,I_k⊆A such that ⋃_{1≤j≤k} I_j=A, bundle capacities c_1,c_2,···,c_k∈Z^+, requirement R∈Z^+.
> QUESTION: Is there a flow function f: A→Z_0^+ such that
> (1) for 1≤j≤k, Σ_{a∈I_j} f(a)≤c_j,
> (2) for each v∈V−{s,t}, flow is conserved at v, and
> (3) the net flow into t is at least R?
> Reference: [Sahni, 1974]. Transformation from INDEPENDENT SET.
> Comment: Remains NP-complete if all capacities are 1 and all bundles have two arcs. Corresponding problem with non-integral flows allowed can be solved by linear programming.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given an INDEPENDENT SET instance (graph G'=(V',E'), target size K), construct an INTEGRAL FLOW WITH BUNDLES instance as follows:

1. **Vertex arcs:** For each vertex v_i in V', create a directed arc a_i from an intermediate node to t (or equivalently, from s through v_i's gadget to t). Specifically, create a path s -> w_i -> t using arc a_i = (w_i, t) for each vertex v_i. Each arc a_i can carry flow 0 or 1.

2. **Edge bundles:** For each edge {v_i, v_j} in E', create a bundle I_{ij} = {a_i, a_j} with bundle capacity c_{ij} = 1. This constraint ensures that at most one of a_i and a_j carries flow 1 — i.e., at most one endpoint of each edge is "selected."

3. **Vertex bundles (optional):** Each arc a_i also belongs to a singleton bundle {a_i} with capacity 1, ensuring flow on each arc is at most 1.

4. **Requirement:** Set R = K.

The graph G' has an independent set of size K if and only if there exists an integral flow of value at least K in the constructed network satisfying all bundle capacity constraints. Selecting flow 1 on arc a_i corresponds to including vertex v_i in the independent set; the bundle constraint for each edge ensures no two adjacent vertices are both selected.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | n + 2 where n = \|V'\| (s, t, and one intermediate node per vertex) |
| `num_arcs` | 2n (one arc s->w_i and one arc w_i->t per vertex) |
| `num_bundles` | \|E'\| + n (one bundle per edge plus one singleton per vertex) |
| `max_bundle_size` | 2 (edge bundles have exactly 2 arcs) |
| `max_capacity` | 1 (all capacities are 1) |
| `requirement` | K |

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source INDEPENDENT SET instance, solve target integral flow with bundles using BruteForce, extract solution, verify on source
- Compare with known results from literature
- Verify that graphs with independent set of size >= K yield feasible flow >= K and those without do not

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source (Independent Set):**
Graph G' with 6 vertices {v_1, v_2, v_3, v_4, v_5, v_6} and 7 edges:
- Edges: {v_1,v_2}, {v_2,v_3}, {v_3,v_4}, {v_4,v_5}, {v_5,v_6}, {v_6,v_1}, {v_1,v_4}
Target: K = 2

**Constructed Target (Integral Flow with Bundles):**

Vertices: s, w_1, w_2, w_3, w_4, w_5, w_6, t (8 vertices total).

Arcs: For each i in {1..6}:
- a_i^in = (s, w_i) and a_i^out = (w_i, t)

Bundles (edge bundles, capacity 1 each):
- I_{12} = {a_1^out, a_2^out} — edge {v_1, v_2}
- I_{23} = {a_2^out, a_3^out} — edge {v_2, v_3}
- I_{34} = {a_3^out, a_4^out} — edge {v_3, v_4}
- I_{45} = {a_4^out, a_5^out} — edge {v_4, v_5}
- I_{56} = {a_5^out, a_6^out} — edge {v_5, v_6}
- I_{61} = {a_6^out, a_1^out} — edge {v_6, v_1}
- I_{14} = {a_1^out, a_4^out} — edge {v_1, v_4}

Singleton bundles (capacity 1 each): {a_i^out} for i = 1..6.

Requirement R = 2.

**Solution mapping:**
Independent set {v_2, v_5} of size 2:
- Flow: f(a_2^in) = f(a_2^out) = 1, f(a_5^in) = f(a_5^out) = 1, all others 0.
- Bundle checks: I_{12}: f(a_1^out)+f(a_2^out) = 0+1 = 1 <= 1. I_{23}: 1+0 = 1 <= 1. I_{34}: 0+0 = 0 <= 1. I_{45}: 0+1 = 1 <= 1. I_{56}: 1+0 = 1 <= 1. I_{61}: 0+0 = 0 <= 1. I_{14}: 0+0 = 0 <= 1.
- Total flow into t = 2 = R.


## References

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262-279.
