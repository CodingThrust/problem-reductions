---
name: Problem
about: Propose a new problem type
title: "[Model] ConsistencyOfDatabaseFrequencyTables"
labels: model
assignees: ''
---

## Motivation

CONSISTENCY OF DATABASE FREQUENCY TABLES (P183) from Garey & Johnson, A4 SR35. A classical NP-complete problem at the intersection of database theory and statistical disclosure control. It asks whether published frequency tables (cross-tabulations of attribute pairs) are consistent with a set of known attribute values — that is, whether there exists a complete assignment of attribute values to all objects that matches both the frequency tables and the known values. The NP-completeness result (Reiss, 1977) has important implications for database privacy: it shows that "compromising" a database by deducing individual attribute values from published frequency tables is computationally intractable unless P = NP.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R129: 3SAT -> Consistency of Database Frequency Tables (GJ SR35)
- **As source:** (none known in GJ)

## Definition

**Name:** <!-- ⚠️ Unverified --> `ConsistencyOfDatabaseFrequencyTables`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Consistency of Database Frequency Tables (also: Frequency Table Satisfiability, Statistical Database Consistency, Contingency Table Realizability)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR35

**Mathematical definition:**

INSTANCE: Set A of attribute names, domain set D_a for each a in A, set V of objects, collection F of frequency tables for some pairs a,b in A (where a frequency table for a,b in A is a function f_{a,b}: D_a x D_b -> Z+ with the sum, over all pairs x in D_a and y in D_b, of f_{a,b}(x,y) equal to |V|), and a set K of triples (v,a,x) with v in V, a in A, and x in D_a, representing the known attribute values.

QUESTION: Are the frequency tables in F consistent with the known attribute values in K, i.e., is there a collection of functions g_a: V -> D_a, for each a in A, such that g_a(v) = x if (v,a,x) in K and such that, for each f_{a,b} in F, x in D_a, and y in D_b, the number of v in V for which g_a(v) = x and g_b(v) = y is exactly f_{a,b}(x,y)?

The problem is a decision (satisfaction) problem: given frequency tables and partial knowledge, determine if there exists a complete assignment consistent with everything.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |V| * |A| (one variable per object-attribute pair, representing the attribute value assigned to each object)
- **Per-variable domain:** For object v and attribute a, the variable g_a(v) takes values in D_a. If (v,a,x) is in K, then g_a(v) is fixed to x.
- **Meaning:** Variable g_a(v) = x means object v has value x for attribute a. The configuration (g_a(v) for all v in V, a in A) encodes a complete database. The assignment is valid if all frequency table constraints are satisfied and all known values in K are respected.

For a brute-force encoding, one can use |V| * |A| categorical variables, each with domain size max_a |D_a|.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ConsistencyOfDatabaseFrequencyTables`
**Variants:** none (no graph or weight type parameter; the database schema is stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `num_objects` | `usize` | Number of objects in V (indexed 0..num_objects) |
| `attributes` | `Vec<usize>` | Domain sizes: attributes[i] = |D_i| for attribute i (values indexed 0..attributes[i]) |
| `frequency_tables` | `Vec<(usize, usize, Vec<Vec<usize>>)>` | Collection F: each entry (a, b, table) where table[x][y] = f_{a,b}(x,y) |
| `known_values` | `Vec<(usize, usize, usize)>` | Set K of triples (object_index, attribute_index, value) |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Reiss, 1977; transformation from 3SAT).
- **Best known exact algorithm:** Brute-force enumeration over all possible attribute-value assignments: for each object v and each attribute a, try all values in D_a. The total search space is the product over all (v,a) of |D_a|, which is |D|^(|V|*|A|) in the worst case where all domains have the same size |D|. For binary attributes (|D_a| = 2 for all a), this is 2^(|V|*|A|).
- **Related modern work:** The problem is closely related to FREQSAT (Calders, 2004), which asks whether given itemset-frequency pairs can be realized by some database. FREQSAT is also NP-complete, and when the maximum number of duplicate transactions is bounded, it becomes PP-hard. These results generalize the Reiss (1977) result to more general frequency constraints.
- **Practical implications:** The NP-completeness means that there is no efficient general algorithm for determining whether published frequency tables can be used to deduce individual attribute values, providing a theoretical foundation for the security of statistical databases that release aggregate frequency information.
- **References:**
  - [Reiss, 1977b] S. P. Reiss, "Statistical database confidentiality", Dept. of Statistics, University of Stockholm, 1977.
  - [Reiss, 1980] S. P. Reiss, "Practical data-swapping: The first steps", *Proc. IEEE Symposium on Security and Privacy*, pp. 36-44, 1980.
  - [Calders, 2004] T. Calders, "Computational complexity of itemset frequency satisfiability", *Proc. 23rd ACM SIGMOD-SIGACT-SIGART Symposium on Principles of Database Systems (PODS)*, pp. 143-154, 2004.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is related to:** Contingency table realizability, FREQSAT (itemset frequency satisfiability)
- **Known special cases:** When all domains are binary ({0, 1}) and F contains all pairwise frequency tables, the problem is equivalent to finding a binary matrix with given 2x2 marginals. When K is empty, the problem reduces to finding any database consistent with F.
- **Restriction:** When only univariate (single-attribute) frequency tables are given (not pairwise), the problem is solvable in polynomial time by independent assignment.

## Extra Remark

**Full book text:**

INSTANCE: Set A of attribute names, domain set Da for each a ∈ A, set V of objects, collection F of frequency tables for some pairs a,b ∈ A (where a frequency table for a,b ∈ A is a function fa,b: Da×Db → Z+ with the sum, over all pairs x ∈ Da and y ∈ Db, of fa,b(x,y) equal to |V|), and a set K of triples (v,a,x) with v ∈ V, a ∈ A, and x ∈ Da, representing the known attribute values.
QUESTION: Are the frequency tables in F consistent with the known attribute values in K, i.e., is there a collection of functions ga: V → Da, for each a ∈ A, such that ga(v) = x if (v,a,x) ∈ K and such that, for each fa,b ∈ F, x ∈ Da, and y ∈ Db, the number of v ∈ V for which ga(v) = x and gb(v) = y is exactly fa,b(x,y)?
Reference: [Reiss, 1977b]. Transformation from 3SAT.
Comment: Above result implies that no polynomial time algorithm can be given for "compromising" a data base from its frequency tables by deducing prespecified attribute values, unless P = NP (see reference for details).

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible assignments of attribute values to objects (g_a(v) for all v, a), check that all known values in K are respected and all frequency tables in F are exactly matched.
- [x] It can be solved by reducing to integer programming. Introduce integer variables y_{v,a,x} in {0,1} for each object v, attribute a, value x (indicating g_a(v) = x). Constraints: (1) exactly one value per object-attribute: sum_x y_{v,a,x} = 1; (2) known values: y_{v,a,x} = 1 if (v,a,x) in K; (3) frequency tables: sum_v y_{v,a,x} * y_{v,b,y} = f_{a,b}(x,y) for all (a,b) in F, x, y. The quadratic constraint (3) can be linearized by introducing auxiliary variables.
- [ ] Other: (none identified)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Objects V = {v_0, v_1, v_2, v_3, v_4, v_5} (6 objects)
Attributes A = {a_0, a_1, a_2} with domains:
- D_{a_0} = {0, 1} (binary, e.g., "gender")
- D_{a_1} = {0, 1, 2} (ternary, e.g., "age group")
- D_{a_2} = {0, 1} (binary, e.g., "employed")

Frequency tables F (2 tables):

f_{a_0, a_1} (gender x age_group):

| a_0 \ a_1 | 0 | 1 | 2 |
|-----------|---|---|---|
| 0         | 1 | 1 | 1 |
| 1         | 1 | 1 | 1 |

(Each cell has count 1; row sums = 3, column sums = 2, total = 6 = |V|)

f_{a_1, a_2} (age_group x employed):

| a_1 \ a_2 | 0 | 1 |
|-----------|---|---|
| 0         | 1 | 1 |
| 1         | 0 | 2 |
| 2         | 1 | 1 |

(Row sums: 2, 2, 2; column sums: 2, 4; total = 6 = |V|)

Known values K = {(v_0, a_0, 0), (v_3, a_0, 1), (v_1, a_2, 1)}
(We know: v_0 has gender=0, v_3 has gender=1, v_1 is employed=1)

**Consistent assignment:**

| Object | a_0 (gender) | a_1 (age_group) | a_2 (employed) |
|--------|-------------|-----------------|----------------|
| v_0    | 0           | 0               | 0              |
| v_1    | 0           | 1               | 1              |
| v_2    | 0           | 2               | 1              |
| v_3    | 1           | 0               | 1              |
| v_4    | 1           | 1               | 1              |
| v_5    | 1           | 2               | 0              |

**Verification of f_{a_0, a_1}:**
- (0,0): v_0 -> count=1 ✓
- (0,1): v_1 -> count=1 ✓
- (0,2): v_2 -> count=1 ✓
- (1,0): v_3 -> count=1 ✓
- (1,1): v_4 -> count=1 ✓
- (1,2): v_5 -> count=1 ✓

**Verification of f_{a_1, a_2}:**
- (0,0): v_0 -> count=1 ✓
- (0,1): v_3 -> count=1 ✓
- (1,0): (none) -> count=0 ✓
- (1,1): v_1, v_4 -> count=2 ✓
- (2,0): v_5 -> count=1 ✓
- (2,1): v_2 -> count=1 ✓

**Verification of known values K:**
- (v_0, a_0, 0): g_{a_0}(v_0) = 0 ✓
- (v_3, a_0, 1): g_{a_0}(v_3) = 1 ✓
- (v_1, a_2, 1): g_{a_2}(v_1) = 1 ✓

All frequency tables match and all known values are respected. The tables are consistent. ✓
