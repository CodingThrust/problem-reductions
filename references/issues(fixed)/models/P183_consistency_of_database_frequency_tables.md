---
name: Problem
about: Propose a new problem type
title: "[Model] ConsistencyOfDatabaseFrequencyTables"
labels: model
assignees: ''
---

## Motivation

CONSISTENCY OF DATABASE FREQUENCY TABLES (P183) from Garey & Johnson, A4 SR35. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR35

**Mathematical definition:**

INSTANCE: Set A of attribute names, domain set Da for each a ∈ A, set V of objects, collection F of frequency tables for some pairs a,b ∈ A (where a frequency table for a,b ∈ A is a function fa,b: Da×Db → Z+ with the sum, over all pairs x ∈ Da and y ∈ Db, of fa,b(x,y) equal to |V|), and a set K of triples (v,a,x) with v ∈ V, a ∈ A, and x ∈ Da, representing the known attribute values.
QUESTION: Are the frequency tables in F consistent with the known attribute values in K, i.e., is there a collection of functions ga: V → Da, for each a ∈ A, such that ga(v) = x if (v,a,x) ∈ K and such that, for each fa,b ∈ F, x ∈ Da, and y ∈ Db, the number of v ∈ V for which ga(v) = x and gb(v) = y is exactly fa,b(x,y)?

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

INSTANCE: Set A of attribute names, domain set Da for each a ∈ A, set V of objects, collection F of frequency tables for some pairs a,b ∈ A (where a frequency table for a,b ∈ A is a function fa,b: Da×Db → Z+ with the sum, over all pairs x ∈ Da and y ∈ Db, of fa,b(x,y) equal to |V|), and a set K of triples (v,a,x) with v ∈ V, a ∈ A, and x ∈ Da, representing the known attribute values.
QUESTION: Are the frequency tables in F consistent with the known attribute values in K, i.e., is there a collection of functions ga: V → Da, for each a ∈ A, such that ga(v) = x if (v,a,x) ∈ K and such that, for each fa,b ∈ F, x ∈ Da, and y ∈ Db, the number of v ∈ V for which ga(v) = x and gb(v) = y is exactly fa,b(x,y)?
Reference: [Reiss, 1977b]. Transformation from 3SAT.
Comment: Above result implies that no polynomial time algorithm can be given for "compromising" a data base from its frequency tables by deducing prespecified attribute values, unless P = NP (see reference for details).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
