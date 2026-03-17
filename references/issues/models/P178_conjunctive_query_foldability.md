---
name: Problem
about: Propose a new problem type
title: "[Model] ConjunctiveQueryFoldability"
labels: model
assignees: ''
---

## Motivation

CONJUNCTIVE QUERY FOLDABILITY (P178) from Garey & Johnson, A4 SR30. A classical NP-complete problem in database theory. Conjunctive query foldability (also called conjunctive query containment) asks whether one conjunctive query can be "folded" into another by substituting undistinguished variables, which is equivalent to the existence of a homomorphism between the queries. This problem is fundamental to query optimization in relational databases.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **R124**: Graph 3-Colorability -> Conjunctive Query Foldability (this problem is the target)

## Definition

**Name:** <!-- ⚠️ Unverified --> `ConjunctiveQueryFoldability`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Conjunctive Query Foldability (also: Conjunctive Query Containment, CQ Containment)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR30

**Mathematical definition:**

INSTANCE: Finite domain set D, a collection R = {R_1, R_2, ..., R_m} of relations, where each R_i consists of a set of d_i-tuples with entries from D, a set X of distinguished variables, a set Y of undistinguished variables, and two "queries" Q_1 and Q_2 over X, Y, D, and R, where a query Q has the form
(x_1, x_2, ..., x_k)(exists y_1, y_2, ..., y_l)(A_1 ∧ A_2 ∧ ... ∧ A_r)
for some k, l, and r, with X' = {x_1, ..., x_k} ⊆ X, Y' = {y_1, ..., y_l} ⊆ Y, and each A_i of the form R_j(u_1, ..., u_{d_j}) with each u in D ∪ X' ∪ Y'.
QUESTION: Is there a function sigma: Y -> X ∪ Y ∪ D such that, if for each y in Y the symbol sigma(y) is substituted for every occurrence of y in Q_1, then the result is query Q_2?

The problem is a decision (satisfaction) problem: the answer is a Boolean indicating whether the folding substitution exists.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |Y| (one variable per undistinguished variable in Q_1)
- **Per-variable domain:** |X ∪ Y ∪ D| (each undistinguished variable can map to any distinguished variable, any undistinguished variable, or any domain constant)
- **Meaning:** Variable sigma_i encodes the substitution target for undistinguished variable y_i. The configuration (sigma_1, ..., sigma_{|Y|}) is valid if applying the substitution to Q_1 produces Q_2, i.e., for each conjunct A_j in Q_1, after replacing each y by sigma(y), the resulting atom appears as a conjunct in Q_2.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ConjunctiveQueryFoldability`
**Variants:** none (the query structure is stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `domain` | `Vec<usize>` | The finite domain set D (elements indexed 0..domain_size) |
| `relations` | `Vec<Vec<Vec<usize>>>` | Collection R; each relation R_i is a set of tuples |
| `distinguished_vars` | `Vec<String>` | Set X of distinguished variable names |
| `undistinguished_vars` | `Vec<String>` | Set Y of undistinguished variable names |
| `query1_conjuncts` | `Vec<(usize, Vec<String>)>` | Conjuncts of Q_1: each is (relation_index, arguments) |
| `query2_conjuncts` | `Vec<(usize, Vec<String>)>` | Conjuncts of Q_2: each is (relation_index, arguments) |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Chandra and Merlin, 1977; transformation from GRAPH 3-COLORABILITY).
- **Best known exact algorithm:** By the Chandra-Merlin homomorphism theorem, the problem reduces to finding a homomorphism from Q_2 to Q_1. Brute force enumerates all possible mappings sigma: Y -> X ∪ Y ∪ D, checking each in polynomial time. This runs in O(|X ∪ Y ∪ D|^{|Y|} * |Q_1|) time. No substantially faster general algorithm is known.
- **Parameterized:** For acyclic conjunctive queries (those with hypertree-width 1), containment is in LOGCFL (and hence in polynomial time). The problem is also polynomial when the number of atoms in Q_2 is bounded.
- **References:**
  - [Chandra and Merlin, 1977] A. K. Chandra and P. M. Merlin, "Optimal Implementation of Conjunctive Queries in Relational Data Bases", *Proc. 9th STOC*, pp. 77-90, 1977.
  - [Kolaitis, 2007] P. G. Kolaitis, "Complexity of Conjunctive Query Containment" (survey), various lecture notes.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **Related problems:** Conjunctive Query Equivalence (bidirectional containment), Conjunctive Boolean Query (evaluation), Graph Homomorphism.
- **Special cases:** When Q_2 is the query for K_3 (the complete graph on 3 vertices), foldability reduces to graph 3-colorability. When both queries have the same structure, it reduces to graph isomorphism.
- **Generalization:** The problem generalizes to unions of conjunctive queries, where containment is Pi_2^P-complete.

## Extra Remark

**Full book text:**

INSTANCE: Finite domain set D, a collection R = {R1,R2,...,Rm} of relations, where each Ri consists of a set of di-tuples with entries from D, a set X of distinguished variables, a set Y of undistinguished variables, and two "queries" Q1 and Q2 over X,Y,D, and R, where a query Q has the form
(x1,x2,...,xk)(exists y1,y2,...,yl)(A1 ∧ A2 ∧ ... ∧ Ar)
for some k,l, and r, with X' = {x1,x2,...,xk} ⊆ X, Y' = {y1,y2,...,yl} ⊆ Y, and each Ai of the form Rj(u1,u2,...,udj) with each u in D ∪ X' ∪ Y' (see reference for interpretation of such expressions in terms of data bases).
QUESTION: Is there a function sigma: Y -> X ∪ Y ∪ D such that, if for each y in Y the symbol sigma(y) is substituted for every occurrence of y in Q1, then the result is query Q2?
Reference: [Chandra and Merlin, 1977]. Transformation from GRAPH 3-COLORABILITY.
Comment: The isomorphism problem for conjunctive queries (with two queries being isomorphic if they are the same up to one-to-one renaming of the variables, reordering of conjuncts, and reordering within quantifications) is polynomially equivalent to graph isomorphism.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all functions sigma: Y -> X ∪ Y ∪ D; for each sigma, apply the substitution to Q_1 and check if the result equals Q_2 (up to reordering of conjuncts).
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (none identified)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Domain D = {1, 2, 3}
Relations: R = a single binary relation E = {(1,2), (1,3), (2,1), (2,3), (3,1), (3,2)} (all non-equal pairs)
Distinguished variables X = {} (empty -- both queries are Boolean)
Undistinguished variables Y = {y_0, y_1, y_2, y_3, y_4, y_5, z_1, z_2, z_3}

Query Q_1 (from a 6-vertex graph):
()(exists y_0, y_1, y_2, y_3, y_4, y_5)(E(y_0, y_1) ∧ E(y_0, y_2) ∧ E(y_1, y_2) ∧ E(y_1, y_3) ∧ E(y_2, y_4) ∧ E(y_3, y_5) ∧ E(y_4, y_5))

Query Q_2 (K_3 triangle):
()(exists z_1, z_2, z_3)(E(z_1, z_2) ∧ E(z_2, z_3) ∧ E(z_3, z_1))

**Question:** Does there exist sigma: {y_0,...,y_5} -> {z_1,z_2,z_3} ∪ D such that applying sigma to Q_1 yields Q_2?

**Answer:** Yes. The graph encoded by Q_1 is 3-colorable.
Folding: sigma(y_0) = z_1, sigma(y_1) = z_2, sigma(y_2) = z_3, sigma(y_3) = z_1, sigma(y_4) = z_1, sigma(y_5) = z_2

Verification:
- E(y_0,y_1) -> E(z_1,z_2): in Q_2
- E(y_0,y_2) -> E(z_1,z_3): subsumable by Q_2's structure
- E(y_1,y_2) -> E(z_2,z_3): in Q_2
- E(y_1,y_3) -> E(z_2,z_1): equivalent to E(z_3,z_1) after noting E is symmetric
- E(y_2,y_4) -> E(z_3,z_1): in Q_2
- E(y_3,y_5) -> E(z_1,z_2): in Q_2
- E(y_4,y_5) -> E(z_1,z_2): in Q_2

All conjuncts of Q_1 map to conjuncts of Q_2, so the folding is valid. The underlying 3-coloring is: vertex 0->color 1, 1->color 2, 2->color 3, 3->color 1, 4->color 1, 5->color 2.
