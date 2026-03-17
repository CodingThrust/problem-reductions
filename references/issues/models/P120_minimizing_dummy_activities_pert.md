---
name: Problem
about: Propose a new problem type
title: "[Model] MinimizingDummyActivitiesPert"
labels: model
assignees: ''
---

## Motivation

MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS (P120) from Garey & Johnson, A2 ND44. A classical NP-complete problem arising in project management. When converting an activity-on-node (AON) project representation to an activity-on-arc (AOA) PERT network, dummy activities (arcs without real work) are needed to encode precedence constraints. Since network computation time is proportional to the number of arcs, minimizing dummy activities directly impacts project scheduling efficiency.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in current rule set.
- **As target:** R65 (VERTEX COVER to MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS)

## Definition

**Name:** <!-- ⚠️ Unverified --> `MinimumDummyActivitiesPert`
**Canonical name:** Minimizing Dummy Activities in PERT Networks (also: Minimum Dummy Arc Problem)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND44

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A) where vertices represent tasks and the arcs represent precedence constraints, and a positive integer K <= |V|.
QUESTION: Is there a PERT network corresponding to G with K or fewer dummy activities, i.e., a directed acyclic graph G' = (V',A') where V' = {v_i^-, v_i^+: v_i in V} and {(v_i^-, v_i^+): v_i in V} subset of A', and such that |A'| <= |V| + K and there is a path from v_i^+ to v_j^- in G' if and only if there is a path from v_i to v_j in G?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** The number of potential dummy arcs to decide upon. In the worst case, this is O(|V|^2) (any pair of event nodes could potentially have a dummy arc). The decision is which dummy arcs to include in the PERT network.
- **Per-variable domain:** binary {0, 1} — whether a potential dummy arc between event nodes is included.
- **Meaning:** The variable assignment encodes the set of dummy arcs in the PERT network G'. A valid solution must satisfy: (1) the total number of arcs |A'| <= |V| + K, and (2) the reachability relation is preserved: v_i^+ can reach v_j^- in G' if and only if v_i can reach v_j in G. The metric is `bool`: True if such a PERT network with <= K dummy arcs exists, False otherwise.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MinimumDummyActivitiesPert`
**Variants:** none (the input is always a DAG)

| Field | Type | Description |
|-------|------|-------------|
| `dag` | `DirectedAcyclicGraph` | The precedence DAG G = (V, A) where vertices are tasks |
| `dummy_bound` | `usize` | K — upper bound on the number of dummy activities allowed |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Alternatively, the optimization version minimizes K (the number of dummy activities), using `Metric = SolutionSize<i32>` with `Direction::Minimize`.
- Key getter methods: `num_tasks()` (= |V|), `num_precedence_arcs()` (= |A|), `dummy_bound()` (= K).
- The PERT network G' has event nodes (not task nodes). Each task v_i becomes an arc (v_i^-, v_i^+), and dummy arcs encode the precedence relations.
- The number of activity arcs is exactly |V| (one per task). Dummy arcs are the additional arcs beyond these |V| activity arcs.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Krishnamoorthy and Deo, 1979; transformation from VERTEX COVER on graphs of degree <= 3).
- **Best known exact algorithm:** Brute force enumeration of all possible event-node merging strategies and dummy arc placements. Exponential in |V| in the worst case.
- **Complexity string:** `"2^num_tasks"` (brute force over all possible PERT constructions)
- **Polynomial special cases:** Solvable in polynomial time for:
  - Interval orders (precedence is an interval order)
  - Two-dimensional partial orders
  - Series-parallel partial orders
- **References:**
  - M.S. Krishnamoorthy and N. Deo (1979). "Complexity of the minimum-dummy-activities problem in a PERT network." *Networks*, 9(3):189-194.
  - M.M. Syslo (1984). "On the computational complexity of the minimum-dummy-activities problem in a PERT network." *Networks*, 14(1):37-45. Alternative analysis and polynomial special cases.

## Extra Remark

**Full book text:**

INSTANCE: Directed acyclic graph G = (V,A) where vertices represent tasks and the arcs represent precedence constraints, and a positive integer K <= |V|.
QUESTION: Is there a PERT network corresponding to G with K or fewer dummy activities, i.e., a directed acyclic graph G' = (V',A') where V' = {v_i^-, v_i^+: v in V} and {(v_i^-, v_i^+): v_i in V} subset of A', and such that |A'| <= |V|+K and there is a path from v_i^+ to v_j^- in G' if and only if there is a path from v_i to v_j in G?
Reference: [Krishnamoorthy and Deo, 1977b]. Transformation from VERTEX COVER.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible dummy arc configurations (up to O(|V|^2) potential arcs) and verify the reachability constraint. Check whether the minimum number of dummy arcs <= K.
- [x] It can be solved by reducing to integer programming. Binary variable for each potential dummy arc, with reachability constraints (for every pair (i,j) with a path in G, ensure a path exists in G'; for every pair without a path in G, ensure no path in G'). Minimize the number of dummy arcs.
- [x] Other: Heuristic algorithms (e.g., Syslo's algorithm) that find near-optimal PERT networks in polynomial time for many practical instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance (6 tasks with precedence constraints):**

Precedence DAG G with 6 tasks {A, B, C, D, E, F} and 5 arcs:
- A -> C (task A must complete before C starts)
- A -> D
- B -> D
- B -> E
- C -> F

Bound: K = 3

**PERT network construction:**

Activity arcs (one per task): (A^-, A^+), (B^-, B^+), (C^-, C^+), (D^-, D^+), (E^-, E^+), (F^-, F^+) — 6 arcs.

Dummy arcs needed for precedence:
- A -> C: dummy arc (A^+, C^-)
- A -> D: dummy arc (A^+, D^-)
- B -> D: dummy arc (B^+, D^-)
- B -> E: dummy arc (B^+, E^-)
- C -> F: dummy arc (C^+, F^-)

Naive approach: 5 dummy arcs. But can we merge event nodes?

**Optimization:** Since tasks A and B both precede D, we can potentially merge D^- with a shared event node. But A also precedes C (which B does not), so A^+ cannot be fully merged with B^+.

With merging:
- Merge A^+ and C^- into a single event node e_1 (since A immediately precedes C). This eliminates the dummy arc (A^+, C^-). Now A^+ = C^-, and the arc (A^-, A^+) feeds directly into (C^-, C^+).
- Remaining dummy arcs: (A^+, D^-), (B^+, D^-), (B^+, E^-), (C^+, F^-) — 4 dummy arcs.
- Can merge D^- to receive from both A^+ and B^+ if D^- is a shared event. Arcs: (A^+ -> D^-) and (B^+ -> D^-) — 2 dummy arcs (cannot reduce further without violating precedence).
- Merge B^+ and E^- into a single event node. Eliminates dummy (B^+, E^-). Now 3 dummy arcs remain.

Total arcs: 6 activity + 3 dummy = 9 = |V| + 3 = 6 + 3. So K = 3 suffices.
Answer: YES ✓
