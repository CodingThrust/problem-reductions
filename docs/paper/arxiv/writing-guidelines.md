# Writing Guidelines

Lessons distilled from studying "Attention Is All You Need" (Vaswani et al., NeurIPS 2017) and applying them to our paper.

## 1. Start from what the reader knows

Open each section with a familiar concept, then pivot to the gap or novelty.

- **Abstract**: Begin with the real-world context ("Many real-world optimization problems..."), not the technical contribution.
- **Introduction**: Start with the concrete problem (airlines, chip designers, logistics), not benchmarks or related work.
- **Each section**: The first sentence should orient the reader, not assume they just read the previous section closely.

**Bad**: "NP-hard problem reductions form a directed graph that serves as compilation infrastructure."
**Good**: "Many real-world optimization problems are computationally hard, yet specialized solvers exist for a handful of them."

## 2. Define every concept before using it

Never use a term or symbol without having introduced it first. If a concept appears in the abstract, it must be self-explanatory in context.

- Spell out all abbreviations on first use: "Maximum Independent Set (MIS)", not just "MIS."
- Define technical terms in plain language before using them: "A *reduction* is a mathematical transformation that converts one problem into another while preserving the solution."
- Introduce notation gradually: describe in words what $G = (V, E)$ means before writing the formula.

**The Vaswani rule**: Before any equation, explain in words what each symbol means and what the equation will do. The math *follows* the intuition, never the reverse.

## 3. One idea per sentence

Short, declarative sentences. Each sentence carries one fact or one claim.

- **Bad**: "The Transformer, which is a novel network architecture based entirely on attention mechanisms rather than recurrence or convolutions, achieves state-of-the-art results."
- **Good**: "The Transformer relies entirely on attention mechanisms. It uses no recurrence or convolution."

Avoid hedging words ("it should be noted that", "it is worth mentioning that"). Just state the fact.

## 4. Lead with the answer, not the reasoning

Put the conclusion first, then the evidence. The reader should know where you're going before you take them there.

- **Bad**: "Because each reduction implements the same trait, follows the same file convention, and requires the same test pattern, reusable skills are possible."
- **Good**: "Reductions form a homogeneous task family, enabling reusable skills. Every reduction implements the same interface, follows the same file convention, and requires the same test pattern."

## 5. Describe the thing, then justify it

"Attention Is All You Need" describes the Transformer architecture in Section 3, then justifies the design choice in Section 4 ("Why Self-Attention"). The reader needs to understand *what* before they can appreciate *why*.

Apply to our paper:
- Section 2 describes the reduction graph and its properties.
- Section 3 describes the methodology (skills, pipeline, verification).
- Justification for choices (why skills? why this verification stack?) follows naturally from the description.

## 6. Use concrete examples to anchor abstractions

Every abstract concept should have a concrete example nearby.

- "Emergent compositionality" → the Factoring → CircuitSAT → ILP story
- "Round-trip testing" → reduce a graph, solve by brute force, extract, verify
- "Quality gate" → 75% rejection rate on 322 batch-submitted issues

When introducing a general pattern, immediately show one instance of it.

## 7. Structure sections as self-contained units

Each section should be readable on its own. A reader who skips straight to Section 4 (Evaluation) should understand what is being evaluated without re-reading Sections 1-3 in detail.

- Re-introduce key terms briefly when they reappear ("round-trip testing, described in Section 2.4, ...").
- Avoid forward references to undefined concepts. If Section 2 mentions "skills," the reader should already have a rough sense of what skills are from the introduction.

## 8. Tables and figures earn their space

Every table and figure must be:
1. Referenced in the text (never orphaned).
2. Self-contained with a caption that explains what the reader should see.
3. Necessary—if the same information fits in one sentence, skip the table.

Captions should tell a story: "Seven-layer verification stack. Each layer catches a distinct class of error that the layers below it miss." Not just: "Verification layers."

## 9. The abstract is a standalone document

The abstract should be understandable by someone who reads *only* the abstract. This means:
- No undefined abbreviations
- No forward references ("as shown in Section 3")
- No citations
- A clear problem → approach → result → significance arc

## 10. Cut ruthlessly

If a sentence doesn't advance the argument, delete it. Common cuts:
- "In this section, we describe..." → just describe it
- "It is important to note that..." → just state the note
- "As mentioned above..." → if it matters, the reader remembers; if not, cut it
- Restating what was just said in different words

The Vaswani paper is 15 pages including references and appendix, covering a paradigm-shifting architecture. If they can do it in 15 pages, so can we.
