---
name: Rule
about: Propose a new reduction rule
title: "[Rule] STEINER TREE IN GRAPHS to NETWORK RELIABILITY"
labels: rule
assignees: ''
canonical_source_name: 'STEINER TREE IN GRAPHS'
canonical_target_name: 'NETWORK RELIABILITY'
source_in_codebase: false
target_in_codebase: false
---

**Source:** STEINER TREE IN GRAPHS
**Target:** NETWORK RELIABILITY
**Motivation:** Establishes NP-hardness of NETWORK RELIABILITY via polynomial-time reduction from STEINER TREE IN GRAPHS. This is a notable reduction because the target problem is not known to be in NP (computing the exact reliability probability may require exponential precision). The reduction, due to Rosenthal (1974), shows that if we could efficiently determine whether the reliability of a network exceeds a threshold, we could solve the Steiner tree problem. This connects combinatorial optimization (Steiner trees) with probabilistic network analysis.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND20, p.211

## GJ Source Entry

> [ND20] NETWORK RELIABILITY (*)
> INSTANCE: Graph G=(V,E), subset V' <= V, a rational "failure probability" p(e), 0 <= p(e) <= 1, for each e in E, a positive rational number q <= 1.
> QUESTION: Assuming edge failures are independent of one another, is the probability q or greater that each pair of vertices in V' is joined by at least one path containing no failed edge?
> Reference: [Rosenthal, 1974]. Transformation from STEINER TREE IN GRAPHS.
> Comment: Not known to be in NP. Remains NP-hard even if |V'|=2 [Valiant, 1977b]. The related problem in which we want two disjoint paths between each pair of vertices in V' is NP-hard even if V'=V [Ball, 1977b]. If G is directed and we ask for a directed path between each ordered pair of vertices in V', the one-path problem is NP-hard for both |V'|=2 [Valiant, 1977b] and V'=V [Ball, 1977a]. Many of the underlying subgraph enumeration problems are #P-complete (see [Valiant, 1977b]).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a SteinerTreeInGraphs instance (G = (V, E), w, R, B) where G is an undirected graph with edge weights w(e) in Z0+, terminal set R <= V, and weight bound B, construct a NetworkReliability instance (G', V', p, q) as follows:

1. **Graph construction:** Use the same graph G' = G = (V, E). The terminal set is V' = R.

2. **Failure probability assignment:** Assign uniform failure probability p(e) = p for all edges e in E, where p is chosen as a specific rational value in (0, 1) that makes the reduction work. The exact value of p depends on the structure of G and the weight bound B.

   The key idea is: the reliability of the network (probability that all terminals in R are connected) is a polynomial in (1-p), and this polynomial has a specific relationship to the number of connected subgraphs spanning R weighted by their sizes. A Steiner tree of weight <= B exists if and only if the reliability exceeds a carefully chosen threshold q.

3. **Threshold construction:** Set the reliability threshold q to a value that distinguishes between:
   - Graphs where a Steiner tree of weight <= B exists (higher reliability)
   - Graphs where no such tree exists (lower reliability)

   Specifically, when p is very small (close to 0), the dominant term in the reliability polynomial corresponds to the minimum-weight Steiner tree. If this minimum weight is <= B, the reliability is approximately 1 - O(p^{B+1}), which exceeds the threshold q. If the minimum weight exceeds B, the reliability drops below q.

4. **Alternative formulation (Rosenthal's approach):** Rosenthal's original reduction works by observing that the K-terminal reliability R(G, V', p) can be expressed as:

   R(G, V', p) = sum over all edge subsets S that connect V' of: prod_{e in S} (1-p(e)) * prod_{e not in S} p(e)

   The Steiner tree problem asks whether there exists a connected subgraph spanning R with at most B edges (in the unit-weight case). By choosing p appropriately, the threshold on R(G, V', p) encodes whether such a subgraph exists.

5. **Solution extraction:** This reduction is for decision problems only (NP-hardness proof). There is no direct solution extraction; the reduction shows that an oracle for Network Reliability would solve Steiner Tree.

**Note:** This is an NP-hardness reduction (not NP-completeness), because Network Reliability is not known to be in NP. The problem is actually #P-hard for computing the exact reliability (Valiant, 1979).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source SteinerTreeInGraphs instance (|V|)
- m = `num_edges` of source SteinerTreeInGraphs instance (|E|)
- k = `num_terminals` of source instance (|R|)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `num_terminals` | `num_terminals` |

**Derivation:**
- The graph is unchanged: same vertices, same edges, same terminal set.
- The transformation only introduces the failure probabilities p(e) and threshold q.
- This is a parameter-setting reduction, not a structural transformation.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a SteinerTreeInGraphs instance to NetworkReliability, compute the exact reliability by brute-force enumeration of all 2^|E| edge failure patterns, verify that reliability >= q iff a Steiner tree of weight <= B exists
- Test with a graph where the minimum Steiner tree is known (e.g., a tree graph where R includes all leaves) and verify the reliability threshold is correctly calibrated
- Test with unit-weight graphs for simplicity in verification
- Note: exact reliability computation is exponential, so only small instances (|E| <= 20) are practical for brute-force validation

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (SteinerTreeInGraphs):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 edges, all unit weight:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {1,4}, {3,4}, {3,5}, {4,5}
- Terminals R = {0, 5}
- Weight bound B = 3

Minimum Steiner tree connecting vertices 0 and 5:
- Path 0-1-4-5: weight 3 (edges {0,1}, {1,4}, {4,5})
- Path 0-2-3-5: weight 3 (edges {0,2}, {2,3}, {3,5})
- Path 0-1-3-5: weight 3 (edges {0,1}, {1,3}, {3,5})
- Minimum Steiner tree weight = 3 = B, so answer is YES.

**Constructed target instance (NetworkReliability):**

Same graph G with 6 vertices and 8 edges.
- Terminals V' = {0, 5}
- Failure probabilities: p(e) = 0.01 for all edges (chosen small so reduction works)
- Threshold: q = (1 - 0.01)^3 * polynomial correction ~ 0.97 (calibrated so that existence of a weight-3 Steiner tree implies reliability >= q)

**Reliability computation (approximate):**
- There are multiple paths from 0 to 5: three paths of length 3 (listed above), plus longer paths.
- P(at least one path works) is very high when p = 0.01.
- Minimum edge cut between 0 and 5 has size 2 (e.g., {0,1} and {0,2}).
- P(disconnected) <= C(cut_size, failures) ~ p^2 = 0.0001.
- Reliability ~ 1 - 0.0001 = 0.9999 >> q.
- Answer: YES (reliability exceeds threshold).

**If B = 2 (no Steiner tree of weight <= 2 exists since minimum path length is 3):**
- The threshold q would be recalibrated higher so that the reliability for minimum tree weight 3 does NOT exceed the new threshold for B=2.
- Answer: NO.


## References

- **[Rosenthal, 1974]**: [`Rosenthal1974`] A. Rosenthal (1974). "Computing Reliability of Complex Systems". University of California.
- **[Valiant, 1977b]**: [`Valiant1977b`] Leslie G. Valiant (1977). "The complexity of enumeration and reliability problems". Computer Science Dept., University of Edinburgh.
- **[Ball, 1977b]**: [`Ball1977b`] M. O. Ball (1977). "".
- **[Ball, 1977a]**: [`Ball1977a`] M. O. Ball (1977). "Network Reliability and Analysis: Algorithms and Complexity". Cornell University.
