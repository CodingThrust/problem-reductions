---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Tableau Equivalence"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'TABLEAU EQUIVALENCE'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Tableau Equivalence
**Motivation:** Establishes NP-completeness of TABLEAU EQUIVALENCE via polynomial-time reduction from 3SAT. This reduction connects Boolean satisfiability to relational algebra optimization: determining whether two tableaux (matrix representations of relational expressions) are weakly equivalent under universal interpretations is at least as hard as satisfiability. This result by Aho, Sagiv, and Ullman (1979) is foundational to relational database query optimization, showing that general tableau equivalence checking cannot be done in polynomial time (unless P = NP), even though restricted subclasses admit efficient algorithms.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.234

## GJ Source Entry

> [SR32] TABLEAU EQUIVALENCE
> INSTANCE: A set A of attribute names, a collection F of ordered pairs of subsets of A, a set X of distinguished variables, a set Y of undistinguished variables, a set C_a of constants for each a E A, and two "tableaux" T_1 and T_2 over X, Y, and the C_a. (A tableau is essentially a matrix with a column for each attribute and entries from X, Y, the C_a, along with a blank symbol. For details and an interpretation in terms of relational expressions, see reference.)
> QUESTION: Are T_1 and T_2 "weakly equivalent," i.e., do they represent identical relations under "universal interpretations"?
> Reference: [Aho, Sagiv, and Ullman, 1978]. Transformation from 3SAT.
> Comment: Remains NP-complete even if the tableaux come from "expressions" that have no "select" operations, or if the tableaux come from expressions that have select operations but F is empty, or if F is empty, the tableaux contain no constants, and the tableaux do not necessarily come from expressions at all. Problem is solvable in polynomial time for "simple" tableaux. The same results hold also for "strong equivalence," where the two tableaux must represent identical relations under all interpretations. The problem of tableau "containment," however, is NP-complete even for simple tableaux and for still further restricted tableaux [Sagiv and Yannakakis, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance phi with n variables x_1, ..., x_n and m clauses C_1, ..., C_m (each clause having exactly 3 literals), construct a Tableau Equivalence instance as follows:

1. **Attribute set construction:** Create an attribute set A with attributes encoding the Boolean variables and clauses. Include one attribute per variable and additional attributes for clause satisfaction structure. The set A has O(n + m) attributes.

2. **Functional dependencies:** Construct a collection F of functional dependencies that encode the relationship between variable assignments and clause satisfaction. The FDs ensure that consistent truth assignments are captured in the tableaux structure.

3. **Tableau T_1 construction:** Build tableau T_1 to represent the "satisfying structure" of phi. Each row in T_1 corresponds to a partial assignment that satisfies at least one literal in a clause. The distinguished variables X encode the output attributes, and the undistinguished variables Y encode the existential choices (which literal satisfies each clause). T_1 has O(m) rows (one per clause, with entries encoding the 3 possible satisfying literals) and O(n + m) columns.

4. **Tableau T_2 construction:** Build T_2 to represent the "trivially true" structure — a simple tableau that would be equivalent to T_1 only if phi is satisfiable. T_2 is constructed so that it represents the same relation as T_1 under universal interpretations if and only if the satisfying assignment exists.

5. **Equivalence condition:** T_1 and T_2 are weakly equivalent (represent identical relations under universal interpretations) if and only if phi is satisfiable. The "universal interpretation" condition means the tableaux must agree on all possible database instances, which constrains the undistinguished variables to encode a consistent truth assignment.

6. **Solution extraction:** From a proof of weak equivalence (specifically, from the containment mappings between T_1 and T_2), extract the truth assignment: if the containment mapping sends an undistinguished variable y_i representing x_i to a specific value, that determines the truth value of x_i.

**Key invariant:** The tableau T_1 encodes the clause structure of phi such that a containment mapping from T_2 to T_1 exists if and only if there is a consistent truth assignment satisfying all clauses. The weak equivalence of T_1 and T_2 is equivalent to the satisfiability of phi.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_variables` (number of Boolean variables in the 3SAT formula)
- m = `num_clauses` (number of clauses in the 3SAT formula)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_attributes` | `num_variables + num_clauses` |
| `num_rows_t1` | `3 * num_clauses` |
| `num_rows_t2` | `1` |
| `num_distinguished_vars` | `num_variables` |
| `num_undistinguished_vars` | `3 * num_clauses` |

**Derivation:**
- Attributes: one per variable plus one per clause, giving O(n + m) columns
- Tableau T_1: up to 3 rows per clause (one per literal that could satisfy it), giving O(m) rows
- Tableau T_2: a single summary row encoding the universal relation
- Distinguished variables: n (encoding the variable columns)
- Undistinguished variables: up to 3m (encoding the literal choices per clause)
- Total encoding size: O(n * m) (each row has n + m entries)

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a KSatisfiability(k=3) instance to TableauEquivalence, solve the tableau equivalence problem with BruteForce (enumerate all containment mappings between T_1 and T_2), extract the truth assignment, verify it satisfies the original 3SAT formula
- Check that a satisfiable 3SAT formula yields equivalent tableaux
- Check that an unsatisfiable 3SAT formula yields non-equivalent tableaux
- Test with a formula that has a unique satisfying assignment to verify precise solution extraction
- Verify that the "simple tableau" special case (which is polynomial-time solvable) does not arise from the reduction

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3, x_4, x_5, x_6
Clauses (7 clauses):
- C_1 = (x_1 ∨ x_2 ∨ not x_3)
- C_2 = (not x_1 ∨ x_3 ∨ x_4)
- C_3 = (x_2 ∨ not x_4 ∨ x_5)
- C_4 = (not x_2 ∨ x_3 ∨ not x_5)
- C_5 = (x_1 ∨ not x_3 ∨ x_6)
- C_6 = (not x_4 ∨ x_5 ∨ not x_6)
- C_7 = (x_3 ∨ x_4 ∨ x_6)

Satisfying assignment: x_1=T, x_2=T, x_3=T, x_4=F, x_5=T, x_6=T
- C_1: x_1=T
- C_2: x_3=T
- C_3: x_2=T
- C_4: x_3=T
- C_5: x_1=T
- C_6: x_5=T
- C_7: x_3=T

**Constructed target instance (TableauEquivalence):**
Attribute set A = {a_1, a_2, a_3, a_4, a_5, a_6, b_1, b_2, b_3, b_4, b_5, b_6, b_7} (13 attributes: 6 for variables, 7 for clauses)

Functional dependencies F: encode relationships between variable attributes and clause attributes based on literal appearances.

Tableau T_1: 7 rows (one per clause) x 13 columns, with entries encoding which literals appear in each clause. Undistinguished variables represent the choice of which literal satisfies each clause.

Tableau T_2: 1 row x 13 columns, a summary row with distinguished variables in the variable columns.

**Solution mapping:**
- The containment mapping from T_2 to T_1 encodes the satisfying assignment:
  - Maps the summary row of T_2 to rows of T_1 corresponding to satisfying literals
  - x_1=T, x_2=T, x_3=T, x_4=F, x_5=T, x_6=T
- The weak equivalence of T_1 and T_2 holds because the 3SAT formula is satisfiable
- The extracted truth assignment satisfies all 7 clauses


## References

- **[Aho, Sagiv, and Ullman, 1978]**: [`Aho1978`] A. V. Aho and Y. Sagiv and J. D. Ullman (1979). "Equivalences among relational expressions". *SIAM Journal on Computing*, 8(2), pp. 218-246.
- **[Sagiv and Yannakakis, 1978]**: [`Sagiv1978`] Y. Sagiv and M. Yannakakis (1978). "Equivalence among relational expressions with the union and difference operations". Dept. of Electrical Engineering and Computer Science, Princeton University.
