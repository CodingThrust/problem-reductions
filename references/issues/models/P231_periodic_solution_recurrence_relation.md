---
name: Problem
about: Propose a new problem type
title: "[Model] PeriodicSolutionRecurrenceRelation"
labels: model
assignees: ''
---

## Motivation

PERIODIC SOLUTION RECURRENCE RELATION (P231) from Garey & Johnson, A7 AN12. An NP-hard problem (not known to be in NP or co-NP) about whether a linear recurrence relation, specified by coefficient-lag pairs, admits an integer-valued periodic solution. The problem is intimately connected to Root of Modulus 1 (P229): a linear recurrence has periodic solutions if and only if its characteristic polynomial has a root on the complex unit circle. Plaisted (1977) established NP-hardness via reduction from 3SAT.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R175: 3SAT -> PERIODIC SOLUTION RECURRENCE RELATION (establishes NP-hardness via connection to characteristic polynomial roots on the unit circle)

## Definition

**Name:** `PeriodicSolutionRecurrenceRelation`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN12

**Mathematical definition:**

INSTANCE: Ordered pairs (c_i, b_i), 1 <= i <= m, of integers, with all b_i positive.
QUESTION: Is there a sequence a_0, a_1, . . . , a_{n-1} of integers, with n >= max{b_i}, such that the infinite sequence a_0, a_1, . . . defined by the recurrence relation

    a_i = Sigma_{j=1}^{m} c_j * a_{(i-b_j)}

satisfies a_i = a_{i (mod n)}, for all i >= n?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n (the period length, which is itself part of the solution -- not fixed by the input). The initial conditions a_0, a_1, ..., a_{n-1} are the n integer values to be determined.
- **Per-variable domain:** Z (integers, unbounded). In practice, for a computational model, one could bound the magnitude of the initial values.
- **Meaning:** a_k for k = 0, ..., n-1 are the initial values of the periodic sequence. The recurrence then extends the sequence, and periodicity requires a_i = a_{i mod n} for all i >= n.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `PeriodicSolutionRecurrenceRelation`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `terms` | `Vec<(i64, usize)>` | Ordered pairs (c_i, b_i) defining the recurrence coefficients c_i and lags b_i (all b_i > 0) |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** No polynomial-time algorithm is known. The problem is NP-hard but not known to be in NP or co-NP (Plaisted, 1977). The key difficulty is that both the period n and the initial values a_0, ..., a_{n-1} are unknowns. By the theory of linear recurrences, periodic solutions exist iff the characteristic polynomial P(z) = z^B - Sigma_{j=1}^{m} c_j * z^{B - b_j} (where B = max{b_j}) has a root on the complex unit circle. This reduces to the Root of Modulus 1 problem, which is itself NP-hard. The related Skolem problem (does a linear recurrence sequence have a zero?) is decidable for order <= 4 (Mignotte-Shorey-Tijdeman) but open for general order. No sub-exponential algorithm is known for detecting periodicity of sparse recurrences in general.

## Extra Remark

**Full book text:**

INSTANCE: Ordered pairs (c_i, b_i), 1 <= i <= m, of integers, with all b_i positive.
QUESTION: Is there a sequence a_0, a_1, . . . , a_{n-1} of integers, with n >= max{b_i}, such that the infinite sequence a_0, a_1, . . . defined by the recurrence relation

    a_i = Sigma_{j=1}^{m} c_j * a_{(i-b_j)}

satisfies a_i = a_{i(mod n)}, for all i >= n?

Reference: [Plaisted, 1977b]. Tranformation from 3SAT
Comment: Not known to be in NP or co-NP. See reference for related results.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Reduce to Root of Modulus 1 by forming the characteristic polynomial z^B - Sigma c_j * z^{B-b_j} and checking for roots on the unit circle. For small lags, this can be done via numerical root-finding. For the general sparse case, no polynomial-time algorithm is known.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Terms: [(1, 1), (-1, 2)]
This defines the recurrence: a_i = 1 * a_{i-1} + (-1) * a_{i-2} = a_{i-1} - a_{i-2}.

**Characteristic polynomial:**
P(z) = z^2 - z + 1.
Roots: z = (1 +/- sqrt(1-4)) / 2 = (1 +/- i*sqrt(3)) / 2 = e^{+/- i*pi/3}.
Both roots have |z| = 1 (they lie on the unit circle), so periodic solutions exist.

**Periodic solution:**
Starting with a_0 = 1, a_1 = 1:
a_2 = a_1 - a_0 = 0
a_3 = a_2 - a_1 = -1
a_4 = a_3 - a_2 = -1
a_5 = a_4 - a_3 = 0
a_6 = a_5 - a_4 = 1
a_7 = a_6 - a_5 = 1

The sequence is 1, 1, 0, -1, -1, 0, 1, 1, 0, -1, -1, 0, ...
This is periodic with period n = 6: a_i = a_{i mod 6} for all i >= 6.

Answer: YES -- the recurrence has a periodic solution with period 6.

**Negative example:**
Terms: [(2, 1)]
Recurrence: a_i = 2 * a_{i-1}.
Characteristic polynomial: z - 2. Root: z = 2, |z| = 2 != 1.
No root on the unit circle, so no periodic solution exists (the sequence grows exponentially: a_0, 2*a_0, 4*a_0, ...).
Answer: NO (unless a_0 = 0, which gives the trivial all-zeros sequence; GJ likely requires a nontrivial solution).
