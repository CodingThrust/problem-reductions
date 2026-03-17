# JSON → Issue Markdown Conversion (Models) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Convert all 342 JSON files in `references/Garey&Johnson/models/` into GitHub-issue-template markdown files in `references/issues/models/`.

**Architecture:** A single focused Python script reads each JSON (which has 4 fields: `id`, `problem_name`, `source_location`, `text`), extracts what it can (math definition from INSTANCE/QUESTION lines), and writes the standard issue template with TBD placeholders for unpopulated sections. Run via a subagent with file write access.

**Tech Stack:** Python 3 (stdlib only), `pathlib`, `json`, `re`

---

### Task 1: Write the conversion script

**Files:**
- Create: `scripts/json_models_to_issues.py`

**Step 1: Create the script**

```python
#!/usr/bin/env python3
"""Convert Garey & Johnson flat-format model JSONs to GitHub issue markdown.

Source: references/Garey&Johnson/models/*.json
Target: references/issues/models/*.md

Each JSON has exactly: id, problem_name, source_location, text.
Only these fields are populated; all others are left as (TBD).
"""

import json
import re
from pathlib import Path


def to_rust_name(problem_name: str) -> str:
    """Convert 'INDEPENDENT SET' -> 'IndependentSet' (Title-cased, no spaces)."""
    return "".join(w.capitalize() for w in re.split(r"[\s\-]+", problem_name))


def extract_math_def(text: str) -> str:
    """Extract INSTANCE + QUESTION lines from text, or return empty string."""
    lines = text.split("\n")
    result = []
    in_section = False
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("INSTANCE:") or stripped.startswith("QUESTION:"):
            in_section = True
            result.append(stripped)
        elif in_section and stripped.startswith(("Reference:", "Comment:", "Note:")):
            break
        elif in_section and stripped:
            # continuation line of INSTANCE/QUESTION
            result.append(stripped)
    return "\n".join(result)


def model_json_to_md(data: dict) -> str:
    pid = data.get("id", "")
    problem_name = data.get("problem_name", "")
    source_loc = data.get("source_location", "")
    text = data.get("text", "")

    rust_name = to_rust_name(problem_name)
    math_def = extract_math_def(text)
    math_def_section = math_def if math_def else "(TBD)"

    lines = [
        "---",
        "name: Problem",
        "about: Propose a new problem type",
        f'title: "[Model] {rust_name}"',
        "labels: model",
        "assignees: ''",
        "---",
        "",
        "## Motivation",
        "",
        f"{problem_name} ({pid}) from Garey & Johnson, {source_loc}. "
        "A classical NP-complete problem useful for reductions.",
        "",
        "## Definition",
        "",
        "**Name:** (TBD — Rust name)",
        f"**Reference:** Garey & Johnson, *Computers and Intractability*, {source_loc}",
        "",
        "**Mathematical definition:**",
        "",
        math_def_section,
        "",
        "## Variables",
        "",
        "- **Count:** (TBD)",
        "- **Per-variable domain:** (TBD)",
        "- **Meaning:** (TBD)",
        "",
        "## Schema (data type)",
        "",
        "**Type name:** (TBD)",
        "**Variants:** (TBD)",
        "",
        "| Field | Type | Description |",
        "|-------|------|-------------|",
        "| (TBD) | (TBD) | (TBD) |",
        "",
        "## Complexity",
        "",
        "- **Best known exact algorithm:** (TBD)",
        "",
        "## Extra Remark",
        "",
        "**Full book text:**",
        "",
        text,
        "",
        "## How to solve",
        "",
        "- [ ] It can be solved by (existing) bruteforce.",
        "- [ ] It can be solved by reducing to integer programming.",
        "- [ ] Other: (TBD)",
        "",
        "## Example Instance",
        "",
        "(TBD)",
        "",
    ]
    return "\n".join(lines)


def main():
    base = Path(__file__).parent.parent / "references"
    src_dir = base / "Garey&Johnson" / "models"
    out_dir = base / "issues" / "models"
    out_dir.mkdir(parents=True, exist_ok=True)

    count = 0
    for f in sorted(src_dir.glob("*.json")):
        with open(f) as fp:
            data = json.load(fp)
        md = model_json_to_md(data)
        out_file = out_dir / f"{f.stem}.md"
        out_file.write_text(md)
        count += 1

    print(f"Converted {count} model JSONs → {out_dir}")


if __name__ == "__main__":
    main()
```

**Step 2: Verify the script runs on a sample**

```bash
cd /Users/xiweipan/Codes/problem-reductions
python3 scripts/json_models_to_issues.py
```

Expected output:
```
Converted 342 model JSONs → .../references/issues/models
```

**Step 3: Spot-check 3 output files**

```bash
# One with INSTANCE/QUESTION pattern
head -50 references/issues/models/P31_independent_set.md

# One without INSTANCE/QUESTION (prose only)
head -50 references/issues/models/P1_directed_hamiltonian_path.md

# One with complex math
head -50 references/issues/models/P98_traveling_salesman.md
```

Expected for P31: `## Definition` section shows `INSTANCE: Graph G = (V,E)...` + `QUESTION: Does G contain...`

Expected for P1: `## Definition` section shows `(TBD)` (no INSTANCE/QUESTION in text)

**Step 4: Count output files**

```bash
ls references/issues/models/*.md | wc -l
```

Expected: `342`

---

### Notes

- The script lives separately from `scripts/convert_json_to_issues.py` (which handles a richer augmented JSON format — different purpose)
- No web search, no inference beyond INSTANCE/QUESTION extraction
- All generated fields marked `(TBD)` will be filled in later by `add-model` workflow
