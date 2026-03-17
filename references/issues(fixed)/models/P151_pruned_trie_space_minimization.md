---
name: Problem
about: Propose a new problem type
title: "[Model] PrunedTrieSpaceMinimization"
labels: model
assignees: ''
---

## Motivation

PRUNED TRIE SPACE MINIMIZATION (P151) from Garey & Johnson, A4 SR3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR3

**Mathematical definition:**

INSTANCE: Finite set S, collection F of functions f: S → Z+, and a positive integer K.
QUESTION: Is there a sequence <f1,f2,...,fm> of distinct functions from F such that for every two elements a,b ∈ S there is some i, 1 ≤ i ≤ m, for which fi(a) ≠ fi(b) and such that, if N(i) denotes the number of distinct i-tuples X = (x1,x2,...,xi) for which there is more than one a ∈ S having (f1(a),f2(a),...,fi(a)) = X, then ∑m i=1 N(i) ≤ K?

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

INSTANCE: Finite set S, collection F of functions f: S → Z+, and a positive integer K.
QUESTION: Is there a sequence <f1,f2,...,fm> of distinct functions from F such that for every two elements a,b ∈ S there is some i, 1 ≤ i ≤ m, for which fi(a) ≠ fi(b) and such that, if N(i) denotes the number of distinct i-tuples X = (x1,x2,...,xi) for which there is more than one a ∈ S having (f1(a),f2(a),...,fi(a)) = X, then ∑m i=1 N(i) ≤ K?
Reference: [Comer and Sethi, 1976]. Transformation from 3DM.
Comment: Remains NP-complete even if all f ∈ F have range {0,1}. Variants in which the "pruned trie" data structure abstracted above is replaced by "full trie," "collapsed trie," or "pruned 0-trie" are also NP-complete. The related "access time minimization" problem is also NP-complete for pruned tries, where we ask for a sequence <f1,f2,...,fm> of functions from F that distinguishes every two elements from S as above and such that, if the access time L(a) for a ∈ S is defined to be the least i for which no other b ∈ S has (f1(b),f2(b),...,fi(b)) identical to (f1(a),f2(a),...,fi(a)), then ∑a ∈ S L(a) ≤ K.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
