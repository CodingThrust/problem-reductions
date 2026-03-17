---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SUBSET SUM to INTEGER EXPRESSION MEMBERSHIP"
labels: rule
assignees: ''
canonical_source_name: 'Subset Sum'
canonical_target_name: 'Integer Expression Membership'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** SUBSET SUM
**Target:** INTEGER EXPRESSION MEMBERSHIP
**Motivation:** Establishes NP-completeness of INTEGER EXPRESSION MEMBERSHIP via polynomial-time reduction from SUBSET SUM. Integer expressions use union (∪) and addition (+) operations on sets of positive integers; the membership problem asks whether a given integer K belongs to the set denoted by an expression. Stockmeyer and Meyer (1973) showed this is NP-complete by encoding subset sum as a membership query. The result is notable because related problems (INEQUIVALENCE with the same operators) are complete for the second level of the polynomial hierarchy (Sigma_2^p), and adding complementation makes both problems PSPACE-complete.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.253

## GJ Source Entry

> [AN18] INTEGER EXPRESSION MEMBERSHIP
> INSTANCE: Integer expression e over the operations ∪ and +, where if n E Z+, the binary representation of n is an integer expression representing n, and if f and g are integer expressions representing the sets F and G, then f ∪ g is an integer expression representing the set F ∪ G and f + g is an integer expression representing the set {m + n: m E F and n E G}, and a positive integer K.
> QUESTION: Is K in the set represented by e?
> Reference: [Stockmeyer and Meyer, 1973]. Transformation from SUBSET SUM.
> Comment: The related INTEGER EXPRESSION INEQUIVALENCE problem, "given two integer expressions e and f, do they represent different sets?" is NP-hard and in fact complete for Σ_2^p in the polynomial hierarchy ([Stockmeyer and Meyer, 1973], [Stockmeyer, 1976a], see also Section 7.2). If the operator "¬" is allowed, with ¬e representing the set of all positive integers not represented by e, then both the membership and inequivalence problems become PSPACE-complete [Stockmeyer and Meyer, 1973].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a SUBSET SUM instance with set A = {a_1, a_2, ..., a_n} of positive integers and target B, construct an INTEGER EXPRESSION MEMBERSHIP instance as follows:

1. **Element expressions:** For each element a_i ∈ A, create the set expression S_i = (0 ∪ a_i), which represents the set {0, a_i}. Here, 0 represents the integer 0 (or we use a shifted encoding where each element is either included or excluded).

   Since the integer expression language only allows positive integers as atoms, we use a shifted encoding: let S_i = a_i (representing the singleton set {a_i}) and let Z_i = 0' where 0' is encoded as appropriate. Alternatively, we directly construct:

2. **Direct encoding using + and ∪:** For each a_i, create a "choice" expression c_i that represents {0, a_i}. Since 0 is not a positive integer, we shift: let each element contribute either 0 or a_i to the sum, and adjust the target accordingly. Concretely:
   - Let c_i = (a_i ∪ (a_i + 0_expr)) where we need a representation of the "no contribution" case.

   A cleaner formulation: encode each element as a singleton {a_i}, and build the expression:
   - e = ({a_1} ∪ {0_shift}) + ({a_2} ∪ {0_shift}) + ... + ({a_n} ∪ {0_shift})

   where + is the set addition (Minkowski sum) and ∪ is union. Since we cannot represent {0} directly (atoms must be positive), we shift all values by 1:
   - Let c_i = (1 ∪ (a_i + 1)), representing {1, a_i + 1}. This means "include 1 (= don't pick a_i) or a_i + 1 (= pick a_i)."
   - Let e = c_1 + c_2 + ... + c_n (Minkowski sum of all choice expressions).
   - The set represented by e contains all values of the form Σ_{i=1}^n d_i where d_i ∈ {1, a_i + 1}.
   - Set K = B + n (accounting for the shift: picking a_i contributes a_i + 1, not picking contributes 1, so the sum of contributions for subset A' is Σ_{a_i ∈ A'} (a_i + 1) + Σ_{a_i ∉ A'} 1 = Σ_{a_i ∈ A'} a_i + n = B + n).

3. **Correctness:** K = B + n is in the set represented by e if and only if there exists a choice d_i ∈ {1, a_i+1} for each i such that Σ d_i = B + n, which holds iff there exists A' ⊆ A with Σ_{a_i ∈ A'} a_i = B.

4. **Solution extraction:** Given that K ∈ [[e]], trace the Minkowski sum decomposition: for each i, if d_i = a_i + 1, include a_i in A'; if d_i = 1, exclude a_i.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in SUBSET SUM instance
- b = max bit-length of any element or target

| Target metric (code name)   | Polynomial (using symbols above) |
|-----------------------------|----------------------------------|
| `expression_size`           | O(n) (n union/addition nodes)    |
| `num_atoms`                 | 2n (two integer atoms per element)|
| `target_K`                  | B + n                            |
| `max_atom_value`            | max(a_i) + 1                     |

**Derivation:** Each element a_i contributes one union node and one addition node (plus two integer atoms). The overall expression is a chain of n-1 additions. Total expression size is O(n), and each atom requires O(b) bits. Construction is O(n * b).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a SUBSET SUM instance, reduce to INTEGER EXPRESSION MEMBERSHIP, evaluate the integer expression by computing the represented set (feasible for small instances), verify K is in the set iff SUBSET SUM has a solution.
- Check that all atoms in the expression are positive integers.
- Edge cases: test with n = 1 (single element, K = a_1 + 1 iff element is selected), test with B = 0 (K = n, always achievable by selecting no elements), test with B = Σa_i (K = Σa_i + n, select all elements).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (SUBSET SUM):**
A = {3, 5, 7} (n = 3 elements)
Target B = 8 (is there A' ⊆ A with sum = 8? Yes: A' = {3, 5})

**Constructed INTEGER EXPRESSION MEMBERSHIP instance:**
- Choice expressions:
  - c_1 = (1 ∪ 4) representing {1, 4} (1 = skip, 4 = 3+1 = pick a_1)
  - c_2 = (1 ∪ 6) representing {1, 6} (1 = skip, 6 = 5+1 = pick a_2)
  - c_3 = (1 ∪ 8) representing {1, 8} (1 = skip, 8 = 7+1 = pick a_3)
- Expression: e = c_1 + c_2 + c_3 = (1 ∪ 4) + (1 ∪ 6) + (1 ∪ 8)
- Target: K = B + n = 8 + 3 = 11

**Set represented by e:**
All possible sums Σ d_i where d_i ∈ {1, a_i+1}:
- (1,1,1): 1+1+1 = 3
- (4,1,1): 4+1+1 = 6
- (1,6,1): 1+6+1 = 8
- (1,1,8): 1+1+8 = 10
- (4,6,1): 4+6+1 = 11 ✓
- (4,1,8): 4+1+8 = 13
- (1,6,8): 1+6+8 = 15
- (4,6,8): 4+6+8 = 18

Set = {3, 6, 8, 10, 11, 13, 15, 18}

**Answer:** Is K = 11 in the set? **YES** ✓

**Solution extraction:**
The decomposition giving 11 = 4 + 6 + 1 corresponds to d_1 = 4 (pick a_1 = 3), d_2 = 6 (pick a_2 = 5), d_3 = 1 (skip a_3 = 7).
A' = {3, 5}, sum = 3 + 5 = 8 = B ✓


## References

- **[Stockmeyer and Meyer, 1973]**: [`Stockmeyer and Meyer1973`] Larry J. Stockmeyer and Albert R. Meyer (1973). "Word problems requiring exponential time". In: *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1–9. Association for Computing Machinery.
- **[Stockmeyer, 1976a]**: [`Stockmeyer1976a`] Larry J. Stockmeyer (1976). "The polynomial-time hierarchy". *Theoretical Computer Science* 3, pp. 1–22.
