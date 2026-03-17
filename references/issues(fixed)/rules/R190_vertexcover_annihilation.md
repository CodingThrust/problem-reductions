---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to ANNIHILATION"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** ANNIHILATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.256

## GJ Source Entry

> [GP9] ANNIHILATION (*)
> INSTANCE: Directed acyclic graph G = (V,A), collection {A_i: 1 ≤ i ≤ r} of (not necessarily disjoint) subsets of A, function f_0 mapping V into {0,1,2,...,r}, where f_0(v) = i > 0 means that a "token" of type i is "on" vertex v and f_0(v) = 0 means that v is unoccupied.
> QUESTION: Does player 1 have a forced win in the following game played on G? A position is a function f: V → {0,1,...,r} with f_0 being the initial position and players alternating moves. A player moves by selecting a vertex v E V with f(v) > 0 and an arc (v,w) E A_{f(v)}, and the move corresponds to moving the token on vertex v to vertex w. The new position f' is the same as f except that f'(v) = 0 and f'(w) is either 0 or f(v), depending, respectively, on whether f(w) > 0 or f(w) = 0. (If f(w) > 0, then both the token moved to w and the token already there are "annihilated.") Player 1 wins if and only if player 2 is the first player unable to move.
> Reference: [Fraenkel and Yesha, 1977]. Transformation from VERTEX COVER.
> Comment: NP-hard and in PSPACE, but not known to be PSPACE-complete. Remains NP-hard even if r = 2 and A_1 ∩ A_2 is empty. Problem can be solved in polynomial time if r = 1 [Fraenkel and Yesha, 1976]. Related NP-hardness results for other token-moving games on directed graphs (REMOVE, CONTRAJUNCTIVE, CAPTURE, BLOCKING, TARGET) can be found in [Fraenkel and Yesha, 1977].

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Fraenkel and Yesha, 1977]**: [`Fraenkel1977`] A. S. Fraenkel and Y. Yesha (1977). "Complexity of problems in games, graphs, and algebraic equations".
- **[Fraenkel and Yesha, 1976]**: [`Fraenkel1976`] A. S. Fraenkel and Y. Yesha (1976). "Theory of annihilation games". *Bulletin of the American Mathematical Society* 82, pp. 775–777.