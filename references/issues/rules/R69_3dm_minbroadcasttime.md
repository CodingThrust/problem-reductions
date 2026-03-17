---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-DIMENSIONAL MATCHING to MINIMUM BROADCAST TIME"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_of: 'MaximumSetPacking'
canonical_source_name: '3-Dimensional Matching (3DM)'
canonical_target_name: 'Minimum Broadcast Time'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3-DIMENSIONAL MATCHING
**Target:** MINIMUM BROADCAST TIME
**Motivation:** Skipped — source problem 3DM is a specialization of Set Packing (3-element sets from three disjoint universes). Implement the general Set Packing reductions first.
**Reference:** Garey & Johnson, *Computers and Intractability*, ND49, p.219

## Specialization Note

- **3-Dimensional Matching (3DM)** is a restriction of **Set Packing** where the universe is partitioned into three disjoint sets X, Y, Z and each set in the collection has exactly one element from each.
- `MaximumSetPacking` exists in the codebase at `src/models/set/maximum_set_packing.rs`.
- 3DM itself does not yet have a dedicated model (P128).
- **Blocked on:** 3DM model implementation (P128).
