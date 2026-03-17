---
name: Problem
about: Propose a new problem type
title: "[Model] ExpectedRetrievalCost"
labels: model
assignees: ''
---

## Motivation

EXPECTED RETRIEVAL COST (P152) from Garey & Johnson, A4 SR4. An NP-complete problem (in the strong sense) from the Storage and Retrieval category. It models the optimization of record placement on drum-like storage devices (rotating storage media with fixed-position read heads), where the goal is to distribute records across m sectors to minimize the expected rotational latency. The problem captures the trade-off between access probability and physical sector distance on a circular layout. NP-complete even for fixed m >= 2 (though solvable in pseudo-polynomial time per fixed m). The main reduction is from PARTITION / 3-PARTITION.

**Associated rules:**
- R098: Partition / 3-Partition → Expected Retrieval Cost (as target)

## Definition

**Name:** `ExpectedRetrievalCost`
**Canonical name:** EXPECTED RETRIEVAL COST
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR4, p.227

**Mathematical definition:**

INSTANCE: Set R of records, rational probability p(r) in [0,1] for each r in R, with sum_{r in R} p(r) = 1, number m of sectors, and a positive integer K.
QUESTION: Is there a partition of R into disjoint subsets R_1, R_2, ..., R_m such that, if p(R_i) = sum_{r in R_i} p(r) and the "latency cost" d(i,j) is defined to be j - i - 1 if 1 <= i < j <= m and to be m - i + j - 1 if 1 <= j <= i <= m, then the sum over all ordered pairs i,j, 1 <= i,j <= m, of p(R_i) * p(R_j) * d(i,j) is at most K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |R| variables. Each variable x_r in {1, 2, ..., m} assigns record r to a sector.
- **Per-variable domain:** {1, 2, ..., m} — the sector index to which each record is assigned.
- **Meaning:** The variable assignment encodes a partition of records into m sectors. Record r is placed in sector x_r. The objective is to find an assignment such that the weighted latency cost sum_{i,j} p(R_i)*p(R_j)*d(i,j) <= K, where the latency d(i,j) measures the circular distance from sector i to sector j on the drum.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `ExpectedRetrievalCost`
**Variants:** none (no type parameters)

| Field           | Type         | Description                                                      |
|-----------------|--------------|------------------------------------------------------------------|
| `probabilities` | `Vec<f64>`   | Access probability p(r) for each record r in R; must sum to 1.0  |
| `num_sectors`   | `usize`      | Number of sectors m on the drum-like device                       |
| `bound`         | `f64`        | Maximum allowable expected retrieval cost K                       |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The latency function d(i,j) is implicit and computed from the circular sector layout: d(i,j) = (j - i - 1) mod m for the "forward" direction, specifically d(i,j) = j - i - 1 if i < j, and d(i,j) = m - i + j - 1 if j <= i. Note d(i,i) = m - 1 (full rotation when head is already at sector i and next request is also sector i).
- The probabilities are rational numbers in [0,1], but for implementation purposes we use f64.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** For fixed m, the problem is solvable in pseudo-polynomial time via dynamic programming. For general m, it is strongly NP-complete. Brute-force: enumerate all m^n assignments of records to sectors, compute cost for each; time O(m^n * m^2). No sub-exponential exact algorithm is known for general m.
- **NP-completeness:** NP-complete in the strong sense [Cody and Coffman, 1976], via reduction from 3-PARTITION.
- **Approximation:** Cody and Coffman showed that highest-access-frequency-first assignment heuristics provide near-optimal solutions with provable worst-case bounds.
- **References:**
  - R. A. Cody and E. G. Coffman, Jr. (1976). "Record allocation for minimizing expected retrieval costs on drum-like storage devices." *Journal of the ACM* 23(1):103-115.

## Extra Remark

**Full book text:**

INSTANCE: Set R of records, rational probability p(r) in [0,1] for each r in R, with sum_{r in R} p(r) = 1, number m of sectors, and a positive integer K.
QUESTION: Is there a partition of R into disjoint subsets R_1, R_2, ..., R_m such that, if p(R_i) = sum_{r in R_i} p(r) and the "latency cost" d(i,j) is defined to be j - i - 1 if 1 <= i < j <= m and to be m - i + j - 1 if 1 <= j <= i <= m, then the sum over all ordered pairs i,j, 1 <= i,j <= m, of p(R_i) * p(R_j) * d(i,j) is at most K?
Reference: [Cody and Coffman, 1976]. Transformation from PARTITION, 3-PARTITION.
Comment: NP-complete in the strong sense. NP-complete and solvable in pseudo-polynomial time for each fixed m >= 2.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all m^n assignments of n records to m sectors, compute the latency cost for each assignment, check if any achieves cost <= K.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: For fixed m, dynamic programming in pseudo-polynomial time. Greedy heuristic: assign records to sectors in decreasing order of probability, placing each record in the sector that minimizes incremental cost.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, balanced allocation possible):**
Records R = {r_1, r_2, r_3, r_4, r_5, r_6} with m = 3 sectors.
Probabilities: p(r_1) = 0.2, p(r_2) = 0.15, p(r_3) = 0.15, p(r_4) = 0.2, p(r_5) = 0.1, p(r_6) = 0.2.
Sum = 1.0 ✓.

Latency matrix for m = 3:
- d(1,1) = 2, d(1,2) = 0, d(1,3) = 1
- d(2,1) = 1, d(2,2) = 2, d(2,3) = 0
- d(3,1) = 0, d(3,2) = 1, d(3,3) = 2

Allocation: R_1 = {r_1, r_5} (p = 0.3), R_2 = {r_2, r_4} (p = 0.35), R_3 = {r_3, r_6} (p = 0.35).

Cost = sum_{i,j} p(R_i)*p(R_j)*d(i,j):
= p1*p1*d(1,1) + p1*p2*d(1,2) + p1*p3*d(1,3) + p2*p1*d(2,1) + p2*p2*d(2,2) + p2*p3*d(2,3) + p3*p1*d(3,1) + p3*p2*d(3,2) + p3*p3*d(3,3)
= 0.3*0.3*2 + 0.3*0.35*0 + 0.3*0.35*1 + 0.35*0.3*1 + 0.35*0.35*2 + 0.35*0.35*0 + 0.35*0.3*0 + 0.35*0.35*1 + 0.35*0.35*2
= 0.18 + 0 + 0.105 + 0.105 + 0.245 + 0 + 0 + 0.1225 + 0.245
= 1.0025

Set K = 1.01. Cost = 1.0025 <= 1.01. Answer: YES ✓

**Instance 2 (NO, impossible to achieve low cost):**
Records R = {r_1, r_2, r_3, r_4, r_5, r_6} with m = 3 sectors.
Probabilities: p(r_1) = 0.5, p(r_2) = 0.1, p(r_3) = 0.1, p(r_4) = 0.1, p(r_5) = 0.1, p(r_6) = 0.1.
Sum = 1.0 ✓.

The highly skewed distribution (r_1 has probability 0.5) makes it impossible to balance sectors evenly. The best allocation puts r_1 alone in a sector with p(R_1) = 0.5, and distributes the rest with p(R_2) = p(R_3) = 0.25. The minimum cost is significantly higher than a balanced allocation.

Set K = 0.5 (very tight bound). The minimum achievable cost exceeds K. Answer: NO.
