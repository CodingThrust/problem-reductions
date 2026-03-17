---
name: Problem
about: Propose a new problem type
title: "[Model] SquareTiling"
labels: model
assignees: ''
---

## Motivation

SQUARE-TILING (P250) from Garey & Johnson, A8 GP13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP13

**Mathematical definition:**

INSTANCE: Set C of "colors," collection T ⊆ C4 of "tiles" (where <a,b,c,d> denotes a tile whose top, right, bottom, and left sides are colored a,b,c, and d, respectively), and a positive integer N ≤ |C|.
QUESTION: Is there a tiling of an N×N square using the tiles in T, i.e., an assignment of a tile A(i,j) ∈ T to each ordered pair i,j, 1 ≤ i ≤ N, 1 ≤ j ≤ N, such that (1) if f(i,j) = <a,b,c,d> and f(i+1,j) = <a',b',c',d'>, then a = c', and (2) if f(i,j) = <a,b,c,d> and f(i,j+1) = <a',b',c',d'>, then b = d'?

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

INSTANCE: Set C of "colors," collection T ⊆ C4 of "tiles" (where <a,b,c,d> denotes a tile whose top, right, bottom, and left sides are colored a,b,c, and d, respectively), and a positive integer N ≤ |C|.
QUESTION: Is there a tiling of an N×N square using the tiles in T, i.e., an assignment of a tile A(i,j) ∈ T to each ordered pair i,j, 1 ≤ i ≤ N, 1 ≤ j ≤ N, such that (1) if f(i,j) = <a,b,c,d> and f(i+1,j) = <a',b',c',d'>, then a = c', and (2) if f(i,j) = <a,b,c,d> and f(i,j+1) = <a',b',c',d'>, then b = d'?

Reference: [Garey, Johnson, and Papadimitriou, 1977]. Transformation from DIRECTED HAMILTONIAN PATH.
Comment: Variant in which we ask if T can be used to tile the entire plane (Z×Z) "periodically" with period less than N is also NP-complete. In general, the problem of whether a set of tiles can be used to tile the plane is undecidable [Berger, 1966], as is the problem of whether a set of tiles can be used to tile the plane periodically.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
