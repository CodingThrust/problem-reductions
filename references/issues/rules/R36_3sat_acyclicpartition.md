---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to ACYCLIC PARTITION"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'ACYCLIC PARTITION'
source_in_codebase: true
target_in_codebase: false
---

**Source:** 3SAT
**Target:** ACYCLIC PARTITION
**Motivation:** Establishes NP-completeness of ACYCLIC PARTITION via polynomial-time reduction from 3SAT. The reduction encodes truth assignments as partition choices in a directed graph, using the acyclicity constraint to force consistency and clause satisfaction. This shows that partitioning a directed graph into bounded-weight acyclic groups is intractable even with just 2 groups and unit weights.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND15, p.210

## GJ Source Entry

> [ND15] ACYCLIC PARTITION
> INSTANCE: Directed graph G=(V,A), positive integer K.
> QUESTION: Can V be partitioned into K disjoint sets V_1,...,V_K such that the subgraph of G induced by each V_i is acyclic?
> Reference: [Garey and Johnson, 1979]. Transformation from 3SAT.
> Comment: NP-complete even for K=2.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a KSatisfiability<K3> instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}, construct an AcyclicPartition instance (G = (V, A), K = 2) as follows:

1. **Variable gadgets:** For each variable u_i, create a directed cycle of length 3 on vertices {v_i, v_i', v_i''}. The arcs are (v_i -> v_i'), (v_i' -> v_i''), (v_i'' -> v_i). In any partition of V into two sets where each induced subgraph is acyclic, at least one arc of this 3-cycle must cross between the two sets -- meaning at least one vertex from each 3-cycle must be in a different partition set. This encodes the binary truth assignment: if v_i is in V_1, interpret u_i = True; if v_i is in V_2, interpret u_i = False.

2. **Clause gadgets:** For each clause c_j = (l_1 OR l_2 OR l_3) where each l_k is a literal (u_i or NOT u_i), create a directed 3-cycle on fresh clause vertices {a_j, b_j, c_j_vertex}. The arcs are (a_j -> b_j), (b_j -> c_j_vertex), (c_j_vertex -> a_j).

3. **Connection arcs (literal to clause):** For each literal l_k in clause c_j, add a pair of arcs connecting the variable gadget vertex corresponding to l_k to the clause gadget. Specifically:
   - If l_k = u_i (positive literal): add arcs (v_i -> a_j) and (a_j -> v_i) creating a 2-cycle that forces v_i and a_j into different partition sets, or alternatively add directed paths that propagate the partition assignment.
   - If l_k = NOT u_i (negated literal): the connection is made to the complementary vertex in the variable gadget.

   The connection structure ensures that if all three literals of a clause are false (i.e., all corresponding variable vertices are on the same side as the clause gadget), the clause gadget together with the connections forms a directed cycle entirely within one partition set, violating the acyclicity constraint.

4. **Partition parameter:** K = 2.

5. **Solution extraction:** Given a valid 2-partition (V_1, V_2) where both induced subgraphs are acyclic, read off the truth assignment from which partition set each variable vertex v_i belongs to. The acyclicity constraint on the clause gadgets guarantees that each clause has at least one satisfied literal.

**Note:** The GJ entry references this as a transformation from 3SAT (or equivalently X3C in some printings). The key insight is that directed cycles of length 3 within each partition set are forbidden, so the partition must "break" every 3-cycle by placing at least one vertex on each side. The clause gadgets are designed so that a clause is satisfied if and only if its 3-cycle can be broken by the partition implied by the truth assignment.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `3 * num_vars + 3 * num_clauses` |
| `num_arcs` | `3 * num_vars + 3 * num_clauses + 6 * num_clauses` |

**Derivation:**
- Vertices: 3 per variable gadget (3-cycle) + 3 per clause gadget (3-cycle) = 3n + 3m
- Arcs: 3 per variable cycle + 3 per clause cycle + 2 connection arcs per literal (3 literals per clause, so 6 per clause) = 3n + 3m + 6m = 3n + 9m

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a KSatisfiability<K3> instance to AcyclicPartition, solve target with BruteForce (enumerate all 2-partitions, check acyclicity of each induced subgraph), extract truth assignment from partition, verify it satisfies all clauses
- Test with both satisfiable and unsatisfiable 3SAT instances to verify bidirectional correctness
- Verify that for K=2, the constructed graph has a valid acyclic 2-partition iff the 3SAT instance is satisfiable
- Check vertex and arc counts match the overhead formulas

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (KSatisfiability<K3>):**
3 variables: u_1, u_2, u_3 (n = 3)
2 clauses (m = 2):
- c_1 = (u_1 OR u_2 OR NOT u_3)
- c_2 = (NOT u_1 OR u_2 OR u_3)

**Constructed target instance (AcyclicPartition):**

Vertices (3n + 3m = 9 + 6 = 15 total):
- Variable gadget for u_1: {v_1, v_1', v_1''} with cycle (v_1 -> v_1' -> v_1'' -> v_1)
- Variable gadget for u_2: {v_2, v_2', v_2''} with cycle (v_2 -> v_2' -> v_2'' -> v_2)
- Variable gadget for u_3: {v_3, v_3', v_3''} with cycle (v_3 -> v_3' -> v_3'' -> v_3)
- Clause gadget for c_1: {a_1, b_1, d_1} with cycle (a_1 -> b_1 -> d_1 -> a_1)
- Clause gadget for c_2: {a_2, b_2, d_2} with cycle (a_2 -> b_2 -> d_2 -> a_2)

Connection arcs (linking literals to clause gadgets):
- c_1 literal u_1 (positive): arcs connecting v_1 to clause-1 gadget
- c_1 literal u_2 (positive): arcs connecting v_2 to clause-1 gadget
- c_1 literal NOT u_3 (negative): arcs connecting v_3' to clause-1 gadget
- c_2 literal NOT u_1 (negative): arcs connecting v_1' to clause-2 gadget
- c_2 literal u_2 (positive): arcs connecting v_2 to clause-2 gadget
- c_2 literal u_3 (positive): arcs connecting v_3 to clause-2 gadget

Partition parameter: K = 2

**Solution mapping:**
- Satisfying assignment: u_1 = True, u_2 = True, u_3 = True
- Partition V_1 (True side): {v_1, v_2, v_3} plus clause vertices as needed
- Partition V_2 (False side): {v_1', v_1'', v_2', v_2'', v_3', v_3''} plus remaining clause vertices
- Each variable 3-cycle is split across V_1 and V_2, so no complete cycle in either induced subgraph
- Each clause has at least one true literal, so clause gadget cycles are also properly split
- Both induced subgraphs are acyclic


## References

- **[Garey and Johnson, 1979]**: [`Garey19xx`] M. R. Garey and D. S. Johnson (1979). "Unpublished results".
