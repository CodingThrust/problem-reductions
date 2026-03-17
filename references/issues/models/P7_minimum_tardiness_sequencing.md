---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumTardinessSequencing"
labels: model
assignees: ''
---

## Motivation

MINIMUM TARDINESS SEQUENCING (P7) from Garey & Johnson, Chapter 3, Section 3.2.3, p.73. A classical NP-complete single-machine scheduling problem where unit-length tasks with precedence constraints and deadlines must be scheduled to minimize the number of tardy tasks (tasks that finish after their deadline). Corresponds to the scheduling notation 1|prec, pⱼ=1|∑Uⱼ.

## Definition

**Name:** `MinimumTardinessSequencing`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.2.3, p.73

**Mathematical definition:**

INSTANCE: A set T of "tasks," each t ∈ T having "length" 1 and a "deadline" d(t) ∈ Z+, a partial order ≤ on T, and a non-negative integer K ≤ |T|.
QUESTION: Is there a "schedule" σ: T → {0,1,...,|T|−1} such that σ(t) ≠ σ(t') whenever t ≠ t', such that σ(t) < σ(t') whenever t ≤ t', and such that |{t ∈ T: σ(t)+1 > d(t)}| ≤ K?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->
- **Count:** |T| variables, one per task.
- **Per-variable domain:** Each task t is assigned a position σ(t) ∈ {0, 1, ..., |T|−1}; the assignment must be a bijection and must respect the partial order.
- **Meaning:** σ(t) is the 0-indexed start time (= finish time − 1) of task t. A task t is tardy if σ(t) + 1 > d(t), i.e., it finishes at time σ(t) + 1 which exceeds its deadline. The problem asks whether a valid ordering (permutation respecting the precedence DAG) exists with at most K tardy tasks.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->
**Type name:** `MinimumTardinessSequencing`
**Variants:** No generic parameters needed; a single concrete decision variant.

| Field | Type | Description |
|-------|------|-------------|
| `num_tasks` | `usize` | Number of tasks \|T\| |
| `deadlines` | `Vec<usize>` | Deadline d(t) for each task t (1-indexed: task finishes at position+1) |
| `precedences` | `Vec<(usize, usize)>` | List of (predecessor, successor) pairs encoding the partial order on T |
| `max_tardy` | `usize` | Bound K: at most K tasks may be tardy |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->
- **Best known exact algorithm:** MINIMUM TARDINESS SEQUENCING (equivalently 1|prec, pⱼ=1|∑Uⱼ) is NP-complete (Garey & Johnson, Theorem 3.10). No polynomial-time algorithm is known for the general case with arbitrary precedence constraints. The best exact approach is branch-and-bound or dynamic programming over permutations; the naive brute-force over all topological sorts runs in O(|T|!) time. More refined exact algorithms (e.g., branch-and-bound with dominance rules) can solve practical instances of moderate size (up to ~50–100 tasks) but have worst-case exponential complexity. The problem remains NP-complete even when the partial order consists only of chains (Lenstra, 1977).

## Extra Remark

**Full book text:**

INSTANCE: A set T of "tasks," each t ∈ T having "length" 1 and a "deadline" d(t) ∈ Z+, a partial order < on T, and a non-negative integer K ≤ |T|.
QUESTION: Is there a "schedule" σ: T → {0,1,...,|T|−1} such that σ(t) ≠ σ(t') whenever t ≠ t', such that σ(t) < σ(t') whenever t < t', and such that |{t ∈ T: σ(t)+1 > d(t)}| ≤ K?

Reference: [Garey and Johnson, 1976c]. Transformation from CLIQUE (see Section 3.2.3).

Comment: Remains NP-complete even if all task lengths are 1 and < consists only of "chains" (each task has at most one immediate predecessor and at most one immediate successor) [Lenstra, 1977]. The general problem can be solved in polynomial time if K = 0 [Lawler, 1973], or if < is empty [Moore, 1968] [Sidney, 1973]. The < empty case remains polynomially solvable if "agreeable" release times (i.e., r(t) < r(t') implies d(t) ≤ d(t')) are added [Kise, Ibaraki, and Mine, 1978], but is NP-complete for arbitrary release times (see previous problem).

## How to solve

- [x] It can be solved by (existing) bruteforce: enumerate all topological sorts of T (permutations respecting the partial order), count tardy tasks in each, check if any has ≤ K tardy tasks.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: N/A — no other known tractable special case applies to the general precedence-constrained variant

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Small satisfiable instance (YES answer):**

Tasks: T = {t₀, t₁, t₂, t₃, t₄} (5 tasks, unit length each)
Deadlines:
- d(t₀) = 5, d(t₁) = 5, d(t₂) = 5   (vertex-like tasks, very late deadline)
- d(t₃) = 3, d(t₄) = 3               (edge-like tasks, early deadline)

Partial order (precedences):
- t₀ ≤ t₃ (t₀ must precede t₃)
- t₁ ≤ t₃ (t₁ must precede t₃)
- t₁ ≤ t₄ (t₁ must precede t₄)
- t₂ ≤ t₄ (t₂ must precede t₄)

(Corresponds to triangle graph on {t₀,t₁,t₂} with edges e₁={t₀,t₁}→t₃ and e₂={t₁,t₂}→t₄, but e₃={t₀,t₂} missing — not a complete triangle)

Max tardy: K = 1

**Valid schedule σ:**
- σ(t₀) = 0 (finish at 1 ≤ d=5 ✓)
- σ(t₁) = 1 (finish at 2 ≤ d=5 ✓)
- σ(t₃) = 2 (finish at 3 ≤ d=3 ✓ — not tardy; t₀ and t₁ scheduled earlier ✓)
- σ(t₂) = 3 (finish at 4 ≤ d=5 ✓)
- σ(t₄) = 4 (finish at 5 > d=3 — TARDY)

Tardy tasks: {t₄}, count = 1 ≤ K = 1 ✓. Schedule is valid (bijection ✓, partial order respected ✓).

**Small unsatisfiable instance (NO answer):**

Same tasks and precedences, but K = 0 (no tardy tasks allowed).

For K = 0, both t₃ and t₄ must finish by time 3. Since t₃ requires t₀ and t₁ before it, and t₄ requires t₁ and t₂ before it, we need at least 3 vertex tasks before position 2 (to allow both edge tasks at positions 2 and 3, finishing at 3). But 3 vertex tasks need positions 0, 1, 2, leaving no position ≤ 1 for both edge tasks to finish by time 3. The earliest both edge tasks can finish on time requires t₀, t₁, t₂ all in positions 0–1 (only 2 slots), which is impossible for 3 tasks. Hence K = 0 is not achievable: this is a NO instance.
