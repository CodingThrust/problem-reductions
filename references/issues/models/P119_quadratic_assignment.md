---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticAssignment"
labels: model
assignees: ''
---

## Motivation

QUADRATIC ASSIGNMENT PROBLEM (P119) from Garey & Johnson, A2 ND43. A classical NP-hard combinatorial optimization problem first introduced by Koopmans and Beckmann (1957) for facility location. It models the problem of assigning facilities to locations while minimizing the total interaction cost (product of flows between facilities and distances between their assigned locations). QAP is considered one of the "hardest of the hard" combinatorial optimization problems, with even moderate-size instances (n > 20) being beyond exact solvers.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in current rule set.
- **As target:** R64 (HAMILTONIAN CIRCUIT to QUADRATIC ASSIGNMENT PROBLEM)

## Definition

**Name:** <!-- ⚠️ Unverified --> `QuadraticAssignment`
**Canonical name:** Quadratic Assignment Problem (QAP)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND43

**Mathematical definition:**

INSTANCE: Non-negative integer costs c_{ij}, 1 <= i,j <= n, and distances d_{kl}, 1 <= k,l <= m, bound B in Z^+.
QUESTION: Is there a one-to-one function f: {1,2,...,n} -> {1,2,...,m} such that
sum_{i=1}^{n} sum_{j=1, j!=i}^{n} c_{ij} * d_{f(i)f(j)} <= B ?

The optimization version asks: find the assignment f minimizing the total cost.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n variables (one per facility), each taking a value in {1, 2, ..., m}. Since m >= n and f must be one-to-one, this is equivalent to choosing a permutation (or injection) from n facilities to m locations.
- **Per-variable domain:** {0, 1, ..., m-1} — which location facility i is assigned to. The domain size is m. The assignment must be injective (no two facilities share a location).
- **Meaning:** Variable f(i) = k means facility i is assigned to location k. The total cost is the sum over all facility pairs (i,j) of flow c_{ij} times distance d_{f(i)f(j)}. For the decision version, Metric = bool (is cost <= B?). For the optimization version, Metric = SolutionSize<i64> (minimize total cost).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `QuadraticAssignment`
**Variants:** none (algebraic problem, no graph type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `cost_matrix` | `Vec<Vec<i64>>` | n x n flow/cost matrix C, where c_{ij} = interaction between facilities i and j |
| `distance_matrix` | `Vec<Vec<i64>>` | m x m distance matrix D, where d_{kl} = distance between locations k and l |
| `bound` | `i64` | B — upper bound on total cost (for decision version) |

**Notes:**
- The decision version is a satisfaction problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The optimization version (minimize total assignment cost) uses `Metric = SolutionSize<i64>`, implementing `OptimizationProblem` with `Direction::Minimize`.
- Typically n = m (same number of facilities and locations), giving a bijection.
- Key getter methods: `num_facilities()` (= n), `num_locations()` (= m).
- Special case: if d_{kl} = |k - l| and C is a 0-1 symmetric matrix, this becomes OPTIMAL LINEAR ARRANGEMENT (also NP-complete).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** Strongly NP-hard (Sahni and Gonzalez, 1976; transformation from HAMILTONIAN CIRCUIT).
- **Inapproximability:** For arbitrary epsilon > 0, no polynomial-time epsilon-approximation algorithm exists unless P = NP (Sahni and Gonzalez, 1976).
- **Best known exact algorithm:** Branch-and-bound algorithms. The brute force approach enumerates all n! permutations in O(n! * n^2) time. Advanced branch-and-bound methods (e.g., Anstreicher 2003, using convex quadratic bounds) can solve instances of size up to about n = 30 in practice, but worst case remains factorial.
- **Complexity string:** `"num_facilities!"` — factorial in the number of facilities (brute force over all permutations).
- **Practical limits:** Exact solutions are feasible only for n <= 30 with state-of-the-art branch-and-bound. Instances from the QAPLIB benchmark library with n >= 20 can take hours to days.
- **References:**
  - S. Sahni and T. Gonzalez (1976). "P-complete approximation problems." *Journal of the ACM*, 23(3):555-565. NP-hardness and inapproximability.
  - T.C. Koopmans and M. Beckmann (1957). "Assignment problems and the location of economic activities." *Econometrica*, 25(1):53-76. Original QAP formulation.
  - K.M. Anstreicher (2003). "Recent advances in the solution of quadratic assignment problems." *Mathematical Programming Ser. B*, 97:27-42.

## Extra Remark

**Full book text:**

INSTANCE: Non-negative integer costs c_{ij}, 1 <= i,j <= n, and distances d_{kl}, 1 <= k,l <= m, bound B in Z^+.
QUESTION: Is there a one-to-one function f: {1,2,...,n} -> {1,2,...,m} such that
sum_{i=1}^{n} sum_{j=1, j!=i}^{n} c_{ij} d_{f(i)f(j)} <= B ?
Reference: [Sahni and Gonzalez, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Special case in which each d_{kl} = k - l and all c_{ji} = c_{ij} in {0,1} is the NP-complete OPTIMAL LINEAR ARRANGEMENT problem. The general problem is discussed, for example, in [Garfinkel and Nemhauser, 1972].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all n! permutations f and compute the objective sum_{i,j} c_{ij} * d_{f(i)f(j)} for each; return the one with minimum cost (or check if cost <= B for decision version). Time: O(n! * n^2).
- [x] It can be solved by reducing to integer programming. Linearize the quadratic objective using binary assignment variables x_{ik} (facility i to location k) and auxiliary variables for products x_{ik} * x_{jl}. Standard Koopmans-Beckmann linearization.
- [x] Other: Branch-and-bound with Gilmore-Lawler lower bounds; semidefinite relaxation bounds; metaheuristics (simulated annealing, genetic algorithms, ant colony optimization) for approximate solutions.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance (n = m = 4, decision version):**

Cost matrix C (flows between 4 facilities):
```
     1  2  3  4
1  [ 0  5  2  0 ]
2  [ 5  0  0  3 ]
3  [ 2  0  0  4 ]
4  [ 0  3  4  0 ]
```

Distance matrix D (distances between 4 locations):
```
     1  2  3  4
1  [ 0  1  2  3 ]
2  [ 1  0  1  2 ]
3  [ 2  1  0  1 ]
4  [ 3  2  1  0 ]
```

Bound: B = 30

**Evaluation of assignment f = (1,2,3,4) (identity):**
Cost = c_{12}*d_{12} + c_{13}*d_{13} + c_{14}*d_{14} + c_{21}*d_{21} + c_{23}*d_{23} + c_{24}*d_{24} + c_{31}*d_{31} + c_{32}*d_{32} + c_{34}*d_{34} + c_{41}*d_{41} + c_{42}*d_{42} + c_{43}*d_{43}
= 5*1 + 2*2 + 0*3 + 5*1 + 0*1 + 3*2 + 2*2 + 0*1 + 4*1 + 0*3 + 3*2 + 4*1
= 5 + 4 + 0 + 5 + 0 + 6 + 4 + 0 + 4 + 0 + 6 + 4 = 38

**Evaluation of assignment f = (2,1,4,3) (swap 1<->2, 3<->4):**
Cost = c_{12}*d_{21} + c_{13}*d_{24} + c_{14}*d_{23} + c_{21}*d_{12} + c_{23}*d_{14} + c_{24}*d_{13} + c_{31}*d_{42} + c_{32}*d_{41} + c_{34}*d_{43} + c_{41}*d_{32} + c_{42}*d_{31} + c_{43}*d_{34}
= 5*1 + 2*2 + 0*1 + 5*1 + 0*3 + 3*2 + 2*2 + 0*3 + 4*1 + 0*1 + 3*2 + 4*1
= 5 + 4 + 0 + 5 + 0 + 6 + 4 + 0 + 4 + 0 + 6 + 4 = 38

**Evaluation of assignment f = (1,3,2,4):**
Cost = c_{12}*d_{13} + c_{13}*d_{12} + c_{14}*d_{14} + c_{21}*d_{31} + c_{23}*d_{32} + c_{24}*d_{34} + c_{31}*d_{21} + c_{32}*d_{23} + c_{34}*d_{24} + c_{41}*d_{41} + c_{42}*d_{43} + c_{43}*d_{42}
= 5*2 + 2*1 + 0*3 + 5*2 + 0*1 + 3*1 + 2*1 + 0*1 + 4*2 + 0*3 + 3*1 + 4*2
= 10 + 2 + 0 + 10 + 0 + 3 + 2 + 0 + 8 + 0 + 3 + 8 = 46

Best assignment found: f = (2,1,4,3) or identity, cost = 38 > B = 30. (For B = 40, answer would be YES.)
Optimal assignment for this instance requires checking all 4! = 24 permutations.
