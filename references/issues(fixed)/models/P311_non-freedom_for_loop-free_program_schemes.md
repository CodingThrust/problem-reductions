---
name: Problem
about: Propose a new problem type
title: "[Model] NonFreedomForLoopFreeProgramSchemes"
labels: model
assignees: ''
---

## Motivation

NON-FREEDOM FOR LOOP-FREE PROGRAM SCHEMES (P311) from Garey & Johnson, A11 PO19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO19

**Mathematical definition:**

INSTANCE: Finite sets F and P of function and predicate symbols, set X of variables, and a loop-free monadic program scheme S over F,P, and X, where such a scheme consists of a sequence I1,I2,...,Im of instructions of the form "x ← f(y)," "if p(x) then goto Ij else goto Ik," and "halt," with x ∈ X, f ∈ F, and p ∈ P, and must be such that no directed cycles occur in the corresponding "flow graph."
QUESTION: Is S non-free, i.e., is there a directed path in the flow graph for S that can never be followed in any computation, no matter what the interpretation of the functions and predicates in F and P and the initial values for the variables in X?

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

INSTANCE: Finite sets F and P of function and predicate symbols, set X of variables, and a loop-free monadic program scheme S over F,P, and X, where such a scheme consists of a sequence I1,I2,...,Im of instructions of the form "x ← f(y)," "if p(x) then goto Ij else goto Ik," and "halt," with x ∈ X, f ∈ F, and p ∈ P, and must be such that no directed cycles occur in the corresponding "flow graph."
QUESTION: Is S non-free, i.e., is there a directed path in the flow graph for S that can never be followed in any computation, no matter what the interpretation of the functions and predicates in F and P and the initial values for the variables in X?
Reference: [Constable, Hunt, and Sahni, 1974]. Transformation from 3SAT.
Comment: Remains NP-complete for |X| = 2. If |X| = 1, the problem is solvable in polynomial time. If loops are allowed and |X| is arbitrary, the problem is undecidable [Paterson, 1967].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
