---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Multiprocessor Scheduling"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Multiprocessor Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1 item (7), p.65

## Reduction Algorithm

> (7) MULTIPROCESSOR SCHEDULING
> INSTANCE: A finite set A of "tasks," a "length" l(a) ∈ Z+ for each a ∈ A, a number m ∈ Z+ of "processors," and a "deadline" D ∈ Z+.
> QUESTION: Is there a partition A = A_1 ∪ A_2 ∪ ⋯ ∪ A_m of A into m disjoint sets such that
>
> max { ∑_{a ∈ A_i} l(a) : 1 ≤ i ≤ m } ≤ D ?
>
> Proof: Restrict to PARTITION by allowing only instances in which m = 2 and D = ½∑_{a ∈ A} l(a).

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
