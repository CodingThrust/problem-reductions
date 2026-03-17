---
name: Rule
about: Propose a new reduction rule
title: "[Rule] REGULAR EXPRESSION NON-UNIVERSALITY to FINITE STATE AUTOMATON INEQUIVALENCE"
labels: rule
assignees: ''
---

**Source:** REGULAR EXPRESSION NON-UNIVERSALITY
**Target:** FINITE STATE AUTOMATON INEQUIVALENCE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A10.1, p.265

## GJ Source Entry

> [AL1] FINITE STATE AUTOMATON INEQUIVALENCE (*)
> INSTANCE: Two nondeterministic finite state automata A_1 and A_2 having the same input alphabet Σ (where such an automaton A = (Q,Σ,δ,q_0,F) consists of a finite set Q of states, input alphabet Σ, transition function δ mapping Q×Σ into subsets of Q, initial state q_0, and a set F ⊆ K of "accept" states, e.g., see [Hopcroft and Ullman, 1969]).
> QUESTION: Do A_1 and A_2 recognize different languages?
> Reference: [Kleene, 1956]. Transformation from REGULAR EXPRESSION NON-UNIVERSALITY.
> Comment: PSPACE-complete, even if |Σ| = 2 and A_2 is the trivial automaton recognizing Σ*. The general problem is NP-complete if |Σ| = 1, or if A_1 and A_2 both recognize finite languages (a property that can be checked in polynomial time, e.g., see [Hopcroft and Ullman, 1969]). Problem is solvable in polynomial time if A_1 and A_2 are deterministic finite state automata, e.g., see [Hopcroft and Ullman, 1969].

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

- **[Hopcroft and Ullman, 1969]**: [`Hopcroft1969`] John E. Hopcroft and Jeffrey D. Ullman (1969). "Formal Languages and their Relation to Automata". Addison-Wesley, Reading, MA.
- **[Kleene, 1956]**: [`Kleene1956`] Stephen C. Kleene (1956). "Representation of events in nerve nets and finite automata". In: *Automata Studies*. Princeton University Press.