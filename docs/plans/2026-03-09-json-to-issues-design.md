# Design: Convert G&J Model JSONs to Issue Markdown

**Date:** 2026-03-09

## Goal

Convert 342 JSON files from `references/Garey&Johnson/models/` into GitHub-issue-template markdown files in `references/issues/models/`.

## Source & Target

- **Source:** `references/Garey&Johnson/models/*.json` (342 files)
- **Target:** `references/issues/models/*.md` (342 files, same stem name)

## JSON Format (current)

Each JSON has exactly 4 fields:

```json
{
  "id": "P31",
  "problem_name": "INDEPENDENT SET",
  "source_location": "A1.2 GT20",
  "text": "INSTANCE: ...\nQUESTION: ...\nReference: ...\nComment: ..."
}
```

## Output Template

Standard GitHub issue format (matching `scripts/convert_json_to_issues.py` structure), with only extractable fields filled in:

```markdown
---
name: Problem
about: Propose a new problem type
title: "[Model] {RustName}"
labels: model
assignees: ''
---

## Motivation

{problem_name} ({id}) from Garey & Johnson, {source_location}.
A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, {source_location}

**Mathematical definition:**
{INSTANCE + QUESTION lines extracted from text}

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**
{verbatim text field}

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
```

## Field Extraction Rules

| Template field | Source |
|---------------|--------|
| `[Model] {name}` title | `problem_name` Title-cased, spaces/hyphens removed |
| Motivation line | `problem_name` + `id` + `source_location` |
| **Reference:** | `source_location` |
| Mathematical definition | Lines starting with `INSTANCE:` and `QUESTION:` from `text`; if not found, leave `(TBD)` |
| Full book text | Verbatim `text` field |
| All other sections | `(TBD)` or blank placeholders |

## Execution

6 subagents in parallel, each processing ~57 files, writing directly to `references/issues/models/`.
