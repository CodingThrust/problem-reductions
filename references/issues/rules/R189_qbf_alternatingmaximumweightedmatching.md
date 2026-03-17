---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to ALTERNATING MAXIMUM WEIGHTED MATCHING"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formula (QBF)'
canonical_target_name: 'Alternating Maximum Weighted Matching'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** ALTERNATING MAXIMUM WEIGHTED MATCHING
**Motivation:** This reduction establishes that the alternating maximum weighted matching game is PSPACE-complete by reducing from QBF, demonstrating that although maximum weighted matching is solvable in polynomial time, the two-player alternating version captures the full power of PSPACE through the game-tree structure inherent in quantified boolean formulas.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.256

## GJ Source Entry

> [GP8] ALTERNATING MAXIMUM WEIGHTED MATCHING (*)
> INSTANCE: Graph G = (V,E), a weight w(e) E Z+ for each e E E, and a bound B E Z+.
> QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new edge from E, subject to the constraint that no edge can share an endpoint with any of the already chosen edges. If the sum of the weights of the edges chosen ever exceeds B, player 1 wins.
> Reference: [Dobkin and Ladner, 1978]. Transformation from QBF.
> Comment: PSPACE-complete, even though the corresponding weighted matching problem can be solved in polynomial time (e.g., see [Lawler, 1976a]).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

The reduction from QBF to Alternating Maximum Weighted Matching follows the standard pattern for proving PSPACE-completeness of two-player games via reduction from TQBF (True Quantified Boolean Formulas). The original result is due to Dobkin and Ladner (1978, private communication), as cited in Garey & Johnson.

**High-level approach:**
Given a QBF instance $\forall x_1 \exists x_2 \forall x_3 \cdots \phi(x_1, \ldots, x_n)$, construct a weighted graph G = (V, E) with edge weights and bound B such that:

1. **Variable gadgets:** For each quantified variable $x_i$, construct a subgraph where the corresponding player (player 1 for existential, player 2 for universal quantifiers) must choose one of two edges, encoding the truth assignment of that variable.

2. **Clause gadgets:** For each clause in the CNF formula $\phi$, construct a subgraph with weighted edges such that the total weight contributed depends on whether the clause is satisfied by the chosen variable assignments.

3. **Weight encoding:** Assign weights to edges so that the total weight of edges chosen exceeds the bound B if and only if the formula $\phi$ evaluates to true under the assignment determined by the players' alternating edge choices.

4. **Matching constraint enforcement:** The constraint that no two chosen edges share an endpoint is used to force the game tree to mirror the quantifier structure of the QBF.

Player 1 has a forced win (total weight exceeds B) if and only if the QBF is true.

**Note:** The specific construction details were communicated privately by Dobkin and Ladner in 1978, and the full proof is not published in a standard reference. The reduction uses polynomial-time constructibility and preserves the alternating quantifier structure of QBF.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in the QBF
- m = number of clauses in the QBF

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n + m) — one gadget per variable and per clause |
| `num_edges` | O(n * m) — variable-clause connections plus gadget edges |
| `bound` | Derived from clause weights, polynomial in m |

**Note:** Exact overhead expressions are not available since the Dobkin-Ladner construction was a private communication. The expressions above are estimated based on standard QBF-to-game reductions.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a small QBF instance, reduce to an Alternating Maximum Weighted Matching instance, use game-tree search (minimax) to determine the winner, and verify it matches the truth value of the QBF
- Test with both true and false QBF instances to verify bidirectionality
- Verify that the constructed graph respects the matching constraint (no two edges share an endpoint within any valid play sequence)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
$\exists x_1 \forall x_2 \exists x_3: (x_1 \lor x_2 \lor x_3) \land (\neg x_1 \lor \neg x_2 \lor x_3) \land (x_1 \lor \neg x_2 \lor \neg x_3)$

This QBF is true: player 1 (existential) can set $x_1 = \text{true}$; then for any $x_2$ chosen by player 2 (universal), player 1 can set $x_3$ appropriately:
- If $x_2 = \text{true}$: set $x_3 = \text{true}$, all clauses satisfied
- If $x_2 = \text{false}$: set $x_3 = \text{false}$, all clauses satisfied

**Constructed target instance (Alternating Maximum Weighted Matching):**
Graph G with 12 vertices and weighted edges:
- Variable gadget for $x_1$: vertices {v1a, v1b, v1c} with edges (v1a, v1b) weight 0 [true], (v1a, v1c) weight 0 [false]
- Variable gadget for $x_2$: vertices {v2a, v2b, v2c} with edges (v2a, v2b) weight 0 [true], (v2a, v2c) weight 0 [false]
- Variable gadget for $x_3$: vertices {v3a, v3b, v3c} with edges (v3a, v3b) weight 0 [true], (v3a, v3c) weight 0 [false]
- Clause reward edges: 3 clause vertices {c1, c2, c3} connected to variable vertices via weighted edges encoding satisfaction, with weight B/3 each when the clause is satisfied
- Bound B chosen so that total weight > B iff all 3 clauses are satisfied

**Solution mapping:**
- Player 1 (existential) chooses edge for $x_1$ = true, then player 2 (universal) picks $x_2$, then player 1 picks $x_3$
- Under optimal play by player 1, the total weight of chosen edges exceeds B regardless of player 2's moves
- This corresponds to the QBF being true


## References

- **[Dobkin and Ladner, 1978]**: [`Dobkin1978`] D. Dobkin and R. E. Ladner (1978). "Private communication".
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.
