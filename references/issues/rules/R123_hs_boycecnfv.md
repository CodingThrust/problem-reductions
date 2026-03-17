---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hitting Set to Boyce-Codd Normal Form Violation"
labels: rule
assignees: ''
canonical_source_name: 'HITTING SET'
canonical_target_name: 'BOYCE-CODD NORMAL FORM VIOLATION'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Hitting Set
**Target:** Boyce-Codd Normal Form Violation
**Motivation:** Establishes NP-completeness of BOYCE-CODD NORMAL FORM VIOLATION via polynomial-time reduction from HITTING SET. The reduction encodes the combinatorial structure of hitting a collection of subsets into the problem of finding a subset of attributes that violates the Boyce-Codd normal form condition with respect to a system of functional dependencies, linking classical set-cover-type problems to database schema design questions.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.233

## GJ Source Entry

> [SR29] BOYCE-CODD NORMAL FORM VIOLATION
> INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a subset A' ⊆ A.
> QUESTION: Does A' violate Boyce-Codd normal form for the relational system <A,F>, i.e., is there a subset X ⊆ A' and two attribute names y,z E A' - X such that (X,{y}) E F* and (X,{z}) ∉ F*, where F* is the closure of F?
> Reference: [Bernstein and Beeri, 1976], [Beeri and Bernstein, 1978]. Transformation from HITTING SET.
> Comment: Remains NP-complete even if A' is required to satisfy "third normal form," i.e., if X ⊆ A' is a key for the system <A',F> and if two names y,z E A'-X satisfy (X,{y}) E F* and (X,{z}) ∉ F*, then z is a prime attribute for <A',F>.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Hitting Set instance (S, C, K) where S is the universe, C = {c_1, ..., c_m} is a collection of subsets of S, and K is the budget, construct a BCNF Violation instance as follows:

1. **Attribute set construction:** Create an attribute set A that encodes the universe elements and the subsets in C. For each element s_i in S, create an attribute a_i. Additionally, create auxiliary attributes to encode the structure of C. Let |S| = n and |C| = m. The total attribute set A has O(n + m) attributes.

2. **Functional dependency construction:** Design a collection F of functional dependencies on A such that the closure F* encodes the membership relationships between elements and subsets. Specifically, for each subset c_j in C, introduce functional dependencies that relate the attributes corresponding to elements in c_j so that "hitting" c_j corresponds to a non-trivial FD holding over those attributes.

3. **Target subset construction:** Set A' to be the subset of A corresponding to the universe elements S. The BCNF condition on A' is violated if and only if there exists a subset X of A' and attributes y, z in A' - X such that X functionally determines y (via F*) but not z. This structure mirrors the hitting set condition: a "hit" of a subset c_j means selecting some element from c_j to include in the hitting set.

4. **Budget encoding:** The budget K is encoded by controlling the minimum number of elements needed to create a BCNF violation. The original hitting set has a solution of size <= K if and only if A' violates BCNF.

5. **Solution extraction:** Given a BCNF violation witness (X, y, z), extract the hitting set from the attributes in X (or from the specific violation structure). The correspondence ensures that the violation identifies exactly which elements from S are needed to "hit" all subsets in C.

**Key invariant:** The functional dependencies F are designed so that the closure F* encodes the subset-membership structure of C. A BCNF violation in A' occurs precisely when the underlying hitting set condition is satisfied.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `universe_size` (number of elements in S)
- m = `num_sets` (number of subsets in C)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_attributes` | `universe_size + num_sets` |
| `num_functional_deps` | `O(num_sets * max_subset_size)` |

**Derivation:**
- Attribute set: one attribute per universe element plus auxiliary attributes for encoding subset structure, giving O(n + m) attributes
- Functional dependencies: at most proportional to the total size of the collection C (sum of subset sizes)
- The target subset A' has at most n attributes (one per universe element)

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HittingSet instance to BoyceCoddNormalFormViolation, solve the BCNF violation problem with BruteForce (enumerate all subsets X of A' and check the FD closure condition), extract the hitting set, verify it is a valid hitting set on the original instance
- Check that the BCNF violation exists if and only if the hitting set instance is satisfiable with budget K
- Test with a non-trivial instance where greedy element selection fails
- Verify that the functional dependency closure is correctly computed

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HittingSet):**
Universe S = {s_0, s_1, s_2, s_3, s_4, s_5} (6 elements)
Collection C (4 subsets):
- c_0 = {s_0, s_1, s_2}
- c_1 = {s_1, s_3, s_4}
- c_2 = {s_2, s_4, s_5}
- c_3 = {s_0, s_3, s_5}
Budget K = 2

**Constructed target instance (BoyceCoddNormalFormViolation):**
Attribute set A = {a_0, a_1, a_2, a_3, a_4, a_5, b_0, b_1, b_2, b_3} where a_i corresponds to universe element s_i and b_j is an auxiliary attribute for subset c_j.

Functional dependencies F:
- For c_0: {a_0, a_1, a_2} -> {b_0}
- For c_1: {a_1, a_3, a_4} -> {b_1}
- For c_2: {a_2, a_4, a_5} -> {b_2}
- For c_3: {a_0, a_3, a_5} -> {b_3}
- Additional FDs encoding the hitting structure

Target subset A' = {a_0, a_1, a_2, a_3, a_4, a_5}

**Solution mapping:**
- Hitting set solution: S' = {s_1, s_5} (size 2 = K):
  - c_0 = {s_0, s_1, s_2}: s_1 in S' -- hit
  - c_1 = {s_1, s_3, s_4}: s_1 in S' -- hit
  - c_2 = {s_2, s_4, s_5}: s_5 in S' -- hit
  - c_3 = {s_0, s_3, s_5}: s_5 in S' -- hit
- The corresponding BCNF violation in A' identifies a subset X and attributes y, z such that the violation encodes the choice of {s_1, s_5} as the hitting set
- All 4 subsets are hit by S' = {s_1, s_5} with |S'| = 2 <= K


## References

- **[Bernstein and Beeri, 1976]**: [`Bernstein1976`] P. A. Bernstein and C. Beeri (1976). "An algorithmic approach to normalization of relational database schemas". University of Toronto.
- **[Beeri and Bernstein, 1978]**: [`Beeri1978`] C. Beeri and P. A. Bernstein (1978). "Computational problems related to the design of normal form relational schemes".
- **[Beeri and Bernstein, 1979]**: [`Beeri1979`] C. Beeri and P. A. Bernstein (1979). "Computational problems related to the design of normal form relational schemas". *ACM Transactions on Database Systems*, 4(1), pp. 30-59.
