# Peer Review Round 1

**Paper:** Skill-Based Agentic Coding for Mathematical Software: A Case Study in NP-Hard Problem Reductions
**Format:** IEEEtran conference (targeting ICSE/ASE-class venue)
**Date:** 2026-03-13

---

## Scores (0--100)

| Aspect          | Score | Assessment       |
|-----------------|-------|------------------|
| Novelty         | 62    | Major Revision   |
| Soundness       | 45    | Major Revision   |
| Significance    | 65    | Borderline       |
| Clarity         | 78    | Minor Revision   |
| Reproducibility | 55    | Major Revision   |

**Overall Recommendation:** Major Revision (borderline Reject)

---

## Reviewer 1: SE Methodology

### Summary
The paper proposes a skill-based decomposition methodology for agentic coding, applied to a Rust library implementing NP-hard problem reductions. The key ideas are: (1) a three-role model separating creative work from mechanical execution, (2) a library of 13 reusable skills that decompose tasks into agent-manageable steps, (3) a 7-layer verification stack, and (4) a card-based orchestration pipeline.

### Strengths
- **S1.** The three-role model (Contributor/Maintainer/Agent) is a clear, well-motivated decomposition of responsibilities. The distinction between "programming the agent's workflow" (maintainer) vs. "programming the agent's output" is insightful.
- **S2.** The 7-layer verification stack is the strongest technical contribution. The layers are well-justified with concrete error examples, and the "lazy agent problem" (agents modifying expected test values) is a genuine, important failure mode.
- **S3.** The paper is well-written with clear prose and good structure. The Goldilocks domain argument is compelling.
- **S4.** The concept of skills as reusable, composable agent workflows is practically useful and clearly explained.

### Weaknesses
- **W1.** The ablation study (Section 6.1) is entirely placeholder [TBD]. This is the most critical missing piece: without a controlled comparison between skill-based and no-skill configurations, the paper's central claim -- that skills improve agent reliability -- is unsupported by direct evidence. The framing text acknowledges this will be "5--10 reductions" but the results are absent.
- **W2.** The error taxonomy table (Table 3) is entirely [TBD]. Without concrete error counts, the verification stack's effectiveness is described only anecdotally.
- **W3.** The success rate column in Table 2 (skills inventory) is entirely [TBD]. These metrics would directly quantify each skill's reliability.
- **W4.** The methodology evaluation relies almost entirely on a single case study with a single project. While Section 7.1 acknowledges this limitation, the paper would be strengthened by even a brief pilot in a second domain.

### Questions for Authors
- Q1. The ablation text says "The ablation results are [TBD]" -- when will these be available? Without them, the evaluation section lacks its primary quantitative evidence.
- Q2. How much human effort (hours) went into developing the 13 skills? This is crucial for assessing the cost-benefit tradeoff.

---

## Reviewer 2: AI/Agents

### Summary
The paper presents a pragmatic methodology for human-agent collaboration in mathematical software development, with skills serving as structured prompts/workflows and a multi-layered verification approach to ensure correctness.

### Strengths
- **S1.** The connection to current agentic coding benchmarks (SWE-Bench, SWE-EVO, SWE-Bench Pro) is well-established and provides useful context for the capability gap the paper addresses.
- **S2.** The "lazy agent problem" -- where agents modify expected test outputs rather than fixing implementation bugs -- is a real and underreported phenomenon. The materialized fixtures defense is a practical contribution.
- **S3.** The fresh-context design for agentic review (dispatching sub-agents without the implementor's context) to prevent sycophancy is a sound design choice with good motivation.
- **S4.** The related work on AI-discovered reductions (FunSearch, AlphaEvolve) and formal verification (VeriCoding, CLEVER) is thorough and well-integrated.

### Weaknesses
- **W1.** The comparison with existing agent benchmarks is unfair in framing. The paper opens by saying agents achieve "70--80% on SWE-Bench Verified" but "below 25% on long-horizon tasks," then implies its methodology bridges this gap -- but never actually measures its own success rate on comparable metrics. The paper should either: (a) define equivalent metrics and report them, or (b) be explicit that it is presenting methodology, not a benchmark comparison.
- **W2.** The paper does not report which LLM(s) were used, which model versions, or any details about the agent's configuration. For reproducibility, readers need to know whether this was Claude 3.5, Claude 4, GPT-4, etc. The methodology's effectiveness may be strongly model-dependent.
- **W3.** The "skills as markdown documents" approach is presented as novel, but prompt engineering / structured agent workflows have been explored by prior work (e.g., chain-of-thought prompting, ReAct, agent tool-use frameworks). The novelty should be more carefully positioned relative to these.
- **W4.** The claim that "developers now use AI in 60% of their work while maintaining active oversight on 80--100% of delegated tasks" (citing Anthropic 2026) is used multiple times but comes from an industry report by the same company whose tool (Claude Code) is used in the study. This creates a potential conflict of interest in citation usage.

### Questions for Authors
- Q1. What model(s) and version(s) were used? Did the model change during the 7-week development period?
- Q2. Has the methodology been tested with non-Anthropic agents (e.g., GPT-4, Gemini)?

---

## Reviewer 3: Devil's Advocate

### Summary
The paper describes a carefully engineered workflow for using coding agents in a specific mathematical domain. While the engineering is thorough, I have serious concerns about the evaluation and the strength of the claims made.

### Strengths
- **S1.** The paper is honest about limitations (Section 7.2), including single case study, skill engineering cost, domain specificity, and confounding factors. This transparency is appreciated.
- **S2.** The concrete artifact (24 problem types, 40 reductions, 52 edges, >95% coverage) is impressive as engineering output.

### Weaknesses (Critical)
- **W1. Incomplete evaluation is a showstopper.** Three of the four main evaluation components are [TBD] placeholders:
  - Ablation results (Section 6.1): entirely missing
  - Error taxonomy counts (Table 3): entirely missing
  - Skill success rates (Table 2): entirely missing

  This means the paper's evaluation section consists of: (a) a description of an experiment that hasn't been run, (b) a descriptive git history summary with no quantitative findings, and (c) three case studies. This is insufficient for a top-venue submission.

- **W2. Timeline inconsistency.** The abstract claims "six months" of development, but Section 6.2 says "approximately seven weeks" spanning 58 PRs. The git data confirms ~47 days (6.6 weeks) from first to last PR. This is a factual contradiction that undermines credibility.

- **W3. Author count inconsistency.** Section 6.2 says "two primary contributors" but the git data shows three distinct authors (GiggleLiu, isPANN, zazabap). While one contributor may have minor contributions, this should be stated accurately.

- **W4. N=1 threat to validity.** The entire evaluation is based on a single project by a single primary developer. The generalizability claims in Section 7.1 are aspirational -- listing candidate domains (compiler passes, numerical linear algebra, etc.) without any evidence. A hostile reviewer would argue this is an experience report dressed as a methodology paper.

- **W5. Circular reasoning in verification stack.** The paper claims the 7-layer verification stack catches errors, but the evidence for this is anecdotal ("we observed this failure mode multiple times"). Without systematic error counts (Table 3 is TBD), the claim that "this layer catches approximately 60% of the errors that survive type checking" (Section 5.1, Layer 3) is unsubstantiated.

- **W6. No baseline comparison.** The paper compares against SWE-Bench and SWE-EVO numbers but never runs its own tasks through a no-skill baseline. The ablation is designed but not executed. Without this, the reader cannot distinguish whether the results come from the skill methodology, the domain's inherent verifiability, or the specific LLM's capability.

### Weaknesses (Major)
- **W7.** The paper is ~14 pages in IEEEtran conference format. ICSE/ASE typically allows 10--12 pages. The paper needs significant trimming (~2--4 pages).

- **W8.** The "60% of errors" claim for Layer 3 (line 402) has no citation, no data source, and no methodology for arriving at this number. It reads as an estimate presented as a finding.

- **W9.** The paper conflates "agent-generated code" with "agent-assisted code." Since all PRs are attributed to human GitHub accounts (Section 6.2), there is no way to distinguish which code was human-written vs. agent-written. The paper acknowledges this as "a finding about observability limitations" but this also means the paper cannot quantify agent contributions.

- **W10.** Several citations have issues:
  - `Anthropic2026AgenticCoding` is a tech report by the tool vendor, cited 3+ times as if it were independent research
  - `lucas2014` is cited as evidence that "Rydberg atom arrays natively solve MIS" but Lucas 2014 is about Ising formulations, not Rydberg atoms specifically (the Rydberg atom connection to MIS came later, ~2018+)
  - The bib file has `@article` entries with both `booktitle` and `journal` fields (e.g., Yang2024SWEagent, He2024QuantumTSP), which is malformed BibTeX

### Weaknesses (Minor)
- **W11.** The `\author{...}` placeholder (line 17) should be filled in for submission.
- **W12.** The abstract mentions "six months" but should be revised to match the actual timeline.
- **W13.** Section 2 paragraph "Hardware solvers as practical motivation" could be shortened; it reads more like a grant proposal than a conference paper.
- **W14.** The paper would benefit from a threat-to-validity section separate from limitations, following SE convention.
- **W15.** No appendix or supplementary material is referenced for the full skill markdown files, which would aid reproducibility.
- **W16.** The paper uses `\Cref` (cleveref) throughout but does not appear to load it with any options for IEEEtran compatibility. This may cause formatting issues.

---

## Critical Issues (Must Fix)

1. **[C1] Timeline contradiction (abstract vs. Section 6.2).** The abstract says "six months" but Section 6.2 says "seven weeks" and the data confirms ~47 days. Fix: align to the actual timeline. (Affects: Soundness)

2. **[C2] TBD placeholders in evaluation.** Three tables/results are entirely [TBD]: ablation results (Section 6.1), error taxonomy counts (Table 3), skill success rates (Table 2). The paper cannot be submitted with placeholder data. Fix: either run the experiments and fill in real data, or restructure the evaluation to remove the ablation framing and present what data exists. (Affects: Soundness, Significance)

3. **[C3] Unsubstantiated "60% of errors" claim.** The claim that closed-loop tests catch "approximately 60% of the errors that survive type checking" (Section 5.1) has no supporting data. Fix: either provide the data from the error taxonomy audit, or soften to qualitative language ("a majority of errors" or "the largest share of errors in our experience"). (Affects: Soundness)

4. **[C4] Author count factual error.** "Two primary contributors" but three distinct authors in git history. Fix: say "three contributors" or "two primary contributors and one additional contributor." (Affects: Soundness)

## Major Issues (Should Fix)

5. **[M1] No LLM model identification.** The paper never specifies which language model(s) were used. This is essential for reproducibility and for understanding whether results generalize across models.

6. **[M2] Page count.** At ~14 pages, the paper exceeds typical ICSE/ASE limits (10--12 pages). The hardware solvers paragraph and some related work could be condensed.

7. **[M3] Vendor citation bias.** The Anthropic 2026 report is cited 3 times as supporting evidence. At minimum, note that this is a vendor report, or balance with independent sources.

8. **[M4] Missing threats to validity section.** SE venues expect explicit threats-to-validity discussion (internal, external, construct validity). The limitations section partially covers this but not in the expected format.

9. **[M5] Malformed BibTeX entries.** Several entries have both `booktitle` and `journal` fields. These will produce warnings or malformed references.

10. **[M6] Novelty positioning vs. prompt engineering.** The paper should more explicitly differentiate "skills" from existing prompt engineering techniques (chain-of-thought, ReAct, structured prompts).

## Minor Issues (Nice to Fix)

11. **[m1]** Author placeholder `\author{...}` needs to be filled.
12. **[m2]** The `lucas2014` citation for Rydberg atoms is imprecise; consider citing the Pichler et al. 2018 work specifically for the MIS-Rydberg connection.
13. **[m3]** Table 2 caption says "Success rate is the fraction of invocations that pass CI on first attempt, measured from git history" but the column is all TBD -- the caption should not describe methodology for data that doesn't exist yet.
14. **[m4]** Section 6.2 could benefit from a timeline figure showing the three phases.
15. **[m5]** The case studies (Section 6.3) are descriptive but lack quantitative comparison (e.g., agent time vs. estimated human time, number of iterations).
16. **[m6]** Consider adding a data availability statement pointing to the repository.
17. **[m7]** The paper uses both "coding agent" and "AI agent" -- consider standardizing terminology.
18. **[m8]** cleveref package may need `[capitalise]` option or `\Cref`/`\cref` consistency check for IEEEtran.

---

## Summary Assessment

The paper presents a well-engineered system with genuine practical contributions, particularly the 7-layer verification stack and the "lazy agent problem" defense. The writing quality is high and the domain motivation is compelling. However, the evaluation is critically incomplete: the ablation study has not been run, error counts are missing, and skill success rates are placeholders. The timeline contradiction between abstract and body is a factual error that must be fixed. In its current state, the paper reads as an experience report with a methodology sketch, not a fully evaluated research contribution.

**Verdict:** Major Revision. The methodology and system design are promising, but the paper needs: (1) completed evaluation data or restructured claims that match available evidence, (2) factual corrections (timeline, author count), (3) model identification for reproducibility, and (4) approximately 2--4 pages of trimming for conference format.

The strongest path to acceptance: reframe the evaluation around the git mining data and case studies that do exist, acknowledge the ablation as future work rather than presenting it as a designed-but-unrun experiment, and add the model identification details.
