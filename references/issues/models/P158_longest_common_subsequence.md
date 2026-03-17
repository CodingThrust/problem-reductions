---
name: Problem
about: Propose a new problem type
title: "[Model] LongestCommonSubsequence"
labels: model
assignees: ''
---

## Motivation

LONGEST COMMON SUBSEQUENCE (P158) from Garey & Johnson, A4 SR10. A fundamental problem in string algorithms with deep connections to edit distance, diff utilities, and bioinformatics (sequence alignment). For |R| = 2, solvable in O(n^2) by classic dynamic programming (Wagner and Fischer, 1974). For an arbitrary number of strings, NP-complete even over a binary alphabet |Σ| = 2 (Maier, 1978). Dual to Shortest Common Supersequence: for two strings, SCS length = |x| + |y| - LCS length. The analogous LONGEST COMMON SUBSTRING problem (contiguous) is trivially solvable in polynomial time.

**Associated rules:**
- R104: MinimumVertexCover → LongestCommonSubsequence (as target)

## Definition

**Name:** `LongestCommonSubsequence`
**Canonical name:** LONGEST COMMON SUBSEQUENCE
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR10

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≥ K such that w is a subsequence of each string x ∈ R?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** K variables (one per position in the candidate common subsequence w), where K is the length bound.
- **Per-variable domain:** {0, 1, ..., |Σ|-1} — index into the alphabet Σ.
- **Meaning:** Variable i encodes the symbol at position i of the candidate common subsequence w. A satisfying assignment produces a string w of length ≥ K such that w is a subsequence of every string in R. That is, for each x ∈ R, there exist indices 1 ≤ i_1 < i_2 < ... < i_K ≤ |x| with x[i_j] = w[j] for all j.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `LongestCommonSubsequence`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `alphabet_size` | `usize` | Size of the alphabet |Σ| |
| `strings` | `Vec<Vec<usize>>` | Set R of input strings, each encoded as a vector of alphabet indices |
| `bound` | `usize` | Length bound K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- A string w is a subsequence of x if w can be obtained by deleting zero or more characters from x (not necessarily contiguous).
- For the optimization version, maximize |w| subject to w being a subsequence of all strings in R.
- Dual to ShortestCommonSupersequence: for two strings x, y, LCS(x,y) + SCS(x,y) = |x| + |y|.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:**
  - For |R| = 2: O(n · m) DP where n = |x_1|, m = |x_2| (classic Needleman-Wunsch / Wagner-Fischer). Subquadratic algorithms exist: O(n^2 / log^2 n) (Masek and Paterson, 1980). Conditional lower bound: no truly subquadratic O(n^{2-ε}) algorithm unless SETH fails.
  - For fixed |R| = k: O(n^k) multi-dimensional DP, where n = max string length.
  - For arbitrary |R|: NP-hard. The dominant-point approach with divide-and-conquer heuristics is used in practice.
- **Approximation:** Hard to approximate within ratio n^{1-ε} for any ε > 0 unless P = NP (for arbitrary |R|).
- **NP-completeness:** NP-complete (Maier, 1978). Remains NP-complete even if |Σ| = 2.
- **Polynomial cases:**
  - Fixed |R|: O(n^|R|) DP.
  - Fixed K: O(n^K · |R|) by checking all length-K subsequences.
  - |R| = 2: O(n · m) classical DP.
- **References:**
  - David Maier (1978). "The complexity of some problems on subsequences and supersequences". *JACM* 25(2):322-336.
  - Robert A. Wagner and Michael J. Fischer (1974). "The string-to-string correction problem". *JACM* 21:168-173.
  - William J. Masek and Michael S. Paterson (1980). "A faster algorithm computing string edit distances". *J. Comput. Syst. Sci.* 20(1):18-31.

## Extra Remark

**Full book text:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≥ K such that w is a subsequence of each x ∈ R?
Reference: [Maier, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if |Σ| = 2. Solvable in polynomial time for any fixed K or for fixed |R| (by dynamic programming, e.g., see [Wagner and Fischer, 1974]). The analogous LONGEST COMMON SUBSTRING problem is trivially solvable in polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all strings w ∈ Σ* with |w| = K and check if w is a subsequence of each x ∈ R.
- [x] It can be solved by reducing to integer programming — encode matching positions as integer variables with ordering constraints.
- [x] Other: For |R| = 2, classic DP in O(n · m) time. For fixed |R|, multi-dimensional DP in O(n^|R|). For arbitrary |R|, dominant-point algorithms, A* search, or branch-and-bound. Also related to maximum clique in a "match graph" formulation.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, binary alphabet, 6 strings):**
- Alphabet: Σ = {0, 1} (|Σ| = 2)
- Strings: R = {"010110", "100101", "001011", "110010", "010101", "101010"}
- Bound K = 3
- Candidate LCS: w = "010"
  - "010" ⊆ "010110"? 0(1)1(2)0(3) -- wait, w = "010": 0(pos1), 1(pos3), 0(pos4) ✓
  - "010" ⊆ "100101"? 0(pos2), 1(pos4), 0(pos5) ✓
  - "010" ⊆ "001011"? 0(pos1), 1(pos4), 0(-- need 0 after pos 4) -- "001011" = 0,0,1,0,1,1. 0(pos1), 1(pos3), 0(pos4) ✓
  - "010" ⊆ "110010"? 0(pos3), 1(pos4)... wait: "110010" = 1,1,0,0,1,0. 0(pos3), 1(pos5), 0(pos6) ✓
  - "010" ⊆ "010101"? 0(pos1), 1(pos2), 0(pos3) ✓
  - "010" ⊆ "101010"? 0(pos2), 1(pos3), 0(pos4) ✓
- Answer: YES

**Instance 2 (YES, ternary alphabet, 6 strings, verified):**
- Alphabet: Σ = {a, b, c} (|Σ| = 3)
- Strings: R = {"abcabc", "bacbac", "cabcab", "abcbac", "bcabca", "acbacb"}
- Bound K = 3
- Candidate LCS: w = "abc"
  - "abc" ⊆ "abcabc"? a(1)b(2)c(3) ✓
  - "abc" ⊆ "bacbac"? a(2)... wait: "bacbac" = b,a,c,b,a,c. a(pos2), b(pos4), c(pos6) ✓
  - "abc" ⊆ "cabcab"? a(pos3), b(pos4), c(-- need c after pos 4): "cabcab" = c,a,b,c,a,b. a(pos2), b(pos3), c(pos4) ✓
  - "abc" ⊆ "abcbac"? a(pos1), b(pos2), c(pos3) ✓
  - "abc" ⊆ "bcabca"? a(pos3), b(pos4), c(pos5) ✓
  - "abc" ⊆ "acbacb"? a(pos1), -- need b after pos1: b(pos3), c(pos4)... "acbacb" = a,c,b,a,c,b. a(pos1), b(pos3), c(pos5) ✓ (but w = "abc" needs a then b then c in order: a(1), c is at 2 but we need b first... a(pos1), b(pos3), c(pos5) ✓ since 1 < 3 < 5)
- Candidate LCS: w = "bac"
  - "bac" ⊆ "abcabc"? b(pos2), a(pos4), c(pos6) ✓
  - "bac" ⊆ "bacbac"? b(pos1), a(pos2), c(pos3) ✓
  - "bac" ⊆ "cabcab"? -- "cabcab" = c,a,b,c,a,b. b(pos3), a(pos5), -- need c after pos5: no c after pos5. b(pos6)... only one b after a. Actually b(pos3), a(pos5), but no c after pos 5. Try b(pos6)? only one b at pos 3 and 6. b(pos6), but then need a after 6 -- fails.
  - "bac" does not work for "cabcab".
- The LCS "abc" works for all 6 strings with K = 3.
- Answer: YES

**Instance 3 (NO):**
- Alphabet: Σ = {a, b} (|Σ| = 2)
- Strings: R = {"aaa", "bbb", "aba", "bab", "aab", "bba"}
- Bound K = 2
- Any common subsequence of length 2 must use symbols from {a, b}. Check w = "aa": not subsequence of "bbb" ✗. w = "bb": not subsequence of "aaa" ✗. w = "ab": "ab" ⊆ "bbb"? No 'a' ✗. w = "ba": "ba" ⊆ "aaa"? No 'b' ✗.
- Even K = 1: w = "a" ⊆ "bbb"? ✗. w = "b" ⊆ "aaa"? ✗.
- No common subsequence of length ≥ 1 exists (strings "aaa" and "bbb" share no common symbols in common subsequence terms -- wait, both are over {a,b}, but "aaa" contains no 'b' and "bbb" contains no 'a'. So LCS("aaa", "bbb") = empty string.
- Answer: NO

**Instance 4 (YES, non-trivial with 8 strings):**
- Alphabet: Σ = {0, 1} (|Σ| = 2)
- Strings: R = {"01100110", "10011001", "01010101", "10101010", "00110011", "11001100", "01001011", "10110100"}
- Bound K = 3
- Candidate LCS: w = "010"
  - "010" ⊆ "01100110"? 0(1),1(2),0(4) ✓
  - "010" ⊆ "10011001"? 0(2),1(4),0(5) ✓
  - "010" ⊆ "01010101"? 0(1),1(2),0(3) ✓
  - "010" ⊆ "10101010"? 0(2),1(3),0(4) ✓
  - "010" ⊆ "00110011"? 0(1),1(3),0(5) ✓
  - "010" ⊆ "11001100"? 0(3),1(5),0(7) ✓
  - "010" ⊆ "01001011"? 0(1),1(5),0(-- need 0 after pos5): "01001011" = 0,1,0,0,1,0,1,1. 0(1),1(2),0(3) ✓
  - "010" ⊆ "10110100"? 0(3),1(-- need 1 after 3): "10110100" = 1,0,1,1,0,1,0,0. 0(2),1(3),0(5) ✓
- Answer: YES
