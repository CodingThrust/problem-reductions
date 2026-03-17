---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to UNIFICATION WITH COMMUTATIVE OPERATORS"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Unification with Commutative Operators'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** UNIFICATION WITH COMMUTATIVE OPERATORS
**Motivation:** Establishes NP-completeness of UNIFICATION WITH COMMUTATIVE OPERATORS via polynomial-time reduction from 3SAT. Standard (non-commutative) unification is solvable in polynomial time by the Paterson-Wegman algorithm (1976), but adding commutativity to a single binary operator makes the problem NP-complete. The reduction encodes Boolean variables as unification variables and clauses as equation pairs, exploiting commutativity to represent the "or" of literals. This result, due to Sethi (1977), delineates a sharp complexity boundary in automated theorem proving and term rewriting systems.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.252

## GJ Source Entry

> [AN16] UNIFICATION WITH COMMUTATIVE OPERATORS
> INSTANCE: Set V of variables, set C of constants, ordered pairs (e_i,f_i), 1 ≤ i ≤ n, of "expressions," where an expression is either a variable from V, a constant from C, or (e + f) where e and f are expressions.
> QUESTION: Is there an assignment to each v E V of a variable-free expression I(v) such that, if I(e) denotes the expression obtained by replacing each occurrence of each variable v in e by I(v), then I(e_i) ≡ I(f_i) for 1 ≤ i ≤ n, where e ≡ f if e = f or if e = (a+b), f = (c+d), and either a ≡ c and b ≡ d or a ≡ d and b ≡ c?
> Reference: [Sethi, 1977b]. Transformation from 3SAT.
> Comment: Remains NP-complete even if no e_j or f_j contains more than 7 occurrences of constants and variables. The variant in which the operator is non-commutative (and hence e ≡ f only if e = f) is solvable in polynomial time [Paterson and Wegman, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with variables x_1, ..., x_n and clauses C_1, ..., C_m, construct a UNIFICATION WITH COMMUTATIVE OPERATORS instance as follows:

1. **Constants and variables:** Create constants T (true) and F (false). For each Boolean variable x_i, create a unification variable v_i that will be assigned either T or F.

2. **Encoding Boolean values:** For each Boolean variable x_i, create an equation pair that constrains v_i to be either T or F. This can be done using auxiliary constants and the commutative operator "+".

3. **Clause encoding:** For each clause C_j = (l_1 ∨ l_2 ∨ l_3), construct expression pairs (e_j, f_j) using the commutative operator "+" such that:
   - The expression e_j encodes the clause structure using the variables v_i (or their complements).
   - The expression f_j is a target expression using constants.
   - The commutativity of "+" allows the matcher to "choose" which literal satisfies the clause: since (a+b) ≡ (b+a), the unifier can permute operands to find a matching, which models the disjunction.

4. **Complement encoding:** For literal ¬x_i, use an expression that unifies with T exactly when v_i = F, leveraging the commutative structure. For example, encode (v_i + c_i) where c_i is chosen so that the equation is satisfiable iff v_i takes the complementary value.

5. **Correctness:** A unifying substitution I exists if and only if the Boolean assignment x_i = (I(v_i) = T) satisfies all clauses. Commutativity provides exactly the nondeterminism needed to model the OR of three literals per clause.

6. **Solution extraction:** Given a unifier I, set x_i = TRUE if I(v_i) = T, and x_i = FALSE if I(v_i) = F.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in 3SAT instance
- m = number of clauses in 3SAT instance

| Target metric (code name)   | Polynomial (using symbols above) |
|-----------------------------|----------------------------------|
| `num_variables`             | n                                |
| `num_constants`             | O(n + m)                         |
| `num_equation_pairs`        | O(n + m)                         |
| `max_expression_size`       | O(1) (at most 7 occurrences)     |

**Derivation:** Each Boolean variable contributes one unification variable and O(1) equation pairs for the Boolean constraint. Each clause contributes O(1) equation pairs with constant-size expressions (at most 7 occurrences of constants and variables per the GJ comment). Total construction size is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3SAT instance, reduce to UNIFICATION WITH COMMUTATIVE OPERATORS, enumerate all possible substitutions mapping variables to ground terms over the constants, check that a unifier exists iff the formula is satisfiable.
- Check that the GJ bound holds: no expression contains more than 7 occurrences of constants and variables.
- Edge cases: test with a single clause (3 variables), test with a contradiction (unsatisfiable formula), test that removing commutativity makes the problem tractable.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3
Clauses: C_1 = (x_1 ∨ x_2 ∨ ¬x_3)
(n = 3 variables, m = 1 clause)

**Constructed UNIFICATION instance (conceptual):**
- Constants: {T, F, c_1, c_2, c_3, d}
- Variables: {v_1, v_2, v_3}
- Equation pairs enforce:
  1. v_i ∈ {T, F} for i = 1, 2, 3 (Boolean constraint pairs)
  2. Clause pair: an expression built from (v_1 + (v_2 + v_3')) where v_3' encodes ¬x_3, matched against a target expression using constants, such that commutativity allows any one of the three "slots" to provide the satisfying literal.

**Solution:**
Assignment x_1 = TRUE, x_2 = FALSE, x_3 = FALSE satisfies C_1 = (TRUE ∨ FALSE ∨ TRUE) = TRUE.
The corresponding unifier sets I(v_1) = T, I(v_2) = F, I(v_3) = F, and the equation pairs are all satisfiable under commutativity.

**Solution extraction:**
x_1 = TRUE (I(v_1) = T), x_2 = FALSE (I(v_2) = F), x_3 = FALSE (I(v_3) = F) ✓


## References

- **[Sethi, 1977b]**: [`Sethi1977b`] R. Sethi (1977). "Complete problems for closure classes of WRTs". Unpublished manuscript, Bell Laboratories.
- **[Paterson and Wegman, 1976]**: [`Paterson and Wegman1976`] M. S. Paterson and M. N. Wegman (1976). "Linear unification". *Journal of Computer and System Sciences* 16, pp. 158–167.
