---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to MAXIMUM SUBGRAPH MATCHING"
labels: rule
assignees: ''
---

**Source:** CLIQUE
**Target:** MAXIMUM SUBGRAPH MATCHING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT50

## Reduction Algorithm

> INSTANCE: Directed graphs G = (V1,A1), H = (V2,A2), positive integer K.
> QUESTION: Is there a subset R ⊆ V1×V2 with |R| ≥ K such that, for all <u,u'>,<v,v'> ∈ R, (u,v) ∈ A1 if and only if (u',v') ∈ A2?
>
> Reference: [Garey and Johnson, ——]. Transformation from CLIQUE. Problem is discussed in [Barrow and Burstall, 1976].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Barrow and Burstall, 1976]**: [`Barrow1976`] H. G. Barrow and R. M. Burstall (1976). "Subgraph isomorphism, matching relational structures and maximal cliques". *Information Processing Letters* 4, pp. 83–84.