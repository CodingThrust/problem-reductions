#!/usr/bin/env python3
"""Migrate model/reduction JSON files to source/augmented provenance structure.

Splits each JSON into:
- `source`: book-verifiable metadata (problem name, page refs, INSTANCE/QUESTION text)
- `augmented`: AI-generated content (Rust design, algorithms, examples, constructions)

The `id` field stays at top level. Scan JSONs are not modified.
"""

import json
import glob
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / "references" / "Garey&Johnson"


def build_complexity_map():
    """Build problem_name -> complexity_class from scan JSONs."""
    mapping = {}
    for f in sorted(glob.glob(str(BASE / "json" / "scan_*.json"))):
        data = json.load(open(f))
        for p in data.get("problems", []):
            name = p.get("problem_name", "").strip().upper()
            cc = p.get("complexity_class", "")
            if name and cc:
                mapping[name] = cc
    return mapping


def migrate_model(data, complexity_map):
    """Restructure a model JSON into source/augmented format."""
    checklist = data.get("add_model_checklist", {})
    book_name = data.get("problem_name_book", "")

    # Look up complexity_class from scan data
    complexity_class = complexity_map.get(book_name.upper().strip(), "")

    source = {}
    source["problem_name"] = book_name
    source["source_location"] = data.get("source_location", "")
    source["mathematical_definition"] = checklist.get("2_mathematical_definition", "")
    if complexity_class:
        source["complexity_class"] = complexity_class
    # related_reductions_in_book -> source
    if data.get("related_reductions_in_book"):
        source["related_reductions"] = data["related_reductions_in_book"]

    augmented = {}
    augmented["rust_name"] = checklist.get("1_problem_name", "")
    augmented["problem_type"] = checklist.get("3_problem_type", "")
    if checklist.get("4_type_parameters"):
        augmented["type_parameters"] = checklist["4_type_parameters"]
    if checklist.get("5_struct_fields"):
        augmented["struct_fields"] = checklist["5_struct_fields"]
    if checklist.get("6_configuration_space"):
        augmented["configuration_space"] = checklist["6_configuration_space"]
    if checklist.get("7_feasibility_check"):
        augmented["feasibility_check"] = checklist["7_feasibility_check"]
    if checklist.get("8_objective_function"):
        augmented["objective_function"] = checklist["8_objective_function"]
    if checklist.get("9_best_known_exact_algorithm"):
        augmented["best_known_algorithm"] = checklist["9_best_known_exact_algorithm"]
    if checklist.get("10_solving_strategy"):
        augmented["solving_strategy"] = checklist["10_solving_strategy"]
    if checklist.get("11_category"):
        augmented["category"] = checklist["11_category"]
    if data.get("problem_size_getters"):
        augmented["problem_size_getters"] = data["problem_size_getters"]
    if data.get("design_notes"):
        augmented["design_notes"] = data["design_notes"]
    if data.get("example"):
        augmented["example"] = data["example"]

    result = {"id": data.get("id", "")}
    result["source"] = source
    result["augmented"] = augmented
    return result


def migrate_reduction(data):
    """Restructure a reduction JSON into source/augmented format."""
    source = {
        "from_problem": data.get("from_problem_book", ""),
        "to_problem": data.get("to_problem_book", ""),
        "source_reference": data.get("source_reference", ""),
        "reduction_type": data.get("reduction_type", ""),
    }

    augmented = {}
    if data.get("from_problem_codebase"):
        augmented["from_problem_codebase"] = data["from_problem_codebase"]
    if data.get("to_problem_codebase"):
        augmented["to_problem_codebase"] = data["to_problem_codebase"]
    augmented["from_exists_in_codebase"] = data.get("from_exists_in_codebase", False)
    augmented["to_exists_in_codebase"] = data.get("to_exists_in_codebase", False)
    if data.get("construction"):
        augmented["construction"] = data["construction"]
    if data.get("correctness"):
        augmented["correctness"] = data["correctness"]
    if data.get("overhead"):
        augmented["overhead"] = data["overhead"]
    if data.get("notes"):
        augmented["notes"] = data["notes"]

    result = {"id": data.get("id", "")}
    result["source"] = source
    result["augmented"] = augmented
    return result


def is_already_migrated(data):
    """Check if JSON already has source/augmented structure."""
    return "source" in data and "augmented" in data


def main():
    complexity_map = build_complexity_map()
    print(f"Loaded {len(complexity_map)} complexity_class entries from scan files")

    models_dir = BASE / "models"
    reductions_dir = BASE / "reductions"

    # Migrate models
    model_count = 0
    model_skip = 0
    for f in sorted(models_dir.glob("*.json")):
        with open(f) as fp:
            data = json.load(fp)
        if is_already_migrated(data):
            model_skip += 1
            continue
        migrated = migrate_model(data, complexity_map)
        with open(f, "w") as fp:
            json.dump(migrated, fp, indent=2, ensure_ascii=False)
            fp.write("\n")
        model_count += 1

    # Migrate reductions
    reduction_count = 0
    reduction_skip = 0
    for f in sorted(reductions_dir.glob("*.json")):
        with open(f) as fp:
            data = json.load(fp)
        if is_already_migrated(data):
            reduction_skip += 1
            continue
        migrated = migrate_reduction(data)
        with open(f, "w") as fp:
            json.dump(migrated, fp, indent=2, ensure_ascii=False)
            fp.write("\n")
        reduction_count += 1

    print(f"Migrated {model_count} models ({model_skip} already migrated)")
    print(f"Migrated {reduction_count} reductions ({reduction_skip} already migrated)")


if __name__ == "__main__":
    main()
