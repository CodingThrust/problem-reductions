---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Clique to Minimum Tardiness Sequencing"
labels: rule
assignees: ''
---

**Source:** Clique
**Target:** Minimum Tardiness Sequencing
**Motivation:** (TBD)
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

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
