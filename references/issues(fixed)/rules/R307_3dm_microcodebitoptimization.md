---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3DM to MICROCODE BIT OPTIMIZATION"
labels: rule
assignees: ''
---

**Source:** 3DM
**Target:** MICROCODE BIT OPTIMIZATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO10

## GJ Source Entry

> [PO10]  MICROCODE BIT OPTIMIZATION
> INSTANCE:  Finite set A of "micro-commands," collection C = {C1,C2,...,Cm} of subsets of A called "micro-instructions," and a positive integer K.
> QUESTION:  Is there a K-bit instruction format for the given micro-instructions, i.e., is there a partition of A into disjoint subsets A1,A2,...,An such that no pair Ai,Cj have more than one element in common and such that ∑i=1n[log2(|Ai|+1)] ≤ K?
> Reference:  [Robertson, 1978]. Transformation from 3DM.

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Robertson, 1978]**: [`Robertson1978`] E. L. Robertson (1978). "Microcode bit optimization is {NP}-complete". *IEEE Transactions on Computers*.