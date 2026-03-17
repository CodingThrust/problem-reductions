---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumCardinalityKey"
labels: model
assignees: ''
---

## Motivation

MINIMUM CARDINALITY KEY (P174) from Garey & Johnson, A4 SR26. A classical NP-complete problem from relational database theory. Given a set of attribute names and functional dependencies, the problem asks whether there exists a candidate key of cardinality at most M. This is fundamental to database normalization: identifying the smallest key determines the most efficient way to uniquely identify rows in a relation. The problem connects graph-theoretic vertex cover to database schema design.

**Associated rules:**
<!-- ⚠️ Unverified: AI-collected rule associations -->
- R120: Vertex Cover -> Minimum Cardinality Key (this model is the target)
- R122: Minimum Cardinality Key -> Prime Attribute Name (this model is the source)

## Definition

**Name:** <!-- ⚠️ Unverified --> `MinimumCardinalityKey`
**Canonical name:** Minimum Cardinality Key (also: Minimum Key, Least Cardinality Candidate Key)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR26, p.232

**Mathematical definition:**

INSTANCE: A set A of "attribute names," a collection F of ordered pairs of subsets of A (called "functional dependencies" on A), and a positive integer M.
QUESTION: Is there a key of cardinality M or less for the relational system <A,F>, i.e., a minimal subset K ⊆ A with |K| <= M such that the ordered pair (K,A) belongs to the "closure" F* of F defined by (1) F ⊆ F*, (2) B ⊆ C ⊆ A implies (C,B) in F*, (3) (B,C),(C,D) in F* implies (B,D) in F*, and (4) (B,C),(B,D) in F* implies (B,C ∪ D) in F*?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |A| binary variables, one per attribute name.
- **Per-variable domain:** binary {0, 1} -- whether the attribute is included in the candidate key K.
- **Meaning:** Variable x_i = 1 if attribute a_i is included in the key K. The configuration (x_1, ..., x_{|A|}) encodes a candidate key K = {a_i : x_i = 1}. The assignment is valid if K determines all of A under the closure F* and |K| <= M. Additionally, K must be minimal: no proper subset of K is also a key.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `MinimumCardinalityKey`
**Variants:** none (no graph or weight parameters; the problem is purely set-theoretic)

| Field | Type | Description |
|-------|------|-------------|
| `num_attributes` | `usize` | Number of attributes |A| (attributes indexed 0..num_attributes) |
| `dependencies` | `Vec<(Vec<usize>, Vec<usize>)>` | Functional dependencies F; each pair (lhs, rhs) means lhs -> rhs |
| `budget` | `usize` | Budget M: key must have cardinality <= budget |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- The closure F* is computed using Armstrong's axioms: reflexivity (B ⊆ C implies C -> B), transitivity ((B,C) and (C,D) imply (B,D)), and augmentation/union ((B,C) and (B,D) imply (B, C ∪ D)).
- A key K must satisfy: (1) K determines all of A (i.e., (K,A) in F*), and (2) K is minimal (no proper subset of K also determines A).
- The number of candidate keys can be exponential in |A|.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Brute-force enumeration of all subsets of A of size at most M, checking each for the key property via attribute closure computation. Closure computation is O(|F| * |A|) per subset (linear pass through dependencies). Total: O(binom(|A|, M) * |F| * |A|). For M = |A|/2, this is O(2^|A| * |F| * |A|). Lucchesi and Osborne (1978) give an algorithm that finds all candidate keys in time polynomial in |A|, |F|, and the number of keys |K|, but since |K| can be exponential, the worst case remains exponential.
- **Parameterized complexity:** The problem is W[2]-hard parameterized by M (as it generalizes Hitting Set), so no FPT algorithm parameterized by M is expected.
- **NP-completeness:** NP-complete [Lucchesi and Osborne, 1978], via transformation from VERTEX COVER.
- **References:**
  - C. L. Lucchesi and S. L. Osborne (1978). "Candidate keys for relations." *J. Computer and System Sciences*, 17(2), pp. 270-279.
  - W. Lipsky, Jr. (1977). "Two NP-complete problems related to information retrieval." *Fundamentals of Computation Theory*, Springer.

## Extra Remark

**Full book text:**

INSTANCE: A set A of "attribute names," a collection F of ordered pairs of subsets of A (called "functional dependencies" on A), and a positive integer M.
QUESTION: Is there a key of cardinality M or less for the relational system <A,F>, i.e., a minimal subset K ⊆ A with |K| <= M such that the ordered pair (K,A) belongs to the "closure" F* of F defined by (1) F ⊆ F*, (2) B ⊆ C ⊆ A implies (C,B) ∈ F*, (3) (B,C),(C,D) ∈ F* implies (B,D) ∈ F*, and (4) (B,C),(B,D) ∈ F* implies (B,C ∪ D) ∈ F*?
Reference: [Lucchesi and Osborne, 1977], [Lipsky, 1977a]. Transformation from VERTEX COVER. See [Date, 1975] for general background on relational data bases.

**Connection to Vertex Cover:** The Minimum Cardinality Key problem generalizes Vertex Cover. In the reduction, vertices become attributes, edges become functional dependencies (each endpoint determines the edge attribute), and a vertex cover corresponds to a set of attributes that determines all edge attributes. The key cardinality bound M corresponds directly to the vertex cover budget k.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all subsets of A of size <= M, compute attribute closure for each, check if closure equals A.
- [x] It can be solved by reducing to integer programming -- binary variable x_i per attribute; constraint that the closure of selected attributes covers all attributes; sum constraint sum(x_i) <= M. The closure constraint can be linearized using auxiliary variables.
- [x] Other: Lucchesi-Osborne algorithm enumerates all candidate keys in output-polynomial time. Can also reduce to Set Cover / Hitting Set and use known solvers.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has small key):**
Attributes A = {a, b, c, d, e, f} (6 attributes)
Functional dependencies F:
- {a, b} -> {c}
- {a, c} -> {d}
- {b, d} -> {e}
- {c, e} -> {f}

Budget M = 2

Analysis of candidate key {a, b}:
- Start: {a, b}
- Apply {a, b} -> {c}: closure = {a, b, c}
- Apply {a, c} -> {d}: closure = {a, b, c, d}
- Apply {b, d} -> {e}: closure = {a, b, c, d, e}
- Apply {c, e} -> {f}: closure = {a, b, c, d, e, f} = A
- {a, b} is a key of cardinality 2 <= M = 2.
- Check minimality: {a} alone: closure = {a} (no applicable FD). {b} alone: closure = {b}. Neither determines A.
- So {a, b} is a minimal key (candidate key) of size 2.

Answer: YES.

**Instance 2 (no small key):**
Attributes A = {a, b, c, d, e, f} (6 attributes)
Functional dependencies F:
- {a, b, c} -> {d}
- {d, e} -> {f}

Budget M = 2

No subset of size <= 2 can determine all of A:
- Any 2-element subset can derive at most {d} (if it contains {a,b} or {a,c} or {b,c}, still missing the third for the FD). Actually, {a,b,c} -> {d} requires all three.
- Even {a,b}: closure = {a,b} (cannot fire {a,b,c}->{d} without c). Not a key.
- No 2-element subset determines A.

Answer: NO.
