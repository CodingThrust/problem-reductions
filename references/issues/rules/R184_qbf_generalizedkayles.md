---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to GENERALIZED KAYLES"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formulas (QBF)'
canonical_target_name: 'Generalized Kayles'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** GENERALIZED KAYLES
**Motivation:** Establishes the PSPACE-completeness of Generalized Kayles (Node Kayles) by reducing from QBF, showing that determining the winner in vertex-removal games on graphs is as hard as evaluating quantified Boolean formulas, and founding the complexity-theoretic study of combinatorial games on graphs.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254

## GJ Source Entry

> [GP3] GENERALIZED KAYLES (*)
> INSTANCE: Graph G = (V,E).
> QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a vertex in the graph, removing that vertex and all vertices adjacent to it from the graph. Player 1 wins if and only if player 2 is the first player left with no vertices to choose from.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete. The variant in which G = (V_1 ∪ V_2, E) is bipartite, with each edge involving one vertex from V_1 and one from V_2, and player i can only choose vertices from the set V_i (but still removes all adjacent vertices as before) is also PSPACE-complete. For a description of the game Kayles upon which this generalization is based, see [Conway, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a QBF instance F = (Q_1 u_1)(Q_2 u_2)...(Q_n u_n)E, where E is a Boolean expression in CNF with clauses C_1, ..., C_m, construct a Generalized Kayles instance G = (V, E) as follows. The key idea is to encode the formula game as a vertex-removal game where choosing a vertex (and removing its neighbors) simulates setting truth values for variables.

1. **Variable gadgets:** For each variable u_i, create a pair of vertices T_i and F_i connected by an edge {T_i, F_i}. Choosing T_i removes F_i (and vice versa), simulating the assignment u_i = TRUE or u_i = FALSE. These are the "choice" vertices.

2. **Turn-control gadgets:** To ensure that the correct player (Player 1 for EXISTS, Player 2 for FORALL) makes the choice at each variable, insert chains of isolated vertices or paired vertices between variable gadgets. These force a specific number of moves before the next variable choice, ensuring the right player's turn at each diamond. If Q_i and Q_{i+1} require different players, a single spacer vertex suffices; if they require the same player, two spacer vertices are needed.

3. **Clause gadgets:** For each clause C_j, create a clause vertex c_j. Connect c_j to the literal vertices that make it true: edge {c_j, T_i} if u_i appears positively in C_j, edge {c_j, F_i} if NOT u_i appears in C_j.

4. **Parity and winning condition:** The total number of vertices and the game dynamics are arranged so that after all variable assignments are made (T_i or F_i chosen for each i, removing its partner), the remaining moves involve clause vertices. If the formula is satisfied, the parity of remaining playable vertices ensures Player 1 makes the last move (Player 2 is first to have no vertex to choose). If the formula is not satisfied, some clause vertex c_j remains along with its unsatisfied literal vertices, giving Player 2 extra moves to win.

5. **Correctness:** Player 1 has a forced win in the Kayles game if and only if the existential player has a winning strategy in the QBF formula game, i.e., F is true.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source QBF
- m = `num_clauses` of source QBF
- L = total number of literal occurrences across all clauses

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `2 * num_vars + num_vars + num_clauses` |
| `num_edges` | `num_vars + num_vars + L` |

**Derivation:** Each variable contributes 2 choice vertices (T_i, F_i). Up to n spacer vertices are needed for turn control between variables. There are m clause vertices. Edges include n variable-pair edges {T_i, F_i}, up to n spacer-connection edges, and L clause-to-literal edges (one per literal occurrence).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a small QBF instance, apply the reduction to produce a Generalized Kayles graph G, solve the game by exhaustive game-tree search (minimax over legal vertex choices with neighbor removal), and verify the game outcome matches the QBF truth value
- For a TRUE QBF, verify Player 1 has a forced win (Player 2 is the first player unable to move)
- For a FALSE QBF, verify Player 2 has a forced win
- Test with both TRUE and FALSE QBF instances to ensure bidirectional correctness
- Verify that removing a chosen vertex and its neighbors correctly simulates truth assignment

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
F = EXISTS u_1 FORALL u_2 EXISTS u_3 [(u_1 OR u_2 OR u_3) AND (NOT u_1 OR NOT u_2 OR u_3) AND (u_1 OR NOT u_2 OR NOT u_3)]

This has n = 3 variables and m = 3 clauses.
- C_1 = (u_1 OR u_2 OR u_3)
- C_2 = (NOT u_1 OR NOT u_2 OR u_3)
- C_3 = (u_1 OR NOT u_2 OR NOT u_3)

**Truth evaluation:** F is TRUE.
- Set u_1 = TRUE:
  - u_2 = TRUE: set u_3 = TRUE => C_1 = T, C_2 = (F OR F OR T) = T, C_3 = (T OR F OR F) = T.
  - u_2 = FALSE: set u_3 = FALSE => C_1 = (T OR F OR F) = T, C_2 = (F OR T OR F) = T, C_3 = (T OR T OR T) = T.

**Constructed target instance (Generalized Kayles):**
Graph G with vertices:
- Variable pairs: {T_1, F_1}, {T_2, F_2}, {T_3, F_3} (6 vertices)
- Spacer vertices: s_1 (between variables 1 and 2, to shift turn to Player 2), s_2 (between variables 2 and 3, to shift turn back to Player 1) (2 vertices)
- Clause vertices: c_1, c_2, c_3 (3 vertices)
- Total: 11 vertices

Edges:
- Variable pair edges: {T_1, F_1}, {T_2, F_2}, {T_3, F_3} (3 edges)
- Spacer connections: {s_1, s_2} or isolated spacers depending on parity needs (1-2 edges)
- Clause-literal edges:
  - C_1 (u_1 OR u_2 OR u_3): {c_1, T_1}, {c_1, T_2}, {c_1, T_3}
  - C_2 (NOT u_1 OR NOT u_2 OR u_3): {c_2, F_1}, {c_2, F_2}, {c_2, T_3}
  - C_3 (u_1 OR NOT u_2 OR NOT u_3): {c_3, T_1}, {c_3, F_2}, {c_3, F_3}
- Total clause-literal edges: 9

**Solution mapping:**
- Player 1 picks T_1 (removes T_1 and F_1, since they are adjacent). This sets u_1 = TRUE. Also removes any clause vertices adjacent to T_1 (c_1 and c_3 are adjacent to T_1, so they get removed — these clauses are satisfied by u_1 = TRUE).
- Spacer move(s) ensure Player 2 is next at variable 2.
- Player 2 picks T_2 or F_2 (say T_2, setting u_2 = TRUE). Removes T_2 and F_2. Clause c_2 is adjacent to F_2 and gets removed (satisfied by NOT u_2 being... wait, c_2 is connected to F_2 because NOT u_2 appears in C_2).
  - Actually, choosing T_2 removes T_2 and neighbors: F_2 (paired), and any clause vertices adjacent to T_2 (c_1 is connected to T_2, but c_1 was already removed).
- Player 1 picks T_3 (setting u_3 = TRUE). Removes T_3 and F_3.
- After all variable choices, remaining clause vertices (if any) determine the winner by parity. Since F is TRUE, Player 1 has a strategy to ensure Player 2 runs out of moves first.

The game outcome (Player 1 wins) matches F being TRUE.


## References

- **[Schaefer, 1978a]**: [`Schaefer1978a`] T. J. Schaefer (1978). "Complexity of some two-person perfect-information games". *Journal of Computer and System Sciences* 16, pp. 185–225.
- **[Conway, 1976]**: [`Conway1976`] J. H. Conway (1976). "On Numbers and Games". Academic Press, New York.
