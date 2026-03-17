# JSON → Rule Issue Markdown Conversion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Convert all 335 JSON files in `references/Garey&Johnson/reductions/` into GitHub-issue-template markdown files in `references/issues/rules/`.

**Architecture:** A single focused Python script reads each JSON (fields: `id`, `from_problem`, `to_problem`, `source_location`, `text`), converts problem names to PascalCase, extracts the `Reference:` line from text if present, and writes the standard rule issue template with TBD placeholders for unpopulated sections.

**Tech Stack:** Python 3 (stdlib only), `pathlib`, `json`, `re`

---

### Task 1: Write the conversion script

**Files:**
- Create: `scripts/json_rules_to_issues.py`

**Step 1: Create the script**

```python
#!/usr/bin/env python3
"""Convert Garey & Johnson reduction JSONs to GitHub rule issue markdown.

Source: references/Garey&Johnson/reductions/*.json
Target: references/issues/rules/*.md

Each JSON has: id, from_problem, to_problem, source_location, text.
"""

import json
import re
from pathlib import Path


def to_rust_name(problem_name: str) -> str:
    """Convert 'VERTEX COVER' -> 'VertexCover', '3-SAT (3SAT)' -> '3Sat'.

    Steps:
    1. Strip parenthetical abbreviations like (3SAT) or (SAT).
    2. Split on whitespace and hyphens.
    3. Capitalize each token and join.
    """
    # Strip content in parentheses
    name = re.sub(r"\s*\(.*?\)", "", problem_name).strip()
    return "".join(w.capitalize() for w in re.split(r"[\s\-]+", name) if w)


def extract_reference_line(text: str) -> str:
    """Extract the 'Reference: ...' line from text, or return empty string."""
    for line in text.split("\n"):
        if re.match(r"\s*Reference:", line):
            return line.strip()
    return ""


def rule_json_to_md(data: dict) -> str:
    pid = data.get("id", "")
    from_problem = data.get("from_problem", "")
    to_problem = data.get("to_problem", "")
    source_loc = data.get("source_location", "")
    text = data.get("text", "")

    from_rust = to_rust_name(from_problem)
    to_rust = to_rust_name(to_problem)

    # Build reference field
    ref_line = extract_reference_line(text)
    gj_ref = f"Garey & Johnson, *Computers and Intractability*, {source_loc}"
    if ref_line:
        # ref_line is e.g. "Reference: [Author, 1976]. Transformation from X."
        ref_detail = re.sub(r"^Reference:\s*", "", ref_line)
        reference = f"{gj_ref}; {ref_detail}"
    else:
        reference = gj_ref

    lines = [
        "---",
        "name: Rule",
        "about: Propose a new reduction rule",
        f'title: "[Rule] {from_rust} to {to_rust}"',
        "labels: rule",
        "assignees: ''",
        "---",
        "",
        f"**Source:** {from_rust}",
        f"**Target:** {to_rust}",
        "**Motivation:** (TBD)",
        f"**Reference:** {reference}",
        "",
        "## Reduction Algorithm",
        "",
        text,
        "",
        "## Size Overhead",
        "",
        "| Target metric (code name) | Polynomial (using symbols above) |",
        "|----------------------------|----------------------------------|",
        "| (TBD) | (TBD) |",
        "",
        "## Validation Method",
        "",
        "(TBD)",
        "",
        "## Example",
        "",
        "(TBD)",
        "",
    ]
    return "\n".join(lines)


def main():
    base = Path(__file__).parent.parent / "references"
    src_dir = base / "Garey&Johnson" / "reductions"
    out_dir = base / "issues" / "rules"
    out_dir.mkdir(parents=True, exist_ok=True)

    count = 0
    for f in sorted(src_dir.glob("*.json")):
        with open(f) as fp:
            data = json.load(fp)
        md = rule_json_to_md(data)
        out_file = out_dir / f"{f.stem}.md"
        out_file.write_text(md)
        count += 1

    print(f"Converted {count} rule JSONs → {out_dir}")


if __name__ == "__main__":
    main()
```

**Step 2: Run the script**

```bash
cd /Users/xiweipan/Codes/problem-reductions
python3 scripts/json_rules_to_issues.py
```

Expected output:
```
Converted 335 rule JSONs → .../references/issues/rules
```

**Step 3: Spot-check 3 output files**

```bash
# One with full theorem proof (no Reference: line)
head -20 references/issues/rules/R01_sat_3sat.md

# One with Reference: line + citation
head -20 references/issues/rules/R098_partition_expectedretrievalcost.md

# One with plain Reference: line (no citation)
head -20 references/issues/rules/R04_vc_is.md
```

Expected for R01: `**Reference:** Garey & Johnson, ..., Theorem 3.1, p.48-50` (no extra ref)
Expected for R098: `**Reference:** Garey & Johnson, ..., Appendix A4.1, p.227; [Cody and Coffman, 1976]. Transformation from PARTITION, 3-PARTITION.`
Expected for R04: `**Reference:** Garey & Johnson, ..., A1.2 GT20; Transformation from VERTEX COVER (see Chapter 3).`

**Step 4: Count output files**

```bash
ls references/issues/rules/*.md | wc -l
```

Expected: `335`
