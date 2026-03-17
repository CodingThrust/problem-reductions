---
name: Rule
about: Propose a new reduction rule
title: "[Rule] (generic transformation) to LINEAR BOUNDED AUTOMATON ACCEPTANCE"
labels: rule
assignees: ''
---

**Source:** (generic transformation)
**Target:** LINEAR BOUNDED AUTOMATON ACCEPTANCE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A10.1, p.265

## GJ Source Entry

> [AL3] LINEAR BOUNDED AUTOMATON ACCEPTANCE (*)
> INSTANCE: A "linear bounded automaton" A with input alphabet Σ (see [Hopcroft and Ullman, 1969] for definition), and a string x E Σ*.
> QUESTION: Does A accept x?
> Reference: [Karp, 1972]. Generic transformation.
> Comment: PSPACE-complete, even if A is deterministic (the LINEAR SPACE ACCEPTANCE problem of Section 7.4). Moreover, there exist fixed deterministic linear bounded automata for which the problem is PSPACE-complete.

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
- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.