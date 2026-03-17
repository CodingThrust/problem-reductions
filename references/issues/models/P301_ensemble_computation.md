---
name: Problem
about: Propose a new problem type
title: "[Model] EnsembleComputation"
labels: model
assignees: ''
---

## Motivation

ENSEMBLE COMPUTATION (P301) from Garey & Johnson, A11 PO9. A classical NP-complete problem useful for reductions. It asks whether a given collection of target subsets can all be computed by a short sequence of disjoint union operations, starting from singletons or previously computed intermediate sets.

## Definition

**Name:** `EnsembleComputation`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO9

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set A, positive integer J.
QUESTION: Is there a sequence S = (z₁ ← x₁ ∪ y₁, z₂ ← x₂ ∪ y₂, ..., zⱼ ← xⱼ ∪ yⱼ) of j ≤ J union operations, where each xᵢ and yᵢ is either {a} for some a ∈ A or z_k for some k < i, such that xᵢ and yᵢ are disjoint, 1 ≤ i ≤ j, and such that for every subset c ∈ C there exists some zᵢ, 1 ≤ i ≤ j, that is identical to c?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->
- **Count:** The decision variable is the sequence of up to J union operations. Each operation i ∈ {1, ..., j} selects two operands (x_i, y_i), each of which is either a singleton {a} for some a ∈ A or a previously computed zₖ with k < i.
- **Per-variable domain:** For step i, each operand can be chosen from |A| singletons plus i−1 previously computed sets; the two operands must be disjoint.
- **Meaning:** A valid assignment is a sequence of ≤ J disjoint unions whose results include every target subset in C as some intermediate zᵢ.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->
**Type name:** `EnsembleComputation`
**Variants:** No generic parameters needed (elements and subsets represented by integer indices); a single concrete variant.

| Field | Type | Description |
|-------|------|-------------|
| `universe_size` | `usize` | Number of elements in A (elements are represented as 0..universe_size) |
| `subsets` | `Vec<Vec<usize>>` | The collection C: each inner Vec lists elements of one required subset |
| `budget` | `usize` | Maximum number of union operations J allowed |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->
- **Best known exact algorithm:** ENSEMBLE COMPUTATION is NP-complete (Garey & Johnson, Theorem 3.6). No polynomial-time algorithm is known. A brute-force search over all valid operation sequences has complexity O\*(2^(|A|·J)) in the worst case (exponential in both universe size and budget). In practice, branch-and-bound or dynamic programming over subsets can solve small instances, but no known O\*(c^n) algorithm with c < 2 is established for the general case.

## Extra Remark

**Full book text:**

INSTANCE: Collection C of subsets of a finite set A, positive integer J.
QUESTION: Is there a sequence S = (z1 ← x1 ∪ y1, z2 ← x2 ∪ y2,...,zj ← xj ∪ yj) of j ≤ J union operations, where each xi and yi is either {a} for some a ∈ A or zk for some k < i, such that xi and yi are disjoint, 1 ≤ i ≤ j, and such that for every subset c ∈ C there exists some zi, 1 ≤ i ≤ j, that is identical to c?
Reference: [Garey and Johnson, ——]. Transformation from VERTEX COVER (see Section 3.2.2).
Comment: Remains NP-complete even if each c ∈ C satisfies |c| ≤ 3. The analogous problem in which xi and yi need not be disjoint for 1 ≤ i ≤ j is also NP-complete under the same restriction.

## How to solve

- [x] It can be solved by (existing) bruteforce: enumerate all valid sequences of ≤ J disjoint-union operations over elements of A (and previously built sets), check if every c ∈ C is produced.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: N/A — no other known tractable special case applies to the general problem

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Small satisfiable instance:**
- Universe: A = {0, 1, 2, 3} (4 elements)
- Required subsets: C = {{0,1,2}, {0,1,3}}
- Budget: J = 4

**Witness sequence:**
- z₁ = {0} ∪ {1} = {0,1}   (disjoint ✓)
- z₂ = z₁ ∪ {2} = {0,1,2} = C[0] ✓
- z₃ = z₁ ∪ {3} = {0,1,3} = C[1] ✓

All subsets in C appear (j = 3 ≤ J = 4 ✓). Note z₁ is reused by both z₂ and z₃, which is valid since z₁ = {0,1} and the added singletons {2} and {3} are disjoint from z₁.

**Small unsatisfiable instance:**
- Universe: A = {0, 1, 2}
- Required subsets: C = {{0,1,2}}
- Budget: J = 1

With only 1 operation, we can produce at most one 2-element set (e.g., {0,1}). We cannot produce the 3-element set {0,1,2} in a single union of two disjoint sets from singletons: {0}∪{1,2} fails because {1,2} is not a singleton or previously computed z; {0,1}∪{2} fails because {0,1} has not been computed yet. Hence the instance is a NO instance.
