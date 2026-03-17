---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to SIMULTANEOUS INCONGRUENCES"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'SIMULTANEOUS INCONGRUENCES'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** SIMULTANEOUS INCONGRUENCES
**Motivation:** Establishes NP-completeness of SIMULTANEOUS INCONGRUENCES via polynomial-time reduction from 3SAT. This result by Stockmeyer and Meyer (1973) shows that finding an integer that avoids a specified set of residue classes is computationally intractable. The problem is closely related to covering systems in number theory. Despite the simplicity of the problem statement (find an integer not in any forbidden residue class), the interaction of multiple moduli makes it NP-complete.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249

## GJ Source Entry

> [AN2] SIMULTANEOUS INCONGRUENCES
> INSTANCE: Collection {(a_1,b_1),...,(a_n,b_n)} of ordered pairs of positive integers, with a_i ≤ b_i for 1 ≤ i ≤ n.
> QUESTION: Is there an integer x such that, for 1 ≤ i ≤ n, x ≢ a_i (mod b_i)?
> Reference: [Stockmeyer and Meyer, 1973]. Transformation from 3SAT.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

The reduction from 3SAT to SIMULTANEOUS INCONGRUENCES follows the approach of Stockmeyer and Meyer (1973). Given a 3SAT instance with n variables U = {u_1, ..., u_n} and m clauses C = {c_1, ..., c_m}:

**High-level approach:**
The idea is to encode each Boolean variable using distinct prime moduli. A truth assignment corresponds to choosing x in certain residue classes. The clauses are encoded by requiring x to avoid (be incongruent to) certain residue classes -- specifically, the residue classes that correspond to falsifying assignments.

**Construction:**

1. **Prime assignment:** For each variable u_i, assign a distinct prime p_i >= 3. Each prime has at least 3 residue classes (0, 1, ..., p_i - 1). Use two residue classes per prime to encode TRUE/FALSE:
   - x ≡ 0 (mod p_i) encodes u_i = TRUE
   - x ≡ 1 (mod p_i) encodes u_i = FALSE
   - The remaining residue classes (2, ..., p_i - 1) must be forbidden.

2. **Forbid invalid residue classes:** For each variable u_i and each residue r in {2, 3, ..., p_i - 1}, add the pair (r, p_i) to the collection. This ensures x ≡ 0 or 1 (mod p_i), encoding a valid Boolean assignment.

3. **Clause encoding:** For each clause c_j = (l_1 ∨ l_2 ∨ l_3) with literals over variables u_{i1}, u_{i2}, u_{i3}: The clause is violated when all three literals are false. This corresponds to a specific combination of residues modulo p_{i1}, p_{i2}, p_{i3}. Use the Chinese Remainder Theorem: since p_{i1}, p_{i2}, p_{i3} are coprime, there is a unique residue r_j modulo M_j = p_{i1} * p_{i2} * p_{i3} corresponding to the all-false assignment. Add the pair (r_j, M_j) to the collection -- but this would require x ≢ r_j (mod M_j), which is what we want since simultaneous incongruences requires x to avoid ALL listed residue classes.

   However, the direction is subtle: we need the clause to be *satisfied*, meaning at least one literal is true. The falsifying assignment for a clause is exactly one residue class mod M_j. We add (r_j, M_j) to forbid that falsifying assignment.

   Wait -- the problem asks for x such that x ≢ a_i (mod b_i) for ALL i. So every pair in the collection is a *forbidden* residue class. We need:
   - The invalid Boolean assignments (residues 2, ..., p_i-1) to be forbidden -- correct, we add those.
   - The all-literals-false assignment for each clause to be forbidden -- but this is ONE residue class we want to forbid, and adding it means x must avoid it, which is what we want.

   Actually the structure works differently. The key insight is:
   - We want x to NOT be in any forbidden class.
   - Forbidden classes include: (a) non-Boolean residues for each variable, and (b) clause-violating residues.
   - If such an x exists, reading off x mod p_i gives a satisfying assignment.

4. **Solution extraction:** Given x satisfying all incongruences, for each variable u_i, compute x mod p_i. If x mod p_i = 0, set u_i = TRUE; if x mod p_i = 1, set u_i = FALSE.

**Key properties:**
- Number of pairs: (p_1 - 2) + ... + (p_n - 2) + m = O(n * max_prime + m) = O(n^2 + m) pairs (using primes up to O(n log n))
- The moduli b_i are either small primes (for variable encoding) or products of 3 primes (for clause encoding)
- All values are polynomial in the input size

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source 3SAT instance (number of variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_pairs` | O(n^2 + m) |
| `max_modulus` | O(n^3 * log(n)^3) |

**Derivation:**
- Variable encoding: For each of n primes p_i (each O(n log n) by prime number theorem), we add (p_i - 2) forbidden residue pairs. Total: sum_{i=1}^{n} (p_i - 2) = O(n^2)
- Clause encoding: m pairs, one per clause, with modulus being the product of 3 primes = O(n^3 log^3 n)
- Total pairs: O(n^2 + m)

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce KSatisfiability<K3> instance to SimultaneousIncongruences, solve target with BruteForce (search for x that avoids all forbidden residue classes), extract truth assignment from x mod p_i, verify truth assignment satisfies all clauses
- Test with both satisfiable and unsatisfiable 3SAT instances
- Verify that the number of pairs matches the expected overhead formula

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (KSatisfiability<K3>):**
2 variables: u_1, u_2 (n = 2)
2 clauses (m = 2):
- c_1 = (u_1 ∨ u_2)
- c_2 = (¬u_1 ∨ u_2)

**Construction:**

1. Assign primes: p_1 = 3 (for u_1), p_2 = 5 (for u_2).
   - u_1 = TRUE ↔ x ≡ 0 (mod 3), u_1 = FALSE ↔ x ≡ 1 (mod 3)
   - u_2 = TRUE ↔ x ≡ 0 (mod 5), u_2 = FALSE ↔ x ≡ 1 (mod 5)

2. Forbid invalid residues:
   - For p_1 = 3: forbid residue 2. Add pair (2, 3).
   - For p_2 = 5: forbid residues 2, 3, 4. Add pairs (2, 5), (3, 5), (4, 5).

3. Clause encoding:
   - c_1 = (u_1 ∨ u_2): violated when u_1 = F, u_2 = F, i.e., x ≡ 1 (mod 3) and x ≡ 1 (mod 5). By CRT: x ≡ 1 (mod 15). Add pair (1, 15).
   - c_2 = (¬u_1 ∨ u_2): violated when u_1 = T, u_2 = F, i.e., x ≡ 0 (mod 3) and x ≡ 1 (mod 5). By CRT: x ≡ 6 (mod 15). Add pair (6, 15).

4. Complete collection of forbidden pairs:
   {(2, 3), (2, 5), (3, 5), (4, 5), (1, 15), (6, 15)}

**Verification (search for valid x in range [0, 14]):**
- x = 0: 0 mod 3 = 0 (ok), 0 mod 5 = 0 (ok), 0 mod 15 = 0 (not 1, ok; not 6, ok). x = 0 works!
  - u_1 = TRUE (0 mod 3 = 0), u_2 = TRUE (0 mod 5 = 0)
  - Check: c_1 = (T ∨ T) = T, c_2 = (F ∨ T) = T. Satisfies both clauses.

- x = 3: 3 mod 3 = 0 (ok), 3 mod 5 = 3 (FORBIDDEN: pair (3,5)). x = 3 fails.

- x = 5: 5 mod 3 = 2 (FORBIDDEN: pair (2,3)). x = 5 fails.

- x = 10: 10 mod 3 = 1 (ok), 10 mod 5 = 0 (ok), 10 mod 15 = 10 (not 1, ok; not 6, ok). x = 10 works!
  - u_1 = FALSE (10 mod 3 = 1), u_2 = TRUE (10 mod 5 = 0)
  - Check: c_1 = (F ∨ T) = T, c_2 = (T ∨ T) = T. Satisfies both clauses.


## References

- **[Stockmeyer and Meyer, 1973]**: [`Stockmeyer and Meyer1973`] Larry J. Stockmeyer and Albert R. Meyer (1973). "Word problems requiring exponential time". In: *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1-9. Association for Computing Machinery.
