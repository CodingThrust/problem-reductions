---
name: Problem
about: Propose a new problem type
title: "[Model] SubgraphIsomorphism"
labels: model
assignees: ''
---

## Motivation

SUBGRAPH ISOMORPHISM (P59) from Garey & Johnson, A1.4 GT48. A classical NP-complete problem useful for reductions. It asks whether a "pattern" graph H can be found embedded within a "host" graph G as a subgraph. This is strictly more general than CLIQUE (which is the special case where H = K_k), and contains HAMILTONIAN CIRCUIT, HAMILTONIAN PATH, and COMPLETE BIPARTITE SUBGRAPH as further special cases.

## Definition

**Name:** <!-- ⚠️ Unverified --> `SubgraphIsomorphism`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Subgraph Isomorphism Problem
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT48

**Mathematical definition:**

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2).
QUESTION: Does G contain a subgraph isomorphic to H, i.e., a subset V ⊆ V_1 and a subset E ⊆ E_1 such that |V| = |V_2|, |E| = |E_2|, and there exists a one-to-one function f: V_2 → V satisfying {u,v} ∈ E_2 if and only if {f(u),f(v)} ∈ E?

The problem is a decision (satisfaction) problem: we ask whether such an injective homomorphism preserving edge structure exists.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |V_2| (one variable per pattern-graph vertex, choosing which host-graph vertex to map it to)
- **Per-variable domain:** {0, 1, ..., |V_1| - 1} — each pattern vertex maps to one host vertex (an assignment must be injective)
- **Meaning:** variable x_i ∈ {0, ..., n-1} represents the host vertex that pattern vertex i is mapped to. The assignment (x_0, ..., x_{|V_2|-1}) encodes a candidate injective mapping f: V_2 → V_1. It is valid if (a) all x_i are distinct (injective), and (b) for every edge {u,v} ∈ E_2, we have {x_u, x_v} ∈ E_1.

Note: For brute-force solving, the configuration space is the set of all injective functions f: V_2 → V_1, which has size |V_1|! / (|V_1| - |V_2|)! entries.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SubgraphIsomorphism`
**Variants:** graph topology (SimpleGraph for both host and pattern)

| Field | Type | Description |
|-------|------|-------------|
| `host_graph` | `SimpleGraph` | The graph G = (V_1, E_1) to search in (the larger graph) |
| `pattern_graph` | `SimpleGraph` | The graph H = (V_2, E_2) to find as a subgraph (the smaller/pattern graph) |

The problem has no weight parameters since it is a pure structural (decision) problem.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Cook, 1971; transformation from CLIQUE).
- **Best known exact algorithm:** Brute-force enumeration of all injective mappings f: V_2 → V_1, in O(|V_1|^{|V_2|} · |E_2|) time. For the case where the pattern graph H is fixed, the problem is solvable in polynomial time O(n^{|V_2|}). For general H, the color-coding technique of Alon, Yuster, and Zwick (1995) gives a randomized algorithm running in time 2^{O(|V_2|)} · |V_1|^{O(tw(H))}, where tw(H) is the treewidth of the pattern graph.
- **Practical algorithms:** The VF2 algorithm (Cordella et al., 2004) and its improvements VF2++ (Jüttner and Madarasi, 2018) are state-of-the-art backtracking algorithms with exponential worst-case complexity but good practical performance.
- **References:**
  - [Cook, 1971] S. A. Cook, "The Complexity of Theorem Proving Procedures", *STOC 1971*, pp. 151–158. Original NP-completeness framework.
  - [Alon, Yuster, Zwick, 1995] N. Alon, R. Yuster, U. Zwick, "Color-coding", *Journal of the ACM* 42(4), pp. 844–856. Color-coding technique for fixed-parameter algorithms.
  - [Cordella et al., 2004] L. P. Cordella, P. Foggia, C. Sansone, M. Vento, "A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs", *IEEE TPAMI* 26(10), pp. 1367–1372. VF2 algorithm.
  - [Jüttner and Madarasi, 2018] A. Jüttner and P. Madarasi, "VF2++: An Improved Subgraph Isomorphism Algorithm", *Discrete Applied Mathematics* 242, pp. 69–81. VF2++ improvement.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a generalization of:** CLIQUE (special case where H = K_k, the complete graph on k vertices)
- **Known special cases:** CLIQUE, HAMILTONIAN CIRCUIT (H = cycle C_n), HAMILTONIAN PATH (H = path P_n), COMPLETE BIPARTITE SUBGRAPH
- **Polynomial cases:** Solvable in polynomial time if G is a forest and H is a tree [Edmonds and Matula, 1975]; remains NP-complete if G is a tree and H is a forest (see GJ Chapter 4).

## Extra Remark

**Full book text:**

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2).
QUESTION: Does G contain a subgraph isomorphic to H, i.e., a subset V ⊆ V_1 and a subset E ⊆ E_1 such that |V| = |V_2|, |E| = |E_2|, and there exists a one-to-one function f: V_2 → V satisfying {u,v} ∈ E_2 if and only if {f(u),f(v)} ∈ E?

Reference: [Cook, 1971a]. Transformation from CLIQUE.
Comment: Contains CLIQUE, COMPLETE BIPARTITE SUBGRAPH, HAMILTONIAN CIRCUIT, etc., as special cases. Can be solved in polynomial time if G is a forest and H is a tree [Edmonds and Matula, 1975] (see also [Reyner, 1977]), but remains NP-complete if G is a tree and H is a forest (see Chapter 4) or if G is a graph and H is a tree (HAMILTONIAN PATH). Variant for directed graphs is also NP-complete, even if G is acyclic and H is a directed tree [Aho and Sethi, 1977], but can be solved in polynomial time if G is a directed forest and H is a directed tree [Reyner, 1977]. If |V_1| = |V_2| and |E_1| = |E_2| we have the GRAPH ISOMORPHISM problem, which is open for both directed and undirected graphs.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all injective functions f: V_2 → V_1; for each f, check that every edge {u,v} ∈ E_2 maps to an edge {f(u),f(v)} ∈ E_1. This has |V_1|!/(|V_1|-|V_2|)! candidates.
- [ ] It can be solved by reducing to integer programming. Introduce binary variables x_{i,j} = 1 if pattern vertex i maps to host vertex j; enforce injectivity and edge-preservation constraints. Integer program has |V_1|·|V_2| variables and O(|V_1|·|V_2| + |E_2|·|V_1|^2) constraints.
- [ ] Other: (none identified)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Host graph G = (V_1, E_1):**
Vertices V_1 = {0, 1, 2, 3, 4, 5, 6} (7 vertices)
Edges E_1 = {0,1}, {0,2}, {0,3}, {1,2}, {1,3}, {2,3}, {3,4}, {4,5}, {4,6}, {5,6} (10 edges)
- Vertices {0,1,2,3} form K_4 (a 4-clique)
- Vertices {4,5,6} form K_3 (a triangle), connected to {0,1,2,3} via vertex 3→4

**Pattern graph H = (V_2, E_2):**
Vertices V_2 = {a, b, c, d} (4 vertices — we search for K_4)
Edges E_2 = {a,b}, {a,c}, {a,d}, {b,c}, {b,d}, {c,d} (6 edges — complete graph K_4)

**Greedy trap:** The triangle {4,5,6} looks like a good starting point but has only 3 vertices and cannot accommodate the 4-vertex pattern. The isomorphism must use the 4-clique {0,1,2,3}.

**Solution:**
Injective mapping f: V_2 → V_1 with f(a)=0, f(b)=1, f(c)=2, f(d)=3
- Edge {a,b} → {0,1} ∈ E_1 ✓
- Edge {a,c} → {0,2} ∈ E_1 ✓
- Edge {a,d} → {0,3} ∈ E_1 ✓
- Edge {b,c} → {1,2} ∈ E_1 ✓
- Edge {b,d} → {1,3} ∈ E_1 ✓
- Edge {c,d} → {2,3} ∈ E_1 ✓

All 6 pattern edges are preserved under f. Subgraph isomorphism exists: YES ✓

**Verification of non-existence for larger pattern:**
If we instead search for K_5 (pattern with 5 vertices and 10 edges), no isomorphism exists because the maximum clique in G has size 4.
