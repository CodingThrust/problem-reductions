---
name: Problem
about: Propose a new problem type
title: "[Model] NetworkReliability"
labels: model
assignees: ''
---

## Motivation

NETWORK RELIABILITY (P96) from Garey & Johnson, A2 ND20. An NP-hard (and not known to be in NP) problem of computing the probability that a set of terminal vertices remains connected when edges fail independently with given probabilities. This problem is fundamental to the design and analysis of communication networks, power grids, and distributed systems. It is marked with (*) in GJ, indicating it is not known to be in NP (the reliability probability may require exponential precision to verify).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R41: STEINER TREE IN GRAPHS -> NETWORK RELIABILITY (ND20)

## Definition

**Name:** <!-- ⚠️ Unverified --> `NetworkReliability`
**Canonical name:** Network Reliability (also: K-Terminal Reliability, All-Terminal Reliability)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND20

**Mathematical definition:**

INSTANCE: Graph G = (V,E), subset V' <= V, a rational "failure probability" p(e), 0 <= p(e) <= 1, for each e in E, a positive rational number q <= 1.
QUESTION: Assuming edge failures are independent of one another, is the probability q or greater that each pair of vertices in V' is joined by at least one path containing no failed edge?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |E| binary variables (one per edge)
- **Per-variable domain:** binary {0, 1} -- whether edge e survives (1) or fails (0)
- **Meaning:** variable x_e = 1 if edge e is operational, x_e = 0 if edge e has failed. The reliability is the probability (over all 2^|E| failure patterns, weighted by the product of individual edge probabilities) that all terminals in V' are pairwise connected in the surviving subgraph.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `NetworkReliability`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `terminals` | `Vec<usize>` | The set of terminal vertices V' <= V |
| `failure_probs` | `Vec<f64>` | Failure probability p(e) for each edge e in E |
| `threshold` | `f64` | Reliability threshold q |

**Notes:**
- This problem is NOT known to be in NP. The answer involves computing a probability that may not have a polynomial-size certificate.
- The problem is NP-hard (and the underlying counting problem is #P-complete).
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|), `num_terminals()` (= |V'|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-hard, not known to be in NP. Marked with (*) in GJ.
- **Counting complexity:** The underlying problem of computing the reliability polynomial is #P-complete (Valiant, 1979). Remains NP-hard even for |V'| = 2 (two-terminal reliability).
- **Best known exact algorithm:** Exact computation requires enumerating connected subgraphs or using inclusion-exclusion over min-cuts. Time complexity is exponential: O(2^|E|) in the worst case for general graphs. Binary Decision Diagram (BDD) methods can compute exact reliability for moderately sized graphs. For graphs with bounded treewidth w, exact computation is possible in O(2^w * n) time.
- **Special cases:** Polynomial-time solvable for series-parallel graphs, graphs of bounded treewidth, and some other restricted graph classes.
- **References:**
  - A. Rosenthal (1974). "Computing Reliability of Complex Systems." Ph.D. thesis, University of California, Berkeley.
  - L.G. Valiant (1979). "The Complexity of Enumeration and Reliability Problems." *SIAM Journal on Computing*, 8(3):410-421.
  - M.O. Ball (1986). "Computational Complexity of Network Reliability Analysis: An Overview." *IEEE Transactions on Reliability*, R-35(3):230-239.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), subset V' <= V, a rational "failure probability" p(e), 0 <= p(e) <= 1, for each e in E, a positive rational number q <= 1.
QUESTION: Assuming edge failures are independent of one another, is the probability q or greater that each pair of vertices in V' is joined by at least one path containing no failed edge?

Reference: [Rosenthal, 1974]. Transformation from STEINER TREE IN GRAPHS.
Comment: Not known to be in NP. Remains NP-hard even if |V'| = 2 [Valiant, 1977b]. The related problem in which we want two disjoint paths between each pair of vertices in V' is NP-hard even if V' = V [Ball, 1977b]. If G is directed and we ask for a directed path between each ordered pair of vertices in V', the one-path problem is NP-hard for both |V'| = 2 [Valiant, 1977b] and V' = V [Ball, 1977a]. Many of the underlying subgraph enumeration problems are #P-complete (see [Valiant, 1977b]).

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all 2^|E| edge failure patterns, for each pattern check if terminals V' are connected, sum the probabilities of connected patterns.
- [ ] It can be solved by reducing to integer programming. (Not directly -- this is a probability computation problem, not an optimization problem.)
- [x] Other: BDD-based exact methods for moderate-size graphs; Monte Carlo simulation for estimation; inclusion-exclusion over min-cuts; factoring/decomposition methods for series-parallel graphs.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 edges:**
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {1,4}, {3,4}, {3,5}, {4,5}
- Terminals V' = {0, 5}
- Failure probabilities: all p(e) = 0.1 (each edge fails with 10% probability, survives with 90%)
- Threshold q = 0.95

**Analysis (two-terminal reliability from vertex 0 to vertex 5):**
- There are multiple vertex-disjoint paths from 0 to 5:
  - Path 1: 0-1-3-5
  - Path 2: 0-2-3-5
  - Path 3: 0-1-4-5
  - Path 4: 0-2-3-4-5
- With high individual edge survival probability (0.9), the redundancy from multiple paths makes the reliability high.
- Exact computation requires summing over all 2^8 = 256 edge survival/failure patterns.
- By inclusion-exclusion on the 4 paths or by factoring the graph:
  - P(path 1 works) = 0.9^3 = 0.729
  - P(at least one path works) is close to 1 - (0.1)^k for the minimum edge cut size k
  - Minimum edge cut between 0 and 5 has size 2 (e.g., removing {0,1} and {0,2} disconnects 0 from rest)
  - P(disconnected) ~ sum of P(all edges in some cut fail) which is approximately 0.1^2 = 0.01
  - Reliability ~ 0.99 > q = 0.95
- Answer: YES (reliability exceeds threshold)
