---
name: Problem
about: Propose a new problem type
title: "[Model] ReductionOfIncompletelySpecifiedAutomata"
labels: model
assignees: ''
---

## Motivation

REDUCTION OF INCOMPLETELY SPECIFIED AUTOMATA (P278) from Garey & Johnson, A10 AL7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL7

**Mathematical definition:**

INSTANCE: An incompletely specified deterministic finite state automaton A = (Q,Σ,δ,q_0,F), where Q is the set of states, Σ is the input alphabet, δ is a "partial" transition function mapping a subset of Q×Σ into Q, q_0 ∈ Q is the initial state, and F ⊆ Q is the set of "accept" states, and a positive integer K.
QUESTION: Can the transition function δ be extended to a total function from Q×Σ into Q in such a way that the resulting completely specified automaton has an equivalent "reduced automaton" with K or fewer states?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: An incompletely specified deterministic finite state automaton A = (Q,Σ,δ,q_0,F), where Q is the set of states, Σ is the input alphabet, δ is a "partial" transition function mapping a subset of Q×Σ into Q, q_0 ∈ Q is the initial state, and F ⊆ Q is the set of "accept" states, and a positive integer K.
QUESTION: Can the transition function δ be extended to a total function from Q×Σ into Q in such a way that the resulting completely specified automaton has an equivalent "reduced automaton" with K or fewer states?
Reference: [Pfleeger, 1973]. Transformation from GRAPH 3-COLORABILITY.
Comment: Remains NP-complete for any fixed K ≥ 6. Related question in which "state-splitting" (as used in [Paull and Unger, 1959]) is allowed is also NP-complete for any fixed K ≥ 6 [Pfleeger, 1973]. If both "state-splitting" and "symbol-splitting" (as used in [Grasselli and Luccio, 1966]) are allowed, the analogous problem in which the corresponding reduced automaton is to have the sum of the number of states and the number of symbols be no more than K is also NP-complete [Pfleeger, 1974]. The problem of determining the minimum state deterministic finite state automaton equivalent to a given completely specified one can be solved in polynomial time (e.g., see [Hopcroft, 1971] or [Aho and Ullman, 1972]). The corresponding problem for completely specified nondeterministic finite state automata is PSPACE-complete (see FINITE STATE AUTOMATA INEQUIVALENCE).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
