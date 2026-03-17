---
name: Problem
about: Propose a new problem type
title: "[Model] RegisterSufficiency"
labels: model
assignees: ''
---

## Motivation

REGISTER SUFFICIENCY (P293) from Garey & Johnson, A11 PO1. A fundamental NP-complete problem in compiler optimization: given a directed acyclic graph representing a straight-line computation and a bound K, determine whether the computation can be performed using at most K registers. The DAG vertices represent values (inputs, intermediates, outputs), and edges represent data dependencies. The evaluation order determines which values must be held simultaneously in registers. NP-complete even when all vertices have out-degree ≤ 2 [Sethi, 1975]. The problem connects compiler register allocation to scheduling theory.

**Associated rules:**
- R225: 3SAT → Register Sufficiency (this model is the **target**)
- R137: Register Sufficiency → Sequencing to Minimize Maximum Cumulative Cost (this model is the **source**)

## Definition

**Name:** `RegisterSufficiency`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO1

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V, A), positive integer K.
QUESTION: Is there a computation for G that uses K or fewer registers, i.e., an ordering v_1, v_2, ..., v_n of the vertices in V, where n = |V|, and a sequence S_0, S_1, ..., S_n of subsets of V, each satisfying |S_i| ≤ K, such that S_0 is empty, S_n contains all vertices with in-degree 0 in G, and, for 1 ≤ i ≤ n, v_i ∈ S_i, S_i \ {v_i} ⊆ S_{i−1}, and S_{i−1} contains all vertices u for which (v_i, u) ∈ A?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V| (one variable per vertex, representing its position in the evaluation order)
- **Per-variable domain:** {0, 1, ..., n−1} — the position of the vertex in the computation ordering
- **Meaning:** π(i) ∈ {0, ..., n−1} gives the evaluation position of vertex v_i. The ordering must be a valid computation: when evaluating v_i, all its successors (operands) must already be in registers (i.e., in the current register set S_{i−1}). The register set at each step must have size ≤ K. Vertices with in-degree 0 are "leaf" inputs that must persist in registers until consumed.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `RegisterSufficiency`
**Variants:** none (no type parameters; the DAG is unweighted)

| Field       | Type              | Description                                                |
|-------------|-------------------|------------------------------------------------------------|
| `num_vertices` | `usize`        | Number of vertices n = |V|                                 |
| `arcs`      | `Vec<(usize, usize)>` | Directed arcs (v, u) ∈ A (v depends on u)            |
| `bound`     | `usize`           | Register bound K                                           |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete [Sethi, 1975]. It remains NP-complete even when all vertices have out-degree ≤ 2. For expression trees (DAGs with tree structure), the Sethi-Ullman algorithm solves the problem in O(n) time. For general DAGs, exact algorithms based on dynamic programming over register states have complexity O(n · 2^n) [Kessler, 1998]. Branch-and-bound methods can handle DAGs with up to 40–50 vertices in practice. The trivial brute-force approach enumerates all n! orderings in O(n! · n) time.

## Extra Remark

**Full book text:**

INSTANCE: Directed acyclic graph G = (V,A), positive integer K.
QUESTION: Is there a computation for G that uses K or fewer registers, i.e., an ordering v1,v2,...,vn of the vertices in V, where n = |V|, and a sequence S0,S1,...,Sn of subsets of V, each satisfying |Si| ≤ K, such that S0 is empty, Sn contains all vertices with in-degree 0 in G, and, for 1 ≤ i ≤ n, vi ∈ Si, Si−{vi} ⊆ Si−1, and Si−1 contains all vertices u for which (vi,u) ∈ A?
Reference: [Sethi, 1975]. Transformation from 3SAT.
Comment: Remains NP-complete even if all vertices of G have out-degree 2 or less. The variant in which "recomputation" is allowed (i.e., we ask for sequences v1,v2,...,vm and S0,S1,...,Sm, where no a priori bound is placed on m and the vertex sequence can contain repeated vertices, but all other properties stated above must hold) is NP-hard and is not known to be in NP.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! evaluation orderings of the DAG; simulate register usage for each; check if max registers ≤ K.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{ij} ∈ {0,1} = vertex i evaluated at step j; enforce dependency constraints; track register set sizes at each step; constrain all set sizes ≤ K.)
- [ ] Other: Sethi-Ullman algorithm for trees [Sethi & Ullman, 1970]; DP over register states for general DAGs.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
DAG G = (V, A):
V = {v_1, v_2, v_3, v_4, v_5, v_6, v_7}
Arcs (directed edges, "depends on"):
- (v_3, v_1): v_3 uses v_1 as input
- (v_3, v_2): v_3 uses v_2 as input
- (v_4, v_2): v_4 uses v_2 as input
- (v_5, v_3): v_5 uses v_3 as input
- (v_5, v_4): v_5 uses v_4 as input
- (v_6, v_1): v_6 uses v_1 as input
- (v_7, v_5): v_7 uses v_5 as input
- (v_7, v_6): v_7 uses v_6 as input

K = 3.

Vertices with in-degree 0 (inputs): v_1, v_2. These values must be in registers when they are first consumed.

**Feasible computation order (evaluation from outputs to inputs):**
Order: v_7, v_5, v_6, v_3, v_4, v_1, v_2

Register sets:
- S_0 = {} (empty)
- Evaluate v_7: need v_5, v_6 in registers. S_1 = {v_5, v_6} → need v_5, v_6 loaded. |S_1| = 2 ≤ 3 ✓
  But wait — at step i we evaluate v_i and remove it. We need S_{i-1} to contain all successors.

Let me use the correct semantics: we evaluate in reverse. Actually, the GJ definition evaluates from root to leaves (consuming values). Let's use a forward computation order (leaves first):

Order: v_1, v_2, v_3, v_4, v_6, v_5, v_7

Register simulation:
- Load v_1: S = {v_1}, |S| = 1 ≤ 3 ✓
- Load v_2: S = {v_1, v_2}, |S| = 2 ≤ 3 ✓
- Compute v_3 (needs v_1, v_2): S = {v_1, v_2, v_3}, |S| = 3 ≤ 3 ✓. v_1 still needed by v_6, v_2 still needed by v_4.
- Compute v_4 (needs v_2): v_2 no longer needed after. S = {v_1, v_3, v_4}, |S| = 3 ≤ 3 ✓
- Compute v_6 (needs v_1): v_1 freed. S = {v_3, v_4, v_6}, |S| = 3 ≤ 3 ✓
- Compute v_5 (needs v_3, v_4): v_3, v_4 freed. S = {v_5, v_6}, |S| = 2 ≤ 3 ✓
- Compute v_7 (needs v_5, v_6): S = {v_7}, |S| = 1 ≤ 3 ✓

Maximum register usage = 3 ≤ K = 3 ✓

Answer: YES — a computation using at most 3 registers exists.
