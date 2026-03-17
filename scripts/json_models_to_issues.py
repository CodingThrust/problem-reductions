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
    math_def_section = math_def

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
