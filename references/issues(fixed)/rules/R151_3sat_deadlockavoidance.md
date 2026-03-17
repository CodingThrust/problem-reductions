---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Deadlock Avoidance"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Deadlock Avoidance
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.4, p.244

## GJ Source Entry

> [SS22] DEADLOCK AVOIDANCE
> INSTANCE: Set {P_1, P_2, ..., P_m} of "process flow diagrams" (directed acyclic graphs), set Q of "resources," state S of system giving current "active" vertex in each process and "allocation" of resources (see references for details).
> QUESTION: Is S "unsafe," i.e., are there control flows for the various processes from state S such that no sequence of resource allocations and deallocations can enable the system to reach a "final" state?
> Reference: [Araki, Sugiyama, Kasami, and Okui, 1977], [Sugiyama, Araki, Okui, and Kasami, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete even if allocation calls are "properly nested" and no allocation call involves more than two resources. See references for additional complexity results. See also [Gold, 1978] for results and algorithms for a related model of the deadlock problem.

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

- **[Araki, Sugiyama, Kasami, and Okui, 1977]**: [`Araki1977`] T. Araki and Y. Sugiyama and T. Kasami and J. Okui (1977). "Complexity of the deadlock avoidance problem". In: *Proceedings of the 2nd IBM Symposium on Mathematical Foundations of Computer Science*, pp. 229–252. IBM Japan.
- **[Sugiyama, Araki, Okui, and Kasami, 1977]**: [`Sugiyama and Araki and Okui and Kasami1977`] Yasuaki Sugiyama and Toshinori Araki and Junichi Okui and Tadao Kasami (1977). "Complexity of the deadlock avoidance problem". *Trans. IECE Japan* 60-D, pp. 251–258.
- **[Gold, 1978]**: [`Gold1978`] E. M. Gold (1978). "Deadlock protection: easy and difficult cases". *SIAM Journal on Computing* 7, pp. 320–336.