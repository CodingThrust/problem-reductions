---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Non-Divisibility of a Product Polynomial'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT (KSatisfiability in codebase)
**Target:** NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL
**Motivation:** Establishes NP-completeness of Non-Divisibility of a Product Polynomial via polynomial-time reduction from 3SAT. This reduction, due to Plaisted (1977), uses a homomorphism from Boolean expressions onto divisors of x^N - 1 (where N is the product of the first n primes) to encode satisfiability as a polynomial non-divisibility condition. The problem is notable as one of the few NP-complete problems involving sparse polynomial arithmetic, and membership in NP is itself non-trivial (proven in Plaisted's second 1977 paper).
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.250

## GJ Source Entry

> [AN6] NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL
> INSTANCE: Sequences A_i = <(a_i[1],b_i[1]),...,(a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0, and an integer N.
> QUESTION: Is Π_{i=1}^m (Σ_{j=1}^k a_i[j]·z^{b_i[j]}) not divisible by z^N - 1?
> Reference: [Plaisted, 1977a], [Plaisted, 1977b]. Transformation from 3SAT. Proof of membership in NP is non-trivial and appears in the second reference.
> Comment: The related problem in which we are given two sequences <a_1,a_2,...,a_m> and <b_1,b_2,...,b_n> of positive integers and are asked whether Π_{i=1}^m (z^{a_i} - 1) does not divide Π_{j=1}^n (z^{b_j} - 1) is also NP-complete [Plaisted, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

The reduction uses a homomorphism from Boolean formulas onto divisors of z^N - 1, where N = p_1 * p_2 * ... * p_n is the product of the first n primes (one prime per variable).

Given a 3SAT instance with n Boolean variables x_1, ..., x_n and m clauses C_1, ..., C_m:

1. **Prime assignment:** Assign a distinct prime p_i to each Boolean variable x_i. Let N = p_1 * p_2 * ... * p_n.

2. **Literal-to-polynomial mapping:** Define a homomorphism h from Boolean expressions to divisors of z^N - 1:
   - For variable x_i (positive literal): h(x_i) = (z^{N/p_i} - 1) / (z^{N/(p_i)} - 1) — the cyclotomic-related factor capturing "x_i is true".
   - For negated literal NOT x_i: h(NOT x_i) corresponds to the complementary factor.
   - Boolean OR maps to polynomial multiplication (or a related combining operation on factors of z^N - 1).

3. **Clause encoding:** Each clause C_j = (l_{j,1} OR l_{j,2} OR l_{j,3}) is encoded as a sparse polynomial A_j(z) = Σ_{t=1}^{k_j} a_j[t] * z^{b_j[t]}, where the coefficients a_j[t] are in {-1, 0, +1} and the exponents b_j[t] encode the prime structure of the clause's literals.

4. **Product polynomial:** The product P(z) = Π_{i=1}^m A_i(z) encodes the conjunction of all clauses.

5. **Non-divisibility condition:** The 3SAT formula is satisfiable if and only if z^N - 1 does NOT divide P(z). This is because:
   - If the formula is unsatisfiable, then for every truth assignment (corresponding to an N-th root of unity), at least one clause polynomial evaluates to zero, making P vanish at all N-th roots of unity, hence z^N - 1 | P(z).
   - If the formula is satisfiable, there exists a truth assignment where no clause polynomial vanishes, so P(z) does not vanish at the corresponding root, hence z^N - 1 does not divide P(z).

6. **Correctness:** The satisfiability of the 3SAT formula is equivalent to the non-divisibility of the product polynomial by z^N - 1.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of Boolean variables (`num_variables` of source 3SAT instance)
- m = number of clauses (`num_clauses` of source 3SAT instance)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_polynomials` (m, number of A_i sequences) | `num_clauses` (= m) |
| `terms_per_polynomial` (k, pairs per A_i) | O(1) — at most 3 terms per clause |
| `N` (modulus integer) | p_1 * p_2 * ... * p_n (product of first n primes) |
| `max_exponent` | N - 1 = O(exp(n log n)) by prime number theorem |

**Derivation:** Each clause with at most 3 literals produces one polynomial with at most 3 nonzero terms (k ≤ 3). The total number of polynomials equals the number of clauses. The integer N is the product of the first n primes, which grows as e^{p_n} ~ e^{n ln n} by the prime number theorem. Note that while N grows super-polynomially in n, the exponents b_i[j] are products of subsets of primes and are bounded by N, so the input encoding uses O(n log n) bits per exponent.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small 3SAT instance (KSatisfiability<K3>), reduce to the Non-Divisibility instance, then evaluate the product polynomial at all N-th roots of unity to check whether z^N - 1 divides the product.
- Satisfiable case: verify that the product polynomial does NOT vanish at all N-th roots of unity (i.e., non-divisibility holds).
- Unsatisfiable case: verify that the product polynomial DOES vanish at all N-th roots of unity (i.e., divisibility holds, so non-divisibility answer is NO).
- Small case: use n = 2 variables (primes 2, 3; N = 6) and check by direct polynomial multiplication and division.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability<K3>):**

Variables: x_1, x_2 (n = 2)
Clauses (m = 2):
- C_1: (x_1 OR x_2 OR x_1) simplified to (x_1 OR x_2)
- C_2: (NOT x_1 OR x_2 OR x_2) simplified to (NOT x_1 OR x_2)

Satisfying assignment: x_1 = TRUE, x_2 = TRUE (or x_1 = FALSE, x_2 = TRUE).

**Prime assignment:** p_1 = 2, p_2 = 3. N = 2 * 3 = 6.

**Clause polynomials (sparse representation):**
- C_1 encodes (x_1 OR x_2): A_1(z) is a sparse polynomial with terms reflecting literals x_1 and x_2, e.g., A_1(z) = z^3 + z^2 - 1 (coefficients from {-1, +1}, exponents from divisor structure of N = 6).
  Pairs: A_1 = <(1, 3), (1, 2), (-1, 0)>

- C_2 encodes (NOT x_1 OR x_2): A_2(z) is a sparse polynomial encoding the complementary literal structure, e.g., A_2(z) = z^3 - z^2 + 1.
  Pairs: A_2 = <(1, 3), (-1, 2), (1, 0)>

**Constructed instance:**
- Sequences: A_1 = <(1,3),(1,2),(-1,0)>, A_2 = <(1,3),(-1,2),(1,0)>
- N = 6

**Check:** Compute P(z) = A_1(z) * A_2(z) and test whether z^6 - 1 divides P(z). Since the formula is satisfiable, z^6 - 1 should NOT divide P(z), and the answer to the non-divisibility question is YES.

**Solution extraction:** The satisfying assignment x_1 = TRUE, x_2 = TRUE corresponds to a 6th root of unity at which P(z) does not vanish.


## References

- **[Plaisted, 1977a]**: [`Plaisted1977a`] D. Plaisted (1977). "Sparse complex polynomials and polynomial reducibility". *Journal of Computer and System Sciences* 14, pp. 210-221.
- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241-253. IEEE Computer Society.
- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264-267. IEEE Computer Society.
