# Re-Extract Problem Definitions

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Extract every problem definition verbatim from the entire Garey & Johnson book — Chapter 3 and all appendix sections A1–A12 — creating one simple JSON file per problem.

**Architecture:** 14 parallel extraction subagents (Tasks 0–13), each reading their PDF section directly and creating `P_<name>.json` files. A final consolidation task (Task 14) assigns sequential IDs.

**PDF:** `/Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf`
**Output directory:** `/Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/`
**PDF page offset:** PDF page = book page + 11

---

## Schema (every new file)

```json
{
  "id": "P_<lowercasename>",
  "problem_name": "EXACT PROBLEM NAME FROM BOOK (ALL CAPS)",
  "source_location": "Chapter 3, p.46",
  "text": "INSTANCE: ...\nQUESTION: ..."
}
```

- `id`: `"P_<lowercasename>"` placeholder — Task 14 replaces with sequential IDs starting P333
- `problem_name`: exact name as it appears in the book (all caps)
- `source_location`: chapter + page for Ch.3 problems; `"A1.1 GT5"` for appendix entries
- `text`: full verbatim text — problem name header, INSTANCE, QUESTION, Reference, Comment (all sections, verbatim, no paraphrasing)

---

## Task 0 — Chapter 3 (PDF p.47–100, book p.36–89)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 47–100 (book pp.36–89, Chapter 3: Provably Difficult Combinatorial Problems).

Step 2: Find every problem defined with the INSTANCE/QUESTION format in Chapter 3. These include the six basic NP-complete problems and all intermediate problems introduced during the reduction proofs (e.g., SATISFIABILITY, 3SAT, 3-DIMENSIONAL MATCHING, EXACT COVER BY 3-SETS, VERTEX COVER, CLIQUE, INDEPENDENT SET, HAMILTONIAN CIRCUIT, PARTITION, SEQUENCING WITHIN INTERVALS, etc.).

Step 3: For each problem found, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Fields (exactly these 4 keys):
    "id": "P_<lowercasename>"
    "problem_name": exact problem name from the book (ALL CAPS as printed)
    "source_location": e.g. "Chapter 3, p.46" or "Chapter 3, Section 3.2.1, p.64"
    "text": full verbatim INSTANCE + QUESTION text as it appears (include any immediately following Note or Proof restriction if it is part of the problem definition block)

Rules:
- Verbatim only. No paraphrasing.
- Only extract problems that have an explicit INSTANCE:/QUESTION: format in the book.
- Use the Write tool to create each file.

Return a list of files created with their problem names.
```

---

## Task 1 — GT A1.1 (PDF p.201–204, book p.190–193)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 201–204 (book pp.190–193, section A1.1 Covering and Partitioning, GT1–GT18).

Step 2: For EVERY entry GT1 through GT18 found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Fields (exactly these 4 keys):
    "id": "P_<lowercasename>"
    "problem_name": exact problem name from book header (ALL CAPS as in book)
    "source_location": e.g. "A1.1 GT5"
    "text": full verbatim text — INSTANCE, QUESTION, Reference, Comment (all sections verbatim)

Rules:
- Verbatim only. No paraphrasing.
- Skip figure images. Include figure captions if they appear as inline text.
- Use the Write tool to create each file.

Return a list of files created.
```

---

## Task 2 — GT A1.2 (PDF p.205–210, book p.194–199)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 205–210 (book pp.194–199, section A1.2 Subgraphs and Supergraphs, GT19–GT36).

Step 2: For EVERY entry GT19 through GT36 found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A1.2 GT21"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 3 — GT A1.3–A1.5 (PDF p.209–217, book p.198–206)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 209–217 (book pp.198–206, A1.3 Vertex Ordering + A1.4 Morphisms + A1.5 Miscellaneous, GT37–GT65).

Step 2: For EVERY entry GT37 through GT65 found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A1.3 GT40"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 4 — ND A2 part 1 (PDF p.206–218, book p.195–207, ND1–ND25)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 206–218 (book pp.195–207, section A2 Network Design, ND1–ND25).

Step 2: For EVERY entry ND1 through ND25 found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A2 ND5"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 5 — ND A2 part 2 + SP A3 (PDF p.218–236, book p.207–225)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 218–236 (book pp.207–225, ND26–ND51 and A3 Sets & Partitions SP1–SP21).

Step 2: For EVERY entry ND26–ND51 and SP1–SP21 found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A2 ND30" or "A3 SP7"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 6 — SR A4 (PDF p.237–249, book p.226–238)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 237–249 (book pp.226–238, section A4 Storage and Retrieval, SR entries).

Step 2: For EVERY SR entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A4 SR3"

Rules: Verbatim only. Use Write tool.

Return a list of files created (include total SR entry count found).
```

---

## Task 7 — SS A5 (PDF p.247–257, book p.236–246)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 247–257 (book pp.236–246, section A5 Sequencing and Scheduling, SS entries).

Step 2: For EVERY SS entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A5 SS4"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 8 — MP A6 (PDF p.256–261, book p.245–250)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 256–261 (book pp.245–250, section A6 Mathematical Programming, MP entries).

Step 2: For EVERY MP entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A6 MP2"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 9 — AN A7 (PDF p.260–265, book p.249–254)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 260–265 (book pp.249–254, section A7 Algebra and Number Theory, AN entries).

Step 2: For EVERY AN entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A7 AN5"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 10 — GP A8 (PDF p.265–270, book p.254–259)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 265–270 (book pp.254–259, section A8 Games and Puzzles, GP entries).

Step 2: For EVERY GP entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A8 GP3"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 11 — LO A9 (PDF p.270–277, book p.259–266)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 270–277 (book pp.259–266, section A9 Logic, LO entries).

Step 2: For EVERY LO entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A9 LO7"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 12 — AL A10 (PDF p.277–282, book p.266–271)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 277–282 (book pp.266–271, section A10 Automata and Language Theory, AL1–AL21).

Step 2: For EVERY AL entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A10 AL7"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 13 — PO A11 + MS A12 (PDF p.283–295, book p.272–284)

**Subagent prompt:**

```
Extract verbatim problem definitions from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 283–295 (book pp.272–284, A11 Program Optimization PO1–PO20 + A12 Miscellaneous MS1–MS19).

Step 2: For EVERY PO and MS entry found on those pages, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/P_<lowercasename>.json

  Same 4-field schema. source_location: e.g. "A11 PO2" or "A12 MS5"

Rules: Verbatim only. Use Write tool.

Return a list of files created.
```

---

## Task 14 — ID Consolidation

After Tasks 0–13 complete, assign sequential IDs to all `P_*.json` files.

**Step 1:** List placeholder files:

```bash
cd /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/models/
ls P_*.json | wc -l
```

**Step 2:** Sort by section order (Chapter 3 first, then GT, ND, SP, SR, SS, MP, AN, GP, LO, AL, PO, MS), then by entry number within section. Read `source_location` from each file to sort.

**Step 3:** Assign IDs starting from P333. For each file:
- Read file, update `"id"` field to `"P{N}"`
- Rename `P_<name>.json` → `P{N}_<name>.json`

**Step 4:** Verify:

```bash
ls P[0-9]*.json | grep -E '^P3[3-9][0-9]_|^P[4-9][0-9]{2}_' | sed 's/_.*$//' | sort | uniq -d
```

Expected: no output (no duplicates in the new range).
