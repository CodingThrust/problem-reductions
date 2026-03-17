---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INDUCED CONNECTED SUBGRAPH WITH PROPERTY Π (*)"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** INDUCED CONNECTED SUBGRAPH WITH PROPERTY Π (*)
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT22

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' is connected and has property Π (see comments for possible choices for Π)?
>
> Reference: [Yannakakis, 1978b]. Transformation from 3SAT.
> Comment: NP-hard for any hereditary property that holds for arbitrarily large connected graphs but not for all connected graphs. If, in addition, one can determine in polynomial time whether Π holds for a graph, then the problem is NP-complete. Examples include all the properties mentioned for the preceding problem except "G is an independent set". The related question "Is the maximum induced subgraph of G having property Π also connected?" is not in NP or co-NP unless NP = co-NP [Yannakakis, 1978b].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.