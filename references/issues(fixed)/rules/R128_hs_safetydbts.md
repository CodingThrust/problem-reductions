---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hitting Set to Safety of Database Transaction Systems"
labels: rule
assignees: ''
---

**Source:** Hitting Set
**Target:** Safety of Database Transaction Systems
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.234-235

## GJ Source Entry

> [SR34] SAFETY OF DATABASE TRANSACTION SYSTEMS (*)
> INSTANCE: Set V of database variables, and a collection T of "transactions" (R_i, W_i), 1 <= i <= n, where R_i and W_i are both subsets of V.
> QUESTION: Is every history H for T equivalent to some serial history?
> Reference: [Papadimitriou, Bernstein, and Rothnie, 1977]. Transformation from HITTING SET.
> Comment: Not known either to be in NP or to be in co-NP. Testing whether every history H for T is "D-equivalent" to some serial history can be done in polynomial time, where two histories are D-equivalent if one can be obtained from the other by a sequence of interchanges of adjacent sets in such a way that at each step the new history is equivalent to the previous one.

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

- **[Papadimitriou, Bernstein, and Rothnie, 1977]**: [`Papadimitriou1977b`] Christos H. Papadimitriou and P. A. Bernstein and J. B. Rothnie (1977). "Some computational problems related to database concurrency control". In: *Proceedings of the Conference on Theoretical Computer Science*, pp. 275–282.