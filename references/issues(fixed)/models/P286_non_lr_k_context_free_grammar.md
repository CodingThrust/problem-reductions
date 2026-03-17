---
name: Problem
about: Propose a new problem type
title: "[Model] NonLr(k)ContextFreeGrammar"
labels: model
assignees: ''
---

## Motivation

NON-LR(K) CONTEXT-FREE GRAMMAR (P286) from Garey & Johnson, A10 AL15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL15

**Mathematical definition:**

INSTANCE: Context-free grammar G, positive integer K written in unary notation.
QUESTION: Is G not an LR(K) grammar (see reference for definition)?

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

INSTANCE: Context-free grammar G, positive integer K written in unary notation.
QUESTION: Is G not an LR(K) grammar (see reference for definition)?
Reference: [Hunt, Szymanski, and Ullman, 1975]. Generic transformation.
Comment: Solvable in polynomial time for any fixed K. If K is written in binary (as in our standard encodings), then the problem is complete for NEXP-TIME and hence intractable. Determining whether there exists an integer K such that G is an LR(K) grammar is undecidable [Hunt and Szymanski, 1976a]. The same results hold if "LR(K)" is replaced by "LL(K)," "LC(K)," "SLR(K)," or any one of a number of other properties (see above references). However, in the case of LL(K), if it is known that there is some K' for which G is LR(K'), then one can decide whether there exists a K for which G is LL(K) in polynomial time [Hunt and Szymanski, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
