---
name: Rule
about: Propose a new reduction rule
title: "[Rule] (generic transformation) to QUASI-REALTIME AUTOMATON ACCEPTANCE"
labels: rule
assignees: ''
---

**Source:** (generic transformation)
**Target:** QUASI-REALTIME AUTOMATON ACCEPTANCE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A10.1, p.265-266

## GJ Source Entry

> [AL4] QUASI-REALTIME AUTOMATON ACCEPTANCE
> INSTANCE: A multi-tape nondeterministic Turing machine M (Turing machine program, in our terminology), whose input tape read-head must move right at each step, and which must halt whenever the read-head sees a blank, and a string x over the input alphabet Σ of M. (For a more complete description of this type of machine and its equivalent formulations, see [Book and Greibach, 1970].)
> QUESTION: Does M accept x?
> Reference: [Book, 1972]. Generic transformation.
> Comment: Remains NP-complete even if M has only a single work tape in addition to its input tape. See also QUASI-REALTIME LANGUAGE MEMBERSHIP (the languages accepted by quasi-realtime automata are the same as the quasi-realtime languages defined in that entry).

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

- **[Book and Greibach, 1970]**: [`Book1970`] R. V. Book and S. Greibach (1970). "Quasi-realtime languages". *Mathematical Systems Theory* 4, pp. 97–111.
- **[Book, 1972]**: [`Book1972`] R. V. Book (1972). "On languages accepted in polynomial time". *SIAM Journal on Computing* 1, pp. 281–287.