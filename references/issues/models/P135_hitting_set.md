---
name: Problem
about: Propose a new problem type
title: "[Model] HittingSet"
labels: model
assignees: ''
---

## Motivation

HITTING SET (P135) from Garey & Johnson, A3 SP8. A classical NP-complete problem useful for reductions. It is the dual of SET COVER: while Set Cover asks which sets to pick to cover all universe elements, Hitting Set asks which universe elements to pick to "hit" all sets. Every instance of VERTEX COVER reduces to HITTING SET by encoding each edge as a 2-element subset.

## Definition

**Name:** <!-- ⚠️ Unverified --> `HittingSet`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Hitting Set (also: Transversal Problem)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP8

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |S|.
QUESTION: Is there a subset S' ⊆ S with |S'| ≤ K such that S' contains at least one element from each subset in C?

The problem is a decision (satisfaction) problem: there is no natural optimization direction embedded in the GJ formulation, though the minimum hitting set size is a natural optimization variant.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |S| (one binary variable per universe element)
- **Per-variable domain:** binary {0, 1} — whether element s ∈ S is included in the hitting set S'
- **Meaning:** variable x_i = 1 if element i is selected into the hitting set; the configuration (x_0, ..., x_{|S|-1}) encodes a candidate subset S' ⊆ S. The assignment is valid if for every subset c ∈ C, at least one element of c has x_i = 1.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `HittingSet`
**Variants:** none (no graph or weight type parameter needed; the collection is stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `universe_size` | `usize` | Number of elements in S (elements are indexed 0..universe_size) |
| `subsets` | `Vec<Vec<usize>>` | The collection C; each inner Vec is a subset of element indices |
| `budget` | `usize` | The budget K: hitting set must have size ≤ budget |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Karp, 1972; transformation from VERTEX COVER).
- **Best known exact algorithm:** Brute-force enumeration of all 2^|S| subsets of S in O(2^|S| · |S| · |C|) time; for fixed-parameter tractability parameterized by k (budget), an FPT algorithm runs in O(|C|^k · poly(|S|, |C|)) time, but is exponential in k.
- **Parameterized:** W[2]-complete parameterized by solution size k, meaning no FPT algorithm parameterized by k is expected under standard complexity assumptions. FPT algorithms exist for d-bounded instances (each subset has size ≤ d): for 3-Hitting Set an O(2.270^k · n) algorithm is known (Abu-Khzam, 2010).
- **References:**
  - [Karp, 1972] R. M. Karp, "Reducibility Among Combinatorial Problems", *Complexity of Computer Computations*, pp. 85–103. Original NP-completeness proof.
  - [Abu-Khzam, 2010] F. N. Abu-Khzam, "An improved kernelization algorithm for r-Set Packing", *Information Processing Letters* 110(16), pp. 621–624. FPT result for bounded-arity hitting set.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a generalization of:** VERTEX COVER (special case where every subset has exactly 2 elements, corresponding to edges)
- **Known special cases:** VERTEX COVER (|c| = 2 for all c ∈ C), 3-Hitting Set (|c| ≤ 3 for all c ∈ C)
- **Restriction:** VERTEX COVER is obtained by restricting C to 2-element subsets (edges of a graph)

## Extra Remark

**Full book text:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |S|.
QUESTION: Is there a subset S' ⊆ S with |S'| ≤ K such that S' contains at least one element from each subset in C?
Reference: [Karp, 1972]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if |c| ≤ 2 for all c ∈ C.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all 2^|S| subsets of S; for each candidate S', check if it hits every subset in C and has |S'| ≤ K.
- [x] It can be solved by reducing to integer programming. Introduce binary variable x_i for each element; minimize ∑x_i subject to ∑_{i∈c} x_i ≥ 1 for all c ∈ C and ∑x_i ≤ K.
- [ ] Other: (none identified)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Universe S = {0, 1, 2, 3, 4, 5} (6 elements)
Collection C (7 subsets):
- c_0 = {0, 1, 2}
- c_1 = {0, 3, 4}
- c_2 = {1, 3, 5}
- c_3 = {2, 4, 5}
- c_4 = {0, 1, 5}
- c_5 = {2, 3}
- c_6 = {1, 4}

Budget K = 3

**Greedy trap:** Greedily picking the element appearing most often might pick element 1 (appears in c_0, c_2, c_4, c_6 — 4 subsets), but this alone leaves c_1, c_3, c_5 uncovered and we still need 2 more elements. A better choice requires looking at coverage interaction.

**Optimal hitting set:** S' = {1, 3, 4} (size 3 = K):
- c_0 = {0,1,2}: 1 ∈ S' ✓
- c_1 = {0,3,4}: 3,4 ∈ S' ✓
- c_2 = {1,3,5}: 1,3 ∈ S' ✓
- c_3 = {2,4,5}: 4 ∈ S' ✓
- c_4 = {0,1,5}: 1 ∈ S' ✓
- c_5 = {2,3}: 3 ∈ S' ✓
- c_6 = {1,4}: 1,4 ∈ S' ✓

All 7 subsets are hit by S' = {1, 3, 4} with |S'| = 3 ≤ K ✓.
