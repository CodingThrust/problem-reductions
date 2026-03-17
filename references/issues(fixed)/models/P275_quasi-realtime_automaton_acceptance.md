---
name: Problem
about: Propose a new problem type
title: "[Model] QuasiRealtimeAutomatonAcceptance"
labels: model
assignees: ''
---

## Motivation

QUASI-REALTIME AUTOMATON ACCEPTANCE (P275) from Garey & Johnson, A10 AL4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL4

**Mathematical definition:**

INSTANCE: A multi-tape nondeterministic Turing machine M (Turing machine program, in our terminology), whose input tape read-head must move right at each step, and which must halt whenever the read-head sees a blank, and a string x over the input alphabet Σ of M. (For a more complete description of this type of machine and its equivalent formulations, see [Book and Greibach, 1970].)
QUESTION: Does M accept x?

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

INSTANCE: A multi-tape nondeterministic Turing machine M (Turing machine program, in our terminology), whose input tape read-head must move right at each step, and which must halt whenever the read-head sees a blank, and a string x over the input alphabet Σ of M. (For a more complete description of this type of machine and its equivalent formulations, see [Book and Greibach, 1970].)
QUESTION: Does M accept x?

Reference: [Book, 1972]. Generic transformation.
Comment: Remains NP-complete even if M has only a single work tape in addition to its input tape. See also QUASI-REALTIME LANGUAGE MEMBERSHIP (the languages accepted by quasi-realtime automata are the same as the quasi-realtime languages defined in that entry).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
