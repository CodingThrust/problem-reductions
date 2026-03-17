---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GRAPH 3-COLORABILITY to REDUCTION OF INCOMPLETELY SPECIFIED AUTOMATA"
labels: rule
assignees: ''
---

**Source:** GRAPH 3-COLORABILITY
**Target:** REDUCTION OF INCOMPLETELY SPECIFIED AUTOMATA
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL7

## Reduction Algorithm

> INSTANCE: An incompletely specified deterministic finite state automaton A = (Q,Σ,δ,q₀,F), where Q is the set of states, Σ is the input alphabet, δ is a "partial" transition function mapping a subset of Q×Σ into Q, q₀∈Q is the initial state, and F⊆Q is the set of "accept" states, and a positive integer K.
> QUESTION: Can the transition function δ be extended to a total function from Q×Σ into Q in such a way that the resulting completely specified automaton has an equivalent "reduced automaton" with K or fewer states?
> Reference: [Pfleeger, 1973]. Transformation from GRAPH 3-COLORABILITY.
> Comment: Remains NP-complete for any fixed K≥6. Related question in which "state-splitting" (as used in [Paull and Unger, 1959]) is allowed is also NP-complete for any fixed K≥6 [Pfleeger, 1973]. If both "state-splitting" and "symbol-splitting" (as used in [Grasselli and Luccio, 1966]) are allowed, the analogous problem in which the corresponding reduced automaton is to have the sum of the number of states and the number of symbols be no more than K is also NP-complete [Pfleeger, 1974]. The problem of determining the minimum state deterministic finite state automaton equivalent to a given completely specified one can be solved in polynomial time (e.g., see [Hopcroft, 1971] or [Aho and Ullman, 1972]). The corresponding problem for completely specified nondeterministic finite state automata is PSPACE-complete (see FINITE STATE AUTOMATA INEQUIVALENCE).

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Pfleeger, 1973]**: [`Pfleeger1973`] C. F. Pfleeger (1973). "State reduction in incompletely specified finite-state machines". *IEEE Transactions on Computers* C-22, pp. 1099–1102.
- **[Paull and Unger, 1959]**: [`Paull1959`] M. Paull and S. Unger (1959). "Minimizing the number of states in incompletely specified sequential switching functions". *IRE Transactions on Electronic Computers* EC-8, pp. 356–367.
- **[Grasselli and Luccio, 1966]**: [`Grasselli1966`] A. Grasselli and F. Luccio (1966). "A method for the combined row-column reduction of flow tables". In: *Proceedings of the 7th Annual Symposium on Switching and Automata Theory*, pp. 136–147. IEEE Computer Society.
- **[Pfleeger, 1974]**: [`Pfleeger1974`] C. F. Pfleeger (1974). "Complete Sets and Time and Space Bounded Computation". Pennsylvania State University.
- **[Hopcroft, 1971]**: [`Hopcroft1971`] J. E. Hopcroft (1971). "An $n \log n$ algorithm for minimizing states in a finite automaton". In: *Theory of Machines and Computations*. Academic Press.
- **[Aho and Ullman, 1972]**: [`Aho1972b`] A. V. Aho and J. D. Ullman (1972). "The Theory of Parsing, Translation, and Compiling --- Volume 1: Parsing". Prentice-Hall, Inc., Englewood Cliffs, NJ.