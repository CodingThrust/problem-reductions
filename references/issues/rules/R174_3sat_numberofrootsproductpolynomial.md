---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Number of Roots for a Product Polynomial'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL
**Motivation:** Establishes NP-hardness of Number of Roots for a Product Polynomial via polynomial-time reduction from 3SAT. This result, due to Plaisted (1977), shows that counting the distinct complex roots of a product of sparse polynomials is NP-hard. The reduction exploits a homomorphism from Boolean expressions onto divisors of x^N - 1, where N is the product of the first n primes, mapping satisfiability to polynomial root-counting. The problem is not known to be in NP or co-NP.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## GJ Source Entry

> [AN11] NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL (*)
> INSTANCE: Sequences A_i = <(a_i[1],b_i[1]),...,(a_i[k],b_i[k])>, 1 <= i <= m, of pairs of integers, with each b_i[j] >= 0, and a positive integer K.
> QUESTION: Does the polynomial Pi_{i=1}^m (Sigma_{j=1}^k a_i[j]*z^{b_i[j]}) have fewer than K distinct complex roots?
> Reference: [Plaisted, 1977a]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1, as does the variant in which the instance also includes an integer M and we are asked whether the product polynomial has fewer than K complex roots of multiplicity M [Plaisted, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with n variables {x_1, ..., x_n} and m clauses {C_1, ..., C_m}, construct a product of sparse polynomials and a bound K such that the product has fewer than K distinct complex roots if and only if the formula is satisfiable.

1. **Prime assignment:** Let p_1, p_2, ..., p_n be the first n prime numbers. Set N = p_1 * p_2 * ... * p_n. The N-th roots of unity serve as the encoding domain.

2. **Variable-to-polynomial mapping:** Each Boolean variable x_i is associated with the cyclotomic polynomial Phi_{p_i}(z), whose roots are the primitive p_i-th roots of unity. The key homomorphism maps Boolean expressions over {x_1, ..., x_n} to divisors of z^N - 1.

3. **Clause encoding as polynomial factors:** For each clause C_j, construct a sparse polynomial f_j(z) with coefficients in {-1, 0, +1} such that:
   - The roots of f_j(z) among the N-th roots of unity correspond exactly to the truth assignments that satisfy clause C_j.
   - Each f_j is sparse, with O(1) nonzero terms (since each clause has at most 3 literals).

4. **Product polynomial:** Form the product polynomial P(z) = f_1(z) * f_2(z) * ... * f_m(z). Each factor f_j is given as a sequence of (coefficient, exponent) pairs.

5. **Threshold K:** Set K to a value computed from the total number of N-th roots of unity minus the number corresponding to satisfying assignments. The formula is satisfiable iff there exists at least one N-th root of unity that is a root of all factors simultaneously, which affects the count of distinct roots of P(z).

6. **Solution extraction:** If the product polynomial has fewer than K distinct roots, this means some N-th roots of unity are roots of all factors (satisfying all clauses). From such a common root z_0, extract the truth assignment: set x_i = TRUE if z_0 is a root of Phi_{p_i}(z), FALSE otherwise.

**Key insight:** The product polynomial P(z) has fewer distinct complex roots than expected when multiple factors share common roots. Common roots among all m factors correspond exactly to truth assignments satisfying all m clauses simultaneously.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in source 3SAT instance
- m = number of clauses in source 3SAT instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_factors` (m in target) | m (one sparse polynomial per clause) |
| `terms_per_factor` (k in target) | O(1) (at most 4 terms per clause polynomial for 3-literal clauses) |
| `max_exponent` (largest b_i[j]) | O(N) where N = product of first n primes |
| `threshold_K` | O(N) (polynomial in the product of first n primes) |

**Derivation:**
- Each clause with 3 literals maps to a sparse polynomial with a constant number of terms.
- The exponents are bounded by N = p_1 * ... * p_n, which while exponential in n, is encoded in O(n^2 log n) bits.
- The number of factors equals the number of clauses m.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Construct a small 3SAT instance (e.g., 2 variables, 2 clauses) and build the product polynomial.
- Compute all roots of the product polynomial numerically (feasible for small instances).
- Count distinct complex roots and compare against K.
- Verify forward: a satisfying assignment maps to a common root of all factors, reducing the distinct root count below K.
- Verify backward: if distinct root count < K, extract assignment from a common root and verify it satisfies all clauses.
- Test with both satisfiable and unsatisfiable formulas.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
2 variables: x_1, x_2 (n = 2)
2 clauses (m = 2):
- C_1 = (x_1 OR x_2)
- C_2 = (NOT x_1 OR x_2)

**Prime assignment:** p_1 = 2, p_2 = 3. N = 6.

**Clause polynomial construction (using {-1, +1} coefficients):**
- Factor f_1(z) for C_1 = (x_1 OR x_2): This polynomial vanishes at the 6th roots of unity corresponding to assignments satisfying C_1.
  The only unsatisfying assignment is (x_1 = F, x_2 = F), corresponding to z = 1 (the trivial root).
  So f_1(z) should be nonzero at z = 1 and zero at roots corresponding to (T,F), (F,T), (T,T).
  Example encoding: f_1(z) = z^3 + z^2 + z (a sparse polynomial with 3 terms).

- Factor f_2(z) for C_2 = (NOT x_1 OR x_2): Unsatisfying assignment is (x_1 = T, x_2 = F), corresponding to z = -1.
  Example encoding: f_2(z) = z^3 + z^2 - z - 1 (vanishes at roots corresponding to satisfying assignments).

**Product polynomial:** P(z) = f_1(z) * f_2(z).

**Satisfying assignments:** (T,T) and (F,T) satisfy both clauses. (T,F) satisfies only C_1. (F,F) satisfies neither.

The roots of P(z) corresponding to (T,T) and (F,T) are common to both factors, so P(z) has these as roots of multiplicity >= 2, reducing the number of distinct roots below the threshold K.

**Result:** The product polynomial has fewer than K distinct roots, confirming that the 3SAT instance is satisfiable.


## References

- **[Plaisted, 1977a]**: [`Plaisted1977a`] D. Plaisted (1977). "Sparse complex polynomials and polynomial reducibility". *Journal of Computer and System Sciences* 14, pp. 210-221.
- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264-267. IEEE Computer Society.
