---
name: Problem
about: Propose a new problem type
title: "[Model] NonContainmentForFreeBSchemes"
labels: model
assignees: ''
---

## Motivation

NON-CONTAINMENT FOR FREE B-SCHEMES (P310) from Garey & Johnson, A11 PO18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO18

**Mathematical definition:**

INSTANCE: Two free B-schemes S1 and S2, where a free B-scheme is a rooted, directed acyclic graph G = (V,A), all of whose vertices have out-degree 0 (leaves) or 2 (tests), with the two arcs leaving a test vertex labeled L and R respectively, together with a set B of Boolean variable symbols and a label l(v) ∈ B for each test vertex, such that no two test vertices on the same directed path get the same label, and a set F of function symbols along with a label l(v) ∈ F ∪ {Ω} for each leaf in V.
QUESTION: Is S1 not "contained" in S2, i.e., is there an assignment t: B1 ∪ B2→{L,R} such that if the paths from the roots of G1 and G2 to leaf vertices determined by always leaving a test vertex v by the arc labeled t(l(v)) terminate at leaves labeled f1 and f2 respectively, then f1 ≠ f2 and f1 ≠ Ω?

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

INSTANCE: Two free B-schemes S1 and S2, where a free B-scheme is a rooted, directed acyclic graph G = (V,A), all of whose vertices have out-degree 0 (leaves) or 2 (tests), with the two arcs leaving a test vertex labeled L and R respectively, together with a set B of Boolean variable symbols and a label l(v) ∈ B for each test vertex, such that no two test vertices on the same directed path get the same label, and a set F of function symbols along with a label l(v) ∈ F ∪ {Ω} for each leaf in V.
QUESTION: Is S1 not "contained" in S2, i.e., is there an assignment t: B1 ∪ B2→{L,R} such that if the paths from the roots of G1 and G2 to leaf vertices determined by always leaving a test vertex v by the arc labeled t(l(v)) terminate at leaves labeled f1 and f2 respectively, then f1 ≠ f2 and f1 ≠ Ω?
Reference: [Fortune, Hopcroft, and Schmidt, 1977]. Transformation from 3SAT.
Comment: The "strong inequivalence" problem for free B-schemes (same as above, only all that we now require is that f1 ≠ f2) is open, but can be solved in polynomial time if one of S1 and S2 is an "ordered" B-scheme. The open version is Turing equivalent to the strong inequivalence problem for free Ianov schemes (see STRONG INEQUIVALENCE OF IANOV SCHEMES).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
