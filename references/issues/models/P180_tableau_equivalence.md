---
name: Problem
about: Propose a new problem type
title: "[Model] TableauEquivalence"
labels: model
assignees: ''
---

## Motivation

TABLEAU EQUIVALENCE (P180) from Garey & Johnson, A4 SR32. A classical NP-complete problem in relational database theory. Tableaux are matrix representations of relational expressions (compositions of select, project, and join operations), and testing their equivalence is fundamental to query optimization. The problem was shown to be NP-complete by Aho, Sagiv, and Ullman (1979), though polynomial-time algorithms exist for "simple" tableaux.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **R126**: 3SAT -> Tableau Equivalence (this problem is the target)

## Definition

**Name:** <!-- ⚠️ Unverified --> `TableauEquivalence`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Tableau Equivalence (also: Relational Expression Equivalence, Weak Tableau Equivalence)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR32

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of ordered pairs of subsets of A, a set X of distinguished variables, a set Y of undistinguished variables, a set C_a of constants for each a in A, and two "tableaux" T_1 and T_2 over X, Y, and the C_a. (A tableau is essentially a matrix with a column for each attribute and entries from X, Y, the C_a, along with a blank symbol.)
QUESTION: Are T_1 and T_2 "weakly equivalent," i.e., do they represent identical relations under "universal interpretations"?

The problem is a decision (satisfaction) problem: the answer is a Boolean indicating whether the two tableaux are weakly equivalent.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** Tableau containment (T_1 ⊆ T_2) requires finding a mapping from the rows/variables of T_2 to those of T_1. The number of variables is determined by the number of undistinguished variables in the larger tableau.
- **Per-variable domain:** |X ∪ Y ∪ D ∪ {blank}| where D = union of all C_a. Each undistinguished variable in the mapping can be sent to any distinguished variable, undistinguished variable, constant, or blank.
- **Meaning:** A containment mapping h from T_2 to T_1 maps each row of T_2 to a row of T_1 such that: (1) distinguished variables are preserved, (2) constants are preserved, (3) functional dependencies in F are respected. T_1 and T_2 are weakly equivalent if both T_1 ⊆ T_2 and T_2 ⊆ T_1 (mutual containment under universal interpretations).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `TableauEquivalence`
**Variants:** none (the tableaux structure is stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `attributes` | `Vec<String>` | Set A of attribute names (columns of the tableau) |
| `functional_deps` | `Vec<(Vec<usize>, Vec<usize>)>` | Collection F of ordered pairs of attribute subsets |
| `distinguished_vars` | `Vec<String>` | Set X of distinguished variable names |
| `undistinguished_vars` | `Vec<String>` | Set Y of undistinguished variable names |
| `constants` | `Vec<Vec<usize>>` | Constants C_a for each attribute a |
| `tableau1` | `Vec<Vec<Option<String>>>` | Tableau T_1: matrix with rows x columns, entries are variable names, constants, or None (blank) |
| `tableau2` | `Vec<Vec<Option<String>>>` | Tableau T_2: same format as T_1 |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Aho, Sagiv, and Ullman, 1979; transformation from 3SAT).
- **Best known exact algorithm:** Brute-force: check both containment directions. For T_1 ⊆ T_2 (containment), enumerate all mappings from the rows/variables of T_2 to T_1 and check if each is a valid containment mapping. For a tableau with r rows and c columns, the brute-force runs in O(r_1^{r_2} * r_2 * c) time for containment T_1 ⊆ T_2. Weak equivalence requires checking both directions. For "simple" tableaux (those with no repeated undistinguished variables in any column), polynomial-time algorithms exist with O(r^2 * c) complexity.
- **Parameterized:** Tableau containment is NP-complete even for restricted cases (simple tableaux, per Sagiv and Yannakakis, 1978). When tableaux come from expressions without select operations, the problem remains NP-complete. Polynomial-time solvability holds for the subclass of "simple" tableaux under equivalence (not containment).
- **References:**
  - [Aho, Sagiv, and Ullman, 1979] A. V. Aho, Y. Sagiv, and J. D. Ullman, "Equivalences Among Relational Expressions", *SIAM J. Computing*, 8(2), pp. 218-246, 1979.
  - [Sagiv and Yannakakis, 1978] Y. Sagiv and M. Yannakakis, "Equivalence among relational expressions with the union and difference operations", Princeton University, 1978.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **Related problems:** Conjunctive Query Containment (closely related -- tableaux represent conjunctive queries), Conjunctive Query Foldability, Relational Expression Optimization.
- **Special cases:** "Simple" tableaux (no repeated undistinguished variables per column) admit polynomial-time equivalence testing. Tableaux from expressions without select operations are still NP-complete for equivalence.
- **Strong vs. Weak equivalence:** The same NP-completeness results hold for "strong equivalence" (identical relations under all interpretations, not just universal ones).

## Extra Remark

**Full book text:**

INSTANCE: A set A of attribute names, a collection F of ordered pairs of subsets of A, a set X of distinguished variables, a set Y of undistinguished variables, a set Ca of constants for each a in A, and two "tableaux" T1 and T2 over X, Y, and the Ca. (A tableau is essentially a matrix with a column for each attribute and entries from X, Y, the Ca, along with a blank symbol. For details and an interpretation in terms of relational expressions, see reference.)
QUESTION: Are T1 and T2 "weakly equivalent," i.e., do they represent identical relations under "universal interpretations"?
Reference: [Aho, Sagiv, and Ullman, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete even if the tableaux come from "expressions" that have no "select" operations, or if the tableaux come from expressions that have select operations but F is empty, or if F is empty, the tableaux contain no constants, and the tableaux do not necessarily come from expressions at all. Problem is solvable in polynomial time for "simple" tableaux. The same results hold also for "strong equivalence," where the two tableaux must represent identical relations under all interpretations. The problem of tableau "containment," however, is NP-complete even for simple tableaux and for still further restricted tableaux [Sagiv and Yannakakis, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all containment mappings from T_2 to T_1 and from T_1 to T_2; check if both containment directions hold. A mapping is valid if it preserves distinguished variables, constants, and respects functional dependencies.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (none identified)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Attributes A = {A, B, C, D, E, F} (6 attributes, indexed 0..5)

Functional dependencies F:
- FD1: {A, B} -> {C} (attributes 0,1 determine attribute 2)

Distinguished variables X = {x_A, x_B, x_C, x_D, x_E, x_F} (one per attribute)
Undistinguished variables Y = {y_1, y_2, y_3, y_4, y_5, y_6}
Constants: C_A = {}, C_B = {}, C_C = {}, C_D = {}, C_E = {}, C_F = {} (no constants)

Tableau T_1 (3 rows x 6 columns):
| A   | B   | C   | D   | E   | F   |
|-----|-----|-----|-----|-----|-----|
| x_A | x_B | y_1 | x_D | x_E | y_2 |
| x_A | y_3 | x_C | y_4 | x_E | x_F |
| y_5 | x_B | x_C | x_D | y_6 | x_F |

Tableau T_2 (2 rows x 6 columns):
| A   | B   | C   | D   | E   | F   |
|-----|-----|-----|-----|-----|-----|
| x_A | x_B | x_C | x_D | x_E | x_F |
| x_A | x_B | x_C | x_D | x_E | x_F |

(T_2 has a single distinct summary row.)

**Analysis:**
T_2 represents the "identity" relation (just the tuple of distinguished variables). T_1 represents a join of three projections. The question is whether T_1 and T_2 are weakly equivalent.

For T_2 ⊆ T_1: We need a containment mapping h from T_1 to T_2. This requires sending each row of T_1 to the single row of T_2, mapping all undistinguished variables to their corresponding distinguished variables. This works if the substitution y_1 -> x_C, y_2 -> x_F, y_3 -> x_B, y_4 -> x_D, y_5 -> x_A, y_6 -> x_E is consistent, and the FD {A,B} -> {C} is respected. Since x_A, x_B are preserved and y_1 maps to x_C (consistent with FD1), this mapping is valid.

For T_1 ⊆ T_2: T_2's single row can map to any row of T_1, substituting distinguished variables. Row 1 of T_2 maps to row 1 of T_1 with x_C -> y_1, x_F -> y_2; but this changes distinguished variables, which is not allowed. So we need a different approach: the containment mapping must preserve distinguished variables. This requires checking if T_1's relation is contained in T_2's relation under universal interpretations.

The equivalence depends on whether the FD {A,B} -> {C} forces the three rows of T_1 to collapse to a single row matching T_2. This is a non-trivial verification that demonstrates the complexity of the problem.
