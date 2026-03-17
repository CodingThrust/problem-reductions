---
name: extract-from-book
description: Use when extracting computational complexity problem definitions and reduction rules from a book or paper (PDF/text). Triggers include any mention of 'extract problems from book', 'extract reductions', 'parse textbook', 'import problems from PDF', or requests to systematically pull problem definitions and polynomial-time reductions from academic sources like Garey & Johnson. Also use when the user has a PDF of a complexity theory textbook and wants to populate the codebase with models and reductions from it.
---

# Extract From Book

Step-by-step guide for extracting computational complexity problem definitions and reduction rules from a book or paper, then feeding them into downstream skills (`add-model`, `add-reduction`, etc.).

## Overview

The pipeline has three phases:

1. **Scan** — Build a table of contents: list every problem and every reduction mentioned in the source
2. **Extract** — For each item, do a deep read to fill in the structured checklist required by downstream skills
3. **Execute** — Invoke the appropriate skill (`add-model`, `add-reduction`) for each extracted item

Work chapter-by-chapter or section-by-section to stay within context limits. Always confirm the scan results with the user before moving to extraction.

---

## Phase 1: Scan — Build the Master Inventory (Parallel)

### Step 1.1: Determine chunks

Ask the user for the PDF file path. Read the table of contents or first few pages to determine the book's structure and total page count.

Divide the book into non-overlapping chunks of ~20 pages each. Prefer chapter boundaries when they align. Record each chunk as `{ chunk_id, start_page, end_page, label }` (e.g., `{ "C01", 1, 22, "Ch.1 Introduction" }`).

**OCR quality warning:** Scanned PDFs may contain recognition errors — especially in mathematical notation (e.g., `∈` misread as `E`, subscripts lost, `≤` rendered as `<`). Include this warning in every subagent prompt so each agent flags ambiguous symbols.

### Step 1.2: Dispatch parallel subagents

Launch one `Task` subagent per chunk **in parallel** (use `subagent_type: "general-purpose"`). Each subagent receives:

- The PDF path and its assigned page range
- Instructions to read those pages with `Read(file_path, pages: "start-end")`
- The JSON schema below for output
- Instructions to write its results to `docs/plans/scan_<chunk_id>.json`

**Subagent prompt template:**

```
Read the PDF at <path>, pages <start>-<end>.
Identify every problem definition and every reduction rule in this page range.

For each problem, record:
{
  "id": "P<seq>",
  "problem_name": "<StructName>",
  "source_location": "Chapter X, Section Y, p.Z",
  "problem_type": "Optimization (Maximize|Minimize) | Satisfaction",
  "complexity_class": "NP-complete | NP-hard | ...",
  "brief_description": "<one line>"
}

For each reduction, record:
{
  "id": "R<seq>",
  "from_problem": "<name>",
  "to_problem": "<name>",
  "reduction_type": "polynomial (Karp) | Turing | ...",
  "source_location": "Theorem X, p.Y",
  "brief_description": "<one line>"
}

Write the result as JSON to docs/plans/scan_<chunk_id>.json:
{ "chunk_id": "<id>", "pages": "<range>", "problems": [...], "reductions": [...] }

If you find nothing in this range, write an empty arrays JSON.
Flag any OCR-ambiguous symbols in a top-level "warnings" array.
```

### Step 1.3: Merge and deduplicate

After all subagents complete:

1. Read every `docs/plans/scan_*.json` file
2. Merge all problems into one list, assigning globally unique IDs (`P01`, `P02`, ...)
3. Merge all reductions into one list, assigning globally unique IDs (`R01`, `R02`, ...)
4. Deduplicate by `(problem_name)` for problems and `(from_problem, to_problem)` for reductions — keep the entry with the most specific source location
5. Write the merged inventory to `docs/plans/scan_merged.json`

### Step 1.4: User confirmation

Present both tables (problems and reductions) from the merged inventory to the user. Ask:

1. Are there problems or reductions I missed?
2. Are there items you want to skip?
3. What is the priority order? (If the book has 50+ problems, the user may want to start with a subset.)

Produce a **confirmed work list** before proceeding to Phase 2.

---

## Phase 2: Extract — Deep Structured Extraction

Extraction is split into two sub-phases to separate **book-verifiable content** from **AI-generated content**. Each JSON file uses a `source`/`augmented` top-level split (see JSON format below).

### Step 2a: Book extraction (source fields only)

For each confirmed item, go back to the source text and extract **only fields that can be instantly verified against the book**. Subagent instructions must explicitly state: *"Only write content that appears verbatim or near-verbatim in the source text. Do not infer, search the web, or design Rust structures."*

**Model `source` fields:**

| Field | How to extract |
|-------|----------------|
| `problem_name` | The problem name as it appears in the book (e.g., "HITTING SET") |
| `source_location` | Chapter, section, page (e.g., "Chapter 3, Section 3.2.1, p.64") |
| `mathematical_definition` | Copy the INSTANCE/QUESTION text verbatim |
| `complexity_class` | "NP-complete", "NP-hard", etc. — from the book's classification |
| `related_reductions` | List of reductions mentioning this problem, with theorem/page refs |

**Reduction `source` fields:**

| Field | How to extract |
|-------|----------------|
| `from_problem` | Book name of the source problem |
| `to_problem` | Book name of the target problem |
| `source_reference` | Theorem/Lemma number and page (e.g., "Theorem 3.2, p.50-53") |
| `reduction_type` | "polynomial (Karp)", "Turing", etc. — from the book's description |

### Step 2b: AI augmentation (augmented fields)

In a separate pass, generate the AI-derived fields. These are explicitly marked as unverified in downstream output.

**Model `augmented` fields:**

| Field | How to generate |
|-------|----------------|
| `rust_name` | Map book name to Rust struct convention (`MaximumX`, `MinimumX`) |
| `problem_type` | Maximize / Minimize / Satisfaction — infer from definition |
| `type_parameters` | Infer from definition (graph → `G: Graph`, weights → `W: WeightElement`) |
| `struct_fields` | Infer from what the problem takes as input |
| `configuration_space` | Infer from decision variables (binary vertex selection → `vec![2; n]`) |
| `feasibility_check` | Extract constraints from the definition |
| `objective_function` | Extract the optimization target |
| `best_known_algorithm` | Web search for current best known exact algorithm |
| `solving_strategy` | Infer (BruteForce, ILP, DP, etc.) |
| `category` | Map to: `graph`, `formula`, `set`, `algebraic`, `misc` |
| `problem_size_getters` | Design getter methods for overhead expressions |
| `design_notes` | Free-form design notes |
| `example` | Construct a non-trivial illustrative example (see quality requirements below) |

**Reduction `augmented` fields:**

| Field | How to generate |
|-------|----------------|
| `from_problem_codebase` | Rust struct name of source problem |
| `to_problem_codebase` | Rust struct name of target problem |
| `from_exists_in_codebase` | Check if source model exists in `src/models/` |
| `to_exists_in_codebase` | Check if target model exists in `src/models/` |
| `construction` | AI-written summary/detail/components of the reduction |
| `correctness` | Forward and backward correctness arguments |
| `overhead` | Size expressions for the reduction |
| `notes` | Additional context and references |

### JSON output format

**Model:**
```json
{
  "id": "P01",
  "source": {
    "problem_name": "3-DIMENSIONAL MATCHING (3DM)",
    "source_location": "Chapter 3, Section 3.1.2, p.46-47",
    "mathematical_definition": "INSTANCE: ... QUESTION: ...",
    "complexity_class": "NP-complete",
    "related_reductions": ["R02: 3SAT → 3DM (Theorem 3.2, p.50-53)"]
  },
  "augmented": {
    "rust_name": "ThreeDimensionalMatching",
    "problem_type": "Satisfaction",
    "type_parameters": "None",
    "struct_fields": { ... },
    "configuration_space": "vec![2; triples.len()]",
    "feasibility_check": "...",
    "objective_function": "...",
    "best_known_algorithm": { ... },
    "solving_strategy": "BruteForce",
    "category": "set",
    "problem_size_getters": { ... },
    "design_notes": "...",
    "example": { ... }
  }
}
```

**Reduction:**
```json
{
  "id": "R01",
  "source": {
    "from_problem": "3-SATISFIABILITY (3SAT)",
    "to_problem": "3-DIMENSIONAL MATCHING (3DM)",
    "source_reference": "Theorem 3.2, p.50-53",
    "reduction_type": "polynomial (Karp)"
  },
  "augmented": {
    "from_problem_codebase": "KSatisfiability",
    "to_problem_codebase": "ThreeDimensionalMatching",
    "from_exists_in_codebase": true,
    "to_exists_in_codebase": false,
    "construction": { "summary": "...", "detail": "...", "components": [...] },
    "correctness": { "forward": "...", "backward": "..." },
    "overhead": { "description": "...", "expressions": { ... } },
    "notes": "..."
  }
}
```

### Step 2c: Add illustrative examples

Every extracted problem model **must** include a non-trivial example in `augmented.example`. Trivially small instances (e.g., a 2-variable QUBO or a 3-vertex graph) are not acceptable — they fail to show the problem's structure.

Example JSON structure:

```json
"example": {
  "description": "Human-readable description of the instance",
  "instance": { ... },
  "solution": { ... },
  "explanation": "Why this is a good example — what structure it illustrates"
}
```

**Example quality requirements:**
- **Size:** Large enough to show non-trivial structure (typically 6–10 vertices/elements, 5–8 items, etc.)
- **Difficulty:** Greedy or naive approaches should fail or give suboptimal results — the example should demonstrate *why* the problem is hard
- **Verifiability:** Small enough that the solution can be verified by hand or with a short script
- **Structure:** Highlight the problem's key features — e.g., conflicting constraints, greedy traps, forced choices, decoy solutions

**How to find good examples:**
1. Use web search for classic textbook examples, competition problems, or well-known instances
2. Construct instances where greedy fails (e.g., Knapsack where value/weight ratio ordering ≠ optimal)
3. Include both positive and negative aspects when useful (e.g., a graph that has an HP but not an HC)
4. For graph problems, use 8–10 vertices with enough edges to create interesting structure
5. For set/number problems, use 6–8 elements with non-obvious solutions

**Dispatch strategy:** Batch problems into groups of ~8 and dispatch parallel subagents, each responsible for reading the model files and adding examples with web search support.

### Step 2d: User review

Present all extracted checklists to the user for review. Corrections at this stage are cheap; corrections after code generation are expensive. Ask:

- Does the mathematical definition look correct?
- Are the inferred type parameters and configuration spaces right?
- Any corrections to the reduction constructions?

---

## Phase 2.5: Convert to Issue Markdown

After extraction produces structured JSON files (in `references/<BookName>/models/` and `references/<BookName>/reductions/`), convert them to issue-like markdown files that match the GitHub issue templates.

### Step 2.5.1: Run the converter

Use the script `scripts/convert_json_to_issues.py` to batch-convert all JSON files:

```bash
python3 scripts/convert_json_to_issues.py
```

This produces markdown files in `references/<BookName>/issues/models/` and `references/<BookName>/issues/reductions/`.

### Step 2.5.2: Output format and provenance labels

The converter reads the `source`/`augmented` JSON structure and adds `⚠️ Unverified` banners to sections whose data comes from `augmented`.

**Model issues** follow `.github/ISSUE_TEMPLATE/problem.md`:
- Frontmatter with `title: "[Model] ProblemName"`, `labels: model`
- Sections without banner: Motivation, Definition (from `source`)
- Sections with `⚠️ Unverified` banner: Variables, Schema, Complexity, Example Instance (from `augmented`)
- Extra Remark: mixed provenance (related_reductions from `source`, design_notes/getters from `augmented`)

**Reduction issues** follow `.github/ISSUE_TEMPLATE/rule.md`:
- Frontmatter with `title: "[Rule] Source to Target"`, `labels: rule`
- Header without banner: Source, Target, Reference (from `source`)
- Sections with `⚠️ Unverified` banner: Reduction Algorithm, Size Overhead, Correctness, Example (from `augmented`)

### Step 2.5.3: Field mapping from JSON to issue markdown

**Model JSON → Issue:**

| JSON field | Issue section | Provenance |
|------------|---------------|------------|
| `source.problem_name`, `id`, `source.source_location` | Motivation | `source` |
| `augmented.rust_name` | Definition → Name | `augmented` |
| `source.mathematical_definition` | Definition body | `source` |
| `augmented.configuration_space` | Variables → Count | `augmented` |
| `augmented.type_parameters` | Schema → Variants | `augmented` |
| `augmented.struct_fields` | Schema → field table | `augmented` |
| `augmented.best_known_algorithm` | Complexity | `augmented` |
| `augmented.design_notes`, `source.related_reductions`, `augmented.problem_size_getters` | Extra Remark | mixed |
| `augmented.solving_strategy` | How to solve checkboxes | `augmented` |
| `augmented.example` | Example Instance | `augmented` |

**Reduction JSON → Issue:**

| JSON field | Issue section | Provenance |
|------------|---------------|------------|
| `augmented.from_problem_codebase` | Source | `augmented` |
| `augmented.to_problem_codebase` | Target | `augmented` |
| `augmented.construction.summary` | Motivation | `augmented` |
| `source.source_reference` | Reference | `source` |
| `augmented.construction.detail` | Reduction Algorithm | `augmented` |
| `augmented.construction.components` | Reduction Algorithm → Components | `augmented` |
| `augmented.overhead.expressions` | Size Overhead table | `augmented` |
| `augmented.correctness.forward/backward` | Correctness section | `augmented` |
| `augmented.from_exists_in_codebase`, `augmented.to_exists_in_codebase` | Validation Method | `augmented` |
| `augmented.notes` | Example section | `augmented` |

---

## Phase 3: Execute — Drive Downstream Skills

Once the user confirms the extracted data, invoke the appropriate skills. The issue markdown files from Phase 2.5 can be used directly as input to `issue-to-pr`, `add-model`, or `add-rule` skills, or filed as GitHub issues.

### Step 3.1: Add models

For each confirmed problem, invoke the `add-model` skill with the completed checklist. Process them in dependency order: if problem B's reduction comes FROM problem A, make sure A exists first.

### Step 3.2: Add reductions

For each confirmed reduction, invoke the corresponding reduction skill with the extracted construction, correctness argument, and overhead.

### Step 3.3: Verify the dependency graph

After all items are added, build and display the full reduction graph:

```
3-SAT ──→ MaximumClique ──→ MinimumVertexCover
  │                              │
  └──→ MaximumIndependentSet ────┘
```

Use a Mermaid diagram or similar visualization. Verify:
- Every reduction's source and target problem exist as models
- No dangling references
- The graph is consistent with the book's claims

---

## Reading Large PDFs

The `Read` tool supports PDFs with the `pages` parameter (e.g., `pages: "1-20"`). Maximum 20 pages per request.

**Strategy for large books (200+ pages):**

1. **Read TOC first** — `pages: "1-5"` to get the table of contents and determine structure
2. **Chunk by chapter boundaries** — align chunks to chapter/section breaks when possible (keeps context coherent for the subagent)
3. **20-page chunks** — default chunk size; reduce to 10–15 for dense mathematical content
4. **Overlap by 1 page** at chunk boundaries if a section spans the break (tell the subagent which page is overlap to avoid double-counting)

**For books that won't open with Read** (encrypted, image-only scanned PDFs):
- Ask the user to provide a text export or OCR'd version
- Alternatively, ask the user to convert specific page ranges to text using `pdftotext` or similar

## Chunking Strategy for Large Books

For books with many problems (e.g., Garey & Johnson has 300+), all three phases use parallelism:

1. **Phase 1 — Parallel scan:** Dispatch subagents for all chunks simultaneously. This is fast regardless of book size.
2. **Phase 2 — Batch by chapter:** Extract one chapter at a time (5–10 problems + reductions per batch). Subagents can parallelize within a batch.
3. **Phase 3 — Execute per batch:** Add models and reductions after each chapter's extraction is confirmed.
4. **Cumulative verification** — after each batch, verify the growing dependency graph.

---

## Common Patterns in Textbooks

Different books organize reductions differently. Here are patterns to watch for:

| Book style | How reductions appear |
|------------|----------------------|
| **Garey & Johnson** style | Appendix with problem catalog; reductions scattered in chapters |
| **Sipser** style | Reductions presented as theorem proofs inline |
| **Arora & Barak** style | Reductions organized by complexity class, with formal constructions |
| **Survey paper** style | Reduction chains presented as figures/diagrams |

Adapt the scanning strategy to the book's structure.

---

## Edge Cases and Pitfalls

| Issue | How to handle |
|-------|---------------|
| Problem appears under multiple names | Pick the canonical name; record aliases |
| Reduction is only sketched, not fully constructed | Mark as incomplete; ask user whether to supplement from other sources or skip |
| Book uses non-standard notation | Build a notation mapping table at scan time and apply consistently |
| Problem is a variant of an existing model | Note the relationship; ask user whether to add as separate model or parameterize existing one |
| Complexity result is outdated | Use web search to find the current best; note both the book's claim and the updated result |

---

## Verification Checklist

Before declaring a batch complete:

- [ ] Every problem in the work list has a corresponding model
- [ ] Every problem model has a non-trivial illustrative example (not trivially small)
- [ ] Every reduction in the work list has been implemented
- [ ] `make test clippy` passes
- [ ] The reduction graph has no dangling references
- [ ] User has reviewed and approved the extracted definitions
