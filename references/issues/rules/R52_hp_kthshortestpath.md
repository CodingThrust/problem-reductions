---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to K-th SHORTEST PATH"
labels: rule
assignees: ''
status: SKIP_TURING
canonical_source_name: 'Hamiltonian Path'
canonical_target_name: 'K-th Shortest Path'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** HAMILTONIAN PATH
**Target:** K-th SHORTEST PATH
**Motivation:** Skipped — GJ specifies a **Turing reduction** (not a Karp/many-one reduction). Cannot be implemented as a single `ReduceTo` trait.
**Reference:** Garey & Johnson, *Computers and Intractability*, ND31, p.214

## Note

This reduction is a **Turing reduction** (the target problem is used as an oracle multiple times), not a polynomial-time many-one (Karp) reduction. The GJ entry explicitly states "Turing reduction from HAMILTONIAN PATH" and notes the problem is "Not known to be in NP" (marked with `*`).

The codebase's `ReduceTo<T>` trait models Karp reductions (single transformation + solution extraction). A Turing reduction would require a different abstraction (oracle access pattern).
