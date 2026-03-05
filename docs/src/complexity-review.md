# Complexity & Overhead Correctness Review

Systematic review of `declare_variants!` complexity expressions and `#[reduction(overhead)]` formulas.
New reviews should be appended to the appropriate table. See [Issue #107](https://github.com/CodingThrust/problem-reductions/issues/107) for methodology.

**Review result:** PASS (correct), FAIL (wrong — needs fix), UNVERIFIED (not yet reviewed).

## Models (Variant Complexity)

The complexity field represents the **worst-case time complexity of the best known algorithm** for each problem variant.

| Result | Problem | Location | Declared | Best Known | Algorithm | Authors | Fix | Source | Reviewed |
|---|---|---|---|---|---|---|---|---|---|
| FAIL | MaximumMatching\<SimpleGraph, i32\> | `maximum_matching.rs:223` | `2^num_vertices` | O(V³) poly | Blossom algorithm | Edmonds 1965; Gabow 1990 | `num_vertices^3` | [DOI:10.4153/CJM-1965-045-4](https://doi.org/10.4153/CJM-1965-045-4) | 2026-02-28 |
| FAIL | KColoring\<K2\> | `kcoloring.rs` | `2^num_vertices` | O(V+E) poly | BFS bipartiteness check | Textbook (CLRS) | `num_vertices + num_edges` | Standard textbook result | 2026-02-28 |
| FAIL | KSatisfiability\<K2\> | `ksat.rs` | `2^num_variables` | O(n+m) poly | Implication graph SCC | Aspvall, Plass, Tarjan 1979 | `num_variables + num_clauses` | [DOI:10.1016/0020-0190(79)90002-4](https://doi.org/10.1016/0020-0190(79)90002-4) | 2026-02-28 |
| FAIL | KColoring\<K3\> | `kcoloring.rs` | `3^num_vertices` | O\*(1.3289^n) | Constraint satisfaction reduction | Beigel & Eppstein 2005 | `1.3289^num_vertices` | [DOI:10.1016/j.jalgor.2004.06.008](https://doi.org/10.1016/j.jalgor.2004.06.008) | 2026-02-28 |
| FAIL | KColoring\<K4\> | `kcoloring.rs` | `4^num_vertices` | O\*(1.7159^n) | Measure and conquer | Wu, Gu, Jiang, Shao, Xu 2024 | `1.7159^num_vertices` | [DOI:10.4230/LIPIcs.ESA.2024.103](https://doi.org/10.4230/LIPIcs.ESA.2024.103) | 2026-02-28 |
| FAIL | KColoring\<K5\> | `kcoloring.rs` | `5^num_vertices` | O\*((2−ε)^n) | Breaking the 2^n barrier | Zamir 2021 | `2^num_vertices` | [DOI:10.4230/LIPIcs.ICALP.2021.113](https://doi.org/10.4230/LIPIcs.ICALP.2021.113) | 2026-02-28 |
| FAIL | KColoring\<KN\> | `kcoloring.rs` | `k^num_vertices` | O\*(2^n) | Inclusion-exclusion / zeta transform | Björklund, Husfeldt, Koivisto 2009 | `2^num_vertices` | [DOI:10.1137/070683933](https://doi.org/10.1137/070683933) | 2026-02-28 |
| PASS | MIS\<SimpleGraph\> (7 variants) | | `2^n` | O\*(1.1996^n) | Measure & Conquer | Xiao & Nagamochi 2017 | `1.1996^num_vertices` | [DOI:10.1016/j.ic.2017.06.001](https://doi.org/10.1016/j.ic.2017.06.001) | 2026-02-28 |
| PASS | MIS\<TriangularSubgraph\> | | `2^n` | O\*(1.1893^n) | Degree-6 MIS | Xiao & Nagamochi 2017 | `1.1893^num_vertices` | Same as above | 2026-02-28 |
| PASS | MVC\<SimpleGraph\> | | `2^n` | O\*(1.1996^n) | Via MIS complement | Xiao & Nagamochi 2017 | `1.1996^num_vertices` | Same as above | 2026-02-28 |
| PASS | MaxClique\<SimpleGraph\> | | `2^n` | O\*(1.1892^n) exp space | Backtracking+DP | Robson 2001 | `1.1892^num_vertices` | [DOI:10.1016/0196-6774(86)90032-5](https://doi.org/10.1016/0196-6774(86)90032-5) | 2026-02-28 |
| PASS | MDS\<SimpleGraph\> | | `2^n` | O\*(1.4969^n) | Measure & Conquer | van Rooij & Bodlaender 2011 | `1.4969^num_vertices` | [DOI:10.1016/j.dam.2011.07.001](https://doi.org/10.1016/j.dam.2011.07.001) | 2026-02-28 |
| PASS | MaximalIS\<SimpleGraph\> | | `2^n` | O\*(1.4423^n) | MIS enumeration | Moon-Moser 1965; Tomita 2006 | `1.4423^num_vertices` | Tomita et al., TCS 363(1):28–42, 2006 | 2026-02-28 |
| PASS | TSP\<SimpleGraph\> | | `n!` | O\*(2^n · n²) | Held-Karp DP | Held & Karp 1962 | `2^num_vertices` | [DOI:10.1137/0110015](https://doi.org/10.1137/0110015) | 2026-02-28 |
| PASS | MaxCut\<SimpleGraph\> | | `2^n` | O\*(2^{ωn/3}) exp space | 2-CSP optimization | Williams 2005 | `2^(2.372 * num_vertices / 3)` | [DOI:10.1016/j.tcs.2005.09.023](https://doi.org/10.1016/j.tcs.2005.09.023) | 2026-02-28 |
| PASS | KSatisfiability\<K3\> | | `2^n` | O\*(1.307^n) | Biased-PPSZ | Hansen, Kaplan, Zamir & Zwick 2019 | `1.307^num_variables` | [DOI:10.1145/3313276.3316359](https://doi.org/10.1145/3313276.3316359) | 2026-02-28 |
| PASS | ILP | | `exp(n)` | exp(O(n log n)) | FPT algorithm | Dadush 2012 | | [Thesis](https://homepages.cwi.nl/~dadush/papers/dadush-thesis.pdf) | 2026-02-28 |
| PASS | Factoring | | `exp(√n)` | exp(O(n^{1/3} (log n)^{2/3})) | GNFS | Lenstra et al. 1993 | | [Springer](https://link.springer.com/chapter/10.1007/BFb0091539) | 2026-02-28 |
| PASS | SAT (general) | | `2^num_variables` | O\*(2^n) | SETH-tight | | | No sub-2^n for unbounded width | 2026-02-28 |
| PASS | KSatisfiability\<KN\> | | `2^num_variables` | O\*(2^n) | PPSZ base → 2 as k→∞ | | | | 2026-02-28 |
| PASS | CircuitSAT | | `2^num_inputs` | O\*(2^n) | SETH-tight (Williams 2010) | | | Note: `num_inputs()` getter missing | 2026-02-28 |
| PASS | QUBO\<f64\> | | `2^num_vars` | O\*(2^n) | NP-hard | | | No known sub-2^n | 2026-02-28 |
| PASS | SpinGlass (both) | | `2^num_vertices` | O\*(2^n) | NP-hard | Barahona 1982 | | | 2026-02-28 |
| PASS | MaxSetPacking (all) | | `2^num_sets` | O\*(2^m) | Brute-force | | | No known improvement | 2026-02-28 |
| PASS | MinSetCovering | | `2^num_sets` | O\*(2^m) | Brute-force | | | No known improvement | 2026-02-28 |

## Rules (Reduction Overhead)

The overhead formula describes how target problem size relates to source problem size.

| Result | Reduction | Location | Declared | Actual | Fix | Source | Reviewed |
|---|---|---|---|---|---|---|---|
| FAIL | MIS → MaxSetPacking | `maximumindependentset_maximumsetpacking.rs:39` | `universe_size = "num_vertices"` | Universe elements are edge indices (0..num_edges−1) | `universe_size = "num_edges"` | Karp 1972, [DOI:10.1007/978-1-4684-2001-2_9](https://doi.org/10.1007/978-1-4684-2001-2_9) | 2026-02-28 |
| FAIL | MaxSetPacking → MIS | `maximumindependentset_maximumsetpacking.rs:90` | `num_edges = "num_sets"` | Intersection graph: worst case C(n,2) = O(n²) | `num_edges = "num_sets^2"` | Gavril 1972, [DOI:10.1137/0201013](https://doi.org/10.1137/0201013) | 2026-02-28 |
| FAIL | SAT → k-SAT | `sat_ksat.rs:114-117` | `num_clauses = "num_clauses + num_literals"` | Recursive padding underestimates; unit clause → 4 output clauses | `num_clauses = "4 * num_clauses + num_literals"`, `num_vars = "num_vars + 3 * num_clauses + num_literals"` | Sipser 2012, Theorem 7.32 | 2026-02-28 |
| FAIL | Factoring → CircuitSAT | `factoring_circuit.rs:177-180` | `num_variables = "num_bits_first * num_bits_second"` | Each cell creates 6 assignments+variables, plus m+n inputs | `"6 * num_bits_first * num_bits_second + num_bits_first + num_bits_second"` | Paar & Pelzl 2010, [DOI:10.1007/978-3-642-04101-3](https://doi.org/10.1007/978-3-642-04101-3) | 2026-02-28 |
| PASS | MIS ↔ MVC | | `num_vertices = "num_vertices", num_edges = "num_edges"` | | | Gallai 1959; Garey & Johnson 1979 | 2026-02-28 |
| PASS | k-SAT → SAT | | `num_clauses = "num_clauses", num_vars = "num_vars", num_literals = "num_literals"` | | | Trivial embedding | 2026-02-28 |
| PASS | Factoring → ILP | | `num_vars = "2m + 2n + mn"`, `num_constraints = "3mn + m + n + 1"` | | | McCormick 1976, [DOI:10.1007/BF01580665](https://doi.org/10.1007/BF01580665) | 2026-02-28 |
