---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NON-TRIVIAL GREATEST COMMON DIVISOR"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Non-Trivial Greatest Common Divisor'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT (KSatisfiability in codebase)
**Target:** NON-TRIVIAL GREATEST COMMON DIVISOR
**Motivation:** Establishes NP-hardness of the Non-Trivial Greatest Common Divisor problem for sparse polynomials via polynomial-time reduction from 3SAT. Due to Plaisted (1977), this reduction encodes Boolean satisfiability into polynomial GCD structure by mapping variables to primes and using a homomorphism from Boolean expressions onto divisors of z^N - 1, where N is the product of the first n primes. The problem is notable because it is not known to be in NP or co-NP, yet is NP-hard. The hardness persists even when restricted to polynomials with {-1, +1} coefficients or when m = 2 (just two polynomials).
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.250

## GJ Source Entry

> [AN7] NON-TRIVIAL GREATEST COMMON DIVISOR (*)
> INSTANCE: Sequences A_i = <(a_i[1],b_i[1]),...,(a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0.
> QUESTION: Does the greatest common divisor of the polynomials Σ_{j=1}^k a_i[j]·z^{b_i[j]}, 1 ≤ i ≤ m, have degree greater than zero?
> Reference: [Plaisted, 1977a]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1 [Plaisted, 1976] or if m = 2 [Plaisted, 1977b]. The analogous problem in which the instance also includes a positive integer K, and we are asked if the least common multiple of the given polynomials has degree less than K, is NP-hard under the same restrictions. Both problems can be solved in pseudo-polynomial time using standard algorithms.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

The reduction uses Plaisted's homomorphism from Boolean formulas onto divisors of the polynomial z^N - 1, where N = p_1 * p_2 * ... * p_n is the product of the first n primes.

Given a 3SAT instance with n Boolean variables x_1, ..., x_n and m clauses C_1, ..., C_m:

1. **Prime assignment:** Assign a distinct prime p_i to each Boolean variable x_i. Let N = p_1 * p_2 * ... * p_n.

2. **Literal-to-polynomial factor mapping:** Define a homomorphism h from Boolean expressions to polynomial divisors of z^N - 1:
   - For variable x_i: associate cyclotomic factors of z^N - 1 related to multiples of p_i.
   - For NOT x_i: associate the complementary cyclotomic factors.
   - The key identity is z^N - 1 = Π_{d|N} Φ_d(z), where Φ_d is the d-th cyclotomic polynomial.

3. **Clause-to-polynomial encoding:** Each clause C_j = (l_{j,1} OR l_{j,2} OR l_{j,3}) is encoded as a sparse polynomial f_j(z) = Σ_{t=1}^{k_j} a_j[t] * z^{b_j[t]}, where:
   - The coefficients a_j[t] are in {-1, +1} (since the problem remains NP-hard under this restriction).
   - The exponents b_j[t] are derived from the prime structure encoding the clause's literals.
   - The polynomial f_j(z) is constructed so that its roots among the N-th roots of unity correspond exactly to truth assignments that do NOT satisfy clause C_j.

4. **GCD structure:** The GCD of the polynomials f_1(z), f_2(z), ..., f_m(z) has degree > 0 if and only if there exists a common root (an N-th root of unity) shared by all polynomials. Such a common root corresponds to a truth assignment that fails to satisfy at least one literal in EVERY clause -- equivalently, makes the formula unsatisfiable. Therefore:
   - gcd(f_1, ..., f_m) has degree > 0  <=>  the 3SAT formula is UNSATISFIABLE.
   - To get the correct polarity (GCD degree > 0 iff formula IS satisfiable), the construction is adjusted: encode the negation of each clause, so that common roots correspond to satisfying assignments.

5. **Correctness:** The polynomials share a non-trivial common factor (of z^N - 1) if and only if there exists a consistent truth assignment satisfying all clauses of the 3SAT formula.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of Boolean variables (`num_variables` of source 3SAT instance)
- m = number of clauses (`num_clauses` of source 3SAT instance)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_polynomials` (m, number of A_i sequences) | `num_clauses` (= m) |
| `terms_per_polynomial` (k, pairs per A_i) | O(1) -- at most a constant number of terms per clause |
| `max_exponent` | N - 1 = p_1 * p_2 * ... * p_n - 1 (product of first n primes) |

**Derivation:** Each clause with at most 3 literals produces one sparse polynomial with O(1) nonzero terms. The total number of polynomials equals m. The exponents are bounded by N = p_1 * ... * p_n. While N grows super-polynomially, the encoding of each exponent uses O(n log n) bits. The coefficients are restricted to {-1, +1}. The reduction runs in polynomial time because only the sparse representation (pairs of coefficient and exponent) is output.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small 3SAT instance (KSatisfiability<K3>), reduce to the Non-Trivial GCD instance, then compute the GCD of the resulting polynomials using standard polynomial GCD algorithms and check whether the degree is > 0.
- Satisfiable case: verify that the GCD has degree > 0 (the polynomials share a common root corresponding to a satisfying assignment).
- Unsatisfiable case: verify that the GCD has degree 0 (i.e., gcd = 1, no common factor).
- Coefficient restriction: verify all coefficients are in {-1, +1} as guaranteed by Plaisted's construction.
- Small case: use n = 2 variables (primes 2, 3; N = 6), compute GCD by direct polynomial arithmetic.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability<K3>):**

Variables: x_1, x_2 (n = 2)
Clauses (m = 2):
- C_1: (x_1 OR x_2 OR x_1) simplified to (x_1 OR x_2)
- C_2: (NOT x_1 OR NOT x_2 OR NOT x_1) simplified to (NOT x_1 OR NOT x_2)

Satisfying assignments: x_1 = TRUE, x_2 = FALSE; or x_1 = FALSE, x_2 = TRUE. Formula is satisfiable.

**Prime assignment:** p_1 = 2, p_2 = 3. N = 6.

**Clause polynomials (sparse, {-1,+1}-coefficient representation):**
- C_1 encodes (x_1 OR x_2): f_1(z) is a polynomial whose roots among 6th roots of unity correspond to the assignment x_1 = FALSE AND x_2 = FALSE.
  f_1(z) = z^3 + z^2 - z - 1
  Pairs: A_1 = <(1,3),(1,2),(-1,1),(-1,0)>

- C_2 encodes (NOT x_1 OR NOT x_2): f_2(z) is a polynomial whose roots correspond to x_1 = TRUE AND x_2 = TRUE.
  f_2(z) = z^3 - z^2 + z - 1
  Pairs: A_2 = <(1,3),(-1,2),(1,1),(-1,0)>

**Constructed instance:**
- Sequences: A_1 = <(1,3),(1,2),(-1,1),(-1,0)>, A_2 = <(1,3),(-1,2),(1,1),(-1,0)>
- m = 2

**Check:** Compute gcd(f_1(z), f_2(z)).
- f_1(z) = z^3 + z^2 - z - 1 = (z+1)(z+1)(z-1) = (z+1)^2(z-1)
- f_2(z) = z^3 - z^2 + z - 1 = (z-1)(z^2+1)
- gcd(f_1, f_2) = (z - 1), which has degree 1 > 0.

The root z = 1 corresponds to a truth assignment. Since the formula is satisfiable, the GCD is non-trivial, and the answer is YES.

**Solution extraction:** The common root z = 1 (a primitive 1st root of unity, dividing N = 6) corresponds to a satisfying assignment that can be decoded from the prime factorization structure.


## References

- **[Plaisted, 1977a]**: [`Plaisted1977a`] D. Plaisted (1977). "Sparse complex polynomials and polynomial reducibility". *Journal of Computer and System Sciences* 14, pp. 210-221.
- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264-267. IEEE Computer Society.
- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241-253. IEEE Computer Society.
