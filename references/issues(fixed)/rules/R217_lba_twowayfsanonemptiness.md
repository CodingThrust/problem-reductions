---
name: Rule
about: Propose a new reduction rule
title: "[Rule] LINEAR BOUNDED AUTOMATON ACCEPTANCE to TWO-WAY FINITE STATE AUTOMATON NON-EMPTINESS"
labels: rule
assignees: ''
---

**Source:** LINEAR BOUNDED AUTOMATON ACCEPTANCE
**Target:** TWO-WAY FINITE STATE AUTOMATON NON-EMPTINESS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A10.1, p.265

## GJ Source Entry

> [AL2] TWO-WAY FINITE STATE AUTOMATON NON-EMPTINESS (*)
> INSTANCE: A two-way nondeterministic finite state automaton A = (Q,Σ,δ,q_0,F) (where Q, Σ, q_0, and F are the same as for a one-way nondeterministic finite state automaton, but the transition function δ maps Q×Σ into subsets of Q×{-1,0,1}, e.g., see [Hopcroft and Ullman, 1969]).
> QUESTION: Is there an x E Σ* such that A accepts x?
> Reference: [Hunt, 1973b]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
> Comment: PSPACE-complete, even if |Σ| = 2 and A is deterministic. If |Σ| = 1 the general problem is NP-complete [Galil, 1976]. If A is a one-way nondeterministic finite state automaton, the general problem can be solved in polynomial time (e.g., see [Hopcroft and Ullman, 1969]). Analogous results for the question of whether A recognizes an infinite language can be found in the above references.

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
- **[Hunt, 1973b]**: [`Hunt1973b`] Harry B. Hunt III (1973). "On the time and tape complexity of languages {I}". In: *Proceedings of the 5th Annual ACM Symposium on Theory of Computing*, pp. 10–19. Association for Computing Machinery.
- **[Galil, 1976]**: [`Galil1976`] Z. Galil (1976). "Hierarchies of complete problems". *Acta Informatica* 6, pp. 77–88.