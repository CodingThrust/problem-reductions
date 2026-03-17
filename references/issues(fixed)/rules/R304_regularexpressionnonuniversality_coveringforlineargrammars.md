---
name: Rule
about: Propose a new reduction rule
title: "[Rule] REGULAR EXPRESSION NON-UNIVERSALITY to COVERING FOR LINEAR GRAMMARS"
labels: rule
assignees: ''
---

**Source:** REGULAR EXPRESSION NON-UNIVERSALITY
**Target:** COVERING FOR LINEAR GRAMMARS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL12

## Reduction Algorithm

> INSTANCE: Two linear context-free grammars G₁ = (N₁,Σ,Π₁,S₁) and G₂ = (N₂,Σ,Π₂,S₂), where no production in such a grammar is allowed to have more than one nonterminal symbol on its right hand side.
> QUESTION: Is there a function h:P₁→P₂∪{λ} (where λ denotes the empty production) such that G₁ covers G₂ under h, i.e., such that for all strings w∈Σ* (1) if w is derivable from S₁ under the sequence of productions p₁,p₂,…,pₙ, then w is derivable from S₂ under the sequence h(p₁),h(p₂),…,h(pₙ), and (2) if w is derivable from S₂ under the sequence of productions q₁,q₂,…,qₙ from Π₂, then there exists a sequence of productions p₁,p₂,…,pₘ that is a derivation of w in G₁ such that h(p₁),h(p₂),…,h(pₘ) equals q₁,q₂,…,qₙ?
> Reference: [Hunt, Rosenkrantz, and Szymanski, 1976a], [Hunt, Rosenkrantz, and Szymanski, 1976b]. Transformation from REGULAR EXPRESSION NON-UNIVERSALITY. The second reference proves membership in PSPACE.
> Comment: PSPACE-complete, even for "regular" grammars. Undecidable for arbitrary context-free grammars. See [Hunt and Rosenkrantz, 1977] for related results.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Hunt, Rosenkrantz, and Szymanski, 1976a]**: [`Hunt1976b`] Harry B. Hunt III and Daniel J. Rosenkrantz and Thomas G. Szymanski (1976). "On the equivalence, containment, and covering problems for the regular and context-free languages". *Journal of Computer and System Sciences* 12, pp. 222–268.
- **[Hunt, Rosenkrantz, and Szymanski, 1976b]**: [`Hunt1976b`] Harry B. Hunt III and Daniel J. Rosenkrantz and Thomas G. Szymanski (1976). "On the equivalence, containment, and covering problems for the regular and context-free languages". *Journal of Computer and System Sciences* 12, pp. 222–268.
- **[Hunt and Rosenkrantz, 1977]**: [`Hunt1977b`] Harry B. Hunt III and Daniel J. Rosenkrantz (1977). "Complexity of grammatical similarity relations: preliminary report". In: *Proceedings of the Conference on Theoretical Computer Science*, pp. 139–148. Dept. of Computer Science, University of Waterloo.