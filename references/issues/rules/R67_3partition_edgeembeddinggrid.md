---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-PARTITION to EDGE EMBEDDING ON A GRID"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_of: 'Partition'
canonical_source_name: '3-Partition'
canonical_target_name: 'Edge Embedding on a Grid'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3-PARTITION
**Target:** EDGE EMBEDDING ON A GRID
**Motivation:** Skipped — source problem 3-Partition is a specialization of Partition (partition into triples with size constraints). Implement the general Partition model and reductions first.
**Reference:** Garey & Johnson, *Computers and Intractability*, ND47, p.219

## Specialization Note

- **3-Partition** is a restriction of **Partition** where elements must be grouped into triples, each summing to a target value.
- Neither Partition (P139) nor 3-Partition (P142) has a codebase implementation yet.
- **Blocked on:** Partition model implementation (P139), then 3-Partition (P142).
