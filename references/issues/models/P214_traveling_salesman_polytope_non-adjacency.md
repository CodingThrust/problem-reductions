---
name: Problem
about: Propose a new problem type
title: "[Model] TravelingSalesmanPolytopeNonAdjacency"
labels: model
assignees: ''
---

## Motivation

TRAVELING SALESMAN POLYTOPE NON-ADJACENCY (P214) from Garey & Johnson, A6 MP8. This problem asks whether two given Hamiltonian circuits in a graph correspond to non-adjacent vertices of the traveling salesman polytope -- the convex hull of characteristic vectors of all Hamiltonian tours. Its NP-completeness (Papadimitriou 1978a) has deep implications for LP-based approaches to TSP: it means that no polynomial-time pivot rule for the simplex method can efficiently navigate the TSP polytope, since even determining the local neighborhood structure of a vertex is intractable. The result also holds for the non-symmetric (directed) case.

**Associated rules:**
- R159: Hamiltonian Circuit -> Traveling Salesman Polytope Non-Adjacency (Papadimitriou 1978a)

## Definition

**Name:** <!-- ⚠️ Unverified --> `TravelingSalesmanPolytopeNonAdjacency`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Traveling Salesman Polytope Non-Adjacency; also: TSP Polytope Vertex Non-Adjacency, Adjacency on the TSP Polytope
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP8

**Mathematical definition:**

INSTANCE: Graph G = (V, E), two Hamiltonian circuits C and C' for G.
QUESTION: Do C and C' correspond to non-adjacent vertices of the "traveling salesman polytope" for G?

**Background:** The traveling salesman polytope (also called the TSP polytope or Hamiltonian cycle polytope) for a graph with n vertices is the convex hull of the characteristic (0-1 incidence) vectors of all Hamiltonian tours in the complete graph K_n, projected to the edge set. Each Hamiltonian tour maps to a vertex of this polytope in R^{|E|} (or R^{n(n-1)/2} for the complete graph). Two vertices of the polytope are *adjacent* if they are connected by an edge (1-face) of the polytope, i.e., there is no other vertex of the polytope that lies on the line segment between them in a proper geometric sense. Equivalently, two tours are adjacent on the polytope if and only if their characteristic vectors cannot be written as a convex combination of characteristic vectors of other tours.

A sufficient condition for non-adjacency: if the symmetric difference of the edge sets of C and C' can be decomposed into two other Hamiltonian tours, then C and C' are non-adjacent (since the midpoint of their characteristic vectors is a convex combination of the other two tours).

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** This is a decision (satisfaction) problem. The "witness" for non-adjacency is a pair of Hamiltonian tours T1, T2 such that the characteristic vector of C can be written as a strict convex combination involving T1, T2, and C'. In the simplest case, we need to find two Hamiltonian tours whose edge sets form a decomposition of the symmetric difference of C and C'. Equivalently, the variable space is all possible Hamiltonian tour pairs.
- **Per-variable domain:** Each candidate witness tour is a permutation of n vertices.
- **Meaning:** The decision asks whether the two given tours are NOT on a common edge (1-face) of the TSP polytope. This is equivalent to asking whether there exist other tours that "separate" C from C' in the polytope geometry.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `TravelingSalesmanPolytopeNonAdjacency`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `circuit1` | `Vec<usize>` | First Hamiltonian circuit C (as ordered vertex sequence) |
| `circuit2` | `Vec<usize>` | Second Hamiltonian circuit C' (as ordered vertex sequence) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Both circuits C and C' must be valid Hamiltonian circuits in G (this is a precondition, not checked by the problem).
- The two circuits must be distinct (otherwise they trivially correspond to the same polytope vertex).
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|).
- The problem also makes sense for directed graphs (asymmetric TSP polytope), which GJ notes is also NP-complete.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Papadimitriou, 1978a; transformation from 3SAT per GJ, though the paper uses a construction related to Hamiltonian circuits).
- **Best known exact algorithm:** No dedicated sub-exponential algorithm is known. Brute-force approach: enumerate candidate tour pairs and check if they witness non-adjacency. The number of Hamiltonian tours can be exponential, so this is O*((n-1)!^2) in the worst case. Checking adjacency of two given polytope vertices is co-NP-complete (checking adjacency is the complement problem).
- **Sufficient condition (polynomial-time checkable):** If the multigraph formed by the symmetric difference of C and C' (a 4-regular multigraph on the vertices where C and C' differ) admits a Hamiltonian decomposition into two tours, then C and C' are non-adjacent. However, finding such a decomposition is itself a hard problem in general.
- **Polynomial-time special cases:** For matching polytopes and clique polytopes, analogous adjacency problems can be solved in polynomial time (Chvatal, 1975).
- **References:**
  - C.H. Papadimitriou (1978). "The adjacency relation on the traveling salesman polytope is NP-Complete." *Mathematical Programming* 14, pp. 312-324.
  - V. Chvatal (1975). "On certain polytopes associated with graphs." *Journal of Combinatorial Theory (B)* 18, pp. 138-154. Polynomial adjacency for matching/clique polytopes.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** General polytope vertex adjacency (which is co-NP-complete for 0/1-polytopes in general).
- **Known special cases:** For small n (say n <= 8), the TSP polytope can be fully enumerated. For specific graph structures (e.g., complete bipartite), special adjacency criteria may apply.
- **Related problems:** TSP, Hamiltonian Circuit, Polytope Diameter (the diameter of the TSP polytope is known to be bounded by a constant times n).

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), two Hamiltonian circuits C and C' for G.
QUESTION: Do C and C' correspond to non-adjacent vertices of the "traveling salesman polytope" for G?

Reference: [Papadimitriou, 1978a]. Transformation from 3SAT.
Comment: Result also holds for the "non-symmetric" case where G is a directed graph and C and C' are directed Hamiltonian circuits. Analogous polytope non-adjacency problems for graph matching and CLIQUE can be solved in polynomial time [Chvatal, 1975].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all Hamiltonian tours in G; check if the characteristic vectors of C and C' lie on a common edge of the convex hull. Equivalently, check if there exist tours T1, T2 distinct from C, C' such that (chi_C + chi_{C'}) / 2 = (chi_{T1} + chi_{T2}) / 2, where chi denotes the characteristic vector. This witnesses non-adjacency. Time: O*((n-1)! * poly(n)).
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (non-adjacent -- answer YES):**

Graph G = K_6 (complete graph on 6 vertices {0, 1, 2, 3, 4, 5}).

Circuit C: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0
  Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}

Circuit C': 0 -> 2 -> 4 -> 1 -> 3 -> 5 -> 0
  Edges: {0,2}, {2,4}, {4,1}, {1,3}, {3,5}, {5,0}

Symmetric difference of edge sets:
  In C but not C': {0,1}, {1,2}, {2,3}, {3,4}, {4,5}
  In C' but not C: {0,2}, {2,4}, {4,1}, {1,3}, {3,5}
  Common: {5,0}

The union of the symmetric difference edges forms a 4-regular multigraph on {0,1,2,3,4,5} (ignoring the common edge). If this multigraph can be decomposed into two Hamiltonian tours T1 and T2, then C and C' are non-adjacent.

Consider:
  T1: 0 -> 1 -> 3 -> 2 -> 4 -> 5 -> 0  (edges: {0,1}, {1,3}, {3,2}, {2,4}, {4,5}, {5,0})
  T2: 0 -> 2 -> 1 -> 4 -> 3 -> 5 -> 0  (edges: {0,2}, {2,1}, {1,4}, {4,3}, {3,5}, {5,0})

Check: chi_C + chi_{C'} = chi_{T1} + chi_{T2} (each edge in the symmetric difference appears once in T1 or T2; common edge {5,0} appears in both T1 and T2 as well as both C and C').

Since (chi_C + chi_{C'})/2 = (chi_{T1} + chi_{T2})/2, the midpoint of C and C' is also the midpoint of T1 and T2, proving C and C' are non-adjacent.
Answer: YES (non-adjacent).

**Instance 2 (adjacent -- answer NO):**

Graph G = K_4 (complete graph on 4 vertices {0, 1, 2, 3}).

Circuit C: 0 -> 1 -> 2 -> 3 -> 0
  Edges: {0,1}, {1,2}, {2,3}, {3,0}

Circuit C': 0 -> 1 -> 3 -> 2 -> 0
  Edges: {0,1}, {1,3}, {3,2}, {2,0}

K_4 has exactly 3 distinct Hamiltonian circuits (up to direction):
  C1: 0-1-2-3-0, C2: 0-1-3-2-0, C3: 0-2-1-3-0

Since C = C1 and C' = C2, the only other tour is C3 = {0,2}, {2,1}, {1,3}, {3,0}.
No convex combination of other tour(s) equals the midpoint of C and C', because there is only one other tour. Two vertices of a polytope are adjacent iff the face they span is an edge (1-dimensional). With only 3 vertices total in the TSP polytope for K_4, every pair of vertices is adjacent (the polytope is a triangle).
Answer: NO (they are adjacent, not non-adjacent).
