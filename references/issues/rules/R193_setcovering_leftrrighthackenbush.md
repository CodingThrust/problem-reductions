---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SET COVERING to LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE"
labels: rule
assignees: ''
canonical_source_name: 'Set Covering'
canonical_target_name: 'Left-Right Hackenbush for Redwood Furniture'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** SET COVERING
**Target:** LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE
**Motivation:** This reduction establishes that computing the game value of Left-Right Hackenbush on redwood furniture positions is NP-complete by reducing from Set Covering, demonstrating that even highly structured (restricted) Hackenbush positions encode the combinatorial difficulty of covering problems; as a consequence, determining a winner in arbitrary Left-Right Hackenbush is NP-hard.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.257

## GJ Source Entry

> [GP12] LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE
> INSTANCE: A piece of "redwood furniture," i.e., a connected graph G = (V,E) with a specified "ground" vertex v E V and a partition of the edges into sets L and R, where L is the set of all edges containing v (the set of "feet"), R = E - L, and each "foot" in L shares a vertex with at most one edge in R, which is its corresponding "leg" (not all edges in R need to be legs however), and a positive integer K.
> QUESTION: Is the "value" of the Left-Right Hackenbush game played on G less than or equal to 2^{-K} (see [Conway, 1976] for the definition of the game, there called Hackenbush Restrained, and for the definition of "value")?
> Reference: [Berlekamp, 1976]. Transformation from SET COVERING.
> Comment: Remains NP-complete even for "bipartite" redwood furniture, but can be solved in polynomial time for the subclass of redwood furniture known as "redwood trees." As a consequence of this result, the problem of determining if player 1 has a win in an arbitrary game of Left-Right Hackenbush is NP-hard.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

The reduction from Set Covering to Left-Right Hackenbush for Redwood Furniture is due to Berlekamp (1976). Given a Set Covering instance, construct a piece of redwood furniture such that the game value encodes whether a valid cover of bounded size exists.

**High-level approach:**
Given a Set Covering instance with universe S = {s_1, ..., s_n}, collection C = {C_1, ..., C_m} of subsets of S, and budget k:

1. **Ground vertex and feet:** Create a ground vertex v. For each element s_i in the universe S, create a blue "foot" edge (in set L) connecting v to a vertex u_i. These feet represent the elements that need to be covered.

2. **Legs and upper structure:** For each subset C_j in the collection, create red edges (in set R) forming a "leg" structure. Each foot u_i is connected via a red edge to subset vertices corresponding to the subsets containing element s_i. The structure ensures each foot shares a vertex with at most one leg edge.

3. **Redwood furniture constraint:** The construction satisfies the redwood furniture definition: blue edges (feet) touch the ground, red edges (legs and upper structure) do not touch the ground, and each foot shares a vertex with at most one leg.

4. **Weight encoding via structure:** The game value of the resulting Hackenbush position encodes the covering structure. The value is determined by which edges Left and Right can profitably remove:
   - Left (blue) removes feet, Right (red) removes legs/upper structure
   - The game value is small (at most 2^{-K}) if and only if the universe can be covered by at most k subsets

5. **Bound K:** Set K as a function of k (the covering budget) so that the game value threshold 2^{-K} correctly separates yes/no instances.

**Key invariant:** The game value of the constructed redwood furniture position is at most 2^{-K} if and only if there exists a set cover of size at most k for the original Set Covering instance.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_elements` (universe size) of source MinimumSetCovering
- m = `num_sets` of source MinimumSetCovering
- d = total number of element-set incidences (sum of subset sizes)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n + m + d) — ground + element vertices + subset vertices + incidence vertices |
| `num_edges` | O(n + d) — n feet (blue) + d incidence edges (red) |
| `num_left_edges` | `num_elements` — one blue foot per universe element |
| `num_right_edges` | O(d) — red legs encoding subset memberships |
| `bound_k` | Polynomial in the covering budget |

**Derivation:** Each universe element contributes one foot (blue edge from ground). Each subset membership contributes a red edge. The total graph size is polynomial in the Set Covering input.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a MinimumSetCovering instance, reduce to a Left-Right Hackenbush redwood furniture instance, compute the game value using Conway's surreal number theory, and verify the value is at most 2^{-K} iff a valid set cover of size k exists
- Test with known Set Covering instances (e.g., instances where the universe can be covered by 2 sets vs. requiring 3)
- Verify the constructed graph satisfies all redwood furniture constraints: L edges touch ground, R edges do not, each foot shares at most one vertex with a leg
- Verify the graph is connected

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumSetCovering):**
Universe S = {1, 2, 3, 4, 5, 6} (6 elements)
Collection C of 4 subsets:
- C_1 = {1, 2, 3}
- C_2 = {2, 4, 5}
- C_3 = {3, 5, 6}
- C_4 = {1, 4, 6}
Budget k = 2

Minimum set cover: {C_1, C_2, C_3} requires 3 subsets (no 2-subset cover exists that covers all 6 elements):
- C_1 ∪ C_2 = {1,2,3,4,5} -- misses 6
- C_1 ∪ C_3 = {1,2,3,5,6} -- misses 4
- C_1 ∪ C_4 = {1,2,3,4,6} -- misses 5
- C_2 ∪ C_3 = {2,3,4,5,6} -- misses 1
- C_2 ∪ C_4 = {1,2,4,5,6} -- misses 3
- C_3 ∪ C_4 = {1,3,4,5,6} -- misses 2

No set cover of size k = 2 exists. Answer: NO.

**Constructed target instance (Left-Right Hackenbush for Redwood Furniture):**
Graph G:
- Ground vertex v (the "floor")
- Element vertices: u_1, u_2, u_3, u_4, u_5, u_6
- Subset vertices: w_1, w_2, w_3, w_4
- Blue edges (set L = feet): {v, u_1}, {v, u_2}, {v, u_3}, {v, u_4}, {v, u_5}, {v, u_6}
- Red edges (set R): connections encoding membership:
  - {u_1, w_1}, {u_2, w_1}, {u_3, w_1} (elements in C_1)
  - {u_2, w_2}, {u_4, w_2}, {u_5, w_2} (elements in C_2)
  - {u_3, w_3}, {u_5, w_3}, {u_6, w_3} (elements in C_3)
  - {u_1, w_4}, {u_4, w_4}, {u_6, w_4} (elements in C_4)
- Bound K chosen such that game value <= 2^{-K} iff set cover of size 2 exists

Total: 11 vertices, 18 edges (6 blue + 12 red)

**Solution mapping:**
- Since no set cover of size k = 2 exists, the game value of this redwood furniture position is greater than 2^{-K}
- Answer to the Hackenbush question: NO (value > 2^{-K})
- This correctly reflects the Set Covering answer: NO


## References

- **[Conway, 1976]**: [`Conway1976`] J. H. Conway (1976). "On Numbers and Games". Academic Press, New York.
- **[Berlekamp, 1976]**: [`Berlekamp1976`] E. R. Berlekamp (1976). "".
