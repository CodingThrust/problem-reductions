---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PLANAR 3SAT to GEOMETRIC CONNECTED DOMINATING SET"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_of: 'KSatisfiability'
canonical_source_name: 'Planar 3-SAT'
canonical_target_name: 'Geometric Connected Dominating Set'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PLANAR 3SAT
**Target:** GEOMETRIC CONNECTED DOMINATING SET
**Motivation:** Skipped — source problem Planar 3-SAT is a specialization of 3-SAT (variable-clause incidence graph must be planar). Implement general 3-SAT reductions first.
**Reference:** Garey & Johnson, *Computers and Intractability*, ND48, p.219

## Specialization Note

- **Planar 3-SAT** restricts 3-SAT to instances whose variable-clause incidence bipartite graph is planar.
- 3-SAT is implemented in the codebase as `KSatisfiability` (k=3) at `src/models/formula/ksat.rs`.
- Planar 3-SAT does not yet have a dedicated model — it would require planarity enforcement on the incidence graph.
- **Blocked on:** Planar 3-SAT model implementation.
