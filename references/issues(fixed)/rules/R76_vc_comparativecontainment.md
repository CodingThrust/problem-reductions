---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to COMPARATIVE CONTAINMENT"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** COMPARATIVE CONTAINMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP10, p.223

## GJ Source Entry

> [SP10] COMPARATIVE CONTAINMENT
> INSTANCE: Two collections R={R_1,R_2,…,R_k} and S={S_1,S_2,…,S_l} of subsets of a finite set X, weights w(R_i)∈Z^+, 1≤i≤k, and w(S_j)∈Z^+, 1≤j≤l.
> QUESTION: Is there a subset Y⊆X such that
> Σ_{Y⊆R_i} w(R_i) ≥ Σ_{Y⊆S_j} w(S_j) ?
> Reference: [Plaisted, 1976]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if all subsets in R and S have weight 1 [Garey and Johnson, ——].

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264–267. IEEE Computer Society.
- **[Garey and Johnson, ——]**: *(not found in bibliography)*