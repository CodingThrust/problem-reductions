---
name: Problem
about: Propose a new problem type
title: "[Model] CoveringForLinearGrammars"
labels: model
assignees: ''
---

## Motivation

COVERING FOR LINEAR GRAMMARS (P283) from Garey & Johnson, A10 AL12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL12

**Mathematical definition:**

INSTANCE: Two linear context-free grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2), where no production in such a grammar is allowed to have more than one nonterminal symbol on its right hand side.
QUESTION: Is there a function h: P_1 → P_2 ∪ {λ} (where λ denotes the empty production) such that G_1 covers G_2 under h, i.e., such that for all strings w ∈ Σ* (1) if w is derivable from S_1 under the sequence of productions p_1,p_2, . . . , p_n, then w is derivable from S_2 under the sequence h(p_1),h(p_2), . . . , h(p_n), and (2) if w is derivable from S_2 under the sequence of productions q_1,q_2, . . . , q_n from Π_2, then there exists a sequence of productions p_1,p_2, . . . , p_m that is a derivation of w in G_1 such that h(p_1),h(p_2), . . . , h(p_m) equals q_1,q_2, . . . , q_n?

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

INSTANCE: Two linear context-free grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2), where no production in such a grammar is allowed to have more than one nonterminal symbol on its right hand side.
QUESTION: Is there a function h: P_1 → P_2 ∪ {λ} (where λ denotes the empty production) such that G_1 covers G_2 under h, i.e., such that for all strings w ∈ Σ* (1) if w is derivable from S_1 under the sequence of productions p_1,p_2, . . . , p_n, then w is derivable from S_2 under the sequence h(p_1),h(p_2), . . . , h(p_n), and (2) if w is derivable from S_2 under the sequence of productions q_1,q_2, . . . , q_n from Π_2, then there exists a sequence of productions p_1,p_2, . . . , p_m that is a derivation of w in G_1 such that h(p_1),h(p_2), . . . , h(p_m) equals q_1,q_2, . . . , q_n?
Reference: [Hunt, Rosenkrantz, and Szymanski, 1976a], [Hunt, Rosenkrantz, and Szymanski, 1976b]. Transformation from REGULAR EXPRESSION NON-UNIVERSALITY. The second reference proves membership in PSPACE.
Comment: PSPACE-complete, even for "regular" grammars. Undecidable for arbitrary context-free grammars. See [Hunt and Rosenkrantz, 1977] for related results.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
