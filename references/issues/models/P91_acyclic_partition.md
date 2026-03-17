---
name: Problem
about: Propose a new problem type
title: "[Model] AcyclicPartition"
labels: model
assignees: ''
---

## Motivation

ACYCLIC PARTITION (P91) from Garey & Johnson, A2 ND15. An NP-complete graph partitioning problem on directed graphs. Given a directed graph with vertex weights and arc costs, the problem asks for a partition of the vertices into bounded-weight groups such that the quotient graph (where groups become super-nodes with arcs between groups inheriting from original arcs) is acyclic, while also bounding the total inter-group arc cost. This problem arises in task scheduling, parallel computation, and automatic differentiation where preserving topological ordering across partitions is essential.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R36a: 3SAT -> ACYCLIC PARTITION (ND15)

## Definition

**Name:** <!-- ⚠️ Unverified --> `AcyclicPartition`
**Canonical name:** Acyclic Partition (also: Acyclic Graph Partitioning, DAG Partitioning)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND15

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), weight w(v) in Z+ for each v in V, cost c(a) in Z+ for each a in A, positive integers B and K.
QUESTION: Is there a partition of V into disjoint sets V1,V2,...,Vm such that the directed graph G' = (V',A'), where V' = {V1,V2,...,Vm}, and (Vi,Vj) in A' if and only if (vi,vj) in A for some vi in Vi and some vj in Vj, is acyclic, such that the sum of the weights of the vertices in each Vi does not exceed B, and such that the sum of the costs of all those arcs having their endpoints in different sets does not exceed K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V| variables (one per vertex)
- **Per-variable domain:** {1, 2, ..., m} where m <= n -- the partition index assigned to each vertex
- **Meaning:** variable x_v = i means vertex v is assigned to partition Vi. A valid assignment satisfies: (1) the quotient graph on {V1,...,Vm} is a DAG, (2) for each Vi, sum of w(v) for v in Vi <= B, and (3) the total cost of inter-partition arcs <= K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `AcyclicPartition`
**Variants:** graph topology (directed graph type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `DirectedGraph` | The directed graph G = (V, A) |
| `vertex_weights` | `Vec<i32>` | Weight w(v) for each vertex v |
| `arc_costs` | `Vec<i32>` | Cost c(a) for each arc a |
| `weight_bound` | `i32` | Maximum total vertex weight B per partition group |
| `cost_bound` | `i32` | Maximum total inter-partition arc cost K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Key getter methods: `num_vertices()` (= |V|), `num_arcs()` (= |A|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Garey and Johnson, 1979; transformation from 3SAT). NP-complete even for K=2 partitions and unit weights/costs.
- **Best known exact algorithm:** The general problem can be solved by exhaustive enumeration of all partitions in O*(n^n) time. For the unit-weight, unit-cost case with bounded number of parts k, ILP-based exact methods are used in practice. For the special case when G is a tree, the problem is NP-complete in the ordinary sense but solvable in pseudo-polynomial time (Lukes, 1974). When G is a tree with equal edge weights or equal vertex weights, it is polynomial-time solvable.
- **Parameterized complexity:** When the number of parts k is fixed, the problem is solvable in O(k^n * poly(n)) time by dynamic programming over subset assignments.
- **References:**
  - M.R. Garey, D.S. Johnson (1979). "Computers and Intractability." W.H. Freeman.
  - J.A. Lukes (1974). "Efficient Algorithm for the Partitioning of Trees." *IBM Journal of Research and Development*, 18(3):217-224.
  - B.W. Kernighan (1971). "Some graph partitioning problems related to program segmentation." PhD thesis, Princeton University.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), weight w(v) in Z+ for each v in V, cost c(a) in Z+ for each a in A, positive integers B and K.
QUESTION: Is there a partition of V into disjoint sets V1,V2,...,Vm such that the directed graph G' = (V',A'), where V' = {V1,V2,...,Vm}, and (Vi,Vj) in A' if and only if (vi,vj) in A for some vi in Vi and some vj in Vj, is acyclic, such that the sum of the weights of the vertices in each Vi does not exceed B, and such that the sum of the costs of all those arcs having their endpoints in different sets does not exceed K?

Reference: [Garey and Johnson, ----]. Transformation from X3C.
Comment: Remains NP-complete even if all v in V have w(v) = 1 and all a in A have c(a) = 1. Can be solved in polynomial time if G contains a Hamiltonian path (a property that can be verified in polynomial time for acyclic digraphs) [Kernighan, 1971]. If G is a tree the general problem is NP-complete in the ordinary sense, but can be solved in pseudo-polynomial time [Lukes, 1974]. The tree problem can be solved in polynomial time if all edge weights are equal (see [Hadlock, 1974]) or if all vertex weights are equal [Garey and Johnson, ----].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible partitions of V, check acyclicity of quotient graph, weight bounds, and cost bounds.
- [x] It can be solved by reducing to integer programming. Integer variable per vertex indicating partition assignment, with acyclicity enforced via topological ordering constraints.
- [x] Other: Multilevel heuristic algorithms (dagP tool), ILP formulations for exact solutions.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Directed graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 arcs:**
- Arcs: (0->1), (0->2), (1->3), (1->4), (2->4), (2->5), (3->5), (4->5)
- Vertex weights: w(0)=2, w(1)=3, w(2)=2, w(3)=1, w(4)=3, w(5)=1
- Arc costs: all c(a) = 1
- Weight bound B = 5, Cost bound K = 4

**Valid partition (m=3 parts):**
- V1 = {0, 2} (weight 2+2=4 <= 5)
- V2 = {1, 4} (weight 3+3=6 > 5) -- INVALID

**Revised partition:**
- V1 = {0} (weight 2 <= 5)
- V2 = {1, 2} (weight 3+2=5 <= 5)
- V3 = {3, 4, 5} (weight 1+3+1=5 <= 5)

Quotient graph edges: V1->V2 (from 0->1, 0->2), V2->V3 (from 1->3, 1->4, 2->4, 2->5). No back edges. Quotient is acyclic (V1->V2->V3).

Inter-partition arcs: (0->1), (0->2), (1->3), (1->4), (2->4), (2->5) = 6 arcs with total cost 6. But K=4, so this partition exceeds cost bound.

**Better partition:**
- V1 = {0, 1, 2} (weight 2+3+2=7 > 5) -- INVALID

**Working partition with B=5, K=5:**
- V1 = {0} (weight 2)
- V2 = {1, 2} (weight 5)
- V3 = {3, 4, 5} (weight 5)
- Inter-partition cost = 6 > K=5 -- still exceeds

With K=6: answer is YES. With K=4: answer is NO.
