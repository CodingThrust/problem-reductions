---
name: Problem
about: Propose a new problem type
title: "[Model] ExactCoverBy3Sets(x3c)"
labels: model
assignees: ''
---

## Motivation

EXACT COVER BY 3-SETS (X3C) (P129) from Garey & Johnson, A3 SP2. A classical NP-complete covering problem: given a universe X of 3q elements and a collection C of 3-element subsets, does C contain an exact cover — a subcollection of q disjoint triples covering every element exactly once? Shown NP-complete by Karp (1972) via transformation from 3-DIMENSIONAL MATCHING. X3C remains NP-complete even when no element appears in more than three subsets, but is solvable in polynomial time when no element appears in more than two subsets. X3C is one of the most widely used source problems for NP-completeness reductions, serving as the starting point for proving hardness of problems in scheduling, graph theory, set systems, coding, and number theory.

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules (incoming):**
- R03: 3DM -> X3C (Karp, 1972)

**Associated rules (outgoing, X3C as source):**
- R11: X3C -> Minimum Cover
- R19: X3C -> Partition into Triangles
- R27: X3C -> Geometric Capacitated Spanning Tree
- R28: X3C -> Optimum Communication Spanning Tree
- R33: X3C -> Steiner Tree in Graphs
- R34: X3C -> Geometric Steiner Tree
- R36: X3C -> Acyclic Partition
- R44: X3C -> Geometric TSP
- R53: X3C -> Minimum Edge-Cost Flow
- R72: X3C -> Set Packing
- R79: X3C -> Subset Product
- R83: X3C -> Expected Component Sum
- R118: X3C -> Regular Expression Substitution
- R149: X3C -> Staff Scheduling
- R156: X3C -> Minimum Weight Solution to Linear Equations
- R158: X3C -> K-Relevancy
- R172: X3C -> Algebraic Equations over GF[2]
- R195: X3C -> Crossword Puzzle Construction
- R213: X3C -> Minimum Axiom Set
- R255: X3C -> Elimination Degree Sequence
- R290: X3C -> Bounded Diameter Spanning Tree
- R291: X3C -> Optimum Communication Spanning Tree
- R337: X3C -> Decision Tree
- R338: X3C -> Fault Detection in Directed Graphs
- R339: X3C -> Fault Detection with Test Points
- R340: X3C -> Minimum Weight AND/OR Graph Solution
- R341: X3C -> Permutation Generation

## Definition

**Name:** `ExactCoverBy3Sets`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP2

**Mathematical definition:**

INSTANCE: Set X with |X| = 3q and a collection C of 3-element subsets of X.
QUESTION: Does C contain an exact cover for X, i.e., a subcollection C' ⊆ C such that every element of X occurs in exactly one member of C'?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |C| (one binary variable per 3-element subset)
- **Per-variable domain:** {0, 1} — whether the subset is included in the cover
- **Meaning:** y_j = 1 if subset S_j is selected for the cover; 0 otherwise. The constraints require that every element of X appears in exactly one selected subset, and exactly q subsets are selected (since |X| = 3q and each subset covers 3 elements).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ExactCoverBy3Sets`
**Variants:** none (no type parameters)

| Field           | Type                  | Description                                           |
|-----------------|-----------------------|-------------------------------------------------------|
| `universe_size` | `usize`               | Size of universe |X| = 3q                            |
| `subsets`       | `Vec<[usize; 3]>`    | Collection C of 3-element subsets (indices into X)    |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete (Karp, 1972) and strongly NP-hard. Knuth's Algorithm X with Dancing Links (DLX) is the most widely used practical exact solver. For worst-case bounds, the problem can be solved via inclusion-exclusion or subset enumeration in O*(2^|C|) time, or more precisely using the relationship to Set Cover: an O*(2^n) algorithm where n = |X| = 3q is achievable via inclusion-exclusion (Bjorklund, Husfeldt, Koivisto, 2009). Since each subset has exactly 3 elements, structured enumeration may perform better in practice, but no known worst-case bound significantly improves upon O*(2^(3q)) in general.

## Extra Remark

**Full book text:**

INSTANCE: Set X with |X| = 3q and a collection C of 3-element subsets of X.
QUESTION: Does C contain an exact cover for X, i.e., a subcollection C' ⊆ C such that every element of X occurs in exactly one member of C'?
Reference: [Karp, 1972]. Transformation from 3DM.
Comment: Remains NP-complete if no element occurs in more than three subsets, but is solvable in polynomial time if no element occurs in more than two subsets [Garey and Johnson, ——]. Related EXACT COVER BY 2-SETS problem is also solvable in polynomial time by matching techniques.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all subcollections of C of size q; check if they form an exact cover — each element of X appears exactly once.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: y_j in {0,1} for each S_j in C; for each element x in X: sum_{j: x in S_j} y_j = 1; sum y_j = q.)
- [ ] Other: Knuth's Algorithm X with Dancing Links (DLX); backtracking with constraint propagation.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
X = {1, 2, 3, 4, 5, 6, 7, 8, 9} (q = 3, so |X| = 9)
C = {S_1, S_2, S_3, S_4, S_5, S_6, S_7}:
- S_1 = {1, 2, 3}
- S_2 = {1, 3, 5}
- S_3 = {4, 5, 6}
- S_4 = {4, 6, 8}
- S_5 = {7, 8, 9}
- S_6 = {2, 5, 7}
- S_7 = {3, 6, 9}

**Exact cover:**
C' = {S_1, S_3, S_5} = {{1,2,3}, {4,5,6}, {7,8,9}}
- S_1 covers {1,2,3}
- S_3 covers {4,5,6}
- S_5 covers {7,8,9}
- Union = {1,2,3,4,5,6,7,8,9} = X ✓
- All pairwise disjoint ✓
- |C'| = 3 = q ✓

Answer: YES — an exact cover exists.
