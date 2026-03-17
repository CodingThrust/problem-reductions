---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Deadlock Avoidance"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'DEADLOCK AVOIDANCE'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Deadlock Avoidance
**Motivation:** 3SAT asks whether a Boolean formula in 3-CNF is satisfiable; DEADLOCK AVOIDANCE asks whether a system state is unsafe, meaning there exist control flows for processes such that no sequence of resource allocations/deallocations can reach a final state. Araki, Sugiyama, Kasami, and Okui (1977) proved NP-completeness of deadlock avoidance by reduction from 3SAT, establishing that determining safety of resource allocation states is intractable even when allocation calls are properly nested and involve at most two resources. This is the foundational complexity result for deadlock analysis in operating systems and concurrent programming.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.4, p.244

## GJ Source Entry

> [SS22] DEADLOCK AVOIDANCE
> INSTANCE: Set {P_1, P_2, ..., P_m} of "process flow diagrams" (directed acyclic graphs), set Q of "resources," state S of system giving current "active" vertex in each process and "allocation" of resources (see references for details).
> QUESTION: Is S "unsafe," i.e., are there control flows for the various processes from state S such that no sequence of resource allocations and deallocations can enable the system to reach a "final" state?
> Reference: [Araki, Sugiyama, Kasami, and Okui, 1977], [Sugiyama, Araki, Okui, and Kasami, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete even if allocation calls are "properly nested" and no allocation call involves more than two resources. See references for additional complexity results. See also [Gold, 1978] for results and algorithms for a related model of the deadlock problem.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3-CNF formula phi with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a DEADLOCK AVOIDANCE instance as follows:

1. **Resources:** Create one resource q_i for each variable x_i (n resources total), plus auxiliary resources for clause enforcement (O(m) additional resources). Each resource has capacity 1 (single unit).
2. **Variable processes:** For each variable x_i, create a process P_i whose flow diagram is a DAG with a branch: one branch allocates q_i in the "positive" order (representing x_i = true) and the other in the "negative" order (representing x_i = false). The process must choose one branch, locking in a truth assignment for x_i.
3. **Clause processes:** For each clause C_j = (l_a ∨ l_b ∨ l_c), create a process P_{n+j} that requires resources corresponding to all three literals. The process attempts to acquire the resources in an order that succeeds only if at least one literal's resource is available (i.e., the corresponding variable process chose the branch that does not hold that resource).
4. **Initial state S:** All processes are at their start vertices. Variable resources are unallocated.
5. **Correctness:** The state S is safe (not deadlocked) if and only if there is a truth assignment satisfying phi. If phi is satisfiable, variable processes can choose branches matching the satisfying assignment, and clause processes can complete because each clause has at least one satisfied literal freeing a resource. If phi is unsatisfiable, for any choice of branches, some clause process will be blocked, leading to a potential deadlock.
6. **Solution extraction:** From a safe execution sequence, extract the truth assignment from the branch choices of variable processes: x_i = true if P_i took the positive branch, false otherwise.

Note: The reduction shows that the complement problem (is S safe?) is co-NP-complete, but since the question asks whether S is unsafe, the decision problem "is S unsafe?" is NP-complete.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in the 3SAT instance (`num_variables`)
- m = number of clauses (`num_clauses`)

| Target metric (code name)   | Polynomial (using symbols above)   |
|-----------------------------|------------------------------------|
| `num_processes`             | n + m                              |
| `num_resources`             | O(n + m)                           |
| `num_dag_vertices`          | O(n + m) (constant-size per gadget)|

**Derivation:** Each variable contributes one process and one primary resource. Each clause contributes one process and O(1) auxiliary resources. The DAGs are constant-size per gadget (bounded branching). Construction is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3SAT instance, reduce to DEADLOCK AVOIDANCE, determine safety by exhaustive exploration of all possible execution interleavings (exponential but correct), verify that the state is unsafe iff the original formula is satisfiable.
- Check that the constructed instance has n + m processes and O(n + m) resources.
- Edge cases: satisfiable formula (expect unsafe state), unsatisfiable formula (expect safe state — no deadlock-inducing execution exists), formula with single variable and one clause.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3, x_4, x_5
Clauses (m = 5):
- C_1 = (x_1 ∨ ¬x_2 ∨ x_3)
- C_2 = (¬x_1 ∨ x_2 ∨ x_4)
- C_3 = (x_2 ∨ ¬x_3 ∨ x_5)
- C_4 = (¬x_1 ∨ ¬x_4 ∨ x_5)
- C_5 = (x_3 ∨ x_4 ∨ ¬x_5)

Satisfying assignment: x_1 = T, x_2 = T, x_3 = T, x_4 = T, x_5 = T.

**Constructed DEADLOCK AVOIDANCE instance:**
- Resources: q_1, q_2, q_3, q_4, q_5 (one per variable) + auxiliary clause resources
- Processes: P_1, ..., P_5 (variable processes) + P_6, ..., P_10 (clause processes)
- Total: 10 processes, ~10 resources

**Variable process branches:**
- P_1: branch-true (x_1 = T): holds q_1 in "positive" pattern; branch-false: "negative" pattern
- P_2 through P_5: similarly for x_2 through x_5

**Solution (safe execution for x_1=x_2=x_3=x_4=x_5=T):**
All variable processes take the "true" branch. For each clause:
- C_1: x_1 = T satisfies literal x_1, so P_6 can acquire its needed resource
- C_2: x_2 = T satisfies literal x_2, so P_7 completes
- C_3: x_2 = T satisfies literal x_2, so P_8 completes
- C_4: x_5 = T satisfies literal x_5, so P_9 completes
- C_5: x_3 = T satisfies literal x_3, so P_10 completes

All processes reach their final states — the state S is safe under this assignment.
Since the formula is satisfiable, the answer to "is S unsafe?" depends on whether adversarial control flows can force deadlock — with a satisfying assignment available, the system can always be steered to completion, so S is safe. ✓

(Note: The NP-complete question asks whether S is unsafe. For a satisfiable formula, S is safe; for an unsatisfiable formula, S would be unsafe.)


## References

- **[Araki, Sugiyama, Kasami, and Okui, 1977]**: [`Araki1977`] T. Araki and Y. Sugiyama and T. Kasami and J. Okui (1977). "Complexity of the deadlock avoidance problem". In: *Proceedings of the 2nd IBM Symposium on Mathematical Foundations of Computer Science*, pp. 229–252. IBM Japan.
- **[Sugiyama, Araki, Okui, and Kasami, 1977]**: [`Sugiyama and Araki and Okui and Kasami1977`] Yasuaki Sugiyama and Toshinori Araki and Junichi Okui and Tadao Kasami (1977). "Complexity of the deadlock avoidance problem". *Trans. IECE Japan* 60-D, pp. 251–258.
- **[Gold, 1978]**: [`Gold1978`] E. M. Gold (1978). "Deadlock protection: easy and difficult cases". *SIAM Journal on Computing* 7, pp. 320–336.
