# Quick Reference: Known Facts for Issue Fact-Checking

Use this file to cross-check claims in `[Rule]` and `[Model]` issues against established results. Built from the Related Projects in README.md.

---

## 1. Karp DSL (PLDI 2022)

**Source:** [REA1/karp](https://github.com/REA1/karp) — A Racket DSL for writing and testing Karp reductions between NP-complete problems.
**Paper:** Zhang, Hartline & Dimoulas, "Karp: A Language for NP Reductions", PLDI 2022.

### Karp's 21 NP-Complete Problems

All 21 problems from Karp (1972), which the DSL supports:

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

| Class | Description |
|-------|-------------|
| **P** | Deterministic polynomial time |
| **NP** | Nondeterministic polynomial time; "yes" certificates verifiable in poly time |
| **co-NP** | Complements of NP problems |
| **PSPACE** | Polynomial space (contains NP) |
| **EXP** | Exponential time |
| **BPP** | Bounded-error probabilistic polynomial time |
| **BQP** | Bounded-error quantum polynomial time |
| **PH** | Polynomial hierarchy |
| **APX** | Problems with constant-factor approximation |
| **MAX SNP** | Syntactically defined optimization class |

### Canonical NP-Complete Problems (from Complexity Zoo)

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

### Problem Categories

| Code | Category | Problem Count |
|------|----------|---------------|
| GT | Graph Theory | ~60 |
| ND | Network Design | ~30 |
| SP | Sets and Partitions | ~20 |
| SR | Storage and Retrieval | ~10 |
| SS | Sequencing and Scheduling | ~20 |
| MP | Mathematical Programming | ~15 |
| AN | Algebra and Number Theory | ~10 |
| GP | Games and Puzzles | ~5 |
| LO | Logic | ~10 |
| AL | Automata and Language Theory | ~5 |
| PO | Program Optimization | ~5 |
| MS | Miscellaneous | ~10 |

### Graph Theory: Covering and Partitioning

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

| Problem | Type |
|---------|------|
| Minimum Bandwidth | Min |
| Minimum Directed Bandwidth | Min |
| Minimum Linear Arrangement | Min |
| Minimum Cut Linear Arrangement | Min |

### Network Design

| Problem | Type |
|---------|------|
| Minimum Steiner Tree | Min |
| Minimum Biconnectivity Augmentation | Min |
| Minimum k-Connectivity Augmentation | Min |
| Minimum k-Vertex Connected Subgraph | Min |
| Traveling Salesman Problem | Min |
| Maximum Priority Flow | Max |

### Approximability Classes

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

**Source:** The classic reference cataloging 300+ NP-complete problems with reductions. The most cited book in computer science.

### Problem Classification (Garey-Johnson numbering)

Uses same category codes as the Compendium (GT, ND, SP, SS, MP, AN, GP, LO, AL, PO, MS).

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

| Source | Target | Method |
|--------|--------|--------|
| SAT → 3-SAT | Clause splitting | Auxiliary variables for long clauses |
| 3-SAT → 3-Coloring | Variable + palette gadget | |
| 3-SAT → MIS | Clause-variable gadget | Triangle per clause, conflict edges |
| MIS ↔ Vertex Cover | Complement | IS + VC = n |
| MIS ↔ Clique | Complement graph | IS in G = Clique in complement(G) |
| Vertex Cover → Set Cover | Edge-based | Edges as universe, vertex neighborhoods as sets |
| SAT → Min Dominating Set | Triangle gadget | Variable triangle + clause vertices |
| Vertex Cover → Feedback Vertex Set | | |
| Exact Cover → 3D Matching | | |
| Partition → Bin Packing | | |
| SAT → 0-1 Integer Programming | Direct encoding | Variables → integers, clauses → constraints |
| SAT → Max Cut | | |

---

## Cross-Reference: Our Problems vs External Sources

| Our Problem Name | Karp 21 | GJ # | Compendium | Complexity Zoo |
|-----------------|---------|------|------------|----------------|
| MaximumIndependentSet | via Clique complement | GT20 | GT: Subgraph | |
| MinimumVertexCover | #5 Vertex Cover | GT1 | GT: Covering | |
| MaximumClique | #3 Clique | GT5/GT21 | GT: Subgraph | NP-complete |
| MaxCut | #21 Max Cut | GT24 | GT: Subgraph | MAX SNP |
| KColoring | #12 Chromatic Number | GT4 | GT: Covering | NP-complete (k>=3) |
| MinimumDominatingSet | | GT2 | GT: Covering | |
| MaximumMatching | | | GT: Covering | **P** (polynomial) |
| TravelingSalesman | | GT46 | ND | NP-complete |
| Satisfiability | #1 SAT | LO1 | LO | NP-complete |
| KSatisfiability | #11 3-SAT | LO2 | LO | NP-complete |
| MinimumSetCovering | #6 Set Covering | SP5 | SP | |
| MaximumSetPacking | #4 Set Packing | SP3 | SP | |
| BinPacking | | SP13 | SP | |
| ILP | #2 0-1 Integer Prog | MP1 | MP | |
| SpinGlass | | | | |
| QUBO | | | | |
| CircuitSAT | | | | NP-complete |
| Factoring | | | | Not known NP-complete |
| PaintShop | | | | |
| BMF | | | | |
| BicliqueCover | | | GT: Covering | |
| MaximalIS | | | | |
| CVP | | | | |
