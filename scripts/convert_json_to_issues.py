#!/usr/bin/env python3
"""Convert Garey & Johnson reference JSON files to issue-like markdown files.

Supports both the new source/augmented JSON format and the legacy flat format.
Sections sourced from `augmented` are marked with an ⚠️ Unverified banner.
"""

import json
import os
import sys
from pathlib import Path

UNVERIFIED_BANNER = (
    '> ⚠️ **Unverified** — AI-generated content, not directly from source text\n'
)


def format_complexity(algo_info):
    """Format complexity information from a single algorithm entry."""
    if isinstance(algo_info, dict) and any(
        k in algo_info for k in ["deterministic", "randomized"]
    ):
        # Multiple algorithm variants
        parts = []
        for variant_name, variant in algo_info.items():
            if isinstance(variant, dict) and "complexity" in variant:
                ref = variant.get("reference", "")
                note = variant.get("note", "")
                line = f"  - **{variant_name.title()}:** {variant['complexity']}"
                if variant.get("authors"):
                    line += f" by {variant['authors']}"
                if variant.get("year"):
                    line += f" ({variant['year']})"
                if variant.get("variable_definitions"):
                    defs = ", ".join(
                        f"{k} = {v}"
                        for k, v in variant["variable_definitions"].items()
                    )
                    line += f", where {defs}"
                if ref:
                    line += f"\n    - Ref: {ref}"
                if note:
                    line += f"\n    - Note: {note}"
                parts.append(line)
        return "\n".join(parts)
    elif isinstance(algo_info, dict) and "complexity" in algo_info:
        line = f"{algo_info['complexity']}"
        if algo_info.get("authors"):
            line += f" by {algo_info['authors']}"
        if algo_info.get("year"):
            line += f" ({algo_info['year']})"
        if algo_info.get("variable_definitions"):
            defs = ", ".join(
                f"{k} = {v}" for k, v in algo_info["variable_definitions"].items()
            )
            line += f", where {defs}"
        ref = algo_info.get("reference", "")
        note = algo_info.get("note", "")
        if ref:
            line += f"\n- **Reference:** {ref}"
        if note:
            line += f"\n- **Note:** {note}"
        return line
    return str(algo_info)


def format_struct_fields(fields):
    """Format struct fields as a markdown table."""
    if not fields:
        return "| Field | Type | Description |\n|-------|------|-------------|"
    lines = ["| Field | Type | Description |", "|-------|------|-------------|"]
    for field_name, field_desc in fields.items():
        # Parse "Type -- description" or "Type — description"
        for sep in [" -- ", " — ", " - "]:
            if sep in field_desc:
                type_part, desc_part = field_desc.split(sep, 1)
                break
        else:
            type_part = field_desc
            desc_part = ""
        lines.append(f"| {field_name} | {type_part.strip()} | {desc_part.strip()} |")
    return "\n".join(lines)


def format_example(example):
    """Format example section from JSON."""
    if not example:
        return ""
    parts = []
    if example.get("description"):
        parts.append(example["description"])
    if example.get("instance"):
        parts.append(
            "\n**Instance:**\n```json\n"
            + json.dumps(example["instance"], indent=2)
            + "\n```"
        )
    if example.get("solution"):
        sol = example["solution"]
        if isinstance(sol, dict):
            parts.append(
                "\n**Solution:**\n```json\n"
                + json.dumps(sol, indent=2)
                + "\n```"
            )
        else:
            parts.append(f"\n**Solution:** {sol}")
    if example.get("explanation"):
        parts.append(f"\n**Explanation:** {example['explanation']}")
    return "\n".join(parts)


# ---------------------------------------------------------------------------
# Accessors: support both new (source/augmented) and legacy (flat) formats
# ---------------------------------------------------------------------------

def _model_source(data):
    """Get the source section of a model JSON (book-verifiable content)."""
    return data.get("source", {})


def _model_augmented(data):
    """Get the augmented section of a model JSON (AI-generated content)."""
    return data.get("augmented", {})


def _model_get(data, source_key=None, aug_key=None, legacy_key=None, default=""):
    """Fetch a field, preferring new format, falling back to legacy."""
    if source_key and source_key in _model_source(data):
        return _model_source(data)[source_key]
    if aug_key and aug_key in _model_augmented(data):
        return _model_augmented(data)[aug_key]
    # Legacy flat format fallback
    if legacy_key and legacy_key in data:
        return data[legacy_key]
    # Try checklist for legacy
    checklist = data.get("add_model_checklist", {})
    if legacy_key and legacy_key in checklist:
        return checklist[legacy_key]
    return default


def model_json_to_md(data):
    """Convert a model JSON to issue-like markdown."""
    is_new_format = "source" in data and "augmented" in data
    src = _model_source(data) if is_new_format else {}
    aug = _model_augmented(data) if is_new_format else {}
    checklist = data.get("add_model_checklist", {})

    # Problem name: from augmented.rust_name (new) or checklist.1_problem_name (legacy)
    problem_name = aug.get("rust_name") or checklist.get("1_problem_name", "")
    book_name = src.get("problem_name") or data.get("problem_name_book", "")
    source_loc = src.get("source_location") or data.get("source_location", "")
    pid = data.get("id", "")
    math_def = src.get("mathematical_definition") or checklist.get("2_mathematical_definition", "")

    # Augmented fields
    config_space = aug.get("configuration_space") or checklist.get("6_configuration_space", "N/A")
    type_params = aug.get("type_parameters") or checklist.get("4_type_parameters", "None")
    struct_fields = aug.get("struct_fields") or checklist.get("5_struct_fields", {})
    algo = aug.get("best_known_algorithm") or checklist.get("9_best_known_exact_algorithm", {})
    strategy = aug.get("solving_strategy") or checklist.get("10_solving_strategy", "")
    design_notes = aug.get("design_notes") or data.get("design_notes", "")
    related_reds = src.get("related_reductions") or data.get("related_reductions_in_book", [])
    size_getters = aug.get("problem_size_getters") or data.get("problem_size_getters", {})
    example = aug.get("example") or data.get("example", {})

    lines = [
        "---",
        "name: Problem",
        "about: Propose a new problem type",
        f'title: "[Model] {problem_name}"',
        "labels: model",
        "assignees: ''",
        "---",
        "",
        "## Motivation",
        "",
        f"{book_name} ({pid}) from Garey & Johnson, {source_loc}. "
        f"A classical NP-complete problem useful for reductions.",
        "",
        "## Definition",
        "",
        f"**Name:** {problem_name}",
        f"**Reference:** Garey & Johnson, *Computers and Intractability*, {source_loc}",
        "",
        math_def,
        "",
        "## Variables",
        "",
    ]

    if is_new_format:
        lines.append(UNVERIFIED_BANNER)

    lines.extend([
        f"- **Count:** {config_space}",
        f"- **Per-variable domain:** {'binary {{0,1}}' if 'binary' in config_space.lower() or 'vec![2' in config_space else config_space}",
        f"- **Meaning:** See configuration space above",
        "",
        "## Schema (data type)",
        "",
    ])

    if is_new_format:
        lines.append(UNVERIFIED_BANNER)

    lines.extend([
        f"**Type name:** {problem_name}",
        f"**Variants:** {type_params}",
        "",
        format_struct_fields(struct_fields),
        "",
        "## Complexity",
        "",
    ])

    if is_new_format:
        lines.append(UNVERIFIED_BANNER)

    lines.append(f"- **Best known exact algorithm:** {format_complexity(algo)}")
    lines.append("")

    # Extra remark — mixed provenance, no banner (contains related_reductions from source)
    lines.append("## Extra Remark")
    lines.append("")
    remarks = []
    if design_notes:
        remarks.append(design_notes)
    if related_reds:
        remarks.append(
            "**Related reductions in book:**\n"
            + "\n".join(f"- {r}" for r in related_reds)
        )
    if strategy:
        remarks.append(f"**Solving strategy:** {strategy}")
    if size_getters:
        getter_lines = ", ".join(
            f"`{k}` = `{v}`" for k, v in size_getters.items()
        )
        remarks.append(f"**Problem size getters:** {getter_lines}")
    lines.append("\n\n".join(remarks) if remarks else "N/A")
    lines.append("")

    # How to solve
    strategy_lower = strategy.lower()
    lines.append("## How to solve")
    lines.append("")
    lines.append(
        f"- [{'x' if 'brute' in strategy_lower else ' '}] It can be solved by (existing) bruteforce."
    )
    lines.append(
        f"- [{'x' if 'ilp' in strategy_lower or 'integer' in strategy_lower else ' '}] It can be solved by reducing to integer programming."
    )
    lines.append(
        f"- [{'x' if 'brute' not in strategy_lower and 'ilp' not in strategy_lower and 'integer' not in strategy_lower else ' '}] Other: {strategy}"
    )
    lines.append("")

    # Example
    lines.append("## Example Instance")
    lines.append("")
    if is_new_format:
        lines.append(UNVERIFIED_BANNER)
    lines.append(format_example(example))
    lines.append("")

    return "\n".join(lines)


def reduction_json_to_md(data):
    """Convert a reduction JSON to issue-like markdown."""
    is_new_format = "source" in data and "augmented" in data
    src = data.get("source", {}) if is_new_format else {}
    aug = data.get("augmented", {}) if is_new_format else {}

    source_problem = aug.get("from_problem_codebase") or data.get("from_problem_codebase", "")
    target_problem = aug.get("to_problem_codebase") or data.get("to_problem_codebase", "")
    # Fall back to book names if codebase names missing
    if not source_problem:
        source_problem = src.get("from_problem") or data.get("from_problem_book", "Unknown")
    if not target_problem:
        target_problem = src.get("to_problem") or data.get("to_problem_book", "Unknown")

    rid = data.get("id", "")
    source_ref = src.get("source_reference") or data.get("source_reference", "")
    reduction_type = src.get("reduction_type") or data.get("reduction_type", "polynomial (Karp)")
    from_book = src.get("from_problem") or data.get("from_problem_book", "")
    to_book = src.get("to_problem") or data.get("to_problem_book", "")

    construction = aug.get("construction") or data.get("construction", {})
    correctness = aug.get("correctness") or data.get("correctness", {})
    overhead = aug.get("overhead") or data.get("overhead", {})
    notes = aug.get("notes") or data.get("notes", [])
    from_exists = aug.get("from_exists_in_codebase", data.get("from_exists_in_codebase", False))
    to_exists = aug.get("to_exists_in_codebase", data.get("to_exists_in_codebase", False))

    motivation = construction.get(
        "summary",
        f"{rid}: {from_book} to {to_book}",
    )

    lines = [
        "---",
        "name: Rule",
        "about: Propose a new reduction rule",
        f'title: "[Rule] {source_problem} to {target_problem}"',
        "labels: rule",
        "assignees: ''",
        "---",
        "",
        f"**Source:** {source_problem}",
        f"**Target:** {target_problem}",
        f"**Motivation:** {motivation}",
        f"**Reference:** Garey & Johnson, *Computers and Intractability*, {source_ref}",
        "",
        "## Reduction Algorithm",
        "",
    ]

    if is_new_format:
        lines.append(UNVERIFIED_BANNER)

    if construction.get("detail"):
        lines.append(construction["detail"])
    elif construction.get("summary"):
        lines.append(construction["summary"])

    if construction.get("components"):
        lines.append("")
        lines.append("**Components:**")
        for comp in construction["components"]:
            lines.append(f"- {comp}")

    lines.append("")
    lines.append("## Size Overhead")
    lines.append("")

    if is_new_format:
        lines.append(UNVERIFIED_BANNER)

    if overhead.get("expressions"):
        lines.append("| Target metric (code name) | Polynomial (using symbols above) |")
        lines.append("|----------------------------|----------------------------------|")
        for metric, formula in overhead["expressions"].items():
            lines.append(f"| {metric} | {formula} |")
    else:
        lines.append("| Target metric (code name) | Polynomial (using symbols above) |")
        lines.append("|----------------------------|----------------------------------|")
        if overhead.get("description"):
            lines.append(f"\n{overhead['description']}")

    # Correctness
    if correctness:
        lines.append("")
        lines.append("## Correctness")
        lines.append("")
        if is_new_format:
            lines.append(UNVERIFIED_BANNER)
        if correctness.get("forward"):
            lines.append(f"**Forward:** {correctness['forward']}")
            lines.append("")
        if correctness.get("backward"):
            lines.append(f"**Backward:** {correctness['backward']}")

    lines.append("")
    lines.append("## Validation Method")
    lines.append("")
    if from_exists and to_exists:
        lines.append("- Closed-loop test: reduce, solve target, extract solution, verify on source")
    elif not from_exists:
        lines.append(f"- Source problem ({source_problem}) does not exist in codebase yet — implement model first")
    elif not to_exists:
        lines.append(f"- Target problem ({target_problem}) does not exist in codebase yet — implement model first")
    lines.append(f"- Reduction type: {reduction_type}")

    lines.append("")
    lines.append("## Example")
    lines.append("")

    if is_new_format:
        lines.append(UNVERIFIED_BANNER)

    # Notes as extra context
    if notes:
        if isinstance(notes, list):
            for note in notes:
                lines.append(f"- {note}")
        else:
            lines.append(str(notes))

    lines.append("")

    return "\n".join(lines)


def main():
    base = Path("/Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson")
    models_dir = base / "models"
    reductions_dir = base / "reductions"

    out_models = base / "issues" / "models"
    out_reductions = base / "issues" / "reductions"
    out_models.mkdir(parents=True, exist_ok=True)
    out_reductions.mkdir(parents=True, exist_ok=True)

    # Convert models
    model_count = 0
    for f in sorted(models_dir.glob("*.json")):
        with open(f) as fp:
            data = json.load(fp)
        md = model_json_to_md(data)
        out_file = out_models / f"{f.stem}.md"
        with open(out_file, "w") as fp:
            fp.write(md)
        model_count += 1

    # Convert reductions
    reduction_count = 0
    for f in sorted(reductions_dir.glob("*.json")):
        with open(f) as fp:
            data = json.load(fp)
        md = reduction_json_to_md(data)
        out_file = out_reductions / f"{f.stem}.md"
        with open(out_file, "w") as fp:
            fp.write(md)
        reduction_count += 1

    print(f"Converted {model_count} models and {reduction_count} reductions")
    print(f"Output: {out_models}")
    print(f"Output: {out_reductions}")


if __name__ == "__main__":
    main()
