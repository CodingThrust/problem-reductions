---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Precedence Constrained Scheduling"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'PRECEDENCE CONSTRAINED SCHEDULING'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Precedence Constrained Scheduling
**Motivation:** 3SAT is the canonical NP-complete problem; PRECEDENCE CONSTRAINED SCHEDULING asks whether unit-length tasks with precedence constraints can be scheduled on m processors by deadline D. Ullman's 1975 reduction encodes Boolean variables as pairs of complementary tasks (literal gadgets) that compete for processor time slots, and clauses as chain gadgets whose scheduling is feasible only when at least one literal per clause is "true" (scheduled early). This establishes NP-completeness of multiprocessor scheduling even with unit execution times, a foundational result in scheduling theory.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.239

## GJ Source Entry

> [SS9] PRECEDENCE CONSTRAINED SCHEDULING
> INSTANCE: Set T of tasks, each having length l(t) = 1, number m E Z+ of processors, partial order < on T, and a deadline D E Z+.
> QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the precedence constraints, i.e., such that t < t' implies σ(t') >= σ(t) + l(t) = σ(t) + 1?
> Reference: [Ullman, 1975]. Transformation from 3SAT.
> Comment: Remains NP-complete for D = 3 [Lenstra and Rinnooy Kan, 1978a]. Can be solved in polynomial time if m = 2 (e.g., see [Coffman and Graham, 1972]) or if m is arbitrary and < is a "forest" [Hu, 1961] or has a chordal graph as complement [Papadimitriou and Yannakakis, 1978b]. Complexity remains open for all fixed m >= 3 when < is arbitrary. The m = 2 case becomes NP-complete if both task lengths 1 and 2 are allowed [Ullman, 1975]. If each task t can only be executed by a specified processor p(t), the problem is NP-complete for m = 2 and < arbitrary, and for m arbitrary and < a forest, but can be solved in polynomial time for m arbitrary if < is a "cyclic forest" [Goyal, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance: a Boolean formula φ in 3-CNF with variables x_1, ..., x_n and clauses C_1, ..., C_k, construct a PRECEDENCE CONSTRAINED SCHEDULING instance as follows.

1. **Variable gadgets:** For each variable x_i, create two unit-length tasks: t_{x_i} (representing the positive literal) and t_{¬x_i} (representing the negative literal). Add a precedence constraint forcing them into a chain of length 2: one task in time slot 1 and the other in time slot 2. This models the truth assignment — whichever task is scheduled in slot 1 corresponds to the literal being set TRUE.

2. **Clause gadgets:** For each clause C_j = (l_{j1} ∨ l_{j2} ∨ l_{j3}), create a chain of D − 1 unit tasks (a "clause chain") that must occupy time slots 2, 3, ..., D. The first task in the chain has a precedence dependency on the three literal tasks corresponding to l_{j1}, l_{j2}, l_{j3}. Specifically, it requires that at least one of these literal tasks is scheduled in time slot 1 (i.e., set TRUE), so the clause chain can begin in slot 2.

3. **Processors and deadline:** Set the number of processors m and deadline D based on the formula's size. Specifically, m is chosen so that the processor capacity per time slot is tight: time slot 1 must accommodate exactly n literal tasks (one per variable pair), and subsequent slots must accommodate the clause chain tasks. A typical construction uses D = k + 1 (where k = number of clauses) and m = n + k, with the constraint that each clause chain occupies exactly one processor slot per time step from slot 2 to slot D.

4. **Correctness:** The variable gadget forces a truth assignment (which literal task goes in slot 1). The clause chains can only start in slot 2 if at least one literal in the clause is TRUE (scheduled in slot 1). If all three literal tasks of a clause are in slot 2 (all FALSE), the clause chain's first task also needs slot 2, exceeding processor capacity. Hence, φ is satisfiable iff a feasible schedule exists.

5. **Solution extraction:** From a feasible schedule σ, set x_i = TRUE if t_{x_i} is scheduled in time slot 1, and x_i = FALSE if t_{¬x_i} is scheduled in slot 1. This yields a satisfying assignment for φ.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in the 3SAT instance (`num_variables` of source)
- k = number of clauses (`num_clauses` of source)

| Target metric (code name)       | Polynomial (using symbols above) |
|----------------------------------|----------------------------------|
| `num_tasks`                      | 2n + k(D − 1), typically O(n + k^2) |
| `num_processors`                 | n + k                            |
| `deadline`                       | k + 1                            |
| `num_precedence_constraints`     | n + 3k + k(D − 2), typically O(n + k^2) |

**Derivation:** Each variable contributes 2 tasks (literal pair) and 1 precedence edge. Each clause contributes a chain of D − 1 tasks with D − 2 internal edges plus 3 edges from literals. Construction is O(n + k^2).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small 3SAT instance (e.g., 3 variables, 4 clauses), reduce to PRECEDENCE CONSTRAINED SCHEDULING, enumerate all valid schedules (assignments of tasks to time slots respecting precedence and processor count), verify a feasible schedule exists iff the formula is satisfiable.
- Check the number of tasks, processors, deadline, and precedence edges match the overhead formulas.
- Edge cases: test with an unsatisfiable 3SAT formula (expect no feasible schedule), a single-clause formula (trivially satisfiable), and a formula where every variable appears in both polarities.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3, x_4, x_5
Clauses:
- C_1 = (x_1 ∨ x_2 ∨ ¬x_3)
- C_2 = (¬x_1 ∨ x_3 ∨ x_4)
- C_3 = (x_2 ∨ ¬x_4 ∨ x_5)
- C_4 = (¬x_2 ∨ ¬x_3 ∨ ¬x_5)
- C_5 = (x_1 ∨ x_4 ∨ x_5)

n = 5 variables, k = 5 clauses.

Satisfying assignment: x_1 = T, x_2 = T, x_3 = F, x_4 = T, x_5 = F.

**Constructed PRECEDENCE CONSTRAINED SCHEDULING instance (simplified):**

- 10 literal tasks: t_{x1}, t_{¬x1}, t_{x2}, t_{¬x2}, t_{x3}, t_{¬x3}, t_{x4}, t_{¬x4}, t_{x5}, t_{¬x5}
- 5 clause chains, each of length D − 1
- D = 6 (= k + 1), m = 10 (= n + k)
- Each variable pair has precedence: t_{xi} < t_{¬xi} or vice versa (forming a 2-chain)
- Each clause chain's first task depends on the 3 corresponding literal tasks

**Schedule (time slot assignments):**

| Time slot | Tasks scheduled (up to m = 10 processors) |
|-----------|-------------------------------------------|
| 1         | t_{x1}, t_{x2}, t_{¬x3}, t_{x4}, t_{¬x5} (TRUE literals) |
| 2         | t_{¬x1}, t_{¬x2}, t_{x3}, t_{¬x4}, t_{x5}, clause chain heads c1_1, c2_1, c3_1, c4_1, c5_1 |
| 3–6       | Remaining clause chain tasks |

All clause chains can start at slot 2 because each clause has at least one TRUE literal in slot 1 ✓
Total tasks fit within m = 10 processors per slot ✓
All tasks complete by deadline D = 6 ✓

**Solution extraction:**
x_1 = T (t_{x1} in slot 1), x_2 = T (t_{x2} in slot 1), x_3 = F (t_{¬x3} in slot 1), x_4 = T (t_{x4} in slot 1), x_5 = F (t_{¬x5} in slot 1).
Verification: C_1 = T∨T∨T ✓, C_2 = F∨T∨T ✓, C_3 = T∨F∨F — need to check: x_2=T ✓, so C_3 satisfied ✓, C_4 = F∨T∨T ✓, C_5 = T∨T∨F ✓.


## References

- **[Ullman, 1975]**: [`Ullman1975`] Jeffrey D. Ullman (1975). "{NP}-complete scheduling problems". *Journal of Computer and System Sciences* 10, pp. 384–393.
- **[Lenstra and Rinnooy Kan, 1978a]**: [`Lenstra1978a`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Complexity of scheduling under precedence constraints". *Operations Research* 26, pp. 22–35.
- **[Coffman and Graham, 1972]**: [`Coffman1972`] E. G. Coffman, Jr and R. L. Graham (1972). "Optimal scheduling for two-processor systems". *Acta Informatica* 1, pp. 200–213.
- **[Hu, 1961]**: [`Hu1961`] Te C. Hu (1961). "Parallel sequencing and assembly line problems". *Operations Research* 9, pp. 841–848.
- **[Papadimitriou and Yannakakis, 1978b]**: [`Papadimitriou1978f`] Christos H. Papadimitriou and M. Yannakakis (1978). "On the complexity of minimum spanning tree problems".
- **[Goyal, 1976]**: [`Goyal1976`] D. K. Goyal (1976). "Scheduling processor bound systems". Computer Science Department, Washington State University.
