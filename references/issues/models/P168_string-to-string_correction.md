---
name: Problem
about: Propose a new problem type
title: "[Model] StringToStringCorrection"
labels: model
assignees: ''
---

## Motivation

STRING-TO-STRING CORRECTION (P168) from Garey & Johnson, A4 SR20. A classical NP-complete problem concerning the minimum-cost transformation of one string into another using only deletion and adjacent-symbol interchange operations. While the standard edit distance (with insert, delete, change) is solvable in polynomial time via dynamic programming (Wagner-Fischer algorithm), restricting the operation set to only deletions and adjacent swaps makes the problem NP-complete for unbounded alphabets.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **R114:** SET COVERING -> STRING-TO-STRING CORRECTION (this is the GJ reference reduction)

## Definition

**Name:** <!-- ⚠️ Unverified --> `StringToStringCorrection`
**Canonical name:** <!-- ⚠️ Unverified: web search --> String-to-String Correction (also: Extended Edit Distance with Swaps and Deletions)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR20

**Mathematical definition:**

INSTANCE: Finite alphabet Sigma, two strings x,y in Sigma*, and a positive integer K.
QUESTION: Is there a way to derive the string y from the string x by a sequence of K or fewer operations of single symbol deletion or adjacent symbol interchange?

The problem is a decision (satisfaction) problem: the answer is YES or NO depending on whether the restricted edit distance (using only swap and delete) from x to y is at most K.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** K (the maximum number of operations; alternatively, the search space is over all sequences of at most K operations, each being a deletion at some position or a swap of adjacent positions)
- **Per-variable domain:** Each operation is either a deletion (specifying a position in the current string) or an adjacent swap (specifying a position i to swap positions i and i+1)
- **Meaning:** A sequence of operations (o_1, o_2, ..., o_t) with t <= K, where each o_j is applied to the current intermediate string. The sequence is valid if applying all operations in order to x produces y.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `StringToStringCorrection`
**Variants:** none (no graph or weight type parameter; operates on strings over a finite alphabet)

| Field | Type | Description |
|-------|------|-------------|
| `alphabet_size` | `usize` | Size of the finite alphabet Sigma (symbols indexed 0..alphabet_size) |
| `source` | `Vec<usize>` | The source string x, encoded as a sequence of symbol indices |
| `target` | `Vec<usize>` | The target string y, encoded as a sequence of symbol indices |
| `budget` | `usize` | The budget K: maximum number of operations allowed |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete for unbounded alphabet size (Wagner, 1975; transformation from SET COVERING).
- **Best known exact algorithm:** The CELLAR algorithm by Wagner solves the extended problem in time O(|x| * |y| * |Sigma|^(s^2)) where s = min(4*W_C, W_I + W_D) / W_S + 1 and W_C, W_I, W_D, W_S are operation costs. For the restricted swap-delete-only variant, brute-force enumeration of operation sequences gives O(K! * (|x| + |y|)^K) in the worst case, or O(2^|x| * |x|^2) by considering all possible subsequences and permutation orderings.
- **Special polynomial cases:**
  - If insert and change operations are also allowed (even without swap), solvable in O(|x| * |y|) time (Wagner-Fischer, 1974).
  - If only adjacent swap is allowed (no delete), solvable in polynomial time (Wagner, 1975) -- equivalent to counting inversions.
  - Binary alphabet: some restricted cases are polynomial (Meister, 2015).
- **References:**
  - [Wagner, 1975] R. A. Wagner, "On the complexity of the extended string-to-string correction problem", Proc. 7th STOC, pp. 218-223.
  - [Wagner and Fischer, 1974] R. A. Wagner and M. J. Fischer, "The string-to-string correction problem", JACM 21, pp. 168-173.

## Extra Remark

**Full book text:**

INSTANCE: Finite alphabet Sigma, two strings x,y in Sigma*, and a positive integer K.
QUESTION: Is there a way to derive the string y from the string x by a sequence of K or fewer operations of single symbol deletion or adjacent symbol interchange?
Reference: [Wagner, 1975]. Transformation from SET COVERING.
Comment: Solvable in polynomial time if the operation set is expanded to include the operations of changing a single character and of inserting a single character, even if interchanges are not allowed (e.g., see [Wagner and Fischer, 1974]), or if the only operation is adjacent symbol interchange [Wagner, 1975]. See reference for related results for cases in which different operations can have different costs.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all sequences of at most K operations (each being a delete or adjacent swap at some position), apply each sequence to x, and check whether the result equals y.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: For small alphabet or special cases, dynamic programming approaches exist (CELLAR algorithm).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Alphabet Sigma = {a, b, c, d} (alphabet_size = 4)
Source string x = "abcdba" (length 6)
Target string y = "abdcb" (length 5)
Budget K = 3

**Step-by-step solution (one possible sequence of 3 operations):**
1. Start: "abcdba"
2. Swap positions 2 and 3 (c and d): "abdcba"
3. Delete position 5 (a): "abdcb"
4. Result: "abdcb" = y

Total operations: 2 (swap + delete), which is <= K = 3. Answer: YES.

**Verification that fewer operations are insufficient:**
- With 0 operations: "abcdba" != "abdcb" (different length and content)
- With 1 operation: a single delete can remove one character from a 6-char string to get a 5-char string, but no single position deletion of "abcdba" yields "abdcb". A single swap keeps length 6, which cannot equal the 5-char target. So 1 operation is insufficient.
- With 2 operations: as shown above, 2 operations suffice (1 swap + 1 delete). The minimum cost is 2.
