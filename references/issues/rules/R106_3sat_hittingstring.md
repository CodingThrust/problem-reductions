---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Hitting String"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Hitting String'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Hitting String
**Motivation:** Establishes NP-completeness of HITTING STRING via polynomial-time reduction from 3SAT. The reduction is natural and direct: each Boolean variable maps to a position in the binary string, and each 3-literal clause maps to a pattern string over {0,1,*}. A truth assignment satisfies a clause if and only if the corresponding binary string "hits" (agrees on at least one non-* position with) the pattern derived from that clause. This makes Hitting String essentially a matrix-based reformulation of SAT.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, SR12, p.229

## GJ Source Entry

> [SR12] HITTING STRING
> INSTANCE: Finite set A of strings over {0,1,*}, all having the same length n.
> QUESTION: Is there a string x E {0,1}* with |x| = n such that for each string a E A there is some i, 1 <= i <= n, for which the i^th symbol of a and the i^th symbol of x are identical?
> Reference: [Fagin, 1974]. Transformation from 3SAT.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m (each clause has exactly 3 literals), construct a Hitting String instance as follows:

1. **String length:** Set the string length to n (one position per variable).

2. **Pattern strings:** For each clause C_j, create a pattern string a_j of length n over {0,1,*}:
   - For each position i (1 <= i <= n):
     - If x_i appears as a positive literal in C_j, set a_j[i] = 1 (the "true" value that would satisfy this literal).
     - If x_i appears as a negative literal (not x_i) in C_j, set a_j[i] = 0 (the "true" assignment for x_i is 1, but the negated literal is satisfied when x_i = 0).
     - If x_i does not appear in C_j, set a_j[i] = * (wildcard / don't care).

3. **Set A:** The set A = {a_1, a_2, ..., a_m} contains exactly m pattern strings, each of length n.

4. **Correctness (forward):** If the 3SAT instance is satisfiable with assignment sigma, construct x in {0,1}^n where x[i] = sigma(x_i). For each clause C_j, at least one literal is true under sigma. If that literal involves variable x_i, then x[i] agrees with a_j[i] on a non-* position, so x hits a_j.

5. **Correctness (reverse):** If x is a hitting string for A, define sigma(x_i) = x[i]. For each clause C_j, x must agree with a_j on some non-* position i. By construction, a_j[i] encodes the satisfying value for the literal involving x_i in clause C_j, so that literal is true under sigma, meaning C_j is satisfied.

**Key invariant:** The non-* positions in each pattern a_j correspond exactly to the 3 variables appearing in clause C_j, and the values at those positions encode which truth values satisfy the corresponding literals. Hitting = satisfying at least one literal.

**Time complexity of reduction:** O(m * n) to construct the m pattern strings of length n. Since each clause has exactly 3 literals, the total work per pattern is O(n) (initialize to *, then set 3 positions).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_variables` of source 3SAT instance (number of Boolean variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `string_length` | `num_variables` |
| `num_patterns` | `num_clauses` |

**Derivation:** Each variable becomes one position in the string (length = n). Each clause becomes one pattern string (|A| = m). Each pattern has exactly 3 non-* entries (corresponding to the 3 literals in the clause).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a KSatisfiability(k=3) instance to HittingString, solve target with BruteForce (enumerate all 2^n binary strings, check each against all patterns), extract solution, verify on source
- Test with known YES instance: (x1 v x2 v x3) ^ (~x1 v x2 v ~x3) with n=3, m=2; patterns: {1,1,1}, {0,1,0}; hitting string "110" should work
- Test with known NO instance: construct an unsatisfiable 3SAT formula and verify that no hitting string exists
- Verify bijection: each satisfying assignment maps to exactly one hitting string and vice versa

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability k=3):**
Variables: x_1, x_2, x_3, x_4, x_5, x_6 (n = 6)
Clauses (m = 7):
- C_1: (x_1 v x_2 v x_3)
- C_2: (~x_1 v x_3 v x_4)
- C_3: (x_2 v ~x_3 v x_5)
- C_4: (~x_2 v x_4 v x_6)
- C_5: (x_1 v ~x_4 v ~x_5)
- C_6: (~x_1 v ~x_2 v x_6)
- C_7: (x_3 v x_5 v ~x_6)

**Constructed target instance (HittingString):**
String length: n = 6
Pattern set A (7 strings of length 6):
- a_1 = 1 1 1 * * * (from C_1: x_1=T, x_2=T, x_3=T)
- a_2 = 0 * 1 1 * * (from C_2: ~x_1, x_3=T, x_4=T)
- a_3 = * 1 0 * 1 * (from C_3: x_2=T, ~x_3, x_5=T)
- a_4 = * 0 * 1 * 1 (from C_4: ~x_2, x_4=T, x_6=T)
- a_5 = 1 * * 0 0 * (from C_5: x_1=T, ~x_4, ~x_5)
- a_6 = 0 0 * * * 1 (from C_6: ~x_1, ~x_2, x_6=T)
- a_7 = * * 1 * 1 0 (from C_7: x_3=T, x_5=T, ~x_6)

**Solution mapping:**
Consider the truth assignment: x_1=T, x_2=T, x_3=T, x_4=F, x_5=T, x_6=F
Corresponding binary string: x = 1 1 1 0 1 0

Verification (x hits each pattern):
- a_1 = 1 1 1 * * *: x[1]=1=a_1[1] (hit at position 1)
- a_2 = 0 * 1 1 * *: x[3]=1=a_2[3] (hit at position 3)
- a_3 = * 1 0 * 1 *: x[2]=1=a_3[2] (hit at position 2)
- a_4 = * 0 * 1 * 1: x[2]=1 != a_4[2]=0, but x[4]=0 != a_4[4]=1, x[6]=0 != a_4[6]=1... Let's re-check.
  Actually: a_4[2]=0, x[2]=1 (no); a_4[4]=1, x[4]=0 (no); a_4[6]=1, x[6]=0 (no).
  Clause C_4: (~x_2 v x_4 v x_6) = (F v F v F) = F. So C_4 is not satisfied.

Revised assignment: x_1=T, x_2=F, x_3=T, x_4=T, x_5=T, x_6=T
Corresponding binary string: x = 1 0 1 1 1 1

Verification:
- C_1: (T v F v T) = T. a_1 = 1 1 1 * * *: x[1]=1=a_1[1] (hit)
- C_2: (F v T v T) = T. a_2 = 0 * 1 1 * *: x[3]=1=a_2[3] (hit)
- C_3: (F v F v T) = T. a_3 = * 1 0 * 1 *: x[5]=1=a_3[5] (hit)
- C_4: (T v T v T) = T. a_4 = * 0 * 1 * 1: x[2]=0=a_4[2] (hit)
- C_5: (T v F v F) = T. a_5 = 1 * * 0 0 *: x[1]=1=a_5[1] (hit)
- C_6: (F v T v T) = T. a_6 = 0 0 * * * 1: x[2]=0=a_6[2] (hit)
- C_7: (T v T v F) = T. a_7 = * * 1 * 1 0: x[3]=1=a_7[3] (hit)

All 7 clauses are satisfied, and x = 101111 hits all 7 patterns.

**Reverse mapping:**
The hitting string x = 101111 directly gives the satisfying assignment:
x_1=1(T), x_2=0(F), x_3=1(T), x_4=1(T), x_5=1(T), x_6=1(T).


## References

- **[Fagin, 1974]**: [`Fagin1974`] R. Fagin (1974). "Generalized first-order spectra and polynomial time recognizable sets". In: *Complexity of Computation*, SIAM-AMS Proceedings Vol. 7, pp. 43-73. American Mathematical Society.
