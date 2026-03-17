---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveSets"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE SETS (P166) from Garey & Johnson, A4 SR18. An NP-complete problem from the domain of storage and retrieval. Given a finite alphabet and a collection of subsets, the question is whether there exists a short string over the alphabet such that each subset's elements appear as a consecutive block in the string. This is a generalization of the consecutive ones property from matrices to a string-based formulation and arises in information retrieval and file organization.

**Associated rules:**
- R112: Hamiltonian Path -> Consecutive Sets (as target)

## Definition

**Name:** `ConsecutiveSets`
**Canonical name:** CONSECUTIVE SETS
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR18

**Mathematical definition:**

INSTANCE: Finite alphabet Sigma, collection C = {Sigma_1, Sigma_2, ..., Sigma_n} of subsets of Sigma, and a positive integer K.
QUESTION: Is there a string w in Sigma* with |w| <= K such that, for each i, the elements of Sigma_i occur in a consecutive block of |Sigma_i| symbols of w?

## Variables

<!-- Unverified: AI-inferred variable mapping -->
- **Count:** The primary decision variable is the string w of length at most K over the alphabet Sigma. This can be modeled as K position variables, each taking a value in Sigma (or a blank symbol).
- **Per-variable domain:** Each position in the string takes a value from Sigma or is unused (if |w| < K).
- **Meaning:** A satisfying assignment is a string w with |w| <= K such that for every subset Sigma_i in C, there exists a contiguous substring of w of length |Sigma_i| that contains exactly the elements of Sigma_i (each appearing exactly once in that block).

## Schema (data type)

<!-- Unverified: AI-designed schema -->
**Type name:** `ConsecutiveSets`
**Variants:** None

| Field | Type | Description |
|-------|------|-------------|
| `alphabet` | `Vec<char>` | The finite alphabet Sigma |
| `subsets` | `Vec<Vec<char>>` | The collection C of subsets of Sigma |
| `bound` | `usize` | The positive integer K (max string length) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- When K = |Sigma| (number of distinct symbols), the problem is equivalent to testing a matrix for the consecutive ones property, which is polynomial-time solvable.
- The circular variant (blocks may wrap around from end to beginning of ww) is also NP-complete [Booth, 1975].

## Complexity

<!-- Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O(|Sigma|! * n) brute-force by trying all permutations of the alphabet and checking if subsets form consecutive blocks within length K. More sophisticated approaches can be modeled as constraint satisfaction.
- **NP-completeness:** NP-complete [Kou, 1977]. Transformation from HAMILTONIAN PATH.
- **Polynomial special case:** If K equals the number of distinct symbols appearing in the subsets, the problem reduces to testing a binary matrix for the consecutive ones property [Booth and Lueker, 1976], solvable in linear time.
- **References:**
  - L. T. Kou (1977). "Polynomial complete consecutive information retrieval problems." *SIAM Journal on Computing*, 6(1):67-75.
  - K. S. Booth (1975). "PQ Tree Algorithms." Ph.D. thesis, University of California, Berkeley.
  - K. S. Booth and G. S. Lueker (1976). "Testing for the consecutive ones property, interval graphs, and graph planarity using PQ-tree algorithms." *J. Computer and System Sciences*, 13:335-379.

## Extra Remark

**Full book text:**

INSTANCE: Finite alphabet Sigma, collection C = {Sigma_1, Sigma_2, ..., Sigma_n} of subsets of Sigma, and a positive integer K.
QUESTION: Is there a string w in Sigma* with |w| <= K such that, for each i, the elements of Sigma_i occur in a consecutive block of |Sigma_i| symbols of W?
Reference: [Kou, 1977]. Transformation from HAMILTONIAN PATH.
Comment: The variant in which we ask only that the elements of each Sigma_i occur in a consecutive block of |Sigma_i| symbols of the string ww (i.e., we allow blocks that circulate from the end of w back to its beginning) is also NP-complete [Booth, 1975]. If K is the number of distinct symbols in the Sigma_i, then these problems are equivalent to determining whether a matrix has the consecutive ones property or the circular ones property and are solvable in polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all strings w of length <= K over Sigma and verify the consecutive block condition for each subset.
- [x] It can be solved by reducing to integer programming -- assign position variables to symbols and linearize the consecutiveness constraints.
- [x] Other: Reduction to consecutive ones property testing for the special case K = |Sigma|; constraint programming for general instances.

## Example Instance

<!-- Unverified: AI-constructed example -->

**Instance 1 (YES instance):**
Alphabet: Sigma = {a, b, c, d, e, f}
Subsets: C = {{a, b, c}, {c, d}, {d, e, f}, {b, c, d}}
K = 6

String w = "abcdef" (length 6 = K):
- {a, b, c}: positions 0-2 = "abc" -- consecutive block of 3 containing {a,b,c}. YES.
- {c, d}: positions 2-3 = "cd" -- consecutive block of 2 containing {c,d}. YES.
- {d, e, f}: positions 3-5 = "def" -- consecutive block of 3 containing {d,e,f}. YES.
- {b, c, d}: positions 1-3 = "bcd" -- consecutive block of 3 containing {b,c,d}. YES.
Answer: YES

**Instance 2 (NO instance):**
Alphabet: Sigma = {a, b, c, d, e, f}
Subsets: C = {{a, c, e}, {b, d, f}, {a, b}, {c, d}, {e, f}}
K = 6

For any string w of length 6 that is a permutation of {a,b,c,d,e,f}:
- {a, c, e} must be consecutive: a, c, e must appear in 3 adjacent positions.
- {b, d, f} must be consecutive: b, d, f must appear in 3 adjacent positions.
- But {a, b} must also be consecutive (2 adjacent positions), requiring a and b to be neighbors.
- If {a,c,e} occupies positions 0-2 and {b,d,f} occupies positions 3-5, then a and b are not neighbors (distance >= 1).
- If {a,c,e} occupies positions 3-5 and {b,d,f} occupies positions 0-2, same problem for {a,b}.
- No arrangement satisfies all constraints simultaneously.
Answer: NO
