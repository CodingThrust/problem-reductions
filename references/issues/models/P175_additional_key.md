---
name: Problem
about: Propose a new problem type
title: "[Model] AdditionalKey"
labels: model
assignees: ''
---

## Motivation

ADDITIONAL KEY (P175) from Garey & Johnson, A4 SR27. A classical NP-complete problem from relational database theory that asks whether a relational schema has a candidate key not already in a given set of known keys. This problem is central to database normalization: when designing schemas, one must enumerate all candidate keys to determine normal forms (especially BCNF). The NP-completeness of this problem means that verifying completeness of key enumeration is computationally intractable.

**Associated rules:**
<!-- ⚠️ Unverified: AI-collected rule associations -->
- R121: Hitting Set -> Additional Key (this model is the target)

## Definition

**Name:** <!-- ⚠️ Unverified --> `AdditionalKey`
**Canonical name:** Additional Key
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR27, p.232

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, a subset R ⊆ A, and a set K of keys for the relational scheme <R,F>.
QUESTION: Does R have a key not already contained in K, i.e., is there an R' ⊆ R such that R' not in K, (R',R) in F*, and for no R'' strictly contained in R' is (R'',R) in F*?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |R| binary variables, one per attribute in the relation scheme R.
- **Per-variable domain:** binary {0, 1} -- whether the attribute is included in the candidate key R'.
- **Meaning:** Variable x_i = 1 if attribute a_i in R is included in the candidate key R'. The configuration encodes a subset R' ⊆ R. The assignment is valid if: (1) R' determines all of R under F* (i.e., (R',R) in F*), (2) R' is minimal (no proper subset also determines R), and (3) R' is not already in K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `AdditionalKey`
**Variants:** none (no graph or weight parameters)

| Field | Type | Description |
|-------|------|-------------|
| `num_attributes` | `usize` | Number of attributes in A (attributes indexed 0..num_attributes) |
| `dependencies` | `Vec<(Vec<usize>, Vec<usize>)>` | Functional dependencies F; each pair (lhs, rhs) means lhs -> rhs |
| `relation_attrs` | `Vec<usize>` | Subset R ⊆ A: the relation scheme attributes |
| `known_keys` | `Vec<Vec<usize>>` | Set K of known candidate keys for <R,F> |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- The answer is YES if and only if there exists a candidate key for <R,F> that is not in the known set K.
- The problem is related to key enumeration: if the Lucchesi-Osborne algorithm finds all keys, then checking if any are missing from K is straightforward, but the enumeration itself may produce exponentially many keys.
- The input includes both the full attribute set A (for functional dependency context) and the relation subset R.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Enumerate all candidate keys using the Lucchesi-Osborne algorithm (polynomial per key), then check if any key is absent from K. Worst case: exponentially many candidate keys, so O(2^|R| * |F| * |A|). For small K, one can also try to construct a new key by testing subsets of R not in K.
- **NP-completeness:** NP-complete [Beeri and Bernstein, 1979], via transformation from HITTING SET.
- **Relationship to Hitting Set:** The reduction from Hitting Set encodes the covering constraint as a key-determination constraint, making the additional key search equivalent to finding an uncovered hitting set.
- **References:**
  - C. Beeri and P. A. Bernstein (1979). "Computational problems related to the design of normal form relational schemas." *ACM Transactions on Database Systems*, 4(1), pp. 30-59.

## Extra Remark

**Full book text:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, a subset R ⊆ A, and a set K of keys for the relational scheme <R,F>.
QUESTION: Does R have a key not already contained in K, i.e., is there an R' ⊆ R such that R' not in K, (R',R) ∈ F*, and for no R'' ⊆ R' is (R'',R) ∈ F*?
Reference: [Beeri and Bernstein, 1978]. Transformation from HITTING SET.

**Connection to BCNF normalization:** A relation scheme <R,F> is in Boyce-Codd Normal Form (BCNF) if and only if every non-trivial functional dependency X -> Y has X as a superkey. Checking BCNF requires knowing all candidate keys, which in turn requires solving the Additional Key problem to ensure no keys have been missed. The NP-completeness of Additional Key implies that BCNF testing is also intractable in general (Beeri and Bernstein, 1979).

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all subsets of R, check each for key property (closure = R, minimality), and verify it is not in K.
- [x] It can be solved by reducing to integer programming -- binary variable per attribute in R; constraint that closure covers R; constraint that the selected set is not equal to any known key (expressed via at least one differing attribute per known key); minimality as additional constraint.
- [x] Other: Use Lucchesi-Osborne key enumeration algorithm to find all keys, then check against K. Alternatively, reduce to SAT: encode closure computation, minimality, and exclusion of known keys as Boolean constraints.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (additional key exists):**
Attributes A = {a, b, c, d, e, f} (6 attributes)
R = A (full relation)
Functional dependencies F:
- {a, b} -> {c, d, e, f}
- {c, d} -> {a, b, e, f}
- {a, c} -> {b, d, e, f}

Known keys K = {{a, b}, {c, d}}

Analysis:
- {a, b}: closure = {a,b} -> apply {a,b}->{c,d,e,f} -> {a,b,c,d,e,f} = A. Key. Minimal (neither {a} nor {b} alone determines A).
- {c, d}: closure = {c,d} -> apply {c,d}->{a,b,e,f} -> {a,b,c,d,e,f} = A. Key. Minimal.
- {a, c}: closure = {a,c} -> apply {a,c}->{b,d,e,f} -> {a,b,c,d,e,f} = A. Key. Minimal (neither {a} nor {c} alone determines A).
- {a, c} is not in K = {{a,b}, {c,d}}.

Answer: YES, {a, c} is an additional key not in K.

**Instance 2 (no additional key):**
Attributes A = {a, b, c} (3 attributes)
R = A
Functional dependencies F:
- {a} -> {b, c}

Known keys K = {{a}}

Analysis:
- {a}: closure = {a,b,c} = A. Key.
- {b}: closure = {b}. Not a key.
- {c}: closure = {c}. Not a key.
- {a,b}: contains {a} which is already a key, so {a,b} is not minimal (it's a superkey but not a candidate key).
- {a,c}: same, contains {a}.
- {b,c}: closure = {b,c}. Not a key.
- The only candidate key is {a}, which is already in K.

Answer: NO, no additional key exists.
