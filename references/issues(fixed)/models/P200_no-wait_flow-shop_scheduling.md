---
name: Problem
about: Propose a new problem type
title: "[Model] NoWaitFlowShopScheduling"
labels: model
assignees: ''
---

## Motivation

NO-WAIT FLOW-SHOP SCHEDULING (P200) from Garey & Johnson, A5 SS16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS16

**Mathematical definition:**

INSTANCE: (Same as for FLOW-SHOP SCHEDULING).
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and has the property that, for each j ∈ J and 1 ≤ i < m, σi+1(j) = σi(j) + l(ti[j])?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: (Same as for FLOW-SHOP SCHEDULING).
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and has the property that, for each j ∈ J and 1 ≤ i < m, σi+1(j) = σi(j) + l(ti[j])?

Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from DIRECTED HAMILTONIAN PATH.

Comment: NP-complete in the strong sense for any fixed m ≥ 4 [Papadimitriou and Kanellakis, 1978]. Solvable in polynomial time for m = 2 [Gilmore and Gomory, 1964]. (However, NP-complete in the strong sense for m = 2 if jobs with no tasks on the first processor are allowed [Sahni and Cho, 1977b].) Open for fixed m = 3. If the goal is to meet a bound K on the sum, over all j ∈ J, of σm(j) + l(tm[j]), then the problem is NP-complete in the strong sense for m arbitrary [Lenstra, Rinnooy Kan, and Brucker, 1977] and open for fixed m ≥ 2. The analogous "no-wait" versions of OPEN-SHOP SCHEDULING and JOB-SHOP SCHEDULING are NP-complete in the strong sense for m = 2 [Sahni and Cho, 1977b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
