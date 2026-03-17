# Garey & Johnson Source Field Verification Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Verify every field in `source` sections of Chapter 3 JSONs against the book, fixing errors/omissions directly and moving any AI-generated content out of `source` into `augmented`.

**Architecture:** Two parallel subagents cover the two PDF chunks for Chapter 3 (p.45–76). Each subagent reads the assigned PDF pages, checks each JSON's `source` fields against the book text, and edits the JSONs in-place — no separate report files. The invariant is: after verification, `source` contains only content verifiable from the cited pages.

**Scope:** Chapter 3 only (31 models + 22 reductions). Appendix problems (P36+) are a separate effort.

---

## Rules for Each JSON

When reading the book page(s) for a problem/reduction:

| Finding | Action |
|---------|--------|
| `source` field matches book text | Leave unchanged |
| `source` field has wrong/inaccurate text | Fix the text directly in `source` |
| `source` field contains content NOT in the book (AI-generated) | Move the field to `augmented` |
| Book has content not captured in `source` | Add a new field to `source` |
| `source_location` page number is wrong | Fix it |

**Marking convention:** The field's location is the mark. `source` = from book. `augmented` = AI-generated. No extra annotation fields needed.

---

## Task 1 — Verify C01 chunk (PDF p.56–75, book p.45–64)

**Dispatch one subagent with this exact prompt:**

```
You are verifying Garey & Johnson JSON files against the book PDF.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Your PDF page range: 56–75 (book pages 45–64, Chapter 3, Sections 3.1.1–3.1.5 and start of 3.2.1)

JSON files to verify (models):
- references/Garey&Johnson/models/P02_3dimensional_matching.json     (book p.46)
- references/Garey&Johnson/models/P06_hamiltonian_circuit.json       (book p.47, 56–60)
- references/Garey&Johnson/models/P07_partition.json                 (book p.47, 60–62)
- references/Garey&Johnson/models/P08_exact_cover_by_3sets.json      (book p.53)
- references/Garey&Johnson/models/P09_hamiltonian_path.json          (book p.60)
- references/Garey&Johnson/models/P10_hamiltonian_path_between_two_points.json (book p.60)
- references/Garey&Johnson/models/P11_directed_hamiltonian_circuit.json (book p.60)
- references/Garey&Johnson/models/P12_directed_hamiltonian_path.json (book p.60)
- references/Garey&Johnson/models/P13_minimum_cover.json             (book p.64)
- references/Garey&Johnson/models/P14_hitting_set.json               (book p.64)
- references/Garey&Johnson/models/P15_subgraph_isomorphism.json      (book p.64)
- references/Garey&Johnson/models/P16_bounded_degree_spanning_tree.json (book p.64)

JSON files to verify (reductions):
- references/Garey&Johnson/reductions/R02_3sat_3dm.json             (Theorem 3.2, p.50–53)
- references/Garey&Johnson/reductions/R03_3dm_x3c.json              (Section 3.1.2, p.53)
- references/Garey&Johnson/reductions/R05_vc_clique.json            (Lemma 3.1, p.54)
- references/Garey&Johnson/reductions/R06_3sat_vc.json              (Theorem 3.3, p.54–56)
- references/Garey&Johnson/reductions/R07_vc_hc.json                (Theorem 3.4, p.56–60)
- references/Garey&Johnson/reductions/R08_hc_hp.json                (Section 3.1.4, p.60)
- references/Garey&Johnson/reductions/R09_hc_dhc.json               (Section 3.1.4, p.60)
- references/Garey&Johnson/reductions/R10_3dm_partition.json        (Theorem 3.5, p.60–62)
- references/Garey&Johnson/reductions/R11_x3c_mincover.json         (Section 3.2.1, p.64)
- references/Garey&Johnson/reductions/R12_vc_hittingset.json        (Section 3.2.1, p.64)
- references/Garey&Johnson/reductions/R13_clique_subiso.json        (Section 3.2.1, p.64)
- references/Garey&Johnson/reductions/R14_hp_bdst.json              (Section 3.2.1, p.64)

For each JSON:

1. Read the JSON (using Read tool)
2. Read the relevant PDF pages (using Read tool with pages parameter, max 20 pages per call)
3. For each field in the "source" section, check against the book:
   - mathematical_definition: must match the book's INSTANCE/QUESTION text verbatim (minor spacing/symbol differences are OK)
   - problem_name: must match book's exact name
   - source_location: page numbers must be correct
   - reduction_type (reductions): must match what the book says (Karp/restriction/etc.)
   - source_reference (reductions): theorem/lemma number and page must be correct
   - Any other "source" fields: verify they appear in the book

4. Edit the JSON directly:
   - Wrong text → fix it in "source"
   - Content NOT in the book found in "source" → move that field to "augmented"
   - Book content missing from "source" → add it to "source"
   - Wrong page number → fix source_location

Note on P02: mathematical_definition already verified as MATCH (p.46). Check remaining source fields.
Note: OCR symbol differences (∈ vs E, subscripts) are acceptable — mark as cosmetic only, don't change the JSON for those.
Note: The book uses 3DM component names "truth-setting and fan-out", not "truth-setting" alone — check if this matters for any field.

Read PDF in chunks as needed (max 20 pages per Read call).
```

---

## Task 2 — Verify C02 chunk (PDF p.76–87, book p.65–76)

**Dispatch one subagent with this exact prompt:**

```
You are verifying Garey & Johnson JSON files against the book PDF.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Your PDF page range: 76–87 (book pages 65–76, Chapter 3, Sections 3.2.1 continued through 3.3)

JSON files to verify (models):
- references/Garey&Johnson/models/P17_minimum_equivalent_digraph.json  (book p.65)
- references/Garey&Johnson/models/P18_knapsack.json                     (book p.65)
- references/Garey&Johnson/models/P19_multiprocessor_scheduling.json    (book p.65)
- references/Garey&Johnson/models/P20_ensemble_computation.json         (book p.66–67)
- references/Garey&Johnson/models/P21_partition_into_triangles.json     (book p.68–69)
- references/Garey&Johnson/models/P22_sequencing_within_intervals.json  (book p.70–71)
- references/Garey&Johnson/models/P23_minimum_test_collection.json      (book p.71–72)
- references/Garey&Johnson/models/P24_minimum_tardiness_sequencing.json (book p.73–74)
- references/Garey&Johnson/models/P25_longest_path.json                 (book p.75, exercise)
- references/Garey&Johnson/models/P26_partition_into_hamiltonian_subgraphs.json (book p.75, exercise)
- references/Garey&Johnson/models/P27_largest_common_subgraph.json      (book p.75, exercise)
- references/Garey&Johnson/models/P28_minimum_sum_of_squares.json       (book p.75, exercise)
- references/Garey&Johnson/models/P29_feedback_vertex_set.json          (book p.75, exercise)
- references/Garey&Johnson/models/P30_exact_cover_by_4sets.json         (book p.75, exercise)
- references/Garey&Johnson/models/P31_steiner_tree_in_graphs.json       (book p.75, exercise)
- references/Garey&Johnson/models/P32_star_free_regular_expression_inequivalence.json (book p.76, exercise)
- references/Garey&Johnson/models/P33_set_splitting.json                (book p.76, exercise)
- references/Garey&Johnson/models/P34_partition_into_paths_of_length_2.json (book p.76, exercise)
- references/Garey&Johnson/models/P35_graph_grundy_numbering.json       (book p.76, exercise)

JSON files to verify (reductions):
- references/Garey&Johnson/reductions/R15_dhc_minequivdigraph.json          (Section 3.2.1, p.65)
- references/Garey&Johnson/reductions/R16_partition_knapsack.json            (Section 3.2.1, p.65)
- references/Garey&Johnson/reductions/R17_partition_multiprocessorscheduling.json (Section 3.2.1, p.65)
- references/Garey&Johnson/reductions/R18_vc_ensemblecomputation.json       (Theorem 3.6, p.66–68)
- references/Garey&Johnson/reductions/R19_x3c_partitionintotriangles.json   (Theorem 3.7, p.68–69)
- references/Garey&Johnson/reductions/R20_partition_sequencingwithinintervals.json (Theorem 3.8, p.70–71)
- references/Garey&Johnson/reductions/R21_3dm_mintestcollection.json        (Theorem 3.9, p.71–72)
- references/Garey&Johnson/reductions/R22_clique_mintardinesssequencing.json (Theorem 3.10, p.73–74)
- references/Garey&Johnson/reductions/R25_3sat_clique.json                  (p.54–56)

For each JSON:

1. Read the JSON (using Read tool)
2. Read the relevant PDF pages (using Read tool with pages parameter, max 20 pages per call)
3. For each field in "source", check against the book text:
   - mathematical_definition: must match INSTANCE/QUESTION verbatim
   - problem_name: exact book name
   - source_location: correct pages
   - For Section 3.3 exercises (P25–P35): book only gives a brief exercise statement, NOT a full
     INSTANCE/QUESTION definition. If the JSON has a full formal definition not in the exercise
     text, move mathematical_definition to "augmented" (it's AI-generated).
   - For reductions: reduction_type and source_reference

4. Edit JSON directly (same rules as Task 1).

IMPORTANT for Section 3.3 exercises: the book's Section 3.3 (p.75–76) lists problems as
exercises with brief descriptions like "Show that LONGEST PATH is NP-complete." There is NO
full INSTANCE/QUESTION definition in Section 3.3 — those appear only in the Appendix.
If a model's source_location says "Section 3.3 (Exercise N), p.75" and it has a full
mathematical_definition, that definition is AI-generated and must be moved to "augmented".
The source_location should be updated to also reference the Appendix entry where the real
definition appears.

Read PDF in chunks (max 20 pages per Read call).
```

---

## After Both Tasks Complete

Review a sample of edits (3–5 random JSONs) to confirm the rules were applied consistently. If the Section 3.3 exercise problem definitions were all moved to `augmented`, note that their real `mathematical_definition` needs to be re-extracted from the Appendix entries in a follow-up pass.
