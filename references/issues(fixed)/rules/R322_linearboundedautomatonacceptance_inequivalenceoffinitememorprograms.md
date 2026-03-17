---
name: Rule
about: Propose a new reduction rule
title: "[Rule] LINEAR BOUNDED AUTOMATON ACCEPTANCE to INEQUIVALENCE OF FINITE MEMORY PROGRAMS"
labels: rule
assignees: ''
---

**Source:** LINEAR BOUNDED AUTOMATON ACCEPTANCE
**Target:** INEQUIVALENCE OF FINITE MEMORY PROGRAMS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO13

## GJ Source Entry

> [PO13]  INEQUIVALENCE OF FINITE MEMORY PROGRAMS (*)
> INSTANCE:  Finite set X of variables, finite alphabet Σ, two programs P1 and P2, each a sequence I1,I2,...,Im of instructions (not necessarily of the same length m) of the form "read xi," "write vj," "xi←vj," "if vj=vk goto Il," "accept," or "halt," where each xi ∈ X, each vj ∈ X ∪ Σ ∪ {$}, and Im is either "halt" or "accept."
> QUESTION:  Is there a string w ∈ Σ* such that the two programs yield different outputs for input w (see reference for details on the execution of such programs)?
> Reference:  [Jones and Muchnik, 1977]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
> Comment:  PSPACE-complete, even if P2 is a fixed program with no write instructions and hence no output.  See reference for a number of other special cases and variants that are PSPACE-complete or harder.

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

- **[Jones and Muchnik, 1977]**: [`Jones1977b`] Neil D. Jones and Steven S. Muchnik (1977). "Even simple programs are hard to analyze". *Journal of the Association for Computing Machinery* 24, pp. 338–350.