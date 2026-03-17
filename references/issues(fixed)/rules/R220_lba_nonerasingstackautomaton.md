---
name: Rule
about: Propose a new reduction rule
title: "[Rule] LINEAR BOUNDED AUTOMATON ACCEPTANCE to NON-ERASING STACK AUTOMATON ACCEPTANCE"
labels: rule
assignees: ''
---

**Source:** LINEAR BOUNDED AUTOMATON ACCEPTANCE
**Target:** NON-ERASING STACK AUTOMATON ACCEPTANCE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A10.1, p.266

## GJ Source Entry

> [AL5] NON-ERASING STACK AUTOMATON ACCEPTANCE (*)
> INSTANCE: A "one-way nondeterministic non-erasing stack automaton" (a 1NESA) A with input alphabet Σ (see [Hopcroft and Ullman, 1969] for definition), and a string x E Σ*.
> QUESTION: Does A accept x?
> Reference: [Galil, 1976], [Hopcroft and Ullman, 1967]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE. The second reference proves membership in PSPACE.
> Comment: PSPACE-complete, even if x E Σ* is fixed and A is restricted to be a "checking stack automaton" (as defined in [Greibach, 1969]). If x is the empty string and A is further restricted to be a checking stack automaton with a single stack symbol, the problem becomes NP-complete [Galil, 1976]. If instead x is allowed to vary and A is fixed, the problem is in NP for each 1NESA and remains so if A is allowed to be a general "nested stack automaton" [Rounds, 1973]. There exist particular 1NESAs for which the problem is NP-complete [Rounds, 1973], and these particular 1NESAs can be chosen to be checking stack automata [Shamir and Beeri, 1974] that are also "reading pushdown automata" [Hunt, 1976]. However, if A is restricted to be a "one-way nondeterministic pushdown automaton," then the problem can be solved in polynomial time (even with A allowed to vary), as indeed is the case for "two-way nondeterministic pushdown automata" [Aho, Hopcroft, and Ullman, 1968].

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

- **[Hopcroft and Ullman, 1969]**: [`Hopcroft1969`] John E. Hopcroft and Jeffrey D. Ullman (1969). "Formal Languages and their Relation to Automata". Addison-Wesley, Reading, MA.
- **[Galil, 1976]**: [`Galil1976`] Z. Galil (1976). "Hierarchies of complete problems". *Acta Informatica* 6, pp. 77–88.
- **[Hopcroft and Ullman, 1967]**: [`Hopcroft1967`] John E. Hopcroft and Jeffrey D. Ullman (1967). "Nonerasing stack automata". *Journal of Computer and System Sciences* 1, pp. 166–186.
- **[Greibach, 1969]**: [`Greibach1969`] S. Greibach (1969). "Checking automata and one-way stack languages". *Journal of Computer and System Sciences* 3, pp. 196–217.
- **[Rounds, 1973]**: [`Rounds1973`] W. C. Rounds (1973). "Complexity of recognition in intermediate level languages". In: *Proceedings of the 14th Annual Symposium on Switching and Automata Theory*, pp. 145–158. IEEE Computer Society.
- **[Shamir and Beeri, 1974]**: [`Shamir and Beeri1974`] Eli Shamir and Catriel Beeri (1974). "Checking stacks and context-free programmed grammars accept {P}-complete languages". In: *Proc. 2nd Colloq. on Automata, Languages, and Programming*, pp. 27–33. Springer.
- **[Hunt, 1976]**: [`Hunt1976a`] Harry B. Hunt III (1976). "On the complexity of finite, pushdown, and stack automata". *Mathematical Systems Theory* 10, pp. 33–52.
- **[Aho, Hopcroft, and Ullman, 1968]**: [`Aho1968`] A. V. Aho and J. E. Hopcroft and J. D. Ullman (1968). "Time and tape complexity of pushdown automaton languages". *Information and Control* 13, pp. 186–206.