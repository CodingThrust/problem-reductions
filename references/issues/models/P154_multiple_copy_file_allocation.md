---
name: Problem
about: Propose a new problem type
title: "[Model] MultipleCopyFileAllocation"
labels: model
assignees: ''
---

## Motivation

MULTIPLE COPY FILE ALLOCATION (P154) from Garey & Johnson, A4 SR6. An NP-complete problem (in the strong sense) from the Storage and Retrieval category. It models the problem of deciding where to place copies of a file in a computer network to minimize the combined storage and access costs. Each node in the network has a usage frequency and a storage cost; the access cost for a node depends on its shortest-path distance to the nearest file copy. This problem is fundamental to distributed systems, content delivery networks, and facility location theory. NP-complete in the strong sense, even with uniform usage and storage costs. Proved by Van Sickle and Chandy (1977) via reduction from VERTEX COVER.

**Associated rules:**
- R100: Vertex Cover → Multiple Copy File Allocation (as target)

## Definition

**Name:** `MultipleCopyFileAllocation`
**Canonical name:** MULTIPLE COPY FILE ALLOCATION
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR6, p.227

**Mathematical definition:**

INSTANCE: Graph G = (V, E), for each v in V a usage u(v) in Z+ and a storage cost s(v) in Z+, and a positive integer K.
QUESTION: Is there a subset V' of V such that, if for each v in V we let d(v) denote the number of edges in the shortest path in G from v to a member of V', we have
sum_{v in V'} s(v) + sum_{v in V} d(v) * u(v) <= K ?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| binary variables, one per vertex.
- **Per-variable domain:** {0, 1} -- 0 means no file copy at vertex v, 1 means a file copy is placed at vertex v.
- **Meaning:** x_v = 1 if v in V' (vertex stores a file copy), x_v = 0 otherwise. For each vertex v, d(v) is the shortest-path distance in G to the nearest vertex with x_w = 1. The objective is to find an assignment such that the total cost (storage + access) is at most K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `MultipleCopyFileAllocation`
**Variants:** graph topology (graph type parameter G)

| Field     | Type          | Description                                                     |
|-----------|---------------|-----------------------------------------------------------------|
| `graph`   | `SimpleGraph` | Network topology G = (V, E)                                     |
| `usage`   | `Vec<u64>`    | Usage frequency u(v) for each vertex v in V                     |
| `storage` | `Vec<u64>`    | Storage cost s(v) for placing a file copy at vertex v            |
| `bound`   | `u64`         | Maximum total cost K (storage + weighted access distance)        |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The shortest-path distance d(v) is computed via BFS on the unweighted graph G.
- If v in V' (v stores a copy), then d(v) = 0.
- The problem is related to the Uncapacitated Facility Location Problem (UFLP) but uses graph shortest-path distances rather than arbitrary metric distances.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Brute-force: enumerate all 2^n subsets V' of V, for each compute BFS from V' to determine all d(v), then sum costs. Time O(2^n * (n + m)). No sub-exponential exact algorithm is known for general graphs.
- **NP-completeness:** NP-complete in the strong sense [Van Sickle and Chandy, 1977], even if all vertices have the same usage u and the same storage cost s.
- **Approximation:** Related to the metric Uncapacitated Facility Location Problem, which has a 1.488-approximation algorithm [Li, 2013]. However, the graph-distance variant has its own structure.
- **References:**
  - L. van Sickle and K. M. Chandy (1977). "The complexity of computer network design problems." Tech report, University of Texas at Austin.
  - S. Li (2013). "A 1.488 approximation algorithm for the uncapacitated facility location problem." *Information and Computation* 222:45-58.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), for each v in V a usage u(v) in Z+ and a storage cost s(v) in Z+, and a positive integer K.
QUESTION: Is there a subset V' of V such that, if for each v in V we let d(v) denote the number of edges in the shortest path in G from v to a member of V', we have
sum_{v in V'} s(v) + sum_{v in V} d(v)*u(v) <= K ?
Reference: [Van Sickle and Chandy, 1977]. Transformation from VERTEX COVER.
Comment: NP-complete in the strong sense, even if all v in V have the same value of u(v) and the same value of s(v).

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all 2^n subsets V' of V, compute BFS distances from each vertex to V', compute total cost, check if any V' achieves cost <= K.
- [x] It can be solved by reducing to integer programming — binary variables x_v for placement, auxiliary variables for distances with big-M constraints. Minimize sum s(v)*x_v + sum u(v)*d_v subject to distance constraints.
- [x] Other: Greedy heuristic: iteratively add the vertex to V' that gives the largest cost reduction, until no improvement is possible.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, vertex cover placement is optimal):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 6 edges (cycle C_6):
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}
- Usage: u(v) = 10 for all v
- Storage: s(v) = 1 for all v
- Bound K = 33.

Placement V' = {1, 3, 5}:
- Storage cost: 3 * 1 = 3
- Distances: d(0) = 1 (adjacent to 1 and 5), d(1) = 0, d(2) = 1 (adjacent to 1 and 3), d(3) = 0, d(4) = 1 (adjacent to 3 and 5), d(5) = 0.
- Access cost: (1+0+1+0+1+0) * 10 = 30.
- Total cost: 3 + 30 = 33 <= K = 33. ✓
- Answer: YES

**Greedy trap:** Placing copies at {0, 2, 4} also gives total = 3 + 30 = 33 (symmetric). But placing only 2 copies, say V' = {0, 3}: d(0) = 0, d(1) = 1, d(2) = 1, d(3) = 0, d(4) = 1, d(5) = 1. Total = 2 + 40 = 42 > 33. So fewer copies increase access cost.

**Instance 2 (YES, non-uniform costs):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {0,5}, {0,3}
- Usage: u = [5, 20, 3, 15, 8, 2]
- Storage: s = [10, 5, 10, 5, 10, 10]
- Bound K = 43.

Placement V' = {1, 3}:
- Storage cost: s(1) + s(3) = 5 + 5 = 10
- Distances:
  - d(0) = 1 (adjacent to 1)
  - d(1) = 0
  - d(2) = 1 (adjacent to 1 and 3)
  - d(3) = 0
  - d(4) = 1 (adjacent to 3)
  - d(5) = 1 (path 5->0->3 = 2 edges, but also 5->0->1 = 2 edges; shortest via edge {0,3}: 5->0->3 = 2. Wait, d(5) = min shortest path to V'. V' = {1,3}. 5->0->1 = 2, 5->0->3 = 2 (since {0,3} is an edge, d(5,3) = 2 via 5->0->3). Actually d(5) = 2.
  - Correction: d(5) to nearest of {1,3}: 5-0-1 (length 2) or 5-0-3 (length 2, using edge {0,3}). So d(5) = 2.
- Access cost: 1*5 + 0*20 + 1*3 + 0*15 + 1*8 + 2*2 = 5+0+3+0+8+4 = 20.
- Total cost: 10 + 20 = 30 <= K = 43. ✓
- Answer: YES

**Instance 3 (NO, high cost forced):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} — path P_6:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}
- Usage: u(v) = 100 for all v
- Storage: s(v) = 1 for all v
- Bound K = 5.

Any V' with |V'| copies costs at least |V'| in storage. To keep access cost at most 5 - |V'|, we'd need sum d(v)*100 <= 5 - |V'|, which requires sum d(v) = 0, meaning V' = V (all 6 vertices). But then storage cost = 6 > 5 = K. Answer: NO.
