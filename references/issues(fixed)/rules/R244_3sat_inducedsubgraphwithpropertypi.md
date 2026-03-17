---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INDUCED SUBGRAPH WITH PROPERTY Π (*)"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** INDUCED SUBGRAPH WITH PROPERTY Π (*)
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT21

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' has property Π (see comments for possible choices for Π)?
>
> Reference: [Yannakakis, 1978a], [Yannakakis, 1978b], [Lewis, 1978]. Transformation from 3SAT.
> Comment: NP-hard for any property Π that holds for arbitrarily large graphs, does not hold for all graphs, and is "hereditary," i.e., holds for all induced subgraphs of G whenever it holds for G. If in addition one can determine in polynomial time whether Π holds for a graph, then the problem is NP-complete. Examples of such properties Π include "G is a clique," "G is an independent set," "G is planar," "G is bipartite," "G is outerplanar," "G is an edge graph," "G is chordal," "G is a comparability graph," and "G is a forest." The same general results hold if G is restricted to planar graphs and Π satisfies the above constraints for planar graphs, or if G is restricted to acyclic directed graphs and Π satisfies the above constraints for such graphs. A weaker result holds when G is restricted to bipartite graphs [Yannakakis, 1978b].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Yannakakis, 1978a]**: [`Yannakakis1978a`] Mihalis Yannakakis (1978). "The node deletion problem for hereditary properties". Computer Science Laboratory, Princeton University.
- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.
- **[Lewis, 1978]**: [`Lewis1978a`] Harry R. Lewis (1978). "Satisfiability problems for propositional calculi".