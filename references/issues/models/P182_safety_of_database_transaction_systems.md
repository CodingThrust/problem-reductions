---
name: Problem
about: Propose a new problem type
title: "[Model] SafetyOfDatabaseTransactionSystems(*)"
labels: model
assignees: ''
---

## Motivation

SAFETY OF DATABASE TRANSACTION SYSTEMS (*) (P182) from Garey & Johnson, A4 SR34. A classical problem from database concurrency control theory. It asks whether a set of transactions is "safe," meaning that every possible interleaved execution history is equivalent to some serial (sequential) execution. This problem was shown to be NP-hard by Papadimitriou, Bernstein, and Rothnie (1977), but is notably not known to be in NP or co-NP, making it one of the most unusual problems in the GJ compendium. It is closely related to the problem of testing view serializability, which Papadimitriou (1979) proved NP-complete for individual schedules.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R128: Hitting Set -> Safety of Database Transaction Systems (GJ SR34)
- **As source:** (none known in GJ)

## Definition

**Name:** <!-- ⚠️ Unverified --> `SafetyOfDatabaseTransactionSystems`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Safety of Database Transaction Systems (also: Transaction System Safety, Serializability Safety)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR34

**Mathematical definition:**

INSTANCE: Set V of database variables, and a collection T of "transactions" (R_i, W_i), 1 <= i <= n, where R_i and W_i are both subsets of V.
QUESTION: Is every history H for T equivalent to some serial history?

A **history** H for T is an interleaving of the read and write operations of all transactions, where each transaction's read R_i precedes its write W_i. Two histories are **equivalent** if, for every initial database state, they produce the same final database state and every transaction reads the same values. A **serial history** is one in which all operations of one transaction complete before any operation of another transaction begins.

The problem is a decision (satisfaction) problem asking a universal question: whether ALL possible histories are serializable, not just a single given history.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** The problem does not have a natural binary-variable encoding like combinatorial optimization problems. The "configuration space" is the set of all possible interleavings of the 2n operations (n reads + n writes), subject to the constraint that R_i precedes W_i for each i.
- **Per-variable domain:** Each history is a permutation of the 2n operations satisfying precedence constraints.
- **Meaning:** The question is whether every such permutation (history) produces a result equivalent to some serial ordering of the n transactions.

For a brute-force approach, one could encode the problem as: for each pair of transactions (i, j) that conflict (share a variable), assign an ordering variable o_{i,j} in {0, 1} indicating whether i precedes j. The question is whether all consistent orderings lead to serializable executions.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SafetyOfDatabaseTransactionSystems`
**Variants:** none (no graph or weight type parameter; transactions are stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `num_variables` | `usize` | Number of database variables in V (indexed 0..num_variables) |
| `transactions` | `Vec<(Vec<usize>, Vec<usize>)>` | The collection T of transactions; each tuple (R_i, W_i) contains the read set and write set as vectors of variable indices |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-hard (Papadimitriou, Bernstein, and Rothnie, 1977; transformation from HITTING SET). Not known to be in NP or co-NP — the (*) annotation in GJ marks this exceptional status.
- **Related problem:** Testing whether a single given history is serializable (view serializability) is NP-complete (Papadimitriou, 1979). The safety problem is harder because it quantifies over ALL possible histories.
- **Best known exact algorithm:** Brute-force: enumerate all possible histories (there can be exponentially many interleavings of 2n operations) and test each for equivalence to some serial history. The number of interleavings is (2n)! / 2^n (accounting for the R_i-before-W_i constraint), making brute force impractical even for small n.
- **Special case (D-equivalence):** Testing whether every history is D-equivalent to some serial history (where D-equivalence only allows interchange of adjacent non-conflicting operations) can be done in polynomial time (noted in GJ comment). This is equivalent to testing conflict-serializability safety, which reduces to checking a conflict graph.
- **Parameterized:** For locked transaction systems (where transactions acquire locks), Lipski and Papadimitriou (1981) gave an O(n log n log log n) algorithm for safety testing.
- **References:**
  - [Papadimitriou, Bernstein, Rothnie, 1977] C. H. Papadimitriou, P. A. Bernstein, J. B. Rothnie, "Some computational problems related to database concurrency control", *Proc. Conf. on Theoretical Computer Science*, pp. 275-282.
  - [Papadimitriou, 1979] C. H. Papadimitriou, "The serializability of concurrent database updates", *Journal of the ACM*, 26(4), pp. 631-653.
  - [Lipski, Papadimitriou, 1981] W. Lipski, C. H. Papadimitriou, "A fast algorithm for testing for safety and detecting deadlocks in locked transaction systems", *Journal of Algorithms*, 2, pp. 211-226.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a generalization of:** Serializability of Database Histories (SR33, testing a single history) — safety tests ALL histories
- **Known special cases:** D-equivalence safety (polynomial time); locked transaction systems (O(n log n log log n) algorithm)
- **Restriction:** When each R_i and W_i are identical singletons ({v_i}), the problem simplifies to checking for cyclic dependencies

## Extra Remark

**Full book text:**

INSTANCE: Set V of database variables, and a collection T of "transactions" (Ri,Wi), 1 ≤ i ≤ n, where Ri and Wi are both subsets of V.
QUESTION: Is every history H for T equivalent to some serial history?
Reference: [Papadimitriou, Bernstein, and Rothnie, 1977]. Transformation from HITTING SET.
Comment: Not known either to be in NP or to be in co-NP. Testing whether every history H for T is "D-equivalent" to some serial history can be done in polynomial time, where two histories are D-equivalent if one can be obtained from the other by a sequence of interchanges of adjacent sets in such a way that at each step the new history is equivalent to the one.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible interleavings of the 2n read/write operations (subject to R_i before W_i), test each for equivalence to some serial history. Exponential in n.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: For the D-equivalence variant, construct a conflict graph and check for acyclicity (polynomial time). For locked transaction systems, use the Lipski-Papadimitriou algorithm.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Database variables V = {v_0, v_1, v_2, v_3, v_4, v_5} (6 variables)
Transactions T (4 transactions):
- T_1: R_1 = {v_0, v_1, v_2}, W_1 = {v_0, v_1}
- T_2: R_2 = {v_1, v_3, v_4}, W_2 = {v_3, v_4}
- T_3: R_3 = {v_2, v_4, v_5}, W_3 = {v_2, v_5}
- T_4: R_4 = {v_0, v_3, v_5}, W_4 = {v_0, v_5}

**Conflict analysis:**
- T_1 and T_2 conflict on v_1 (T_1 reads v_1, T_2 reads v_1; T_1 writes v_1)
- T_1 and T_3 conflict on v_2 (T_1 reads v_2, T_3 writes v_2)
- T_2 and T_3 conflict on v_4 (T_2 writes v_4, T_3 reads v_4)
- T_1 and T_4 conflict on v_0 (T_1 writes v_0, T_4 reads v_0 and writes v_0)
- T_2 and T_4 conflict on v_3 (T_2 writes v_3, T_4 reads v_3)
- T_3 and T_4 conflict on v_5 (T_3 writes v_5, T_4 reads v_5 and writes v_5)

Every pair of transactions has a conflict, forming a complete conflict graph on 4 nodes. The system is NOT safe because there exist interleavings whose serialization order is forced into a cycle: T_1 -> T_2 (via v_1), T_2 -> T_3 (via v_4), T_3 -> T_4 (via v_5), T_4 -> T_1 (via v_0) — creating a cyclic dependency that cannot be resolved into any serial history.

**A safe example (for comparison):**
T_1: R_1 = {v_0}, W_1 = {v_0}
T_2: R_2 = {v_1}, W_2 = {v_1}
These two transactions access disjoint variables, so every interleaving is trivially equivalent to both serial orderings (T_1 then T_2, or T_2 then T_1). The system is safe.
