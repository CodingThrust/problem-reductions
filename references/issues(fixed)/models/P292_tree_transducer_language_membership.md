---
name: Problem
about: Propose a new problem type
title: "[Model] TreeTransducerLanguageMembership"
labels: model
assignees: ''
---

## Motivation

TREE TRANSDUCER LANGUAGE MEMBERSHIP (P292) from Garey & Johnson, A10 AL21. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL21

**Mathematical definition:**

INSTANCE: A "top-down finite-state tree transducer" M with output alphabet Γ, a context-free grammar G, and a string w ∈ Γ* (see references for detailed definitions).
QUESTION: Is w in the "yield" of the "surface set" determined by M and G?

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

INSTANCE: A "top-down finite-state tree transducer" M with output alphabet Γ, a context-free grammar G, and a string w ∈ Γ* (see references for detailed definitions).
QUESTION: Is w in the "yield" of the "surface set" determined by M and G?
Reference: [Reiss, 1977a]. Generic transformation.
Comment: PSPACE-complete. Problem is in NP for fixed M and G, and there exist particular choices for M and G for which the problem is NP-complete [Rounds, 1973]. The general problem is solvable in polynomial time if M is required to be "linear", while for fixed M the problem is solvable in polynomial time if M is "deterministic" [Reiss, 1977b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
