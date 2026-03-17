# Re-extract All Reduction Rules from Book (Verbatim)

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace all reduction JSON files with faithful verbatim extractions from the book — only `id`, `from_problem`, `to_problem`, `source_location`, and `text` (verbatim book text). No `augmented` section. No paraphrasing. No added fields.

**Architecture:** Seven parallel subagents, one per scan chunk (C01–C07). Each subagent reads the scan JSON for the list of reductions in scope, reads the PDF pages, and overwrites each R-file with verbatim text. Problem names must exactly match the names used in the corresponding scan file.

**Schema (every file):**

```json
{
  "id": "R01",
  "from_problem": "SATISFIABILITY (SAT)",
  "to_problem": "3-SATISFIABILITY (3SAT)",
  "source_location": "Theorem 3.1, p.48-50",
  "text": "...verbatim text from book..."
}
```

---

## Rules

- Copy `from_problem`, `to_problem`, `source_location` exactly from the corresponding scan JSON entry.
- `text`: copy all prose text from the book verbatim for that entry. Do not paraphrase, summarize, or add anything.
- **Chapter 3 reductions (C01, C02):** include the full theorem/lemma statement + all proof text.
- **Appendix reductions (C03–C07):** include the full text of the appendix entry for the *target* problem, which typically contains the "Transformation from: X [ref]" line plus the INSTANCE and QUESTION text.
- Skip figure images. Figure captions that appear as inline text may be included.
- Acceptable OCR artifacts: `E` for `∈`, approximate subscripts/superscripts.
- Use Write tool to overwrite each R-file completely.

---

## Task 1 — C01 (PDF p.56–75, book p.45–64)

**Subagent prompt:**

```
You are extracting text verbatim from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read references/Garey&Johnson/json/scan_C01.json to get the list of reductions and their exact from_problem, to_problem, source_location values.

Step 2: Read PDF pages 59–73 (max 20 pages per call; split into two calls: 59–73 and 73–75 if needed).

Step 3: For each reduction in scan_C01.json, write its R-file with exactly five fields:
  id, from_problem, to_problem, source_location, text

  Use the exact from_problem / to_problem / source_location from the scan file.
  text = verbatim prose from the book for that theorem/lemma/section entry.
  - For theorem entries: full theorem statement + full proof text.
  - For restriction/lemma entries: the full paragraph(s) from the book.
  - Skip figures.

R-files to write (from scan_C01.json reductions):
  references/Garey&Johnson/reductions/R01_sat_3sat.json
  references/Garey&Johnson/reductions/R02_3sat_3dm.json
  references/Garey&Johnson/reductions/R03_3dm_x3c.json
  references/Garey&Johnson/reductions/R04_vc_is.json         (create if missing)
  references/Garey&Johnson/reductions/R05_vc_clique.json
  references/Garey&Johnson/reductions/R06_3sat_vc.json
  references/Garey&Johnson/reductions/R07_vc_hc.json
  references/Garey&Johnson/reductions/R08_hc_hp.json
  references/Garey&Johnson/reductions/R09_hc_dhc.json
  references/Garey&Johnson/reductions/R10_3dm_partition.json
  references/Garey&Johnson/reductions/R11_x3c_mincover.json
  references/Garey&Johnson/reductions/R12_vc_hittingset.json
  references/Garey&Johnson/reductions/R13_clique_subiso.json
  references/Garey&Johnson/reductions/R14_hp_bdst.json

All paths relative to /Users/xiweipan/Codes/problem-reductions/.
```

---

## Task 2 — C02 (PDF p.76–87, book p.65–76)

**Subagent prompt:**

```
You are extracting text verbatim from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read references/Garey&Johnson/json/scan_C02.json to get the list of reductions.

Step 2: Read PDF pages 76–87 (two calls if needed: 76–87 is 12 pages, fits in one call).

Step 3: For each reduction in scan_C02.json, write its R-file with exactly five fields:
  id, from_problem, to_problem, source_location, text

  Use exact from_problem / to_problem / source_location from the scan file.
  For theorem entries: full theorem statement + proof text.
  For restriction entries: the full paragraph(s) from the book.
  Skip figures.

R-files to write (match each scan_C02.json reduction to the existing R-file by from/to problem name):
  references/Garey&Johnson/reductions/R15_dhc_minequivdigraph.json
  references/Garey&Johnson/reductions/R16_partition_knapsack.json
  references/Garey&Johnson/reductions/R17_partition_multiprocessorscheduling.json
  references/Garey&Johnson/reductions/R18_vc_ensemblecomputation.json
  references/Garey&Johnson/reductions/R19_x3c_partitionintotriangles.json
  references/Garey&Johnson/reductions/R20_partition_sequencingwithinintervals.json
  references/Garey&Johnson/reductions/R21_3dm_mintestcollection.json
  references/Garey&Johnson/reductions/R22_clique_mintardinesssequencing.json

Also handle R25 (3SAT→CLIQUE) which is referenced from Section 3.1.3, p.54 — check PDF p.65.

All paths relative to /Users/xiweipan/Codes/problem-reductions/.
```

---

## Task 3 — C03 (PDF p.198–217, book p.187–206, Appendix A1)

**Subagent prompt:**

```
You are extracting text verbatim from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read references/Garey&Johnson/json/scan_C03.json to get the list of reductions in this chunk.

Step 2: Read PDF pages 198–217 in chunks (max 20 pages per call).

Step 3: For each reduction in scan_C03.json, find the corresponding R-file under
  references/Garey&Johnson/reductions/ by matching from_problem and to_problem.
  Overwrite it with exactly five fields: id, from_problem, to_problem, source_location, text.

  Use exact from_problem / to_problem / source_location from the scan file.
  text = the full text of the appendix entry for the TARGET problem, which contains
         the "Transformation from: X [ref]" line. Include the INSTANCE and QUESTION
         text if present in that entry.

All paths relative to /Users/xiweipan/Codes/problem-reductions/.
```

---

## Task 4 — C04 (PDF p.218–237, book p.207–226, Appendix A2)

**Subagent prompt:**

```
You are extracting text verbatim from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read references/Garey&Johnson/json/scan_C04.json to get the list of reductions.

Step 2: Read PDF pages 218–237 in chunks (max 20 pages per call).

Step 3: For each reduction in scan_C04.json, find the corresponding R-file under
  references/Garey&Johnson/reductions/ by matching from_problem and to_problem.
  Overwrite with: id, from_problem, to_problem, source_location, text.
  Same rules as Task 3.

All paths relative to /Users/xiweipan/Codes/problem-reductions/.
```

---

## Task 5 — C05 (PDF p.238–257, book p.227–246, Appendix A4–A5)

**Subagent prompt:**

```
You are extracting text verbatim from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read references/Garey&Johnson/json/scan_C05.json to get the list of reductions.

Step 2: Read PDF pages 238–257 in chunks (max 20 pages per call).

Step 3: For each reduction in scan_C05.json, find the corresponding R-file under
  references/Garey&Johnson/reductions/ by matching from_problem and to_problem.
  Overwrite with: id, from_problem, to_problem, source_location, text.
  Same rules as Task 3.

All paths relative to /Users/xiweipan/Codes/problem-reductions/.
```

---

## Task 6 — C06 (PDF p.258–277, book p.247–266, Appendix A6–A10)

**Subagent prompt:**

```
You are extracting text verbatim from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read references/Garey&Johnson/json/scan_C06.json to get the list of reductions.

Step 2: Read PDF pages 258–277 in chunks (max 20 pages per call).

Step 3: For each reduction in scan_C06.json, find the corresponding R-file under
  references/Garey&Johnson/reductions/ by matching from_problem and to_problem.
  Overwrite with: id, from_problem, to_problem, source_location, text.
  Same rules as Task 3.

All paths relative to /Users/xiweipan/Codes/problem-reductions/.
```

---

## Task 7 — C07 (PDF p.278–299, book p.267–288, Appendix A10 continued)

**Subagent prompt:**

```
You are extracting text verbatim from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read references/Garey&Johnson/json/scan_C07.json to get the list of reductions.

Step 2: Read PDF pages 278–299 in chunks (max 20 pages per call).

Step 3: For each reduction in scan_C07.json, find the corresponding R-file under
  references/Garey&Johnson/reductions/ by matching from_problem and to_problem.
  Overwrite with: id, from_problem, to_problem, source_location, text.
  Same rules as Task 3.

All paths relative to /Users/xiweipan/Codes/problem-reductions/.
```

---

## After All Tasks Complete

Spot-check 5 files across different chunks: verify `text` contains verbatim book content by comparing against the PDF.
