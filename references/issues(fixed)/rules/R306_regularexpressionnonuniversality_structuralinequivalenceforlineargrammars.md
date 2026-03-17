---
name: Rule
about: Propose a new reduction rule
title: "[Rule] REGULAR EXPRESSION NON-UNIVERSALITY to STRUCTURAL INEQUIVALENCE FOR LINEAR GRAMMARS"
labels: rule
assignees: ''
---

**Source:** REGULAR EXPRESSION NON-UNIVERSALITY
**Target:** STRUCTURAL INEQUIVALENCE FOR LINEAR GRAMMARS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL13

## Reduction Algorithm

> INSTANCE: Two linear context-free grammars G₁ = (N₁,Σ,Π₁,S₁) and G₂ = (N₂,Σ,Π₂,S₂).
> QUESTION: Are G₁ and G₂ "structurally inequivalent," i.e., do the parenthesized grammars obtained from G₁ and G₂ by replacing each production A→w by A→(w) (where "(" and ")" are new terminal symbols) generate different languages?
> Reference: [Hunt, Rosenkrantz, and Szymanski, 1976a]. Transformation from REGULAR EXPRESSION NON-UNIVERSALITY.
> Comment: PSPACE-complete, even if G₁ and G₂ are regular and |Σ|=2. NP-complete if |Σ|=1. For arbitrary context-free grammars, problem is decidable but not known to be in PSPACE.

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