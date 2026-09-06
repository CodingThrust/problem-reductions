#!/usr/bin/env python3
"""Overlay the product website on mdBook, using its generated registry exports."""

import argparse
import json
from pathlib import Path
import shutil
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[1]


def build(output, graph_path, schemas_path):
    graph = json.loads(graph_path.read_text())
    schemas = json.loads(schemas_path.read_text())
    nodes, edges = graph["nodes"], graph["edges"]
    if not nodes or any(
        not 0 <= edge[end] < len(nodes)
        for edge in edges
        for end in ("source", "target")
    ):
        raise ValueError("The atlas requires nodes and valid edge endpoints")
    source = ROOT / "docs/website"
    output.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source / "assets", output / "assets", dirs_exist_ok=True)
    # Preserve the original artwork, adapting only its colors for the dark website.
    logo = ET.parse(ROOT / "docs/logo.svg")
    palette = {
        "#5c6bc0": "#c4dda6",  # Sage lettering, nodes, and outline.
        "#ced3ec": "#243a28",  # Forest polygon fill.
        "#7d89cd": "#89a673",  # Muted green graph edges.
        "#4a569a": "#e8ede5",  # Ivory numeral.
        "#ffffff": "#101713",  # Dark separation around nodes.
    }
    for element in logo.iter():
        for attribute in ("fill", "stroke"):
            color = element.get(attribute)
            if color in palette:
                element.set(attribute, palette[color])
    ET.register_namespace("", "http://www.w3.org/2000/svg")
    logo.write(output / "assets/logo.svg", encoding="unicode")
    # Reuse the original logo's polygon as the small browser icon.
    logo.getroot().set("viewBox", "0 0 107 107")
    logo.getroot().set("width", "107")
    logo.getroot().set("height", "107")
    logo.write(output / "assets/favicon.svg", encoding="unicode")
    # Keep mdBook's original introduction reachable from its sidebar and search.
    # mdBook already emits introduction.html alongside its index.html alias.
    html = (source / "index.html").read_text()
    counts = {
        "PROBLEM_COUNT": len({node["name"] for node in nodes}),
        "VARIANT_COUNT": len(nodes),
        "RULE_COUNT": len(edges),
    }
    for key, value in counts.items():
        html = html.replace(f"__{key}__", str(value))
    (output / "index.html").write_text(html)
    # Some cast rules are covered by shared suites rather than a dedicated file.
    # Only publish direct test links when the file actually exists.
    site_edges = []
    for edge in edges:
        module_path = edge["doc_path"].removesuffix("/index.html")
        test_path = f"src/unit_tests/{module_path}.rs"
        source_path = (
            "src/models/decision.rs" if module_path.startswith("models/")
            else f"src/{module_path}.rs"
        )
        if not (ROOT / source_path).is_file():
            raise ValueError(f"Missing reduction implementation: {source_path}")
        # Rule modules are private; rustdoc publishes the shared public contracts.
        contract = (
            "rules/enum.ReductionMode.html" if edge.get("turing") else
            "rules/trait.ReduceTo.html" if edge.get("witness") else
            "rules/trait.ReduceToAggregate.html"
        )
        site_edges.append({
            **edge,
            "source_path": source_path,
            "api_path": contract,
            "test_path": test_path if (ROOT / test_path).is_file() else None,
        })
    # Decision<Inner> is one generic Rust type, not a struct for every catalog name.
    site_nodes = [{
        **node,
        "api_path": (
            "models/decision/struct.Decision.html"
            if node["name"].startswith("Decision") else node["doc_path"]
        ),
    } for node in nodes]
    payload = json.dumps({**graph, "nodes": site_nodes,
                         "edges": site_edges, "schemas": schemas},
                         ensure_ascii=True)
    (output / "assets/atlas-data.js").write_text(f"window.REDUCTIONS = {payload};\n")
    print(f"Built website in {output}: {counts['PROBLEM_COUNT']} families, "
          f"{len(nodes)} variants, {len(edges)} directed reductions")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "book")
    parser.add_argument("--graph", type=Path,
                        default=ROOT / "docs/src/reductions/reduction_graph.json")
    parser.add_argument("--schemas", type=Path,
                        default=ROOT / "docs/src/reductions/problem_schemas.json")
    args = parser.parse_args()
    try:
        build(args.output, args.graph, args.schemas)
    except FileNotFoundError as error:
        parser.exit(1, f"{error}\nGenerate atlas data with cargo run --example "
                    "export_graph and cargo run --example export_schemas first.\n")
