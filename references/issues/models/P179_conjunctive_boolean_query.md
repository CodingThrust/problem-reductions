---
name: Problem
about: Propose a new problem type
title: "[Model] ConjunctiveBooleanQuery"
labels: model
assignees: ''
---

## Motivation

CONJUNCTIVE BOOLEAN QUERY (P179) from Garey & Johnson, A4 SR31. A classical NP-complete problem in database theory. Evaluating a conjunctive Boolean query over a relational database asks whether a given existentially quantified conjunction of relational atoms is true. This is the most basic query evaluation problem in databases and is equivalent to the constraint satisfaction problem and the graph homomorphism problem.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **R125**: Clique -> Conjunctive Boolean Query (this problem is the target)

## Definition

**Name:** <!-- ⚠️ Unverified --> `ConjunctiveBooleanQuery`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Conjunctive Boolean Query (also: Boolean Conjunctive Query Evaluation, BCQ, CQ Evaluation)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR31

**Mathematical definition:**

INSTANCE: Finite domain set D, a collection R = {R_1, R_2, ..., R_m} of relations, where each R_i consists of a set of d_i-tuples with entries from D, and a conjunctive Boolean query Q over R and D, where such a query Q is of the form
(exists y_1, y_2, ..., y_l)(A_1 ∧ A_2 ∧ ... ∧ A_r)
with each A_i of the form R_j(u_1, u_2, ..., u_{d_j}) where each u in {y_1, ..., y_l} ∪ D.
QUESTION: Is Q, when interpreted as a statement about R and D, true?

The problem is a decision (satisfaction) problem: the answer is a Boolean.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** l (number of existentially quantified variables y_1, ..., y_l)
- **Per-variable domain:** |D| (each variable can take any value from the domain D)
- **Meaning:** Variable y_i is assigned a value from D. The configuration (y_1 = d_1, ..., y_l = d_l) is valid (query is true) if for every conjunct A_j = R_k(u_1, ..., u_{d_k}), the tuple obtained by substituting each y_i with d_i is present in relation R_k.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ConjunctiveBooleanQuery`
**Variants:** none (the query and database are stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `domain_size` | `usize` | Size of the finite domain D (elements indexed 0..domain_size) |
| `relations` | `Vec<Vec<Vec<usize>>>` | Collection R; each relation R_i is a set of tuples (Vec of d_i-tuples) |
| `num_variables` | `usize` | Number of existentially quantified variables l |
| `conjuncts` | `Vec<(usize, Vec<QueryArg>)>` | Query conjuncts: each is (relation_index, arguments) where arguments are Variable(idx) or Constant(val) |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Chandra and Merlin, 1977; transformation from CLIQUE).
- **Best known exact algorithm:** Brute-force enumeration of all |D|^l assignments of the l variables to domain elements, checking each assignment against all r conjuncts. This runs in O(|D|^l * r * max_arity) time. No substantially faster general algorithm is known.
- **Parameterized:** For queries of bounded treewidth (or more generally, bounded hypertree-width), evaluation is in polynomial time (combined complexity). Specifically, acyclic conjunctive queries can be evaluated in O(|D| * |Q| * |R|) time using Yannakakis's algorithm. The problem is W[1]-hard parameterized by the number of variables.
- **References:**
  - [Chandra and Merlin, 1977] A. K. Chandra and P. M. Merlin, "Optimal Implementation of Conjunctive Queries in Relational Data Bases", *Proc. 9th STOC*, pp. 77-90, 1977.
  - [Yannakakis, 1981] M. Yannakakis, "Algorithms for Acyclic Database Schemes", *Proc. VLDB*, pp. 82-94, 1981.
  - [Grohe, 2007] M. Grohe, "The Complexity of Homomorphism and Constraint Satisfaction Problems Seen from the Other Side", *J. ACM*, 54(1), Article 1, 2007.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **Related problems:** Conjunctive Query Foldability (containment), Constraint Satisfaction Problem (CSP), Graph Homomorphism, Clique.
- **Special cases:** When the query is the k-clique query (k existential variables, C(k,2) binary conjuncts), the problem is exactly k-CLIQUE. When R has a single binary relation and the query is a triangle query, it is TRIANGLE DETECTION.
- **Generalization:** Replacing the conjunctive query by an arbitrary first-order sentence makes the problem PSPACE-complete, even for D = {0,1}.

## Extra Remark

**Full book text:**

INSTANCE: Finite domain set D, a collection R = {R1,R2,...,Rm} of relations, where each Ri consists of a set of di-tuples with entries from D, and a conjunctive Boolean query Q over R and D, where such a query Q is of the form
(exists y1,y2,...,yl)(A1 ∧ A2 ∧ ... ∧ Ar)
with each Ai of the form Rj(u1,u2,...,udj) where each u in {y1,y2,...,yl} ∪ D.
QUESTION: Is Q, when interpreted as a statement about R and D, true?
Reference: [Chandra and Merlin, 1977]. Transformation from CLIQUE.
Comment: If we are allowed to replace the conjunctive query Q by an arbitrary first-order sentence involving the predicates in R, then the problem becomes PSPACE-complete, even for D = {0,1}.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all |D|^l assignments of the l existential variables to domain values; for each assignment, check whether every conjunct A_i is satisfied (i.e., the resulting tuple is in the appropriate relation R_j).
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Can be solved by reducing to constraint satisfaction problem (CSP) formulations, or by graph homomorphism algorithms for the special case of a single binary relation.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Domain D = {0, 1, 2, 3, 4, 5, 6} (7 elements)

Relations: R = a single binary relation E (the edge set of a graph):
E = {(0,1),(1,0), (0,2),(2,0), (0,3),(3,0), (1,2),(2,1), (1,3),(3,1), (2,3),(3,2), (2,4),(4,2), (3,5),(5,3), (4,5),(5,4), (4,6),(6,4), (5,6),(6,5), (1,6),(6,1)}
(12 undirected edges, 24 directed tuples)

Query Q (4-clique query):
(exists y_1, y_2, y_3, y_4)(E(y_1,y_2) ∧ E(y_1,y_3) ∧ E(y_1,y_4) ∧ E(y_2,y_3) ∧ E(y_2,y_4) ∧ E(y_3,y_4))

4 existential variables, 6 conjuncts (one per pair).

**Answer:** TRUE

**Satisfying assignment:** y_1=0, y_2=1, y_3=2, y_4=3

Verification:
- E(0,1): (0,1) in E
- E(0,2): (0,2) in E
- E(0,3): (0,3) in E
- E(1,2): (1,2) in E
- E(1,3): (1,3) in E
- E(2,3): (2,3) in E
All 6 conjuncts satisfied. Vertices {0,1,2,3} form a 4-clique in the underlying graph.

**Non-existence check:** The query for k=5 (5-clique) would be FALSE on this graph, since the maximum clique size is 4.

**Greedy trap:** Starting from vertex 4 (degree 3: neighbors 2, 5, 6), one might check {4,5,6} which is only a triangle. The 4-clique {0,1,2,3} is found only by examining vertices with higher mutual adjacency.
