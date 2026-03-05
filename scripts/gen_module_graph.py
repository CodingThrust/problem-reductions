#!/usr/bin/env python3
"""Generate module-graph.json for the mdbook interactive architecture page.

Reads the rustdoc JSON output and produces a Cytoscape-ready graph with:
- Module nodes (compound, color-coded by category)
- Public item nodes (children of modules)
- Dependency edges between modules

Usage:
    cargo +nightly rustdoc -- -Z unstable-options --output-format json
    python3 scripts/gen_module_graph.py
"""
import json
import sys
from pathlib import Path

RUSTDOC_JSON = Path("target/doc/problemreductions.json")
OUTPUT = Path("docs/src/static/module-graph.json")
CRATE_NAME = "problemreductions"

# Category assignment by full module path
CATEGORIES = {
    "traits": "core",
    "types": "core",
    "variant": "core",
    "topology": "core",
    "models/graph": "model_graph",
    "models/formula": "model_formula",
    "models/set": "model_set",
    "models/algebraic": "model_algebraic",
    "models/misc": "model_misc",
    "rules": "rule",
    "registry": "registry",
    "solvers": "solver",
    "io": "utility",
    "export": "utility",
    "config": "utility",
    "error": "utility",
}

# Only include these top-level and second-level modules in the graph
INCLUDED_MODULES = {
    "traits",
    "types",
    "variant",
    "topology",
    "models/graph",
    "models/formula",
    "models/set",
    "models/algebraic",
    "models/misc",
    "rules",
    "registry",
    "solvers",
    "io",
    "export",
}

# Item kinds to include in the child list
INCLUDE_KINDS = {"struct", "enum", "trait", "function", "type_alias", "constant"}


def main():
    if not RUSTDOC_JSON.exists():
        print(
            "Error: {} not found. Run:\n"
            "  cargo +nightly rustdoc -- -Z unstable-options --output-format json".format(
                RUSTDOC_JSON
            ),
            file=sys.stderr,
        )
        sys.exit(1)

    with open(RUSTDOC_JSON) as f:
        data = json.load(f)

    index = data["index"]

    # 1. Build parent map to reconstruct full module paths
    parent_map = {}  # child_module_id → (parent_module_id, parent_name)
    for item_id, item in index.items():
        inner = item.get("inner", {})
        if not isinstance(inner, dict) or "module" not in inner:
            continue
        for child_id in inner["module"].get("items", []):
            cid = str(child_id)
            if cid in index:
                child_inner = index[cid].get("inner", {})
                if isinstance(child_inner, dict) and "module" in child_inner:
                    parent_map[cid] = (item_id, item.get("name"))

    def get_full_path(item_id):
        parts = [index[item_id].get("name")]
        current = item_id
        while current in parent_map:
            current = parent_map[current][0]
            parts.append(index[current].get("name"))
        parts.reverse()
        return "/".join(p for p in parts if p != CRATE_NAME)

    def resolve_item(item_id):
        """Follow re-exports (use) to get the actual item."""
        cid = str(item_id)
        seen = set()
        while cid in index and cid not in seen:
            seen.add(cid)
            item = index[cid]
            inner = item.get("inner", {})
            if isinstance(inner, dict) and "use" in inner:
                target_id = str(inner["use"].get("id", ""))
                if target_id and target_id in index:
                    cid = target_id
                else:
                    return cid
            else:
                return cid
        return cid

    # 2. Find included modules and their public items
    item_to_module = {}  # item_id → module full path
    modules = {}
    for item_id, item in index.items():
        inner = item.get("inner", {})
        if not isinstance(inner, dict) or "module" not in inner:
            continue
        name = item.get("name")
        if name == CRATE_NAME:
            continue
        full_path = get_full_path(item_id)
        if full_path not in INCLUDED_MODULES:
            continue

        children = []
        for child_id in inner["module"].get("items", []):
            cid = str(child_id)
            # Register the direct child for dependency tracking
            item_to_module[cid] = full_path

            # Resolve through re-exports to find the actual item
            resolved_id = resolve_item(cid)
            item_to_module[resolved_id] = full_path

            if resolved_id not in index:
                continue
            child = index[resolved_id]
            child_inner = child.get("inner", {})
            if not isinstance(child_inner, dict):
                continue
            kind = list(child_inner.keys())[0] if child_inner else "unknown"
            if kind not in INCLUDE_KINDS:
                continue
            doc = child.get("docs", "") or ""
            doc_summary = doc.strip().split("\n")[0][:120] if doc.strip() else ""
            child_name = child.get("name")
            if child_name:
                children.append(
                    {"name": child_name, "kind": kind, "doc": doc_summary}
                )

        doc_path = full_path.replace("/", "/") + "/index.html"
        modules[full_path] = {
            "name": full_path,
            "category": CATEGORIES.get(full_path, "utility"),
            "doc_path": doc_path,
            "items": children,
        }

    # 3. Extract inter-module dependencies by scanning type references
    def find_ids(obj, found=None):
        if found is None:
            found = set()
        if isinstance(obj, dict):
            for k, v in obj.items():
                if k == "id" and isinstance(v, (int, str)):
                    found.add(str(v))
                else:
                    find_ids(v, found)
        elif isinstance(obj, list):
            for v in obj:
                find_ids(v, found)
        return found

    edges = set()
    for item_id, item in index.items():
        src = item_to_module.get(item_id)
        if not src:
            continue
        inner = item.get("inner", {})
        if not isinstance(inner, dict):
            continue
        for ref_id in find_ids(inner):
            dst = item_to_module.get(ref_id)
            if dst and dst != src:
                edges.add((src, dst))

    # 4. Add well-known architectural edges that rustdoc heuristics may miss
    #    (re-exports at crate root obscure the actual module→module dependencies)
    KNOWN_EDGES = [
        # All model categories depend on core
        ("models/graph", "traits"),
        ("models/graph", "types"),
        ("models/graph", "topology"),
        ("models/graph", "variant"),
        ("models/formula", "traits"),
        ("models/formula", "types"),
        ("models/set", "traits"),
        ("models/set", "types"),
        ("models/algebraic", "traits"),
        ("models/algebraic", "types"),
        ("models/misc", "traits"),
        ("models/misc", "types"),
        # Rules depend on models and core
        ("rules", "traits"),
        ("rules", "types"),
        # Registry depends on rules and core
        ("registry", "rules"),
        ("registry", "traits"),
        ("registry", "types"),
        # Solvers depend on core
        ("solvers", "traits"),
        ("solvers", "types"),
        # IO depends on core
        ("io", "traits"),
        # Topology uses variant system
        ("topology", "variant"),
    ]
    for src, dst in KNOWN_EDGES:
        if src in modules and dst in modules:
            edges.add((src, dst))

    # 5. Write output
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    result = {
        "modules": sorted(modules.values(), key=lambda m: m["name"]),
        "edges": [{"source": s, "target": t} for s, t in sorted(edges)],
    }
    with open(OUTPUT, "w") as f:
        json.dump(result, f, indent=2)
    print(
        "Wrote {} ({} modules, {} edges)".format(
            OUTPUT, len(result["modules"]), len(result["edges"])
        )
    )


if __name__ == "__main__":
    main()
