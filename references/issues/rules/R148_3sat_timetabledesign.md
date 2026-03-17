---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Timetable Design"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'TIMETABLE DESIGN'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Timetable Design
**Motivation:** 3SAT asks whether a Boolean formula in 3-CNF is satisfiable; TIMETABLE DESIGN asks whether craftsmen can be assigned to tasks across work periods subject to availability and requirement constraints. Even, Itai, and Shamir (1976) showed that even a very primitive version of the timetable problem is NP-complete via reduction from 3SAT, establishing that all common timetabling problems are intractable. This is the foundational hardness result for university and school scheduling.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.4, p.243

## GJ Source Entry

> [SS19] TIMETABLE DESIGN
> INSTANCE: Set H of "work periods," set C of "craftsmen," set T of "tasks," a subset A(c) ⊆ H of "available hours" for each craftsman c E C, a subset A(t) ⊆ H of "available hours" for each task t E T, and, for each pair (c,t) E C×T, a number R(c,t) E Z_0+ of "required work periods."
> QUESTION: Is there a timetable for completing all the tasks, i.e., a function f: C×T×H → {0,1} (where f(c,t,h) = 1 means that craftsman c works on task t during period h) such that (1) f(c,t,h) = 1 only if h E A(c) ∩ A(t), (2) for each h E H and c E C there is at most one t E T for which f(c,t,h) = 1, (3) for each h E H and t E T there is at most one c E C for which f(c,t,h) = 1, and (4) for each pair (c,t) E C×T there are exactly R(c,t) values of h for which f(c,t,h) = 1?
> Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete even if |H| = 3, A(t) = H for all t E T, and each R(c,t) E {0,1}. The general problem can be solved in polynomial time if |A(c)| <= 2 for all c E C or if A(c) = A(t) = H for all c E C and t E T.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3-CNF formula phi with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a TIMETABLE DESIGN instance with |H| = 3 work periods, A(t) = H for all tasks, and all R(c,t) in {0,1} as follows:

1. **Work periods:** H = {h_1, h_2, h_3} (three periods).
2. **Variable gadgets:** For each variable x_i, create two craftsmen c_i^+ (representing x_i = true) and c_i^- (representing x_i = false). Create three tasks for each variable: t_i^1, t_i^2, t_i^3. Set up requirements so that exactly one of c_i^+ or c_i^- works during each period, encoding a truth assignment.
3. **Clause gadgets:** For each clause C_j = (l_a ∨ l_b ∨ l_c), create a task t_j^clause that must be performed exactly once. The three literals' craftsmen are made available for this task in distinct periods. If a literal's craftsman is "free" in the period corresponding to its clause task (i.e., the variable is set to satisfy that literal), it can cover the clause task.
4. **Availability constraints:** Craftsmen for variable x_i have availability sets that force a binary choice (true/false) across the three periods. Clause tasks are available in all three periods, but only a craftsman whose literal satisfies the clause is required to work on it.
5. **Correctness:** The timetable exists if and only if there is a truth assignment satisfying phi. A satisfying assignment frees at least one literal-craftsman per clause to cover the clause task. Conversely, a valid timetable implies an assignment where each clause has a covering literal.
6. **Solution extraction:** From a valid timetable f, set x_i = true if c_i^+ is used in the "positive" pattern, x_i = false otherwise.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in the 3SAT instance (`num_variables`)
- m = number of clauses (`num_clauses`)

| Target metric (code name)   | Polynomial (using symbols above) |
|-----------------------------|----------------------------------|
| `num_work_periods`          | 3 (constant)                     |
| `num_craftsmen`             | O(n + m) = 2 * n + m            |
| `num_tasks`                 | O(n + m) = 3 * n + m            |

**Derivation:** Each variable contributes 2 craftsmen and 3 tasks for the variable gadget. Each clause contributes 1 task and potentially 1 auxiliary craftsman. The number of work periods is fixed at 3 (as noted in the GJ comment, NP-completeness holds even with |H| = 3). Construction is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3SAT instance, reduce to TIMETABLE DESIGN, solve the timetable by brute-force enumeration of all possible assignment functions f: C x T x H -> {0,1} satisfying constraints (1)-(4), verify that a valid timetable exists iff the original formula is satisfiable.
- Check that the constructed instance has |H| = 3, all R(c,t) in {0,1}, and A(t) = H for all tasks.
- Edge cases: unsatisfiable formula (expect no valid timetable), formula with single clause (minimal instance), all-positive or all-negative literals.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3, x_4, x_5
Clauses (m = 5):
- C_1 = (x_1 ∨ x_2 ∨ ¬x_3)
- C_2 = (¬x_1 ∨ x_3 ∨ x_4)
- C_3 = (x_2 ∨ ¬x_4 ∨ x_5)
- C_4 = (¬x_2 ∨ ¬x_3 ∨ ¬x_5)
- C_5 = (x_1 ∨ x_4 ∨ x_5)

Satisfying assignment: x_1 = T, x_2 = T, x_3 = F, x_4 = T, x_5 = T.

**Constructed TIMETABLE DESIGN instance:**
- H = {h_1, h_2, h_3}
- Craftsmen: c_1^+, c_1^-, c_2^+, c_2^-, c_3^+, c_3^-, c_4^+, c_4^-, c_5^+, c_5^- (10 variable craftsmen) + auxiliary clause craftsmen (15 total)
- Tasks: t_1^1, t_1^2, t_1^3, ..., t_5^1, t_5^2, t_5^3 (15 variable tasks) + t_C1, t_C2, t_C3, t_C4, t_C5 (5 clause tasks) = 20 tasks total
- All R(c,t) in {0,1}, A(t) = H for all tasks

**Solution:**
The satisfying assignment x_1=T, x_2=T, x_3=F, x_4=T, x_5=T determines which craftsmen take the "positive" vs "negative" pattern. For each clause, at least one literal is true, so its craftsman is free to cover the clause task:
- C_1: x_1=T covers it (c_1^+ is free)
- C_2: x_4=T covers it (c_4^+ is free)
- C_3: x_2=T covers it (c_2^+ is free)
- C_4: x_3=F means ¬x_3=T covers it (c_3^- is free)
- C_5: x_1=T covers it (c_1^+ is free)

A valid timetable exists. ✓


## References

- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691–703.
