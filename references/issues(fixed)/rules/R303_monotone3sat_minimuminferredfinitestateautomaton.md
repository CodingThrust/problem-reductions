---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MONOTONE 3SAT to MINIMUM INFERRED FINITE STATE AUTOMATON"
labels: rule
assignees: ''
---

**Source:** MONOTONE 3SAT
**Target:** MINIMUM INFERRED FINITE STATE AUTOMATON
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL8

## Reduction Algorithm

> INSTANCE: Finite alphabet Σ, two finite subsets S, T ⊆ Σ*, positive integer K.
> QUESTION: Is there a K-state deterministic finite automaton A that recognizes a language L ⊆ Σ* such that S ⊆ L and T ⊆ Σ*−L?
> Reference: [Gold, 1974]. Transformation from MONOTONE 3SAT.
> Comment: Can be solved in polynomial time if S ∪ T = Σⁿ for some n, where Σⁿ is the set of all strings of length n or less over Σ [Trakhtenbrot and Barzdin, 1973]. However, for any fixed ε>0, the problem remains NP-complete if restricted to instances for which (S ∪ T) ⊆ Σⁿ and |Σⁿ−(S ∪ T)| ≤ |Σⁿ|ᶜ [Angluin, 1977].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Gold, 1974]**: [`Gold1974`] E. M. Gold (1974). "Complexity of automaton identification from given data".
- **[Trakhtenbrot and Barzdin, 1973]**: [`Trakhtenbrot and Barzdin1973`] Boris A. Trakhtenbrot and Ya. M. Barzdin (1973). "Finite Automata". North-Holland, Amsterdam.
- **[Angluin, 1977]**: [`Angluin1977a`] D. Angluin (1977). "On the complexity of minimum inference of regular sets".