---
name: Rule
about: Propose a new reduction rule
title: "[Rule] FINITE STATE AUTOMATON INEQUIVALENCE to REGULAR GRAMMAR INEQUIVALENCE"
labels: rule
assignees: ''
---

**Source:** FINITE STATE AUTOMATON INEQUIVALENCE
**Target:** REGULAR GRAMMAR INEQUIVALENCE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL14

## Reduction Algorithm

> INSTANCE: Regular grammars G₁ = (N₁,Σ,Π₁,S₁) and G₂ = (N₂,Σ,Π₂,S₂), where a regular grammar is a context-free grammar in which each production has the form A→aB or A→a with A,B∈N and a∈Σ.
> QUESTION: Do G₁ and G₂ generate different languages?
> Reference: [Chomsky and Miller, 1958]. Transformation from FINITE STATE AUTOMATON INEQUIVALENCE.
> Comment: PSPACE-complete, even if |Σ|=2 and G₂ is a fixed grammar generating Σ* (REGULAR GRAMMAR NON-UNIVERSALITY). The general problem is NP-complete if |Σ|=1 or if both grammars generate finite languages (a property that can be checked in polynomial time, e.g., see [Hopcroft and Ullman, 1969]). If G₁ is allowed to be an arbitrary linear grammar and G₂ is a fixed grammar generating Σ* (LINEAR GRAMMAR NON-UNIVERSALITY), the problem is undecidable [Hunt, Rosenkrantz, and Szymanski, 1976a].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Chomsky and Miller, 1958]**: [`Chomsky1958`] N. Chomsky and G. A. Miller (1958). "Finite state languages". *Information and Control* 1, pp. 91–112.
- **[Hopcroft and Ullman, 1969]**: [`Hopcroft1969`] John E. Hopcroft and Jeffrey D. Ullman (1969). "Formal Languages and their Relation to Automata". Addison-Wesley, Reading, MA.
- **[Hunt, Rosenkrantz, and Szymanski, 1976a]**: [`Hunt1976b`] Harry B. Hunt III and Daniel J. Rosenkrantz and Thomas G. Szymanski (1976). "On the equivalence, containment, and covering problems for the regular and context-free languages". *Journal of Computer and System Sciences* 12, pp. 222–268.