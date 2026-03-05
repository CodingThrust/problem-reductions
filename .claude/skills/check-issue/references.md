# Quick Reference: Known Facts for Issue Fact-Checking

Use this file to cross-check claims in `[Rule]` and `[Model]` issues against established results. Built from the Related Projects in README.md. Each entry includes source URLs for traceability.

---

## 1. Karp DSL (PLDI 2022)

**Source:** [REA1/karp](https://github.com/REA1/karp) — A Racket DSL for writing and testing Karp reductions between NP-complete problems.
**Paper:** Zhang, Hartline & Dimoulas, ["Karp: A Language for NP Reductions"](https://dl.acm.org/doi/abs/10.1145/3519939.3523732), PLDI 2022. ([PDF](https://users.cs.northwestern.edu/~chrdimo/pubs/pldi22-zhd.pdf))
**Problem definitions:** [karp/problem-definition/](https://github.com/REA1/karp/tree/main/problem-definition)
**Reduction implementations:** [karp/reduction/](https://github.com/REA1/karp/tree/main/reduction)

### Karp's 21 NP-Complete Problems

All 21 problems from [Karp (1972)](https://en.wikipedia.org/wiki/Karp%27s_21_NP-complete_problems), which the DSL supports:

| # | Problem | Category |
|---|---------|----------|
| 1 | Satisfiability (SAT) | Logic |
| 2 | 0-1 Integer Programming | Mathematical Programming |
| 3 | Clique | Graph Theory |
| 4 | Set Packing | Sets and Partitions |
| 5 | Vertex Cover | Graph Theory |
| 6 | Set Covering | Sets and Partitions |
| 7 | Feedback Node Set | Graph Theory |
| 8 | Feedback Arc Set | Graph Theory |
| 9 | Directed Hamiltonian Circuit | Graph Theory |
| 10 | Undirected Hamiltonian Circuit | Graph Theory |
| 11 | 3-SAT | Logic |
| 12 | Chromatic Number (Graph Coloring) | Graph Theory |
| 13 | Clique Cover | Graph Theory |
| 14 | Exact Cover | Sets and Partitions |
| 15 | Hitting Set | Sets and Partitions |
| 16 | Steiner Tree | Network Design |
| 17 | 3-Dimensional Matching | Sets and Partitions |
| 18 | Knapsack | Mathematical Programming |
| 19 | Job Sequencing | Scheduling |
| 20 | Partition | Sets and Partitions |
| 21 | Max Cut | Graph Theory |

### Karp's Reduction Tree (source → target)

Source: [Wikipedia: Karp's 21 NP-complete problems](https://en.wikipedia.org/wiki/Karp%27s_21_NP-complete_problems)

```
SAT
├── 3-SAT
│   ├── Chromatic Number (3-Coloring)
│   ├── Clique
│   │   ├── Set Packing
│   │   └── Vertex Cover
│   │       ├── Feedback Node Set
│   │       │   └── Directed Hamiltonian Circuit
│   │       │       └── Undirected Hamiltonian Circuit
│   │       └── Set Covering
│   │           ├── Steiner Tree
│   │           └── Hitting Set
│   └── Exact Cover
│       ├── 3-Dimensional Matching
│       └── Knapsack
│           ├── Job Sequencing
│           └── Partition
├── 0-1 Integer Programming
├── Clique Cover
├── Feedback Arc Set
└── Max Cut
```

**Key reductions:**
- SAT → 3-SAT (clause splitting with auxiliary variables)
- 3-SAT → Clique (clause-variable gadget)
- Clique → Vertex Cover (complement graph: VC = n - Clique)
- Clique → Set Packing (clique edges as sets)
- Vertex Cover → Set Covering (edges as universe, vertices as sets)
- Vertex Cover → Feedback Node Set
- Exact Cover → 3-Dimensional Matching
- Exact Cover → Knapsack
- SAT → Max Cut

---

## 2. Complexity Zoo

**Source:** [complexityzoo.net](https://complexityzoo.net/) — Comprehensive catalog of 550+ complexity classes (Scott Aaronson).

### Key Complexity Classes

| Class | Description | Source |
|-------|-------------|--------|
| **P** | Deterministic polynomial time | [P](https://complexityzoo.net/Complexity_Zoo:P#p) |
| **NP** | Nondeterministic polynomial time; "yes" certificates verifiable in poly time | [NP](https://complexityzoo.net/Complexity_Zoo:N#np) |
| **co-NP** | Complements of NP problems | [co-NP](https://complexityzoo.net/Complexity_Zoo:C#conp) |
| **PSPACE** | Polynomial space (contains NP) | [PSPACE](https://complexityzoo.net/Complexity_Zoo:P#pspace) |
| **EXP** | Exponential time | [EXP](https://complexityzoo.net/Complexity_Zoo:E#exp) |
| **BPP** | Bounded-error probabilistic polynomial time | [BPP](https://complexityzoo.net/Complexity_Zoo:B#bpp) |
| **BQP** | Bounded-error quantum polynomial time | [BQP](https://complexityzoo.net/Complexity_Zoo:B#bqp) |
| **PH** | Polynomial hierarchy | [PH](https://complexityzoo.net/Complexity_Zoo:P#ph) |
| **APX** | Problems with constant-factor approximation | [APX](https://complexityzoo.net/Complexity_Zoo:A#apx) |
| **MAX SNP** | Syntactically defined optimization class | [MAX SNP](https://complexityzoo.net/Complexity_Zoo:M#maxsnp) |

### Canonical NP-Complete Problems (from Complexity Zoo)

Source: [Complexity Zoo: NP](https://complexityzoo.net/Complexity_Zoo:N#np)

- **SAT** (Boolean satisfiability) — the first NP-complete problem (Cook 1971)
- **3-Colorability** — Can vertices be colored with 3 colors, no adjacent same color?
- **Hamiltonian Cycle** — Does a cycle visiting each vertex exactly once exist?
- **Traveling Salesperson** — Is there a tour within distance T?
- **Maximum Clique** — Do k mutually-adjacent vertices exist?
- **Subset Sum** — Does a subset sum to exactly x?

### Key Class Relationships

- P vs NP: Open problem; unequal relative to random oracles
- NP = co-NP iff PH collapses
- NP ⊆ PSPACE (Savitch's theorem)
- If NP ⊆ P/poly then PH collapses to Σ₂P

---

## 3. Compendium of NP Optimization Problems

**Source:** [csc.kth.se/tcs/compendium](https://www.csc.kth.se/tcs/compendium/) — Online catalog of NP optimization problems with approximability results (Crescenzi & Kann).
**Problem list index:** [node6.html](https://www.csc.kth.se/tcs/compendium/node6.html)

### Problem Categories

| Code | Category | Source |
|------|----------|--------|
| GT | Graph Theory | [Covering](https://www.csc.kth.se/tcs/compendium/node9.html), [Subgraph](https://www.csc.kth.se/tcs/compendium/node32.html), [Ordering](https://www.csc.kth.se/tcs/compendium/node52.html), [Iso/Homo](https://www.csc.kth.se/tcs/compendium/node56.html), [Connectivity](https://www.csc.kth.se/tcs/compendium/node100.html) |
| ND | Network Design | [node](https://www.csc.kth.se/tcs/compendium/node104.html) |
| SP | Sets and Partitions | [node](https://www.csc.kth.se/tcs/compendium/node120.html) |
| SR | Storage and Retrieval | [node](https://www.csc.kth.se/tcs/compendium/node135.html) |
| SS | Sequencing and Scheduling | [node](https://www.csc.kth.se/tcs/compendium/node140.html) |
| MP | Mathematical Programming | [node](https://www.csc.kth.se/tcs/compendium/node155.html) |
| AN | Algebra and Number Theory | [node](https://www.csc.kth.se/tcs/compendium/node163.html) |
| GP | Games and Puzzles | [node](https://www.csc.kth.se/tcs/compendium/node167.html) |
| LO | Logic | [node](https://www.csc.kth.se/tcs/compendium/node170.html) |
| AL | Automata and Language Theory | [node](https://www.csc.kth.se/tcs/compendium/node175.html) |
| PO | Program Optimization | [node](https://www.csc.kth.se/tcs/compendium/node178.html) |
| MS | Miscellaneous | [node](https://www.csc.kth.se/tcs/compendium/node180.html) |

### Graph Theory: Covering and Partitioning

Source: [node9.html](https://www.csc.kth.se/tcs/compendium/node9.html)

| Problem | Type |
|---------|------|
| Minimum Vertex Cover | Min |
| Minimum Dominating Set | Min |
| Maximum Domatic Partition | Max |
| Minimum Edge Dominating Set | Min |
| Minimum Independent Dominating Set | Min |
| Minimum Graph Coloring (Chromatic Number) | Min |
| Minimum Color Sum | Min |
| Maximum Achromatic Number | Max |
| Minimum Edge Coloring | Min |
| Minimum Feedback Vertex Set | Min |
| Minimum Feedback Arc Set | Min |
| Minimum Maximal Matching | Min |
| Maximum Triangle Packing | Max |
| Maximum H-Matching | Max |
| Minimum Clique Partition | Min |
| Minimum Clique Cover | Min |
| Minimum Complete Bipartite Subgraph Cover | Min |

### Graph Theory: Subgraph Problems

Source: [node32.html](https://www.csc.kth.se/tcs/compendium/node32.html)

| Problem | Type |
|---------|------|
| Maximum Clique | Max |
| Maximum Independent Set | Max |
| Maximum Independent Sequence | Max |
| Maximum Induced Subgraph with Property P | Max |
| Minimum Vertex Deletion (Subgraph Property) | Min |
| Minimum Edge Deletion (Subgraph Property) | Min |
| Maximum Degree-Bounded Connected Subgraph | Max |
| Maximum Planar Subgraph | Max |
| Maximum K-Colorable Subgraph | Max |
| Maximum Subforest | Max |
| Minimum Interval Graph Completion | Min |
| Minimum Chordal Graph Completion | Min |

### Graph Theory: Vertex Ordering

Source: [node52.html](https://www.csc.kth.se/tcs/compendium/node52.html)

| Problem | Type |
|---------|------|
| Minimum Bandwidth | Min |
| Minimum Directed Bandwidth | Min |
| Minimum Linear Arrangement | Min |
| Minimum Cut Linear Arrangement | Min |

### Network Design

Source: [node104.html](https://www.csc.kth.se/tcs/compendium/node104.html)

| Problem | Type |
|---------|------|
| Minimum Steiner Tree | Min |
| Minimum Biconnectivity Augmentation | Min |
| Minimum k-Connectivity Augmentation | Min |
| Minimum k-Vertex Connected Subgraph | Min |
| Traveling Salesman Problem | Min |
| Maximum Priority Flow | Max |

### Approximability Classes

Source: [node3.html](https://www.csc.kth.se/tcs/compendium/node3.html)

| Class | Meaning |
|-------|---------|
| **PO** | Polynomial-time solvable exactly |
| **FPTAS** | Fully polynomial-time approximation scheme |
| **PTAS** | Polynomial-time approximation scheme |
| **APX** | Constant-factor approximation |
| **poly-APX** | Polynomial-factor approximation |
| **NPO** | General NP optimization (may have no good approximation) |

---

## 4. Computers and Intractability (Garey & Johnson, 1979)

**Source:** Garey & Johnson, *Computers and Intractability: A Guide to the Theory of NP-Completeness*, W. H. Freeman, 1979. ISBN 0-7167-1045-5. No online version; see [WorldCat](https://www.worldcat.org/title/4527557).

### Problem Classification (Garey-Johnson numbering)

Uses same category codes as the Compendium (GT, ND, SP, SS, MP, AN, GP, LO, AL, PO, MS). Problems listed in Appendix A (pp. 187-213).

### Key Problems and Their GJ Numbers

| GJ # | Problem | Our Name |
|------|---------|----------|
| GT1 | Minimum Vertex Cover | MinimumVertexCover |
| GT2 | Minimum Independent Dominating Set | MinimumDominatingSet |
| GT4 | Graph Coloring (Chromatic Number) | KColoring |
| GT5 | Clique | MaximumClique |
| GT20 | Maximum Independent Set | MaximumIndependentSet |
| GT21 | Maximum Clique | MaximumClique |
| GT24 | Maximum Cut | MaxCut |
| GT34 | Hamiltonian Circuit | — |
| GT39 | Feedback Vertex Set | — |
| GT46 | Traveling Salesman | TravelingSalesman |
| ND5 | Steiner Tree in Graphs | — |
| SP1 | 3-Dimensional Matching | — |
| SP2 | Partition | — |
| SP5 | Set Covering | MinimumSetCovering |
| SP3 | Set Packing | MaximumSetPacking |
| SP13 | Bin Packing | BinPacking |
| SS1 | Multiprocessor Scheduling | — |
| MP1 | Integer Programming | ILP |
| LO1 | Satisfiability (SAT) | Satisfiability |
| LO2 | 3-Satisfiability | KSatisfiability |

### Classic Reductions from Garey & Johnson

Source: Chapter 3, "Proving NP-Completeness Results" (pp. 45-89).

| Source → Target | Method |
|-----------------|--------|
| SAT → 3-SAT | Auxiliary variables for long clauses |
| 3-SAT → 3-Coloring | Variable + palette gadget |
| 3-SAT → MIS | Triangle per clause, conflict edges |
| MIS ↔ Vertex Cover | Complement: IS + VC = n |
| MIS ↔ Clique | IS in G = Clique in complement(G) |
| Vertex Cover → Set Cover | Edges as universe, vertex neighborhoods as sets |
| SAT → Min Dominating Set | Variable triangle + clause vertices |
| Vertex Cover → Feedback Vertex Set | |
| Exact Cover → 3D Matching | |
| Partition → Bin Packing | |
| SAT → 0-1 Integer Programming | Variables → integers, clauses → constraints |
| SAT → Max Cut | |

---

## Cross-Reference: Our Problems vs External Sources

| Our Problem Name | Karp 21 | GJ # | Compendium | Complexity Zoo |
|-----------------|---------|------|------------|----------------|
| MaximumIndependentSet | via Clique complement | GT20 | [GT: Subgraph](https://www.csc.kth.se/tcs/compendium/node32.html) | |
| MinimumVertexCover | #5 Vertex Cover | GT1 | [GT: Covering](https://www.csc.kth.se/tcs/compendium/node9.html) | |
| MaximumClique | #3 Clique | GT5/GT21 | [GT: Subgraph](https://www.csc.kth.se/tcs/compendium/node32.html) | [NP-complete](https://complexityzoo.net/Complexity_Zoo:N#np) |
| MaxCut | #21 Max Cut | GT24 | [GT: Subgraph](https://www.csc.kth.se/tcs/compendium/node32.html) | [MAX SNP](https://complexityzoo.net/Complexity_Zoo:M#maxsnp) |
| KColoring | #12 Chromatic Number | GT4 | [GT: Covering](https://www.csc.kth.se/tcs/compendium/node9.html) | NP-complete (k>=3) |
| MinimumDominatingSet | | GT2 | [GT: Covering](https://www.csc.kth.se/tcs/compendium/node9.html) | |
| MaximumMatching | | | [GT: Covering](https://www.csc.kth.se/tcs/compendium/node9.html) | **P** (polynomial) |
| TravelingSalesman | | GT46 | [ND](https://www.csc.kth.se/tcs/compendium/node104.html) | [NP-complete](https://complexityzoo.net/Complexity_Zoo:N#np) |
| Satisfiability | #1 SAT | LO1 | [LO](https://www.csc.kth.se/tcs/compendium/node170.html) | [NP-complete](https://complexityzoo.net/Complexity_Zoo:N#np) |
| KSatisfiability | #11 3-SAT | LO2 | [LO](https://www.csc.kth.se/tcs/compendium/node170.html) | [NP-complete](https://complexityzoo.net/Complexity_Zoo:N#np) |
| MinimumSetCovering | #6 Set Covering | SP5 | [SP](https://www.csc.kth.se/tcs/compendium/node120.html) | |
| MaximumSetPacking | #4 Set Packing | SP3 | [SP](https://www.csc.kth.se/tcs/compendium/node120.html) | |
| BinPacking | | SP13 | [SP](https://www.csc.kth.se/tcs/compendium/node120.html) | |
| ILP | #2 0-1 Integer Prog | MP1 | [MP](https://www.csc.kth.se/tcs/compendium/node155.html) | |
| SpinGlass | | | | |
| QUBO | | | | |
| CircuitSAT | | | | [NP-complete](https://complexityzoo.net/Complexity_Zoo:N#np) |
| Factoring | | | | Not known NP-complete |
| PaintShop | | | | |
| BMF | | | | |
| BicliqueCover | | | [GT: Covering](https://www.csc.kth.se/tcs/compendium/node9.html) | |
| MaximalIS | | | | |
| CVP | | | | |
