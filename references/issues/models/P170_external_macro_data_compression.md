---
name: Problem
about: Propose a new problem type
title: "[Model] ExternalMacroDataCompression"
labels: model
assignees: ''
---

## Motivation

EXTERNAL MACRO DATA COMPRESSION (P170) from Garey & Johnson, A4 SR22. A classical NP-complete problem in data compression theory, where the goal is to compress a string using a separate dictionary string and a compressed string with pointers. This problem formalizes the macro model of data compression introduced by Storer and Szymanski, which generalizes many practical compression schemes including LZ-family algorithms.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **R116:** VERTEX COVER -> EXTERNAL MACRO DATA COMPRESSION (this is the GJ reference reduction)

## Definition

**Name:** <!-- ⚠️ Unverified --> `ExternalMacroDataCompression`
**Canonical name:** <!-- ⚠️ Unverified: web search --> External Macro Data Compression (also: External Pointer Macro Compression, EPM Compression)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR22

**Mathematical definition:**

INSTANCE: Alphabet Sigma, string s in Sigma*, pointer cost h in Z+, and a bound B in Z+.
QUESTION: Are there strings D (dictionary string) and C (compressed string) in (Sigma union {p_i: 1 <= i <= |s|})*, where the symbols p_i are "pointers," such that
|D| + |C| + (h-1) * (number of occurrences of pointers in D and C) <= B
and such that there is a way of identifying pointers with substrings of D so that s can be obtained from C by repeatedly replacing pointers in C by their corresponding substrings in D?

The problem is a decision (satisfaction) problem: the answer is YES or NO depending on whether the string s can be compressed within the given cost bound B.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** The search is over all possible pairs (D, C) of dictionary and compressed strings. The number of variables depends on the lengths |D| and |C| (which are themselves part of the optimization), and each position can be either an alphabet symbol or a pointer.
- **Per-variable domain:** At each position, the choice is from Sigma union {pointers}. The pointer at position i specifies a (start, length) pair into D.
- **Meaning:** The pair (D, C) encodes a compression scheme. D is a dictionary of reusable substrings; C is the compressed representation that references D via pointers. The total cost is |D| + |C| + (h-1) * (number of pointer symbols in D and C).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ExternalMacroDataCompression`
**Variants:** none (no graph or weight type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `alphabet_size` | `usize` | Size of the alphabet Sigma (symbols indexed 0..alphabet_size) |
| `string` | `Vec<usize>` | The source string s to be compressed, as a sequence of symbol indices |
| `pointer_cost` | `usize` | The pointer cost h (cost per pointer occurrence is h, contributing h-1 extra beyond the position it occupies) |
| `bound` | `usize` | The compression bound B |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Storer, 1977; Storer and Szymanski, 1978; transformation from VERTEX COVER). Remains NP-complete even for fixed h >= 2, for alphabet size >= 3 with pointer cost ceiling(h * log|s|), and for variants where D contains no pointers or pointers cannot refer to overlapping strings.
- **Best known exact algorithm:** Brute-force over all possible (D, C) pairs: for each candidate dictionary string D of length up to |s|, enumerate all compressed strings C that can reconstruct s via D. The search space is exponential in |s|. Upper bound: O(|Sigma|^|s| * 2^|s|) by enumerating dictionary contents and pointer placements.
- **Approximation:** Practical heuristic algorithms (LZ77, LZSS, LZ78) achieve good compression ratios in linear or near-linear time but do not guarantee optimality. LZSS (Lempel-Ziv-Storer-Szymanski) is a direct practical algorithm derived from this theoretical framework.
- **References:**
  - [Storer, 1977] J. A. Storer, "NP-completeness results concerning data compression", Tech. Report 234, Princeton University.
  - [Storer and Szymanski, 1978] J. A. Storer and T. G. Szymanski, "The macro model for data compression", Proc. 10th STOC, pp. 30-39.
  - [Storer and Szymanski, 1982] J. A. Storer and T. G. Szymanski, "Data compression via textual substitution", JACM 29(4), pp. 928-951.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is related to:** INTERNAL MACRO DATA COMPRESSION (P171) -- the variant where D and C are merged into a single self-referencing string
- **Generalization of:** Many practical compression schemes (LZ77, LZSS) are restricted forms of external macro compression

## Extra Remark

**Full book text:**

INSTANCE: Alphabet Sigma, string s in Sigma*, pointer cost h in Z+, and a bound B in Z+.
QUESTION: Are there strings D (dictionary string) and C (compressed string) in (Sigma union {p_i: 1 <= i <= |s|})*, where the symbols p_i are "pointers," such that
|D| + |C| + (h-1) * (number of occurrences of pointers in D and C) <= B
and such that there is a way of identifying pointers with substrings of D so that S can be obtained from C by repeatedly replacing pointers in C by their corresponding substrings in D?
Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if h is any fixed integer 2 or greater. Many variants, including those in which D can contain no pointers and/or no pointers can refer to overlapping strings, are also NP-complete. If the alphabet size is fixed at 3 or greater, and the pointer cost is ceiling(h * log|s|), the problem is also NP-complete. For further variants, including the case of "original pointers," see references.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible dictionary strings D up to length |s|, and for each D, enumerate all compressed strings C using alphabet symbols and pointers into D, checking whether C decodes to s and the total cost is <= B.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Practical heuristic compression algorithms (LZSS, LZ77) provide approximate solutions in linear time, though they do not guarantee optimality.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Alphabet Sigma = {a, b, c} (alphabet_size = 3)
String s = "abcabcabc" (length 9)
Pointer cost h = 2
Bound B = 10

**Analysis:**
- Uncompressed cost: |s| = 9 (no dictionary, no pointers)
- The string has a repeating pattern "abc" appearing 3 times.

**Compression scheme:**
- Dictionary D = "abc" (length 3)
- Compressed string C = "p1 p1 p1" where p1 points to the substring "abc" in D
  - C has 3 pointer symbols, length |C| = 3
- Total cost = |D| + |C| + (h-1) * (pointer count) = 3 + 3 + (2-1) * 3 = 3 + 3 + 3 = 9

That achieves cost 9, which is not better than the original. Let's try a different scheme:
- Dictionary D = "abc" (length 3)
- Compressed string C = "p1 p1 abc" (two pointers + literal copy)
  - |C| = 2 + 3 = 5, pointer count = 2
- Total cost = 3 + 5 + 1 * 2 = 10 = B. This meets the bound.

**Better scheme:**
- Dictionary D = "abcabc" (length 6)
- Compressed string C = "p1 abc" where p1 points to "abcabc"
  - |C| = 1 + 3 = 4, pointer count = 1
- Total cost = 6 + 4 + 1 * 1 = 11 > B. Too expensive.

**Optimal at B = 9 (uncompressed).** For B = 10, the scheme D = "abc", C = "p1 p1 abc" works.

Actually, let's be more careful:
- D = "abc", C = "p1p1p1" (3 pointers each referencing "abc")
- Decoding: replace each p1 with "abc" -> "abcabcabc" = s
- Cost = |D| + |C| + (h-1) * 3 = 3 + 3 + 3 = 9 <= 10 = B. Answer: YES.
