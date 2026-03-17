---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SET COVERING to LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE"
labels: rule
assignees: ''
---

**Source:** SET COVERING
**Target:** LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.257

## GJ Source Entry

> [GP12] LEFT-RIGHT HACKENBUSH FOR REDWOOD FURNITURE
> INSTANCE: A piece of "redwood furniture," i.e., a connected graph G = (V,E) with a specified "ground" vertex v E V and a partition of the edges into sets L and R, where L is the set of all edges containing v (the set of "feet"), R = E - L, and each "foot" in L shares a vertex with at most one edge in R, which is its corresponding "leg" (not all edges in R need to be legs however), and a positive integer K.
> QUESTION: Is the "value" of the Left-Right Hackenbush game played on G less than or equal to 2^{-K} (see [Conway, 1976] for the definition of the game, there called Hackenbush Restrained, and for the definition of "value")?
> Reference: [Berlekamp, 1976]. Transformation from SET COVERING.
> Comment: Remains NP-complete even for "bipartite" redwood furniture, but can be solved in polynomial time for the subclass of redwood furniture known as "redwood trees." As a consequence of this result, the problem of determining if player 1 has a win in an arbitrary game of Left-Right Hackenbush is NP-hard.

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

- **[Conway, 1976]**: [`Conway1976`] J. H. Conway (1976). "On Numbers and Games". Academic Press, New York.
- **[Berlekamp, 1976]**: [`Berlekamp1976`] E. R. Berlekamp (1976). "".