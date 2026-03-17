---
name: Problem
about: Propose a new problem type
title: "[Model] GroupingBySwapping"
labels: model
assignees: ''
---

## Motivation

GROUPING BY SWAPPING (P169) from Garey & Johnson, A4 SR21. A classical NP-complete problem concerning the minimum number of adjacent symbol interchanges needed to rearrange a string so that all occurrences of each symbol form a single contiguous block. This is closely related to sorting problems and token swapping on graphs, with connections to the colored token swapping problem on paths.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **R115:** FEEDBACK EDGE SET -> GROUPING BY SWAPPING (this is the GJ reference reduction)

## Definition

**Name:** <!-- ⚠️ Unverified --> `GroupingBySwapping`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Grouping by Swapping (also: Block Sorting by Adjacent Transpositions, Colored Token Swapping on a Path)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR21

**Mathematical definition:**

INSTANCE: Finite alphabet Sigma, string x in Sigma*, and a positive integer K.
QUESTION: Is there a sequence of K or fewer adjacent symbol interchanges that converts x into a string y in which all occurrences of each symbol a in Sigma are in a single block, i.e., y has no subsequences of the form aba for a,b in Sigma and a != b?

The problem is a decision (satisfaction) problem: the answer is YES or NO depending on whether x can be grouped with at most K adjacent swaps.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** The search is over sequences of at most K adjacent transpositions. Alternatively, one can view the solution as a permutation of the string positions, with the constraint that the permutation can be decomposed into at most K adjacent transpositions.
- **Per-variable domain:** Each operation specifies a position i in {0, ..., |x|-2} to swap positions i and i+1.
- **Meaning:** A sequence of swap operations (i_1, i_2, ..., i_t) with t <= K, where each i_j indicates swapping positions i_j and i_j+1 in the current string. The sequence is valid if the resulting string has all occurrences of each symbol contiguous.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `GroupingBySwapping`
**Variants:** none (no graph or weight type parameter; operates on strings over a finite alphabet)

| Field | Type | Description |
|-------|------|-------------|
| `alphabet_size` | `usize` | Size of the finite alphabet Sigma (symbols indexed 0..alphabet_size) |
| `string` | `Vec<usize>` | The input string x, encoded as a sequence of symbol indices |
| `budget` | `usize` | The budget K: maximum number of adjacent swaps allowed |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Howell, 1977; transformation from FEEDBACK EDGE SET).
- **Best known exact algorithm:** Brute-force enumeration of all swap sequences of length at most K in O((|x|-1)^K) time, checking if the result is grouped. For small alphabet sizes, dynamic programming on the relative order of symbol blocks may reduce the search space, but the problem remains NP-hard in general.
- **Related tractable cases:**
  - For a 2-symbol alphabet (binary), the problem is equivalent to counting inversions between the two groups and is solvable in O(|x| log |x|) time.
  - The c-Colored Token Swapping problem on a path is NP-complete for c >= 3 colors (Bonnet et al., 2018), which is essentially the same problem.
  - On special graph topologies (stars, complete graphs), token swapping is polynomial.
- **Parameterized:** The problem is fixed-parameter tractable when parameterized by the alphabet size |Sigma| (since the number of possible block orderings is |Sigma|!, and for each ordering the minimum swaps can be computed).
- **References:**
  - [Howell, 1977] T. D. Howell, "Grouping by swapping is NP-complete".
  - [Bonnet et al., 2018] E. Bonnet et al., "Complexity of Token Swapping and Its Variants", Algorithmica 80(9), pp. 2535-2571.

## Extra Remark

**Full book text:**

INSTANCE: Finite alphabet Sigma, string x in Sigma*, and a positive integer K.
QUESTION: Is there a sequence of K or fewer adjacent symbol interchanges that converts x into a string y in which all occurrences of each symbol a in Sigma are in a single block, i.e., y has no subsequences of the form aba for a,b in Sigma and a != b?
Reference: [Howell, 1977]. Transformation from FEEDBACK EDGE SET.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all sequences of at most K adjacent swaps on x; for each resulting string, check if all occurrences of each symbol are contiguous (no "aba" pattern).
- [ ] It can be solved by reducing to integer programming.
- [x] Other: For fixed alphabet size c, enumerate all c! possible block orderings; for each ordering, compute the minimum adjacent transpositions needed (equivalent to counting inversions for that target permutation) and check if any achieves cost <= K.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Alphabet Sigma = {a, b, c} (alphabet_size = 3)
String x = "abcbacba" (length 8)
Budget K = 6

**Symbol positions in x:**
- a: positions 0, 4, 7
- b: positions 1, 3, 6
- c: positions 2, 5

**Target: group all symbols into contiguous blocks.**
One possible target ordering: all a's, then all b's, then all c's -> "aaabbbcc"
Another ordering: "ccbbbaa", etc. There are 3! = 6 possible block orderings.

**Step-by-step solution (grouping to "aabbbcca" then to "aaabbbcc"):**
1. Start: a b c b a c b a
2. Swap pos 3,4 (b,a): a b c a a c b a -- wait, this doesn't group well. Let's try a different approach.

**Better approach -- target "aaa bbb cc":**
1. Start: a b c b a c b a (positions: a=0,4,7; b=1,3,6; c=2,5)
2. Swap pos 1,2 (b,c): a c b b a c b a
3. Swap pos 0,1 (a,c): c a b b a c b a
4. Swap pos 4,5 (a,c): c a b b c a b a
5. Swap pos 5,6 (a,b): c a b b c b a a
6. Swap pos 4,5 (c,b): c a b b b c a a
7. Swap pos 1,2 (a,b): c b a b b c a a -- not converging well.

**Simpler verified example:**
String x = "abcabc" (length 6), Budget K = 5
- a: pos 0, 3; b: pos 1, 4; c: pos 2, 5
- Target "aabbcc":
  1. "abcabc" -> swap(2,3): "abacbc"
  2. "abacbc" -> swap(1,2): "aabcbc"
  3. "aabcbc" -> swap(3,4): "aabcbc" wait, swap(3,4): "aabbcc"? No: "aab c bc" -> swap pos 3,4 (c,b): "aabbcc". Yes!
- Total: 3 swaps. 3 <= 5 = K. Answer: YES.

**Minimum is 3 swaps.** No sequence of 2 or fewer swaps can group "abcabc" into contiguous blocks, since at least 3 crossings must be resolved (a3 crosses b1,c2; b4 crosses c2).
