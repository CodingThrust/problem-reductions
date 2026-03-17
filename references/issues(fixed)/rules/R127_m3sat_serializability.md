---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Monotone 3SAT to Serializability of Database Histories"
labels: rule
assignees: ''
---

**Source:** Monotone 3SAT
**Target:** Serializability of Database Histories
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.234

## GJ Source Entry

> [SR33] SERIALIZABILITY OF DATABASE HISTORIES
> INSTANCE: Set V of database variables, collection T of "transactions" (R_i, W_i), 1 <= i <= n, where R_i and W_i are both subsets of V (called the "read set" and the "write set," respectively), and a "history" H for T, where a history is simply a permutation of all the R_i and the W_i in which each R_i occurs before the corresponding W_i.
> QUESTION: Is there a serial history H' for T (i.e., a history in which each R_i occurs immediately before the corresponding W_i) that is equivalent to H in the sense that (1) both histories have the same set of "live" transactions (where a transaction (R_i, W_i) is live in a history if there is some v E V such that either W_i is the last write set to contain v or W_i is the last write set to contain v before v appears in the read set of some other live transaction), and (2) for any two live transactions (R_i, W_i) and (R_j, W_j) and any v E W_i ∩ R_j, W_i is the last write set to contain v before R_j in H if and only if W_i is the last write set to contain v before R_j in H'?
> Reference: [Papadimitriou, Bernstein, and Rothnie, 1977], [Papadimitriou, 1978c]. Transformation from MONOTONE 3SAT.
> Comment: For related polynomial time solvable subcases and variants, see [Papadimitriou, 1978c].

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
- **[Papadimitriou, 1978c]**: [`Papadimitriou1978c`] Christos H. Papadimitriou (1978). "Serializability of concurrent updates". Center for Research in Computing Technology, Harvard University.