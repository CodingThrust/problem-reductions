---
name: Rule
title: "[Rule] 3-PARTITION to DYNAMIC STORAGE ALLOCATION"
labels: rule
status: SKIP_SPECIALIZATION
specialization_of: 'Partition'
---

**Source:** 3-PARTITION
**Target:** DYNAMIC STORAGE ALLOCATION
**Reference:** Garey & Johnson, SR2, p.226

## Specialization Note

This rule's source problem (3-PARTITION) is a specialization of PARTITION (elements partitioned into triples summing to B, with B/4 < s(a) < B/2). Implementation should wait until 3-Partition is available as a codebase model.
