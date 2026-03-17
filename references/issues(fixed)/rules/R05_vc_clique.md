---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to CLIQUE"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** CLIQUE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Lemma 3.1, p.54

## Reduction Algorithm

> Lemma 3.1 For any graph G = (V,E) and subset V' ⊆ V, the following statements are equivalent:
>
> (a) V' is a vertex cover for G.
> (b) V-V' is an independent set for G.
> (c) V-V' is a clique in the complement G^c of G, where G^c = (V,E^c) with E^c = {{u,v}: u,v E V and {u,v} not-E E}.
>
> Thus we see that, in a rather strong sense, these three problems might be regarded simply as "different versions" of one another. Furthermore, the relationships displayed in the lemma make it a trivial matter to transform any one of the problems to either of the others.
>
> For example, to transform VERTEX COVER to CLIQUE, let G = (V,E) and K <= |V| constitute any instance of VC. The corresponding instance of CLIQUE is provided simply by the graph G^c and the integer J = |V|-K.
>
> This implies that the NP-completeness of all three problems will follow as an immediate consequence of proving that any one of them is NP-complete. We choose to prove this for VERTEX COVER.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
