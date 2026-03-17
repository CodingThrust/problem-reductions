---
name: Problem
about: Propose a new problem type
title: "[Model] InternalMacroDataCompression"
labels: model
assignees: ''
---

## Motivation

INTERNAL MACRO DATA COMPRESSION (P171) from Garey & Johnson, A4 SR23. A classical NP-complete problem in data compression theory, where the goal is to compress a string into a single self-referencing string with embedded pointers. Unlike the external variant (P170), there is no separate dictionary -- the compressed string C serves as both the dictionary and the output, with pointers referencing substrings within C itself.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **R117:** VERTEX COVER -> INTERNAL MACRO DATA COMPRESSION (this is the GJ reference reduction)

## Definition

**Name:** <!-- ⚠️ Unverified --> `InternalMacroDataCompression`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Internal Macro Data Compression (also: Internal Pointer Macro Compression, Self-Referencing Macro Compression)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR23

**Mathematical definition:**

INSTANCE: Alphabet Sigma, string s in Sigma*, pointer cost h in Z+, and a bound B in Z+.
QUESTION: Is there a single string C in (Sigma union {p_i: 1 <= i <= |s|})* such that
|C| + (h-1) * (number of occurrences of pointers in C) <= B
and such that there is a way of identifying pointers with substrings of C so that s can be obtained from C by using C as both compressed string and dictionary string?

The problem is a decision (satisfaction) problem: the answer is YES or NO depending on whether the string s can be internally compressed within the cost bound B.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** The search is over all possible strings C. The length of C is bounded by |s| (since the uncompressed string is always a valid but possibly suboptimal solution). Each position can be either an alphabet symbol or a pointer.
- **Per-variable domain:** At each position, the choice is from Sigma union {pointers}. A pointer at position i references a (start, length) pair within C itself (i.e., it points to a substring of C that has already been "decoded" or is decodable).
- **Meaning:** C is a single self-referencing compressed string. Pointers in C reference other substrings of C. The decoding process replaces pointers with their referenced substrings until no pointers remain, yielding s. The total cost is |C| + (h-1) * (pointer count).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `InternalMacroDataCompression`
**Variants:** none (no graph or weight type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `alphabet_size` | `usize` | Size of the alphabet Sigma (symbols indexed 0..alphabet_size) |
| `string` | `Vec<usize>` | The source string s to be compressed, as a sequence of symbol indices |
| `pointer_cost` | `usize` | The pointer cost h (cost per pointer occurrence is h, contributing h-1 extra) |
| `bound` | `usize` | The compression bound B |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Storer, 1977; Storer and Szymanski, 1978; transformation from VERTEX COVER). Remains NP-complete even for fixed h >= 2.
- **Best known exact algorithm:** Brute-force over all possible compressed strings C: enumerate strings over Sigma union {pointers} of length up to |s|, checking whether each C decodes to s and has cost <= B. The search space is exponential: O((|Sigma| + |s|)^|s|) candidates.
- **Approximation:** Practical algorithms like LZ77 and its variants (which use a sliding window as an "internal dictionary") approximate this problem. LZ78 also uses internal referencing. These run in O(|s|) or O(|s| log |s|) time but do not guarantee optimal compression.
- **Relationship to grammar compression:** Internal macro compression is closely related to the smallest grammar problem (finding the smallest context-free grammar generating exactly the string s), which is also NP-hard (Charikar et al., 2005).
- **References:**
  - [Storer, 1977] J. A. Storer, "NP-completeness results concerning data compression", Tech. Report 234, Princeton University.
  - [Storer and Szymanski, 1978] J. A. Storer and T. G. Szymanski, "The macro model for data compression", Proc. 10th STOC, pp. 30-39.
  - [Storer and Szymanski, 1982] J. A. Storer and T. G. Szymanski, "Data compression via textual substitution", JACM 29(4), pp. 928-951.
  - [Charikar et al., 2005] M. Charikar et al., "The smallest grammar problem", IEEE Trans. Inf. Theory 51(7), pp. 2554-2576.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is related to:** EXTERNAL MACRO DATA COMPRESSION (P170) -- the variant with a separate dictionary string
- **This is a special case of:** General macro compression (both external and internal variants are special cases of the unified macro model)

## Extra Remark

**Full book text:**

INSTANCE: Alphabet Sigma, string s in Sigma*, pointer cost h in Z+, and a bound B in Z+.
QUESTION: Is there a single string C in (Sigma union {p_i: 1 <= i <= |s|})* such that
|C| + (h-1) * (number of occurences of pointers in C) <= B
and such that there is a way of identifying pointers with substrings of C so that s can be obtained from C by using C as both compressed string and dictionary string in the manner indicated in the previous problem?
Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if h is any fixed integer 2 or greater. For other NP-complete variants (as in the previous problem), see references.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible strings C over Sigma union {pointers} of length up to |s|; for each C, check if it decodes to s (by resolving all pointer references within C) and compute the total cost |C| + (h-1) * (pointer count), accepting if cost <= B.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Practical compression algorithms (LZ77 with sliding window, LZ78) provide heuristic solutions in near-linear time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Alphabet Sigma = {a, b, c} (alphabet_size = 3)
String s = "abcabcabc" (length 9)
Pointer cost h = 2
Bound B = 8

**Analysis:**
- Uncompressed: C = "abcabcabc", |C| = 9, pointer count = 0, cost = 9.
- The string has a repeating pattern "abc" appearing 3 times.

**Internal compression scheme:**
- C = "abc p1 p1" where p1 references positions 0-2 of C (the substring "abc")
  - |C| = 3 + 1 + 1 = 5 (3 literal symbols + 2 pointer symbols)
  - Pointer count = 2
  - Cost = 5 + (2-1) * 2 = 5 + 2 = 7 <= 8 = B. Answer: YES.

**Decoding verification:**
- C = [a, b, c, p1, p1]
- p1 references C[0..3] = "abc"
- Replace p1 at position 3: "abc abc p1"
- Replace p1 at position 5 (now, after first expansion, p1 references C[0..3] = "abc"): "abc abc abc"
- Result: "abcabcabc" = s. Correct.

**Cost accounting:**
- |C| = 5 (length of the compressed string including pointer symbols)
- Number of pointer occurrences = 2
- Total cost = 5 + (2-1) * 2 = 7

**Can we do better (cost 6)?**
- C = "abc p1 p2" where p1 = C[0..3], p2 = C[0..3]: same as above, cost 7. No improvement with this structure.
- C = "abcabc p1" where p1 = C[0..6]: |C| = 7, pointers = 1, cost = 7 + 1 = 8. Same bound.
- Cost 7 appears to be optimal for h = 2 on this input.
