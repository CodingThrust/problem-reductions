---
name: Rule
about: Propose a new reduction rule
title: "[Rule] generic to NON-LR(K) CONTEXT-FREE GRAMMAR"
labels: rule
assignees: ''
---

**Source:** generic
**Target:** NON-LR(K) CONTEXT-FREE GRAMMAR
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL15

## Reduction Algorithm

> INSTANCE: Context-free grammar G, positive integer K written in unary notation.
> QUESTION: Is G not an LR(K) grammar (see reference for definition)?
> Reference: [Hunt, Szymanski, and Ullman, 1975]. Generic transformation.
> Comment: Solvable in polynomial time for any fixed K. If K is written in binary (as in our standard encodings), then the problem is complete for NEXP-TIME and hence intractable. Determining whether there exists an integer K such that G is an LR(K) grammar is undecidable [Hunt and Szymanski, 1976a]. The same results hold if "LR(K)" is replaced by "LL(K)," "LC(K)," "SLR(K)," or any one of a number of other properties (see above references). However, in the case of LL(K), if it is known that there is some K' for which G is LR(K'), then one can decide whether there exists a K for which G is LL(K) in polynomial time [Hunt and Szymanski, 1978].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Hunt, Szymanski, and Ullman, 1975]**: [`Hunt1975`] Harry B. Hunt III and Thomas G. Szymanski and Jeffrey D. Ullman (1975). "On the complexity of {LR}(k) testing". *Communications of the ACM* 18, pp. 707–716.
- **[Hunt and Szymanski, 1976a]**: [`Hunt1976d`] Harry B. Hunt III and Thomas G. Szymanski (1976). "Complexity metatheorems for context-free grammar problems". *Journal of Computer and System Sciences* 13, pp. 318–334.
- **[Hunt and Szymanski, 1978]**: [`Hunt1978c`] Harry B. Hunt III and Thomas G. Szymanski (1978). "Lower bounds and reductions between grammar problems". *Journal of the Association for Computing Machinery* 25, pp. 32–51.