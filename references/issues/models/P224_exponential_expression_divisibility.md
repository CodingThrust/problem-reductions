---
name: Problem
about: Propose a new problem type
title: "[Model] ExponentialExpressionDivisibility"
labels: model
assignees: ''
---

## Motivation

EXPONENTIAL EXPRESSION DIVISIBILITY (P224) from Garey & Johnson, A7 AN5. A classical NP-hard problem connecting number theory and computational complexity. Given two products of terms of the form (q^a - 1), asks whether one divides the other. Not known to be in NP or co-NP, but solvable in pseudo-polynomial time. Remains NP-hard for any fixed q with |q| > 1.

**Associated rules:**
<!-- ⚠️ Unverified: AI-collected rule associations -->
- R168: 3SAT -> Exponential Expression Divisibility (source: Plaisted, 1976)

## Definition

**Name:** `ExponentialExpressionDivisibility`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN5

**Mathematical definition:**

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers, and an integer q.
QUESTION: Does product_{i=1}^{n} (q^{a_i} - 1) divide product_{j=1}^{m} (q^{b_j} - 1)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 0 (this is a pure decision problem with no configuration variables to optimize over; the answer is determined entirely by the input parameters)
- **Per-variable domain:** N/A
- **Meaning:** The problem is a satisfaction problem: given fixed sequences a and b and base q, determine a divisibility relation. There are no variables to assign; the answer is a function of the input alone. In a brute-force setting, one could search over all prime power factorizations, but there is no natural binary configuration space.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ExponentialExpressionDivisibility`
**Variants:** none (no type parameters)

| Field | Type | Description |
|-------|------|-------------|
| `a_seq` | `Vec<u64>` | Sequence a_1, ..., a_n of positive integers (exponents in numerator) |
| `b_seq` | `Vec<u64>` | Sequence b_1, ..., b_m of positive integers (exponents in denominator) |
| `q` | `i64` | Base integer q (with \|q\| > 1 for NP-hardness) |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Solvable in pseudo-polynomial time using standard GCD algorithms on the integer products. The divisibility product_{i=1}^n (q^{a_i} - 1) | product_{j=1}^m (q^{b_j} - 1) can be checked by computing the actual integer values and testing divisibility, but these integers can be exponentially large in the input. Using cyclotomic polynomial factorization (q^a - 1 = product_{d|a} Phi_d(q)), the problem reduces to checking multiplicity conditions on cyclotomic factors. For fixed q, this yields a pseudo-polynomial algorithm. The problem is NP-hard but not known to be in NP (since certifying divisibility of large integers is non-trivial). No strongly polynomial algorithm is known. [Plaisted, FOCS 1976]

## Extra Remark

**Full book text:**

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers, and an integer q.
QUESTION: Does product_{i=1}^{n} (q^{a_i} - 1) divide product_{j=1}^{m} (q^{b_j} - 1)?

Reference: [Plaisted, 1976]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP, but solvable in pseudo-polynomial time using standard greatest common divisor algorithms. Remains NP-hard for any fixed value of q with |q| > 1, even if the a_i and b_j are restricted to being products of distinct primes.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Pseudo-polynomial GCD-based algorithm. Factor each q^a - 1 using cyclotomic decomposition and compare multiplicities.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
- a_seq = [2, 3] (n = 2)
- b_seq = [6] (m = 1)
- q = 2

**Computation:**
- Numerator: (2^2 - 1) * (2^3 - 1) = 3 * 7 = 21
- Denominator: (2^6 - 1) = 63

**Check:** Does 21 divide 63? 63 / 21 = 3. YES.

**Cyclotomic verification:**
- 2^2 - 1 = Phi_1(2) * Phi_2(2) = 1 * 3 = 3
- 2^3 - 1 = Phi_1(2) * Phi_3(2) = 1 * 7 = 7
- 2^6 - 1 = Phi_1(2) * Phi_2(2) * Phi_3(2) * Phi_6(2) = 1 * 3 * 7 * 3 = 63
- Numerator cyclotomic factors: Phi_1^2 * Phi_2 * Phi_3
- Denominator cyclotomic factors: Phi_1 * Phi_2 * Phi_3 * Phi_6
- The numerator has Phi_1 with multiplicity 2, but denominator has Phi_1 with multiplicity 1, so 21 does NOT divide 63 by cyclotomic analysis... but 63/21 = 3 exactly. The resolution: Phi_1(2) = 1, so extra Phi_1 factors contribute 1, and the integer divisibility holds.

**Another example (NO answer):**
- a_seq = [4, 3] (n = 2)
- b_seq = [6] (m = 1)
- q = 2

- Numerator: (2^4 - 1) * (2^3 - 1) = 15 * 7 = 105
- Denominator: (2^6 - 1) = 63
- Does 105 divide 63? 63 / 105 is not an integer. NO.
