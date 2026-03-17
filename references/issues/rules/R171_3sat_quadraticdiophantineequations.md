---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to QUADRATIC DIOPHANTINE EQUATIONS"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Quadratic Diophantine Equations'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT (KSatisfiability in codebase)
**Target:** QUADRATIC DIOPHANTINE EQUATIONS
**Motivation:** Establishes NP-completeness of Quadratic Diophantine Equations via polynomial-time reduction from 3SAT. Due to Manders and Adleman (1978), this is a foundational result connecting Boolean satisfiability to number-theoretic problems. The reduction shows that even the simple equation ax^2 + by = c with positive integer unknowns is NP-complete, despite the facts that (i) single-variable polynomial equations are solvable in polynomial time, and (ii) linear Diophantine equations in any number of variables are also polynomial-time solvable. The result implies NP-completeness of deciding quadratic congruences z^2 = alpha (mod beta) with bounded solutions.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.250

## GJ Source Entry

> [AN8] QUADRATIC DIOPHANTINE EQUATIONS
> INSTANCE: Positive integers a, b, and c.
> QUESTION: Are there positive integers x and y such that ax^2 + by = c?
> Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
> Comment: Diophantine equations of the forms ax^k = c and Σ_{i=1}^k a_i·x_i = c are solvable in polynomial time for arbitrary values of k. The general Diophantine problem, "Given a polynomial with integer coefficients in k variables, does it have an integer solution?" is undecidable, even for k = 13 [Matijasevic and Robinson, 1975]. However, the given problem can be generalized considerably (to simultaneous equations in many variables) while remaining in NP, so long as only one variable enters into the equations in a non-linear way (see [Gurari and Ibarra, 1978]).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

The reduction proceeds through an intermediate step via quadratic congruences. Manders and Adleman first reduce 3SAT to QUADRATIC CONGRUENCES (deciding whether there exists z <= gamma such that z^2 = alpha (mod beta)), and then show this is equivalent to the Quadratic Diophantine Equations problem.

Given a 3SAT instance with n Boolean variables x_1, ..., x_n and m clauses C_1, ..., C_m:

1. **Variable encoding via primes:** Assign distinct odd primes q_1, q_2, ..., q_n to variables x_1, ..., x_n. These primes serve as moduli for encoding variable assignments.

2. **Clause encoding via quadratic residues:** For each clause C_j, construct a system of quadratic congruence constraints. The key insight is that for an odd prime q, the quadratic residues modulo q partition {1, ..., q-1} into two equal halves. This binary partition naturally encodes TRUE/FALSE:
   - x_i = TRUE iff z is a quadratic residue mod q_i
   - x_i = FALSE iff z is a quadratic non-residue mod q_i

3. **Chinese Remainder Theorem combination:** Use CRT to combine the per-variable congruence conditions into a single congruence modulo beta = q_1 * q_2 * ... * q_n. For each clause C_j, the set of "satisfying" residues modulo beta (those corresponding to truth assignments satisfying the clause) forms a subset S_j of Z/beta*Z.

4. **Intersection of satisfying residues:** The 3SAT formula is satisfiable iff the intersection S_1 ∩ S_2 ∩ ... ∩ S_m is non-empty. This intersection can be expressed as a single quadratic congruence z^2 = alpha (mod beta) with an appropriate bound z <= gamma.

5. **Conversion to ax^2 + by = c form:** The quadratic congruence z^2 = alpha (mod beta) with z <= gamma is equivalent to asking: do there exist positive integers x, y such that x^2 - alpha = beta * y', which can be rewritten as ax^2 + by = c with:
   - a = 1
   - b = beta (= product of the chosen primes)
   - c = alpha + beta (adjusted for positive integer requirement)
   - x corresponds to z, y corresponds to the quotient

6. **Correctness:** A satisfying assignment for the 3SAT formula exists if and only if the constructed equation ax^2 + by = c has a solution in positive integers. The quadratic residuosity conditions, combined via CRT, ensure a bijection between satisfying truth assignments and valid solutions (x, y).

**Key technical detail:** The choice of primes q_i must be large enough that quadratic residues/non-residues can encode the clause constraints. Manders and Adleman show this can be done in polynomial time.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of Boolean variables (`num_variables` of source 3SAT instance)
- m = number of clauses (`num_clauses` of source 3SAT instance)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `bit_length_a` | O(1) -- a is typically 1 |
| `bit_length_b` | O(n log n) -- product of n primes, each O(log n) bits |
| `bit_length_c` | O(n log n) -- comparable to b |

**Derivation:** The modulus beta is a product of n distinct primes. By the prime number theorem, the i-th prime is approximately i ln i, so each prime requires O(log n) bits. The product of n such primes requires O(n log n) bits. The values alpha and c are bounded by beta, so they also have O(n log n) bit-length. The coefficient a is typically a small constant (often 1). The overall encoding size is polynomial in n, but the integers themselves grow exponentially in the number of bits.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small 3SAT instance (KSatisfiability<K3>), reduce to positive integers (a, b, c), then search for positive integer solutions (x, y) to ax^2 + by = c using bounded enumeration (x ranges from 1 to sqrt(c/a)).
- Satisfiable case: verify that a solution (x, y) exists and maps back to a satisfying assignment.
- Unsatisfiable case: use a known unsatisfiable 3SAT instance, verify no solution exists.
- Membership in NP check: verify that given a candidate (x, y), checking ax^2 + by = c is polynomial time.
- Edge cases: test with a single clause, test with pure positive/negative literal formulas.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability<K3>):**

Variables: x_1, x_2 (n = 2)
Clauses (m = 1):
- C_1: (x_1 OR x_2 OR x_1) simplified to (x_1 OR x_2)

Satisfying assignments: (T,T), (T,F), (F,T). Formula is satisfiable.

**Prime assignment:** q_1 = 3, q_2 = 5.
Modulus beta = q_1 * q_2 = 15.

**Quadratic residue encoding:**
- Quadratic residues mod 3: {1} (since 1^2 = 1 mod 3, 2^2 = 1 mod 3). QR mod 3 = {1}, QNR mod 3 = {2}.
- Quadratic residues mod 5: {1, 4} (since 1^2=1, 2^2=4, 3^2=4, 4^2=1 mod 5). QR mod 5 = {1, 4}, QNR mod 5 = {2, 3}.

**Truth assignment encoding:**
- x_1 = TRUE iff z = 1 (mod 3); x_1 = FALSE iff z = 2 (mod 3)
- x_2 = TRUE iff z in {1,4} (mod 5); x_2 = FALSE iff z in {2,3} (mod 5)

**Clause constraint:** C_1 = (x_1 OR x_2) is satisfied unless x_1 = FALSE AND x_2 = FALSE, i.e., z = 2 (mod 3) AND z in {2,3} (mod 5). By CRT: z = 2 (mod 3) and z = 2 (mod 5) gives z = 2 (mod 15); z = 2 (mod 3) and z = 3 (mod 5) gives z = 8 (mod 15). So the "unsatisfying" residues are {2, 8} mod 15.

**Conversion to quadratic form:** We need z^2 = alpha (mod 15) with z in the satisfying set. One satisfying residue class: z = 1 (mod 15) (corresponding to x_1 = TRUE, x_2 = TRUE). Then z^2 = 1 (mod 15), so alpha = 1.

**Constructed Diophantine instance:**
- a = 1, b = 15, c = 16
- Question: do positive integers x, y exist with x^2 + 15y = 16?

**Solution:** x = 1, y = 1: 1^2 + 15*1 = 1 + 15 = 16 = c. YES.

**Solution extraction:** x = 1 corresponds to z = 1. z = 1 mod 3 = 1 (QR, so x_1 = TRUE). z = 1 mod 5 = 1 (QR, so x_2 = TRUE). Assignment: x_1 = TRUE, x_2 = TRUE.
- C_1: TRUE OR TRUE = TRUE. Verified.


## References

- **[Manders and Adleman, 1978]**: [`Manders1978`] Kenneth Manders and Leonard Adleman (1978). "{NP}-complete decision problems for binary quadratics". *Journal of Computer and System Sciences* 16, pp. 168-184.
- **[Matijasevic and Robinson, 1975]**: [`Matijasevic1975`] Yuri V. Matijasevic and Julia Robinson (1975). "Reduction of an arbitrary {Diophantine} equation to one in 13 unknowns". *Acta Arithmetica* 27, pp. 521-553.
- **[Gurari and Ibarra, 1978]**: [`Gurari1978`] E. M. Gurari and O. H. Ibarra (1978). "An {NP}-complete number theoretic problem". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 205-215. Association for Computing Machinery.
