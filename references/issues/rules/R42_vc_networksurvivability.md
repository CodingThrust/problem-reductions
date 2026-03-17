---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to NETWORK SURVIVABILITY"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'NETWORK SURVIVABILITY'
source_in_codebase: true
target_in_codebase: false
---

**Source:** VERTEX COVER
**Target:** NETWORK SURVIVABILITY
**Motivation:** Establishes NP-hardness of NETWORK SURVIVABILITY via polynomial-time reduction from VERTEX COVER. This reduction is notable because the target problem is not known to be in NP — computing the exact failure probability may require summing over exponentially many configurations. The reduction shows that even deciding whether a network's all-edges-fail probability meets a threshold is at least as hard as finding a minimum vertex cover.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND21, p.211

## GJ Source Entry

> [ND21] NETWORK SURVIVABILITY (*)
> INSTANCE: Graph G=(V,E), a rational "failure probability" p(x), 0≤p(x)≤1, for each x∈V∪E, a positive rational number q≤1.
> QUESTION: Assuming all edge and vertex failures are independent of one another, is the probability q or greater that for all {u,v}∈E at least one of u, v, or {u,v} will fail?
> Reference: [Rosenthal, 1974]. Transformation from VERTEX COVER.
> Comment: Not known to be in NP.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G, K) where G = (V, E) with n = |V| vertices, m = |E| edges, and K the cover size bound, construct a NetworkSurvivability instance as follows:

1. **Graph preservation:** Use the same graph G = (V, E).

2. **Edge failure probabilities:** Set p(e) = 0 for all edges e ∈ E (edges never fail on their own — only vertex failures can "cover" edges).

3. **Vertex failure probabilities:** Set p(v) = p for each vertex v ∈ V, where p is chosen so that the probability that exactly K or more vertices fail yields a threshold-crossing event. Specifically, set p(v) = 1/2 for all v ∈ V. Under this assignment, each subset S ⊆ V is equally likely (probability (1/2)^n). The event "all edges are covered" (i.e., for each edge {u,v}, at least one of u or v has failed) occurs exactly when the set of failed vertices forms a vertex cover of G.

4. **Threshold q:** Set q equal to the probability that a uniformly random subset of V is a vertex cover. Since G has a vertex cover of size K, there is at least one such subset. The threshold q is set so that the NetworkSurvivability answer is YES if and only if G has a vertex cover of size ≤ K.

   More precisely: with p(v) = 1/2 for all v and p(e) = 0 for all e, the probability that "for all {u,v} ∈ E at least one of u, v, or {u,v} fails" equals (number of vertex covers of G) / 2^n. The reduction encodes the vertex cover question into this probability threshold.

5. **Correctness:** A vertex cover of size ≤ K in G exists if and only if the set of failed vertices (with each vertex failing independently with probability 1/2) can form a vertex cover. The threshold q is calibrated so that the probability meets q iff a small enough vertex cover exists. Formally, the key insight from Rosenthal (1974) is that computing this reliability probability is at least as hard as deciding vertex cover.

**Key invariant:** With edge failure probabilities set to 0, the event "all edges are covered by failures" reduces to "the set of failed vertices is a vertex cover." The probability of this event under independent Bernoulli vertex failures encodes the combinatorial structure of vertex covers in G.

**Note:** The exact details of the threshold calibration follow Rosenthal (1974). The core idea is that network reliability computation subsumes vertex cover detection as a special case.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source G
- m = `num_edges` of source G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| number of probability parameters | `num_vertices + num_edges` |

**Derivation:** The graph structure is preserved. The overhead is in assigning O(n + m) rational failure probabilities and computing the threshold q. The graph itself is unchanged.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small MinimumVertexCover instance (G, K) to NetworkSurvivability, enumerate all 2^n failure subsets, compute the probability that the failed set forms a vertex cover, and verify it matches the threshold condition.
- Test with a known graph: triangle K_3 has minimum VC size 2. With p(v)=1/2, p(e)=0, the vertex covers are: {0,1}, {0,2}, {1,2}, {0,1,2} — 4 out of 8 subsets — probability = 0.5. Setting q = 0.5 should yield YES; setting q = 0.6 should yield NO (or YES, depending on the exact cover count).
- Verify that a star graph K_{1,n-1} (min VC = 1, the center) yields a high probability of coverage when the center fails.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}
- Minimum vertex cover size: K = 3, e.g., {1, 2, 3}

**Constructed target instance (NetworkSurvivability):**
- Same graph G with 6 vertices and 7 edges
- Vertex failure probabilities: p(v) = 0.5 for all v ∈ {0,1,2,3,4,5}
- Edge failure probabilities: p(e) = 0 for all 7 edges
- Threshold: q is set to the fraction of subsets of V that form vertex covers

**Verification:**
With p(e) = 0, an edge {u,v} is "covered by failure" iff at least one of u, v has failed. This is exactly the vertex cover condition. The probability of the compound event equals (number of vertex covers) / 2^6 = (number of vertex covers) / 64.

The vertex covers of this graph include: {1,2,3}, {0,1,2,3}, {1,2,3,4}, {1,2,3,5}, {0,1,2,3,4}, {0,1,2,3,5}, {1,2,3,4,5}, {0,1,2,3,4,5}, and several others. Each subset S where every edge has an endpoint in S is a cover. The count and resulting probability encode the vertex cover structure of G.


## References

- **[Rosenthal, 1974]**: [`Rosenthal1974`] A. Rosenthal (1974). "Computing Reliability of Complex Systems". University of California.
