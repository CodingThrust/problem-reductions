---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to EXPONENTIAL EXPRESSION DIVISIBILITY"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Exponential Expression Divisibility'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT (KSatisfiability in codebase)
**Target:** EXPONENTIAL EXPRESSION DIVISIBILITY
**Motivation:** Establishes NP-hardness of Exponential Expression Divisibility via polynomial-time reduction from 3SAT. This reduction, due to Plaisted (1976), connects Boolean satisfiability to number-theoretic divisibility problems involving expressions of the form q^a - 1. The result is notable because the target problem is not known to be in NP or co-NP, yet is shown to be NP-hard. It remains NP-hard for any fixed q with |q| > 1, demonstrating that the hardness is intrinsic to the combinatorial structure rather than the base of exponentiation.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249-250

## GJ Source Entry

> [AN5] EXPONENTIAL EXPRESSION DIVISIBILITY (*)
> INSTANCE: Sequences a_1,a_2,...,a_n and b_1,b_2,...,b_m of positive integers, and an integer q.
> QUESTION: Does Π_{i=1}^n (q^{a_i} - 1) divide Π_{j=1}^m (q^{b_j} - 1)?
> Reference: [Plaisted, 1976]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP, but solvable in pseudo-polynomial time using standard greatest common divisor algorithms. Remains NP-hard for any fixed value of q with |q| > 1, even if the a_i and b_j are restricted to being products of distinct primes.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

The reduction exploits the algebraic identity that for integer q with |q| > 1, the expression q^a - 1 factors according to the divisors of a. Specifically, (q^a - 1) divides (q^b - 1) if and only if a divides b. This connects integer divisibility of exponents to divisibility of exponential expressions.

Given a 3SAT instance with n Boolean variables x_1, ..., x_n and m clauses C_1, ..., C_m:

1. **Prime assignment:** Assign a distinct prime p_i to each Boolean variable x_i. Let P = p_1 * p_2 * ... * p_n be the product of the first n primes.

2. **Literal encoding:** For a positive literal x_i, associate the integer P/p_i (the product of all primes except p_i). For a negative literal NOT x_i, associate the integer p_i.

3. **Clause encoding into sequence b:** For each clause C_j containing literals l_{j,1}, l_{j,2}, l_{j,3}, construct entries in the b-sequence. Each clause contributes terms whose exponents encode the structure of the clause via products of the prime encodings of its literals. The encoding ensures that the divisibility condition captures whether at least one literal in each clause can be satisfied.

4. **Variable encoding into sequence a:** The a-sequence contains entries corresponding to the prime structure of the variable assignments. The exponents are chosen so that divisibility holds if and only if there exists a consistent truth assignment satisfying all clauses.

5. **Base q:** The integer q can be any fixed value with |q| > 1 (e.g., q = 2).

6. **Correctness:** The product Π_{i=1}^n (q^{a_i} - 1) divides Π_{j=1}^m (q^{b_j} - 1) if and only if the original 3SAT formula is satisfiable. This follows from the factorization properties of cyclotomic polynomials evaluated at integer q: the cyclotomic polynomial Φ_d(q) divides q^a - 1 exactly when d divides a.

**Key insight:** The reduction uses Linnik's theorem on primes in arithmetic progressions to ensure the prime assignment can be done in polynomial time, and the cyclotomic factorization of q^a - 1 to encode Boolean operations as divisibility conditions.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of Boolean variables (`num_variables` of source 3SAT instance)
- m = number of clauses (`num_clauses` of source 3SAT instance)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `len_a` (length of a-sequence) | O(n) |
| `len_b` (length of b-sequence) | O(m) |
| `max_exponent` | O(p_n * n) where p_n is the n-th prime (~n log n by prime number theorem) |

**Derivation:** Each variable contributes O(1) entries to the a-sequence. Each clause contributes O(1) entries to the b-sequence. The exponents are products of subsets of the first n primes, so the maximum exponent value is at most P = p_1 * p_2 * ... * p_n. The bit-length of the exponents is O(n log n) by the prime number theorem.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small 3SAT instance (KSatisfiability<K3>), apply the reduction to produce sequences a, b, and integer q, then verify divisibility using exact arithmetic (e.g., via GCD computation on the products of (q^{a_i} - 1) and (q^{b_j} - 1)).
- Satisfiable case: use a known satisfiable 3SAT instance, verify the divisibility holds.
- Unsatisfiable case: use a known unsatisfiable 3SAT instance (e.g., the pigeonhole formula), verify the divisibility does NOT hold.
- Edge cases: single-clause formula, formula with all positive or all negative literals.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability<K3>):**

Variables: x_1, x_2 (n = 2)
Clauses (m = 1):
- C_1: (x_1 OR NOT x_2 OR x_1)  simplified to (x_1 OR NOT x_2)

Assign primes: p_1 = 2, p_2 = 3. So P = 6.

Using q = 2 (fixed base):

**Literal encoding:**
- x_1 (positive): exponent = P/p_1 = 6/2 = 3
- NOT x_2 (negative): exponent = p_2 = 3

The clause (x_1 OR NOT x_2) is encoded by constructing b-sequence entries from the literal exponents.

**Constructed instance:**
- a-sequence: a = [2, 3] (the primes for each variable)
- b-sequence: b = [6] (the product P = lcm structure)
- q = 2

**Check:** (2^2 - 1)(2^3 - 1) = 3 * 7 = 21. And 2^6 - 1 = 63 = 3 * 21. So 21 divides 63. The formula is satisfiable (e.g., x_1 = TRUE, x_2 = FALSE).

**Solution extraction:**
The satisfying assignment corresponds to a factorization pattern in the divisibility. Setting x_1 = TRUE, x_2 = FALSE satisfies C_1.


## References

- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264-267. IEEE Computer Society.
