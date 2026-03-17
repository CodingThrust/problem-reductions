---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to PLANAR SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN PATH
**Target:** PLANAR SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT27

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that G' = (V,E') is planar?
>
> Reference: [Liu and Geldmacher, 1978]. Transformation from HAMILTONIAN PATH restricted to bipartite graphs.
> Comment: Corresponding problem in which G' is the subgraph induced by a set V' of at least K vertices is also NP-complete [Krishnamoorthy and Deo, 1977a], [Yannakakis, 1978b]. The former can be solved in polynomial time when K = |E|, and the latter when K = |V|, since planarity testing can be done in polynomial time (e.g., see [Hopcroft and Tarjan, 1974]). The related problem in which we ask if G contains a connected "outerplanar" subgraph with K or more edges is also NP-complete [Yannakakis, 1978b].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Liu and Geldmacher, 1978]**: [`Liu1978`] P. C. Liu and R. C. Geldmacher (1978). "On the deletion of nonplanar edges of a graph". *SIAM Journal on Computing*.
- **[Krishnamoorthy and Deo, 1977a]**: [`Krishnamoorthy1977a`] M. S. Krishnamoorthy and N. Deo (1977). "Node deletion {NP}-complete problems". Computer Centre, Indian Institute of Technology.
- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.
- **[Hopcroft and Tarjan, 1974]**: [`Hopcroft1974`] J. E. Hopcroft and R. E. Tarjan (1974). "Efficient planarity testing". *Journal of the Association for Computing Machinery* 21, pp. 549–568.