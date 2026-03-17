---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumInferredFiniteStateAutomaton"
labels: model
assignees: ''
---

## Motivation

MINIMUM INFERRED FINITE STATE AUTOMATON (P279) from Garey & Johnson, A10 AL8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL8

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, two finite subsets S, T ⊆ Σ*, positive integer K.
QUESTION: Is there a K-state deterministic finite automaton A that recognizes a language L ⊆ Σ* such that S ⊆ L and T ⊆ Σ*-L?

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

INSTANCE: Finite alphabet Σ, two finite subsets S, T ⊆ Σ*, positive integer K.
QUESTION: Is there a K-state deterministic finite automaton A that recognizes a language L ⊆ Σ* such that S ⊆ L and T ⊆ Σ*-L?
Reference: [Gold, 1974]. Transformation from MONOTONE 3SAT.
Comment: Can be solved in polynomial time if S ∪ T = Σ^(n) for some n, where Σ^(n) is the set of all strings of length n or less over Σ [Trakhtenbrot and Barzdin, 1973]. However, for any fixed ε > 0, the problem remains NP-complete if restricted to instances for which (S ∪ T) ⊆ Σ^(n) and |Σ^(n) − (S ∪ T)| ≤ |Σ^(n)|^ε [Angluin, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
