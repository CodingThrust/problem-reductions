---
name: Problem
about: Propose a new problem type
title: "[Model] NetworkSurvivability"
labels: model
assignees: ''
---

## Motivation

NETWORK SURVIVABILITY (P97) from Garey & Johnson, A2 ND21. A classical problem in network reliability analysis that asks whether the probability that all edges fail (i.e., at least one endpoint or the edge itself fails for every edge) meets a given threshold. It is used as a target in the reduction from VERTEX COVER (R42). Notably, this problem is not known to be in NP since computing the exact failure probability may require exponential-precision arithmetic.

**Associated rules:**
- R42: VERTEX COVER -> NETWORK SURVIVABILITY (incoming)

<!-- ⚠️ Unverified: AI-collected rule associations -->

## Definition

**Name:** `NetworkSurvivability`
<!-- ⚠️ Unverified -->
**Canonical name:** NETWORK SURVIVABILITY
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND21

**Mathematical definition:**

INSTANCE: Graph G = (V,E), a rational "failure probability" p(x), 0 ≤ p(x) ≤ 1, for each x ∈ V∪E, a positive rational number q ≤ 1.
QUESTION: Assuming all edge and vertex failures are independent of one another, is the probability q or greater that for all {u,v} ∈ E at least one of u, v, or {u,v} will fail?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |V| + |E| binary variables (one per element of V∪E), each representing whether that vertex or edge has failed.
- **Per-variable domain:** {0, 1} where 1 = "element has failed" and 0 = "element is operational."
- **Meaning:** A configuration assigns a failure/operational status to each vertex and edge. The question asks whether the probability (over independent Bernoulli failures with parameters p(x)) that every edge {u,v} has at least one of u, v, or {u,v} in the "failed" state meets or exceeds the threshold q.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `NetworkSurvivability`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V,E) |
| `vertex_failure_prob` | `Vec<f64>` | Failure probability p(v) for each vertex v ∈ V |
| `edge_failure_prob` | `Vec<f64>` | Failure probability p(e) for each edge e ∈ E |
| `threshold` | `f64` | The probability threshold q |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The problem asks whether the compound event "every edge has at least one failed endpoint or is itself failed" has probability ≥ q.
- Not known to be in NP: verifying a YES answer requires summing over exponentially many failure configurations.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Exact computation of network reliability is #P-complete (Valiant, 1979). Brute-force enumeration over all 2^(|V|+|E|) failure configurations takes O(2^(|V|+|E|) * |E|) time. For graphs with bounded treewidth w, linear-time FPT algorithms exist with complexity O(2^(O(w)) * (|V|+|E|)).
- **NP-hardness:** The problem is NP-hard (Rosenthal, 1974; proved by reduction from VERTEX COVER). It is not known to be in NP since computing the probability exactly may require summing over exponentially many configurations.
- **References:**
  - A. Rosenthal (1974). "Computing Reliability of Complex Systems." University of California.
  - L.G. Valiant (1979). "The Complexity of Enumeration and Reliability Problems." *SIAM Journal on Computing*, 8(3):410–421.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), a rational "failure probability" p(x), 0 ≤ p(x) ≤ 1, for each x ∈ V∪E, a positive rational number q ≤ 1.
QUESTION: Assuming all edge and vertex failures are independent of one another, is the probability q or greater that for all {u,v} ∈ E at least one of u, v, or {u,v} will fail?

Reference: [Rosenthal, 1974]. Transformation from VERTEX COVER.
Comment: Not known to be in NP.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Enumerate all 2^(|V|+|E|) failure configurations, compute the probability of each configuration (product of independent Bernoulli probabilities), sum over configurations where every edge has at least one failed endpoint or is itself failed, and compare with q. This is exact but exponential.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — probability meets threshold):**
Graph G with 4 vertices {0, 1, 2, 3} and 4 edges:
- Edges: e0={0,1}, e1={1,2}, e2={2,3}, e3={0,3} (cycle C_4)
- Vertex failure probabilities: p(0) = 0.5, p(1) = 0.5, p(2) = 0.5, p(3) = 0.5
- Edge failure probabilities: p(e0) = 0.5, p(e1) = 0.5, p(e2) = 0.5, p(e3) = 0.5
- Threshold: q = 0.01
- Each element fails independently with probability 0.5. The event "all edges covered by a failure" requires that for each of the 4 edges, at least one of its endpoints or the edge itself fails. With 8 independent coin flips at p=0.5, the probability that all 4 edges are "covered" is relatively high (since each edge has 3 chances to be covered, each with p=0.5, giving 1 - 0.5^3 = 0.875 per edge, and these events overlap significantly).
- Answer: YES (the probability exceeds 0.01)

**Instance 2 (NO — probability below threshold):**
Graph G with 6 vertices {0,1,2,3,4,5} and 7 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}
- All failure probabilities: p(x) = 0.01 for all x ∈ V∪E
- Threshold: q = 0.5
- With very low failure probabilities, it is extremely unlikely that every edge has a failure among its endpoints or itself. The probability is far below 0.5.
- Answer: NO
