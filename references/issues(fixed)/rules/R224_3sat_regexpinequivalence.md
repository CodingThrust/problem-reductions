---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to REGULAR EXPRESSION INEQUIVALENCE"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** REGULAR EXPRESSION INEQUIVALENCE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, AL11

## GJ Source Entry

> [AL9]  REGULAR EXPRESSION INEQUIVALENCE (*)
> INSTANCE:  Regular expressions E1 and E2 over the operators {∪,·,*} and the alphabet Σ (see Section 7.4 for definition).
> QUESTION:  Do E1 and E2 represent different languages?
> Reference:  [Stockmeyer and Meyer, 1973], [Stockmeyer, 1974a]. Generic transformation. The second reference proves membership in PSPACE.
> Comment:  PSPACE-complete, even if |Σ| = 2 and E2 = Σ* (REGULAR EXPRESSION NON-UNIVERSALITY, see Section 7.4). In fact, PSPACE-complete if E2 is any fixed expression representing an "unbounded" language [Hunt, Rosenkrantz, and Szymanski, 1976a]. NP-complete for fixed E2 representing any infinite "bounded" language, but solvable in polynomial time for fixed E2 representing any finite language. The general problem remains PSPACE-complete if E1 and E2 both have "star height" k for a fixed k ≥ 1 [Hunt, Rosenkrantz, and Szymanski, 1976a], but is NP-complete for k = 0 ("star free") [Stockmeyer and Meyer, 1973], [Hunt, 1973a]. Also NP-complete if one or both of E1 and E2 represent bounded languages (a property that can be checked in polynomial time) [Hunt, Rosenkrantz, and Szymanski, 1976a] or if |Σ| = 1 [Stockmeyer and Meyer, 1973]. For related results and intractable generalizations, see cited references, [Hunt, 1973b], and [Hunt and Rosenkrantz, 1978].

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

- **[Stockmeyer and Meyer, 1973]**: [`Stockmeyer and Meyer1973`] Larry J. Stockmeyer and Albert R. Meyer (1973). "Word problems requiring exponential time". In: *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1–9. Association for Computing Machinery.
- **[Stockmeyer, 1974a]**: [`Stockmeyer1974a`] Larry J. Stockmeyer (1974). "The Complexity of Decision Problems in Automata Theory and Logic". Dept. of Electrical Engineering, Massachusetts Institute of Technology.
- **[Hunt, Rosenkrantz, and Szymanski, 1976a]**: [`Hunt1976b`] Harry B. Hunt III and Daniel J. Rosenkrantz and Thomas G. Szymanski (1976). "On the equivalence, containment, and covering problems for the regular and context-free languages". *Journal of Computer and System Sciences* 12, pp. 222–268.
- **[Hunt, 1973a]**: [`Hunt1973a`] Harry B. Hunt III (1973). "On the Time and Tape Complexity of Languages". Dept. of Computer Science, Cornell University.
- **[Hunt, 1973b]**: [`Hunt1973b`] Harry B. Hunt III (1973). "On the time and tape complexity of languages {I}". In: *Proceedings of the 5th Annual ACM Symposium on Theory of Computing*, pp. 10–19. Association for Computing Machinery.
- **[Hunt and Rosenkrantz, 1978]**: [`Hunt1978b`] Harry B. Hunt III and Daniel J. Rosenkrantz (1978). "Computational parallels between regular and context-free languages". *SIAM Journal on Computing* 7, pp. 99–114.