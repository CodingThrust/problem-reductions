---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to DISJOINT CONNECTING PATHS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** DISJOINT CONNECTING PATHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND40, p.217

## GJ Source Entry

> [ND40] DISJOINT CONNECTING PATHS
> INSTANCE: Graph G=(V,E), collection of disjoint vertex pairs (s_1,t_1),(s_2,t_2),…,(s_k,t_k).
> QUESTION: Does G contain k mutually vertex-disjoint paths, one connecting s_i and t_i for each i, 1≤i≤k?
> Reference: [Knuth, 1974c], [Karp, 1975a], [Lynch, 1974]. Transformation from 3SAT.
> Comment: Remains NP-complete for planar graphs [Lynch, 1974], [Lynch, 1975]. Complexity is open for any fixed k≥2, but can be solved in polynomial time if k=2 and G is planar or chordal [Perl and Shiloach, 1978]. (A polynomial time algorithm for the general 2 path problem has been announced in [Shiloach, 1978]). The directed version of this problem is also NP-complete in general and solvable in polynomial time when k=2 and G is planar or acyclic [Perl and Shiloach, 1978].

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

- **[Knuth, 1974c]**: [`Knuth1974c`] Donald E. Knuth (1974). "".
- **[Karp, 1975a]**: [`Karp1975a`] Richard M. Karp (1975). "On the complexity of combinatorial problems". *Networks* 5, pp. 45–68.
- **[Lynch, 1974]**: [`Lynch1974`] J. F. Lynch (1974). "The equivalence of theorem proving and the interconnection problem".
- **[Lynch, 1975]**: [`Lynch1975`] James F. Lynch (1975). "The equivalence of theorem proving and the interconnection problem". *ACM SIGDA Newsletter* 5(3).
- **[Perl and Shiloach, 1978]**: [`Perl1978`] Y. Perl and Y. Shiloach (1978). "Finding two disjoint paths between two pairs of vertices in a graph". *Journal of the Association for Computing Machinery* 25, pp. 1–9.
- **[Shiloach, 1978]**: [`Shiloach1978`] Yossi Shiloach (1978). "The two paths problem is polynomial". Computer Science Department, Stanford University.