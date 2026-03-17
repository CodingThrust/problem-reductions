---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PERIODIC SOLUTION RECURRENCE RELATION"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Periodic Solution Recurrence Relation'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** PERIODIC SOLUTION RECURRENCE RELATION
**Motivation:** Establishes NP-hardness of the Periodic Solution Recurrence Relation problem via polynomial-time reduction from 3SAT. This result, due to Plaisted (1977), demonstrates that determining whether a linear recurrence relation (given in sparse form by coefficient-lag pairs) admits a periodic solution is NP-hard. The connection arises because periodic solutions of a recurrence a_i = sum c_j * a_{i-b_j} correspond to roots of the characteristic polynomial sum c_j * z^{b_j} on the unit circle. Thus the problem reduces to finding a root of modulus 1 of a sparse polynomial, which Plaisted showed is NP-hard from 3SAT. The problem is not known to be in NP or co-NP.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## GJ Source Entry

> [AN12] PERIODIC SOLUTION RECURRENCE RELATION (*)
> INSTANCE: Ordered pairs (c_i, b_i), 1 <= i <= m, of integers, with all b_i positive.
> QUESTION: Is there a sequence a_0,a_1,...,a_{n-1} of integers, with n >= max{b_i}, such that the infinite sequence a_0,a_1,... defined by the recurrence relation
>
> a_i = Sigma_{j=1}^m c_j*a_{(i-b_j)}
>
> satisfies a_i = a_{i(mod n)}, for all i >= n?
> Reference: [Plaisted, 1977b]. Tranformation from 3SAT
> Comment: Not known to be in NP or co-NP. See reference for related results.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with n variables and m clauses, construct a set of recurrence relation coefficient-lag pairs such that the recurrence has a periodic solution if and only if the formula is satisfiable.

The reduction proceeds via the correspondence between periodic solutions of linear recurrences and roots of the characteristic polynomial on the unit circle:

1. **Characteristic polynomial connection:** A linear recurrence relation a_i = sum_{j=1}^m c_j * a_{i-b_j} has periodic solutions if and only if its characteristic polynomial P(z) = z^B - sum_{j=1}^m c_j * z^{B-b_j} (where B = max{b_j}) has a root on the complex unit circle. This is a classical result from the theory of linear recurrences: if z_0 is a root with |z_0| = 1, then a_i = z_0^i gives a periodic (or quasi-periodic) solution.

2. **Reduction from Root of Modulus 1:** Given a 3SAT formula, first apply Plaisted's reduction to construct a sparse polynomial Q(z) = sum a_i * z^{b_i} that has a root on the unit circle iff the formula is satisfiable (see R173).

3. **Conversion to recurrence form:** Rewrite Q(z) = 0 as a recurrence relation. If Q(z) = sum_{i=1}^n a_i * z^{b_i}, then the recurrence a_t = sum c_j * a_{t-b_j} with appropriate coefficients c_j and lags b_j has Q(z) as (part of) its characteristic polynomial. The key is to ensure that Q(z) being zero at z_0 on the unit circle is equivalent to the recurrence having a periodic solution with period related to z_0.

4. **Coefficient-lag pairs:** Output the pairs {(c_1, b_1), ..., (c_m, b_m)} derived from the sparse polynomial encoding. Each pair specifies a coefficient c_i and a lag b_i in the recurrence relation.

5. **Solution extraction:** If a periodic solution exists with period n, the corresponding root of the characteristic polynomial on the unit circle encodes a truth assignment. Extract the assignment as in the Root of Modulus 1 reduction: determine which primes divide n and set variables accordingly.

**Key insight:** The existence of a periodic solution for a linear recurrence is equivalent to the characteristic polynomial having a root on the unit circle. Plaisted's encoding of 3SAT into a sparse polynomial with roots on the unit circle thus directly translates to a recurrence relation with periodic solutions.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n_vars = number of variables in source 3SAT instance
- n_clauses = number of clauses in source 3SAT instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_pairs` (m in target, number of coefficient-lag pairs) | O(n_vars * n_clauses) |
| `max_lag` (max b_i) | O(N) where N = product of first n_vars primes |
| `max_coefficient` (max |c_i|) | O(poly(n_vars, n_clauses)) |

**Derivation:**
- The number of recurrence terms equals the number of nonzero terms in the characteristic polynomial, which is polynomial in the 3SAT instance size.
- The lags b_i correspond to exponents in the characteristic polynomial and can be as large as N = product of the first n_vars primes.
- Coefficients are bounded polynomially in the input size.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Construct a small 3SAT instance (2-3 variables, 2-3 clauses) and build the corresponding recurrence relation.
- For a satisfiable instance, verify that a periodic integer sequence exists satisfying the recurrence by:
  - Computing the roots of the characteristic polynomial.
  - Checking that at least one root lies on the unit circle.
  - Constructing the periodic sequence from that root.
- For an unsatisfiable instance, verify that no root of the characteristic polynomial lies on the unit circle, so no periodic solution exists.
- Cross-validate by checking consistency with the Root of Modulus 1 reduction (R173).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
2 variables: x_1, x_2 (n = 2)
1 clause (m = 1):
- C_1 = (x_1 OR x_2)

This instance is satisfiable (e.g., x_1 = T, x_2 = T).

**Prime assignment:** p_1 = 2, p_2 = 3. N = 6.

**Constructed recurrence:**
After applying Plaisted's polynomial encoding, suppose the characteristic polynomial is:
P(z) = z^6 - z^4 - z^3 + z

This can be rewritten as the recurrence relation:
a_i = a_{i-2} + a_{i-3} - a_{i-5}

with coefficient-lag pairs: {(1, 2), (1, 3), (-1, 5)}.

**Verification:**
The polynomial P(z) = z^6 - z^4 - z^3 + z = z(z^5 - z^3 - z^2 + 1) = z(z^2 - 1)(z^3 - 1).
Roots on the unit circle: z = 1, z = -1, z = e^{2 pi i/3}, z = e^{4 pi i/3}.
Since roots on the unit circle exist, the recurrence has periodic solutions, confirming the 3SAT instance is satisfiable.

For example, with z_0 = -1 (corresponding to x_1 = T, x_2 = F via the prime encoding):
The periodic solution is a_i = (-1)^i, giving period 2: {1, -1, 1, -1, ...}.
Check: a_i = a_{i-2} + a_{i-3} - a_{i-5} = (-1)^{i-2} + (-1)^{i-3} - (-1)^{i-5} = (-1)^i + (-1)^{i-1} - (-1)^{i-1} = (-1)^i. Correct.


## References

- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241-253. IEEE Computer Society.
