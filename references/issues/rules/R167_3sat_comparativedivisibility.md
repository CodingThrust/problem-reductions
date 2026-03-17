---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to COMPARATIVE DIVISIBILITY"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'COMPARATIVE DIVISIBILITY'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** COMPARATIVE DIVISIBILITY
**Motivation:** Establishes NP-completeness of COMPARATIVE DIVISIBILITY via polynomial-time reduction from 3SAT. This result by Plaisted (1976) demonstrates that comparing divisibility counts -- determining whether an integer divides more elements of one sequence than another -- is computationally intractable. The problem is notable because the nondeterminism is "hidden": the problem statement does not explicitly involve choosing from alternatives, yet it is NP-complete. The reduction uses properties of prime numbers to encode Boolean satisfiability into divisibility relationships.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249

## GJ Source Entry

> [AN4] COMPARATIVE DIVISIBILITY
> INSTANCE: Sequences a_1,a_2,...,a_n and b_1,b_2,...,b_m of positive integers.
> QUESTION: Is there a positive integer c such that the number of i for which c divides a_i is more than the number of j for which c divides b_j?
> Reference: [Plaisted, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete even if all a_i are different and all b_j are different [Garey and Johnson, ——].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

The reduction from 3SAT to COMPARATIVE DIVISIBILITY follows the approach of Plaisted (1976). Given a 3SAT instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}:

**High-level approach:**
The key idea is to use prime numbers to encode truth assignments. Each Boolean variable is associated with a pair of distinct primes (one for TRUE, one for FALSE). The sequences a_i and b_i are constructed so that a divisor c that divides more a_i's than b_j's corresponds to a satisfying assignment. By Linnik's theorem, sufficiently many primes exist in arithmetic progressions, enabling the encoding.

**Construction:**

1. **Prime assignment:** For each variable u_k (1 <= k <= n), assign two distinct primes:
   - p_k for u_k = TRUE
   - q_k for u_k = FALSE
   All 2n primes are chosen to be distinct.

2. **Encoding truth assignment consistency:** The divisor c must "choose" exactly one of p_k or q_k for each variable. This is enforced by including appropriate products in the b-sequence that penalize choosing both or neither.

3. **Sequence a (reward sequence):** For each clause c_j = (l_1 ∨ l_2 ∨ l_3), add an element to the a-sequence that is the product of the primes corresponding to each literal being TRUE. Specifically:
   - For literal u_k in clause c_j: include factor p_k
   - For literal ¬u_k in clause c_j: include factor q_k
   - a_j = product of the three primes for the literals in clause c_j

   When c is a product of primes encoding a truth assignment, c divides a_j if and only if all three literals in c_j are made true -- but we want at least one literal true. A more nuanced construction is needed:

4. **Refined encoding:** Instead of a single product per clause, the construction uses multiple elements in sequence a to reward each individual literal being satisfied:
   - For each clause c_j and each literal l in c_j: add an element to the a-sequence divisible by the prime for l being true.
   - This gives 3m elements in the a-sequence.

5. **Sequence b (penalty sequence):** Add elements to penalize invalid assignments:
   - For each variable u_k: add elements to the b-sequence that are divisible by both p_k and q_k, penalizing choosing both TRUE and FALSE.
   - Add a "baseline" count to the b-sequence to ensure c must satisfy enough clauses to have a positive comparative count.

6. **Solution extraction:** Given c such that |{i : c | a_i}| > |{j : c | b_j}|, factor c to determine which primes it contains, and thus which truth values are assigned to each variable.

**Key properties:**
- The sequences have length polynomial in n + m
- All elements are products of O(n) primes, so their bit-length is O(n log n)
- The construction runs in polynomial time

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `len_a` | O(3 * num_clauses) = O(m) |
| `len_b` | O(num_vars + num_clauses) = O(n + m) |
| `max_element_bitlength` | O(n * log(n)) |

**Derivation:**
- The a-sequence has O(m) elements (one per literal-clause pair or per clause)
- The b-sequence has O(n + m) elements (variable consistency penalties + baseline)
- Each element is a product of up to n primes, each O(n log n) bits, so elements have O(n^2 log n) bits in the worst case

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce KSatisfiability<K3> instance to ComparativeDivisibility, solve target with BruteForce (enumerate candidate divisors c up to the LCM of all sequence elements), count divisibilities, verify c divides more a's than b's, extract truth assignment from prime factorization of c
- Test with both satisfiable and unsatisfiable 3SAT instances
- Verify that unsatisfiable instances yield no valid c

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (KSatisfiability<K3>):**
2 variables: u_1, u_2 (n = 2)
2 clauses (m = 2):
- c_1 = (u_1 ∨ u_2)
- c_2 = (¬u_1 ∨ u_2)

**Construction (simplified illustration):**

1. Assign primes: p_1 = 2 (u_1 = T), q_1 = 3 (u_1 = F), p_2 = 5 (u_2 = T), q_2 = 7 (u_2 = F).

2. Construct a-sequence (reward for satisfying literals):
   - c_1 = (u_1 ∨ u_2): Literals u_1 (prime 2) and u_2 (prime 5).
     - a_1 = 2 (reward for u_1 = T in c_1)
     - a_2 = 5 (reward for u_2 = T in c_1)
   - c_2 = (¬u_1 ∨ u_2): Literals ¬u_1 (prime 3) and u_2 (prime 5).
     - a_3 = 3 (reward for u_1 = F in c_2)
     - a_4 = 5 (reward for u_2 = T in c_2)

   a-sequence: [2, 5, 3, 5]

3. Construct b-sequence (penalty for inconsistency + baseline):
   - b_1 = 6 = 2*3 (divisible by both p_1 and q_1 -- penalizes choosing both T and F for u_1)
   - b_2 = 35 = 5*7 (divisible by both p_2 and q_2 -- penalizes choosing both T and F for u_2)
   - b_3 = 1 (baseline -- always divisible, acts as threshold)

   b-sequence: [6, 35, 1]

**Solution mapping:**
- Assignment u_1 = F, u_2 = T: c should be divisible by q_1 = 3 and p_2 = 5, so try c = 15.
  - a-sequence: 15 | 2? No. 15 | 5? Yes. 15 | 3? Yes. 15 | 5? Yes. Count = 3.
  - b-sequence: 15 | 6? No. 15 | 35? No. 15 | 1? No. Count = 0.
  - Comparative: 3 > 0. YES.

- Assignment u_1 = T, u_2 = T: c = 10 = 2*5.
  - a-sequence: 10 | 2? Yes. 10 | 5? Yes. 10 | 3? No. 10 | 5? Yes. Count = 3.
  - b-sequence: 10 | 6? No. 10 | 35? No. 10 | 1? No. Count = 0.
  - Comparative: 3 > 0. YES.

Both satisfying assignments (u_1=F,u_2=T and u_1=T,u_2=T) yield valid c values.


## References

- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264-267. IEEE Computer Society.
- **[Garey and Johnson, ----]**: *(not found in bibliography)*
