---
name: Problem
about: Propose a new problem type
title: "[Model] RootedTreeStorageAssignment"
labels: model
assignees: ''
---

## Motivation

ROOTED TREE STORAGE ASSIGNMENT (P153) from Garey & Johnson, A4 SR5. An NP-complete problem from the Storage and Retrieval category. Given a finite set X and a collection of subsets of X, the goal is to find a directed rooted tree on X and extend each subset (by adding extra elements) so that each extended subset forms a directed path in the tree, while minimizing the total number of added elements. This models the problem of organizing data in a hierarchical (tree-structured) storage system where related data items (subsets) must be stored along contiguous paths in the hierarchy. NP-completeness is proved by Gavril (1977) via reduction from ROOTED TREE ARRANGEMENT.

**Associated rules:**
- R099: Rooted Tree Arrangement → Rooted Tree Storage Assignment (as target)

## Definition

**Name:** `RootedTreeStorageAssignment`
**Canonical name:** ROOTED TREE STORAGE ASSIGNMENT
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR5, p.227

**Mathematical definition:**

INSTANCE: Finite set X, collection C = {X_1, X_2, ..., X_n} of subsets of X, positive integer K.
QUESTION: Is there a collection C' = {X_1', X_2', ..., X_n'} of subsets of X such that X_i is a subset of X_i' for 1 <= i <= n, such that sum_{i=1}^{n} |X_i' - X_i| <= K, and such that there is a directed rooted tree T = (X, A) in which the elements of each X_i', 1 <= i <= n, form a directed path?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** The solution has two components: (1) a directed rooted tree T on |X| nodes, and (2) for each subset X_i, a superset X_i' that forms a path in T. Together this involves choosing both the tree structure and n path extensions.
- **Per-variable domain:** For the tree: one of the many possible rooted trees on |X| nodes. For each extension X_i': a superset of X_i whose elements form a directed path in T.
- **Meaning:** The variable assignment encodes a hierarchical storage layout (the tree T) and the allocation of each data group (X_i) to a contiguous path in the tree. Elements added to X_i (forming X_i' - X_i) represent "wasted" storage slots used to maintain path contiguity. The total wasted storage must not exceed K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `RootedTreeStorageAssignment`
**Variants:** none (no type parameters)

| Field             | Type                | Description                                                |
|-------------------|---------------------|------------------------------------------------------------|
| `universe_size`   | `usize`             | Size of the finite set X (elements labeled 0..|X|-1)       |
| `subsets`         | `Vec<Vec<usize>>`   | Collection C of subsets of X (each subset is a Vec of element indices) |
| `bound`           | `usize`             | Maximum total extension cost K                             |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- A "directed path" in T means a sequence of nodes x_1, x_2, ..., x_k where each x_{i+1} is a child of x_i (i.e., the path goes from ancestor to descendant along the tree).
- The tree T is part of the solution, not the instance. The solver must find both T and the extensions.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** No specialized sub-exponential exact algorithm is known. Brute-force: enumerate all rooted trees on |X| nodes, and for each tree, determine optimal path extensions for each subset (can be done greedily once the tree is fixed). The bottleneck is the exponential number of tree topologies.
- **NP-completeness:** NP-complete [Gavril, 1977a], via reduction from ROOTED TREE ARRANGEMENT (GT45).
- **Special cases:** If all subsets have size <= 2 (pairs), the problem relates directly to tree arrangement. If the tree T is given as part of the input, finding optimal extensions can be done in polynomial time.
- **References:**
  - F. Gavril (1977). "Some NP-complete problems on graphs." In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91-95. Johns Hopkins University.

## Extra Remark

**Full book text:**

INSTANCE: Finite set X, collection C = {X_1, X_2, ..., X_n} of subsets of X, positive integer K.
QUESTION: Is there a collection C' = {X_1', X_2', ..., X_n'} of subsets of X such that X_i is a subset of X_i' for 1 <= i <= n, such that sum_{i=1}^{n} |X_i' - X_i| <= K, and such that there is a directed rooted tree T = (X, A) in which the elements of each X_i', 1 <= i <= n, form a directed path?
Reference: [Gavril, 1977a]. Transformation from ROOTED TREE ARRANGEMENT.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all possible rooted trees on |X| nodes, for each tree find the minimum-cost extensions of each subset to form directed paths, sum costs and check against K.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: When the tree T is fixed (given as input), the extension problem for each subset reduces to finding the shortest directed path in T that contains all elements of the subset. This is solvable in O(|X|) per subset by finding the lowest common ancestor and verifying that all subset elements lie on the path from the LCA to the deepest element.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, chain tree works):**
Universe X = {0, 1, 2, 3, 4, 5} (|X| = 6).
Collection C:
- X_1 = {0, 2} (elements 0 and 2)
- X_2 = {1, 3} (elements 1 and 3)
- X_3 = {2, 4} (elements 2 and 4)
- X_4 = {3, 5} (elements 3 and 5)
- X_5 = {0, 5} (elements 0 and 5)
- X_6 = {1, 4} (elements 1 and 4)

Bound K = 8.

Solution: Use chain tree T: 0 -> 1 -> 2 -> 3 -> 4 -> 5 (rooted at 0).

Extended subsets (each must form a directed path in T):
- X_1' = {0, 1, 2}: path 0->1->2, extending {0,2} by adding {1}. Cost = 1.
- X_2' = {1, 2, 3}: path 1->2->3, extending {1,3} by adding {2}. Cost = 1.
- X_3' = {2, 3, 4}: path 2->3->4, extending {2,4} by adding {3}. Cost = 1.
- X_4' = {3, 4, 5}: path 3->4->5, extending {3,5} by adding {4}. Cost = 1.
- X_5' = {0, 1, 2, 3, 4, 5}: path 0->1->2->3->4->5, extending {0,5} by adding {1,2,3,4}. Cost = 4.
- X_6' = {1, 2, 3, 4}: path 1->2->3->4, extending {1,4} by adding {2,3}. Cost = 2.
- Total cost = 1+1+1+1+4+2 = 10 > K = 8. Need K >= 10 or a better tree.

Revised: Set K = 10. Answer: YES with the chain tree. Total extension cost = 10 <= 10 ✓.

**Instance 2 (YES, star tree is better for some instances):**
Universe X = {0, 1, 2, 3, 4, 5} (|X| = 6).
Collection C:
- X_1 = {0, 1}
- X_2 = {0, 2}
- X_3 = {0, 3}
- X_4 = {0, 4}
- X_5 = {0, 5}
- X_6 = {1, 2}

Bound K = 2.

Solution with star tree (root 0, children 1..5):
```
    0
   /|\ \ \
  1 2 3 4 5
```
- X_1' = {0, 1}: path 0->1, no extension. Cost = 0.
- X_2' = {0, 2}: path 0->2, no extension. Cost = 0.
- X_3' = {0, 3}: path 0->3, no extension. Cost = 0.
- X_4' = {0, 4}: path 0->4, no extension. Cost = 0.
- X_5' = {0, 5}: path 0->5, no extension. Cost = 0.
- X_6' = {0, 1, 2}... wait, {1, 2} are siblings (both children of 0). They are NOT on the same directed path. We need to extend to include 0. But then {0, 1, 2} has 0->1 and 0->2 which are two separate paths, not a single directed path!

In a star tree, X_6 = {1,2} cannot be extended to a directed path since 1 and 2 are in different branches. We need a tree where 1 and 2 are on the same root-to-leaf path.

Revised tree:
```
    0 -> 1 -> 2
    |
    3
    |
    4
    |
    5
```
- X_1' = {0, 1}: path 0->1. Cost = 0.
- X_2' = {0, 1, 2}: path 0->1->2, extending {0,2} by adding {1}. Cost = 1.
- X_3' = {0, 3}: path 0->3. Cost = 0.
- X_4' = {0, 3, 4}: path 0->3->4, extending {0,4} by adding {3}. Cost = 1.
- X_5' = {0, 3, 4, 5}: path 0->3->4->5, extending {0,5} by adding {3,4}. Cost = 2.
- X_6' = {1, 2}: path 1->2. Cost = 0.
- Total cost = 0+1+0+1+2+0 = 4 > K = 2.

Set K = 4. Answer: YES ✓.

**Instance 3 (NO, conflicting path requirements):**
Universe X = {0, 1, 2, 3, 4, 5} (|X| = 6).
Collection C:
- X_1 = {0, 5}
- X_2 = {1, 4}
- X_3 = {2, 3}
- X_4 = {0, 3}
- X_5 = {1, 5}
- X_6 = {2, 4}

Bound K = 0.

For K = 0, we need each pair to already form a directed path in some tree. This means every pair must be in an ancestor-descendant relationship. But a rooted tree on 6 nodes has at most 5 ancestor-descendant pairs (parent-child edges). We have 6 required pairs. Since each pair constrains the tree, it is very restrictive. In fact, the pairs {0,5},{1,4},{2,3} plus {0,3},{1,5},{2,4} form a structure requiring all 6 elements on a single chain, but then the extension cost for non-adjacent pairs is > 0. Answer: NO.
