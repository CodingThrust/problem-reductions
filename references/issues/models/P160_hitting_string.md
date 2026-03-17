---
name: Problem
about: Propose a new problem type
title: "[Model] HittingString"
labels: model
assignees: ''
---

## Motivation

HITTING STRING (P160) from Garey & Johnson, A4 SR12. A classical NP-complete problem that provides a matrix/string-based reformulation of Boolean satisfiability. Each clause in a SAT formula maps naturally to a pattern string over {0,1,*}, and a satisfying assignment corresponds to a binary "hitting string" that agrees with every pattern on at least one non-wildcard position. This connection makes Hitting String a useful bridge between logic-based and combinatorial/string-based formulations of constraint satisfaction.

**Associated rules:**
- R106: 3SAT -> Hitting String (this model is the target)

## Definition

**Name:** `HittingString`
**Canonical name:** HITTING STRING
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR12, p.229

**Mathematical definition:**

INSTANCE: Finite set A of strings over {0,1,*}, all having the same length n.
QUESTION: Is there a string x in {0,1}^n such that for each string a in A there is some i, 1 <= i <= n, for which the i-th symbol of a and the i-th symbol of x are identical (i.e., a[i] != * and a[i] = x[i])?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n binary variables, one per position in the string x.
- **Per-variable domain:** {0, 1} -- each position of x is either 0 or 1.
- **Meaning:** The variable x[i] represents the i-th bit of the candidate hitting string. The string x = (x[1], ..., x[n]) must "hit" (agree on at least one non-* position with) every pattern string in A.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `HittingString`
**Variants:** none (no graph or weight parameters)

| Field | Type | Description |
|-------|------|-------------|
| `patterns` | `Vec<Vec<Option<bool>>>` | Set A of pattern strings; each pattern is a vector of length n where `Some(true)` = 1, `Some(false)` = 0, `None` = * |
| `string_length` | `usize` | Length n of all pattern strings (redundant with patterns[0].len() but explicit for clarity) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- Alternative encoding: patterns as `Vec<Vec<u8>>` with 0, 1, 2 representing '0', '1', '*' respectively.
- The problem is closely related to SAT: there is a bijection between 3SAT instances and Hitting String instances where each pattern has exactly 3 non-* positions (see R106).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(2^n) brute-force enumeration over all binary strings of length n, checking each against all m patterns in O(m * n) time. Total: O(m * n * 2^n). Since Hitting String is equivalent to SAT (see below), the best SAT algorithms apply: Schöning's randomized algorithm gives O*((4/3)^n) for the 3-pattern case, and PPSZ/Hertli's O*(1.308^n) for 3SAT translates to the 3-non-star case.
- **Equivalence to SAT:** Hitting String with patterns having at most k non-* positions is equivalent to k-SAT. Therefore, all complexity bounds for k-SAT transfer directly.
- **NP-completeness:** NP-complete [Fagin, 1974], via transformation from 3SAT.
- **References:**
  - R. Fagin (1974). "Generalized first-order spectra and polynomial-time recognizable sets." *Complexity of Computation*, SIAM-AMS Proceedings Vol. 7, pp. 43-73.

## Extra Remark

**Full book text:**

INSTANCE: Finite set A of strings over {0,1,*}, all having the same length n.
QUESTION: Is there a string x in {0,1}* with |x| = n such that for each string a in A there is some i, 1 <= i <= n, for which the ith symbol of a and the ith symbol of x are identical?
Reference: [Fagin, 1974]. Transformation from 3SAT.

**Connection to SAT:** The Hitting String problem is essentially SAT in disguise. Given a CNF formula with n variables and m clauses (each clause having at most k literals), map each clause to a pattern of length n: position i gets '1' if x_i appears positively, '0' if x_i appears negated, and '*' if x_i does not appear. A hitting string x corresponds to a satisfying truth assignment (x[i]=1 means x_i=true). The "hitting" condition (agreeing on at least one non-* position) is equivalent to satisfying at least one literal in the clause.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all 2^n binary strings and check each against all patterns.
- [x] It can be solved by reducing to integer programming -- encode as ILP: binary variable x_i for each position; for each pattern a_j, add constraint sum_{i: a_j[i]=1} x_i + sum_{i: a_j[i]=0} (1 - x_i) >= 1.
- [x] Other: Reduce to SAT (natural bijection), then use any SAT solver. Alternatively, reduce to Hitting Set (each pattern defines a "set" of binary strings that hit it, and we need a string in the intersection).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has hitting string):**
String length: n = 6
Pattern set A with 7 patterns:
- a_1 = [1, 1, 1, *, *, *]
- a_2 = [0, *, 1, 1, *, *]
- a_3 = [*, 1, 0, *, 1, *]
- a_4 = [*, 0, *, 1, *, 1]
- a_5 = [1, *, *, 0, 0, *]
- a_6 = [0, 0, *, *, *, 1]
- a_7 = [*, *, 1, *, 1, 0]

Candidate hitting string: x = [1, 0, 1, 1, 1, 1]

Verification:
- a_1 = [1,1,1,*,*,*]: x[1]=1=a_1[1] -- hit at position 1
- a_2 = [0,*,1,1,*,*]: x[3]=1=a_2[3] -- hit at position 3
- a_3 = [*,1,0,*,1,*]: x[5]=1=a_3[5] -- hit at position 5
- a_4 = [*,0,*,1,*,1]: x[4]=1=a_4[4] -- hit at position 4
- a_5 = [1,*,*,0,0,*]: x[1]=1=a_5[1] -- hit at position 1
- a_6 = [0,0,*,*,*,1]: x[2]=0=a_6[2] -- hit at position 2
- a_7 = [*,*,1,*,1,0]: x[3]=1=a_7[3] -- hit at position 3

Answer: YES, x = [1, 0, 1, 1, 1, 1] is a hitting string.

**Instance 2 (no hitting string):**
String length: n = 3
Pattern set A with 8 patterns (all possible 3-length patterns with no wildcards):
- a_1 = [0, 0, 0]
- a_2 = [0, 0, 1]
- a_3 = [0, 1, 0]
- a_4 = [0, 1, 1]
- a_5 = [1, 0, 0]
- a_6 = [1, 0, 1]
- a_7 = [1, 1, 0]
- a_8 = [1, 1, 1]

Wait -- any x in {0,1}^3 trivially hits all these patterns (since every pattern has no wildcards, x always agrees at every position). So this is always YES.

Revised Instance 2 (no hitting string):
String length: n = 2
Pattern set A:
- a_1 = [0, *]
- a_2 = [1, *]
- a_3 = [*, 0]
- a_4 = [*, 1]

Actually, any x in {0,1}^2 hits all of these. To make a NO instance:

String length: n = 1
Pattern set A:
- a_1 = [0]
- a_2 = [1]

Wait, x=0 hits a_1, and x=1 hits a_2. Both YES.

Actually, a NO instance requires contradictory constraints that cannot be simultaneously met. With patterns over {0,1,*}, any string of length n hits any single pattern that has at least one non-* position. A NO instance arises from SAT unsatisfiability. Consider this:

String length: n = 2
Pattern set A (corresponding to an unsatisfiable 2-CNF):
Clauses: (x_1) ^ (~x_1) ^ (x_2) ^ (~x_2)
- a_1 = [1, *] (from x_1)
- a_2 = [0, *] (from ~x_1)
- a_3 = [*, 1] (from x_2)
- a_4 = [*, 0] (from ~x_2)

x = [0,0]: hits a_2 (pos 1) and a_4 (pos 2), but a_1=[1,*]: x[1]=0 != 1, only non-* position fails. Not hit.
Wait: a_1 has only one non-* position (position 1 = 1). x[1]=0 != 1. So x=[0,0] does NOT hit a_1.
x = [1,0]: hits a_1 (pos 1, x[1]=1=1), hits a_4 (pos 2, x[2]=0=0), but a_2=[0,*]: x[1]=1 != 0. Not hit.

No x in {0,1}^2 can simultaneously hit a_1 and a_2 since position 1 must be both 0 and 1.
Answer: NO (no hitting string exists).
