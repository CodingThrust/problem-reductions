---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GENERIC (any NP problem) to BoundedPostCorrespondenceProblem"
labels: rule
assignees: ''
---

**Source:** GENERIC (any NP problem)
**Target:** BoundedPostCorrespondenceProblem
**Motivation:** A generic (Cook-Levin style) transformation showing that the Bounded PCP is NP-complete by encoding any polynomial-time nondeterministic Turing machine computation as a bounded PCP instance.
**Reference:** Garey & Johnson, *Computers and Intractability*, SR11, p.228. [Constable, Hunt, and Sahni, 1974].

## Reduction Algorithm

> ⚠️ **Unverified** — AI-generated content, not directly from source text

Given any language L in NP with a nondeterministic Turing machine M that decides L in polynomial time p(n):

1. For an input x of length n, construct string pairs (a_i, b_i) that encode the transition function of M.
2. The string pairs simulate the computation of M: choosing index i_j at step j corresponds to a nondeterministic choice of M at that step.
3. The concatenation a_{i_1}...a_{i_k} encodes the sequence of configurations from one 'view', and b_{i_1}...b_{i_k} encodes it from another.
4. The two concatenations are equal if and only if the sequence of configurations forms a valid accepting computation of M on x.
5. The bound K is set to p(n) (the polynomial time bound), ensuring the computation has bounded length.

This is a generic transformation (not a specific Karp reduction from a particular NP-complete problem), similar in spirit to Cook's theorem. The key insight is that the Post Correspondence mechanism naturally captures the verification of computation sequences.

Detailed construction in [Constable, Hunt, and Sahni, 1974].

**Components:**
- String pairs encoding TM transition function
- Index selection simulates nondeterministic choices
- Concatenation equality verifies valid computation
- Bound K = p(n) limits computation length

## Size Overhead

> ⚠️ **Unverified** — AI-generated content, not directly from source text

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| num_pairs | polynomial in |M| and n |
| max_string_length | polynomial in |M| and n |
| bound_K | p(n) |

## Correctness

> ⚠️ **Unverified** — AI-generated content, not directly from source text

**Forward:** If x in L, there exists an accepting computation of M on x of length <= p(n). This computation corresponds to an index sequence whose a- and b-concatenations are identical, with sequence length <= K.

**Backward:** If an index sequence of length <= K makes the concatenations equal, it encodes a valid accepting computation of M on x, proving x in L.

## Validation Method

- Source problem (GENERIC (any NP problem)) does not exist in codebase yet — implement model first
- Reduction type: generic transformation (Cook-Levin style)

## Example

> ⚠️ **Unverified** — AI-generated content, not directly from source text

- This is a GENERIC transformation, not a specific reduction from a single NP-complete problem. It directly encodes arbitrary NP computations.
- The unbounded Post Correspondence Problem (no limit on k) is undecidable (Hopcroft and Ullman, 1969).
- The bounded version is one of the few NP-completeness results established via generic transformation rather than specific polynomial reduction.
- Reference: [Constable, Hunt, and Sahni, 1974].
