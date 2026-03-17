---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Clique to Minimum Tardiness Sequencing"
labels: rule
assignees: ''
canonical_source_name: 'CLIQUE'
canonical_target_name: 'MINIMUM TARDINESS SEQUENCING'
source_in_codebase: true
target_in_codebase: false
---

**Source:** CLIQUE
**Target:** MINIMUM TARDINESS SEQUENCING
**Motivation:** Establishes NP-completeness of MINIMUM TARDINESS SEQUENCING by encoding J-clique selection as a scheduling problem where meeting an early edge-task deadline forces exactly J vertex-tasks and J(J−1)/2 edge-tasks to be scheduled early — which is only possible if those tasks form a complete J-vertex subgraph.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.10, p.73

## Reduction Algorithm

> MINIMUM TARDINESS SEQUENCING
> INSTANCE: A set T of "tasks," each t ∈ T having "length" 1 and a "deadline" d(t) ∈ Z+, a partial order ≤ on T, and a non-negative integer K ≤ |T|.
> QUESTION: Is there a "schedule" σ: T → {0,1, . . . , |T|−1} such that σ(t) ≠ σ(t') whenever t ≠ t', such that σ(t) < σ(t') whenever t ≤ t', and such that |{t ∈ T: σ(t)+1 > d(t)}| ≤ K?
>
> Theorem 3.10 MINIMUM TARDINESS SEQUENCING is NP-complete.
> Proof: Let the graph G = (V,E) and the positive integer J ≤ |V| constitute an arbitrary instance of CLIQUE. The corresponding instance of MINIMUM TARDINESS SEQUENCING has task set T = V ∪ E, K = |E|−(J(J−1)/2), and partial order and deadlines defined as follows:
>
>     t ≤ t'  ⟺  t ∈ V, t' ∈ E, and vertex t is an endpoint of edge t'
>
>     d(t) = { J(J+1)/2    if t ∈ E
>             { |V|+|E|    if t ∈ V
>
> Thus the "component" corresponding to each vertex is a single task with deadline |V|+|E|, and the "component" corresponding to each edge is a single task with deadline J(J+1)/2. The task corresponding to an edge is forced by the partial order to occur after the tasks corresponding to its two endpoints in the desired schedule, and only edge tasks are in danger of being tardy (being completed after their deadlines).
>
> It is convenient to view the desired schedule schematically, as shown in Figure 3.10. We can think of the portion of the schedule before the edge task deadline as our "clique selection component." There is room for J(J+1)/2 tasks before this deadline. In order to have no more than the specified number of tardy tasks, at least J(J−1)/2 of these "early" tasks must be edge tasks. However, if an edge task precedes this deadline, then so must the vertex tasks corresponding to its endpoints. The minimum possible number of vertices that can be involved in J(J−1)/2 distinct edges is J (which can happen if and only if those edges form a complete graph on those J vertices). This implies that there must be at least J vertex tasks among the "early" tasks. However, there is room for at most
>
>     (J(J+1)/2) − (J(J−1)/2) = J
>
> vertex tasks before the edge task deadline. Therefore, any such schedule must have exactly J vertex tasks and exactly J(J−1)/2 edge tasks before this deadline, and these must correspond to a J-vertex clique in G. Conversely, if G contains a complete subgraph of size J, the desired schedule can be constructed as in Figure 3.10. ∎

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MaximumClique instance (G, J) where G = (V, E), construct a MinimumTardinessSequencing instance as follows:

1. **Task set:** Create one task t_v for each vertex v ∈ V and one task t_e for each edge e ∈ E. Thus |T| = |V| + |E|.
2. **Deadlines:** Set d(t_v) = |V| + |E| for all vertex tasks (very late, never tardy in practice) and d(t_e) = J(J+1)/2 for all edge tasks (an early "clique selection" deadline).
3. **Partial order:** For each edge e = {u, v} ∈ E, add precedence constraints t_u ≤ t_e and t_v ≤ t_e (both endpoints must be scheduled before the edge task).
4. **Tardiness bound:** Set K = |E| − J(J−1)/2. This is the maximum allowed number of tardy tasks (edge tasks that miss their early deadline).
5. **Solution extraction:** In any valid schedule with ≤ K tardy tasks, at least J(J−1)/2 edge tasks must be scheduled before time J(J+1)/2. The precedence constraints force their endpoints (vertex tasks) to also be early. A counting argument shows exactly J vertex tasks and J(J−1)/2 edge tasks are early, and those edges must form a complete subgraph on those J vertices — a J-clique in G.

**Key invariant:** G has a J-clique if and only if T has a valid schedule (respecting partial order) with at most K = |E| − J(J−1)/2 tardy tasks.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G
- J = clique size parameter from source instance

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_tasks` | `num_vertices + num_edges` |

**Derivation:**
- One task per vertex in G plus one task per edge in G → |T| = n + m
- The partial order has exactly 2·m precedence pairs (two vertex tasks per edge task)
- K = m − J(J−1)/2 is derived from the source instance parameters; the maximum possible K (when J=1) is m − 0 = m, and minimum K (when J=|V|) is m − |V|(|V|−1)/2 which may be 0 if G is complete

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MaximumClique instance (G, J) to MinimumTardinessSequencing, solve the target with BruteForce (try all permutations σ respecting the partial order), check whether any valid schedule has at most K tardy tasks
- Verify the counting argument: in a satisfying schedule, identify the J vertex-tasks and J(J−1)/2 edge-tasks scheduled before time J(J+1)/2, confirm the corresponding subgraph is a complete graph on J vertices
- Test with K₄ (complete graph on 4 vertices) and J = 3: should find a valid schedule (any 3-clique works)
- Test with a triangle-free graph (e.g., C₅) and J = 3: should find no valid schedule since no 3-clique exists
- Verify the partial order is respected in all candidate schedules by checking that every edge task is scheduled after both its endpoint vertex tasks

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MaximumClique):**
Graph G with 4 vertices {0, 1, 2, 3} and 5 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,3}
- (K₄ minus the edge {0,3}: vertices 0,1,2 form a triangle, plus vertex 3 connected to 1 and 2)
- G contains a 3-clique: {0, 1, 2} (edges {0,1}, {0,2}, {1,2} all present)
- Clique parameter: J = 3

**Constructed target instance (MinimumTardinessSequencing):**

Tasks (|V| + |E| = 4 + 5 = 9 total):
- Vertex tasks: t₀, t₁, t₂, t₃ (deadlines d = |V| + |E| = 9)
- Edge tasks: t₀₁, t₀₂, t₁₂, t₁₃, t₂₃ (deadlines d = J(J+1)/2 = 3·4/2 = 6)

Partial order (endpoints must precede edge task):
- t₀ ≤ t₀₁, t₁ ≤ t₀₁
- t₀ ≤ t₀₂, t₂ ≤ t₀₂
- t₁ ≤ t₁₂, t₂ ≤ t₁₂
- t₁ ≤ t₁₃, t₃ ≤ t₁₃
- t₂ ≤ t₂₃, t₃ ≤ t₂₃

Tardiness bound: K = |E| − J(J−1)/2 = 5 − 3·2/2 = 5 − 3 = 2

**Constructed schedule (from clique {0, 1, 2}):**

Early portion (positions 0–5, before deadline 6 for edge tasks):

Schedule σ:
- σ(t₀) = 0 (position 0, finishes at 1 ≤ d=9 ✓)
- σ(t₁) = 1 (position 1, finishes at 2 ≤ d=9 ✓)
- σ(t₂) = 2 (position 2, finishes at 3 ≤ d=9 ✓)
- σ(t₀₁) = 3 (finishes at 4 ≤ d=6 ✓, not tardy — endpoints t₀,t₁ scheduled earlier ✓)
- σ(t₀₂) = 4 (finishes at 5 ≤ d=6 ✓, not tardy — endpoints t₀,t₂ scheduled earlier ✓)
- σ(t₁₂) = 5 (finishes at 6 ≤ d=6 ✓, not tardy — endpoints t₁,t₂ scheduled earlier ✓)

Late portion (positions 6–8, after deadline 6 for edge tasks):
- σ(t₃) = 6 (finishes at 7 ≤ d=9 ✓, not tardy)
- σ(t₁₃) = 7 (finishes at 8 > d=6 — TARDY ✗)
- σ(t₂₃) = 8 (finishes at 9 > d=6 — TARDY ✗)

Tardy tasks: {t₁₃, t₂₃}, count = 2 ≤ K = 2 ✓
Partial order respected: all vertex tasks precede their edge tasks ✓

**Solution extraction:**
The J(J−1)/2 = 3 edge tasks scheduled before deadline 6 are t₀₁, t₀₂, t₁₂. Their endpoint vertex tasks are {t₀, t₁, t₂}. These correspond to vertices {0, 1, 2} forming a triangle (complete subgraph) in G — a 3-clique ✓.
