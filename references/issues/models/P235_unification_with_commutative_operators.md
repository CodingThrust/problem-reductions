---
name: Problem
about: Propose a new problem type
title: "[Model] UnificationWithCommutativeOperators"
labels: model
assignees: ''
---

## Motivation

UNIFICATION WITH COMMUTATIVE OPERATORS (P235) from Garey & Johnson, A7 AN16. Given pairs of expressions built from variables, constants, and a commutative binary operator "+", the problem asks whether there exists a substitution of variable-free expressions for variables that makes each pair equivalent under commutativity (where (a+b) ≡ (c+d) if {a,c} ≡ {b,d} as unordered pairs). NP-complete by reduction from 3SAT (Sethi, 1977). This is a sharp contrast with standard (non-commutative) unification, which is solvable in linear time by the Paterson-Wegman algorithm (1976). The result remains NP-complete even when each expression contains at most 7 symbols. Foundational for understanding the complexity of equational matching in term rewriting systems and automated theorem proving.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R179 (3SAT to UNIFICATION WITH COMMUTATIVE OPERATORS)
- As source: (none known)

## Definition

**Name:** `UnificationWithCommutativeOperators`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Unification with Commutative Operators
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN16

**Mathematical definition:**

INSTANCE: Set V of variables, set C of constants, ordered pairs (e_i, f_i), 1 ≤ i ≤ n, of "expressions," where an expression is either a variable from V, a constant from C, or (e + f) where e and f are expressions.
QUESTION: Is there an assignment to each v ∈ V of a variable-free expression I(v) such that, if I(e) denotes the expression obtained by replacing each occurrence of each variable v in e by I(v), then I(e_i) ≡ I(f_i) for 1 ≤ i ≤ n, where e ≡ f if e = f or if e = (a+b), f = (c+d), and either a ≡ c and b ≡ d or a ≡ d and b ≡ c?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** |V| (one variable per unification variable)
- **Per-variable domain:** The set of all variable-free (ground) expressions over C and "+". In practice, the relevant ground expressions can be bounded to polynomial size.
- **Meaning:** I(v) is the ground term substituted for variable v. The problem is feasible if and only if there exists a substitution I such that all equation pairs (e_i, f_i) are satisfied under the commutative equivalence relation ≡.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `UnificationWithCommutativeOperators`
**Variants:** none

| Field            | Type                     | Description                                                |
|------------------|--------------------------|------------------------------------------------------------|
| `variables`      | `Vec<String>`            | Set V of variable names                                    |
| `constants`      | `Vec<String>`            | Set C of constant symbols                                  |
| `equation_pairs` | `Vec<(Expr, Expr)>`      | Ordered pairs (e_i, f_i) of expressions to be unified      |

Where `Expr` is a recursive type: either a variable name, a constant, or a binary application of the commutative operator "+" to two sub-expressions.

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** NP-complete (Sethi, 1977). Brute-force: enumerate all possible substitutions of ground terms (bounded by expression size) for each variable, and check commutative equivalence of each pair. The non-commutative variant is solvable in O(n) time by the Paterson-Wegman algorithm (1976). For the commutative case, the NP-completeness holds even with at most 7 symbol occurrences per expression. The associative-commutative variant (where "+" is also associative) is also NP-complete. [Sethi, 1977; Paterson & Wegman, JCSS 16:158-167, 1976; Benanav, Kapur & Narendran, JAR 3:223-243, 1987.]

## Extra Remark

**Full book text:**

INSTANCE: Set V of variables, set C of constants, ordered pairs (e_i, f_i), 1 ≤ i ≤ n, of "expressions," where an expression is either a variable from V, a constant from C, or (e + f) where e and f are expressions.
QUESTION: Is there an assignment to each v ∈ V of a variable-free expression I(v) such that, if I(e) denotes the expression obtained by replacing each occurrence of each variable v in e by I(v), then I(e_i) ≡ I(f_i) for 1 ≤ i ≤ n, where e ≡ f if e = f or if e = (a+b), f = (c+d), and either a ≡ c and b ≡ d or a ≡ d and b ≡ c?

Reference: [Sethi, 1977b]. Transformation from 3SAT.
Comment: Remains NP-complete even if no e_j or f_i contains more than 7 occurrences of constants and variables. The variant in which the operator is non-commutative (and hence e ≡ f only if e = f) is solvable in polynomial time [Paterson and Wegman, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all substitutions of ground terms for variables; check commutative equivalence of each pair. The search space is finite because ground terms can be bounded in size.)
- [ ] It can be solved by reducing to integer programming. (Not directly natural, as the problem involves structural tree matching rather than numeric constraints.)
- [ ] Other: Specialized backtracking algorithms for commutative unification exist in automated reasoning systems, but none achieve polynomial worst-case time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Variables: V = {x, y}
Constants: C = {a, b, c}
Equation pairs:
1. (x + a, b + y) -- asking: does I(x) + a ≡ b + I(y)?

**Analysis:**
Under commutativity, (x + a) ≡ (b + y) requires either:
- I(x) ≡ b and a ≡ I(y), giving I(x) = b, I(y) = a, or
- I(x) ≡ I(y) and a ≡ b, which fails since a ≠ b.

So the unique unifier is I(x) = b, I(y) = a.

Verify: I(x + a) = (b + a), I(b + y) = (b + a). By commutativity, (b + a) ≡ (b + a) ✓.

Answer: **YES** -- a unifying substitution exists.

**A more complex example:**
Variables: V = {x}
Constants: C = {a, b}
Equation pairs:
1. (x + a, a + b) -- requires I(x) = b (direct match) or I(x) = a and a = b (fails). So I(x) = b.
2. (x + b, a + a) -- requires I(x) + b ≡ a + a. With I(x) = b: (b + b) ≡ (a + a)? Only if b = a, contradiction. With I(x) = a: (a + b) ≡ (a + a)? Requires b ≡ a, contradiction.

Answer: **NO** -- no unifying substitution exists (equations 1 and 2 are incompatible).
