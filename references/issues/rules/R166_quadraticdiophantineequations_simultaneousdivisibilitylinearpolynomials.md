---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QUADRATIC DIOPHANTINE EQUATIONS to SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS"
labels: rule
assignees: ''
canonical_source_name: 'QUADRATIC DIOPHANTINE EQUATIONS'
canonical_target_name: 'SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QUADRATIC DIOPHANTINE EQUATIONS
**Target:** SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS
**Motivation:** Establishes NP-hardness of SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS via polynomial-time reduction from QUADRATIC DIOPHANTINE EQUATIONS. Lipshitz (1977, 1978) showed that the existential theory of the integers with addition and divisibility is decidable, and as part of this work demonstrated that the divisibility problem is at least as hard as quadratic Diophantine equations. The target problem is notable: it is not known to be in NP in general, but is NP-complete for any fixed number of divisibility constraints n >= 5. The general problem becomes undecidable over rings of integers in real quadratic extensions of the rationals.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249

## GJ Source Entry

> [AN3] SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS (*)
> INSTANCE: Vectors a_i = (a_i[0],...,a_i[m]) and b_i = (b_i[0],...,b_i[m]), 1 ≤ i ≤ n, with positive integer entries.
> QUESTION: Do there exist positive integers x_1,x_2,...,x_m such that, for 1 ≤ i ≤ n, a_i[0] + Σ_{j=1}^m (a_i[j]·x_j) divides b_i[0] + Σ_{j=1}^m (b_i[j]·x_j)?
> Reference: [Lipshitz, 1977], [Lipshitz, 1978]. Transformation from QUADRATIC DIOPHANTINE EQUATIONS.
> Comment: Not known to be in NP, but belongs to NP for any fixed n. NP-complete for any fixed n ≥ 5. General problem is undecidable if the vector entries and the x_j are allowed to range over the ring of "integers" in a real quadratic extension of the rationals. See reference for related decidability and undecidability results.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

The reduction from QUADRATIC DIOPHANTINE EQUATIONS to SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS follows the approach of Lipshitz (1977, 1978). Given a Quadratic Diophantine Equations instance with positive integers a, b, c asking whether there exist positive integers x, y such that a*x^2 + b*y = c:

**High-level approach:**
The key insight is that quadratic expressions can be encoded using divisibility constraints on linear polynomials. The equation a*x^2 + b*y = c involves a quadratic term a*x^2 which cannot be directly expressed as a linear polynomial. However, divisibility relations between linear expressions can simulate multiplication: if (1 + t) | (1 + t*x), this encodes information about x, and by chaining such divisibility constraints, quadratic (and higher) relationships can be built.

**Construction:**

1. **Variable introduction:** The target instance uses variables x_1, ..., x_m where m is a small constant (polynomial in the description of a, b, c). Introduce auxiliary variables to linearize the quadratic term.

2. **Encoding x^2 via divisibility:** The product x*x can be encoded by introducing an auxiliary variable z and requiring:
   - z = x^2, which can be expressed through divisibility constraints.
   - Specifically, use the identity: x | z and (z - x^2 = 0), which through a sequence of divisibility constraints on linear expressions can encode the multiplication.

3. **Encoding the equation:** The equation a*x^2 + b*y = c becomes:
   - a*z + b*y = c (with z = x^2 encoded by divisibility constraints)
   - This linear equation is encoded as: 1 | (c - a*z - b*y), requiring that c - a*z - b*y = 0.
   - Combined with the divisibility constraints encoding z = x^2.

4. **Positivity constraints:** The requirement that x, y are positive integers is naturally handled by the target problem's requirement that all x_j are positive integers.

5. **Solution extraction:** Given positive integers x_1, ..., x_m satisfying all divisibility constraints, read off x and y from the appropriate variables.

**Key properties:**
- The number of divisibility constraints n is a constant (independent of the magnitudes of a, b, c) -- this is why the problem is in NP for fixed n
- The vector entries have magnitude polynomial in log(a) + log(b) + log(c)
- The number of variables m is also a small constant

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- L = total bit-length of (a, b, c) in the source Quadratic Diophantine Equations instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_constraints` (n) | O(1) -- constant number of divisibility constraints |
| `num_variables` (m) | O(1) -- constant number of variables |
| `max_coefficient` | O(max(a, b, c)) -- polynomial in the input values |

**Derivation:**
- The reduction introduces a fixed number of auxiliary variables and constraints to encode the quadratic relationship
- Coefficient sizes are bounded by the input parameters a, b, c
- The number of constraints is independent of the magnitude of a, b, c (which is why the problem is in NP for fixed n)

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce QuadraticDiophantineEquations instance to SimultaneousDivisibilityOfLinearPolynomials, solve target with BruteForce (enumerate positive integers for each variable up to a bound derived from c), extract x and y, verify a*x^2 + b*y = c
- Test with instances having known solutions and known non-solutions
- Verify that the number of constraints matches the expected constant

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QuadraticDiophantineEquations):**
a = 2, b = 3, c = 21
Question: Are there positive integers x, y such that 2x^2 + 3y = 21?

**Verification of source:**
- Try x = 3: 2*9 + 3y = 21 => 3y = 3 => y = 1. YES, x = 3, y = 1 is a solution.
- Try x = 1: 2*1 + 3y = 21 => 3y = 19 => y = 19/3 (not integer). No.
- Try x = 2: 2*4 + 3y = 21 => 3y = 13 => y = 13/3 (not integer). No.

**Constructed target instance (SimultaneousDivisibilityOfLinearPolynomials):**

Introduce variables x_1 (representing x from source), x_2 (representing y from source), x_3 (auxiliary, representing x^2).

Divisibility constraints (illustrative, simplified):
1. (x_1) | (x_3) -- x divides x^2 (i.e., x_3 = k*x_1 for some integer k)
2. (x_1) | (x_3 - x_1 + 1) -- combined with constraint 1, forces x_3 = x_1^2 (using more sophisticated encoding)
3. (1) | (21 - 2*x_3 - 3*x_2) -- encodes 2*x^2 + 3*y = 21

In vector notation with m = 3 variables:
- Constraint 1: a_1 = (0, 1, 0, 0), b_1 = (0, 0, 0, 1) -- x_1 | x_3
- Constraint 2: (encoding x_3 = x_1^2 via additional divisibility relations)
- Constraint 3: a_3 = (1, 0, 0, 0), b_3 = (21, 0, -3, -2) -- 1 | (21 - 3*x_2 - 2*x_3)

**Solution mapping:**
- Source solution: x = 3, y = 1
- Target solution: x_1 = 3, x_2 = 1, x_3 = 9
- Check constraint 1: 3 | 9 = TRUE
- Check constraint 3: 1 | (21 - 3 - 18) = 1 | 0 = TRUE


## References

- **[Lipshitz, 1977]**: [`Lipshitz1977`] Leonard Lipshitz (1977). "A remark on the {Diophantine} problem for addition and divisibility".
- **[Lipshitz, 1978]**: [`Lipshitz1978`] Leonard Lipshitz (1978). "The {Diophantine} problem for addition and divisibility". *Transactions of the American Mathematical Society* 235, pp. 271-283.
