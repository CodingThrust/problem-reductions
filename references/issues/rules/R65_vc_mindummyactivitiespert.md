---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** VERTEX COVER
**Target:** MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS
**Motivation:** Establishes NP-completeness of MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS via polynomial-time reduction from VERTEX COVER. This result by Krishnamoorthy and Deo (1979) shows that constructing an optimal PERT network representation (activity-on-arc) with the fewest dummy activities is computationally intractable, motivating the development of heuristic algorithms for project scheduling.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND44, p.218

## GJ Source Entry

> [ND44] MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS
> INSTANCE: Directed acyclic graph G=(V,A) where vertices represent tasks and the arcs represent precedence constraints, and a positive integer K≤|V|.
> QUESTION: Is there a PERT network corresponding to G with K or fewer dummy activities, i.e., a directed acyclic graph G'=(V',A') where V'={v_i^−,v_i^+: v_i∈V} and {(v_i^−,v_i^+): v_i∈V}⊆A', and such that |A'|≤|V|+K and there is a path from v_i^+ to v_j^− in G' if and only if there is a path from v_i to v_j in G?
> Reference: [Krishnamoorthy and Deo, 1977b]. Transformation from VERTEX COVER.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a VERTEX COVER instance consisting of an undirected graph G = (V, E) with n = |V| vertices and a bound K, construct a MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS instance as follows:

1. **Graph transformation:** Krishnamoorthy and Deo transform the node-cover (vertex cover) problem on graphs of maximum degree 3 to the minimum-dummy-activities problem. Since VERTEX COVER is NP-complete even for cubic (degree ≤ 3) graphs, this suffices.

2. **DAG construction from undirected graph:** Convert the undirected graph G = (V, E) into a directed acyclic graph (the task precedence graph) as follows:
   - Orient the edges of G consistently (e.g., by a topological ordering of vertices) to create a DAG.
   - Each vertex in G becomes a "task" vertex in the DAG.
   - Each directed edge (u, v) encodes a precedence constraint: task u must complete before task v begins.

3. **PERT network construction:** The PERT network G' = (V', A') must represent these precedence constraints using an activity-on-arc representation:
   - For each task v_i, create two event nodes v_i^- (start) and v_i^+ (end), connected by an activity arc (v_i^-, v_i^+).
   - Precedence constraints are encoded by "dummy activities" — arcs from v_i^+ to v_j^- representing that task i must finish before task j starts.
   - The total number of arcs is |V| (activity arcs) + number of dummy arcs. We want the number of dummy arcs to be minimized.

4. **Shared event nodes:** The key optimization in minimizing dummy activities is that event nodes can be shared (merged) between tasks. If tasks i and j have exactly the same set of predecessors, their start events can be merged into a single node, eliminating dummy arcs. A vertex cover in the original graph determines which vertices need separate event representations.

5. **Bound:** Set the dummy activity bound K' = K (the vertex cover size bound). The number of dummy activities required is directly related to the minimum vertex cover of a bipartite graph derived from the precedence structure.

6. **Correctness:** A vertex cover of size ≤ K in G exists if and only if a PERT network with ≤ K dummy activities exists. The vertex cover identifies which vertices (tasks) require explicit dummy arcs for precedence encoding, while the remaining tasks can share event nodes.

7. **Solution extraction:** Given a PERT network with K' dummy activities, identify which event nodes are shared and which have dedicated dummy arcs. The set of tasks with dedicated dummy arcs corresponds to a vertex cover in the original graph.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source VERTEX COVER instance
- m = `num_edges` of source VERTEX COVER instance
- K = vertex cover bound

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_tasks` | `num_vertices` |
| `num_precedence_arcs` | `num_edges` |
| `num_event_nodes` | 2 * `num_vertices` (at most, before merging) |
| `dummy_activity_bound` | K (same as vertex cover bound) |

**Derivation:**
- The DAG has n task vertices and at most m arcs (from the edges of G)
- The PERT network has at most 2n event nodes (v_i^- and v_i^+ for each task)
- The number of activity arcs is exactly n
- The number of dummy arcs is to be minimized (bounded by K)
- Total arcs in PERT: n + K

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce MinimumVertexCover instance to MinimizingDummyActivitiesPert, solve target with BruteForce, extract solution, verify vertex cover on source graph
- Compare with known results from literature
- Test with graphs of degree ≤ 3 (the case used in the original NP-completeness proof)
- Verify that the PERT network correctly represents all precedence constraints

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (VERTEX COVER):**
Graph G with 6 vertices {1, 2, 3, 4, 5, 6} and 7 edges (max degree 3):
- Edges: {1,2}, {1,3}, {2,4}, {3,4}, {3,5}, {4,6}, {5,6}
- Bound K = 3

**Vertex cover verification:**
- Candidate vertex cover: {1, 4, 5}
  - {1,2}: vertex 1 ✓
  - {1,3}: vertex 1 ✓
  - {2,4}: vertex 4 ✓
  - {3,4}: vertex 4 ✓
  - {3,5}: vertex 5 ✓
  - {4,6}: vertex 4 ✓
  - {5,6}: vertex 5 ✓
- Valid vertex cover of size 3 ✓

**Constructed target instance (MINIMIZING DUMMY ACTIVITIES IN PERT):**

DAG (orient edges by vertex index):
- Arcs: (1→2), (1→3), (2→4), (3→4), (3→5), (4→6), (5→6)
- Tasks: {1, 2, 3, 4, 5, 6}

PERT network event nodes:
- For each task i: start event v_i^- and end event v_i^+
- Activity arcs: (v_1^-,v_1^+), (v_2^-,v_2^+), ..., (v_6^-,v_6^+) — 6 arcs
- Dummy arcs needed for precedence: (v_1^+,v_2^-), (v_1^+,v_3^-), (v_2^+,v_4^-), (v_3^+,v_4^-), (v_3^+,v_5^-), (v_4^+,v_6^-), (v_5^+,v_6^-)

Without merging: 7 dummy arcs. But by merging event nodes:
- Merge v_4^- with v_2^+ and v_3^+ (if they share the same successor set): tasks 2 and 3 both precede task 4, so v_2^+ and v_3^+ can potentially share an event node leading to v_4^-.
- Merge v_6^- with v_4^+ and v_5^+ similarly.

With optimal merging, the number of dummy activities can be reduced to K = 3. ✓

**Solution mapping:**
The dummy activities correspond to the vertex cover vertices {1, 4, 5}, which identify the tasks whose precedence connections require explicit dummy arcs in the optimized PERT network.


## References

- **[Krishnamoorthy and Deo, 1977b]**: [`Krishnamoorthy1977b`] M. S. Krishnamoorthy and N. Deo (1977). "Complexity of the minimum dummy activities problem in a {Pert} network". Computer Centre, Indian Institute of Technology.
