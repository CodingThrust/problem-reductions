---
name: Problem
about: Propose a new problem type
title: "[Model] RectilinearPictureCompression"
labels: model
assignees: ''
---

## Motivation

RECTILINEAR PICTURE COMPRESSION (P173) from Garey & Johnson, A4 SR25. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR25

**Mathematical definition:**

INSTANCE: An n×n matrix M of 0's and 1's, and a positive integer K.
QUESTION: Is there a collection of K or fewer rectangles that covers precisely those entries in M that are 1's, i.e., is there a sequence of quadruples (ai,bi,ci,di), 1 ≤ i ≤ K, where ai ≤ bi, ci ≤ di, 1 ≤ i ≤ K, such that for every pair (i,j), 1 ≤ i,j ≤ n, Mij = 1 if and only if there exists a k, 1 ≤ k ≤ K, such that ak ≤ i ≤ bk and ck ≤ j ≤ dk?

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

INSTANCE: An n×n matrix M of 0's and 1's, and a positive integer K.
QUESTION: Is there a collection of K or fewer rectangles that covers precisely those entries in M that are 1's, i.e., is there a sequence of quadruples (ai,bi,ci,di), 1 ≤ i ≤ K, where ai ≤ bi, ci ≤ di, 1 ≤ i ≤ K, such that for every pair (i,j), 1 ≤ i,j ≤ n, Mij = 1 if and only if there exists a k, 1 ≤ k ≤ K, such that ak ≤ i ≤ bk and ck ≤ j ≤ dk?
Reference: [Masek, 1978]. Transformation from 3SAT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
