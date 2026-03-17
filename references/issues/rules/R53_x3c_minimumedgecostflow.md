---
name: Rule
about: Propose a new reduction rule
title: "[Rule] EXACT COVER BY 3-SETS to MINIMUM EDGE-COST FLOW"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_of: 'MinimumSetCovering'
canonical_source_name: 'Exact Cover by 3-Sets (X3C)'
canonical_target_name: 'Minimum Edge-Cost Flow'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** EXACT COVER BY 3-SETS
**Target:** MINIMUM EDGE-COST FLOW
**Motivation:** Skipped — source problem X3C is a specialization of Set Covering (each set has exactly 3 elements, exact cover required). Implement the general Set Covering reductions first.
**Reference:** Garey & Johnson, *Computers and Intractability*, ND32, p.214

## Specialization Note

- **X3C** (Exact Cover by 3-Sets) is a restriction of **Set Covering** where each set has exactly 3 elements and an exact cover is required.
- `MinimumSetCovering` exists in codebase at `src/models/set/minimum_set_covering.rs`.
- X3C itself does not yet have a dedicated model. It could be modeled as a `MinimumSetCovering` instance with constraints, or as a separate type.
- **Blocked on:** X3C model implementation (P129).
