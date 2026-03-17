---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Preemptive Scheduling"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Preemptive Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.240

## GJ Source Entry

> [SS12] PREEMPTIVE SCHEDULING
> INSTANCE: Set T of tasks, number m E Z+ of processors, partial order < on T, length l(t) E Z+ for each t E T, and an overall deadline D E Z+.
> QUESTION: Is there an m-processor "preemptive" schedule for T that obeys the precedence constraints and meets the overall deadline? (Such a schedule σ is identical to an ordinary m-processor schedule, except that we are allowed to subdivide each task t E T into any number of subtasks t_1, t_2, ..., t_k such that sum_{i=1}^{k} l(t_i) = l(t) and it is required that σ(t_{i+1}) >= σ(t_i) + l(t_i) for 1 <= i < k. The precedence constraints are extended to subtasks by requiring that every subtask of t precede every subtask of t' whenever t < t'.)
> Reference: [Ullman, 1975]. Transformation from 3SAT.
> Comment: Can be solved in polynomial time if m = 2 [Muntz and Coffman, 1969], if < is a "forest" [Muntz and Coffman, 1970], or if < is empty and individual task deadlines are allowed [Horn, 1974]. If "(uniform) different speed" processors are allowed, the problem can be solved in polynomial time if m = 2 or if < is empty [Horvath, Lam, and Sethi, 1977], [Gonzalez and Sahni, 1978b] in the latter case even if individual task deadlines are allowed [Sahni and Cho, 1977a]; if both m = 2 and < is empty, it can be solved in polynomial time, even if both integer release times and deadlines are allowed [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1977]. For "unrelated" processors, the case with m fixed and < empty can be solved in polynomial time [Gonzalez, Lawler, and Sahni, 1978], and the case with m arbitrary and < empty can be solved by linear programming [Lawler and Labetoulle, 1978].

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
- **[Muntz and Coffman, 1969]**: [`Muntz1969`] R. R. Muntz and E. G. Coffman, Jr (1969). "Optimal preemptive scheduling on two-processor systems". *IEEE Transactions on Computers* C-18, pp. 1014–1020.
- **[Muntz and Coffman, 1970]**: [`Muntz1970`] R. R. Muntz and E. G. Coffman, Jr (1970). "Preemptive scheduling of real-time tasks on multiprocessor systems". *Journal of the Association for Computing Machinery* 17, pp. 324–338.
- **[Horn, 1974]**: [`Horn1974`] William A. Horn (1974). "Some simple scheduling algorithms". *Naval Research Logistics Quarterly* 21, pp. 177–185.
- **[Horvath, Lam, and Sethi, 1977]**: [`Horvath1977`] E. C. Horvath and S. Lam and Ravi Sethi (1977). "A level algorithm for preemptive scheduling". *Journal of the Association for Computing Machinery* 24, pp. 32–43.
- **[Gonzalez and Sahni, 1978b]**: [`Gonzalez1978b`] T. Gonzalez and S. Sahni (1978). "Flowshop and jobshop schedules: complexity and approximation". *Operations Research* 26, pp. 36–52.
- **[Sahni and Cho, 1977a]**: [`Sahni1977a`] S. Sahni and Y. Cho (1977). "Scheduling independent tasks with due times on a uniform processor system". Computer Science Dept., University of Minnesota.
- **[Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1977]**: [`Labetoulle and Lawler and Lenstra and Rinnooy Kan1977`] Jacques Labetoulle and Eugene L. Lawler and Jan K. Lenstra and A. H. G. Rinnooy Kan (1977). "Preemptive scheduling of uniform machines".
- **[Gonzalez, Lawler, and Sahni, 1978]**: [`Gonzalez1978a`] T. Gonzalez and E. L. Lawler and S. Sahni (1978). "Optimal preemptive scheduling of a fixed number of unrelated processors in polynomial time".
- **[Lawler and Labetoulle, 1978]**: [`Lawler1978b`] Eugene L. Lawler and Jacques Labetoulle (1978). "Preemptive scheduling of unrelated parallel processors". *Journal of the Association for Computing Machinery*.