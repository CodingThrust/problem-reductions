---
name: Problem
about: Propose a new problem type
title: "[Model] ReynoldsCoveringForContextFreeGrammars"
labels: model
assignees: ''
---

## Motivation

REYNOLDS COVERING FOR CONTEXT-FREE GRAMMARS (P282) from Garey & Johnson, A10 AL11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL11

**Mathematical definition:**

INSTANCE: Context-free grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2), where Σ is a finite set of "terminal" symbols, N_i is a finite set of "nonterminal" symbols, S_i ∈ N_i is the "initial" symbol, and Π_i is a set of "productions" of the form "A → w," where A ∈ N_i and w ∈ (N_i ∪ Σ)*.
QUESTION: Does G_2 "Reynolds cover" G_1, i.e., is there a function f mapping N_1 ∪ Σ into N_2 ∪ Σ such that f(x) = x for all x ∈ Σ, f(A) ∈ N_2 for all A ∈ N_1, f(S_1) = S_2, and for each production A → x_1 x_2 ··· x_n in Π_1, the image f(A) → f(x_1)f(x_2) ··· f(x_n) of that production is in Π_2?

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

INSTANCE: Context-free grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2), where Σ is a finite set of "terminal" symbols, N_i is a finite set of "nonterminal" symbols, S_i ∈ N_i is the "initial" symbol, and Π_i is a set of "productions" of the form "A → w," where A ∈ N_i and w ∈ (N_i ∪ Σ)*.
QUESTION: Does G_2 "Reynolds cover" G_1, i.e., is there a function f mapping N_1 ∪ Σ into N_2 ∪ Σ such that f(x) = x for all x ∈ Σ, f(A) ∈ N_2 for all A ∈ N_1, f(S_1) = S_2, and for each production A → x_1 x_2 ··· x_n in Π_1, the image f(A) → f(x_1)f(x_2) ··· f(x_n) of that production is in Π_2?
Reference: [Hunt and Rosenkrantz, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete even if G_1 and G_2 are restricted to "regular" grammars. The same results hold for the related questions of whether G_2 "weakly Reynolds covers" G_1 or whether G_2 is a "homomorphic image" of G_1. The problem "Given G is there an LL(k) context-free grammar H such that H Reynolds covers G?" is solvable in polynomial time, as are the related problems where LL(k) is replaced by LR(k) or one of a number of other grammar classes (see [Hunt and Rosenkrantz, 1977]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
