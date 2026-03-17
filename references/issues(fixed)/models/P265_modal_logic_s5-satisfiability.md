---
name: Problem
about: Propose a new problem type
title: "[Model] ModalLogicS5Satisfiability"
labels: model
assignees: ''
---

## Motivation

MODAL LOGIC S5-SATISFIABILITY (P265) from Garey & Johnson, A9 LO13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO13

**Mathematical definition:**

INSTANCE: Well-formed modal formula A over a finite set U of variables, where a modal formula is either a variable u∈U or is of the form "(A∧B)," "¬A," or "□A," where A and B are modal formulas.
QUESTION: Is A "S5-satisfiable," i.e., is there a model (W,R,V), where W is a set, R is a reflexive, transitive, and symmetric binary relation on W, and V is a mapping from U×W into {T,F} such that, for some w∈W, V(A,w)=T, where V is extended to formulas by V(A∧B,w)=T if and only if V(A,w)=V(B,w)=T, V(¬A,w)=T if and only if V(A,w)=F, and V(□A,w)=T if and only if V(A,w')=T for all w'∈W satisfying (w,w')∈R?

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

INSTANCE: Well-formed modal formula A over a finite set U of variables, where a modal formula is either a variable u∈U or is of the form "(A∧B)," "¬A," or "□A," where A and B are modal formulas.
QUESTION: Is A "S5-satisfiable," i.e., is there a model (W,R,V), where W is a set, R is a reflexive, transitive, and symmetric binary relation on W, and V is a mapping from U×W into {T,F} such that, for some w∈W, V(A,w)=T, where V is extended to formulas by V(A∧B,w)=T if and only if V(A,w)=V(B,w)=T, V(¬A,w)=T if and only if V(A,w)=F, and V(□A,w)=T if and only if V(A,w')=T for all w'∈W satisfying (w,w')∈R?
Reference: [Ladner, 1977]. Transformation from 3SAT. Nontrivial part is proving membership in NP.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
