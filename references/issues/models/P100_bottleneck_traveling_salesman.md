---
name: Problem
about: Propose a new problem type
title: "[Model] BottleneckTravelingSalesman"
labels: model
assignees: ''
---

## Motivation

BOTTLENECK TRAVELING SALESMAN (P100) from Garey & Johnson, A2 ND24. A variant of the classical Traveling Salesman Problem where the objective is to minimize the maximum (bottleneck) edge weight in a Hamiltonian tour rather than minimizing total tour length. NP-complete even when distances are restricted to {1, 2}. It is a target in the reduction from HAMILTONIAN CIRCUIT (R45).

**Associated rules:**
- R45: HAMILTONIAN CIRCUIT -> BOTTLENECK TRAVELING SALESMAN (incoming)

<!-- ⚠️ Unverified: AI-collected rule associations -->

## Definition

**Name:** `BottleneckTravelingSalesman`
<!-- ⚠️ Unverified -->
**Canonical name:** BOTTLENECK TRAVELING SALESMAN
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND24

**Mathematical definition:**

INSTANCE: Set C of m cities, distance d(c_i, c_j) ∈ Z+ for each pair of cities c_i, c_j ∈ C, positive integer B.
QUESTION: Is there a tour of C whose longest edge is no longer than B, i.e., a permutation <c_{π(1)}, c_{π(2)}, ..., c_{π(m)}> of C such that d(c_{π(i)}, c_{π(i+1)}) ≤ B for 1 ≤ i < m and such that d(c_{π(m)}, c_{π(1)}) ≤ B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** m variables (one per city), representing a permutation of the cities.
- **Per-variable domain:** {0, 1, ..., m-1} — the position of each city in the tour.
- **Meaning:** The variable assignment encodes the order in which cities are visited. A satisfying assignment is a permutation π such that d(c_{π(i)}, c_{π(i+1)}) ≤ B for all consecutive pairs and the wrap-around edge d(c_{π(m)}, c_{π(1)}) ≤ B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `BottleneckTravelingSalesman`
**Variants:** weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `num_cities` | `usize` | Number of cities m |
| `distances` | `Vec<Vec<W>>` | Symmetric distance matrix d(c_i, c_j) for all city pairs |
| `bound` | `W` | The bottleneck bound B |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The distance matrix is symmetric and has zeros on the diagonal.
- Alternatively, could be formulated as an optimization problem minimizing the bottleneck (max edge) of a Hamiltonian tour. The decision version asks if a tour with bottleneck ≤ B exists.
- An alternative schema could store the complete graph implicitly and use the same graph-based representation as TravelingSalesman, adding a `bound` field.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O(m^2 * 2^m) via the Held-Karp dynamic programming algorithm (1962), adapted for the bottleneck objective by replacing sum with max. Alternatively, binary search on the bottleneck value B combined with Hamiltonian cycle detection on the subgraph of edges with weight ≤ B, yielding O(log(max_weight) * T_HC(m)) where T_HC is the time for Hamiltonian cycle detection.
- **NP-completeness:** NP-complete by reduction from HAMILTONIAN CIRCUIT (Garey & Johnson ND24). Remains NP-complete even when d(c_i, c_j) ∈ {1, 2} for all pairs.
- **References:**
  - M. Held and R.M. Karp (1962). "A dynamic programming approach to sequencing problems." *Journal of the Society for Industrial and Applied Mathematics*, 10(1):196–210.
  - P.C. Gilmore and R.E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem." *Operations Research* 12, pp. 655–679.

## Extra Remark

**Full book text:**

INSTANCE: Set C of m cities, distance d(ci,cj) ∈ Z+ for each pair of cities ci,cj ∈ C, positive integer B.
QUESTION: Is there a tour of C whose longest edge is no longer than B, i.e., a permutation <cπ(1),cπ(2),...,cπ(m)> of C such that d(cπ(i),cπ(i+1)) ≤ B for 1 ≤ i < m and such that d(cπ(m),cπ(1)) ≤ B?

Reference: Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete even if d(ci,cj) ∈ {1,2} for all ci,cj ∈ C. An important special case that is solvable in polynomial time can be found in [Gilmore and Gomory, 1964].

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all permutations of cities and check if the maximum edge weight is ≤ B.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Held-Karp DP in O(m^2 * 2^m) time; binary search on bottleneck value combined with Hamiltonian cycle detection.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — tour with bottleneck ≤ B exists):**
6 cities {0, 1, 2, 3, 4, 5}, B = 2.
Distance matrix (symmetric):
```
     0  1  2  3  4  5
  0: -  1  3  5  3  1
  1: 1  -  1  3  5  2
  2: 3  1  -  1  3  4
  3: 5  3  1  -  1  3
  4: 3  5  3  1  -  1
  5: 1  2  4  3  1  -
```
Tour: 0 → 1 → 2 → 3 → 4 → 5 → 0
Edge weights: d(0,1)=1, d(1,2)=1, d(2,3)=1, d(3,4)=1, d(4,5)=1, d(5,0)=1
Bottleneck: max = 1 ≤ 2 = B
Answer: YES

**Instance 2 (NO — no tour with bottleneck ≤ B):**
6 cities {0, 1, 2, 3, 4, 5}, B = 1.
Distance matrix (symmetric):
```
     0  1  2  3  4  5
  0: -  1  2  2  2  1
  1: 1  -  1  2  2  2
  2: 2  1  -  1  2  2
  3: 2  2  1  -  1  2
  4: 2  2  2  1  -  1
  5: 1  2  2  2  1  -
```
Edges with distance 1: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {0,5} — these form a single cycle C_6.
The only Hamiltonian tour using only distance-1 edges is this cycle: 0→1→2→3→4→5→0, bottleneck = 1 ≤ B = 1.
Answer: YES

Now change B = 0: No tour can have all edges with weight ≤ 0 since all distances are ≥ 1.
Answer: NO
