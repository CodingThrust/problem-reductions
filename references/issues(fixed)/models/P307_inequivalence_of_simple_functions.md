---
name: Problem
about: Propose a new problem type
title: "[Model] InequivalenceOfSimpleFunctions"
labels: model
assignees: ''
---

## Motivation

INEQUIVALENCE OF SIMPLE FUNCTIONS (P307) from Garey & Johnson, A11 PO15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO15

**Mathematical definition:**

INSTANCE: Finite set X of variables, two expressions f and g over X, each being a composition of functions from the collection "s(x) = x+1," "p(x) = max{x−1,0}," "plus(x,y) = x+y," "div(x,t) = ⌊x/t⌋," "mod(x,t) = x − t·⌊x/t⌋," "w(x,y) = if y=0 then x else 0," and "selectin(x1,x2,...,xn) = xi" where x,y,xi ∈ X, i,n,t ∈ Z+, and i ≤ n.
QUESTION: Is there an assignment of non-negative integer values to the variables in X for which the values of f and g differ?

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

INSTANCE: Finite set X of variables, two expressions f and g over X, each being a composition of functions from the collection "s(x) = x+1," "p(x) = max{x−1,0}," "plus(x,y) = x+y," "div(x,t) = ⌊x/t⌋," "mod(x,t) = x − t·⌊x/t⌋," "w(x,y) = if y=0 then x else 0," and "selectin(x1,x2,...,xn) = xi" where x,y,xi ∈ X, i,n,t ∈ Z+, and i ≤ n.
QUESTION: Is there an assignment of non-negative integer values to the variables in X for which the values of f and g differ?
Reference: [Tsichritzis, 1970]. Transformation from INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING.
Comment: Remains NP-complete even if f and g are defined only in terms of w(x,y), in terms of plus and mod, or in terms of plus and p [Lieberherr, 1977]. Variants in which f and g are defined in terms of plus and "sub1(x) = max{0,1−x}," or solely in terms of "minus(x,y) = max{0,x−y}," (where in both cases x,y ∈ X ∪ Z+) are also NP-complete [Constable, Hunt, and Sahni, 1974].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
