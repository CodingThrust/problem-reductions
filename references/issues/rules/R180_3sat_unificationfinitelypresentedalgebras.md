---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to UNIFICATION FOR FINITELY PRESENTED ALGEBRAS"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Unification for Finitely Presented Algebras'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** UNIFICATION FOR FINITELY PRESENTED ALGEBRAS
**Motivation:** Establishes NP-completeness of UNIFICATION FOR FINITELY PRESENTED ALGEBRAS via polynomial-time reduction from 3SAT. Given a finitely presented algebra (generators, operators, defining relations) and two expressions with variables, the problem asks whether there is a substitution making them equal in the algebra. Kozen (1977) showed this "schema satisfiability" problem is NP-complete by encoding 3SAT clauses into the algebraic structure, where generators model Boolean values and defining relations enforce clause satisfaction. The proof of membership in NP is non-trivial and requires showing that witnesses (substitution terms) can be bounded in size polynomially.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.253

## GJ Source Entry

> [AN17] UNIFICATION FOR FINITELY PRESENTED ALGEBRAS
> INSTANCE: Finite presentation of an algebra A in terms of a set G of generators, a collection O of operators of various finite dimensions, and a collection Γ of defining relations on well-formed formulas over G and O; two well-formed expressions e and f over G, O, and a variable set V (see reference for details).
> QUESTION: Is there an assignment to each v E V of a unique "term" I(v) over G and O such that, if I(e) and I(f) denote the expressions obtained by replacing all variables in e and f by their corresponding terms, then I(e) and I(f) represent the same element in A?
> Reference: [Kozen, 1977a], [Kozen, 1976]. Transformation from 3SAT. Proof of membership in NP is non-trivial and appears in the second reference.
> Comment: Remains NP-complete if only one of e and f contains variable symbols, but is solvable in polynomial time if neither contains variable symbols. See [Kozen, 1977b] for quantified versions of this problem that are complete for PSPACE and for the various levels of the polynomial hierarchy.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with variables x_1, ..., x_n and clauses C_1, ..., C_m, construct a UNIFICATION FOR FINITELY PRESENTED ALGEBRAS instance as follows:

1. **Algebra construction:** Define a finitely presented algebra A with:
   - **Generators:** G = {0, 1} representing Boolean values FALSE and TRUE.
   - **Operators:** A binary operator ∨ (or) and a unary operator ¬ (not), plus possibly auxiliary operators.
   - **Defining relations:** Γ encodes the Boolean algebra identities needed for clause evaluation:
     - ¬0 = 1, ¬1 = 0
     - 0 ∨ 0 = 0, 0 ∨ 1 = 1, 1 ∨ 0 = 1, 1 ∨ 1 = 1
     - Additional relations to ensure the algebra behaves as the two-element Boolean algebra.

2. **Variable set:** V = {v_1, ..., v_n}, one for each Boolean variable.

3. **Expression construction:** For each clause C_j = (l_1 ∨ l_2 ∨ l_3), construct an expression e_j = l_1' ∨ (l_2' ∨ l_3') where l_k' is v_i if literal l_k = x_i, or ¬v_i if l_k = ¬x_i.
   - Let e = e_1 ∧ e_2 ∧ ... ∧ e_m (the conjunction of all clause expressions, encoded using an additional binary "and" operator with appropriate relations).
   - Let f = 1 (the generator representing TRUE).
   - Alternatively, encode as m separate equation pairs (e_j, 1) for j = 1, ..., m by introducing a conjunction operator.

4. **Correctness:** A substitution I mapping each v_i to either 0 or 1 makes I(e) = I(f) = 1 in algebra A if and only if the assignment x_i = I(v_i) satisfies all clauses. The algebra's defining relations ensure that the expressions evaluate according to Boolean logic.

5. **Solution extraction:** Given a unifying substitution I, the satisfying assignment is x_i = I(v_i) for all i.

**Note:** The proof that UNIFICATION FOR FINITELY PRESENTED ALGEBRAS is in NP is non-trivial. The key insight (from Kozen, 1976) is that the substitution terms I(v) can be bounded in size by a polynomial in the input, even though the algebra may be infinite. This uses properties of finitely presented algebras and the congruence closure of the defining relations.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in 3SAT instance
- m = number of clauses in 3SAT instance

| Target metric (code name)    | Polynomial (using symbols above) |
|------------------------------|----------------------------------|
| `num_generators`             | 2 (constants 0 and 1)           |
| `num_operators`              | O(1)                             |
| `num_defining_relations`     | O(1) (fixed Boolean identities) |
| `num_variables`              | n                                |
| `expression_size`            | O(m) (one clause term per clause)|

**Derivation:** The algebra presentation is constant-size (it encodes the two-element Boolean algebra). The expressions e and f have size proportional to the number of clauses m, with each clause contributing O(1) symbols. The variable set has size n. Total construction size is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3SAT instance, reduce to UNIFICATION FOR FINITELY PRESENTED ALGEBRAS, enumerate all 2^n substitutions mapping variables to generators {0, 1}, check that a unifying substitution exists iff the formula is satisfiable.
- Verify that the algebra correctly computes Boolean operations by checking the defining relations.
- Edge cases: test with a single clause, test with contradictory clauses (unsatisfiable), test with one expression containing variables and the other being a constant (per the GJ comment, still NP-complete).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3
Clauses: C_1 = (x_1 ∨ ¬x_2 ∨ x_3), C_2 = (¬x_1 ∨ x_2 ∨ ¬x_3)
(n = 3 variables, m = 2 clauses)

**Constructed UNIFICATION instance:**
- Algebra A: generators G = {0, 1}, operators O = {∨ (binary), ¬ (unary)}, relations Γ = {¬0 = 1, ¬1 = 0, 0∨0 = 0, 0∨1 = 1, 1∨0 = 1, 1∨1 = 1}
- Variables: V = {v_1, v_2, v_3}
- Expression e: (v_1 ∨ (¬v_2 ∨ v_3)) ∧ (¬v_1 ∨ (v_2 ∨ ¬v_3)), where ∧ is encoded via additional operator with appropriate relations, or as two separate equations.
- Expression f: 1

**Solution:**
Substitution I(v_1) = 1, I(v_2) = 1, I(v_3) = 0:
- Clause 1: (1 ∨ (¬1 ∨ 0)) = (1 ∨ (0 ∨ 0)) = (1 ∨ 0) = 1 ✓
- Clause 2: (¬1 ∨ (1 ∨ ¬0)) = (0 ∨ (1 ∨ 1)) = (0 ∨ 1) = 1 ✓
- Both clauses evaluate to 1 = f in algebra A. ✓

**Solution extraction:**
x_1 = TRUE, x_2 = TRUE, x_3 = FALSE ✓


## References

- **[Kozen, 1977a]**: [`Kozen1977a`] Dexter Kozen (1977). "Complexity of finitely presented algebras". In: *Proceedings of the 9th Annual ACM Symposium on Theory of Computing*, pp. 164–177. Association for Computing Machinery.
- **[Kozen, 1976]**: [`Kozen1976`] Dexter Kozen (1976). "Complexity of finitely presented algebras". Dept. of Computer Science, Cornell University.
- **[Kozen, 1977b]**: [`Kozen1977b`] Dexter Kozen (1977). "Finitely presented algebras and the polynomial time hierarchy". Dept. of Computer Science, Cornell University.
