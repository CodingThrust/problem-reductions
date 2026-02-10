# Improve Example Instances

**Date**: 2026-02-10
**Branch**: TBD
**Status**: Design approved

## Problem

All 30 reduction examples use trivially small instances (P4 path graph, K3 triangle, 2-3 variable SAT formulas). The P4 path graph alone appears in 8 examples. These produce unserious-looking data for the paper and don't illustrate interesting reduction behavior.

## Design Decisions

- **Purpose**: Examples are primarily data generators for the Typst paper (JSON export)
- **Size range**: 6-10 variables per instance
- **Strategy**: One canonical graph (Petersen) for all graph problems, with exceptions only where necessary
- **Solver**: BruteForce, so target problem must have ≤ ~25 variables

## Instance Plan

### 1. Petersen Graph (10 vertices, 15 edges)

Canonical graph for all graph-based problems. 3-regular, non-bipartite, girth 5.

**Properties**: MIS=4, VC=6, Matching=5 (perfect), DominatingSet=3, ChromaticNumber=3, Clique=2.

**Edge list**: `(0,1), (0,4), (0,5), (1,2), (1,6), (2,3), (2,7), (3,4), (3,8), (4,9), (5,7), (5,8), (6,8), (6,9), (7,9)`

**Used by** (20 examples):
- `mis_to_qubo`, `mis_to_ilp`, `mis_to_mvc`, `mis_to_msp`
- `mvc_to_ilp`, `mvc_to_qubo`, `mvc_to_mis`, `mvc_to_msc`
- `mm_to_ilp`, `mm_to_msp`
- `mds_to_ilp`
- `coloring_to_ilp`, `coloring_to_qubo`
- `maxcut_to_spinglass`
- `spinglass_to_maxcut` (MaxCut on Petersen topology)

### 2. Octahedron (6 vertices, 12 edges)

For MaximumClique example. Complete tripartite K_{2,2,2}. Clique number = 3.

**Used by**: `mclique_to_ilp`

### 3. Random 3-SAT (5 variables, ~7 clauses)

Hand-picked instance with ~2-4 satisfying assignments. Ratio ~1.4. Compact enough to display inline in the paper. Target MIS graph has 21 vertices (BruteForce feasible).

**Used by** (4 examples):
- `sat_to_mis`, `sat_to_coloring`, `sat_to_mds`, `sat_to_ksat`

### 4. Petersen SpinGlass (10 spins, 15 couplings)

SpinGlass on Petersen graph topology with random ±1 couplings (frustrated). Shared between SpinGlass ↔ QUBO ↔ MaxCut conversions.

**Used by** (3 examples):
- `spinglass_to_qubo`, `spinglass_to_maxcut`, `qubo_to_spinglass`

### 5. Knapsack/Assignment ILP (6-8 variables)

Proper knapsack or assignment problem with non-trivial constraints and slack variables.

**Used by**: `ilp_to_qubo`

### 6. 2-bit Adder Circuit

Multi-gate circuit (replaces single AND gate). Shows meaningful CircuitSAT structure.

**Used by**: `circuit_to_spinglass`

### 7. Factor 35 = 5 x 7 (3-bit x 3-bit)

Product of two primes, 6 binary variables. BruteForce feasible (2^6 = 64).

**Used by** (2 examples):
- `factoring_to_circuit`, `factoring_to_ilp`

### 8. 5-variable 3-SAT (reused from #3)

Same SAT instance used for kSAT → QUBO.

**Used by**: `ksatisfiability_to_qubo`

## Paper Display

For Petersen-based reductions producing 10x10 QUBO matrices: show as compact figures rather than inline matrices, or show key properties (dimension, sparsity, penalty weight) instead of full matrix.

## Scope

Only instance data changes. The example structure (create -> reduce -> solve -> extract -> export JSON) stays the same. No API changes.

## Verification

```bash
make test clippy export-graph
```
