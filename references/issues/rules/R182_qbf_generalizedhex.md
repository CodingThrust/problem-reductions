---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to GENERALIZED HEX"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formulas (QBF)'
canonical_target_name: 'Generalized Hex'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** GENERALIZED HEX
**Motivation:** Establishes the PSPACE-completeness of Generalized Hex (the Shannon switching game on vertices) by reducing from QBF, providing one of the earliest and most influential PSPACE-completeness results for combinatorial games and motivating the study of computational complexity in positional games.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254

## GJ Source Entry

> [GP1] GENERALIZED HEX (*)
> INSTANCE: Graph G = (V,E) and two specified vertices s, t E V.
> QUESTION: Does player 1 have a forced win in the following game played on G? The players alternate choosing a vertex from V - {s,t}, with those chosen by player 1 being colored "blue" and those chosen by player 2 being colored "red." Play continues until all such vertices have been colored, and player 1 wins if and only if there is a path from s to t in G that passes through only blue vertices.
> Reference: [Even and Tarjan, 1976]. Transformation from QBF.
> Comment: PSPACE-complete. The variant in which players alternate choosing an edge instead of a vertex, known as "the Shannon switching game on edges," can be solved in polynomial time [Bruno and Weinberg, 1970]. If G is a directed graph and player 1 wants a "blue" directed path from s to t, both the vertex selection game and the arc selection game are PSPACE-complete [Even and Tarjan, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a QBF instance F = (Q_1 u_1)(Q_2 u_2)...(Q_n u_n)E where E is a Boolean expression in CNF with clauses C_1, ..., C_m, construct a Generalized Hex instance (G, s, t) as follows:

1. **Variable gadgets:** For each variable u_i (i = 1, ..., n), create a "diamond" subgraph with four vertices: an entry vertex e_i, a TRUE vertex T_i, a FALSE vertex F_i, and an exit vertex x_i. Add edges {e_i, T_i}, {e_i, F_i}, {T_i, x_i}, {F_i, x_i}. These diamonds are chained: connect x_i to e_{i+1} by a path of intermediate forced-move vertices.

2. **Clause gadgets:** For each clause C_j (j = 1, ..., m), create a clause vertex c_j. For each literal l in C_j, add an edge from c_j to the corresponding truth-value vertex (T_i if l = u_i, F_i if l = NOT u_i).

3. **Terminal structure:** Set s as e_1 (the entry of the first diamond). After the last diamond, connect x_n to a clause-checking structure. The target vertex t is connected to the clause checking structure so that Player 1 can complete an s-t path only if all clauses are satisfiable under the chosen truth assignment.

4. **Quantifier encoding:** Whether Player 1 or Player 2 chooses the truth value of u_i depends on whether Q_i = EXISTS or Q_i = FORALL. The diamond structure forces the appropriate player to choose between T_i and F_i, simulating the quantifier alternation. The parity of forced-move vertices between diamonds ensures the correct player makes the choice.

5. **Winning condition:** Player 1 has a winning strategy in the Hex game (can guarantee an s-t blue path) if and only if F is true (the existential player has a winning strategy in the formula game).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source QBF
- m = `num_clauses` of source QBF
- L = total number of literal occurrences across all clauses

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `4 * num_vars + num_clauses + 2 * num_vars + 2` |
| `num_edges` | `4 * num_vars + 2 * num_vars + L + num_clauses` |

**Derivation:** Each of the n variables contributes 4 diamond vertices plus up to 2 connecting vertices (for forced-move parity adjustment). The m clause vertices contribute m vertices. The 2 accounts for s and t terminal vertices. Edges include 4 per diamond (internal), connecting edges between diamonds (about 2n), literal-to-clause edges (L total), and clause-to-terminal edges (m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a small QBF instance, apply the reduction to produce a Generalized Hex graph (G, s, t), solve the game by exhaustive game-tree search (minimax), and verify the game outcome matches the QBF truth value
- For a TRUE QBF, verify Player 1 has a winning strategy (blue s-t path exists under optimal play)
- For a FALSE QBF, verify Player 2 can prevent any blue s-t path
- Verify overhead formulas by comparing constructed graph sizes against predicted sizes

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
F = EXISTS u_1 FORALL u_2 EXISTS u_3 [(u_1 OR u_2 OR u_3) AND (NOT u_1 OR NOT u_2 OR u_3) AND (u_1 OR NOT u_2 OR NOT u_3)]

This has n = 3 variables and m = 3 clauses.
- C_1 = (u_1 OR u_2 OR u_3)
- C_2 = (NOT u_1 OR NOT u_2 OR u_3)
- C_3 = (u_1 OR NOT u_2 OR NOT u_3)

**Truth evaluation:** F is TRUE. Existential player sets u_1 = TRUE. For any u_2:
- If u_2 = TRUE: set u_3 = TRUE => C_1 = T, C_2 = T (NOT T OR NOT T OR T = T), C_3 = T (T OR NOT T OR NOT T = T). All satisfied.
- If u_2 = FALSE: set u_3 = FALSE => C_1 = T (T OR F OR F), C_2 = T (NOT T OR NOT F OR F = F OR T OR F = T), C_3 = T (T OR T OR T). All satisfied.

**Constructed target instance (Generalized Hex):**
Graph G with vertices:
- Variable diamonds: {e_1, T_1, F_1, x_1}, {e_2, T_2, F_2, x_2}, {e_3, T_3, F_3, x_3}
- Connecting vertices: p_1 (between diamond 1 and 2), p_2 (between diamond 2 and 3)
- Clause vertices: c_1, c_2, c_3
- Terminal vertices: s (= e_1), t
- Total: 12 diamond vertices + 2 connecting + 3 clause + 1 terminal = 18 vertices

Edges (undirected):
- Diamond internals: {e_1,T_1}, {e_1,F_1}, {T_1,x_1}, {F_1,x_1}, same for diamonds 2 and 3 (12 edges)
- Chain connections: {x_1,p_1}, {p_1,e_2}, {x_2,p_2}, {p_2,e_3} (4 edges)
- Clause-literal edges: {c_1,T_1}, {c_1,T_2}, {c_1,T_3}, {c_2,F_1}, {c_2,F_2}, {c_2,T_3}, {c_3,T_1}, {c_3,F_2}, {c_3,F_3} (9 edges)
- Terminal connections: {x_3,c_1}, {x_3,c_2}, {x_3,c_3}, {c_1,t}, {c_2,t}, {c_3,t} (6 edges)
- Total: 31 edges

**Solution mapping:**
- Player 1 (existential) picks T_1 (setting u_1 = TRUE) at diamond 1
- Player 2 (universal) picks some value at diamond 2 (say T_2, setting u_2 = TRUE)
- Player 1 picks T_3 (setting u_3 = TRUE) at diamond 3
- Blue path: s = e_1 -> T_1 -> x_1 -> p_1 -> e_2 -> (F_2 still blue or available) -> ... -> t
- Player 1 can guarantee a blue s-t path regardless of Player 2's choices, confirming F is TRUE.


## References

- **[Even and Tarjan, 1976]**: [`Even1976b`] S. Even and R. E. Tarjan (1976). "A combinatorial problem which is complete in polynomial space". *Journal of the Association for Computing Machinery* 23, pp. 710–719.
- **[Bruno and Weinberg, 1970]**: [`Bruno1970`] J. Bruno and L. Weinberg (1970). "A constructive graph-theoretic solution of the {Shannon} switching game". *IEEE Transactions on Circuit Theory* CT-17, pp. 74–81.
