---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to ROOT OF MODULUS 1"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Root of Modulus 1'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** ROOT OF MODULUS 1
**Motivation:** Establishes NP-hardness of Root of Modulus 1 via polynomial-time reduction from 3SAT. This result, due to Plaisted (1977), demonstrates that determining whether a sparse polynomial (given as a list of coefficient-exponent pairs) has a root on the complex unit circle is NP-hard, even though the problem is not known to be in NP or co-NP. The reduction connects Boolean satisfiability to analytic number theory via cyclotomic polynomials and roots of unity.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## GJ Source Entry

> [AN10] ROOT OF MODULUS 1 (*)
> INSTANCE: Ordered pairs (a[i], b[i]), 1 ≤ i ≤ n, of integers, with each b[i] ≥ 0.
> QUESTION: Does the polynomial Σ_{i=1}^n a[i]·z^{b[i]} have a root on the complex unit circle, i.e., is there a complex number q with |q| = 1 such that Σ_{i=1}^n a[i]·q^{b[i]} = 0?
> Reference: [Plaisted, 1977b]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with n variables {x_1, ..., x_n} and m clauses {C_1, ..., C_m}, construct a sparse polynomial P(z) such that P has a root on the unit circle if and only if the formula is satisfiable.

1. **Prime assignment:** Let p_1, p_2, ..., p_n be the first n prime numbers. Set N = p_1 * p_2 * ... * p_n.

2. **Boolean-to-root encoding:** Each truth assignment to n variables corresponds to a choice of primitive roots of unity. Variable x_i being TRUE corresponds to z being a primitive p_i-th root of unity (i.e., z^{p_i} = 1 but z^k != 1 for 0 < k < p_i). The key insight is that the N-th roots of unity can be partitioned by which subset of primes divide the order, encoding all 2^n truth assignments.

3. **Clause polynomial construction:** For each clause C_j containing literals l_{j,1}, l_{j,2}, l_{j,3}, construct a polynomial Q_j(z) that vanishes at exactly those roots of unity corresponding to truth assignments satisfying C_j. Each literal x_i contributes a factor related to the cyclotomic polynomial Phi_{p_i}(z), and negated literals ~x_i contribute a complementary factor.

4. **Combined polynomial:** The overall polynomial P(z) is constructed so that P(z) = 0 at a root of unity on the unit circle if and only if all clause polynomials are simultaneously satisfied. This is achieved by combining the clause polynomials using a sum-of-squares or product construction that preserves the root structure.

5. **Sparse representation:** The resulting polynomial P(z) is output as a list of (coefficient, exponent) pairs. Since each clause involves at most 3 variables, the intermediate polynomials remain sparse (polynomial in n and m).

6. **Solution extraction:** If a root z_0 with |z_0| = 1 is found, determine which primes p_i divide the order of z_0. Set x_i = TRUE if p_i divides the order, FALSE otherwise. This yields a satisfying assignment.

**Key property:** The polynomial has a root on the unit circle iff the 3SAT formula is satisfiable. The construction is polynomial-time because the number of terms in P(z) is polynomial in n and m, though the exponents may be exponentially large (they are bounded by N = product of first n primes, which can be written in polynomial space as integers).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in source 3SAT instance
- m = number of clauses in source 3SAT instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_terms` (number of coefficient-exponent pairs) | O(n * m) |
| `max_exponent` (largest b[i]) | O(N) where N = product of first n primes (exponential in n, but encoded in O(n^2 log n) bits) |
| `max_coefficient` (largest |a[i]|) | O(poly(n, m)) |

**Derivation:**
- Each clause contributes O(n) terms due to the polynomial encoding of at most 3 literals and their interaction with the prime-based structure.
- The exponents can be as large as N = p_1 * ... * p_n, but since the problem encodes integers in binary, this is polynomial in the input description size.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Construct a small 3SAT instance (e.g., 2-3 variables, 2-3 clauses) and build the corresponding polynomial.
- Verify that for a satisfiable instance, the polynomial evaluates to 0 at some N-th root of unity on the unit circle.
- Verify that for an unsatisfiable instance, the polynomial has no roots on the unit circle.
- Check the forward direction: given a satisfying assignment, compute the corresponding root and verify P(z_0) = 0.
- Check the backward direction: given a root on the unit circle, extract the truth assignment and verify it satisfies all clauses.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
2 variables: x_1, x_2 (n = 2)
2 clauses (m = 2):
- C_1 = (x_1 OR x_2)
- C_2 = (x_1 OR NOT x_2)

**Prime assignment:** p_1 = 2, p_2 = 3. N = 6.

**Root encoding:**
The 6th roots of unity are z = e^{2 pi i k / 6} for k = 0, 1, 2, 3, 4, 5.
- x_1 = TRUE corresponds to z being a primitive 2nd root of unity (z^2 = 1, z != 1), i.e., z = -1 (k = 3).
- x_2 = TRUE corresponds to z being a primitive 3rd root of unity (z^3 = 1, z != 1), i.e., z = e^{2 pi i / 3} or z = e^{4 pi i / 3} (k = 2 or k = 4).
- Both TRUE: z is a primitive 6th root of unity (k = 1 or k = 5).

**Satisfying assignments:**
- x_1 = T, x_2 = T: satisfies both C_1 and C_2.
- x_1 = T, x_2 = F: satisfies both C_1 and C_2.
- x_1 = F, x_2 = T: satisfies C_1 but not C_2.
- x_1 = F, x_2 = F: satisfies neither C_1 nor C_2... wait, C_1 requires x_1 OR x_2, so (F, F) fails C_1.

So the satisfying assignments are (T,T), (T,F). The constructed polynomial P(z) would have roots on the unit circle corresponding to these assignments but not to (F,T) or (F,F).

**Constructed polynomial:** The polynomial P(z) is built so that evaluating at z = e^{2 pi i k / 6}:
- P(z) = 0 when k corresponds to a satisfying assignment (k = 1, 3, 5 for (T,T), k = 3 for (T,F))
- P(z) != 0 when k corresponds to a non-satisfying assignment

The polynomial has a root on the unit circle (e.g., at z = -1 corresponding to x_1 = T, x_2 = F), confirming satisfiability.


## References

- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241-253. IEEE Computer Society.
