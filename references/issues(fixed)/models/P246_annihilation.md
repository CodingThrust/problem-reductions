---
name: Problem
about: Propose a new problem type
title: "[Model] Annihilation"
labels: model
assignees: ''
---

## Motivation

ANNIHILATION (P246) from Garey & Johnson, A8 GP9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP9

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A), collection {Ai: 1 ≤ i ≤ r} of (not necessarily disjoint) subsets of A, function f0 mapping V into {0,1,2,...,r}, where f0(v) = i > 0 means that a "token" of type i is "on" vertex v and f0(v) = 0 means that v is unoccupied.
QUESTION: Does player 1 have a forced win in the following game played on G? A position is a function f: V → {0,1,...,r} with f0 being the initial position and players alternating moves. A player moves by selecting a vertex v ∈ V with f(v) > 0 and an arc (v,w) ∈ Af(v), and the move corresponds to moving the token on vertex v to vertex w. The new position f' is the same as f except that f'(v) = 0 and f'(w) is either 0 or f(v), depending, respectively, on whether f(w) > 0 or f(w) = 0. (If f(w) > 0, then both the token moved to w and the token already there are "annihilated.") Player 1 wins if and only if player 2 is the first player unable to move.

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

INSTANCE: Directed acyclic graph G = (V,A), collection {Ai: 1 ≤ i ≤ r} of (not necessarily disjoint) subsets of A, function f0 mapping V into {0,1,2,...,r}, where f0(v) = i > 0 means that a "token" of type i is "on" vertex v and f0(v) = 0 means that v is unoccupied.
QUESTION: Does player 1 have a forced win in the following game played on G? A position is a function f: V → {0,1,...,r} with f0 being the initial position and players alternating moves. A player moves by selecting a vertex v ∈ V with f(v) > 0 and an arc (v,w) ∈ Af(v), and the move corresponds to moving the token on vertex v to vertex w. The new position f' is the same as f except that f'(v) = 0 and f'(w) is either 0 or f(v), depending, respectively, on whether f(w) > 0 or f(w) = 0. (If f(w) > 0, then both the token moved to w and the token already there are "annihilated.") Player 1 wins if and only if player 2 is the first player unable to move.

Reference: [Fraenkel and Yesha, 1977]. Transformation from VERTEX COVER.
Comment: NP-hard and in PSPACE, but not known to be PSPACE-complete. Remains NP-hard even if r = 2 and A1 ∩ A2 is empty. Problem can be solved in polynomial time if r = 1 [Fraenkel and Yesha, 1976]. Related NP-hardness results for other token-moving games on directed graphs (REMOVE, CONTRAJUNCTIVE, CAPTURE, BLOCKING, TARGET) can be found in [Fraenkel and Yesha, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
