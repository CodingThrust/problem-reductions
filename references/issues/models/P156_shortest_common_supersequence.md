---
name: Problem
about: Propose a new problem type
title: "[Model] ShortestCommonSupersequence"
labels: model
assignees: ''
---

## Motivation

SHORTEST COMMON SUPERSEQUENCE (P156) from Garey & Johnson, A4 SR8. A fundamental NP-complete problem in string algorithms. Given a set of strings, find the shortest string that contains each input string as a subsequence (not necessarily contiguous). Different from Shortest Common Superstring (P157), which requires contiguous containment (substring). For two strings, SCS is solvable in polynomial time via the dual relationship with LCS (longest common subsequence). For an arbitrary number of strings, the problem becomes NP-complete even for |Σ| = 5 (Maier, 1978). Applications include data compression, sequence alignment, and scheduling.

**Associated rules:**
- R102: MinimumVertexCover → ShortestCommonSupersequence (as target)

## Definition

**Name:** `ShortestCommonSupersequence`
**Canonical name:** SHORTEST COMMON SUPERSEQUENCE
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR8

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a subsequence of w, i.e., x can be obtained from w by deleting zero or more characters (not necessarily contiguous)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** K variables (one per position in the candidate supersequence w), where K is the length bound.
- **Per-variable domain:** {0, 1, ..., |Σ|-1} — index into the alphabet Σ.
- **Meaning:** Variable i encodes the symbol at position i of the candidate supersequence w. A satisfying assignment produces a string w of length ≤ K such that every string in R is a subsequence of w. Alternatively, the problem can be viewed as choosing an interleaving order for the symbols of all input strings.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `ShortestCommonSupersequence`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `alphabet_size` | `usize` | Size of the alphabet |Σ| |
| `strings` | `Vec<Vec<usize>>` | Set R of input strings, each encoded as a vector of alphabet indices |
| `bound` | `usize` | Length bound K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- A string x is a subsequence of w if x can be obtained by deleting characters from w (characters need not be contiguous in w).
- Different from ShortestCommonSuperstring where containment is as a contiguous substring.
- For the optimization version, minimize |w| subject to the subsequence constraint.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:**
  - For |R| = 2: O(|x_1| · |x_2|) by DP via the dual LCS relationship: SCS(x_1, x_2) = |x_1| + |x_2| - LCS(x_1, x_2).
  - For fixed |R|: O(∏_{i=1}^{|R|} |x_i|) by multi-dimensional DP.
  - For arbitrary |R|: NP-hard. Exact algorithms are exponential. A* search with lower bounds has been applied. Brute force is O(|Σ|^K) to enumerate all candidate strings.
- **Approximation:** The greedy algorithm (repeatedly pick the symbol that satisfies the most constraints) gives an O(|Σ|)-approximation. No PTAS is known.
- **NP-completeness:** NP-complete (Maier, 1978). Remains NP-complete even if |Σ| = 5. Also NP-complete over binary alphabet (Räihä and Ukkonen, 1981).
- **Polynomial cases:** Solvable in polynomial time if |R| = 2 or if all strings have length ≤ 2.
- **References:**
  - David Maier (1978). "The complexity of some problems on subsequences and supersequences". *JACM* 25(2):322-336.
  - K. J. Räihä and E. Ukkonen (1981). "The shortest common supersequence problem over binary alphabet is NP-complete". *Theoretical Computer Science* 16(2):187-198.

## Extra Remark

**Full book text:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a subsequence of w, i.e., w = w0x1w1x2w2 ··· xkwk where each wi ∈ Σ* and x = x1x2 ··· xk?
Reference: [Maier, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if |Σ| = 5. Solvable in polynomial time if |R| = 2 (by first computing the largest common subsequence) or if all x ∈ R have |x| ≤ 2.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all strings w ∈ Σ* with |w| ≤ K and check the subsequence condition for each x ∈ R.
- [x] It can be solved by reducing to integer programming — encode position variables and ordering constraints as integer linear constraints.
- [x] Other: For |R| = 2, DP in O(|x_1|·|x_2|) via LCS duality. For general |R|, multi-dimensional DP or A* search with bounds. Also reducible to SHORTEST COMMON SUPERSTRING (which is strictly harder due to contiguity requirement).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, two strings over ternary alphabet):**
- Alphabet: Σ = {a, b, c} (|Σ| = 3)
- Strings: R = {"abcb", "bcab", "acba"}
- Bound K = 7
- Candidate supersequence: w = "abcacba" (length 7)
  - "abcb" ⊆ "a**b**c**a**c**b**a"? Check: a(1)b(2)c(3)b(6) -- positions 1,2,3,6 ✓
  - "bcab" ⊆ "abcacba"? Check: b(2)c(3)a(4)b(6) -- positions 2,3,4,6 ✓
  - "acba" ⊆ "abcacba"? Check: a(1)c(3)b(6)a(7) -- positions 1,3,6,7 ✓
- Answer: YES

**Instance 2 (YES, six strings over binary alphabet):**
- Alphabet: Σ = {0, 1} (|Σ| = 2)
- Strings: R = {"0110", "1010", "0101", "1100", "0011", "1001"}
- Bound K = 8
- Candidate supersequence: w = "01101001" (length 8)
  - "0110" ⊆ w? 0(1)1(2)1(3)0(4) ✓
  - "1010" ⊆ w? 1(2)0(4)1(5)0(6) ✓
  - "0101" ⊆ w? 0(1)1(2)0(4)1(5) ✓
  - "1100" ⊆ w? 1(2)1(3)0(4)0(6) ✓
  - "0011" ⊆ w? 0(1)0(4)1(5)1(-- need second 1 after position 5) -- w = 0,1,1,0,1,0,0,1, positions: 0(1)0(6)1(7)1(8)? length 8: 0(1)1(2)1(3)0(4)1(5)0(6)0(7)1(8). 0(1)0(6)1(5)... no, must be increasing. 0(4)0(6)1(-- no 1 after 7). Let me recheck w = "01101001": positions 0,1,1,0,1,0,0,1. "0011": 0(pos1)0(pos4)1(pos5)1(pos8) ✓
  - "1001" ⊆ w? 1(pos2)0(pos4)0(pos6)1(pos8) ✓
- Answer: YES

**Instance 3 (NO):**
- Alphabet: Σ = {a, b, c}
- Strings: R = {"abc", "bca", "cab", "acb", "bac", "cba"}
- Bound K = 5
- All 6 permutations of {a,b,c}. Any supersequence must contain each permutation as a subsequence. The minimum SCS of all permutations of 3 symbols has length 7 (= "abcabca" or similar). So K = 5 is insufficient.
- Answer: NO
