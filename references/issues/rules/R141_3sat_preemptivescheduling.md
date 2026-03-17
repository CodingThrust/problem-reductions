---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Preemptive Scheduling"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'PREEMPTIVE SCHEDULING'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Preemptive Scheduling
**Motivation:** 3-SATISFIABILITY asks whether a CNF formula with at most 3 literals per clause is satisfiable; PREEMPTIVE SCHEDULING asks whether tasks with integer lengths and precedence constraints can be preemptively scheduled on m processors by a deadline D. Ullman's 1975 reduction encodes each variable and clause of a 3SAT formula as tasks with carefully designed lengths and precedence constraints, so that a feasible preemptive schedule within deadline D exists if and only if the formula is satisfiable. This establishes NP-completeness of preemptive scheduling even when the non-preemptive version with unit tasks is already hard.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.240

## GJ Source Entry

> [SS12] PREEMPTIVE SCHEDULING
> INSTANCE: Set T of tasks, number m E Z+ of processors, partial order < on T, length l(t) E Z+ for each t E T, and an overall deadline D E Z+.
> QUESTION: Is there an m-processor "preemptive" schedule for T that obeys the precedence constraints and meets the overall deadline? (Such a schedule σ is identical to an ordinary m-processor schedule, except that we are allowed to subdivide each task t E T into any number of subtasks t_1, t_2, ..., t_k such that sum_{i=1}^{k} l(t_i) = l(t) and it is required that σ(t_{i+1}) >= σ(t_i) + l(t_i) for 1 <= i < k. The precedence constraints are extended to subtasks by requiring that every subtask of t precede every subtask of t' whenever t < t'.)
> Reference: [Ullman, 1975]. Transformation from 3SAT.
> Comment: Can be solved in polynomial time if m = 2 [Muntz and Coffman, 1969], if < is a "forest" [Muntz and Coffman, 1970], or if < is empty and individual task deadlines are allowed [Horn, 1974]. If "(uniform) different speed" processors are allowed, the problem can be solved in polynomial time if m = 2 or if < is empty [Horvath, Lam, and Sethi, 1977], [Gonzalez and Sahni, 1978b] in the latter case even if individual task deadlines are allowed [Sahni and Cho, 1977a]; if both m = 2 and < is empty, it can be solved in polynomial time, even if both integer release times and deadlines are allowed [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1977]. For "unrelated" processors, the case with m fixed and < empty can be solved in polynomial time [Gonzalez, Lawler, and Sahni, 1978], and the case with m arbitrary and < empty can be solved by linear programming [Lawler and Labetoulle, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let phi be a 3SAT instance with n variables x_1, ..., x_n and c clauses C_1, ..., C_c, each clause containing at most 3 literals.

Ullman's construction creates tasks and precedence constraints that encode the satisfiability problem into a preemptive scheduling problem.

1. **Variable gadgets:** For each variable x_i, create two tasks: a "positive" task p_i (representing x_i = TRUE) and a "negative" task n_i (representing x_i = FALSE). These are given equal lengths and are structured so that exactly one can be scheduled in a critical time window — encoding the truth assignment.

2. **Clause gadgets:** For each clause C_j, create a clause task c_j whose execution depends (via precedence) on the tasks corresponding to the literals in the clause. The lengths and precedence structure ensure c_j can meet its deadline only if at least one literal-task in the clause is scheduled favorably (i.e., the corresponding literal is set to TRUE).

3. **Filler tasks:** Additional tasks may be introduced to fill processor capacity, ensuring the scheduling constraints are tight.

4. **Processors and deadline:** Set m and D based on n and c so that the total processing capacity (m * D) exactly matches the total work, leaving no slack.

5. **Correctness:** A satisfying assignment for phi induces a feasible schedule (TRUE literals scheduled in favorable slots, allowing clause tasks to meet deadlines). Conversely, any feasible schedule implies a consistent truth assignment satisfying all clauses.

6. **Solution extraction:** Given a feasible preemptive schedule, determine the truth assignment from which literal-tasks are scheduled in the critical window: x_i = TRUE if p_i is in the favorable slot, x_i = FALSE if n_i is.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in the 3SAT formula
- c = number of clauses

| Target metric (code name)   | Polynomial (using symbols above) |
|------------------------------|----------------------------------|
| `num_tasks`                  | O(n + c)                         |
| `num_processors`             | O(n)                             |
| `num_precedence_constraints` | O(n + c)                         |
| `deadline`                   | O(n)                             |
| `max_task_length`            | O(n)                             |

**Derivation:** Ullman's construction creates O(n) variable-gadget tasks and O(c) clause-gadget tasks. The number of processors and the deadline scale with n. The precedence constraints are O(n + c) since each clause contributes at most 3 edges. Exact constants depend on the specific construction variant. Construction is polynomial in n + c.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3SAT instance, reduce to PREEMPTIVE SCHEDULING, solve the scheduling instance by brute-force enumeration of preemptive schedules, verify the schedule corresponds to a satisfying assignment.
- Check constructed instance sizes match the overhead formulas.
- Edge cases: test with a trivially satisfiable formula (single clause), unsatisfiable formula (e.g., x AND NOT x clauses), and a formula with 5+ variables and 5+ clauses for non-trivial coverage.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3, x_4, x_5
Clauses: C_1 = (x_1 OR x_2 OR NOT x_3), C_2 = (NOT x_1 OR x_3 OR x_4), C_3 = (x_2 OR NOT x_4 OR x_5), C_4 = (NOT x_2 OR NOT x_3 OR NOT x_5), C_5 = (x_1 OR x_4 OR x_5)

Satisfying assignment: x_1 = T, x_2 = T, x_3 = F, x_4 = T, x_5 = F
Check: C_1 = (T OR T OR T) = T, C_2 = (F OR F OR T) = T, C_3 = (T OR F OR F) = T, C_4 = (F OR T OR T) = T, C_5 = (T OR T OR F) = T.

**Constructed PREEMPTIVE SCHEDULING instance:**
Following Ullman's construction (details depend on specific encoding), the instance would have approximately 10-15 tasks (2 per variable + clause tasks + fillers), a handful of processors, and a deadline scaled to the number of variables. The feasible preemptive schedule encodes the satisfying assignment through which variable-gadget tasks occupy the critical time windows.

**Solution:**
The preemptive schedule assigns positive-literal tasks for x_1, x_2, x_4 and negative-literal tasks for x_3, x_5 to favorable time slots. Each clause task meets its deadline because at least one of its literal predecessors is in the favorable position. The schedule meets the overall deadline D.

**Solution extraction:**
Truth assignment: x_1 = T, x_2 = T, x_3 = F, x_4 = T, x_5 = F — satisfies all 5 clauses.


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
