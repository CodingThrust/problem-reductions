---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Ensemble Computation"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'ENSEMBLE COMPUTATION'
source_in_codebase: true
target_in_codebase: false
---

**Source:** VERTEX COVER
**Target:** ENSEMBLE COMPUTATION
**Motivation:** Establishes NP-completeness of ENSEMBLE COMPUTATION by encoding vertex-cover selection as a sequence of disjoint-union operations, where each "a₀-augmented" vertex z_i = {a₀} ∪ {v} corresponds to including vertex v in the cover and each edge subset is built by combining the appropriate cover vertex with its non-cover neighbor.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.6, p.66

## Reduction Algorithm

> ENSEMBLE COMPUTATION
> INSTANCE: A collection C of subsets of a finite set A and a positive integer J.
> QUESTION: Is there a sequence
>
> < z_1 = x_1 ∪ y_1, z_2 = x_2 ∪ y_2, . . . , z_j = x_j ∪ y_j >
>
> of j ≤ J union operations, where each x_i and y_i is either {a} for some a ∈ A or z_k for some k < i, such that x_i and y_i are disjoint for 1 ≤ i ≤ j and such that for every subset c ∈ C there is some z_i, 1 ≤ i ≤ j, that is identical to c ?
>
> Theorem 3.6 ENSEMBLE COMPUTATION is NP-complete.
> Proof: We transform VERTEX COVER to ENSEMBLE COMPUTATION. Let the graph G = (V,E) and the positive integer K ≤ |V| constitute an arbitrary instance of VC.
>
> The basic units of the instance of VC are the edges of G. Let a_0 be some new element not in V. The local replacement just substitutes for each edge {u,v} ∈ E the subset {a_0,u,v} ∈ C. The instance of ENSEMBLE COMPUTATION is completely specified by:
>
>     A = V ∪ {a_0}
>     C = {{a_0,u,v}: {u,v} ∈ E}
>     J = K + |E|
>
> It is easy to see that this instance can be constructed in polynomial time. We claim that G has a vertex cover of size K or less if and only if the desired sequence of j ≤ J operations exists for C.
>
> First, suppose V' is a vertex cover for G of size K or less. Since we can add additional vertices to V' and it will remain a vertex cover, there is no loss of generality in assuming that |V'| = K. Label the elements of V' as v_1,v_2, . . . , v_K and label the edges in E as e_1,e_2, . . . , e_m, where m = |E|. Since V' is a vertex cover, each edge e_j contains at least one element from V'. Thus we can write each e_j as e_j = {u_j,v_{r[j]}}, where r[j] is an integer satisfying 1 ≤ r[j] ≤ K. The following sequence of K + |E| = J operations is easily seen to have all the required properties:
>
>     < z_1 = {a_0} ∪ {v_1}, z_2 = {a_0} ∪ {v_2}, . . . , z_k = {a_0} ∪ {v_K},
>       z_{K+1} = {u_1} ∪ z_{r[1]}, z_{K+2} = {u_2} ∪ z_{r[2]}, . . . , z_J = {u_m} ∪ z_{r[m]} >
>
> Conversely, suppose S = < z_1 = x_1 ∪ y_1, . . . , z_j = x_j ∪ y_j > is the desired sequence of j ≤ J operations for the ENSEMBLE COMPUTATION instance. Furthermore, let us assume that S is the shortest such sequence for this instance and that, among all such minimum sequences, S contains the fewest possible operations of the form z_i = {u} ∪ {v} for u, v ∈ V. Our first claim is that S can contain no operations of this latter form. For suppose that z_i = {u} ∪ {v} with u,v ∈ V is included. Since {u,v} is not in C and since S has minimum length, we must have {u,v} ∈ E, and {a_0,u,v} = {a_0} ∪ z_i (or z_i ∪ {a_0}) must occur later in S. However, since {u,v} is a subset of only one member of C, z_i cannot be used in any other operation in this minimum length sequence. It follows that we can replace the two operations
>
>     z_i = {u} ∪ {v}  and  {a_0,u,v} = {a_0} ∪ z_i
>
> by
>
>     z_i = {a_0} ∪ {u}  and  {a_0,u,v} = {v} ∪ z_i
>
> thereby reducing the number of proscribed operations without lengthening the overall sequence, a contradiction to the choice of S. Hence S consists only of operations having one of the two forms, z_i = {a_0} ∪ {u} for u ∈ V or {a_0,u,v} = {v} ∪ z_i for {u,v} ∈ E (where we disregard the relative order of the two operands in each case). Because |C| = |E| and because every member of C contains three elements, S must contain exactly |E| operations of the latter form and exactly j−|E| ≤ J−|E| = K of the former. Therefore the set
>
>     V' = {u ∈ V: z_i = {a_0} ∪ {u} is an operation in S}
>
> contains at most K vertices from V and, as can be verified easily from the construction of C, must be a vertex cover for G. ∎

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G, K) where G = (V, E), construct an EnsembleComputation instance as follows:

1. **Universe construction:** Let a₀ be a fresh element not in V. Set A = V ∪ {a₀}. The universe has |V| + 1 elements.
2. **Collection construction:** For each edge {u, v} ∈ E, add the 3-element subset {a₀, u, v} to C. Each edge becomes exactly one required subset. The collection has |C| = |E| subsets.
3. **Budget parameter:** Set J = K + |E|. The K operations build "cover-vertex" sets z_i = {a₀} ∪ {v_i} (one per cover vertex), and the |E| operations build the required edge subsets by combining each non-cover neighbor {u_j} with the appropriate z_{r[j]}.
4. **Solution extraction:** Given a valid sequence of ≤ J operations, the cover vertices are exactly those vertices u ∈ V such that z_i = {a₀} ∪ {u} appears in the sequence. There are at most K such operations (since |E| operations are needed for edge subsets), giving a vertex cover of size ≤ K.

**Key invariant:** Every vertex cover of size K in G yields a valid ensemble sequence of exactly J = K + |E| operations; conversely, every minimum-length valid sequence of ≤ J operations (normalized to avoid {u} ∪ {v} form) encodes a vertex cover of size ≤ K.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `universe_size` | `num_vertices + 1` |
| `num_subsets` | `num_edges` |

**Derivation:**
- Universe A = V ∪ {a₀}: one element per vertex plus the fresh element a₀ → |A| = n + 1
- Collection C: one 3-element subset per edge in G → |C| = m
- Budget J = K + m is a parameter derived from the source instance, not a fixed overhead of target size

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance (G, K) to EnsembleComputation, solve the target with BruteForce (try all orderings/choices of union operations up to J steps), verify the sequence covers all required subsets in C
- Check that a YES answer for EnsembleComputation with budget J = K + |E| implies a vertex cover of size ≤ K exists (and vice versa)
- Test with a triangle graph (K₃) with K = 2: A = {a₀, 0, 1, 2}, C = {{a₀,0,1},{a₀,0,2},{a₀,1,2}}, J = 2 + 3 = 5; cover {0, 1} yields the sequence z₁ = {a₀,0}, z₂ = {a₀,1}, z₃ = {2}∪z₁ = {a₀,0,2}, z₄ = {2}∪z₂ = {a₀,1,2}, z₅ = {0}∪z₂ = {a₀,0,1} (j=5≤J✓)
- Test unsatisfiable case: path graph P₃ with K = 0 should yield no valid ensemble sequence of ≤ |E| = 2 operations

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 4 vertices {0, 1, 2, 3} and 4 edges:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}
- (A 4-cycle C₄)
- Minimum vertex cover: K = 2, for example V' = {0, 3} covers:
  - {0,1} by 0 ✓, {0,2} by 0 ✓, {1,3} by 3 ✓, {2,3} by 3 ✓

**Constructed target instance (EnsembleComputation):**
- Fresh element: a₀ (index 4)
- Universe A = {0, 1, 2, 3, 4} (where 4 = a₀), |A| = 5
- Collection C = {{4,0,1}, {4,0,2}, {4,1,3}, {4,2,3}} (|C| = 4 subsets, one per edge)
- Budget: J = K + |E| = 2 + 4 = 6

**Constructed sequence (from cover V' = {0, 3}):**

Label cover vertices v₁ = 0, v₂ = 3. Label edges e₁ = {1,0} (non-cover=1, cover=0), e₂ = {2,0} (non-cover=2, cover=0), e₃ = {1,3} (non-cover=1, cover=3), e₄ = {2,3} (non-cover=2, cover=3).

- z₁ = {a₀} ∪ {0} = {4, 0}  — "a₀-augmentation" of cover vertex 0
- z₂ = {a₀} ∪ {3} = {4, 3}  — "a₀-augmentation" of cover vertex 3
- z₃ = {1} ∪ z₁ = {1} ∪ {4,0} = {4,0,1} = c₁ ✓  — edge {0,1} produced
- z₄ = {2} ∪ z₁ = {2} ∪ {4,0} = {4,0,2} = c₂ ✓  — edge {0,2} produced
- z₅ = {1} ∪ z₂ = {1} ∪ {4,3} = {4,1,3} = c₃ ✓  — edge {1,3} produced
- z₆ = {2} ∪ z₂ = {2} ∪ {4,3} = {4,2,3} = c₄ ✓  — edge {2,3} produced

All 4 subsets in C appear as some z_i ✓, j = 6 = J ✓, all unions involve disjoint operands ✓

**Solution extraction:**
The operations of the form z_i = {a₀} ∪ {u} are z₁ (u=0) and z₂ (u=3). Thus V' = {0, 3} is the extracted vertex cover, |V'| = 2 = K ✓.
