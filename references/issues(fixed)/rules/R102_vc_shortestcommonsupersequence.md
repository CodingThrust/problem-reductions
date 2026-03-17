---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MinimumVertexCover to ShortestCommonSupersequence"
labels: rule
assignees: ''
---

**Source:** MinimumVertexCover
**Target:** ShortestCommonSupersequence
**Motivation:** Transform a Vertex Cover instance into a Shortest Common Supersequence instance by encoding graph edges as strings whose optimal interleaving corresponds to the vertex cover.
**Reference:** Garey & Johnson, *Computers and Intractability*, SR8, p.228. [Maier, 1978].

## Reduction Algorithm

> ⚠️ **Unverified** — AI-generated content, not directly from source text

Given a graph G = (V, E) and integer K:

1. Create an alphabet Sigma from the vertices and edges of G. Each vertex v in V becomes a symbol, and additional separator symbols may be used.
2. For each edge e = {u, v} in E, create a string that encodes the requirement that at least one of u or v must appear in certain positions of the supersequence.
3. The bound K' on the supersequence length is set as a function of |V|, |E|, and K such that a supersequence of length <= K' exists if and only if a vertex cover of size <= K exists.
4. The strings are constructed so that the optimal supersequence must 'choose' which endpoint of each edge to include, corresponding to the vertex cover selection.

The detailed construction appears in [Maier, 1978]. The key insight is that subsequence containment allows encoding the 'at least one endpoint' constraint of vertex cover through string ordering requirements.

**Components:**
- Alphabet Sigma derived from vertices and edges of the graph
- One string per edge encoding the covering requirement
- Additional strings encoding vertex selection constraints
- Supersequence length bound K' = f(|V|, |E|, K)

## Size Overhead

> ⚠️ **Unverified** — AI-generated content, not directly from source text

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| alphabet_size | polynomial in |V| + |E| |
| num_strings | polynomial in |E| |
| bound_K | polynomial in |V| + |E| + K |

## Correctness

> ⚠️ **Unverified** — AI-generated content, not directly from source text

**Forward:** If G has a vertex cover S of size <= K, then the vertices in S can be used to construct a supersequence: the selected vertices appear in positions that allow all edge-strings to be embedded as subsequences within the length bound K'.

**Backward:** If a supersequence w of length <= K' exists, the positions of vertex-symbols in w correspond to a subset S of V that covers every edge (since each edge-string is a subsequence of w, requiring at least one endpoint to be represented), and |S| <= K.

## Validation Method

- Target problem (ShortestCommonSupersequence) does not exist in codebase yet — implement model first
- Reduction type: polynomial (Karp)

## Example

> ⚠️ **Unverified** — AI-generated content, not directly from source text

- NP-complete even if |Sigma| = 5.
- Solvable in polynomial time if |R| = 2 (via longest common subsequence) or if all strings have length <= 2.
- Reference: [Maier, 1978].
