---
name: decision-vs-optimization-framing
description: When a model issue is ambiguous about decision vs optimization, check associated [Rule] issues first to determine which version to implement
type: feedback
---

When a `[Model]` issue is ambiguous about decision vs optimization framing, always check associated `[Rule]` issues first to determine which version the rules reference. Implement whichever version the rules target. If rules reference both versions, split into two separate model issues.

**Why:** Issue #233 (StrongConnectivityAugmentation) was ambiguous. Rule #254 (Hamiltonian Circuit → SCA) targets the decision version (with budget B). The user wants the rule to drive which model version gets implemented — don't guess or ask the user when the answer is already in the associated rules.

**How to apply:** In `/fix-issue` Step 5 (brainstorming substantive issues), when encountering "Decision vs optimization framing", look up associated `[Rule]` issues by searching for the problem name in rule titles/bodies. Present findings to the user with a concrete recommendation based on what the rules need.
