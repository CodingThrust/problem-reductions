---
name: propose
description: Use when a user wants to propose a new problem model or reduction rule — guides them through brainstorming, clarifies the design, and files a GitHub issue
---

# Propose a New Model or Rule

Interactive brainstorming skill that helps domain experts (who may not know the codebase) design a new problem model or reduction rule, then files well-formed GitHub issues.

**No programming knowledge required.** This skill works entirely in mathematical / domain language.

## Invocation

```
/propose
/propose model
/propose rule
```

<HARD-GATE>
Do NOT write any code, create any files, or invoke implementation skills (add-model, add-rule, issue-to-pr).
The ONLY output of this skill is GitHub issues filed via `gh issue create`.
</HARD-GATE>

## Process

```dot
digraph propose {
    rankdir=TB;
    "Start" [shape=doublecircle];
    "Detect type" [shape=diamond];
    "Brainstorm Model" [shape=box];
    "Topology analysis" [shape=box];
    "Propose rules?" [shape=diamond];
    "Brainstorm Rule(s)" [shape=box];
    "Brainstorm Rule (standalone)" [shape=box];
    "Present draft(s)" [shape=box];
    "Run check-issue on draft" [shape=box];
    "Checks pass?" [shape=diamond];
    "User approves?" [shape=diamond];
    "File issue(s)" [shape=box];
    "Done" [shape=doublecircle];

    "Study conventions" [shape=box];

    "Start" -> "Detect type";
    "Detect type" -> "Study conventions" [label="model or rule"];
    "Detect type" -> "Start" [label="ask user"];
    "Study conventions" -> "Brainstorm Model" [label="model"];
    "Study conventions" -> "Brainstorm Rule (standalone)" [label="rule"];
    "Brainstorm Model" -> "Topology analysis";
    "Topology analysis" -> "Propose rules?";
    "Propose rules?" -> "Brainstorm Rule(s)" [label="yes"];
    "Propose rules?" -> "Present draft(s)" [label="no, later"];
    "Brainstorm Rule(s)" -> "Present draft(s)";
    "Brainstorm Rule (standalone)" -> "Present draft(s)";
    "Present draft(s)" -> "Run check-issue on draft";
    "Run check-issue on draft" -> "Checks pass?";
    "Checks pass?" -> "Present draft(s)" [label="fix issues"];
    "Checks pass?" -> "User approves?" [label="pass"];
    "User approves?" -> "Present draft(s)" [label="revise"];
    "User approves?" -> "File issue(s)" [label="yes"];
    "File issue(s)" -> "Done";
}
```

---

## Step 1: Detect Type

If the user didn't specify, use `AskUserQuestion`:

```
AskUserQuestion:
  question: "What would you like to propose?"
  header: "Type"
  options:
    - label: "New problem (model)"
      description: "Define a new computational problem to add to the reduction graph"
    - label: "New reduction rule"
      description: "Add a reduction between two existing problems"
```

---

## Step 1b: Study Conventions

Right after the user picks model or rule, **study at least one existing case** in the relevant category before asking any brainstorming questions. This grounds the conversation in the project's actual conventions and helps produce higher-quality drafts.

### For Models

1. Ask the user a brief orienting question (free text):
   > "What problem are you thinking of? A name or rough description is enough."

2. Based on the answer, identify the most similar existing problem in the graph. Use `pred list --json` to find candidates, then use `pred show <similar_problem>` to study one in detail:

   ```bash
   pred show <similar_problem> --json
   ```

   Also find and read one closed `[Model]` issue in the same category:

   ```bash
   gh issue list --label model --state closed --limit 20 --json number,title,body | jq '[.[] | select(.title | test("<keyword>"; "i"))] | .[0]'
   ```

   If no keyword match, just read the most recent closed model issue to see the template conventions.

3. **Note internally** (do not dump raw output to the user):
   - What fields / size fields the similar problem has
   - How the issue defines variables, schema, complexity
   - What level of mathematical detail is expected in examples
   - How the "Reduction Rule Crossref" section is structured

   Use these conventions to guide the brainstorming questions and draft formatting in later steps.

### For Rules

1. Ask the user a brief orienting question (free text):
   > "Which two problems are you thinking of connecting? Even a rough idea is fine."

2. Based on the answer, study one existing reduction between similar problems. Use `pred neighbors <source_or_target> --json` to find existing reductions, then pick the most relevant one and examine it:

   ```bash
   pred neighbors <problem> --json
   ```

   Also find and read one closed `[Rule]` issue in a similar domain:

   ```bash
   gh issue list --label rule --state closed --limit 20 --json number,title,body | jq '[.[] | select(.title | test("<keyword>"; "i"))] | .[0]'
   ```

3. **Note internally**:
   - How the reduction algorithm is structured (numbered steps, symbol definitions)
   - How the size overhead table is formatted (field names, formulas)
   - How the example is worked through (source → construction → target → solution)
   - What references and validation methods are used

   Use these conventions to guide the brainstorming questions and draft formatting in later steps.

> **Key:** This step asks the user only one light question (to orient the search), then does silent research. Do not show the user raw JSON or code output — just absorb the conventions and let them shape your subsequent questions.

---

## Step 2: Explore Context

Before asking questions, check what already exists. Try `pred`; if not available, build it first with `make cli`.

```bash
pred list --json 2>/dev/null || (make cli && pred list --json)
```

Also **search for existing GitHub issues** to avoid duplicates and surface related work:

```bash
# Search open rule issues for related reductions
gh issue list --label rule --state open --limit 500 --json number,title

# Search open model issues for related problems
gh issue list --label model --state open --limit 500 --json number,title
```

Filter the results for keywords matching the user's area of interest (e.g., "knapsack", "traveling", "coloring"). When presenting suggestions in Step 3, **note any existing issues** that overlap — e.g., "Note: #138 SubsetSum→Knapsack already filed."

This tells you what problems and reductions are already in the graph — essential for:
- Avoiding duplicate model proposals
- Avoiding duplicate rule proposals (check existing issues!)
- Identifying which problems a new rule could connect to
- Suggesting natural reduction targets

---

## Step 3: Brainstorm (one question at a time)

Ask questions **one at a time**. Prefer multiple-choice when possible. Use mathematical language, not programming language.

### For Models

Work through these topics in order, using `AskUserQuestion` where multiple-choice is natural. Adapt based on answers. (The orienting "What problem?" question was already asked in Step 1b.)

1. **Why useful?** — Use `AskUserQuestion`:
   ```
   AskUserQuestion:
     question: "What's the motivation for this problem? Where does it appear?"
     header: "Motivation"
     options:
       - label: "Combinatorial optimization"
         description: "Scheduling, routing, packing, allocation problems"
       - label: "Physics / simulation"
         description: "Spin systems, ground states, quantum computing"
       - label: "Cryptography / number theory"
         description: "Factoring, lattice problems, code-based crypto"
       - label: "Something else"
         description: "I'll describe the domain"
   ```

2. **Definition** — Use `AskUserQuestion` to clarify problem type, then free text for formal definition. Infer the recommendation from the user's problem description (e.g., "find the largest..." → maximize, "find the smallest..." → minimize, "does there exist..." → satisfaction). Mark the inferred option as "(Recommended)":
   ```
   AskUserQuestion:
     question: "What kind of problem is this?"
     header: "Problem type"
     options:
       - label: "Optimization (maximize)"
         description: "Find a solution that maximizes an objective function"
       - label: "Optimization (minimize)"
         description: "Find a solution that minimizes an objective function"
       - label: "Satisfaction (yes/no)"
         description: "Find any solution that meets all constraints, or decide if one exists"
   ```
   Then ask: "Can you state the problem formally? What's the input, constraints, and objective?"

3. **Variables** — Infer the recommendation from the problem structure (e.g., vertex/edge selection → binary, coloring → k-valued, routing → permutation). Mark the inferred option as "(Recommended)":
   ```
   AskUserQuestion:
     question: "How would you represent a solution? What are the decision variables?"
     header: "Variables"
     options:
       - label: "Binary selection"
         description: "Each variable is 0 or 1 (e.g., include/exclude)"
       - label: "k-valued assignment"
         description: "Each variable takes one of k values (e.g., coloring)"
       - label: "Permutation"
         description: "An ordering of all elements (e.g., tour)"
       - label: "Other domain"
         description: "I'll describe the variable structure"
   ```

4. **Complexity & Reference** — Before asking, use WebSearch to research the best known exact algorithms and canonical references for this problem. Then present 3 candidates via `AskUserQuestion`, each combining the complexity bound with its source:

   ```
   AskUserQuestion:
     question: "What is the best known exact algorithm for this problem?"
     header: "Complexity"
     options:
       - label: "O(<expression>) — <algorithm/author>"
         description: "<paper title, year> — <URL>"
       - label: "O(<expression>) — <algorithm/author>"
         description: "<paper title, year> — <URL>"
       - label: "O(<expression>) — <algorithm/author>"
         description: "<paper title, year> — <URL>"
       - label: "I know a different bound"
         description: "I'll provide the complexity, reference, and link"
   ```

   Requirements:
   - Use concrete numeric exponents (e.g., `1.1996^n`, not `(2-ε)^n`)
   - Every option must include a link to the paper or resource
   - After the user picks one, fetch the BibTeX entry for the chosen reference (from the paper's page, DOI resolver, or Google Scholar) and record it — the BibTeX will be included in the filed issue

5. **Solving strategy** — Before presenting options, analyze the problem to determine the best recommendation based on:
   - If the problem has linear constraints and a linear objective → recommend ILP
   - If the problem has binary variables and quadratic interactions → recommend QUBO
   - If the problem is on a small configuration space or has no natural ILP/QUBO formulation → recommend brute-force
   - If a well-known specialized algorithm exists (e.g., polynomial-time for matching, 2-SAT) → recommend that

   Mark the recommended option with "(Recommended)" in the label:
   ```
   AskUserQuestion:
     question: "How can this problem be solved computationally?"
     header: "Solving strategy"
     options:
       - label: "<option> (Recommended)"
         description: "<why this is the best fit for this problem>"
       - label: "<option>"
         description: "<description>"
       - label: "<option>"
         description: "<description>"
       - label: "Specialized algorithm"
         description: "A problem-specific algorithm exists — I'll describe it"
   ```
   This determines the "How to solve" section in the issue. At least one option is required — check-issue rejects issues without a solver path.

   **Important:** If the user chooses "Reduce to ILP" or "Reduce to QUBO", remind them that this requires a reduction rule issue. Either cross-reference an existing rule issue, or plan to file one as a companion rule (which will be proposed in Step 3b Topology Analysis). The model issue's "How to solve" section must reference the rule issue number.

6. **Example** — Generate 3 candidate examples yourself (varying in size and structure), then present via `AskUserQuestion`:

   ```
   AskUserQuestion:
     question: "Which example instance should we use?"
     header: "Example"
     options:
       - label: "<small instance summary>"
         description: "<brief description — minimal but valid>"
       - label: "<medium instance summary>"
         description: "<brief description — exercises core structure>"
       - label: "<larger instance summary>"
         description: "<brief description — richer, more illustrative>"
   ```

   After the user picks one, provide a complete instance with its known optimal solution.
   - Must exercise the problem's core structure
   - Must be small enough to verify by hand

7. **Data representation** — Infer the recommendation from the problem definition (e.g., "vertices and edges" → graph, "rows and columns" → matrix, "universe and subsets" → set system). Mark the inferred option as "(Recommended)":
   ```
   AskUserQuestion:
     question: "What data defines an instance of this problem?"
     header: "Input data"
     options:
       - label: "A graph"
         description: "Vertices and edges, possibly weighted"
       - label: "A matrix"
         description: "Rows and columns of numbers"
       - label: "A set system"
         description: "A universe of elements and a collection of subsets"
       - label: "Something else"
         description: "I'll describe the input structure"
   ```

8. **Variants** — Based on the data representation answer, ask about applicable variants using `AskUserQuestion`. Only show options that are viable for the problem's input structure. Skip this question entirely if no variants apply (e.g., the problem has a fixed unique input structure like Knapsack, Factoring, SubsetSum).

   **If the input is a graph** (from step 7), ask about graph topology:
   ```
   AskUserQuestion:
     question: "Which graph topologies should this problem support?"
     header: "Graph topology"
     multiSelect: true
     options:
       - label: "General graphs"
         description: "No structural restriction (SimpleGraph) — default, almost always needed"
       - label: "Planar graphs"
         description: "Graphs embeddable in the plane without edge crossings"
       - label: "Bipartite graphs"
         description: "Graphs whose vertices split into two groups with edges only between groups"
       - label: "Unit disk graphs"
         description: "Intersection graphs of unit disks in the plane"
       - label: "Kings subgraph"
         description: "Subgraphs of the king's graph on a grid"
       - label: "Triangular subgraph"
         description: "Subgraphs of the triangular lattice"
   ```
   Only include topology options that are meaningful for the problem (e.g., don't offer "Kings subgraph" for a problem that doesn't have special structure on grids).

   **If the problem can be weighted or unweighted**, ask:
   ```
   AskUserQuestion:
     question: "Should this problem support weighted instances?"
     header: "Weights"
     options:
       - label: "Unweighted only"
         description: "All elements have unit weight — simpler formulation"
       - label: "Weighted (integers)"
         description: "Elements have integer weights"
       - label: "Weighted (real numbers)"
         description: "Elements have real-valued weights"
       - label: "Both weighted and unweighted"
         description: "Support unit weight and integer weight variants"
   ```
   Skip this if the problem inherently requires specific numeric values (e.g., QUBO always has a weight matrix, Knapsack always has item values).

   **If the problem has a parameter K** (e.g., K-coloring, K-satisfiability), ask:
   ```
   AskUserQuestion:
     question: "Should K be a fixed constant or a general parameter?"
     header: "K parameter"
     options:
       - label: "General K"
         description: "K is part of the input — problem is NP-hard for general K"
       - label: "Fixed small K values"
         description: "Define variants for specific K (e.g., K=2, K=3) with different complexities"
       - label: "Both"
         description: "General K plus specific fixed-K variants with known better algorithms"
   ```
   Skip this if the problem has no natural K parameter.

   Record the chosen variants — they will appear in the Schema section of the issue draft (the "Variants" field).

After model brainstorming is complete, proceed to **Step 3b: Topology Analysis**.

### For Rules (standalone)

Work through these topics in order, using `AskUserQuestion` for each step. (The orienting "Which two problems?" question was already asked in Step 1b, and conventions were studied.)

1. **Which problems?** — First run topology analysis (orphans, NP-hardness gaps, `pred list --json`) to identify the most needed rules. Incorporate what the user mentioned in Step 1b, then present suggestions via `AskUserQuestion`:

   ```
   AskUserQuestion:
     question: "Which reduction would you like to propose?"
     header: "Reduction"
     options:
       - label: "<Source> → <Target> (Recommended)"
         description: "<why this is the most valuable — e.g., connects orphan, fills NP-hardness gap>"
       - label: "<Source> → <Target>"
         description: "<why valuable>"
       - label: "<Source> → <Target>"
         description: "<why valuable>"
       - label: "I have a different pair"
         description: "I'll describe the source and target problems"
   ```

   Populate the suggestions based on topology analysis:
   - **Priority 1:** Rules that connect orphan problems (8 orphans: BMF, BicliqueCover, CVP, GraphPartitioning, Knapsack, MaximalIS, MinimumFeedbackVertexSet, PaintShop)
   - **Priority 2:** Rules that fill NP-hardness proof gaps (BinPacking, LongestCommonSubsequence, TravelingSalesman have no inbound path from 3-SAT)
   - **Priority 3:** Rules to large clusters (QUBO, ILP, SAT families)

   After selection, verify both problems exist (or one is being proposed alongside).

2. **Why useful?** — Use `AskUserQuestion`:
   ```
   AskUserQuestion:
     question: "What makes this reduction valuable?"
     header: "Motivation"
     options:
       - label: "Connects an isolated problem"
         description: "Links an orphan problem to the main graph"
       - label: "Enables a new solver"
         description: "Allows solving via the target problem's solvers (e.g., ILP, QUBO)"
       - label: "Proves NP-hardness"
         description: "Establishes hardness via a chain from 3-SAT"
       - label: "Shorter reduction path"
         description: "Provides a more efficient path than what currently exists"
   ```
   Also check if a path already exists: `pred path <source> <target> --json`

3. **Algorithm** — Ask as a free-text question (no multiple choice here):
   > "How does the reduction work? Given a source instance, how do you construct the target instance? Please describe step by step."
   - Must define all symbols before using them
   - Must be detailed enough that someone could implement it
   - If the user is unsure, use WebSearch to find known reductions in the literature

4. **Correctness** — Ask as free text:
   > "Why does this work? Why does an optimal solution to the target correspond to an optimal solution of the source?"

5. **Size overhead** — Ask as free text:
   > "How large is the target instance relative to the source? E.g., if the source has n vertices and m edges, how many variables/constraints does the target have?"

6. **Example** — Generate 3 candidate examples yourself (varying in size and structure), then present via `AskUserQuestion`:

   ```
   AskUserQuestion:
     question: "Which example instance should we use?"
     header: "Example"
     options:
       - label: "<small instance summary>"
         description: "<brief description — e.g., 3 items, capacity 5, optimal: items {1,2}>"
       - label: "<medium instance summary>"
         description: "<brief description — shows a non-obvious optimum>"
       - label: "<larger instance summary>"
         description: "<brief description — richer structure, more trade-offs>"
   ```

   After the user picks one, fully work out the example: show source instance, each construction step, resulting target instance, and the optimal solution.
   - Must be non-trivial but hand-verifiable
   - Must exercise the core structure of the reduction

7. **Reference** — Use `AskUserQuestion`:
   ```
   AskUserQuestion:
     question: "Is there a paper or textbook that describes this reduction?"
     header: "Reference"
     options:
       - label: "Yes, I have a reference"
         description: "I'll provide the citation"
       - label: "No, please help find one"
         description: "Search the literature for a known reduction"
       - label: "This is a novel reduction"
         description: "I designed this myself — no existing reference"
   ```

---

## Step 3b: Topology Analysis (models only)

After the model definition is clear, analyze the reduction graph to suggest which rules would be most valuable. Run:

```bash
# Check orphan problems (to understand graph structure)
cargo run --example detect_isolated_problems 2>/dev/null

# Check NP-hardness proof gaps (to find problems that need connections)
cargo run --example detect_unreachable_from_3sat 2>/dev/null

# List existing problems and reductions
pred list --json

# Check if paths exist between the new problem's likely neighbors
pred path <similar_problem_A> <similar_problem_B> --json
```

Based on the topology analysis, present the user with **suggested reductions** via `AskUserQuestion` (use `multiSelect: true`):

```
AskUserQuestion:
  question: "Which reductions would you like to propose to connect your problem to the graph? (select one or more)"
  header: "Rules"
  multiSelect: true
  options:
    - label: "<Source> → <Target> (Recommended)"
      description: "<why most valuable — e.g., proves NP-hardness, connects to main cluster>"
    - label: "<Source> → <Target>"
      description: "<why valuable>"
    - label: "<Source> → <Target>"
      description: "<why valuable>"
    - label: "I'll file rules separately"
      description: "⚠ WARNING: A model with no reduction rules is an orphan node and WILL be rejected during review"
```

**Ranking criteria** (in order of priority):
- Connections that establish NP-hardness (from a problem reachable from 3-SAT)
- Connections to large clusters (QUBO, ILP, SAT families)
- Connections that reduce orphan count or bridge disconnected components
- Connections the user specifically mentioned during brainstorming

---

## Step 3c: Brainstorm Companion Rules (models only)

If the user picks one or more rules from Step 3b (or proposes their own):

For **each** selected rule, run through the rule brainstorming flow (algorithm, correctness, overhead, example, reference) — but keep it lighter since the model context is already established.

If the user declines ("I'll file rules separately later"):
- **Strongly warn** via `AskUserQuestion`:
  ```
  AskUserQuestion:
    question: "A problem with no reduction rules is an orphan node — it will be isolated in the graph and REJECTED during review. Are you sure you want to skip?"
    header: "⚠ Orphan Warning"
    options:
      - label: "Let me propose a rule now"
        description: "I'll define at least one reduction rule to connect this problem to the graph"
      - label: "Skip anyway — I'll file rule issues separately"
        description: "I understand the risk. I will file companion rule issues before review."
  ```
- If the user chooses "Let me propose a rule now", go back to Step 3b and let them pick a rule, then brainstorm it.
- If the user still declines, include a placeholder in the model's "Reduction Rule Crossref" section noting which rules are planned, and add a visible warning in the draft: "⚠ No companion rule filed — this model will be an orphan node until a rule issue is created."

---

## Step 4: Present Draft Issue(s)

Once all information is collected, compose the full issue body following the GitHub issue template format.

If proposing a model + rules, present all drafts together:

> "Here are the draft issues. Please review — I can revise any section before filing."
>
> **Issue 1: [Model] ProblemName**
> (full draft)
>
> **Issue 2: [Rule] ProblemName to QUBO**
> (full draft)

**For models**, the draft must include all template sections:
- Motivation
- Definition (Name, Reference, formal definition)
- Variables (Count, Per-variable domain, Meaning)
- Schema (Type name, Variants, Field table — use mathematical types, not programming types)
- Complexity (expression + citation + BibTeX)
- Extra Remark (if applicable)
- Reduction Rule Crossref (linking to companion rule issues or noting planned rules)
- How to solve (brute-force, ILP, or other — if ILP/QUBO, must cross-reference rule issue)
- Example Instance
- BibTeX (include the BibTeX entry for the complexity/definition reference at the end of the issue)

**For rules**, the draft must include:
- Source, Target, Motivation, Reference (with BibTeX)
- Reduction Algorithm (numbered steps, all symbols defined)
- Size Overhead (table with target metrics and formulas)
- Validation Method
- Example (fully worked)
- BibTeX (include the BibTeX entry for the reference at the end of the issue)

---

## Step 5: Run Check-Issue on Draft (BEFORE filing)

**Critical: Run the check-issue logic on the draft BEFORE filing.** This catches problems early and avoids filing issues that will fail review.

Apply all 4 checks from `/check-issue` against the draft content:

### Rule draft checks
1. **Usefulness:** `pred path <source> <target>` — verify no existing path. If path exists, run redundancy analysis.
2. **Non-trivial:** Review the algorithm for genuine structural transformation (not just variable substitution or subtype coercion).
3. **Correctness:** Verify references exist (check `check-issue/references.md`, `docs/paper/references.bib`, then WebSearch). Cross-check claims.
4. **Well-written:** Verify all sections present, symbols consistent, overhead table field names match `pred show <target> --json` → `size_fields`, example is fully worked.

### Model draft checks
1. **Usefulness:** `pred show <name>` must fail (problem doesn't exist). At least one reduction planned.
2. **Non-trivial:** Not isomorphic to existing problem.
3. **Correctness:** Complexity expression verified against literature.
4. **Well-written:** All template sections present, symbols consistent, example exercises core structure.

**If any check fails:** Fix the draft automatically if possible. If user input is needed, ask. Loop back to Step 4 with the corrected draft.

**If all checks pass:** Show the user a summary: "Draft passes all 4 quality checks (Usefulness ✅, Non-trivial ✅, Correctness ✅, Well-written ✅). Ready to file."

Then present for approval via `AskUserQuestion`:

```
AskUserQuestion:
  question: "The draft passes all quality checks. Ready to file?"
  header: "Approval"
  options:
    - label: "File it"
      description: "File the GitHub issue as-is"
    - label: "Revise first"
      description: "I have changes to suggest before filing"
```

---

## Step 6: File the Issue(s)

Once the user approves, file all issues. For model + rule bundles, file the model issue first so rule issues can cross-reference it.

```bash
# File model issue first
gh issue create \
  --title "[Model] ProblemName" \
  --label "model" \
  --body "$(cat <<'EOF'
<model issue body>
EOF
)"
```

Capture the model issue number, then file companion rule issues with cross-references:

```bash
gh issue create \
  --title "[Rule] ProblemName to Target" \
  --label "rule" \
  --body "$(cat <<'EOF'
<rule issue body, referencing #model-issue-number>
EOF
)"
```

After filing all rule issues, update the model issue's "Reduction Rule Crossref" section with the actual issue numbers:

```bash
# Update model issue body to replace placeholder with real issue numbers
gh issue edit <model-issue-number> --body "$(cat <<'EOF'
<updated body with real rule issue numbers>
EOF
)"
```

Print all issue URLs when done.

---

## Key Principles

- **Use `AskUserQuestion` for structured choices** — whenever the user needs to pick from options (type detection, problem selection, motivation, variable type, data structure, reference type, approval), use the `AskUserQuestion` tool with well-labeled options. Use free text only for open-ended questions (algorithm description, formal definitions, examples, complexity expressions).
- **One question at a time** — don't overwhelm; each `AskUserQuestion` call has one focused question
- **Mathematical language only** — never mention Rust types, traits, macros, or code patterns to the user
- **Help find references** — use WebSearch to help locate papers, verify claims
- **Always provide a recommendation** — for every `AskUserQuestion` with multiple choices, analyze the problem context and mark one option as "(Recommended)" with a brief reason. Domain experts benefit from an informed default they can override. Base recommendations on the problem description, existing graph topology, and literature conventions.
- **Suggest, don't prescribe** — if the user is unsure about complexity or reductions, propose candidates and let them choose
- **Topology-driven suggestions** — run topology analysis first, then populate `AskUserQuestion` options with the most needed reductions ranked by value
- **Self-check before filing** — catch problems before they reach review
- **No implementation** — this skill produces issues, nothing else

## Common Mistakes

- **Don't ask all questions at once.** One `AskUserQuestion` call per message.
- **Don't use plain text for structured choices.** If the user needs to pick from options, use `AskUserQuestion` — not a bulleted list in plain text.
- **Don't use programming jargon.** Say "list of weights" not "Vec<W>". Say "graph" not "SimpleGraph". Say "integer" not "i32".
- **Don't skip the reduction crossref.** An orphan model will be rejected.
- **Don't file without user approval.** Always show the draft first.
- **Don't implement anything.** The output is issues, not code.
- **Don't skip topology analysis for rules.** Always run topology analysis first, then populate `AskUserQuestion` options with the most needed reductions.
