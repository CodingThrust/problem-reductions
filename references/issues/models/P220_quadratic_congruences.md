---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticCongruences"
labels: model
assignees: ''
---

## Motivation

QUADRATIC CONGRUENCES (P220) from Garey & Johnson, A7 AN1. A classical NP-complete problem in number theory: given positive integers a, b, c, determine if there exists a positive integer x < c with x^2 ≡ a (mod b). The problem is NP-complete due to the upper bound constraint x < c; without this bound (c = infinity), the problem is polynomial-time solvable given the factorization of b. This problem is a cornerstone result by Manders and Adleman (1978) connecting computational complexity to quadratic residuosity and Diophantine equations.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** (none known in GJ appendix)
- **As target:** R164 (3SAT -> QUADRATIC CONGRUENCES)

## Definition

**Name:** `QuadraticCongruences`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN1

**Mathematical definition:**

INSTANCE: Positive integers a, b, and c.
QUESTION: Is there a positive integer x < c such that x^2 ≡ a (mod b)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 1 (the unknown x)
- **Per-variable domain:** {1, 2, ..., c-1} -- all positive integers less than c
- **Meaning:** x is the integer whose square, reduced modulo b, must equal a. The search space is bounded by c.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `QuadraticCongruences`
**Variants:** none (operates on three positive integers)

| Field | Type | Description |
|-------|------|-------------|
| `a` | `u64` | The target residue (right-hand side of x^2 ≡ a mod b) |
| `b` | `u64` | The modulus |
| `c` | `u64` | Upper bound on x (exclusive); x must satisfy 1 <= x < c |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The brute-force approach enumerates all x in {1, ..., c-1} and checks x^2 mod b = a, running in O(c * polylog(b)) time. This is pseudo-polynomial. For the general NP-complete case, c can be exponential in the input bit-length. When b is prime and the Extended Riemann Hypothesis holds, the problem is solvable in polynomial time using the Tonelli-Shanks algorithm (O(log^2(b)) operations). When the prime factorization of b is given and c = infinity, polynomial-time algorithms based on the Chinese Remainder Theorem and Hensel's lemma apply.

## Extra Remark

**Full book text:**

INSTANCE: Positive integers a, b, and c.
QUESTION: Is there a positive integer x < c such that x^2 ≡ a (mod b)?

Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete even if the instance includes a prime factorization of b and solutions to the congruence modulo all prime powers occurring in the factorization. Solvable in polynomial time if c = ∞ (i.e., there is no upper bound on x) and the prime factorization of b is given. Assuming the Extended Riemann Hypothesis, the problem is solvable in polynomial time when b is prime. The general problem is trivially solvable in pseudo-polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate x from 1 to c-1; check if x^2 mod b == a.)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: For special cases (b prime, c = infinity), polynomial-time algorithms exist (Tonelli-Shanks, CRT + Hensel lifting).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
a = 2, b = 7, c = 6

**Question:** Is there x in {1, 2, 3, 4, 5} with x^2 ≡ 2 (mod 7)?

**Solution search:**
- x = 1: 1^2 = 1 mod 7 = 1. Not 2.
- x = 2: 2^2 = 4 mod 7 = 4. Not 2.
- x = 3: 3^2 = 9 mod 7 = 2. YES!
- Answer: YES, x = 3.

**Verification:** 3^2 = 9 = 1*7 + 2, so 9 mod 7 = 2 = a.

**Negative example:**
a = 3, b = 7, c = 7

Quadratic residues mod 7: 1^2=1, 2^2=4, 3^2=2, 4^2=2, 5^2=4, 6^2=1. QR mod 7 = {1, 2, 4}.
Since 3 is not a quadratic residue mod 7, no x satisfies x^2 ≡ 3 (mod 7). Answer: NO.
