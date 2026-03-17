---
name: Problem
about: Propose a new problem type
title: "[Model] FaultDetectionInLogicCircuits"
labels: model
assignees: ''
---

## Motivation

FAULT DETECTION IN LOGIC CIRCUITS (P329) from Garey & Johnson, A12 MS17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS17

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A) with a single vertex v* ∈ V having out-degree 0, an assignment f: (V−{v*})→{I, and, or, not} such that f(v) = I implies v has in-degree 0, f(v) = not implies v has in-degree 1, and f(v) = and or f(v) = or implies v has in-degree 2, and a subset V' ⊆ V.
QUESTION: Can all single faults occurring at vertices of V' be detected by input-output experiments, i.e., regarding G as a logic circuit with input vertices I, output vertex v*, and logic gates for the functions "and," "or," and "not" at the specified vertices, is there for each v ∈ V' and x ∈ {T,F} an assignment of a value to each vertex in I of a value in {T,F} such that the output of the circuit for those input values differs from the output of the same circuit with the output of the gate at v "stuck-at" x?

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

INSTANCE: Directed acyclic graph G = (V,A) with a single vertex v* ∈ V having out-degree 0, an assignment f: (V−{v*})→{I, and, or, not} such that f(v) = I implies v has in-degree 0, f(v) = not implies v has in-degree 1, and f(v) = and or f(v) = or implies v has in-degree 2, and a subset V' ⊆ V.
QUESTION: Can all single faults occurring at vertices of V' be detected by input-output experiments, i.e., regarding G as a logic circuit with input vertices I, output vertex v*, and logic gates for the functions "and," "or," and "not" at the specified vertices, is there for each v ∈ V' and x ∈ {T,F} an assignment of a value to each vertex in I of a value in {T,F} such that the output of the circuit for those input values differs from the output of the same circuit with the output of the gate at v "stuck-at" x?
Reference: [Ibarra and Sahni, 1975]. Transformation from 3SAT.
Comment: Remains NP-complete even if V' = V or if V' contains just a single vertex v with f(v) = I.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
