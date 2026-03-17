---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MAXIMUM FIXED-LENGTH DISJOINT PATHS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MAXIMUM FIXED-LENGTH DISJOINT PATHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND42, p.218

## GJ Source Entry

> [ND42] MAXIMUM FIXED-LENGTH DISJOINT PATHS
> INSTANCE: Graph G=(V,E), specified vertices s and t, positive integers J,K≤|V|.
> QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, each involving exactly K edges?
> Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete for fixed K≥4. Solvable in polynomial time for K≤3. Corresponding problem for edge-disjoint paths is NP-complete for fixed K≥4, polynomially solvable for K≤2, and open for K=3. The same results hold for directed graphs and directed paths, except that the arc-disjoint version is polynomially solvable for K≤3 and open for K=4.

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

- **[Itai, Perl, and Shiloach, 1977]**: [`Itai1977b`] Alon Itai and Yehoshua Perl and Yossi Shiloach (1977). "The complexity of finding maximum disjoint paths with length constraints". Dept. of Computer Science, Technion.