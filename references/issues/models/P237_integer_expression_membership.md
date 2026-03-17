---
name: Problem
about: Propose a new problem type
title: "[Model] IntegerExpressionMembership"
labels: model
assignees: ''
---

## Motivation

INTEGER EXPRESSION MEMBERSHIP (P237) from Garey & Johnson, A7 AN18. Given an integer expression built using union (∪) and set addition (+, Minkowski sum) over singleton positive integers, and a target integer K, the problem asks whether K belongs to the set denoted by the expression. NP-complete by reduction from SUBSET SUM (Stockmeyer and Meyer, 1973). Part of a family of problems with varying complexity: the related INEQUIVALENCE problem (do two expressions denote different sets?) is Sigma_2^p-complete, and adding complementation (¬) makes both MEMBERSHIP and INEQUIVALENCE PSPACE-complete. This problem connects formal language theory and arithmetic circuit complexity.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R181 (SUBSET SUM to INTEGER EXPRESSION MEMBERSHIP)
- As source: (none known)

## Definition

**Name:** `IntegerExpressionMembership`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Integer Expression Membership
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN18

**Mathematical definition:**

INSTANCE: Integer expression e over the operations ∪ and +, where if n ∈ Z^+, the binary representation of n is an integer expression representing n, and if f and g are integer expressions representing the sets F and G, then f ∪ g is an integer expression representing the set F ∪ G and f + g is an integer expression representing the set {m + n: m ∈ F and n ∈ G}, and a positive integer K.
QUESTION: Is K in the set represented by e?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** Depends on the expression structure. If the expression has d union-choice points, the decision tree has up to 2^d leaves. The "variables" correspond to the choices made at each "+" node (which element from each operand set to select).
- **Per-variable domain:** For each "+" node, the choice is which element from the left operand set and which from the right operand set to contribute to the sum.
- **Meaning:** The problem asks whether there exists a sequence of choices through the expression tree (selecting one element at each union, and summing at each "+") that produces the target K. This is inherently a search/decision problem over the expression's structure.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `IntegerExpressionMembership`
**Variants:** none

| Field        | Type              | Description                                                      |
|--------------|-------------------|------------------------------------------------------------------|
| `expression` | `IntExpr`         | Integer expression tree over ∪ and + operations                  |
| `target`     | `u64`             | Positive integer K to test for membership                        |

Where `IntExpr` is a recursive type:
- `Atom(u64)` -- a positive integer literal
- `Union(Box<IntExpr>, Box<IntExpr>)` -- set union F ∪ G
- `Sum(Box<IntExpr>, Box<IntExpr>)` -- Minkowski sum {m+n : m ∈ F, n ∈ G}

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** NP-complete (Stockmeyer and Meyer, 1973). The set denoted by an expression of size s can have up to 2^s elements, so explicit enumeration is exponential. For expressions using only ∪ and +, a dynamic programming approach can evaluate the represented set, but the set size can be exponential in the expression size. The problem is in NP because a witness consists of a "path" through the expression tree (choosing one branch at each union node) that produces a sum equaling K; this witness has size linear in the expression. Adding complementation (¬) makes the problem PSPACE-complete. [Stockmeyer & Meyer, STOC 1973, pp. 1-9; Stockmeyer, TCS 3:1-22, 1976.]

## Specialization

<!-- ⚠️ Unverified: AI-generated specialization -->

- SUBSET SUM is a special case: given set {a_1, ..., a_n} and target B, the expression ({0} ∪ {a_1}) + ({0} ∪ {a_2}) + ... + ({0} ∪ {a_n}) with target B encodes the subset sum question (using a shifted encoding since atoms must be positive).
- INTEGER EXPRESSION INEQUIVALENCE (do two expressions denote different sets?) is Sigma_2^p-complete with ∪ and +.
- With ∪, +, and ¬: both MEMBERSHIP and INEQUIVALENCE become PSPACE-complete.

## Extra Remark

**Full book text:**

INSTANCE: Integer expression e over the operations ∪ and +, where if n ∈ Z^+, the binary representation of n is an integer expression representing n, and if f and g are integer expressions representing the sets F and G, then f ∪ g is an integer expression representing the set F ∪ G and f + g is an integer expression representing the set {m + n: m ∈ F and n ∈ G}, and a positive integer K.
QUESTION: Is K in the set represented by e?

Reference: [Stockmeyer and Meyer, 1973]. Transformation from SUBSET SUM.
Comment: The related INTEGER EXPRESSION INEQUIVALENCE problem, "given two integer expressions e and f, do they represent different sets?" is NP-hard and in fact complete for Σ_2^p in the polynomial hierarchy ([Stockmeyer and Meyer, 1973], [Stockmeyer, 1976a], see also Section 7.2). If the operator "¬" is allowed, with ¬e representing the set of all positive integers not represented by e, then both the membership and inequivalence problems become PSPACE-complete [Stockmeyer and Meyer, 1973].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Recursively evaluate the set represented by the expression, then check if K is in it. Feasible for small expressions but the set can be exponentially large.)
- [ ] It can be solved by reducing to integer programming. (Encode each union choice as a binary variable and each sum as a linear constraint; the target K must equal the total sum of selected atoms.)
- [ ] Other: For expressions of bounded depth, dynamic programming on reachable sums at each node. Pseudo-polynomial when atom values are small.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Expression: e = (2 ∪ 5) + (3 ∪ 7)
Target: K = 10

**Analysis:**
The set represented by (2 ∪ 5) is {2, 5}.
The set represented by (3 ∪ 7) is {3, 7}.
The Minkowski sum {2, 5} + {3, 7} = {2+3, 2+7, 5+3, 5+7} = {5, 9, 8, 12}.
Set = {5, 8, 9, 12}.

Is K = 10 in {5, 8, 9, 12}? **NO**.

**Another example (YES answer):**
Expression: e = (1 ∪ 4) + (3 ∪ 6) + (2 ∪ 5)
Target: K = 12

Set computation:
- {1, 4} + {3, 6} = {4, 7, 7, 10} = {4, 7, 10}
- {4, 7, 10} + {2, 5} = {6, 9, 9, 12, 12, 15} = {6, 9, 12, 15}

Is K = 12 in {6, 9, 12, 15}? **YES** ✓

Witness path: choose 4 from (1 ∪ 4), choose 6 from (3 ∪ 6), choose 2 from (2 ∪ 5). Sum = 4 + 6 + 2 = 12 = K.
