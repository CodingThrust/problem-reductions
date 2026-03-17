---
name: Problem
about: Propose a new problem type
title: "[Model] BoyceCoddNormalFormViolation"
labels: model
assignees: ''
---

## Motivation

BOYCE-CODD NORMAL FORM VIOLATION (P177) from Garey & Johnson, A4 SR29. A classical NP-complete problem in database theory. Determining whether a subset of attributes violates Boyce-Codd normal form is fundamental to relational database schema design and normalization.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **R123**: Hitting Set -> Boyce-Codd Normal Form Violation (this problem is the target)

## Definition

**Name:** <!-- ⚠️ Unverified --> `BoyceCoddNormalFormViolation`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Boyce-Codd Normal Form Violation (also: BCNF Violation, BCNF Testing)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR29

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a subset A' ⊆ A.
QUESTION: Does A' violate Boyce-Codd normal form for the relational system <A,F>, i.e., is there a subset X ⊆ A' and two attribute names y,z in A' - X such that (X,{y}) in F* and (X,{z}) not in F*, where F* is the closure of F?

The problem is a decision (satisfaction) problem: the answer is a Boolean indicating whether the BCNF condition is violated.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 2^|A'| possible subsets X, times |A'|^2 possible (y,z) pairs -- but as a search problem, the configuration space can be modeled as selecting a subset X of A' plus two attributes y, z from A' - X.
- **Per-variable domain:** For a binary encoding, one binary variable per attribute in A' indicating membership in X, plus selection of y and z from remaining attributes.
- **Meaning:** The configuration encodes a candidate triple (X, y, z) where X ⊆ A' and y, z in A' - X. The assignment is valid (violation exists) if (X, {y}) is in F* (X functionally determines y) and (X, {z}) is not in F* (X does not functionally determine z), meaning X is not a superkey of A'.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `BoyceCoddNormalFormViolation`
**Variants:** none (no graph or weight type parameter; functional dependencies are stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `num_attributes` | `usize` | Number of attributes in A (attributes indexed 0..num_attributes) |
| `functional_deps` | `Vec<(Vec<usize>, Vec<usize>)>` | Collection F of functional dependencies; each is (lhs_attributes, rhs_attributes) |
| `target_subset` | `Vec<usize>` | The subset A' ⊆ A to test for BCNF violation |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Beeri and Bernstein, 1979; transformation from HITTING SET).
- **Best known exact algorithm:** Brute-force: enumerate all subsets X of A', and for each X check whether there exist y, z in A' - X with (X,{y}) in F* and (X,{z}) not in F*. Computing F* (the closure of a set of attributes under F) is polynomial (linear time per closure computation). The overall brute force runs in O(2^|A'| * |A'|^2 * |F|) time. No substantially better exact algorithm is known for the general case.
- **Parameterized:** The problem remains NP-complete even when A' is required to satisfy third normal form (3NF). For the special case where functional dependencies form a hierarchy, BCNF testing is linear time.
- **References:**
  - [Beeri and Bernstein, 1979] C. Beeri and P. A. Bernstein, "Computational Problems Related to the Design of Normal Form Relational Schemas", *ACM Trans. Database Systems*, 4(1), pp. 30-59, 1979.
  - [Bernstein and Beeri, 1976] P. A. Bernstein and C. Beeri, "An Algorithmic Approach to Normalization of Relational Database Schemas", University of Toronto, 1976.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **Related problems:** The problem is closely related to key determination and superkey testing for relational schemas.
- **Restriction:** When all functional dependencies have single-attribute right-hand sides and the dependency graph is hierarchical, the problem is solvable in polynomial time.
- **Generalization of:** Testing whether a specific FD violates BCNF (which is polynomial) to testing whether any violation exists in a given attribute subset.

## Extra Remark

**Full book text:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a subset A' ⊆ A.
QUESTION: Does A' violate Boyce-Codd normal form for the relational system <A,F>, i.e., is there a subset X ⊆ A' and two attribute names y,z in A' - X such that (X,{y}) in F* and (X,{z}) not in F*, where F* is the closure of F?
Reference: [Bernstein and Beeri, 1976], [Beeri and Bernstein, 1978]. Transformation from HITTING SET.
Comment: Remains NP-complete even if A' is required to satisfy "third normal form," i.e., if X ⊆ A' is a key for the system <A',F> and if two names y,z in A'-X satisfy (X,{y}) in F* and (X,{z}) not in F*, then z is a prime attribute for <A',F>.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all subsets X of A'; for each X and each pair (y, z) in A' - X, compute the attribute closure X+ under F and check if y in X+ but z not in X+.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (none identified)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Attribute set A = {a, b, c, d, e, f} (6 attributes, indexed 0..5 as a=0, b=1, c=2, d=3, e=4, f=5)

Functional dependencies F:
- FD1: {a, b} -> {c} (attributes 0,1 determine attribute 2)
- FD2: {c} -> {d} (attribute 2 determines attribute 3)
- FD3: {d, e} -> {f} (attributes 3,4 determine attribute 5)
- FD4: {a, b, e} -> {f} (attributes 0,1,4 determine attribute 5 -- derived from FD1, FD2, FD3)

Target subset A' = {a, b, c, d, e, f} (all attributes)

**BCNF Violation analysis:**
The key for <A', F> is {a, b, e} (closure: {a,b,e} -> {c} via FD1 -> {d} via FD2 -> {f} via FD3, giving {a,b,c,d,e,f} = A').

Consider X = {c}, y = d, z = a:
- Closure of {c} under F: {c} -> {d} via FD2, so {c}+ = {c, d}
- (X, {y}) = ({c}, {d}): d in {c}+ = {c, d}, so ({c}, {d}) in F*
- (X, {z}) = ({c}, {a}): a not in {c}+ = {c, d}, so ({c}, {a}) not in F*
- But {c} is NOT a superkey of A' (its closure is only {c, d}, not all of A')
- Therefore, X = {c} with y = d, z = a witnesses a BCNF violation

**Verification:** The FD c -> d violates BCNF because the left-hand side {c} is not a superkey of the relation. This is the classic BCNF violation pattern: a non-trivial FD whose determinant is not a superkey.
