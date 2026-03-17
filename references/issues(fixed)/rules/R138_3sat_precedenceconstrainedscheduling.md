---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Precedence Constrained Scheduling"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Precedence Constrained Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.239

## GJ Source Entry

> [SS9] PRECEDENCE CONSTRAINED SCHEDULING
> INSTANCE: Set T of tasks, each having length l(t) = 1, number m E Z+ of processors, partial order < on T, and a deadline D E Z+.
> QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the precedence constraints, i.e., such that t < t' implies σ(t') >= σ(t) + l(t) = σ(t) + 1?
> Reference: [Ullman, 1975]. Transformation from 3SAT.
> Comment: Remains NP-complete for D = 3 [Lenstra and Rinnooy Kan, 1978a]. Can be solved in polynomial time if m = 2 (e.g., see [Coffman and Graham, 1972]) or if m is arbitrary and < is a "forest" [Hu, 1961] or has a chordal graph as complement [Papadimitriou and Yannakakis, 1978b]. Complexity remains open for all fixed m >= 3 when < is arbitrary. The m = 2 case becomes NP-complete if both task lengths 1 and 2 are allowed [Ullman, 1975]. If each task t can only be executed by a specified processor p(t), the problem is NP-complete for m = 2 and < arbitrary, and for m arbitrary and < a forest, but can be solved in polynomial time for m arbitrary if < is a "cyclic forest" [Goyal, 1976].

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

- **[Ullman, 1975]**: [`Ullman1975`] Jeffrey D. Ullman (1975). "{NP}-complete scheduling problems". *Journal of Computer and System Sciences* 10, pp. 384–393.
- **[Lenstra and Rinnooy Kan, 1978a]**: [`Lenstra1978a`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Complexity of scheduling under precedence constraints". *Operations Research* 26, pp. 22–35.
- **[Coffman and Graham, 1972]**: [`Coffman1972`] E. G. Coffman, Jr and R. L. Graham (1972). "Optimal scheduling for two-processor systems". *Acta Informatica* 1, pp. 200–213.
- **[Hu, 1961]**: [`Hu1961`] Te C. Hu (1961). "Parallel sequencing and assembly line problems". *Operations Research* 9, pp. 841–848.
- **[Papadimitriou and Yannakakis, 1978b]**: [`Papadimitriou1978f`] Christos H. Papadimitriou and M. Yannakakis (1978). "On the complexity of minimum spanning tree problems".
- **[Goyal, 1976]**: [`Goyal1976`] D. K. Goyal (1976). "Scheduling processor bound systems". Computer Science Department, Washington State University.