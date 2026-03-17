---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to REYNOLDS COVERING FOR CONTEXT-FREE GRAMMARS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** REYNOLDS COVERING FOR CONTEXT-FREE GRAMMARS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL11

## Reduction Algorithm

> INSTANCE: Context-free grammars G₁ = (N₁,Σ,Π₁,S₁) and G₂ = (N₂,Σ,Π₂,S₂), where Σ is a finite set of "terminal" symbols, Nᵢ is a finite set of "nonterminal" symbols, Sᵢ∈Nᵢ is the "initial" symbol, and Πᵢ is a set of "productions" of the form "A→w," where A∈Nᵢ and w∈(Nᵢ∪Σ)*.
> QUESTION: Does G₂ "Reynolds cover" G₁, i.e., is there a function f mapping N₁∪Σ into N₂∪Σ such that f(x)=x for all x∈Σ, f(A)∈N₂ for all A∈N₁, f(S₁)=S₂, and for each production A→x₁x₂⋯ xₙ in Π₁, the image f(A)→f(x₁)f(x₂)⋯ f(xₙ) of that production is in Π₂?
> Reference: [Hunt and Rosenkrantz, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete even if G₁ and G₂ are restricted to "regular" grammars. The same results hold for the related questions of whether G₂ "weakly Reynolds covers" G₁ or whether G₂ is a "homomorphic image" of G₁. The problem "Given G is there an LL(k) context-free grammar H such that H Reynolds covers G?" is solvable in polynomial time, as are the related problems where LL(k) is replaced by LR(k) or one of a number of other grammar classes (see [Hunt and Rosenkrantz, 1977]).

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Hunt and Rosenkrantz, 1977]**: [`Hunt1977b`] Harry B. Hunt III and Daniel J. Rosenkrantz (1977). "Complexity of grammatical similarity relations: preliminary report". In: *Proceedings of the Conference on Theoretical Computer Science*, pp. 139–148. Dept. of Computer Science, University of Waterloo.