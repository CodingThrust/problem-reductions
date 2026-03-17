---
name: Problem
about: Propose a new problem type
title: "[Model] LeftRightHackenbushForRedwoodFurniture"
labels: model
assignees: ''
---

## Motivation

LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE (P249) from Garey & Johnson, A8 GP12. A restricted form of the Left-Right Hackenbush game (also called Hackenbush Restrained by Conway) played on "redwood furniture" positions. Despite the structural restrictions, computing the game value remains NP-complete. As a consequence, determining the winner in general Left-Right Hackenbush is NP-hard.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R193 (SET COVERING to LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE) -- transformation from Set Covering establishes NP-completeness
- **As source:** None found in the current issue set

## Definition

**Name:** <!-- ⚠️ Unverified --> `LeftRightHackenbushRedwoodFurniture`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Left-Right Hackenbush for Redwood Furniture (also: Red-Blue Hackenbush on Redwood Furniture; Hackenbush Restrained on Redwood Furniture)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP12

**Mathematical definition:**

INSTANCE: A piece of "redwood furniture," i.e., a connected graph G = (V,E) with a specified "ground" vertex v ∈ V and a partition of the edges into sets L and R, where L is the set of all edges containing v (the set of "feet"), R = E - L, and each "foot" in L shares a vertex with at most one edge in R, which is its corresponding "leg" (not all edges in R need to be legs however), and a positive integer K.
QUESTION: Is the "value" of the Left-Right Hackenbush game played on G less than or equal to 2^{-K}?

In Left-Right Hackenbush (Hackenbush Restrained), Left can remove blue/L edges and Right can remove red/R edges. After an edge is removed, any edges no longer connected to the ground vertex are also removed. The "value" is defined in the sense of combinatorial game theory (Conway, 1976).

The problem is a satisfaction (decision) problem: determine whether the game value is at most 2^{-K}.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |E| (one binary variable per edge)
- **Per-variable domain:** binary {0, 1} -- whether edge e ∈ E is still present in the current game position
- **Meaning:** A game position corresponds to a subgraph of G where some edges have been removed. The game tree explores all possible sequences of edge removals by Left (from L) and Right (from R). The game value is a surreal number determined by the game tree structure.

**Note:** The game-tree structure means this is not a standard combinatorial optimization. The value computation involves surreal number arithmetic over the game tree.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `LeftRightHackenbushRedwoodFurniture`
**Variants:** None (the graph structure including L/R partition is stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices |V| |
| `edges` | `Vec<(usize, usize)>` | All edges E of the graph |
| `ground` | `usize` | Index of the ground vertex v |
| `left_edges` | `Vec<usize>` | Indices into `edges` for set L (feet -- edges touching ground) |
| `right_edges` | `Vec<usize>` | Indices into `edges` for set R = E - L (legs and upper structure) |
| `bound_k` | `usize` | The precision bound K: is value ≤ 2^{-K}? |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Berlekamp, 1976; transformation from SET COVERING).
- **Best known exact algorithm:** Compute the game value using surreal number arithmetic. For general Left-Right Hackenbush, compute the game tree and evaluate using Conway's theory. The value of a Hackenbush stalk (path) can be computed in linear time via Berlekamp's binary encoding. For general redwood furniture, the value computation is NP-hard in the worst case.
- **Polynomial-time special case:** Redwood trees (the subclass where the redwood furniture graph is a tree) can be solved in polynomial time (GJ comment).
- **NP-complete restriction:** Remains NP-complete even for bipartite redwood furniture (GJ comment).
- **Consequence:** Since redwood furniture is a special case of general Left-Right Hackenbush, determining a winner in arbitrary Left-Right Hackenbush is NP-hard.
- **References:**
  - [Berlekamp, 1976] E. R. Berlekamp (1976). NP-completeness of redwood furniture value computation.
  - [Conway, 1976] J. H. Conway (1976). "On Numbers and Games". Academic Press. Definition of Hackenbush Restrained and game values.
  - [Berlekamp, Conway, Guy, 1982] "Winning Ways for Your Mathematical Plays". Extended treatment of Hackenbush, redwood furniture, and game values.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a restriction of:** General Left-Right Hackenbush (arbitrary colored graphs rooted at ground)
- **Polynomial special case:** Redwood trees (tree-structured redwood furniture) -- solvable in polynomial time
- **Still NP-complete:** Bipartite redwood furniture
- **Related problems:** Hackenbush Restrained (Conway's name), Blue-Red Hackenbush, Green Hackenbush (impartial variant, solvable in polynomial time via Sprague-Grundy)
- **Redwood furniture structure:** Blue edges (feet) all touch ground, red edges (legs/body) do not touch ground, each foot shares at most one vertex with a leg

## Extra Remark

**Full book text:**

INSTANCE: A piece of "redwood furniture," i.e., a connected graph G = (V,E) with a specified "ground" vertex v ∈ V and a partition of the edges into sets L and R, where L is the set of all edges containing v (the set of "feet"), R = E - L, and each "foot" in L shares a vertex with at most one edge in R, which is its corresponding "leg" (not all edges in R need to be legs however), and a positive integer K.
QUESTION: Is the "value" of the Left-Right Hackenbush game played on G less than or equal to 2^{-K} (see [Conway, 1976] for the definition of the game, there called Hackenbush Restrained, and for the definition of "value")?

Reference: [Berlekamp, 1976]. Transformation from SET COVERING.
Comment: Remains NP-complete even for "bipartite" redwood furniture, but can be solved in polynomial time for the subclass of redwood furniture known as "redwood trees." As a consequence of this result, the problem of determining if player 1 has a win in an arbitrary game of Left-Right Hackenbush is NP-hard.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate the full game tree of Left-Right Hackenbush on the given redwood furniture. At each position, Left removes a blue (L) edge and Right removes a red (R) edge, plus any edges disconnected from ground. Compute the surreal number value via recursive game-tree evaluation. Check if the value ≤ 2^{-K}.
- [ ] It can be solved by reducing to integer programming. Not directly, since game-value computation involves surreal number arithmetic, not linear optimization.
- [x] Other: For Hackenbush stalks (paths from ground), use Berlekamp's binary encoding to compute the value in linear time. For redwood trees, polynomial-time algorithms exist. For general redwood furniture, no polynomial algorithm is known.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

A piece of redwood furniture:
- 9 vertices {v, u1, u2, u3, u4, u5, w1, w2, w3} where v is the ground vertex
- Blue edges L (feet, all touching ground v):
  - e0 = {v, u1}, e1 = {v, u2}, e2 = {v, u3}, e3 = {v, u4}, e4 = {v, u5}
- Red edges R (legs and upper body, none touching ground):
  - e5 = {u1, w1} (leg of foot e0)
  - e6 = {u2, w1} (leg of foot e1)
  - e7 = {u3, w2} (leg of foot e2)
  - e8 = {u4, w2} (leg of foot e3)
  - e9 = {u4, w3} -- note: u4's foot e3 can share vertex with at most one leg, so this violates the constraint. Let us fix: e9 = {u5, w3} (leg of foot e4)
  - e10 = {w1, w2} (internal red edge, not a leg)
  - e11 = {w2, w3} (internal red edge, not a leg)

Corrected structure:
- 9 vertices, ground = v
- L = {e0, e1, e2, e3, e4} -- 5 blue feet
- R = {e5, e6, e7, e8, e9, e10, e11} -- 7 red edges
- Each foot shares at most one vertex with a leg:
  - Foot e0={v,u1}: leg e5={u1,w1}
  - Foot e1={v,u2}: leg e6={u2,w1}
  - Foot e2={v,u3}: leg e7={u3,w2}
  - Foot e3={v,u4}: leg e8={u4,w2}
  - Foot e4={v,u5}: leg e9={u5,w3}
- Bound K = 3 (question: is game value ≤ 2^{-3} = 1/8?)

**Game value analysis:**
The redwood furniture has 5 blue feet and 7 red edges. Left (blue player) can remove feet; Right (red player) can remove red edges. When a foot is removed, any red edges no longer connected to ground through other paths are also removed.

The value of this position depends on the interaction between the feet, legs, and internal red edges. The internal edges {w1,w2} and {w2,w3} create connectivity between the upper vertices, making the structure more complex than independent stalks.

For simple stalks (e.g., ground -- blue -- red -- red), the value is computed via Berlekamp's rule. For this interconnected structure, the value requires full game-tree analysis.
