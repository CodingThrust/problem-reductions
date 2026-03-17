---
name: Problem
about: Propose a new problem type
title: "[Model] TwoDimensionalConsecutiveSets"
labels: model
assignees: ''
---

## Motivation

2-DIMENSIONAL CONSECUTIVE SETS (P167) from Garey & Johnson, A4 SR19. An NP-complete problem from the domain of storage and retrieval. Given an alphabet and a collection of subsets, the question asks whether the alphabet can be partitioned into disjoint groups arranged in a sequence such that each subset is "covered" by consecutive groups with at most one element per group. This generalizes the consecutive sets problem to a two-dimensional arrangement.

**Associated rules:**
- R113: Graph 3-Colorability -> 2-Dimensional Consecutive Sets (as target)

## Definition

**Name:** `TwoDimensionalConsecutiveSets`
**Canonical name:** 2-DIMENSIONAL CONSECUTIVE SETS
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR19

**Mathematical definition:**

INSTANCE: Finite alphabet Sigma, collection C = {Sigma_1, Sigma_2, ..., Sigma_n} of subsets of Sigma.
QUESTION: Is there a partition of Sigma into disjoint sets X_1, X_2, ..., X_k such that each X_i has at most one element in common with each Sigma_j, and such that for each Sigma_j in C, there is an index l(j) such that Sigma_j is contained in X_{l(j)} union X_{l(j)+1} union ... union X_{l(j)+|Sigma_j|-1}?

## Variables

<!-- Unverified: AI-inferred variable mapping -->
- **Count:** |Sigma| assignment variables (one per symbol) plus the number of groups k (which is itself a decision).
- **Per-variable domain:** Each symbol s in Sigma is assigned to a group index in {1, 2, ..., k}.
- **Meaning:** A satisfying assignment partitions Sigma into ordered groups X_1, ..., X_k such that (1) each X_i intersects each Sigma_j in at most one element, and (2) each Sigma_j's elements are spread across exactly |Sigma_j| consecutive groups.

## Schema (data type)

<!-- Unverified: AI-designed schema -->
**Type name:** `TwoDimensionalConsecutiveSets`
**Variants:** None

| Field | Type | Description |
|-------|------|-------------|
| `alphabet` | `Vec<char>` | The finite alphabet Sigma |
| `subsets` | `Vec<Vec<char>>` | The collection C of subsets of Sigma |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Unlike ConsecutiveSets (SR18), there is no explicit bound K; the question is purely about the existence of a valid partition.
- The partition into groups can be viewed as a 2D arrangement: the groups form one dimension (columns) and the elements within each group form the other dimension (rows).

## Complexity

<!-- Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(2^|Sigma|) by trying all partitions and orderings. More precisely, enumerate partitions of Sigma into groups and check the coverage and intersection constraints.
- **NP-completeness:** NP-complete [Lipsky, 1977b]. Transformation from GRAPH 3-COLORABILITY.
- **Restricted cases:** Remains NP-complete if all Sigma_j in C have |Sigma_j| <= 5. Solvable in polynomial time if all Sigma_j in C have |Sigma_j| <= 2.
- **References:**
  - W. Lipski Jr. (1977). "One more polynomial complete consecutive retrieval problem." *Information Processing Letters*, 6(3):91-93.
  - W. Lipski Jr. (1977). "Two NP-complete problems related to information retrieval." *Fundamentals of Computation Theory, FCT 1977*, Lecture Notes in Computer Science, vol. 56, Springer.

## Extra Remark

**Full book text:**

INSTANCE: Finite alphabet Sigma, collection C = {Sigma_1, Sigma_2, ..., Sigma_n} of subsets of Sigma.
QUESTION: Is there a partition of Sigma into disjoint sets X1, X2, ..., Xk such that each Xi has at most one element in common with each Sigma_j and such that, for each Sigma_j in C, there is an index l(j) such that Sigma_j is contained in Xl(j) union Xl(j)+1 union ... union Xl(j)+|Sigma_j|-1?
Reference: [Lipsky, 1977b]. Transformation from GRAPH 3-COLORABILITY.
Comment: Remains NP-complete if all Sigma_j in C have |Sigma_j| <= 5, but is solvable in polynomial time if all Sigma_j in C have |Sigma_j| <= 2.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all partitions of Sigma into ordered groups and verify the intersection and consecutiveness constraints.
- [x] It can be solved by reducing to integer programming -- assign each symbol to a group index, add constraints for intersection bounds and consecutive coverage.
- [x] Other: Constraint programming with propagation; reduction from/to graph coloring for small subset sizes.

## Example Instance

<!-- Unverified: AI-constructed example -->

**Instance 1 (YES instance):**
Alphabet: Sigma = {a, b, c, d, e, f}
Subsets: C = {{a, b, c}, {d, e, f}, {b, d}, {c, e}}

Partition into 3 groups: X_1 = {a, d}, X_2 = {b, e}, X_3 = {c, f}.

Verification:
- {a, b, c}: a in X_1, b in X_2, c in X_3. Groups 1,2,3 are consecutive. |{a,b,c} intersect X_i| <= 1 for each i. Covered by X_1 union X_2 union X_3. YES.
- {d, e, f}: d in X_1, e in X_2, f in X_3. Groups 1,2,3. Covered by X_1 union X_2 union X_3. YES.
- {b, d}: b in X_2, d in X_1. Groups 1,2 are consecutive. Covered by X_1 union X_2. YES.
- {c, e}: c in X_3, e in X_2. Groups 2,3 are consecutive. Covered by X_2 union X_3. YES.
Answer: YES

**Instance 2 (NO instance):**
Alphabet: Sigma = {a, b, c, d, e, f}
Subsets: C = {{a, b, c}, {a, d, e}, {a, f, b}, {c, d, f}}

Consider subset {a, b, c}: needs 3 consecutive groups, each containing exactly one of a, b, c.
Consider subset {a, d, e}: needs 3 consecutive groups, each containing exactly one of a, d, e.
Consider subset {a, f, b}: needs 3 consecutive groups, each containing exactly one of a, f, b.
Symbol 'a' appears in three subsets of size 3 and must be in a group that is part of each of these three consecutive triples. The constraints on overlapping triples all containing 'a' force conflicting group orderings.
Also {c, d, f} must span 3 consecutive groups, but c, d, f are each constrained by the other subsets. No valid partition exists.
Answer: NO
