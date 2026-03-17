---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedHamiltonianPath"
labels: model
assignees: ''
---

## Motivation

DIRECTED HAMILTONIAN PATH (P1) from Garey & Johnson, Chapter 3, p.60. A fundamental NP-complete graph problem: given a directed graph G = (V, A), does there exist a Hamiltonian path, i.e., an ordering of all vertices such that consecutive vertices in the ordering are connected by arcs? This is the directed counterpart of the undirected Hamiltonian Path problem, obtained by replacing each edge {u,v} with two arcs (u,v) and (v,u). NP-completeness follows directly from the undirected version.

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R145: DIRECTED HAMILTONIAN PATH -> NO-WAIT FLOW-SHOP SCHEDULING (establishes NP-completeness of no-wait flow-shop)

## Definition

**Name:** `DirectedHamiltonianPath`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, p.60

**Mathematical definition:**

INSTANCE: A directed graph G = (V, A).
QUESTION: Does G contain a Hamiltonian path, i.e., an ordering of V as <v_1, v_2, ..., v_n>, where n = |V|, such that (v_i, v_{i+1}) in A for 1 <= i < n?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V| (one variable per vertex, representing its position in the path)
- **Per-variable domain:** {0, 1, ..., n-1} -- the position of vertex v in the Hamiltonian path ordering
- **Meaning:** pi(v) in {0, ..., n-1} assigns each vertex a unique position in the path. The path is valid iff for every consecutive pair (v at position i, w at position i+1), the arc (v, w) exists in A.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `DirectedHamiltonianPath`
**Variants:** none (operates on a general directed graph)

| Field      | Type                    | Description                                      |
|------------|-------------------------|--------------------------------------------------|
| `vertices` | `usize`                 | Number of vertices n = |V|                       |
| `arcs`     | `Vec<(usize, usize)>`   | Directed arcs (u, v) in A                        |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The Held-Karp dynamic programming algorithm solves the Hamiltonian path problem in O(n^2 * 2^n) time and O(n * 2^n) space [Bellman, 1962; Held and Karp, 1962]. For directed graphs, Bjorklund's inclusion-exclusion based algorithm achieves O*(1.657^n) time using polynomial space for detecting Hamiltonian cycles [Bjorklund, 2014]; similar techniques apply to paths. The parity of Hamiltonian cycles can be computed in O(1.619^n) time and polynomial space [Bjorklund and Husfeldt, 2013].

## Extra Remark

**Full book text:**

All three Hamiltonian problems mentioned so far also remain NP-complete if we replace the undirected graph G by a directed graph and replace the undirected Hamiltonian circuit or path by a directed Hamiltonian circuit or path. Recall that a directed graph G = (V, A) consists of a vertex set V and a set of ordered pairs of vertices called arcs. A Hamiltonian path in a directed graph G = (V, A) is an ordering of V as <v_1, v_2, . . . , v_n>, where n = |V|, such that (v_i, v_{i+1}) in A for 1 <= i < n. A Hamiltonian circuit has the additional requirement that (v_n, v_1) in A. Each of the three undirected Hamiltonian problems can be transformed to its directed counterpart simply by replacing each edge {u, v} in the given undirected graph by the two arcs (u, v) and (v, u). In essence, the undirected versions are merely special cases of their directed counterparts.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! permutations of vertices; check if each consecutive pair has an arc.)
- [x] It can be solved by reducing to integer programming. (ILP with binary variables x_{v,k} = 1 if vertex v is at position k; constraints: each vertex at exactly one position, each position has exactly one vertex, and for each consecutive position pair (k, k+1), the selected vertices must have an arc between them.)
- [ ] Other: Held-Karp DP in O(n^2 * 2^n); Bjorklund's algebraic methods.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
G = (V, A) with V = {1, 2, 3, 4, 5} (n = 5 vertices)
A = {(1,2), (1,3), (2,3), (2,5), (3,4), (4,5), (5,1), (3,1), (4,2)}

**Hamiltonian path:** <1, 3, 4, 2, 5>
Check: (1,3) in A ✓, (3,4) in A ✓, (4,2) in A ✓, (2,5) in A ✓.
All 5 vertices visited exactly once. ✓

Answer: YES -- a directed Hamiltonian path exists.

**Alternative path:** <1, 2, 3, 4, 5>
Check: (1,2) in A ✓, (2,3) in A ✓, (3,4) in A ✓, (4,5) in A ✓. Also valid. ✓

**Negative example:**
Remove arc (4,5) and (2,5). Now vertex 5 has no incoming arcs from {1,2,3,4} except (5,1) is outgoing. Only (4,5) was incoming and it's removed. Vertex 5 is unreachable as a non-first vertex unless we start at 5: but (5,1) goes to 1, then we need to reach 2,3,4 -- checking all permutations shows no Hamiltonian path exists.
