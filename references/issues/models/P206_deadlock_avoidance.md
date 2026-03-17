---
name: Problem
about: Propose a new problem type
title: "[Model] DeadlockAvoidance"
labels: model
assignees: ''
---

## Motivation

DEADLOCK AVOIDANCE (P206) from Garey & Johnson, A5 SS22. A classical NP-complete problem in concurrent systems: given a set of processes (each described by a directed acyclic flow diagram with resource allocation/deallocation operations), a set of resources, and a current system state, determine whether the state is "unsafe" — meaning adversarial control flows can lead to deadlock where no execution sequence allows all processes to complete. Shown NP-complete by Araki, Sugiyama, Kasami, and Okui (1977) via reduction from 3SAT. The problem remains NP-complete even with properly nested allocations and at most two resources per allocation call. This is the foundational complexity result for deadlock analysis in operating systems.

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R151: 3SAT -> Deadlock Avoidance (incoming, [Araki, Sugiyama, Kasami, and Okui, 1977])

## Definition

**Name:** `DeadlockAvoidance`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS22

**Mathematical definition:**

INSTANCE: Set {P1,P2,...,Pm} of "process flow diagrams" (directed acyclic graphs), set Q of "resources," state S of system giving current "active" vertex in each process and "allocation" of resources (see references for details).
QUESTION: Is S "unsafe," i.e., are there control flows for the various processes from state S such that no sequence of resource allocations and deallocations can enable the system to reach a "final" state?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** m (one variable per process, representing which branch/path to follow in its flow diagram)
- **Per-variable domain:** The set of possible control flow paths from the current active vertex to the final vertex in each process's DAG
- **Meaning:** For each process P_i, the adversary chooses a control flow path (a sequence of vertices in the DAG). The system must then find an interleaving of resource allocations and deallocations across all processes that allows all to complete. The state is unsafe if there exist adversarial path choices such that no valid interleaving exists.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `DeadlockAvoidance`
**Variants:** none

| Field              | Type                        | Description                                                            |
|--------------------|-----------------------------|------------------------------------------------------------------------|
| `num_processes`    | `usize`                     | Number of processes m                                                  |
| `flow_diagrams`    | `Vec<DAG>`                  | Process flow diagram for each P_i (directed acyclic graph)             |
| `resources`        | `Vec<Resource>`             | Set Q of resources with capacities                                     |
| `active_vertices`  | `Vec<usize>`                | Current active vertex in each process (initial state S)                |
| `allocations`      | `Vec<Vec<usize>>`           | Current resource allocation per process (part of state S)              |

Where `DAG` contains vertices with resource allocation/deallocation operations and edges representing control flow branches, and `Resource` specifies a resource type with capacity (number of available units).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete (Araki et al., 1977). Dijkstra's Banker's algorithm solves the restricted case of single-unit resource requests with sequential (non-branching) processes in O(m^2 * |Q|) time, but this does not apply to the general case with branching flow diagrams. For the general case with arbitrary DAG flow diagrams, determining safety requires exploring all possible control flow combinations and execution interleavings, which is exponential. No known exact algorithm improves upon O*(product of path-count per process) brute-force enumeration in the worst case. Gold (1978) provides algorithms for related restricted deadlock models.

## Extra Remark

**Full book text:**

INSTANCE: Set {P1,P2,...,Pm} of "process flow diagrams" (directed acyclic graphs), set Q of "resources," state S of system giving current "active" vertex in each process and "allocation" of resources (see references for details).
QUESTION: Is S "unsafe," i.e., are there control flows for the various processes from state S such that no sequence of resource allocations and deallocations can enable the system to reach a "final" state?

Reference: [Araki, Sugiyama, Kasami, and Okui, 1977], [Sugiyama, Araki, Okui, and Kasami, 1977]. Transformation from 3SAT.

Comment: Remains NP-complete even if allocation calls are "properly nested" and no allocation call involves more than two resources. See references for additional complexity results. See also [Gold, 1978] for results and algorithms for a related model of the deadlock problem.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all combinations of control flow paths for each process; for each combination, check if any interleaving of allocations/deallocations allows all processes to complete.)
- [x] It can be solved by reducing to integer programming. (Encode flow paths as binary variables, resource constraints as linear inequalities, and interleaving feasibility via scheduling constraints.)
- [ ] Other: Banker's algorithm for restricted cases (sequential processes, single-unit requests). State-space exploration for small instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
m = 3 processes, Q = {R1, R2} (2 resources, each with capacity 1)

Process P1 (flow diagram):
```
start -> alloc(R1) -> alloc(R2) -> dealloc(R2) -> dealloc(R1) -> end
```

Process P2 (flow diagram with branch):
```
start -> branch:
  left:  alloc(R2) -> alloc(R1) -> dealloc(R1) -> dealloc(R2) -> end
  right: alloc(R1) -> dealloc(R1) -> end
```

Process P3 (flow diagram):
```
start -> alloc(R1) -> dealloc(R1) -> alloc(R2) -> dealloc(R2) -> end
```

State S: all processes at start, no resources allocated.

**Analysis:**
If P2 takes the left branch:
- P1 needs R1 then R2; P2 needs R2 then R1.
- If P1 acquires R1 and P2 acquires R2, circular wait occurs (deadlock).
- P3 also needs R1 first, competing with P1.
- However, if we execute P3 first (acquires R1, releases, acquires R2, releases), then P1 (acquires R1, R2, releases both), then P2-left (acquires R2, R1, releases both), all complete.

If P2 takes the right branch:
- P2 only needs R1 briefly. Easy to schedule all processes.

Since for every choice of control flow for P2, there exists an execution order that completes all processes, state S is safe.

Answer: NO — the state is not unsafe (it is safe).
