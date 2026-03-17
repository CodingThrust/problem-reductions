---
name: Rule
title: "[Rule] EXACT COVER BY 3-SETS to SET PACKING"
labels: rule
status: SKIP_SPECIALIZATION
specialization_of: 'MinimumSetCovering'
---

**Source:** EXACT COVER BY 3-SETS
**Target:** SET PACKING
**Reference:** Garey & Johnson, SP3, p.221

## Specialization Note

This rule's source problem (EXACT COVER BY 3-SETS / X3C) is a specialization of MINIMUM SET COVERING (each set has exactly 3 elements, exact cover required). Implementation should wait until X3C is available as a codebase model.
