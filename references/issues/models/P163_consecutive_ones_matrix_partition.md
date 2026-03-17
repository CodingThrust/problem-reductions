---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveOnesMatrixPartition"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE ONES MATRIX PARTITION (P163) from Garey & Johnson, A4 SR15. An NP-complete problem that asks whether the rows of a binary matrix can be split into two groups such that each group's submatrix independently has the consecutive ones property (C1P). This problem arises in computational biology (physical mapping with two chromosomes), scheduling (partitioning tasks into two groups with interval structure), and graph theory (characterizing Hamiltonicity of cubic graphs via the C1P of the adjacency-plus-identity matrix). The NP-hardness comes from the Hamiltonian Path problem restricted to cubic graphs.

**Associated rules:**
- R109: Hamiltonian Path (cubic graphs) -> Consecutive Ones Matrix Partition (this model is the target)

## Definition

**Name:** `ConsecutiveOnesMatrixPartition`
**Canonical name:** CONSECUTIVE ONES MATRIX PARTITION
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR15, p.229

**Mathematical definition:**

INSTANCE: An m x n matrix A of 0's and 1's.
QUESTION: Can the rows of A be partitioned into two groups such that the resulting m_1 x n and m_2 x n matrices (m_1 + m_2 = m) each have the consecutive ones property?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** m binary variables (one per row, indicating which group it belongs to) plus two column permutations (one per group).
- **Per-variable domain:** Row assignment: {0, 1} for each of the m rows (group 0 or group 1). Column permutations: a permutation of {1, ..., n} for each group.
- **Meaning:** The binary variable g_i indicates whether row i belongs to group 0 or group 1. Each group's submatrix must independently have the C1P under its own column permutation. Note that the two groups may use different column permutations.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `ConsecutiveOnesMatrixPartition`
**Variants:** none (no graph or weight parameters)

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<bool>>` | The m x n binary matrix A |
| `num_rows` | `usize` | Number of rows m |
| `num_cols` | `usize` | Number of columns n |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- No bound parameter K is needed (the partition is always into exactly 2 groups).
- The generalization to k groups (k >= 3) is also NP-complete.
- Each group is allowed its own column permutation (the two groups need not share the same column ordering).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Brute-force: enumerate all 2^m row partitions (assigning each row to group 0 or 1), test each group's submatrix for C1P using Booth-Lueker's linear-time PQ-tree algorithm. Total: O(2^m * (m + n + f)) where f is the number of 1-entries.
- **C1P testing:** Each C1P test takes O(m + n + f) time using PQ-trees [Booth and Lueker, 1976].
- **NP-completeness:** NP-complete [Lipsky, 1978], via transformation from HAMILTONIAN PATH for cubic graphs.
- **Related results:** A matrix with the C1P for rows can be recognized in linear time. Partitioning into k >= 2 groups with C1P is NP-complete for every fixed k >= 2.
- **References:**
  - W. Lipsky, Jr. (1978). Unpublished manuscript / technical report on consecutive ones matrix partition.
  - K. S. Booth and G. S. Lueker (1976). "Testing for the consecutive ones property, interval graphs, and graph planarity using PQ-tree algorithms." *JCSS* 13, pp. 335-379.

## Extra Remark

**Full book text:**

INSTANCE: An m x n matrix A of 0's and 1's.
QUESTION: Can the rows of A be partitioned into two groups such that the resulting m1 x n and m2 x n matrices (m1 + m2 = m) each have the consecutive ones property?
Reference: [Lipsky, 1978]. Transformation from HAMILTONIAN PATH for cubic graphs.

**Connection to Hamiltonicity of cubic graphs:** A cubic graph G on p vertices is Hamiltonian if and only if the matrix A + I (where A is the adjacency matrix and I is the identity) has a row permutation that leaves at most 2 blocks of consecutive ones in each column. The connection to this partition problem is that a Hamiltonian path decomposes the edge set of a cubic graph into path edges and non-path edges, inducing two interval structures.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all 2^m row partitions, test each pair of submatrices for C1P.
- [x] It can be solved by reducing to integer programming -- binary variable g_i for each row's group assignment; auxiliary variables for column ordering within each group; C1P constraints encoded as ordering and contiguity constraints.
- [x] Other: Constraint programming with PQ-tree propagation. For small instances, branch-and-bound with C1P feasibility pruning.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (partition exists):**
Matrix A (6 x 5):

|       | c1 | c2 | c3 | c4 | c5 |
|-------|----|----|----|----|-----|
| r1    |  1 |  1 |  0 |  0 |  0 |
| r2    |  0 |  1 |  1 |  0 |  0 |
| r3    |  0 |  0 |  1 |  1 |  0 |
| r4    |  1 |  0 |  0 |  1 |  0 |
| r5    |  0 |  0 |  0 |  1 |  1 |
| r6    |  1 |  0 |  0 |  0 |  1 |

Partition: Group 0 = {r1, r2, r3}, Group 1 = {r4, r5, r6}.

Group 0 submatrix with column order [c1, c2, c3, c4, c5]:

|       | c1 | c2 | c3 | c4 | c5 |
|-------|----|----|----|----|-----|
| r1    |  1 |  1 |  0 |  0 |  0 | 1's at [1,2]: consecutive
| r2    |  0 |  1 |  1 |  0 |  0 | 1's at [2,3]: consecutive
| r3    |  0 |  0 |  1 |  1 |  0 | 1's at [3,4]: consecutive

C1P holds under identity column permutation.

Group 1 submatrix with column order [c4, c1, c5] (only listing columns with any 1):
Actually, the submatrix includes all 5 columns. Try column order [c4, c5, c1, c2, c3]:

|       | c4 | c5 | c1 | c2 | c3 |
|-------|----|----|----|----|-----|
| r4    |  1 |  0 |  1 |  0 |  0 | 1's at [1] and [3]: NOT consecutive

Try column order [c5, c1, c4, c2, c3]:

|       | c5 | c1 | c4 | c2 | c3 |
|-------|----|----|----|----|-----|
| r4    |  0 |  1 |  1 |  0 |  0 | 1's at [2,3]: consecutive
| r5    |  1 |  0 |  1 |  0 |  0 | 1's at [1] and [3]: NOT consecutive

Try column order [c4, c5, c1, c3, c2]:

|       | c4 | c5 | c1 | c3 | c2 |
|-------|----|----|----|----|-----|
| r4    |  1 |  0 |  1 |  0 |  0 | NOT consecutive

Revised partition: Group 0 = {r1, r2, r3, r5}, Group 1 = {r4, r6}.

Group 0 with column order [c1, c2, c3, c4, c5]:

|       | c1 | c2 | c3 | c4 | c5 |
|-------|----|----|----|----|-----|
| r1    |  1 |  1 |  0 |  0 |  0 | [1,2]: consecutive
| r2    |  0 |  1 |  1 |  0 |  0 | [2,3]: consecutive
| r3    |  0 |  0 |  1 |  1 |  0 | [3,4]: consecutive
| r5    |  0 |  0 |  0 |  1 |  1 | [4,5]: consecutive

C1P holds.

Group 1 with column order [c4, c1, c5, c2, c3]:

|       | c4 | c1 | c5 | c2 | c3 |
|-------|----|----|----|----|-----|
| r4    |  1 |  1 |  0 |  0 |  0 | [1,2]: consecutive
| r6    |  0 |  1 |  1 |  0 |  0 | [2,3]: consecutive

C1P holds.

Answer: YES. Partition {r1,r2,r3,r5} / {r4,r6} with respective column orders [c1,c2,c3,c4,c5] and [c4,c1,c5,c2,c3].

**Instance 2 (no valid partition):**
Matrix A (6 x 4) -- Tucker-like obstruction in both groups:

|       | c1 | c2 | c3 | c4 |
|-------|----|----|----|----|
| r1    |  1 |  0 |  1 |  0 |
| r2    |  0 |  1 |  0 |  1 |
| r3    |  1 |  1 |  0 |  0 |
| r4    |  0 |  0 |  1 |  1 |
| r5    |  1 |  0 |  0 |  1 |
| r6    |  0 |  1 |  1 |  0 |

Rows r1, r2, r5 form a Tucker obstruction (each pair forces incompatible column orderings for C1P). Similarly r3, r4, r6 form another obstruction. Any partition into two groups will place at least one Tucker triple entirely within one group, violating C1P for that group.

Verification: r1=[1,0,1,0], r2=[0,1,0,1], r5=[1,0,0,1].
- r1 needs c1 and c3 adjacent.
- r2 needs c2 and c4 adjacent.
- r5 needs c1 and c4 adjacent.
For r1: c1,c3 adjacent -> order includes ...c1,c3... or ...c3,c1...
For r5: c1,c4 adjacent -> c4 must be next to c1.
For r2: c2,c4 adjacent -> c2 must be next to c4.
So: c2 next to c4 next to c1 next to c3 (or reverse). Check r1: c1,c3 adjacent. Check r2: c2,c4 adjacent. Check r5: c1=[1],c4=[1] -- c1 and c4 at positions 3 and 2: adjacent. OK!
Actually this works: column order [c2,c4,c1,c3]:
- r1: [0,0,1,1] consecutive
- r2: [1,1,0,0] consecutive
- r5: [0,1,1,0] consecutive

So {r1,r2,r5} does have C1P. Check {r3,r4,r6} under some permutation:
r3=[1,1,0,0], r4=[0,0,1,1], r6=[0,1,1,0].
Try [c1,c2,c3,c4]:
- r3: [1,1,0,0] consecutive
- r4: [0,0,1,1] consecutive
- r6: [0,1,1,0] consecutive

This works! So the matrix is actually partitionable. Answer: YES.

This problem is harder to construct NO instances for by hand. A NO instance requires that for every row partition into two groups, at least one group contains a Tucker-type obstruction to C1P that cannot be resolved by any column permutation. Such instances arise specifically from the Hamiltonian path reduction on non-Hamiltonian cubic graphs.
