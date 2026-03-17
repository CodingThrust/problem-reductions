---
name: Problem
about: Propose a new problem type
title: "[Model] PrimeAttributeName"
labels: model
assignees: ''
---

## Motivation

PRIME ATTRIBUTE NAME (P176) from Garey & Johnson, A4 SR28. A classical NP-complete problem from relational database theory. An attribute is "prime" if it belongs to at least one candidate key. Determining whether a given attribute is prime is essential for database normalization: the distinction between prime and non-prime attributes is the foundation of Second Normal Form (2NF) and Third Normal Form (3NF). The NP-completeness of this problem means that even this basic classification task is computationally intractable in general.

**Associated rules:**
<!-- ⚠️ Unverified: AI-collected rule associations -->
- R122: Minimum Cardinality Key -> Prime Attribute Name (this model is the target)

## Definition

**Name:** <!-- ⚠️ Unverified --> `PrimeAttributeName`
**Canonical name:** Prime Attribute Name (also: Prime Attribute Testing)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR28, p.232

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a specified name x in A.
QUESTION: Is x a "prime attribute name" for <A,F>, i.e., is there a key K for <A,F> such that x in K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |A| binary variables, one per attribute name.
- **Per-variable domain:** binary {0, 1} -- whether the attribute is included in a candidate key K that contains x.
- **Meaning:** Variable y_i = 1 if attribute a_i is included in a candidate key. The configuration encodes a subset K ⊆ A. The assignment is valid if: (1) K is a key (determines all of A under F*), (2) K is minimal (no proper subset is also a key), and (3) x is in K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `PrimeAttributeName`
**Variants:** none (no graph or weight parameters)

| Field | Type | Description |
|-------|------|-------------|
| `num_attributes` | `usize` | Number of attributes |A| (attributes indexed 0..num_attributes) |
| `dependencies` | `Vec<(Vec<usize>, Vec<usize>)>` | Functional dependencies F; each pair (lhs, rhs) means lhs -> rhs |
| `query_attribute` | `usize` | The specified attribute x (index into the attribute set) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- No budget parameter is needed (unlike Minimum Cardinality Key).
- The problem asks only whether x appears in ANY candidate key, not whether x appears in a key of bounded size.
- An attribute is "non-prime" if it does not appear in any candidate key. Non-prime attributes are those that are functionally determined by every candidate key but never participate in one.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Enumerate all candidate keys using the Lucchesi-Osborne algorithm (polynomial per key), checking if any contain x. Worst case: exponentially many candidate keys, so O(2^|A| * |F| * |A|) in the worst case. However, one can terminate early as soon as any key containing x is found. A smarter approach: try all subsets containing x in increasing size order, checking if each is a key.
- **NP-completeness:** NP-complete [Lucchesi and Osborne, 1978], via transformation from MINIMUM CARDINALITY KEY.
- **Relationship to normal forms:** An attribute x is prime iff it is relevant to 2NF/3NF decomposition. A relation is in 3NF iff for every non-trivial FD X -> Y, either X is a superkey or Y consists only of prime attributes.
- **References:**
  - C. L. Lucchesi and S. L. Osborne (1978). "Candidate keys for relations." *J. Computer and System Sciences*, 17(2), pp. 270-279.

## Extra Remark

**Full book text:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a specified name x ∈ A.
QUESTION: Is x a "prime attribute name" for <A,F>, i.e., is there a key K for <A,F> such that x ∈ K?
Reference: [Lucchesi and Osborne, 1977]. Transformation from MINIMUM CARDINALITY KEY.

**Connection to normal forms:** In database normalization theory:
- A "prime attribute" is an attribute that belongs to at least one candidate key.
- A "non-prime attribute" belongs to no candidate key.
- 2NF requires that no non-prime attribute is partially dependent on any candidate key.
- 3NF requires that for every non-trivial FD X -> A, either X is a superkey or A is a prime attribute.
- Since determining whether an attribute is prime is NP-complete, checking 2NF and 3NF is also intractable in general.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all subsets of A containing x, check each for key property (closure = A, minimality).
- [x] It can be solved by reducing to integer programming -- binary variable per attribute; constraint x = 1; constraint that closure covers A; minimality constraint; feasibility check.
- [x] Other: Use Lucchesi-Osborne key enumeration with early termination when a key containing x is found. Can also reduce to SAT.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (x is prime):**
Attributes A = {a, b, c, d, e, f} (6 attributes)
Functional dependencies F:
- {a, b} -> {c, d, e, f}
- {c, d} -> {a, b, e, f}
- {a, d} -> {b, c, e, f}
Query attribute: x = d

Analysis of candidate keys:
- {a, b}: closure = {a,b,c,d,e,f} = A. Key. Does NOT contain d.
- {c, d}: closure = {c,d,a,b,e,f} = A. Key. Contains d.

Since {c, d} is a candidate key containing d, the attribute d is prime.

Answer: YES.

**Instance 2 (x is not prime):**
Attributes A = {a, b, c, d, e, f} (6 attributes)
Functional dependencies F:
- {a, b} -> {c, d, e, f}

Query attribute: x = d

Analysis of candidate keys:
- {a, b}: closure = {a,b,c,d,e,f} = A. Key. Minimal (neither {a} nor {b} determines A). Does NOT contain d.
- Check all other pairs: {a,c}: closure = {a,c}. {a,d}: closure = {a,d}. {a,e}: closure = {a,e}. {a,f}: closure = {a,f}. {b,c}: closure = {b,c}. {b,d}: closure = {b,d}. {b,e}: closure = {b,e}. {b,f}: closure = {b,f}. {c,d}: closure = {c,d}. None of these determine A.
- Single attributes: none determine A.
- Triples containing d: {a,b,d} is a superkey but not minimal (since {a,b} is already a key). {a,c,d}: closure = {a,c,d}. {a,d,e}: closure = {a,d,e}. {b,c,d}: closure = {b,c,d}. None are keys except supersets of {a,b}.
- The only candidate key is {a, b}, which does not contain d.

Answer: NO, d is not a prime attribute.

**Instance 3 (non-trivial, multiple keys):**
Attributes A = {a, b, c, d, e, f, g, h} (8 attributes)
Functional dependencies F:
- {a, b} -> {c}
- {c, d} -> {e, f}
- {a, d} -> {g}
- {b, g} -> {h}
- {e, h} -> {a, b}

Query attribute: x = e

Analysis:
- Key {a, b, d}: closure -> {a,b,c} -> {c,d,e,f} -> {a,d,g} -> {b,g,h} -> all = A. Key. Does NOT contain e.
- Key {d, e, h}: {e,h} -> {a,b} -> {c} -> {c,d} -> {e,f} -> {a,d} -> {g} -> {b,g} -> {h}. Closure = A. Contains e. But is it minimal? Check {e,h}: closure = {e,h,a,b} -> {c} -> {c,d}? No, d is not derivable from {a,b,c,e,h}. So {e,h} does not determine A. Check {d,e}: closure = {d,e} -- no FD fires. Check {d,h}: closure = {d,h} -- no FD fires. So {d,e,h} is minimal.

Since {d, e, h} is a candidate key containing e, the attribute e is prime.

Answer: YES.
