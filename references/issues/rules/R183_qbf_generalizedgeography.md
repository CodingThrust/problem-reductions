---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to GENERALIZED GEOGRAPHY"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formulas (QBF)'
canonical_target_name: 'Generalized Geography'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** GENERALIZED GEOGRAPHY
**Motivation:** Establishes the PSPACE-completeness of Generalized Geography by reducing from QBF, connecting the complexity of quantified Boolean reasoning to combinatorial game theory and demonstrating that determining the winner in move-based graph traversal games is as hard as evaluating quantified formulas.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254

## GJ Source Entry

> [GP2] GENERALIZED GEOGRAPHY (*)
> INSTANCE: Directed graph G = (V,A) and a specified vertex v_0 E V.
> QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new arc from A. The first arc chosen must have its tail at v_0 and each subsequently chosen arc must have its tail at the vertex that was the head of the previous arc. The first player unable to choose such a new arc loses.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete, even if G is bipartite, planar, and has no in- or out-degree exceeding 2 and no degree exceeding 3 (PLANAR GEOGRAPHY) [Lichtenstein and Sipser, 1978]. This game is a generalization of the "Geography" game in which players alternate choosing countries, each name beginning with the same letter that ends the previous country's name.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a QBF instance (FORMULA-GAME) F = EXISTS x_1 FORALL x_2 EXISTS x_3 ... Q_n x_n (E), where E is in CNF with clauses C_1, ..., C_m, construct a Generalized Geography instance (G, v_0) on a directed graph as follows:

1. **Variable diamonds:** For each variable x_i (i = 1, ..., n), create a diamond-shaped subgraph with four vertices: a top vertex d_i, a TRUE vertex T_i (left branch), a FALSE vertex F_i (right branch), and a bottom vertex b_i. Add arcs (d_i, T_i), (d_i, F_i), (T_i, b_i), (F_i, b_i). The diamonds are chained in series: add arc (b_i, d_{i+1}) for i = 1, ..., n-1.

2. **Quantifier encoding via turn alternation:** The starting vertex v_0 is d_1. Player 1 moves first at d_1, choosing T_1 or F_1 (corresponding to setting x_1 = TRUE or FALSE). Since Q_1 = EXISTS, this is correct. The forced moves through b_i and d_{i+1} ensure that the player whose turn it is at each diamond d_i matches the quantifier Q_i: if Q_i = EXISTS, Player 1 chooses; if Q_i = FORALL, Player 2 chooses. Parity-adjusting vertices are inserted between diamonds as needed to ensure the correct player acts.

3. **Clause gadgets:** After the last diamond, create a clause-checking structure. From b_n, add an arc to a clause selector vertex s. From s, add arcs to clause vertices c_1, ..., c_m (one per clause). From each clause vertex c_j, add arcs back to the truth-value vertices of literals appearing in that clause: arc (c_j, T_i) if literal x_i appears in C_j, arc (c_j, F_i) if literal NOT x_i appears in C_j.

4. **Game play after variable assignment:** After all variables are assigned (all diamonds traversed), the token reaches b_n, then s. Player 2 selects a clause c_j to challenge. Player 1 must then move from c_j to a literal vertex that was already visited (the "true" branch of some diamond). If the literal is true under the assignment, that vertex was already visited, and the opponent is stuck (cannot revisit it), so Player 1 wins. If all literals in c_j are false, the corresponding vertices were not visited along the main path, and Player 2 can continue the game to win.

5. **Winning condition:** Player 1 has a winning strategy in the Geography game starting at v_0 if and only if the QBF F is true.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source QBF
- m = `num_clauses` of source QBF
- L = total number of literal occurrences across all clauses

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `4 * num_vars + num_vars + num_clauses + 2` |
| `num_arcs` | `4 * num_vars + num_vars + num_clauses + L` |

**Derivation:** Each variable contributes 4 diamond vertices (d_i, T_i, F_i, b_i). Up to n parity-adjusting vertices are needed between diamonds. There are m clause vertices plus the clause selector vertex s and starting vertex v_0 (= d_1, already counted). Arcs include 4 per diamond (internal), chain connections between diamonds (n-1 + adjustments ~ n), arcs from s to each clause (m), and literal-back-edges from clause vertices to literal vertices (L total).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a small QBF instance, apply the reduction to produce a Generalized Geography directed graph (G, v_0), solve the game by exhaustive game-tree search (minimax with arc deletion), and verify the game outcome matches the QBF truth value
- For a TRUE QBF, verify Player 1 has a winning strategy from v_0
- For a FALSE QBF, verify Player 2 has a winning strategy (Player 1 cannot avoid losing)
- Verify constructed graph sizes match the overhead formulas
- Check that the diamond structure correctly assigns variable choices to the correct player based on quantifier type

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
F = EXISTS x_1 FORALL x_2 EXISTS x_3 [(x_1 OR x_2 OR x_3) AND (NOT x_1 OR NOT x_2) AND (x_2 OR NOT x_3)]

This has n = 3 variables and m = 3 clauses.
- C_1 = (x_1 OR x_2 OR x_3)
- C_2 = (NOT x_1 OR NOT x_2)
- C_3 = (x_2 OR NOT x_3)

**Truth evaluation:** F is TRUE. Existential player sets x_1 = TRUE.
- If x_2 = TRUE: set x_3 = FALSE => C_1 = T (T), C_2 = T (F OR F, wait: NOT TRUE OR NOT TRUE = F OR F = F). Hmm, let's reconsider.
- Actually with x_1 = TRUE: C_2 = (NOT TRUE OR NOT x_2) = (F OR NOT x_2). For C_2 to be satisfied, need x_2 = FALSE.
- Let x_1 = FALSE instead. C_1 = (F OR x_2 OR x_3), C_2 = (T OR NOT x_2) = T, C_3 = (x_2 OR NOT x_3).
  - If x_2 = TRUE: set x_3 = TRUE => C_1 = T, C_2 = T, C_3 = T. All satisfied.
  - If x_2 = FALSE: set x_3 = TRUE => C_1 = T (F OR F OR T), C_2 = T, C_3 = (F OR F) = F. Not satisfied.
  - If x_2 = FALSE: set x_3 = FALSE => C_1 = F, not satisfied.
- Let x_1 = TRUE. C_2 = (F OR NOT x_2). Need x_2 = FALSE for C_2.
  - If x_2 = FALSE: C_3 = (F OR NOT x_3). Need x_3 = FALSE. C_1 = (T OR F OR F) = T. All satisfied.
  - If x_2 = TRUE: C_2 = F. Need to handle this. Set x_3 = TRUE => C_1 = T, C_2 = F. Fails.

So existential strategy: x_1 = TRUE, then if x_2 = FALSE, set x_3 = FALSE (all clauses T). If x_2 = TRUE, C_2 fails, so universal wins. This means F is FALSE under "for all x_2" since x_2 = TRUE breaks it.

Let us use a correct TRUE instance:
F = EXISTS x_1 FORALL x_2 EXISTS x_3 [(x_1 OR x_2 OR x_3) AND (NOT x_1 OR NOT x_2 OR x_3) AND (x_1 OR NOT x_2 OR NOT x_3)]

- C_1 = (x_1 OR x_2 OR x_3), C_2 = (NOT x_1 OR NOT x_2 OR x_3), C_3 = (x_1 OR NOT x_2 OR NOT x_3)
- Set x_1 = TRUE:
  - x_2 = TRUE: set x_3 = TRUE => C_1 = T, C_2 = (F OR F OR T) = T, C_3 = (T OR F OR F) = T. All satisfied.
  - x_2 = FALSE: set x_3 = FALSE => C_1 = (T OR F OR F) = T, C_2 = (F OR T OR F) = T, C_3 = (T OR T OR T) = T. All satisfied.
- F is TRUE.

**Constructed target instance (Generalized Geography):**
Directed graph G with vertices:
- Diamond 1: d_1, T_1, F_1, b_1
- Diamond 2: d_2, T_2, F_2, b_2
- Diamond 3: d_3, T_3, F_3, b_3
- Parity vertices: p_1 (between diamonds 1-2 to ensure Player 2 picks at d_2)
- Clause structure: s (clause selector), c_1, c_2, c_3
- Starting vertex: v_0 = d_1
- Total: 12 + 1 + 4 = 17 vertices

Arcs:
- Diamond 1: (d_1, T_1), (d_1, F_1), (T_1, b_1), (F_1, b_1)
- Diamond 2: (d_2, T_2), (d_2, F_2), (T_2, b_2), (F_2, b_2)
- Diamond 3: (d_3, T_3), (d_3, F_3), (T_3, b_3), (F_3, b_3)
- Chain: (b_1, p_1), (p_1, d_2), (b_2, d_3)
- Clause selector: (b_3, s), (s, c_1), (s, c_2), (s, c_3)
- Literal back-edges:
  - C_1: (c_1, T_1), (c_1, T_2), (c_1, T_3)
  - C_2: (c_2, F_1), (c_2, F_2), (c_2, T_3)
  - C_3: (c_3, T_1), (c_3, F_2), (c_3, F_3)
- Total: 12 + 3 + 4 + 9 = 28 arcs

**Solution mapping:**
- Player 1 at d_1: chooses T_1 (x_1 = TRUE). Forced moves: T_1 -> b_1 -> p_1 -> d_2.
- Player 2 at d_2: suppose chooses T_2 (x_2 = TRUE). Forced: T_2 -> b_2 -> d_3.
- Player 1 at d_3: chooses T_3 (x_3 = TRUE). Forced: T_3 -> b_3 -> s.
- Player 2 at s: must pick a clause to challenge, say c_2. Moves to c_2.
- Player 1 at c_2: edges go to F_1, F_2, T_3. T_3 was already visited (used on the path), so opponent cannot continue from T_3. Player 1 moves to T_3, and Player 2 is stuck (T_3 already visited, no unvisited outgoing arcs from T_3). Player 1 wins.
- If Player 2 instead challenged c_1 or c_3, Player 1 similarly finds a visited literal vertex. All clauses are satisfied, confirming F is TRUE.


## References

- **[Schaefer, 1978a]**: [`Schaefer1978a`] T. J. Schaefer (1978). "Complexity of some two-person perfect-information games". *Journal of Computer and System Sciences* 16, pp. 185–225.
- **[Lichtenstein and Sipser, 1978]**: [`Lichtenstein1978`] David Lichtenstein and Michael Sipser (1978). "{GO} is {Pspace} hard". In: *Proceedings of the 19th Annual Symposium on Foundations of Computer Science*, pp. 48–54. IEEE Computer Society.
