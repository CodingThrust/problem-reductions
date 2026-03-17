---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Consistency of Database Frequency Tables"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'CONSISTENCY OF DATABASE FREQUENCY TABLES'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Consistency of Database Frequency Tables
**Motivation:** Establishes NP-completeness of Consistency of Database Frequency Tables via polynomial-time reduction from 3SAT. This result has practical implications for statistical database security: it shows that no polynomial-time algorithm can determine whether a set of published frequency tables can be used to "compromise" a database by deducing specific attribute values of individual records, unless P = NP. The reduction encodes Boolean variables as attribute values and clauses as frequency table constraints, so that satisfying all clauses corresponds to finding an assignment of attribute values consistent with the given frequency tables.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.235

## GJ Source Entry

> [SR35] CONSISTENCY OF DATABASE FREQUENCY TABLES
> INSTANCE: Set A of attribute names, domain set D_a for each a E A, set V of objects, collection F of frequency tables for some pairs a,b E A (where a frequency table for a,b E A is a function f_{a,b}: D_a × D_b → Z+ with the sum, over all pairs x E D_a and y E D_b, of f_{a,b}(x,y) equal to |V|), and a set K of triples (v,a,x) with v E V, a E A, and x E D_a, representing the known attribute values.
> QUESTION: Are the frequency tables in F consistent with the known attribute values in K, i.e., is there a collection of functions g_a: V → D_a, for each a E A, such that g_a(v) = x if (v,a,x) E K and such that, for each f_{a,b} E F, x E D_a, and y E D_b, the number of v E V for which g_a(v) = x and g_b(v) = y is exactly f_{a,b}(x,y)?
> Reference: [Reiss, 1977b]. Transformation from 3SAT.
> Comment: Above result implies that no polynomial time algorithm can be given for "compromising" a data base from its frequency tables by deducing prespecified attribute values, unless P = NP (see reference for details).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with variables x_1, ..., x_n and clauses C_1, ..., C_m (each clause having exactly 3 literals), construct a Consistency of Database Frequency Tables instance as follows:

1. **Object construction:** Create one object v_i for each variable x_i in the 3SAT formula. Thus |V| = n (the number of variables).

2. **Attribute construction for variables:** Create one attribute a_i for each variable x_i, with domain D_{a_i} = {T, F} (representing True and False). The assignment g_{a_i}(v_i) encodes the truth value of variable x_i.

3. **Attribute construction for clauses:** For each clause C_j = (l_{j1} ∨ l_{j2} ∨ l_{j3}), create an attribute b_j with domain D_{b_j} = {1, 2, 3, ..., 7} representing which of the 7 satisfying truth assignments for the 3 literals in C_j is realized. (There are 2^3 - 1 = 7 ways to satisfy a 3-literal clause.)

4. **Frequency table construction:** For each clause C_j involving variables x_{p}, x_{q}, x_{r}:
   - Create frequency tables f_{a_p, b_j}, f_{a_q, b_j}, and f_{a_r, b_j} that encode the relationship between the truth value of each variable and the satisfying assignment chosen for clause C_j.
   - The frequency table f_{a_p, b_j}(T, k) = 1 if the k-th satisfying assignment of C_j has x_p = True, and 0 otherwise (similarly for F). These tables enforce that the attribute value of object v_p (the truth value of x_p) is consistent with the satisfying assignment chosen for clause C_j.

5. **Known attribute values (K):** The set K is initially empty (no attribute values are pre-specified), or may contain specific triples to encode unit propagation constraints.

6. **Marginal consistency constraints:** Additional frequency tables between variable-attributes a_p and a_q for variables appearing together in clauses enforce that each object v_i has a unique, globally consistent truth value.

7. **Solution extraction:** The frequency tables in F are consistent with K if and only if there exists an assignment of truth values to x_1, ..., x_n that satisfies all clauses. A consistent set of functions g_a corresponds directly to a satisfying assignment.

**Key invariant:** Each object represents a Boolean variable, each variable-attribute encodes {T, F}, and the frequency tables between variable-attributes and clause-attributes ensure that every clause has at least one true literal — which is exactly the 3SAT satisfiability condition.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in the 3SAT instance
- m = number of clauses in the 3SAT instance

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_objects` | `num_variables` |
| `num_attributes` | `num_variables + num_clauses` |
| `num_frequency_tables` | `3 * num_clauses` |

**Derivation:**
- Objects: one per Boolean variable -> |V| = n
- Attributes: one per variable (domain {T, F}) plus one per clause (domain {1,...,7}) -> |A| = n + m
- Frequency tables: 3 tables per clause (one for each literal's variable paired with the clause attribute) -> |F| = 3m
- Domain sizes: variable attributes have |D| = 2; clause attributes have |D| <= 7
- Known values: |K| = O(n) at most (possibly empty)

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a 3SAT instance to a Consistency of Database Frequency Tables instance, solve the consistency problem by brute-force enumeration of all possible attribute-value assignments, extract the truth assignment, and verify it satisfies all original clauses
- Check that the number of objects, attributes, and frequency tables matches the overhead formula
- Test with a 3SAT instance that is satisfiable and verify that at least one consistent assignment exists
- Test with an unsatisfiable 3SAT instance and verify that no consistent assignment exists
- Verify that frequency table marginals sum to |V| as required by the problem definition

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3, x_4, x_5, x_6
Clauses (7 clauses):
- C_1 = (x_1 ∨ x_2 ∨ x_3)
- C_2 = (¬x_1 ∨ x_4 ∨ x_5)
- C_3 = (¬x_2 ∨ ¬x_3 ∨ x_6)
- C_4 = (x_1 ∨ ¬x_4 ∨ ¬x_6)
- C_5 = (¬x_1 ∨ x_3 ∨ ¬x_5)
- C_6 = (x_2 ∨ ¬x_5 ∨ x_6)
- C_7 = (¬x_3 ∨ x_4 ∨ ¬x_6)

Satisfying assignment: x_1=T, x_2=T, x_3=F, x_4=T, x_5=F, x_6=T
- C_1: x_1=T ✓
- C_2: ¬x_1=F, x_4=T ✓
- C_3: ¬x_2=F, ¬x_3=T ✓
- C_4: x_1=T ✓
- C_5: ¬x_1=F, x_3=F, ¬x_5=T ✓
- C_6: x_2=T ✓
- C_7: ¬x_3=T ✓

**Constructed target instance (Consistency of Database Frequency Tables):**
Objects V = {v_1, v_2, v_3, v_4, v_5, v_6} (6 objects, one per variable)
Attributes A:
- Variable attributes: a_1, a_2, a_3, a_4, a_5, a_6 (domain {T, F} each)
- Clause attributes: b_1, b_2, b_3, b_4, b_5, b_6, b_7 (domain {1,...,7} each)

Total: 13 attributes

Frequency tables F (21 tables, 3 per clause):
- For C_1 = (x_1 ∨ x_2 ∨ x_3): tables f_{a_1,b_1}, f_{a_2,b_1}, f_{a_3,b_1}
- For C_2 = (¬x_1 ∨ x_4 ∨ x_5): tables f_{a_1,b_2}, f_{a_4,b_2}, f_{a_5,b_2}
- (... similarly for C_3 through C_7 ...)

Example frequency table f_{a_1, b_1} (for variable x_1 in clause C_1 = (x_1 ∨ x_2 ∨ x_3)):
The 7 satisfying assignments of (x_1 ∨ x_2 ∨ x_3) are:
1: (T,T,T), 2: (T,T,F), 3: (T,F,T), 4: (T,F,F), 5: (F,T,T), 6: (F,T,F), 7: (F,F,T)

| a_1 \ b_1 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|-----------|---|---|---|---|---|---|---|
| T         | * | * | * | * | 0 | 0 | 0 |
| F         | 0 | 0 | 0 | 0 | * | * | * |

(Entries marked * are determined by the assignment; each column sums to the number of objects that realize that satisfying pattern.)

Known values K = {} (empty)

**Solution mapping:**
- The satisfying assignment x_1=T, x_2=T, x_3=F, x_4=T, x_5=F, x_6=T corresponds to:
  - g_{a_1}(v_1) = T, g_{a_2}(v_2) = T, g_{a_3}(v_3) = F, g_{a_4}(v_4) = T, g_{a_5}(v_5) = F, g_{a_6}(v_6) = T
- For clause C_1 = (x_1 ∨ x_2 ∨ x_3) with assignment (T, T, F): this matches satisfying pattern #2 (T,T,F)
- The frequency tables are consistent with these attribute functions ✓
- All frequency table marginals sum to |V| = 6 ✓


## References

- **[Reiss, 1977b]**: [`Reiss1977b`] S. P. Reiss (1977). "Statistical database confidentiality". Dept. of Statistics, University of Stockholm.
