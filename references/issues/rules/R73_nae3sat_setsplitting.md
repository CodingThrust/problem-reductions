---
name: Rule
title: "[Rule] NOT-ALL-EQUAL 3SAT to SET SPLITTING"
labels: rule
status: SKIP_SPECIALIZATION
specialization_of: 'KSatisfiability'
---

**Source:** NOT-ALL-EQUAL 3SAT
**Target:** SET SPLITTING
**Reference:** Garey & Johnson, SP4, p.221

## Specialization Note

This rule's source problem (NOT-ALL-EQUAL 3SAT) is a specialization of 3-SAT (KSatisfiability with k=3), requiring that no clause has all literals true. Implementation should wait until NAE 3SAT is available as a codebase model.
