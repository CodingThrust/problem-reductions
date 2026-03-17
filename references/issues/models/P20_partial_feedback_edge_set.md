---
name: Problem
about: Propose a new problem type
title: "[Model] PartialFeedbackEdgeSet"
labels: model
assignees: ''
---

## Motivation

PARTIAL FEEDBACK EDGE SET (P20) from Garey & Johnson, A1.1 GT9. A classical NP-complete graph problem asking for the minimum number of edges to remove so that all cycles of length L or less are broken. This generalizes the standard feedback edge set problem (where L = |V|, which is trivially solvable) by restricting attention to short cycles, making the problem hard. It remains NP-complete for any fixed L >= 3.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **R282:** VERTEX COVER -> PARTIAL FEEDBACK EDGE SET (this is the GJ reference reduction, by Yannakakis 1978)
- **R115:** FEEDBACK EDGE SET -> GROUPING BY SWAPPING (related: FEEDBACK EDGE SET is the L=|V| special case; the Partial variant is the source for P20)

## Definition

**Name:** <!-- ⚠️ Unverified --> `PartialFeedbackEdgeSet`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Partial Feedback Edge Set (also: Bounded-Length Cycle Edge Transversal)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT9

**Mathematical definition:**

INSTANCE: Graph G = (V, E), positive integers K <= |E| and L <= |V|.
QUESTION: Is there a subset E' of E with |E'| <= K such that E' contains at least one edge from every circuit of length L or less in G?

The problem is a decision (satisfaction) problem: the answer is YES or NO depending on whether the bounded-length cycle transversal of size at most K exists.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |E| (one binary variable per edge)
- **Per-variable domain:** binary {0, 1} -- whether edge e in E is included in the feedback edge set E'
- **Meaning:** variable x_e = 1 if edge e is selected for removal. The configuration (x_{e_1}, ..., x_{e_m}) encodes a candidate subset E' of E. The assignment is valid if (1) the number of selected edges is at most K, and (2) every cycle of length <= L in G contains at least one selected edge.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `PartialFeedbackEdgeSet<G>`
**Variants:** Graph type G (SimpleGraph, PlanarGraph, BipartiteGraph)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `G` | The undirected graph G = (V, E) |
| `budget` | `usize` | The budget K: maximum number of edges to remove |
| `max_cycle_length` | `usize` | The cycle length bound L: all cycles of length <= L must be hit |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Yannakakis, 1978; transformation from VERTEX COVER). Remains NP-complete for any fixed L >= 3, and also for bipartite graphs with fixed L >= 4.
- **Trivial case:** When L = |V| (no length restriction), the problem asks for a feedback edge set, which is equivalent to finding a spanning forest. This is trivially solvable in polynomial time: remove exactly m - n + c edges, where c is the number of connected components (any spanning forest suffices).
- **Best known exact algorithm:** For general graphs, brute-force enumeration of all 2^|E| subsets of E in O(2^|E| * poly(|V|, |E|)) time, where checking the constraint requires finding all cycles of length <= L (which can be done in O(|V|^L) time by DFS). For the closely related feedback vertex set problem, exact algorithms run in O(1.7347^n) time (Fomin et al., 2006).
- **Parameterized:** For fixed L, the problem can be approached via bounded search tree methods. When parameterized by the solution size K, the problem is likely FPT for fixed L (since each short cycle provides a bounded branching factor).
- **References:**
  - [Yannakakis, 1978b] M. Yannakakis, "Node- and edge-deletion NP-complete problems", Proc. 10th STOC, pp. 253-264.
  - [Fomin et al., 2006] F. V. Fomin, S. Gaspers, A. V. Pyatkin, "Finding a minimum feedback vertex set in time O(1.7548^n)", IWPEC 2006.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a generalization of:** VERTEX COVER restricted to triangle-free graphs (when L = 3, every cycle of length 3 = triangle must be hit; on graphs where every edge is in a triangle, this is close to vertex cover)
- **Special cases:**
  - L = |V|: Feedback Edge Set (polynomial-time solvable; equivalent to finding a spanning forest)
  - L = 3: every triangle must be hit by at least one edge removal
  - L = 4 on bipartite graphs: still NP-complete

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), positive integers K <= |E| and L <= |V|.
QUESTION: Is there a subset E' of E with |E'| <= K such that E' contains at least one edge from every circuit of length L or less in G?
Reference: [Yannakakis, 1978b]. Transformation from VERTEX COVER.
Comment: Remains NP-complete for any fixed L >= 3 and for bipartite graphs (with fixed L >= 4). However, if L = |V|, i.e., if we ask that E' contain an edge from every cycle in G, then the problem is trivially solvable in polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all subsets E' of E with |E'| <= K; for each subset, check if every cycle of length <= L in G contains at least one edge from E'. Cycle enumeration can be done by DFS up to depth L.
- [x] It can be solved by reducing to integer programming. Introduce binary variable x_e for each edge e in E. For each cycle C = (e_1, ..., e_t) with t <= L, add constraint sum_{e in C} x_e >= 1. Minimize sum x_e subject to sum x_e <= K. (Note: the number of cycle constraints can be exponential, so a lazy constraint generation / cutting plane approach is needed in practice.)
- [ ] Other: (none identified)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {1,2}, {2,0}, {2,3}, {3,4}, {4,2}, {3,5}, {5,4}, {0,3}
- Budget K = 3
- Max cycle length L = 4

**Cycles of length <= 4:**
- Length 3: (0,1,2) using edges {0,1},{1,2},{2,0}
- Length 3: (2,3,4) using edges {2,3},{3,4},{4,2}
- Length 3: (3,4,5) using edges {3,4},{4,5},{5,3} -- wait, edge {5,4} and {3,5}
  Actually: (3,5,4) using edges {3,5},{5,4},{4,3} = {3,5},{4,5},{3,4}
- Length 4: (0,1,2,3) using {0,1},{1,2},{2,3},{0,3}
- Length 4: (0,2,3,4) -- not a simple path since we need {0,2} which is the same as {2,0}
  (0,2,4,3) using {0,2},{2,4},{4,3},{3,0} = {0,2},{2,4},{3,4},{0,3}
- Length 4: (2,3,5,4) using {2,3},{3,5},{5,4},{4,2}

**Partial feedback edge set E' = {{2,0}, {3,4}, {0,3}} (size 3 = K):**
- Triangle (0,1,2): edge {2,0} in E' -- hit
- Triangle (2,3,4): edge {3,4} in E' -- hit
- Triangle (3,5,4): edge {3,4} in E' -- hit
- 4-cycle (0,1,2,3): edge {0,3} in E' -- hit (also {2,0})
- 4-cycle (0,2,4,3): edge {0,3} in E' -- hit (also {2,0} and {3,4})
- 4-cycle (2,3,5,4): edge {3,4} in E' -- hit

All cycles of length <= 4 are hit. |E'| = 3 <= K = 3. Answer: YES.
