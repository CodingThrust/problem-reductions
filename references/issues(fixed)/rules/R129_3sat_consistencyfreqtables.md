---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Consistency of Database Frequency Tables"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Consistency of Database Frequency Tables
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.235

## GJ Source Entry

> [SR35] CONSISTENCY OF DATABASE FREQUENCY TABLES
> INSTANCE: Set A of attribute names, domain set D_a for each a E A, set V of objects, collection F of frequency tables for some pairs a,b E A (where a frequency table for a,b E A is a function f_{a,b}: D_a × D_b → Z+ with the sum, over all pairs x E D_a and y E D_b, of f_{a,b}(x,y) equal to |V|), and a set K of triples (v,a,x) with v E V, a E A, and x E D_a, representing the known attribute values.
> QUESTION: Are the frequency tables in F consistent with the known attribute values in K, i.e., is there a collection of functions g_a: V → D_a, for each a E A, such that g_a(v) = x if (v,a,x) E K and such that, for each f_{a,b} E F, x E D_a, and y E D_b, the number of v E V for which g_a(v) = x and g_b(v) = y is exactly f_{a,b}(x,y)?
> Reference: [Reiss, 1977b]. Transformation from 3SAT.
> Comment: Above result implies that no polynomial time algorithm can be given for "compromising" a data base from its frequency tables by deducing prespecified attribute values, unless P = NP (see reference for details).

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

- **[Reiss, 1977b]**: [`Reiss1977b`] S. P. Reiss (1977). "Statistical database confidentiality". Dept. of Statistics, University of Stockholm.