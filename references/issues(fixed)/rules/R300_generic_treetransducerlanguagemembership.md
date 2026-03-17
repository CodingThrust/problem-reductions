---
name: Rule
about: Propose a new reduction rule
title: "[Rule] generic to TREE TRANSDUCER LANGUAGE MEMBERSHIP"
labels: rule
assignees: ''
---

**Source:** generic
**Target:** TREE TRANSDUCER LANGUAGE MEMBERSHIP
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL21

## Reduction Algorithm

> INSTANCE: A "top-down finite-state tree transducer" M with output alphabet Γ, a context-free grammar G, and a string w∈Γ* (see references for detailed definitions).
> QUESTION: Is w in the "yield" of the "surface set" determined by M and G?
> Reference: [Reiss, 1977a]. Generic transformation.
> Comment: PSPACE-complete. Problem is in NP for fixed M and G, and there exist particular choices for M and G for which the problem is NP-complete [Rounds, 1973]. The general problem is solvable in polynomial time if M is required to be "linear", while for fixed M the problem is solvable in polynomial time if M is "deterministic" [Reiss, 1977b].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Reiss, 1977a]**: [`Reiss1977a`] S. P. Reiss (1977). "Inverse Translation: the Theory of Practical Automatic Programming". Yale University.
- **[Rounds, 1973]**: [`Rounds1973`] W. C. Rounds (1973). "Complexity of recognition in intermediate level languages". In: *Proceedings of the 14th Annual Symposium on Switching and Automata Theory*, pp. 145–158. IEEE Computer Society.
- **[Reiss, 1977b]**: [`Reiss1977b`] S. P. Reiss (1977). "Statistical database confidentiality". Dept. of Statistics, University of Stockholm.