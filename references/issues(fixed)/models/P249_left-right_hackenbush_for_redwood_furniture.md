---
name: Problem
about: Propose a new problem type
title: "[Model] LeftRightHackenbushForRedwoodFurniture"
labels: model
assignees: ''
---

## Motivation

LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE (P249) from Garey & Johnson, A8 GP12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP12

**Mathematical definition:**

INSTANCE: A piece of "redwood furniture," i.e., a connected graph G = (V,E) with a specified "ground" vertex v ∈ V and a partition of the edges into sets L and R, where L is the set of all edges containing v (the set of "feet"), R = E−L, and each "foot" in L shares a vertex with at most one edge in R, which is its corresponding "leg" (not all edges in R need to be legs however), and a positive integer K.
QUESTION: Is the "value" of the Left-Right Hackenbush game played on G less than or equal to 2−K (see [Conway, 1976] for the definition of the game, there called Hackenbush Restrained, and for the definition of "value")?

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

INSTANCE: A piece of "redwood furniture," i.e., a connected graph G = (V,E) with a specified "ground" vertex v ∈ V and a partition of the edges into sets L and R, where L is the set of all edges containing v (the set of "feet"), R = E−L, and each "foot" in L shares a vertex with at most one edge in R, which is its corresponding "leg" (not all edges in R need to be legs however), and a positive integer K.
QUESTION: Is the "value" of the Left-Right Hackenbush game played on G less than or equal to 2−K (see [Conway, 1976] for the definition of the game, there called Hackenbush Restrained, and for the definition of "value")?

Reference: [Berlekamp, 1976]. Transformation from SET COVERING.
Comment: Remains NP-complete even for "bipartite" redwood furniture, but can be solved in polynomial time for the subclass of redwood furniture known as "redwood trees." As a consequence of this result, the problem of determining if player 1 has a win in an arbitrary game of Left-Right Hackenbush is NP-hard.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
