---
name: Problem
about: Propose a new problem type
title: "[Model] QuantifiedBooleanFormulas(qbf)(*)"
labels: model
assignees: ''
---

## Motivation

QUANTIFIED BOOLEAN FORMULAS (QBF) (*) (P263) from Garey & Johnson, A9 LO11. The canonical PSPACE-complete problem: given a fully quantified Boolean formula with alternating universal and existential quantifiers, determine whether it is true. QBF serves as the primary source of PSPACE-completeness reductions for combinatorial game problems and is the analogue of SAT for PSPACE.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- **As source:**
  - R182: QBF -> GENERALIZED HEX (PSPACE-completeness of the Shannon switching game on vertices)
  - R183: QBF -> GENERALIZED GEOGRAPHY (PSPACE-completeness of the move-based graph traversal game)
  - R184: QBF -> GENERALIZED KAYLES (PSPACE-completeness of the vertex-removal game)
  - R185: QBF -> SEQUENTIAL TRUTH ASSIGNMENT
  - R186: QBF -> VARIABLE PARTITION TRUTH ASSIGNMENT
  - R187: QBF -> SIFT
  - R188: QBF -> ALTERNATING HITTING SET
  - R189: QBF -> ALTERNATING MAXIMUM WEIGHTED MATCHING
  - R210: QBF -> MODAL LOGIC PROVABILITY
- **As target:**
  - R207: (generic transformation) -> QBF (Stockmeyer and Meyer, 1973)

## Definition

**Name:** `QuantifiedBooleanFormulas`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO11

**Mathematical definition:**

INSTANCE: Set U={u_1,u_2,...,u_n} of variables, well-formed quantified Boolean formula F=(Q_1u_1)(Q_2u_2)···(Q_nu_n)E, where E is a Boolean expression and each Q_i is either ∀ or ∃.
QUESTION: Is F true?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |U| (one variable per Boolean variable in the quantifier prefix)
- **Per-variable domain:** {0, 1} — representing FALSE and TRUE respectively
- **Meaning:** x_i in {0, 1} is the truth assignment for variable u_i. The formula F is true if there exists a strategy for the existential variables such that for all settings of the universal variables, the Boolean expression E evaluates to true under the combined assignment.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `QuantifiedBooleanFormulas`
**Variants:** none (operates on a general quantified Boolean formula)

| Field | Type | Description |
|-------|------|-------------|
| `num_vars` | `usize` | Number of variables n = \|U\| |
| `quantifiers` | `Vec<Quantifier>` | Quantifier for each variable: `Exists` or `ForAll`, length n |
| `clauses` | `Vec<Vec<Literal>>` | CNF representation of the Boolean expression E; each clause is a list of literals |

Where `Quantifier` is an enum `{ Exists, ForAll }` and `Literal` is a struct with fields `variable: usize` (0-indexed) and `negated: bool`.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The naive recursive algorithm evaluates QBF in O(2^n) time and O(n) space by branching on each quantified variable. For QBF with CNF matrix having m clauses, Williams (2002) achieves O(1.709^m) time. For general quantified CNFs of size poly(n) on n variables with O(1) quantifier alternations, recent results by Williams achieve 2^(n - n^{Omega(1)}) randomized time, the first known improvement over brute force for bounded-alternation QBF. The problem is PSPACE-complete (Stockmeyer and Meyer, 1973), so no polynomial-time algorithm is expected.

## Extra Remark

**Full book text:**

INSTANCE: Set U={u_1,u_2,...,u_n} of variables, well-formed quantified Boolean formula F=(Q_1u_1)(Q_2u_2)···(Q_nu_n)E, where E is a Boolean expression and each Q_i is either ∀ or ∃.
QUESTION: Is F true?
Reference: [Stockmeyer and Meyer, 1973]. Generic transformation.
Comment: PSPACE-complete, even if E is in conjunctive normal form with three literals per clause (QUANTIFIED 3SAT), but solvable in polynomial time when there are at most two literals per clause [Schaefer, 1978b]. If F is restricted to at most k alternations of quantifiers (i.e., there are at most k indices i such that Q_i≠Q_{i+1}), then the restricted problem is complete for some class in the polynomial hierarchy, depending on k and the allowed values for Q_1 (see Section 7.2).

## How to solve

- [x] It can be solved by (existing) bruteforce. (Recursively branch on each quantified variable: for EXISTS, accept if any branch is true; for FORALL, accept if all branches are true. Runtime O(2^n), space O(n).)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: QDPLL (generalization of DPLL for QBF with clause/cube learning), CEGAR-based QBF solvers.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
U = {u_1, u_2, u_3, u_4, u_5, u_6}
F = EXISTS u_1 FORALL u_2 EXISTS u_3 FORALL u_4 EXISTS u_5 FORALL u_6
    [(u_1 OR u_2 OR u_3) AND (NOT u_1 OR NOT u_4 OR u_5) AND (u_2 OR NOT u_3 OR u_6) AND (NOT u_5 OR NOT u_6 OR u_4)]

Quantifiers: [EXISTS, FORALL, EXISTS, FORALL, EXISTS, FORALL]
Clauses (CNF):
- C_1 = (u_1, u_2, u_3)
- C_2 = (NOT u_1, NOT u_4, u_5)
- C_3 = (u_2, NOT u_3, u_6)
- C_4 = (NOT u_5, NOT u_6, u_4)

**Evaluation:**
Set u_1 = TRUE. Then:
- C_1 = TRUE (u_1 = T). C_2 = (F OR NOT u_4 OR u_5).
- For any u_2:
  - Set u_3 = TRUE. C_3 = (u_2 OR F OR u_6). If u_2 = FALSE, need u_6 to be TRUE for C_3, but u_6 is universally quantified.
  - Set u_3 = FALSE instead. C_3 = (u_2 OR T OR u_6) = TRUE.
  - For any u_4:
    - C_2 = (F OR NOT u_4 OR u_5). If u_4 = TRUE, need u_5 = TRUE. If u_4 = FALSE, C_2 = TRUE.
    - Set u_5 = TRUE when u_4 = TRUE (C_2 = T), set u_5 = FALSE when u_4 = FALSE (C_2 = T).
    - C_4 = (NOT u_5 OR NOT u_6 OR u_4).
      - u_4 = TRUE, u_5 = TRUE: C_4 = (F OR NOT u_6 OR T) = TRUE for any u_6.
      - u_4 = FALSE, u_5 = FALSE: C_4 = (T OR NOT u_6 OR F) = TRUE for any u_6.

Answer: **TRUE** -- the existential player has a winning strategy.
