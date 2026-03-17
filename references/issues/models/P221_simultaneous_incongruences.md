---
name: Problem
about: Propose a new problem type
title: "[Model] SimultaneousIncongruences"
labels: model
assignees: ''
---

## Motivation

SIMULTANEOUS INCONGRUENCES (P221) from Garey & Johnson, A7 AN2. An NP-complete number-theoretic problem: given a collection of forbidden residue classes {(a_i, b_i)}, determine whether there exists an integer x that avoids all of them (x is not congruent to a_i modulo b_i for any i). This is related to the concept of covering systems in number theory. The problem was shown NP-complete by Stockmeyer and Meyer (1973) via reduction from 3SAT. Despite the seeming simplicity of the question (find an integer outside a union of arithmetic progressions), the interaction of different moduli makes the problem intractable.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** (none known in GJ appendix)
- **As target:** R165 (3SAT -> SIMULTANEOUS INCONGRUENCES)

## Definition

**Name:** `SimultaneousIncongruences`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN2

**Mathematical definition:**

INSTANCE: Collection {(a_1,b_1), . . . , (a_n,b_n)} of ordered pairs of positive integers, with a_i <= b_i for 1 <= i <= n.
QUESTION: Is there an integer x such that, for 1 <= i <= n, x ≢ a_i (mod b_i)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 1 (the unknown integer x)
- **Per-variable domain:** {0, 1, ..., lcm(b_1, ..., b_n) - 1} -- by periodicity, it suffices to search one period of the combined modular system
- **Meaning:** x is an integer that must simultaneously avoid n specified residue classes. Each pair (a_i, b_i) defines a forbidden arithmetic progression: x must not be congruent to a_i modulo b_i.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SimultaneousIncongruences`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `pairs` | `Vec<(u64, u64)>` | Collection of (a_i, b_i) pairs; each defines a forbidden residue class x ≢ a_i (mod b_i). Constraint: a_i <= b_i. |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Brute-force search over x in {0, 1, ..., lcm(b_1, ..., b_n) - 1}, checking all n incongruence conditions for each candidate. Time complexity: O(lcm(b_1, ..., b_n) * n). This is pseudo-polynomial. The problem is NP-complete in general because the moduli b_i can be chosen so that their LCM is exponential in the input size. For fixed moduli (all b_i bounded by a constant), the problem is polynomial.

## Extra Remark

**Full book text:**

INSTANCE: Collection {(a_1,b_1), . . . , (a_n,b_n)} of ordered pairs of positive integers, with a_i <= b_i for 1 <= i <= n.
QUESTION: Is there an integer x such that, for 1 <= i <= n, x ≢ a_i (mod b_i)?

Reference: [Stockmeyer and Meyer, 1973]. Transformation from 3SAT.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate x from 0 to lcm(b_1, ..., b_n) - 1; check all incongruence conditions.)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: For small moduli or special structure, sieving methods can be applied.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
n = 4 pairs:
- (0, 2) -- x must not be even (x ≢ 0 mod 2)
- (1, 3) -- x ≢ 1 (mod 3)
- (2, 5) -- x ≢ 2 (mod 5)
- (3, 7) -- x ≢ 3 (mod 7)

**Question:** Is there an integer x avoiding all four residue classes?

**Solution search (checking small integers):**
- x = 1: 1 mod 2 = 1 (not 0, ok), 1 mod 3 = 1 (FAIL: equals a_2 = 1).
- x = 3: 3 mod 2 = 1 (ok), 3 mod 3 = 0 (not 1, ok), 3 mod 5 = 3 (not 2, ok), 3 mod 7 = 3 (FAIL: equals a_4 = 3).
- x = 5: 5 mod 2 = 1 (ok), 5 mod 3 = 2 (not 1, ok), 5 mod 5 = 0 (not 2, ok), 5 mod 7 = 5 (not 3, ok). ALL PASS!

**Answer:** YES, x = 5.

**Verification:**
- 5 mod 2 = 1 ≠ 0
- 5 mod 3 = 2 ≠ 1
- 5 mod 5 = 0 ≠ 2
- 5 mod 7 = 5 ≠ 3

**Negative example:**
Pairs: (0, 2), (1, 2). These cover all integers (every integer is either even or odd), so no x can avoid both. Answer: NO.
