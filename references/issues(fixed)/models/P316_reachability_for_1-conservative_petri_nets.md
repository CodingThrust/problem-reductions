---
name: Problem
about: Propose a new problem type
title: "[Model] ReachabilityFor1ConservativePetriNets(*)"
labels: model
assignees: ''
---

## Motivation

REACHABILITY FOR 1-CONSERVATIVE PETRI NETS (*) (P316) from Garey & Johnson, A12 MS4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS4

**Mathematical definition:**

INSTANCE: Petri net P = (n,M0,T) that is "1-conservative," i.e., for each <a,b> ∈ T, a and b have the same number of 1's, and an n-tuple M of nonnegative integers.
QUESTION: Is M reachable from M0 in P, i.e., is there a sequence <a1,b1> <a2,b2> ··· <am,bm> of transitions from T such that the sequence M0,M1,...,M2m obtained as in the preceding problem contains no vector with a negative component and satisfies M2m = M?

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

INSTANCE: Petri net P = (n,M0,T) that is "1-conservative," i.e., for each <a,b> ∈ T, a and b have the same number of 1's, and an n-tuple M of nonnegative integers.
QUESTION: Is M reachable from M0 in P, i.e., is there a sequence <a1,b1> <a2,b2> ··· <am,bm> of transitions from T such that the sequence M0,M1,...,M2m obtained as in the preceding problem contains no vector with a negative component and satisfies M2m = M?
Reference: [Jones, Landweber, and Lien, 1977]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
Comment: PSPACE-complete, even if P is also a free choice Petri net. Problem is not known to be decidable for arbitrary Petri nets, but is known to require at least exponential space [Lipton, 1975]. Analogous results hold for the "coverability" problem: Is there an M' having each of its components no smaller than the corresponding component of M such that M' is reachable from M0? The related "K-boundedness" problem (given P and an integer K, is there no vector that exceeds K in every component that is reachable from M0?) is PSPACE-complete for arbitrary Petri nets, as well as for 1-conservative free choice Petri nets. See [Jones, Landweber, and Lien, 1977] and [Hunt, 1977] for additional details and related results.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
