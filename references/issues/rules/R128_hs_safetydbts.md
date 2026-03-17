---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hitting Set to Safety of Database Transaction Systems"
labels: rule
assignees: ''
canonical_source_name: 'HITTING SET'
canonical_target_name: 'SAFETY OF DATABASE TRANSACTION SYSTEMS'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Hitting Set
**Target:** Safety of Database Transaction Systems
**Motivation:** Establishes NP-hardness of Safety of Database Transaction Systems via polynomial-time reduction from Hitting Set. A transaction system is "safe" if every possible interleaved history of its transactions is equivalent to some serial history. Papadimitriou, Bernstein, and Rothnie (1977) showed that determining safety is computationally intractable by reducing from Hitting Set: each subset to be "hit" is encoded as a shared variable mediating conflicts between transactions, so that a hitting set of size k corresponds to a set of variables whose concurrent access patterns make every history serializable. Notably, this problem is not known to be in NP or co-NP, making it harder to classify than standard NP-complete problems.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.234-235

## GJ Source Entry

> [SR34] SAFETY OF DATABASE TRANSACTION SYSTEMS (*)
> INSTANCE: Set V of database variables, and a collection T of "transactions" (R_i, W_i), 1 <= i <= n, where R_i and W_i are both subsets of V.
> QUESTION: Is every history H for T equivalent to some serial history?
> Reference: [Papadimitriou, Bernstein, and Rothnie, 1977]. Transformation from HITTING SET.
> Comment: Not known either to be in NP or to be in co-NP. Testing whether every history H for T is "D-equivalent" to some serial history can be done in polynomial time, where two histories are D-equivalent if one can be obtained from the other by a sequence of interchanges of adjacent sets in such a way that at each step the new history is equivalent to the previous one.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Hitting Set instance (S, C, k) where S is a universe, C = {c_1, ..., c_m} is a collection of subsets of S, and k is a budget, construct a Safety of Database Transaction Systems instance (V, T) as follows:

1. **Variable construction:** Create one database variable v_s for each element s in the universe S. Thus |V| = |S|.

2. **Transaction construction:** For each subset c_j = {s_{j1}, s_{j2}, ..., s_{j,|c_j|}} in the collection C, create a pair of "conflicting" transactions:
   - Transaction T_{j,1} with read set R_{j,1} = {v_s : s in c_j} and write set W_{j,1} = {v_s : s in c_j}.
   - Transaction T_{j,2} with read set R_{j,2} = {v_s : s in c_j} and write set W_{j,2} = {v_s : s in c_j}.
   These two transactions conflict on exactly the variables corresponding to elements of c_j. Any interleaving of T_{j,1} and T_{j,2} is serializable if and only if the conflict is resolved — which happens when at least one variable in c_j is "protected" (i.e., accessed in a way that forces a serial order).

3. **Encoding the hitting set constraint:** Additional "guard" transactions are introduced to ensure that the system is safe if and only if there exists a hitting set of size at most k. For each element s in S, a guard transaction T_s is created that reads and writes {v_s}. The guard transactions interleave with the subset-pair transactions in such a way that making the system safe requires selecting at most k guard transactions to "lock" certain variables — which corresponds exactly to choosing a hitting set of size k that intersects every subset c_j.

4. **Solution extraction:** The transaction system (V, T) is safe (every history equivalent to some serial history) if and only if there exists a subset S' of S with |S'| <= k that hits every c_j in C.

**Key invariant:** The shared variables between conflicting transaction pairs encode the subset structure of the Hitting Set instance. Safety of the transaction system requires that every pair of conflicting transactions can be serialized regardless of their interleaving, which is equivalent to every subset in C being "hit" by at least one selected element.

**Note:** The (*) annotation in GJ indicates that this problem is not known to be in NP or co-NP. The question "is every history equivalent to a serial history?" quantifies universally over all histories, making it a co-problem. The reduction from Hitting Set establishes NP-hardness, placing the problem outside P (assuming P != NP), but its exact complexity class remains open.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `universe_size` of source Hitting Set instance (|S|)
- m = `num_sets` of source Hitting Set instance (|C|)
- d_max = maximum subset size in C

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_variables` | `universe_size` |
| `num_transactions` | `2 * num_sets + universe_size` |

**Derivation:**
- Variables: one database variable per universe element -> |V| = n
- Transactions: two conflicting transactions per subset (2m) plus one guard transaction per universe element (n) -> |T| = 2m + n
- Each transaction's read/write set has size at most d_max (the size of the corresponding subset)
- Budget parameter k is encoded structurally in the guard transaction mechanism

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a Hitting Set instance to a Safety of Database Transaction Systems instance, enumerate all possible histories of the transaction system, verify each is equivalent to some serial history if and only if the original Hitting Set instance has a solution
- Check that the number of variables and transactions matches the overhead formula
- Test with a Hitting Set instance where the greedy heuristic fails (e.g., an element appearing in many subsets but not part of any minimum hitting set)
- Verify that removing one subset from C corresponds to relaxing one conflict constraint in the transaction system

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Hitting Set):**
Universe S = {a, b, c, d, e, f} (6 elements)
Collection C (4 subsets):
- c_1 = {a, b, c}
- c_2 = {b, d, e}
- c_3 = {c, e, f}
- c_4 = {a, d, f}

Budget k = 2

Minimum hitting set: S' = {b, f} (size 2):
- c_1 = {a,b,c}: b in S' ✓
- c_2 = {b,d,e}: b in S' ✓
- c_3 = {c,e,f}: f in S' ✓
- c_4 = {a,d,f}: f in S' ✓

**Constructed target instance (Safety of Database Transaction Systems):**
Database variables V = {v_a, v_b, v_c, v_d, v_e, v_f} (6 variables)
Transactions T (14 transactions):
- Conflicting pair for c_1: T_{1,1} and T_{1,2}, both with R = W = {v_a, v_b, v_c}
- Conflicting pair for c_2: T_{2,1} and T_{2,2}, both with R = W = {v_b, v_d, v_e}
- Conflicting pair for c_3: T_{3,1} and T_{3,2}, both with R = W = {v_c, v_e, v_f}
- Conflicting pair for c_4: T_{4,1} and T_{4,2}, both with R = W = {v_a, v_d, v_f}
- Guard transactions: T_a({v_a}), T_b({v_b}), T_c({v_c}), T_d({v_d}), T_e({v_e}), T_f({v_f})

**Solution mapping:**
- The hitting set S' = {b, f} corresponds to selecting guard transactions T_b and T_f as "locking" guards.
- These guards force serial ordering on the conflicting pairs:
  - c_1 pair: T_b locks v_b, serializing T_{1,1} and T_{1,2} ✓
  - c_2 pair: T_b locks v_b, serializing T_{2,1} and T_{2,2} ✓
  - c_3 pair: T_f locks v_f, serializing T_{3,1} and T_{3,2} ✓
  - c_4 pair: T_f locks v_f, serializing T_{4,1} and T_{4,2} ✓
- Every history is equivalent to some serial history ✓

**Greedy trap:** Element e appears in 2 subsets (c_2, c_3), same frequency as b and f. But choosing {e, a} (greedy by frequency) only hits c_2, c_3, c_4 — missing c_1. The correct choice {b, f} covers all 4 subsets.


## References

- **[Papadimitriou, Bernstein, and Rothnie, 1977]**: [`Papadimitriou1977b`] Christos H. Papadimitriou and P. A. Bernstein and J. B. Rothnie (1977). "Some computational problems related to database concurrency control". In: *Proceedings of the Conference on Theoretical Computer Science*, pp. 275–282.
