---
name: Problem
about: Propose a new problem type
title: "[Model] ShortestCommonSuperstring"
labels: model
assignees: ''
---

## Motivation

SHORTEST COMMON SUPERSTRING (P157) from Garey & Johnson, A4 SR9. A fundamental NP-complete problem in string algorithms and bioinformatics. Given a set of strings, find the shortest string containing each input string as a contiguous substring. This problem is central to genome assembly (reconstructing a genome from short sequencing reads), data compression, and database optimization. Different from Shortest Common Supersequence (P156), which requires subsequence containment (non-contiguous). Proved NP-complete by Maier and Storer (1977) via reduction from VERTEX COVER on cubic graphs, even for binary alphabet. The problem is APX-hard with the best known approximation factor of 2.475.

**Associated rules:**
- R103: MinimumVertexCover (cubic graphs) → ShortestCommonSuperstring (as target)

## Definition

**Name:** `ShortestCommonSuperstring`
**Canonical name:** SHORTEST COMMON SUPERSTRING
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR9

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a substring of w, i.e., w = w_0 x w_1 where w_0, w_1 ∈ Σ* (x appears contiguously in w)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** K variables (one per position in the candidate superstring w), where K is the length bound.
- **Per-variable domain:** {0, 1, ..., |Σ|-1} — index into the alphabet Σ.
- **Meaning:** Variable i encodes the symbol at position i of the candidate superstring w. A satisfying assignment produces a string w of length ≤ K such that every string in R appears as a contiguous substring. Equivalently, one can model this as choosing a permutation of the strings and their overlap amounts, analogous to the asymmetric TSP.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `ShortestCommonSuperstring`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `alphabet_size` | `usize` | Size of the alphabet |Σ| |
| `strings` | `Vec<Vec<usize>>` | Set R of input strings, each encoded as a vector of alphabet indices |
| `bound` | `usize` | Length bound K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- A string x is a substring of w if x appears contiguously in w (stricter than subsequence).
- Without loss of generality, one may assume no string in R is a substring of another (otherwise remove it).
- The optimization version seeks to minimize |w|.
- Can be modeled as an asymmetric TSP on the "overlap graph" of the strings.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O(n^2 · 2^n) via Bellman-Held-Karp style DP on the overlap graph, where n = |R|. The problem is equivalent to finding a minimum-weight Hamiltonian path in the overlap graph (asymmetric TSP variant). For |R| = 2, solvable in O(|x_1| + |x_2|) time.
- **Approximation:**
  - Best known approximation ratio: 2.475 (Mucha, 2013; Paluch, 2014).
  - Greedy algorithm (repeatedly merge the pair with maximum overlap): conjectured 2-approximate, proved 4-approximate (Blum et al., 1994), improved to 3.5-approximate (Kaplan et al., 2005).
  - APX-hard: no PTAS unless P = NP.
- **NP-completeness:** NP-complete (Maier and Storer, 1977; Gallant, Maier, and Storer, 1980). Remains NP-complete even if |Σ| = 2, or if all strings have |x| ≤ 8 with no repeated symbols.
- **Polynomial cases:** Solvable in polynomial time if all strings have length ≤ 2.
- **References:**
  - David Maier and James A. Storer (1977). "A note on the complexity of the superstring problem". Technical Report 233, Princeton University.
  - John Gallant, David Maier, and James A. Storer (1980). "On finding minimal length superstrings". *J. Comput. Syst. Sci.* 20(1):50-58.
  - Avrim Blum, Tao Jiang, Ming Li, John Tromp, and Mihalis Yannakakis (1994). "Linear approximation of shortest superstrings". *JACM* 41(4):630-647.

## Extra Remark

**Full book text:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a substring of w, i.e., w = w0xw1 where each wi ∈ Σ*?
Reference: [Maier and Storer, 1977]. Transformation from VERTEX COVER for cubic graphs.
Comment: Remains NP-complete even if |Σ| = 2 or if all x ∈ R have |x| ≤ 8 and contain no repeated symbols. Solvable in polynomial time if all x ∈ R have |x| ≤ 2.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all strings w ∈ Σ* with |w| ≤ K and check if each x ∈ R appears as a contiguous substring.
- [x] It can be solved by reducing to integer programming — model as asymmetric TSP on the overlap graph with ordering constraints.
- [x] Other: Bellman-Held-Karp DP in O(n^2 · 2^n) via overlap graph / asymmetric TSP formulation. Greedy overlap algorithm (practical, 4-approximate). In bioinformatics, solved heuristically via de Bruijn graphs or overlap-layout-consensus pipelines.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, binary alphabet):**
- Alphabet: Σ = {0, 1} (|Σ| = 2)
- Strings: R = {"01101", "10110", "11010", "01011", "10101", "11001"}
- Bound K = 12
- Candidate superstring: w = "011010110011" (length 12)? Let me verify substrings need not appear but let me try a different arrangement.
- Overlap analysis:
  - "01101" and "10110" overlap by 4: "01101" + "0" = "011010", but "10110" starts with "1011" matching suffix of "01101"... "01101" suffix "1101", "10110" prefix "1011" -- overlap 0. Let me try: suffix of "10110" = "0110", prefix of "01101" = "0110" -- overlap 4.
- Let me construct a simpler verified example.

**Instance 1 (YES, ternary alphabet, verified):**
- Alphabet: Σ = {a, b, c} (|Σ| = 3)
- Strings: R = {"abc", "bca", "cab", "bcc", "cca", "aab"}
- No string is a substring of another ✓
- Overlaps: "aab" → "abc" (overlap 2: "ab"), "abc" → "bca" (overlap 2: "bc"), "bca" → "cab" (overlap 2: "ca"), "cab" → "bcc" (overlap 1: "b"), but "cab" ends "ab", "bcc" starts "b" -- overlap 1. "bcc" → "cca" (overlap 2: "cc"), "cca" → "aab" (overlap 1: "a").
- Greedy chaining: aab → abc → bca → cab → bcc → cca
  - "aab" + "c" (overlap 2) = "aabc"
  - "aabc" + "a" (overlap 2) = "aabca"
  - "aabca" + "b" (overlap 2) = "aabcab"
  - "aabcab" + "cc" (overlap 1) = "aabcabcc"
  - "aabcabcc" + "a" (overlap 2) = "aabcabcca"
  - Length 9. Check all substrings: "abc" at pos 2 ✓, "bca" at pos 3 ✓, "cab" at pos 4 ✓, "bcc" at pos 6 ✓, "cca" at pos 7 ✓, "aab" at pos 1 ✓.
- w = "aabcabcca", |w| = 9.
- Bound K = 9.
- Answer: YES

**Instance 2 (YES, binary alphabet, verified):**
- Alphabet: Σ = {0, 1}
- Strings: R = {"001", "011", "110", "100", "010", "101"}
- No string is a substring of another ✓
- These are all binary strings of length 3 except "000" and "111".
- A de Bruijn sequence B(2,3) = "0011101001..." but we need all length-3 substrings. The full de Bruijn sequence of order 3 over {0,1} is "0001011100" (length 8) containing all 8 binary 3-strings. We only need 6, so we can do shorter.
- Try w = "001011010" (length 9): substrings of length 3: 001, 010, 101, 011, 110, 101, 010. Contains: 001✓, 010✓, 101✓, 011✓, 110✓. Missing: 100.
- Try w = "0010110100" (length 10): substrings: 001, 010, 101, 011, 110, 101, 010, 100. All 6 present ✓.
- w = "0010110100", |w| = 10, K = 10.
- Can we do better? Overlap chain: 001→011 (overlap 2), 011→110 (overlap 2), 110→100 (overlap 1), 100→010 (overlap 0? "100" suffix "00", "010" prefix "01" -- overlap 1: "0"). 100→001 overlap 2. But 001 is already used. Try: 010→101 (overlap 2), 101→010 (overlap 2) -- cycle. Different order: 100→001→011→110→010→101. Overlaps: 1+2+2+1+2 = 8 savings. Total = 6·3 - 8 = 10. So minimum is 10.
- Bound K = 10.
- Answer: YES

**Instance 3 (NO):**
- Alphabet: Σ = {a, b}
- Strings: R = {"aab", "aba", "baa", "abb", "bab", "bba"}
- Bound K = 7
- Total length = 18. Maximum possible savings from overlaps ≤ 5·2 = 10. So minimum superstring length ≥ 8. With K = 7, the answer is NO.
- Answer: NO
