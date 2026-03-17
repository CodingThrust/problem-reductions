---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MULTIPLE CHOICE MATCHING"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MULTIPLE CHOICE MATCHING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT55

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), partition of E into disjoint sets E1,E2, . . . ,EJ, positive integer K.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that no two edges in E' share a common vertex and such that E' contains at most one edge from each Ei, 1 ≤ i ≤ J?
>
> Reference: [Valiant, 1977c], [Itai and Rodeh, 1977a], [Itai, Rodeh, and Tanimota, 1978]. Transformation from 3SAT.
> Comment: Remains NP-complete even if G is bipartite, each Ei contains at most 2 edges, and K = |V|/2. If each Ei contains only a single edge, this becomes the ordinary graph matching problem and is solvable in polynomial time.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Valiant, 1977c]**: [`Valiant1977c`] Leslie G. Valiant (1977). "private communication".
- **[Itai and Rodeh, 1977a]**: [`Itai1977c`] Alon Itai and Michael Rodeh (1977). "Some matching problems". In: *Automata, Languages, and Programming*. Springer.
- **[Itai, Rodeh, and Tanimota, 1978]**: [`Itai1978`] Alon Itai and Michael Rodeh and Shmuel L. Tanimota (1978). "Some matching problems for bipartite graphs". *Journal of the Association for Computing Machinery*.