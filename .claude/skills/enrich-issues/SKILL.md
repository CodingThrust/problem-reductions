---
name: enrich-issues
description: Fill TBD sections in extracted G&J issue files using web search + literature. Takes a rule range (e.g., R01-R24), checks codebase for existing implementations, enriches missing content, writes to references/issues/, submits each as a GitHub issue (milestone Garey & Johnson), and updates tracking issue #183.
---

# Enrich Issues

Fill TBD sections in `references/issues(fixed)/` using web search and literature research. Writes completed files to `references/issues/` (never modifies source files). Then submits each enriched file as a GitHub issue (linked to milestone **Garey & Johnson**) with strict validation to prevent empty or mismatched issues. Updates GitHub issue #183 with status.

## Input

```
/enrich-issues R01-R24
```

Accepts a range of rule IDs. Parse the range to determine which rule files to process.

---

## Phase 1: Scan & Classify

For each rule file in the given range, classify it into one of these categories. This phase uses NO web search — only local file reads and codebase checks.

### Step 1.1: Build the name mapping

Build a mapping from G&J book names to codebase Rust names. Start with these known mappings:

| Book name | Codebase name | File |
|-----------|---------------|------|
| SATISFIABILITY / SAT | `Satisfiability` | `src/models/formula/sat.rs` |
| 3-SATISFIABILITY / 3SAT | `KSatisfiability` (k=3) | `src/models/formula/ksat.rs` |
| VERTEX COVER / VC | `MinimumVertexCover` | `src/models/graph/minimum_vertex_cover.rs` |
| INDEPENDENT SET / IS | `MaximumIndependentSet` | `src/models/graph/maximum_independent_set.rs` |
| CLIQUE | `MaximumClique` | `src/models/graph/maximum_clique.rs` |
| DOMINATING SET | `MinimumDominatingSet` | `src/models/graph/minimum_dominating_set.rs` |
| MAX CUT | `MaxCut` | `src/models/graph/max_cut.rs` |
| GRAPH K-COLORABILITY | `KColoring` | `src/models/graph/kcoloring.rs` |
| TRAVELING SALESMAN / TSP | `TravelingSalesman` | `src/models/graph/traveling_salesman.rs` |
| SET COVERING / MINIMUM COVER | `MinimumSetCovering` | `src/models/set/minimum_set_covering.rs` |
| SET PACKING | `MaximumSetPacking` | `src/models/set/maximum_set_packing.rs` |
| INTEGER PROGRAMMING | `ILP` | `src/models/algebraic/ilp.rs` |
| QUBO | `QUBO` | `src/models/algebraic/qubo.rs` |
| BIN PACKING | `BinPacking` | `src/models/misc/bin_packing.rs` |
| CIRCUIT SAT | `CircuitSAT` | `src/models/formula/circuit.rs` |
| MAXIMUM MATCHING | `MaximumMatching` | `src/models/graph/maximum_matching.rs` |
| SPIN GLASS | `SpinGlass` | `src/models/graph/spin_glass.rs` |
| FACTORING | `Factoring` | `src/models/misc/factoring.rs` |

For problems not in this table, check `src/models/` by searching for the problem name.

### Step 1.2: Check existing implementations

For each rule `R<id>` in the range:

1. **Read** the rule file from `references/issues(fixed)/rules/R<id>_*.md`
2. **Extract** the Source and Target problem names from the frontmatter
3. **Map** book names to codebase names using the mapping table
4. **Check** if a reduction rule file exists in `src/rules/` matching `<source>_<target>.rs` (all lowercase, no underscores within problem name)
5. **Check** if source and target models exist in `src/models/`

### Step 1.3: Detect specializations

Flag rules where source or target is a known special case. Known specializations:

| Special case | General version | Restriction |
|-------------|----------------|-------------|
| 3-SAT | SAT | Clauses of size exactly 3 |
| Planar 3-SAT | 3-SAT | Underlying bipartite incidence graph is planar |
| Monotone 3SAT | 3-SAT | No clause contains both a variable and its negation |
| NAE 3SAT (Not-All-Equal) | 3-SAT | No clause has all literals true |
| 1-in-3 3SAT | 3-SAT | Exactly one literal true per clause |
| MAX 2-SAT | SAT | Maximize satisfied clauses of size ≤ 2 |
| Simple MAX CUT | MAX CUT | Unweighted variant |
| PLANAR GEOGRAPHY | GENERALIZED GEOGRAPHY | Restricted to planar graphs |
| Exact Cover by 3-Sets (X3C) | Set Covering | Each set has exactly 3 elements, exact cover |
| 3-Partition | Partition | Partition into triples with size constraints |
| 3-Dimensional Matching (3DM) | Set Packing | 3-element sets from three disjoint universes |
| Numerical 3DM | 3DM | With numerical target sums |
| DIRECTED HAMILTONIAN CIRCUIT | HAMILTONIAN CIRCUIT | Directed graph variant |
| DIRECTED HAMILTONIAN PATH | HAMILTONIAN PATH | Directed graph variant |
| GRAPH 3-COLORABILITY | GRAPH K-COLORABILITY | k=3 |

Also look for patterns in the GJ text:
- "Restriction to..." or "special case of..."
- "Contains X as a special case"
- "Remains NP-complete even if..."

### Step 1.4: Classify each rule

Assign one of these statuses:

| Status | Condition | Action |
|--------|-----------|--------|
| `SKIP_IMPLEMENTED` | Rule already exists in `src/rules/` | Write nothing, just log |
| `SKIP_SPECIALIZATION` | Source or target is a specialization that should wait for general version | Write a stub file locally with specialization note — do NOT submit as GitHub issue |
| `SKIP_GENERIC` | Source is "(generic transformation)" (e.g., Cook's theorem) | Write nothing — not a concrete Karp reduction |
| `PROCESS` | Rule not implemented, not a specialization | Enrich and write |

For each `PROCESS` rule, also check dependent models:

| Model status | Condition | Action |
|-------------|-----------|--------|
| `MODEL_EXISTS` | Model exists in `src/models/` | No action needed |
| `MODEL_NEEDED` | Model missing, referenced by a PROCESS rule | Enrich model file too |
| `MODEL_ORPHAN` | Model has no associated rules in the current range | Mark but don't process |

### Step 1.5: Present classification to user

Print a summary table:

```
## Scan Results for R01-R24

| ID | Source → Target | Status | Notes |
|----|----------------|--------|-------|
| R01 | SAT → 3SAT | SKIP_IMPLEMENTED | sat_ksat.rs |
| R04 | VC → IS | SKIP_IMPLEMENTED | minimumvertexcover_maximumindependentset.rs |
| R06 | 3SAT → VC | PROCESS | Source: KSatisfiability ✅, Target: MinimumVertexCover ✅ |
| R13 | CLIQUE → SubgraphIso | PROCESS (+P59) | Target model P59 needs enrichment |
| ...

Models to enrich: P59 (SubgraphIsomorphism)
```

Proceed only after user confirms.

---

## Phase 2: Enrich TBD Sections

### Step 2.1: Dispatch parallel subagents

Batch `PROCESS` rules into groups of 3-5 (plus their dependent models). Launch one subagent per batch using the Agent tool.

Each subagent receives:
- The list of rule files and model files to process
- The source file paths in `references/issues(fixed)/`
- The output paths in `references/issues/`
- The enrichment instructions below

### Step 2.2: Subagent instructions — Rule enrichment

For each rule file:

1. **Read** the source file from `references/issues(fixed)/rules/`
2. **Identify TBD sections** that need filling
3. **Research** using WebSearch:
   - Search `"<source problem> to <target problem> reduction NP-complete"` for algorithm details
   - Search `"<target problem> NP-complete complexity"` for background
   - Search for the widely-recognized name of each problem (may differ from GJ book)
4. **Fill each TBD section** as described below
5. **Write** the completed file to `references/issues/rules/`

#### Frontmatter enrichment

Add these fields to the YAML frontmatter:
```yaml
canonical_source_name: 'Widely Recognized Name'    # if different from GJ
canonical_target_name: 'Widely Recognized Name'     # if different from GJ
source_in_codebase: true/false
target_in_codebase: true/false
specialization_of: 'GeneralProblem'                 # only if applicable
milestone: 'Garey & Johnson'
```

#### Section: Motivation

Replace `(TBD)` with a one-sentence description of why this reduction is useful. Pattern:

```
Establishes NP-completeness of <Target> via polynomial-time reduction from <Source>.
<Additional context from web search about practical applications or theoretical significance.>
```

Mark with: `<!-- ⚠️ Unverified: AI-generated motivation -->`

#### Section: Reduction Algorithm

If the GJ text is already present (as a blockquote), keep it and add a structured summary below:

```markdown
> [Original GJ text preserved as blockquote...]

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a <Source> instance (define symbols), construct a <Target> instance as follows:

1. **Variable mapping:** ...
2. **Constraint transformation:** ...
3. **Solution extraction:** ...
```

If the section is fully `(TBD)`, use web search to find the standard reduction algorithm and write it in the same format. Always cite the source.

#### Section: Size Overhead

Fill the table with concrete overhead expressions. Derive from the reduction algorithm:

```markdown
<!-- ⚠️ Unverified: AI-derived overhead expressions -->

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | n + 2m (n variables, m clauses → n + 2m vertices) |
| `num_edges` | n + 3m (n truth-setting edges + 3m triangle edges) |
```

The code name must match the getter method naming convention: `num_vertices`, `num_edges`, `num_vars`, `num_clauses`, `num_sets`, `num_items`, `matrix_size`, etc.

#### Section: Validation Method

Replace `(TBD)` with:

```markdown
<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source instance, solve target with BruteForce, extract solution, verify on source
- Compare with known results from literature
- Cross-check with ProblemReductions.jl if available
```

#### Section: Example

Design a **non-trivial** example that demonstrates the reduction meaningfully. Requirements:

- **Size:** Large enough to be non-trivial but small enough for hand verification
  - Graph problems: 6-10 vertices, interesting edge structure
  - Set problems: 6-8 elements, non-obvious solution
  - Number problems: 5-8 values with greedy traps
- **Structure:** Must exercise the reduction's key mechanism — not a degenerate case
- **Greedy trap:** Include elements that make naive approaches fail
- **Completeness:** Show both the source instance AND the constructed target instance, with the mapping between them

Format:
```markdown
<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (<Source>):**
<concrete instance description with specific values>

**Constructed target instance (<Target>):**
<show the result of applying the reduction step by step>

**Solution mapping:**
- Target solution: ...
- Extracted source solution: ...
- Verification: ...
```

**Anti-patterns to avoid:**
- Triangle graph (3 vertices) for graph problems — too trivial
- 2-variable SAT instances — too simple
- Partition with only 2 elements — trivially solvable
- Any instance where the answer is immediately obvious

### Step 2.3: Subagent instructions — Model enrichment

For each model file that needs enrichment:

1. **Read** the source file from `references/issues(fixed)/models/`
2. **Collect associated rules** — scan issue #183 and all rule files in `references/issues(fixed)/rules/` to find every rule where this model appears as source or target
3. **Research** using WebSearch:
   - `"<problem name> NP-complete best algorithm complexity"`
   - `"<problem name> computational complexity exact algorithm"`
   - Look for the widely-recognized name
4. **Fill each TBD section** as described below
5. **Write** the completed file to `references/issues/models/`

#### Section: Motivation

Enrich the existing motivation line with a list of associated reduction rules. Format:

```markdown
## Motivation

<Original motivation line preserved>

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **As source:** R06 (→ VERTEX COVER), R25 (→ CLIQUE), R199 (→ NAE 3SAT), ...
- **As target:** R01 (SAT →), R198 (SAT →), ...
```

Collect these by scanning all rule files (not just the current batch range) in `references/issues(fixed)/rules/` and the tracking table in issue #183. This gives downstream skills (`add-model`, `issue-to-pr`) full context about which reductions depend on this model.

#### Section: Name (Rust name)

```markdown
**Name:** <!-- ⚠️ Unverified --> ProblemName
```

Follow codebase naming conventions:
- Optimization: `Maximum*` or `Minimum*` prefix
- Satisfaction: no prefix (e.g., `SubgraphIsomorphism`, `Satisfiability`)
- Use CamelCase, no abbreviations unless universally known (e.g., `TSP` → `TravelingSalesman`)

Also add a `canonical_name` field if the widely-recognized name differs from GJ:
```markdown
**Canonical name:** <!-- ⚠️ Unverified: web search --> Subgraph Isomorphism Problem
```

#### Section: Variables

```markdown
<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V₁| × |V₂| (one variable per possible vertex mapping)
- **Per-variable domain:** binary {0, 1}
- **Meaning:** x_{i,j} = 1 if vertex j in H maps to vertex i in G
```

Infer from the mathematical definition. The configuration space must be enumerable for brute-force solving.

#### Section: Schema

```markdown
<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** SubgraphIsomorphism
**Variants:** graph topology (SimpleGraph)

| Field | Type | Description |
|-------|------|-------------|
| `host_graph` | `SimpleGraph` | The graph G to search in |
| `pattern_graph` | `SimpleGraph` | The graph H to find |
```

#### Section: Complexity

```markdown
<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** O(2^n · poly(n)) by brute force enumeration over all vertex subsets, where n = |V₁|
- **References:** [Author, Year] "Title" — <brief description of result>
```

Use web search to find the actual best known algorithm. Use concrete numeric constants (e.g., `1.1996^n`, not `2^n` if a better bound exists).

#### Section: Specialization

If the problem is a known special case, add:

```markdown
## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** (none / general problem name)
- **Known special cases:** List of problems that are restrictions of this one
- **Restriction:** How this specializes the general problem
```

#### Section: Example Instance

Same requirements as rule examples — non-trivial, hand-verifiable, demonstrates key structure.

```markdown
<!-- ⚠️ Unverified: AI-constructed example -->

<Concrete instance with specific values, solution, and explanation>
```

---

## Phase 3: Submit to GitHub Issues

After enrichment, submit each completed markdown file as a GitHub issue linked to the **Garey & Johnson** milestone. This phase processes files **one at a time, sequentially** — never in bulk or parallel — to prevent content mismatches.

### Step 3.1: Collect files to submit

Build the submission list from Phase 2 outputs:
- All `PROCESS` rule files written to `references/issues/rules/`
- All `MODEL_NEEDED` model files written to `references/issues/models/`

**Do NOT submit** `SKIP_SPECIALIZATION` stub files — they are local-only references.

### Step 3.2: Delegate to a single submission subagent

Launch **one** subagent (via the Agent tool) to handle all issue submissions sequentially. This avoids repeated permission prompts by consolidating all `gh` calls into a single agent session.

The subagent receives:
- The ordered list of files to submit (from Step 3.1)
- Instructions to process each file ONE AT A TIME, sequentially

The subagent performs these steps for each file:

#### 3.2.1: Re-read the file fresh

Read the markdown file from disk using the Read tool immediately before submission. Do NOT rely on cached/in-memory content from Phase 2.

#### 3.2.2: Parse frontmatter and body

Extract from the file:
- `title` — from YAML frontmatter (e.g., `[Rule] 3SAT to VERTEX COVER`)
- `labels` — from YAML frontmatter (e.g., `rule` or `model`)
- `body` — everything after the second `---` line, stripped of leading/trailing whitespace

#### 3.2.3: Pre-submission validation (MANDATORY — do NOT skip)

Run ALL of these checks. If ANY fails, **do NOT create the issue** — log the error and move to the next file.

| Check | Condition | Failure message |
|-------|-----------|-----------------|
| **Body non-empty** | `body` has at least 100 characters | `BLOCKED: body is empty or too short ({len} chars) for {filename}` |
| **Title non-empty** | `title` is present and non-empty | `BLOCKED: no title found in frontmatter for {filename}` |
| **Title-body consistency (rules)** | For `[Rule]` issues: the `**Source:**` and `**Target:**` lines in the body must exist and contain words that appear in the title | `BLOCKED: body source/target does not match title for {filename}` |
| **Title-body consistency (models)** | For `[Model]` issues: the `## Definition` or `## Motivation` section must reference the model name from the title | `BLOCKED: body does not reference model name from title for {filename}` |
| **No residual (TBD)** | Body does not contain the literal string `(TBD)` as the sole content of any section (a `(TBD)` inside a table cell for a non-critical field is OK) | `WARNING: {filename} still has TBD sections` (submit anyway but log warning) |
| **Not a duplicate** | Search open issues: `gh issue list --label {label} --search "in:title {title_keywords}" --json number,title --limit 5`. If an open issue with the same title already exists, skip. | `SKIP: duplicate issue already exists: #{number}` |

#### 3.2.4: Create the issue

Write the body to a temp file and use `--body-file` to avoid shell escaping issues:

```bash
# Write body to temp file (reuse same path each iteration)
cat > /tmp/enrich_issue_body.md <<'BODYEOF'
$BODY
BODYEOF

gh issue create \
  --title "$TITLE" \
  --label "$LABELS" \
  --milestone "Garey & Johnson" \
  --body-file /tmp/enrich_issue_body.md
```

#### 3.2.5: Verify after creation

After `gh issue create` succeeds and returns the issue URL (e.g., `https://github.com/.../issues/NNN`):

1. Extract the issue number `NNN` from the URL
2. Read back the issue: `gh issue view NNN --json title,body`
3. Verify:
   - The returned title matches what was submitted
   - The returned body length is within 10% of the submitted body length (GitHub may normalize whitespace)
4. If verification fails, **immediately close the bad issue** with reason "not planned" and comment "Auto-closed: post-creation verification failed", then log the error

#### 3.2.6: Log the result

Print one line per file:
```
✅ R06_3sat_vc.md -> #NNN (title OK, body 2847 chars)
❌ R13_clique_subiso.md -> BLOCKED: body is empty
⏭️ R01_sat_3sat.md -> SKIP: duplicate #229
```

### Step 3.3: Rate limiting

Wait at least 1 second between issue creations to avoid GitHub API rate limits.

---

## Phase 4: Update GitHub Issue #183

After all enrichment and submission is complete, update issue #183 with a status comment.

### Step 4.1: Build the status table

```markdown
## Enrichment batch: R<start>-R<end>

| ID | Source → Target | Status | GitHub Issue | Output file |
|----|----------------|--------|--------------|-------------|
| R01 | SAT → 3SAT | ⏭️ SKIP (implemented: `sat_ksat.rs`) | — | — |
| R06 | 3SAT → VC | ✅ Enriched & submitted | #NNN | `rules/R06_3sat_vc.md` |
| R13 | CLIQUE → SubgraphIso | ✅ Enriched & submitted (+P59) | #NNN, #MMM | `rules/R13_clique_subiso.md`, `models/P59_subgraph_isomorphism.md` |

### Models enriched & submitted
| ID | Name | GitHub Issue | Reason |
|----|------|--------------|--------|
| P59 | SubgraphIsomorphism | #MMM | Target of R13 |
```

### Step 4.2: Post the comment

```bash
gh issue comment 183 --body "$(cat <<'EOF'
<status table from above>
EOF
)"
```

---

## Provenance Markers

All content filled by this skill is marked with HTML comments for easy identification:

| Marker | Meaning |
|--------|---------|
| `<!-- ⚠️ Unverified: AI-generated motivation -->` | Motivation text written by AI |
| `<!-- ⚠️ Unverified: AI-generated summary below -->` | Reduction algorithm summary |
| `<!-- ⚠️ Unverified: AI-derived overhead expressions -->` | Size overhead table |
| `<!-- ⚠️ Unverified: AI-suggested validation -->` | Validation method |
| `<!-- ⚠️ Unverified: AI-constructed example -->` | Example instance |
| `<!-- ⚠️ Unverified: AI-researched complexity -->` | Complexity results from web search |
| `<!-- ⚠️ Unverified: AI-inferred variable mapping -->` | Variable section |
| `<!-- ⚠️ Unverified: AI-designed schema -->` | Schema section |
| `<!-- ⚠️ Unverified: web search -->` | Any content sourced from web search |
| `<!-- ⚠️ Unverified: AI-identified relationship -->` | Specialization relationships |

Content from the original GJ book text (already present in the source file) receives **no marker** — it is considered verified.

---

## Error Handling

| Situation | Action |
|-----------|--------|
| Rule file not found in `issues(fixed)/rules/` | Skip with warning |
| Web search returns no useful results | Fill with best-effort from GJ text + mark `<!-- ⚠️ Unverified: limited sources -->` |
| Ambiguous problem name mapping | List candidates and ask user |
| Subagent fails | Retry once, then skip with error note |
| Model file already exists in `references/issues/models/` | Overwrite (idempotent) |
| Pre-submission validation fails (empty body, title mismatch) | **Do NOT create the issue.** Log the error, skip to next file |
| Post-creation verification fails (title/body mismatch on GitHub) | Immediately close the bad issue with "not planned" reason, log error |
| `gh issue create` fails (API error) | Log error, skip to next file. Do NOT retry blindly |
| Duplicate issue already exists | Skip with `SKIP: duplicate` message |

---

## Quality Checklist

Before posting the issue #183 comment, verify:

- [ ] Every `PROCESS` rule has a completed file in `references/issues/rules/`
- [ ] Every `MODEL_NEEDED` model has a completed file in `references/issues/models/`
- [ ] No `(TBD)` remains in any output file
- [ ] All filled content has appropriate `⚠️ Unverified` markers
- [ ] Examples are non-trivial (at least 6 vertices/elements, greedy traps present)
- [ ] Problem names use widely-recognized conventions (not just GJ names)
- [ ] Specialization relationships are annotated where applicable
- [ ] Source files in `references/issues(fixed)/` are unmodified
- [ ] Every submitted GitHub issue has a non-empty body (>100 chars)
- [ ] Every submitted GitHub issue's body matches its title (source/target or model name)
- [ ] No duplicate issues were created (checked against existing open issues before submission)
- [ ] All submitted issues are linked to milestone **Garey & Johnson**
- [ ] Post-creation verification passed for every submitted issue (title and body read back correctly)
