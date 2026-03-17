---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Graph 3-Colorability to 2-Dimensional Consecutive Sets"
labels: rule
assignees: ''
canonical_source_name: 'Graph 3-Colorability'
canonical_target_name: '2-Dimensional Consecutive Sets'
source_in_codebase: true
target_in_codebase: false
specialization_of: 'KColoring'
milestone: 'Garey & Johnson'
---

**Source:** Graph 3-Colorability
**Target:** 2-Dimensional Consecutive Sets
**Motivation:** Establishes NP-completeness of 2-DIMENSIONAL CONSECUTIVE SETS via polynomial-time reduction from GRAPH 3-COLORABILITY. The reduction encodes a graph coloring problem as a partition problem on an alphabet: each edge of the graph becomes a subset of size 3 in the collection, and finding a valid 3-coloring corresponds to partitioning the alphabet into groups where each edge-subset spans exactly 3 consecutive groups with at most one element per group.
<!-- Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230

## GJ Source Entry

> [SR19] 2-DIMENSIONAL CONSECUTIVE SETS
> INSTANCE: Finite alphabet Sigma, collection C = {Sigma_1, Sigma_2, ..., Sigma_n} of subsets of Sigma.
> QUESTION: Is there a partition of Sigma into disjoint sets X_1, X_2, ..., X_k such that each X_i has at most one element in common with each Sigma_j and such that, for each Sigma_j in C, there is an index l(j) such that Sigma_j is contained in
>
> X_{l(j)} union X_{l(j)+1} union ... union X_{l(j)+|Sigma_j|-1} ?
>
> Reference: [Lipsky, 1977b]. Transformation from GRAPH 3-COLORABILITY.
> Comment: Remains NP-complete if all Sigma_j in C have |Sigma_j| <= 5, but is solvable in polynomial time if all Sigma_j in C have |Sigma_j| <= 2.

## Reduction Algorithm

<!-- Unverified: AI-generated summary below -->

**Summary:**
Given a GRAPH 3-COLORABILITY instance G = (V, E), construct a 2-DIMENSIONAL CONSECUTIVE SETS instance as follows:

1. **Alphabet construction:** For each vertex v in V, create 3 copies: v_1, v_2, v_3 (one per color). Set Sigma = {v_c : v in V, c in {1,2,3}}. So |Sigma| = 3|V|.

2. **Subset construction:** For each edge {u, v} in E, create a subset:
   Sigma_{u,v} = {u_1, u_2, u_3, v_1, v_2, v_3}
   This is a subset of size 6 representing the constraint that u and v must receive different colors.

   Additionally, for each vertex v, create a subset:
   Sigma_v = {v_1, v_2, v_3}
   This is a subset of size 3 ensuring that the three copies of each vertex are spread across exactly 3 consecutive groups.

3. **Partition structure:** The intended partition has k = 3 groups corresponding to the 3 colors:
   - X_1 = {v_1 : v receives color 1}
   - X_2 = {v_2 : v receives color 2}
   - X_3 = {v_3 : v receives color 3}

   More precisely, a valid 3-coloring chi: V -> {1,2,3} yields a partition where v_{chi(v)} goes into X_{chi(v)}, and the other copies are distributed appropriately.

4. **Alternative (simpler) construction:** For each edge {u,v}, create a subset containing one symbol per vertex per color, constrained so that adjacent vertices cannot share a group. Specifically:
   - Alphabet: Sigma = V (one symbol per vertex), |Sigma| = |V|.
   - Subsets: For each edge {u,v} in E, define Sigma_{u,v} = {u, v} (a subset of size 2). But size-2 subsets make the problem polynomial per GJ comment. So we need subsets of size >= 3.

   The Lipsky (1977) construction likely augments each edge subset with auxiliary symbols to create subsets of size 3 (or more), encoding the coloring constraint:
   - For each edge {u,v}, add a unique dummy symbol d_{u,v} and define Sigma_{u,v} = {u, v, d_{u,v}}.
   - This forces u and v to be in different groups (since Sigma_{u,v} spans 3 consecutive groups with one element per group, u and v must be in different groups).

5. **Correctness (forward):** A valid 3-coloring chi partitions V into 3 color classes. Place all vertices of color c in group X_c. Place each dummy d_{u,v} in the group X_c where neither u nor v was assigned (since chi(u) != chi(v), there exists exactly one remaining color). Each Sigma_{u,v} = {u, v, d_{u,v}} spans 3 consecutive groups with one element in each. Each vertex subset Sigma_v spans 1 group (size 1, trivially consecutive).

6. **Correctness (reverse):** If a valid partition into k groups exists with the consecutiveness property, then for each edge subset {u, v, d_{u,v}} of size 3, the three elements must be in 3 distinct consecutive groups. This means u and v are in different groups, defining a proper coloring. Since we can map the groups to colors {1,2,3}, this gives a valid 3-coloring.

**Time complexity of reduction:** O(|V| + |E|) to construct the alphabet and subsets.

## Size Overhead

<!-- Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source Graph 3-Colorability instance (|V|)
- m = `num_edges` of source Graph 3-Colorability instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `alphabet_size` | `num_vertices + num_edges` |
| `num_subsets` | `num_edges` |
| `max_subset_size` | `3` |

**Derivation:** The alphabet has n vertex symbols plus m dummy symbols (one per edge), totaling n + m. Each edge produces one subset of size 3. The GJ comment says the problem remains NP-complete for subsets of size <= 5, consistent with this construction using size-3 subsets.

## Validation Method

<!-- Unverified: AI-suggested validation -->

- Closed-loop test: reduce a Graph3Colorability (KColoring with k=3) instance to TwoDimensionalConsecutiveSets, solve target with BruteForce, extract solution, verify on source.
- Test with known 3-colorable graph: K_3 (triangle) is 3-colorable. The 3 vertex symbols + 3 dummy symbols should admit a valid partition into 3 groups.
- Test with known non-3-colorable graph: K_4 (complete graph on 4 vertices) is not 3-colorable. Verify no valid partition exists.
- Test with bipartite graph (2-colorable, hence also 3-colorable): verify the partition works.
- Edge case: empty graph (always 3-colorable).

## Example

<!-- Unverified: AI-constructed example -->

**Source instance (Graph 3-Colorability):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {0,5}, {1,4}
- (This is a cycle C_6 plus a chord {1,4})
- 3-coloring: chi(0)=1, chi(1)=2, chi(2)=1, chi(3)=2, chi(4)=3, chi(5)=2

Verify: {0,1}: 1!=2 Y, {1,2}: 2!=1 Y, {2,3}: 1!=2 Y, {3,4}: 2!=3 Y, {4,5}: 3!=2 Y, {0,5}: 1!=2 Y, {1,4}: 2!=3 Y. Valid.

**Constructed target instance (TwoDimensionalConsecutiveSets):**
Alphabet: Sigma = {0, 1, 2, 3, 4, 5, d01, d12, d23, d34, d45, d05, d14} (13 symbols)
Subsets:
- Sigma_{0,1} = {0, 1, d01}
- Sigma_{1,2} = {1, 2, d12}
- Sigma_{2,3} = {2, 3, d23}
- Sigma_{3,4} = {3, 4, d34}
- Sigma_{4,5} = {4, 5, d45}
- Sigma_{0,5} = {0, 5, d05}
- Sigma_{1,4} = {1, 4, d14}

**Solution mapping:**
Partition into 3 groups (from coloring chi):
- X_1 = {0, 2} union {dummies assigned to color 1}
- X_2 = {1, 3, 5} union {dummies assigned to color 2}
- X_3 = {4} union {dummies assigned to color 3}

For each edge subset {u, v, d_{u,v}}, the dummy gets the remaining color:
- {0, 1, d01}: 0 in X_1, 1 in X_2, d01 in X_3. Spans X_1, X_2, X_3. One element per group. YES.
- {1, 2, d12}: 1 in X_2, 2 in X_1, d12 in X_3. Spans X_1, X_2, X_3. YES.
- {2, 3, d23}: 2 in X_1, 3 in X_2, d23 in X_3. Spans X_1, X_2, X_3. YES.
- {3, 4, d34}: 3 in X_2, 4 in X_3, d34 in X_1. Spans X_1, X_2, X_3. YES.
- {4, 5, d45}: 4 in X_3, 5 in X_2, d45 in X_1. Spans X_1, X_2, X_3. YES.
- {0, 5, d05}: 0 in X_1, 5 in X_2, d05 in X_3. Spans X_1, X_2, X_3. YES.
- {1, 4, d14}: 1 in X_2, 4 in X_3, d14 in X_1. Spans X_1, X_2, X_3. YES.

All subsets of size 3 span exactly 3 consecutive groups with one element per group.

Final partition:
- X_1 = {0, 2, d34, d45, d14}
- X_2 = {1, 3, 5}
- X_3 = {4, d01, d12, d23, d05}

**Verification:**
Each size-3 subset spans groups 1, 2, 3 (all consecutive) with at most one element per group. This is valid.


## References

- **[Lipsky, 1977b]**: [`Lipsky1977b`] William Lipsky, Jr (1977). "One more polynomial complete consecutive retrieval problem". *Information Processing Letters* 6, pp. 91-93.
