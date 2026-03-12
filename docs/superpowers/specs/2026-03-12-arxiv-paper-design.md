# Skill-Based Agentic Coding for Mathematical Software: A Case Study in NP-Hard Problem Reductions

**Type:** Full research paper (~10-12 pages)
**Venue:** ICSE/ASE-class SE conference
**Output:** `docs/paper/arxiv/paper.typ` (Typst)

## Thesis

The bottleneck in agentic coding is not agent capability but task decomposition and the division of labor between human creativity and agent management/execution. We demonstrate a skill-based pipeline where humans (contributors + maintainer) provide judgment — which problems matter, which reductions are useful — while agents handle both management (orchestrating the pipeline, picking cards, dispatching sub-agents) and execution (implementation, testing, documentation, review). Applied to NP-hard problem reductions, this produces a verified library of 24 problem types with 40 implemented reduction rules and 52 total graph edges (including 12 inferred variant edges), with multi-layered correctness guarantees.

**Terminology note:** "40 reductions" = hand-coded `ReduceTo` implementations. "52 graph edges" = total directed edges in the reduction graph, including natural edges inferred from the type-parameter subtype lattice (e.g., `MIS<KingsSubgraph>` → `MIS<SimpleGraph>`). The paper must consistently distinguish these counts.

## Paper Outline

### S1. Introduction (~1.5 pages)

Frame the problem:
- AI coding agents achieve 70-80% on isolated bug fixes (SWE-Bench Verified) but drop to ~20% on long-horizon, multi-file tasks. The common response is to push for more agent autonomy.
- We argue the bottleneck is not capability but decomposition: how to split creative/judgment work (human) from management/mechanical work (agent).
- The "review is harder than generation" challenge — especially for mathematical/scientific code where correctness is hard to verify.

Present the three roles:
- **Contributors** create issues (creative: identify which reductions are useful, propose new problems, spot gaps in the graph).
- **Maintainer** curates the project board and writes skills (creative: priorities, domain knowledge encoding, quality standards).
- **Agents** both manage (pick cards from the board, orchestrate the pipeline, dispatch sub-agents for review) and execute (implement, test, document).

Contributions:
1. A skill-based methodology for decomposing mathematical coding tasks into agent-manageable steps.
2. A multi-layered verification stack that catches errors across different abstraction levels.
3. A verified reduction library (24 problem types, 40 implemented reductions, 52 graph edges) as a practical artifact.

### S2. Why Reductions? The Goldilocks Domain (~1 page)

Why this domain is ideal for studying agentic coding:
- Each reduction is self-contained (~50-200 LOC), requires non-trivial mathematical reasoning, yet has an automatable correctness criterion (round-trip: reduce → solve target → extract solution back → verify against source).
- Homogeneous task structure enables systematic comparison across tasks (unlike SWE-Bench's heterogeneous issues).
- Contrast with general SE tasks: reductions have a clear mathematical spec, a ground-truth, and bounded scope.

Practical motivation — hardware solvers:
- Rydberg atom arrays solve Maximum Independent Set natively.
- D-Wave quantum annealers solve Ising/QUBO problems.
- A verified reduction graph serves as a **compilation layer**: reduce SAT → MIS → run on Rydberg atoms; reduce MaxCut → SpinGlass → QUBO → run on D-Wave. The library lets specialized hardware solve a much larger class of problems.

Practical motivation — real-world applications:
- Software-defined networking (routing/scheduling → ILP).
- Airline crew scheduling (→ SetCovering).
- VLSI design (→ graph coloring).
- Logistics (→ TSP, BinPacking).
- These domains reduce to problems that already have hardware or algorithmic solutions; the library provides the verified bridge.

Figure 1: The reduction graph (24 problem types, 42 variant nodes, 52 directed edges, QUBO/ILP hubs visible, color-coded by category: graph/formula/set/algebraic/misc). Caption distinguishes 40 implemented reductions from 12 inferred variant edges.

### S3. System Architecture (~1.5 pages)

The Rust library design that makes agent-generated code verifiable by construction. Focus on the aspects that directly enable the verification story (details of trait hierarchy and proc macros in supplementary material).

**Key design choices:**
- `Problem` trait with `evaluate()` enables brute-force verification of any configuration.
- `ReduceTo<T>` trait with `ReductionResult` enforces that every reduction can produce a target problem AND extract solutions back — the type system makes round-trip testing possible by construction.
- `#[reduction(overhead = {...})]` proc macro: overhead expressions are compile-time validated against getter methods — agents cannot write incorrect variable names in overhead formulas.
- `declare_variants!` registers problem variants with complexity strings — the registry enables automated graph export and completeness checking.

**Design philosophy:** Reduce the space of possible agent errors through type-level enforcement. The architecture is not just a code organization choice — it is the foundation of the verification stack (elaborated in S5).

Figure 2: System architecture diagram (key traits + compile-time validation flow). Full trait hierarchy in supplementary material.

### S4. Skill-Based Task Decomposition (~2 pages)

#### 4.1 The Three Roles

How creative/judgment work distributes across human roles, with management and execution delegated to agents:

| Role | Responsibility | Creative/Judgment | Examples |
|------|---------------|-------------------|----------|
| Contributor | Open issues | Which reductions are useful? Non-trivial? | "Add SAT → DominatingSet rule" |
| Maintainer | Curate board, write skills | Priorities, quality standards, domain knowledge | Move card to "Ready", evolve check-issue skill |
| Agent | Manage pipeline + execute | — | Pick card, implement, test, review, create PR |

#### 4.2 Skills as Agent Functions

A skill is a markdown script that decomposes a complex task into agent-manageable subtasks. Key insight: if a task is small and explicit enough, agents handle it well.

Skills inventory (13 skills, grouped by function):

**Orchestration skills** (agent-as-manager):
- **project-pipeline**: The primary card-based automation skill. Picks a "Ready" issue from the GitHub Project board, moves it to "In Progress", runs `issue-to-pr --execute` in an isolated git worktree, then moves to "review-agentic". Supports single-issue, specific-issue, and `--all` batch modes. Processes Models before Rules to satisfy dependencies.
- **review-pipeline**: Second-stage orchestration. Picks a PR from the "review-agentic" column, fixes Copilot review comments, runs agentic feature tests, fixes CI (up to 3 retries), then moves to "In Review" for human merge. Also supports batch mode.
- **issue-to-pr**: The per-issue entry point invoked by `project-pipeline`. Receives a GitHub issue, classifies it (model vs. rule), dispatches to the appropriate implementation skill, and creates a PR.
- **meta-power**: Batch mode alternative. Resolves all open issues autonomously in dependency order. Experimental — being superseded by the pipeline skills above.

**Implementation skills** (agent-as-executor):
- **add-model**: Brainstorm (if interactive) → implement Problem trait → unit tests → serialization tests → review.
- **add-rule**: Brainstorm (if interactive) → implement ReduceTo trait → closed-loop tests → overhead expressions → example → review.

**Quality gate skills:**
- **check-issue**: Validates usefulness, non-triviality, literature correctness of a proposed rule/model. Posts structured report.
- **check-rule-redundancy**: Determines if a proposed rule is dominated by a composite path through existing rules.
- **review-implementation**: Dispatches parallel subagents (structural check + quality check) with fresh context windows.
- **fix-pr**: Resolves review comments, CI failures, coverage gaps.

**Documentation skills** (also serve as verification Layer 7 — see S5):
- **write-model-in-paper**: Generates Typst problem definition (formal definition, background, example with visualization).
- **write-rule-in-paper**: Generates Typst reduction theorem (complexity citation, self-contained proof sketch, detailed example). The proof sketch is the final verification layer — it forces a human-readable argument for correctness.

**Release skill:**
- **release**: Determines version bump from diff, verifies tests/clippy, tags and publishes.

Table 1: Skills inventory — trigger condition, inputs, outputs, typical agent turns, first-attempt success rate from git history.

#### 4.3 Card-Based Orchestration

- GitHub Project board with columns: Backlog → Ready → In Progress → review-agentic → In Review → Done.
- **Two-stage agent pipeline:**
  - Stage 1 (`project-pipeline`): picks Ready card → moves to In Progress → runs issue-to-pr in isolated worktree → moves to review-agentic.
  - Stage 2 (`review-pipeline`): picks review-agentic card → fixes Copilot comments → runs agentic feature tests → fixes CI (up to 3 retries) → moves to In Review.
- **Human touches only two transitions:**
  - Backlog → Ready (maintainer decides what to work on next — the creative/strategic decision).
  - In Review → Done (maintainer merges after final review — the quality gate).
- The agent handles everything in between: worktree creation, implementation, testing, review, CI fixing, board status updates.
- Batch mode (`--all`) processes all Ready issues or all review-agentic PRs in a single invocation, with Models before Rules to satisfy dependencies.

Figure 3: Pipeline diagram — two-stage card flow: contributor opens issue → [Backlog] → maintainer moves to [Ready] → agent: project-pipeline [In Progress → review-agentic] → agent: review-pipeline [In Review] → maintainer merges [Done]. Human decisions highlighted in distinct color.

### S5. Multi-Layered Verification (~1.5 pages)

#### 5.1 The Verification Stack

Seven layers, each catching different error classes:

| Layer | Mechanism | Catches |
|-------|-----------|---------|
| 1. Type system | Rust compiler, trait bounds | Wrong return types, missing trait impls, API misuse |
| 2. Unit tests | `test_*_basic`, `test_*_serialization` | Evaluation errors, serialization roundtrip failures |
| 3. Closed-loop tests | `test_*_to_*_closed_loop` | Incorrect reduction mapping, wrong solution extraction |
| 4. Overhead validation | Symbolic expr vs. actual sizes | Overhead formula errors (e.g., quadratic vs linear edge count) |
| 5. Materialized fixtures | JSON ground truth in `tests/data/` | Agents silently changing expected values to make tests pass |
| 6. Agentic review | Parallel subagents with fresh context | Structural issues, missing edge cases, convention violations |
| 7. Documentation | Paper entry with proof sketch | Logical errors in the reduction argument itself |

#### 5.2 Why Layers?

The "lazy agent" problem: agents take the shortest path to close an issue. Given a failing test, an agent is more likely to change the expected value than fix the underlying bug. Materialized test data (Layer 5) prevents this by locking expected outputs in version-controlled JSON files that the agent cannot modify as part of a rule implementation PR.

No single layer is sufficient: the type system catches API misuse but not logical errors; closed-loop tests verify functional correctness but not overhead formulas; documentation catches proof-level mistakes that no automated test can detect.

Table 2 (defined in S6.2, referenced here): Error taxonomy × verification layer matrix.

Figure 4: Verification pyramid with concrete error examples at each layer.

### S6. Evaluation (~2.5 pages)

#### 6.1 Ablation: Skill-Based vs. No-Skill Agent (quantitative)

To demonstrate that the skill-based approach matters (not just "use a good agent"), we run a controlled comparison:

**Setup:** Select 5-10 reductions of varying complexity. For each, run two configurations:
- **Skill-based:** Full pipeline (issue-to-pr skill, add-rule skill, review-implementation, fix-pr).
- **No-skill baseline:** Raw Claude Code on the same codebase with the same issue description but no skills (only CLAUDE.md for project context).

**Metrics:** First-attempt CI pass rate, number of review rounds, final correctness (round-trip test pass), lines of code quality (convention adherence).

**Framing:** With n=5-10, this ablation is a **controlled illustration** of the skill-based approach's value, not a statistically powered experiment. The results demonstrate the mechanism (how skills prevent specific error classes) rather than establishing effect sizes. The git mining in S6.2 provides broader quantitative evidence across the full project history.

This is feasible: create the same issues on a branch without skill files, run the agent, measure outcomes.

#### 6.2 Git History Mining (quantitative)

Data source: full git/PR history of the problemreductions repository.

Metrics:
- Agent-implemented vs. human-implemented reductions (count and %).
- First-attempt success rate per skill invocation (does the PR pass CI on first push?).
- Number of review rounds before merge.
- Error taxonomy: categorize all errors found during review, map to verification layer that caught them.
- Test coverage across the codebase (>95% target).
- Lines of code per reduction (distribution, compare agent vs human).

**Addressing the confound:** Skills evolved during the project, so early reductions had less agent support. We address this by:
- Stratifying results by skill maturity phase (Phase 1: manual, Phase 2: basic skills, Phase 3: full pipeline with card automation).
- Plotting success rate over time with skill milestone annotations.
- Restricting primary quantitative claims to Phase 3 reductions (stable pipeline).

**Preliminary error taxonomy** (to be populated from git history):
- *Type errors*: wrong return type, missing trait impl → caught by Layer 1 (type system)
- *Mapping errors*: incorrect vertex/edge index in reduction → caught by Layer 3 (closed-loop tests)
- *Formula errors*: wrong overhead expression (e.g., linear vs quadratic edge count) → caught by Layer 4 (overhead validation)
- *Test gaming*: agent changes expected value instead of fixing bug → caught by Layer 5 (materialized fixtures)
- *Convention violations*: wrong file naming, missing `declare_variants!` → caught by Layer 6 (agentic review)
- *Logical errors*: incorrect proof argument → caught by Layer 7 (documentation review)

Table 2: Error taxonomy × verification layer matrix (populated from git mining).

#### 6.3 Case Studies (qualitative)

Three reductions spanning the complexity spectrum:

**Simple — MinimumVertexCover → MaximumIndependentSet:**
- Complement relationship: MIS(G) = V \ MVC(G).
- Near-trivial mapping, ~30 LOC.
- Shows the pipeline working smoothly with minimal human intervention.

**Complex — Satisfiability → MaximumIndependentSet:**
- Clause-variable gadget construction, quadratic blowup in edges.
- Requires understanding both CNF formulas and graph structure.
- Shows where agent makes mistakes (edge count in intersection graph) and how verification layers catch them.

**Composition — Factoring → CircuitSAT → ILP (graph-level, not single-agent):**
- Two independently implemented reductions (Factoring→CircuitSAT and CircuitSAT→ILP) that compose in the reduction graph.
- This case study analyzes each reduction's implementation pipeline separately, then demonstrates how the graph enables composition: factor a number by chaining reductions to ILP and using an off-the-shelf solver.
- The "composition" is a property of the graph structure, not a single agent managing a multi-hop chain.
- Highlights the practical value: the library serves as compilation infrastructure.

For each case study: show the full pipeline from issue to merged PR, highlight where human judgment was needed vs. where agent executed autonomously, and which verification layers activated.

### S7. Related Work (~1 page)

**AI coding agents:**
- SWE-agent (ACI design), OpenHands (open platform + SDK), Claude Code (agentic CLI), Devin (autonomous engineer).
- Benchmarks: SWE-Bench Verified (~70-80%), SWE-EVO (~20% on long-horizon), SWE-Bench Pro (~45%).
- Our contribution: skill-based decomposition as an alternative to pushing for more raw capability.
- Live-SWE-agent's self-evolution is complementary — skills are human-authored evolution.

**AI-assisted discovery of reductions and complexity:**
- AlphaEvolve discovers new NP-hardness gadgets (MAX-3-CUT, MAX-4-CUT, metric TSP bounds).
- URSA uses SAT solvers for formal verification of NP-complete reductions.
- Our work is complementary: we focus on implementing and verifying known reductions, not discovering new ones. AlphaEvolve discovers; our pipeline implements and verifies.

**Formal verification of AI-generated code:**
- VeriCoding (27% Lean, 44% Verus, 82% Dafny success rates).
- CLEVER (near-zero on hard Lean problems).
- VeriBench (self-optimizing agents reach ~90% compilation).
- Our approach: pragmatic multi-layer verification instead of end-to-end formal proofs. Trade-off: less formal guarantee, but practically effective at catching real errors.

**Physics-inspired optimization:**
- GNNs via QUBO Hamiltonian relaxation solve MIS, MaxCut, MinVC at million-variable scale.
- Quantum annealing + GNN hybrids for TSP.
- Our reduction graph provides the verified compilation layer that connects arbitrary problems to these solvers.

### S8. Discussion & Conclusion (~1 page)

**Generalizability:**
- What other domains have the "Goldilocks" property? Candidates: compiler optimizations (peephole rules), algebraic identities, protocol verification lemmas.
- The skill-based approach generalizes to any domain where tasks are homogeneous, formally specified, and independently verifiable.

**Limitations:**
- **n=1 threat to validity**: This is a single case study of a single project by a single maintainer. While we argue the methodology generalizes to other Goldilocks domains, the empirical evidence is from one project. We mitigate this by providing the ablation comparison (S6.1) and by identifying concrete candidate domains for future validation.
- Requires upfront skill engineering — the maintainer must invest significant effort in writing and evolving skills.
- Domain expertise embedded in skills doesn't transfer across domains (a reduction skill won't help with web development).
- Git history mining has confounds: skills evolved during the project (addressed by stratification in S6.2).
- The three-role model requires a knowledgeable maintainer; fully open-source contribution without oversight is not supported.

**The human value proposition:**
- Humans are not eliminated from the pipeline — they are repositioned. Creative work (which problems matter, which reductions are useful, what quality standards to enforce) remains human. Mechanical work (implementation, testing, documentation, review) is delegated to agents that also manage their own workflow.
- This mirrors the broader trend identified in industry surveys: developers increasingly use AI but maintain active oversight on delegated tasks.

**Future directions:**
- Connecting to AlphaEvolve-style discovery: use agents to discover new reductions, then feed them into the verification pipeline.
- Formal verification integration: replace round-trip tests with Lean/Coq proofs for the strongest guarantees.
- Scaling the graph: can the pipeline maintain quality as the number of problems grows from 24 to 100+?

## Page Budget

| Section | Pages | Notes |
|---------|-------|-------|
| S1. Introduction | ~1.5 | |
| S2. Why Reductions? | ~1 | Including Fig 1 (reduction graph) |
| S3. System Architecture | ~1.5 | Trimmed; full trait details in supplementary |
| S4. Skill-Based Decomposition | ~2 | Including Fig 3 (pipeline) + Table 1 |
| S5. Verification Stack | ~1.5 | Including Fig 4 (pyramid) |
| S6. Evaluation | ~2.5 | Ablation + git mining + case studies + Table 2 |
| S7. Related Work | ~1 | |
| S8. Discussion | ~1 | |
| **Total** | **~12** | Page counts include embedded figures/tables for each section. References ~0.5 pages. Supplementary material (full trait hierarchy, proc macro details) is a separate appendix outside the page limit, per ICSE/ASE norms. |

## Key Figures

1. **Reduction graph** — 24 problem types, 42 variant nodes, 52 directed edges, color-coded by category. QUBO/ILP hubs visible. Caption distinguishes 40 implemented reductions from 12 inferred variant edges.
2. **System architecture** — Key traits + compile-time validation flow (compact). Full hierarchy in supplementary.
3. **Pipeline diagram** — Three-role pipeline: contributor → issue → agent:check → maintainer:move card → agent:implement/review → PR → merge. Human decisions highlighted in distinct color.
4. **Verification pyramid** — 7 layers from type system (base) to documentation (top), each annotated with concrete error examples.

## Key Tables

1. **Skills inventory** — Each skill with: trigger condition, inputs, outputs, typical agent turns, first-attempt success rate.
2. **Error taxonomy** — Error categories × which verification layer caught them. Demonstrates complementary coverage.

## References

Survey bibliography: `.claude/survey/agentic-coding-reductions/references.bib` (22 papers across 4 themes).

## Non-Goals

- This paper does NOT claim agents can discover new reductions (that's AlphaEvolve territory).
- This paper does NOT provide formal verification proofs (pragmatic multi-layer approach instead).
- This paper does NOT benchmark against SWE-Bench (different task structure; we argue for domain-specific evaluation).

## Artifact Availability

The code repository (including all skill files, git history, and test fixtures) will be made publicly available as a reproducibility artifact. The reduction graph can be explored interactively via the project's MCP server and CLI tool. This supports ICSE/ASE artifact evaluation tracks.
