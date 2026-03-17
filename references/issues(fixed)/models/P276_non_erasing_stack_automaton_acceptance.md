---
name: Problem
about: Propose a new problem type
title: "[Model] NonErasingStackAutomatonAcceptance"
labels: model
assignees: ''
---

## Motivation

NON-ERASING STACK AUTOMATON ACCEPTANCE (P276) from Garey & Johnson, A10 AL5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL5

**Mathematical definition:**

INSTANCE: A "one-way nondeterministic non-erasing stack automaton" (a 1NESA) A with input alphabet Σ (see [Hopcroft and Ullman, 1969] for definition), and a string x ∈ Σ*.
QUESTION: Does A accept x?

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

INSTANCE: A "one-way nondeterministic non-erasing stack automaton" (a 1NESA) A with input alphabet Σ (see [Hopcroft and Ullman, 1969] for definition), and a string x ∈ Σ*.
QUESTION: Does A accept x?
Reference: [Galil, 1976], [Hopcroft and Ullman, 1967]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE. The second reference proves membership in PSPACE.
Comment: PSPACE-complete, even if x ∈ Σ* is fixed and A is restricted to be a "checking stack automaton" (as defined in [Greibach, 1969]). If x is the empty string and A is further restricted to be a checking stack automaton with a single stack symbol, the problem becomes NP-complete [Galil, 1976]. If instead x is allowed to vary and A is fixed, the problem is in NP for each 1NESA and remains so if A is allowed to be a general "nested stack automaton" [Rounds, 1973]. There exist particular 1NESAs for which the problem is NP-complete [Rounds, 1973], and these particular 1NESAs can be chosen to be checking stack automata [Shamir and Beeri, 1974] that are also "reading pushdown automata" [Hunt, 1976]. However, if A is restricted to be a "one-way nondeterministic pushdown automaton," then the problem can be solved in polynomial time (even with A allowed to vary), as indeed is the case for "two-way nondeterministic pushdown automata" [Aho, Hopcroft, and Ullman, 1968].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
