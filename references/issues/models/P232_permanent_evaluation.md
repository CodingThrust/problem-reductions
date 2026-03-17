---
name: Problem
about: Propose a new problem type
title: "[Model] PermanentEvaluation"
labels: model
assignees: ''
---

## Motivation

PERMANENT EVALUATION (P232) from Garey & Johnson, A7 AN13. An NP-hard problem (not known to be in NP) asking whether the permanent of a 0-1 matrix equals a given integer K. Valiant (1979) proved that computing the permanent of a 0-1 matrix is #P-complete -- one of the most celebrated results in computational complexity theory. The permanent is defined as perm(M) = Sigma_{sigma in S_n} Pi_{i=1}^{n} M[i, sigma(i)], summing over all permutations. Unlike the determinant (computable in O(n^3)), the permanent has no known polynomial-time algorithm. The best known exact algorithm is Ryser's formula, running in O(2^n * n) time.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R176: 3SAT -> PERMANENT EVALUATION (establishes NP-hardness via Valiant's reduction using variable/clause/XOR gadgets in cycle-cover graphs)

## Definition

**Name:** `PermanentEvaluation`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN13

**Mathematical definition:**

INSTANCE: An n x n matrix M of 0's and 1's, and a positive integer K <= n!.
QUESTION: Is the value of the permanent of M equal to K?

Where the permanent is defined as:
perm(M) = Sigma_{sigma in S_n} Pi_{i=1}^{n} M[i, sigma(i)]

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** This is a decision problem about a matrix property; there is no natural configuration space to enumerate. The "computation" is evaluating a sum over n! permutations.
- **Per-variable domain:** N/A (no optimization/search variables).
- **Meaning:** The problem asks whether the permanent (a specific integer-valued function of the matrix) equals a given target K. Equivalently, it counts the number of perfect matchings in the bipartite graph with biadjacency matrix M.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `PermanentEvaluation`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<u8>>` | n x n matrix of 0's and 1's (row-major, M[i][j] in {0, 1}) |
| `target` | `u64` | Positive integer K; we ask if perm(M) = K |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Ryser's formula computes the permanent in O(2^n * n) arithmetic operations using inclusion-exclusion and Gray code optimization (Ryser, 1963). This is the fastest known algorithm; Knuth posed the open problem of whether an arithmetic circuit with fewer than 2^n operations exists. The naive approach (summing over all n! permutations) takes O(n! * n) time. For approximation, Jerrum, Sinclair, and Vigoda (2004) gave an FPRAS (fully polynomial randomized approximation scheme) for the permanent of nonnegative matrices, running in O(n^{11} * (log n)^4) time. However, exact computation remains exponential. The decision version (is perm(M) = K?) inherits the #P-hardness of the computation problem: it is NP-hard but not known to be in NP (since verifying the permanent requires computing it).

## Extra Remark

**Full book text:**

INSTANCE: An n x n matrix M of 0's and 1's, and a positive integer K <= n!.
QUESTION: Is the value of the permanent of M equal to K?

Reference: [Valiant, 1977a]. Transformation from 3SAT.
Comment: The problem is NP-hard but not known to be in NP, as is the case for the variants in which we ask whether the value of the permanent is "K or less" or "K or more." The problem of computing the value of the permanent of M is #P-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! permutations of {1, ..., n}; for each permutation sigma, compute the product Pi M[i, sigma(i)]; sum all products to get perm(M); compare with K.)
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Ryser's formula in O(2^n * n) time. Glynn's formula provides an alternative O(2^n * n) algorithm. For nonneg matrices, FPRAS via Markov chain Monte Carlo (Jerrum-Sinclair-Vigoda 2004). The permanent can also be expressed as a hafnian of a related matrix.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Matrix M (3 x 3):
```
1 1 0
0 1 1
1 0 1
```
Target K = 2.

**Permanent computation:**
Enumerate all 3! = 6 permutations of {1, 2, 3}:
- sigma = (1,2,3): M[1,1]*M[2,2]*M[3,3] = 1*1*1 = 1
- sigma = (1,3,2): M[1,1]*M[2,3]*M[3,2] = 1*1*0 = 0
- sigma = (2,1,3): M[1,2]*M[2,1]*M[3,3] = 1*0*1 = 0
- sigma = (2,3,1): M[1,2]*M[2,3]*M[3,1] = 1*1*1 = 1
- sigma = (3,1,2): M[1,3]*M[2,1]*M[3,2] = 0*0*0 = 0
- sigma = (3,2,1): M[1,3]*M[2,2]*M[3,1] = 0*1*1 = 0

perm(M) = 1 + 0 + 0 + 1 + 0 + 0 = 2.

Since perm(M) = 2 = K, the answer is YES.

**Bipartite graph interpretation:**
The matrix M is the biadjacency matrix of a bipartite graph with left vertices {r1, r2, r3} and right vertices {c1, c2, c3}.
Edges: {r1-c1, r1-c2, r2-c2, r2-c3, r3-c1, r3-c3}.
Perfect matchings:
- {r1-c1, r2-c2, r3-c3} (sigma = (1,2,3))
- {r1-c2, r2-c3, r3-c1} (sigma = (2,3,1))
Two perfect matchings = perm(M) = 2. Consistent.

**Larger example (identity matrix):**
M = I_4 (4 x 4 identity). perm(I_4) = 1 (only sigma = id contributes).
With K = 1: answer YES. With K = 2: answer NO.

**All-ones example:**
M = J_3 (3 x 3 all-ones matrix). perm(J_3) = 3! = 6 (every permutation contributes 1).
With K = 6: answer YES.
