---
name: test-feature
description: Use when testing a project feature from a user's perspective — simulates a downstream user who reads README and docs, then writes and runs code exercising the feature end-to-end, reporting on discoverability, functionality, and doc quality
---

## Test Feature

Simulates a downstream user who wants to use a specific project feature. The user reads README and documentation, writes code exercising the feature, and reports whether the experience matches what the docs promise.

**Input:** User specifies feature name(s) (e.g., `/test-feature MCP server`). If none specified, discover features from README/docs and test all.

---

### Step 0 — Discover Features

1. Read `README.md`, doc files, and project structure to identify user-facing features (e.g., "library API", "CLI tool", "MCP server", "plugin system").
2. Determine scope:
   - User specified feature(s): test only those
   - User specified nothing: list discovered features, then test all
3. For each feature, collect the relevant doc sections — installation instructions, usage examples, API references.

Print a brief summary of features to test. Proceed immediately.

### Step 1 — Test Each Feature

For each feature, dispatch a **subagent** (via Agent tool). Give the subagent:

- **Role:** A lightweight user description relevant to the feature (e.g., "a researcher who wants to solve optimization problems via CLI", "a developer integrating this library into their web service"). Not a full persona — just enough to set expectations and domain knowledge level.
- **Docs:** The README and relevant doc excerpts for this feature.
- **Instructions:**

```
You are [role description]. You want to use the "[feature]" capability of this project.

Here is the README:
[content]

Here are the relevant docs:
[excerpts]

Your task — act as a real user:
1. Read the docs to figure out how to use "[feature]".
2. Follow the installation/setup instructions.
3. Write and run code (or commands) that exercises the feature meaningfully.
4. Report back with:
   - **Discoverability:** Could you figure out how to use this from docs alone? What was missing?
   - **Setup:** Did installation/setup work as described?
   - **Functionality:** Did the feature work? What succeeded, what failed?
   - **Friction points:** What was confusing, misleading, or undocumented?
   - **Doc suggestions:** What would you add or change in the docs?
```

**Parallelism:** Independent features can be tested in parallel via multiple subagents.

### Step 2 — Report

Gather results from all subagents. Save report to `docs/test-reports/test-feature-<YYYYMMDD-HHMMSS>.md`:

```markdown
# Feature Test Report: [project name]

**Date:** [timestamp]
**Features tested:** [list]

## Summary

| Feature | Discoverable | Setup | Works | Doc Quality |
|---------|-------------|-------|-------|-------------|
| CLI tool | yes | yes | yes | good |
| MCP server | partial | yes | yes | missing config example |
| Library API | yes | yes | no | outdated example |

## Per-Feature Details

### [feature name]
- **Role:** [who the simulated user was]
- **What they tried:** [brief description]
- **Discoverability:** [could they find how to use it from docs alone?]
- **Setup:** [did installation work as described?]
- **Functionality:** [what worked, what didn't]
- **Friction points:** [what was confusing or missing]
- **Doc suggestions:** [what would help a real user]

## Issues Found
[problems discovered, ordered by severity]

## Suggestions
[actionable improvements ordered by impact]
```

Present the report path. Offer to fix documentation gaps or re-test specific features.
