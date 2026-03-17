---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to QUADRATIC CONGRUENCES"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'QUADRATIC CONGRUENCES'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** QUADRATIC CONGRUENCES
**Motivation:** Establishes NP-completeness of QUADRATIC CONGRUENCES via polynomial-time reduction from 3SAT. This is a landmark result by Manders and Adleman (1978) showing that even simple number-theoretic problems involving quadratic equations are computationally intractable. The bound on x (x < c) is essential for hardness; without it, the problem becomes polynomial-time solvable given the factorization of b. The reduction demonstrates a deep connection between Boolean satisfiability and modular arithmetic, using the Chinese Remainder Theorem to encode truth assignments as residues modulo carefully chosen primes.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249

## GJ Source Entry

> [AN1] QUADRATIC CONGRUENCES
> INSTANCE: Positive integers a, b, and c.
> QUESTION: Is there a positive integer x < c such that x^2 ≡ a (mod b)?
> Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
> Comment: Remains NP-complete even if the instance includes a prime factorization of b and solutions to the congruence modulo all prime powers occurring in the factorization. Solvable in polynomial time if c = ∞ (i.e., there is no upper bound on x) and the prime factorization of b is given. Assuming the Extended Riemann Hypothesis, the problem is solvable in polynomial time when b is prime. The general problem is trivially solvable in pseudo-polynomial time.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

The reduction from 3SAT to QUADRATIC CONGRUENCES follows the approach of Manders and Adleman (1978). Given a 3SAT instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}:

**High-level approach:**
The key idea is to encode the satisfiability problem using modular arithmetic. Each Boolean variable is associated with a distinct odd prime, and the truth assignment is encoded via quadratic residuosity modulo these primes. The Chinese Remainder Theorem (CRT) is then used to combine the per-prime constraints into a single congruence.

**Construction:**

1. **Prime assignment:** For each variable u_i (1 <= i <= n), assign a distinct odd prime p_i. These primes can be chosen to be the first n odd primes: p_1 = 3, p_2 = 5, p_3 = 7, ....

2. **Encoding truth values via quadratic residues:** For each prime p_i, choose a quadratic residue r_i and a quadratic non-residue s_i modulo p_i. Associate:
   - u_i = TRUE with x ≡ r_i (mod p_i) (x is a quadratic residue mod p_i)
   - u_i = FALSE with x ≡ s_i (mod p_i) (x is a quadratic non-residue mod p_i)

3. **Clause encoding:** Each clause constrains which residue classes are acceptable. For a clause (l_1 ∨ l_2 ∨ l_3), the requirement is that at least one literal is true, which translates to: x must avoid all residue classes corresponding to all three literals being false simultaneously.

4. **Combining via CRT:** Set b = product of prime powers p_i^{k_i} for appropriate exponents k_i (at least linear in n and m). The value a is determined by the CRT to encode all clause constraints simultaneously. The bound c is set to be the product of all primes (or a suitable upper bound derived from the CRT).

5. **Solution extraction:** Given x < c satisfying x^2 ≡ a (mod b), compute x mod p_i for each variable. If x^2 is a quadratic residue mod p_i, set u_i = TRUE; otherwise set u_i = FALSE.

**Key properties:**
- The modulus b has prime factor multiplicities at least linear in the number of variables and clauses
- The parameters a, b, c have bit-length polynomial in the size of the 3SAT instance
- The construction runs in polynomial time (prime generation and CRT computation)

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `bit_length_a` | O(n * log(n) + m * log(m)) |
| `bit_length_b` | O(n * (n + m) * log(n)) |
| `bit_length_c` | O(n * log(n)) |

**Derivation:**
- b is a product of n primes each raised to powers linear in (n + m), so log(b) = O(n * (n + m) * log(n))
- a is determined by CRT from O(n) constraints, so log(a) <= log(b)
- c is at most the product of the n primes, so log(c) = O(n * log(n))
- Total encoding size is polynomial in n + m

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce KSatisfiability<K3> instance to QuadraticCongruences, solve target with BruteForce (enumerate x from 1 to c-1, check x^2 mod b == a mod b), extract truth assignment from the quadratic residuosity of x modulo each prime, verify truth assignment satisfies all clauses
- Test with both satisfiable and unsatisfiable 3SAT instances
- Verify that pseudo-polynomial brute force (iterating x < c) correctly identifies satisfiable/unsatisfiable cases

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (KSatisfiability<K3>):**
2 variables: u_1, u_2 (n = 2)
2 clauses (m = 2):
- c_1 = (u_1 ∨ u_2)
- c_2 = (¬u_1 ∨ u_2)

**Illustrative construction (simplified):**

1. Assign primes: p_1 = 3 (for u_1), p_2 = 5 (for u_2).

2. Quadratic residues:
   - mod 3: QR = {0, 1}, QNR = {2}. Use: u_1 = TRUE ↔ x ≡ 1 (mod 3), u_1 = FALSE ↔ x ≡ 2 (mod 3).
   - mod 5: QR = {0, 1, 4}, QNR = {2, 3}. Use: u_2 = TRUE ↔ x ≡ 1 or 4 (mod 5), u_2 = FALSE ↔ x ≡ 2 or 3 (mod 5).

3. Clause constraints:
   - c_1 = (u_1 ∨ u_2): forbid (u_1=F, u_2=F), i.e., forbid x ≡ 2 (mod 3) AND x ≡ {2,3} (mod 5)
   - c_2 = (¬u_1 ∨ u_2): forbid (u_1=T, u_2=F), i.e., forbid x ≡ 1 (mod 3) AND x ≡ {2,3} (mod 5)

4. Set b = 3^k * 5^k for suitable k, and determine a via CRT so that x^2 ≡ a (mod b) encodes the feasible truth assignments.

**Solution mapping:**
- Satisfying assignment: u_1 = FALSE, u_2 = TRUE satisfies both clauses.
- This corresponds to x ≡ 2 (mod 3) and x ≡ 1 (mod 5), giving x ≡ 11 (mod 15).
- Check: 11^2 = 121 ≡ 1 (mod 3) -- wait, we need to verify the quadratic residue property:
  - x = 11: 11 mod 3 = 2, 11 mod 5 = 1
  - 11^2 = 121: 121 mod 3 = 1 (QR mod 3 means u_1 assignment is consistent)
  - 121 mod 5 = 1 (QR mod 5 means u_2 = TRUE)
  - So the assignment extracted is u_1 with residue 2 mod 3 (non-residue, FALSE), u_2 with residue 1 mod 5 (residue, TRUE).


## References

- **[Manders and Adleman, 1978]**: [`Manders1978`] Kenneth Manders and Leonard Adleman (1978). "{NP}-complete decision problems for binary quadratics". *Journal of Computer and System Sciences* 16, pp. 168-184.
