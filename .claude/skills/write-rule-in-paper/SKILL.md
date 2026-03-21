---
name: write-rule-in-paper
description: Use when writing or improving a reduction-rule entry in the Typst paper (docs/paper/reductions.typ)
---

# Write Reduction Rule in Paper

Full authoring guide for writing a `reduction-rule` entry in `docs/paper/reductions.typ`. Covers Typst mechanics, writing quality, and verification.

> **Note:** This content is also inlined in `add-rule` Step 5 (condensed form). This standalone version has more detail and is useful for improving existing entries.

## Reference Example

**KColoring → QUBO** in `docs/paper/reductions.typ` is the gold-standard reduction example. Search for `reduction-rule("KColoring", "QUBO"` to see the complete entry. Use it as a template for style, depth, and structure.

## Prerequisites

Before using this skill, ensure:
- The reduction is implemented and tested (`src/rules/<source>_<target>.rs`)
- A canonical example exists in `src/example_db/rule_builders.rs`
- If the canonical example changed, fixtures are regenerated (`make regenerate-fixtures`)
- The reduction graph and schemas are up to date (`cargo run --example export_graph && cargo run --example export_schemas`)

## Step 1: Load Example Data

```typst
#let src_tgt = load-example("Source", "Target")
#let src_tgt_sol = src_tgt.solutions.at(0)
```

Where:
- `load-example(source, target, ...)` looks up the canonical rule entry from `src/example_db/fixtures/examples.json`
- The returned record contains `source`, `target`, and `solutions`
- Access fields: `src_tgt.source.instance`, `src_tgt.target.instance`, `src_tgt_sol.source_config`, `src_tgt_sol.target_config`

## Step 2: Write the Theorem Body (Rule Statement)

The theorem body is a concise block with three parts:

### 2a. Complexity with Reference

State the reduction's time complexity with a citation. Examples:

```typst
% With verified reference:
This $O(n + m)$ reduction @Author2023 constructs ...

% Without verified reference — add footnote:
This $O(n^2)$ reduction#footnote[Complexity not independently verified from literature.] constructs ...
```

**Verification**: Identify the best known reference for this reduction's complexity. If you cannot find a peer-reviewed or textbook source, you MUST add the footnote.

### 2b. Construction Summary

One sentence describing what the reduction builds:

```typst
... constructs an intersection graph $G' = (V', E')$ where ...
```

### 2c. Overhead Hint

State target dimensions in terms of source. This complements the auto-derived overhead (which appears automatically from JSON edge data):

```typst
... ($n k$ variables indexed by $v dot k + c$).
```

### Complete theorem body example

```typst
][
  Given $G = (V, E)$ with $k$ colors, construct upper-triangular
  $Q in RR^(n k times n k)$ using one-hot encoding $x_(v,c) in {0,1}$
  ($n k$ variables indexed by $v dot k + c$).
]
```

## Step 3: Write the Proof Body

The proof must be **self-contained** (all notation defined before use) and **reproducible** (enough detail to reimplement the reduction from the proof alone).

### Structure

Use these subsections in order. Use italic labels exactly as shown:

```typst
][
  _Construction._ ...

  _Correctness._ ...

  _Variable mapping._ ...    // only if the reduction has a non-trivial variable mapping

  _Solution extraction._ ...
]
```

### 3a. Construction

Full mathematical construction of the target instance. Define all symbols and notation here.

**For standard reductions** (< 300 LOC): Write the complete construction with enough math to reimplement.

**For heavy reductions** (300+ LOC): Briefly describe the approach and cite a reference:
```typst
_Construction._ The reduction follows the standard Cook–Levin construction @Cook1971,
encoding each gate as a set of clauses. See @Source for full details.
```

### 3b. Correctness

Bidirectional (iff) argument showing solution correspondence. Use ($arrow.r.double$) and ($arrow.l.double$) for each direction:

```typst
_Correctness._ ($arrow.r.double$) If $S$ is independent, then ...
($arrow.l.double$) If $C$ is a vertex cover, then ...
```

### 3c. Variable Mapping (if applicable)

Explicitly state how source variables map to target variables. Include this section when the mapping is non-trivial (encoding, expansion, reindexing). Omit for identity mappings or trivial complement operations.

```typst
_Variable mapping._ Vertices $= {S_1, ..., S_m}$, edges $= {(S_i, S_j) : S_i inter S_j != emptyset}$, $w(v_i) = w(S_i)$.
```

### 3d. Solution Extraction

How to convert a target solution back to a source solution:

```typst
_Solution extraction._ For each vertex $v$, find $c$ with $x_(v,c) = 1$.
```

## Step 4: Write the Worked Example (Extra Block)

Detailed by default. Only use a brief example for trivially obvious reductions (complement, identity).

### 4a. Typst Skeleton

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [Description ($n = ...$, $|E| = ...$)],
  extra: [
    #pred-commands(
      "pred create --example <ALIAS> -o <name>.json",
      "pred reduce <name>.json --to " + target-spec(src_tgt) + " -o bundle.json",
      "pred solve bundle.json",
      "pred evaluate <name>.json --config " + src_tgt_sol.source_config.map(str).join(","),
    )

    // Optional: graph visualization
    #{
      // canvas code for graph rendering
    }

    *Step 1 -- [action].* [description with concrete numbers]

    *Step 2 -- [action].* [construction details]

    // ... more steps as needed

    *Step N -- Verify a solution.* [end-to-end verification]

    *Multiplicity:* The fixture stores one canonical witness. If total multiplicity matters, explain it from the construction.
  ],
)
```

### 4a2. Reproducibility Commands

Add a `pred-commands()` block at the top of the `extra:` content, before any data display or visualization. Commands are constructed dynamically from loaded example data:

```typst
#pred-commands(
  "pred create --example <SOURCE_ALIAS> -o <source>.json",
  "pred reduce <source>.json --to " + target-spec(src_tgt) + " -o bundle.json",
  "pred solve bundle.json",
  "pred evaluate <source>.json --config " + src_tgt_sol.source_config.map(str).join(","),
)
```

Where `<SOURCE_ALIAS>` is the shortest alias for the source problem (e.g., `MVC`, `MIS`, `SAT`). Use the bare alias when the default variant matches; use the full variant path (e.g., `MIS/SimpleGraph/i32`) when a non-default variant is needed. Check `pred list` for available aliases.

The `target-spec()` helper handles empty variant dicts automatically. The `--config` is composed from `src_tgt_sol.source_config`.

### 4b. Step-by-Step Content

Each step should:
1. **Name the action** in bold: `*Step K -- [verb phrase].*`
2. **Show concrete numbers** from the example instance (use Typst expressions to extract from JSON, not hardcoded values)
3. **Explain where overhead comes from** — e.g., "5 vertices x 3 colors = 15 QUBO variables"

### 4c. Required Steps

| Step | Content |
|------|---------|
| First | Show the source instance (dimensions, structure). Include graph visualization if applicable. |
| Middle | Walk through the construction. Show intermediate values. Explicitly quantify overhead. |
| Second-to-last | Verify a concrete solution end-to-end (source config → target config, check validity). |
| Last | State that the fixture stores one canonical witness; if multiplicity matters, justify it mathematically from the construction. |

### 4d. Graph Visualization (if applicable)

```typst
#{
  let fills = src_tgt_sol.source_config.map(c => graph-colors.at(c))
  align(center, canvas(length: 0.8cm, {
    for (u, v) in graph.edges { g-edge(graph.vertices.at(u), graph.vertices.at(v)) }
    for (k, pos) in graph.vertices.enumerate() {
      g-node(pos, name: str(k), fill: fills.at(k), label: str(k))
    }
  }))
}
```

### 4e. Accessing Solution Data

```typst
// Source configuration (e.g., color assignments)
#src_tgt_sol.source_config.map(str).join(", ")

// Target configuration (e.g., binary encoding)
#src_tgt_sol.target_config.map(str).join(", ")

// The canonical witness pair
#src_tgt.solutions.at(0)

// Source instance fields
#src_tgt.source.instance.num_vertices
```

## Step 5: Register Display Name (if new problem)

If this is a new problem not yet in the paper, add to the `display-name` dictionary near the top of `reductions.typ`:

```typst
"ProblemName": [Display Name],
```

## Step 6: Build and Verify

```bash
# Build the paper
make paper
```

### Verification Checklist

- [ ] **Notation self-contained**: every symbol is defined before first use within the proof
- [ ] **Complexity cited**: reference exists, or footnote added for unverified claims
- [ ] **Overhead consistent**: prose dimensions match auto-derived overhead from JSON edge data
- [ ] **Example uses JSON data**: concrete values come from `load-example`/`load-results`, not hardcoded
- [ ] **Solution verified**: at least one solution checked end-to-end in the example
- [ ] **Witness semantics**: text treats `solutions.at(0)` as the canonical witness; any multiplicity claim is derived mathematically, not from fixture length
- [ ] **Pred commands present**: `pred-commands()` block at top of example with create/reduce/solve/evaluate pipeline
- [ ] **Paper compiles**: `make paper` succeeds without errors
- [ ] **Completeness check**: no new warnings about missing edges in the paper

For simpler reductions, see MinimumVertexCover ↔ MaximumIndependentSet as a minimal example.
