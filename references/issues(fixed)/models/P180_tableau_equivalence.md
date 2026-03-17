---
name: Problem
about: Propose a new problem type
title: "[Model] TableauEquivalence"
labels: model
assignees: ''
---

## Motivation

TABLEAU EQUIVALENCE (P180) from Garey & Johnson, A4 SR32. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR32

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of ordered pairs of subsets of A, a set X of distinguished variables, a set Y of undistinguished variables, a set Ca of constants for each a ∈ A, and two "tableaux" T1 and T2 over X, Y, and the Ca. (A tableau is essentially a matrix with a column for each attribute and entries from X, Y, the Ca, along with a blank symbol. For details and an interpretation in terms of relational expressions, see reference.)
QUESTION: Are T1 and T2 "weakly equivalent," i.e., do they represent identical relations under "universal interpretations"?

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

INSTANCE: A set A of attribute names, a collection F of ordered pairs of subsets of A, a set X of distinguished variables, a set Y of undistinguished variables, a set Ca of constants for each a ∈ A, and two "tableaux" T1 and T2 over X, Y, and the Ca. (A tableau is essentially a matrix with a column for each attribute and entries from X, Y, the Ca, along with a blank symbol. For details and an interpretation in terms of relational expressions, see reference.)
QUESTION: Are T1 and T2 "weakly equivalent," i.e., do they represent identical relations under "universal interpretations"?
Reference: [Aho, Sagiv, and Ullman, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete even if the tableaux come from "expressions" that have no "select" operations, or if the tableaux come from expressions that have select operations but F is empty, or if F is empty, the tableaux contain no constants, and the tableaux do not necessarily come from expressions at all. Problem is solvable in polynomial time for "simple" tableaux. The same results hold also for "strong equivalence," where the two tableaux must represent identical relations under all interpretations. The problem of tableau "containment," however, is NP-complete even for simple tableaux and for still further restricted tableaux [Sagiv and Yannakakis, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
