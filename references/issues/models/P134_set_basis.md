---
name: Problem
about: Propose a new problem type
title: "[Model] SetBasis"
labels: model
assignees: ''
---

## Motivation

SET BASIS (P134) from Garey & Johnson, A3 SP7. An NP-complete problem in set/storage theory: given a collection C of subsets of a finite set S, find a minimum-size "basis" B of subsets such that every set in C can be expressed as a union of sets in B. This has applications in data compression, database schema design, and Boolean function minimization -- finding a compact representation of a family of sets. The problem is closely related to set cover but has a fundamentally different structure: instead of covering elements, we must reconstruct exact sets via unions.

**Associated reduction rules:**
- As target: R74 (VERTEX COVER -> SET BASIS)

## Definition

**Name:** <!-- ⚠️ Unverified --> `SetBasis`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Set Basis Problem; also: Minimum Test Set Basis, Minimum Set Basis
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP7

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |C|.
QUESTION: Is there a collection B of subsets of S with |B| = K such that, for each c ∈ C, there is a subcollection of B whose union is exactly c?

The optimization version asks: find the minimum K such that a basis B of size K exists.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 2^|S| possible basis sets (subsets of S); in practice, the search space can be restricted. A natural encoding uses binary variables for each potential basis element.
- **Per-variable domain:** Each basis element b ∈ B is a subset of S; in a fixed enumeration of candidate subsets, each variable is binary (include or exclude that subset from B).
- **Meaning:** The configuration selects K subsets of S to form the basis B. The assignment is valid if every set c ∈ C can be exactly reconstructed as a union of some subcollection of B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SetBasis`
**Variants:** none (pure set-theoretic problem)

| Field | Type | Description |
|-------|------|-------------|
| `universe_size` | `usize` | Size of the ground set S (= \|S\|) |
| `collection` | `Vec<Vec<usize>>` | The collection C of target subsets of S (each represented as sorted element indices) |
| `k` | `usize` | Maximum allowed basis size K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Key getter methods: `num_items()` (= |S|), `num_sets()` (= |C|), `basis_size()` (= K).
- Basis elements are arbitrary subsets of S (not necessarily members of C).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Stockmeyer, 1975; transformation from VERTEX COVER). Remains NP-complete when all c ∈ C have |c| ≤ 3, but is trivial when all |c| ≤ 2.
- **Best known exact algorithm:** No specialized exact exponential algorithm is known. General approach: enumerate subsets of candidate basis elements (at most 2^{2^|S|} in the worst case, but typically much smaller). The problem can be formulated as an ILP. For practical instances, constraint programming and SAT solvers are used.
- **Parameterized complexity:** The problem is W[2]-hard parameterized by K (basis size), analogous to dominating set.
- **References:**
  - L. J. Stockmeyer (1975). "The Set Basis Problem is NP-Complete." IBM Research Report RC 5431, IBM Research Center, Yorktown Heights, NY.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** General set representation / compression problems
- **Known special cases:**
  - All sets in C have size ≤ 2: trivially solvable (each element is its own basis element)
  - All sets in C have size ≤ 3: still NP-complete
- **Related problems:** SET COVER (basis elements must cover S, not reconstruct C), EXACT COVER (disjoint union), MINIMUM EQUIVALENT EXPRESSION

## Extra Remark

**Full book text:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |C|.
QUESTION: Is there a collection B of subsets of S with |B| = K such that, for each c ∈ C, there is a subcollection of B whose union is exactly c?
Reference: [Stockmeyer, 1975]. Transformation from VERTEX COVER.
Comment: Remains NP-complete if all c ∈ C have |c| ≤ 3, but is trivial if all c ∈ C have |c| ≤ 2.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible collections of K subsets of S (there are C(2^|S|, K) such collections); for each collection B, check if every c ∈ C can be expressed as a union of a subcollection of B. Exponential but complete.
- [x] It can be solved by reducing to integer programming. Binary variable x_T for each candidate subset T ⊆ S; minimize/constrain sum(x_T) = K; for each c ∈ C, add constraints ensuring that c is expressible as the union of selected basis sets. This requires auxiliary variables for the subset-union reconstruction.
- [ ] Other: SAT encoding; constraint programming.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Ground set S = {a, b, c, d, e} (|S| = 5), Collection C of 4 subsets, K = 3:**
- c_1 = {a, b, c}
- c_2 = {a, b, d}
- c_3 = {c, d, e}
- c_4 = {a, b, c, d}

**Question:** Is there a basis B of 3 subsets of S such that each c_i is a union of sets in B?

**Solution:** B = { {a, b}, {c}, {d, e} } with |B| = 3.
- c_1 = {a,b,c} = {a,b} ∪ {c} ✓
- c_2 = {a,b,d} -- cannot form {a,b,d} from these basis sets since {d,e} includes e. So this B fails.

**Revised basis:** B = { {a, b}, {c}, {d} } with |B| = 3.
- c_1 = {a,b,c} = {a,b} ∪ {c} ✓
- c_2 = {a,b,d} = {a,b} ∪ {d} ✓
- c_3 = {c,d,e} -- cannot form since {e} is not in any basis element. This B fails too.

**Revised example with S = {a, b, c, d}, C with 4 subsets, K = 3:**
- c_1 = {a, b}
- c_2 = {b, c}
- c_3 = {a, c}
- c_4 = {a, b, c}

**Basis B = { {a}, {b}, {c} } with |B| = 3.**
- c_1 = {a,b} = {a} ∪ {b} ✓
- c_2 = {b,c} = {b} ∪ {c} ✓
- c_3 = {a,c} = {a} ∪ {c} ✓
- c_4 = {a,b,c} = {a} ∪ {b} ∪ {c} ✓

Can we do it with K = 2? Suppose B = {b_1, b_2}. Then c_1 = {a,b} must equal b_1, b_2, or b_1 ∪ b_2. Similarly c_2 = {b,c} and c_3 = {a,c}. We need three distinct 2-element sets all representable as unions of 2 basis sets. If b_1 ∪ b_2 = {a,b,c} (the only superset that contains all elements), then {a,b}, {b,c}, and {a,c} must each be either b_1, b_2, or b_1 ∪ b_2. But b_1 ∪ b_2 = {a,b,c} ≠ any of the three 2-element sets. So b_1 and b_2 must each equal one of {a,b}, {b,c}, {a,c} -- but we can only pick 2 of 3, and the third set cannot be formed. So K = 2 is infeasible.

**Answer:** K = 3 → YES, K = 2 → NO. Minimum basis size = 3.
