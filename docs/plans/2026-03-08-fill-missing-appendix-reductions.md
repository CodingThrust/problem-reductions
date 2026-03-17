# Fill Missing Appendix Reductions

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Extract all missing Garey & Johnson appendix entries (R-files) from the PDF, filling ~112 gaps identified by direct PDF audit.

**Architecture:** One cleanup task (Task 0), then 8 parallel extraction subagents (Tasks 1–8), each reading their PDF section directly and creating new R_ files. A final consolidation task (Task 9) assigns sequential IDs to all R_ files and updates R229's wrong `from_problem`.

**Schema (every new file):**
```json
{
  "id": "R_<from>_<to>",
  "from_problem": "...(exact from book's 'Transformation from:' line)...",
  "to_problem": "...(exact problem name from book header)...",
  "source_location": "A1.1 GT5",
  "text": "...verbatim text from book..."
}
```
Use `"id": "R_<from>_<to>"` as a placeholder — Task 9 replaces these with real sequential IDs.
If the entry has no "Transformation from:" line (generic NP-completeness), set `from_problem` to `"generic"`.

---

## Background: What already exists

The following R-files already exist and must NOT be overwritten:
- R01–R22: Chapter 3 reductions
- R06 (GT1 VERTEX COVER), R_3sat_dominatingset (GT2), R_3sat_domaticnumber (GT3), R_3sat_graphkcolorability (GT4)
- R23 (GT7 FEEDBACK VERTEX SET), R24 (GT8 FEEDBACK ARC SET)
- R19 (GT11 PARTITION INTO TRIANGLES), R25 (GT19 CLIQUE), R04 (GT20 INDEPENDENT SET)
- R26 (GT24 BALANCED COMPLETE BIPARTITE SUBGRAPH), R15 (GT33 MIN EQUIV DIGRAPH)
- R07 (GT37 HAMILTONIAN CIRCUIT), R13 (GT48 SUBGRAPH ISOMORPHISM)
- R27–R89: A2 ND + A3 SP + SR1–SR3
- R098–R163: A4 SR + A5 SS + A6 MP
- R164–R181: A7 AN (all 18 entries)
- R182–R196: A8 GP (all 15 entries)
- R197–R215: A9 LO (all 19 entries)
- R216–R221: A10 AL1–AL6
- R224 (AL9), R225 (PO1), R226 (PO3 — but has wrong from_problem), R229 (MS1 — wrong from_problem)

**Broken files to delete (Task 0):** R222, R223, R227, R228 (empty text, wrong problem identities).

---

## Task 0 — Cleanup

Delete the 4 broken files created by C07 with empty text and wrong mappings:
```
references/Garey&Johnson/reductions/R222_3sat_nonerasingstackautomaton.json
references/Garey&Johnson/reductions/R223_3sat_fsaintersection.json
references/Garey&Johnson/reductions/R227_x3c_latinsquarecompletion.json
references/Garey&Johnson/reductions/R228_feedbackvertexset_maximumacyclicsubgraph.json
```

Use the Bash tool: `rm` each file. Verify with `ls` that they're gone.

---

## Task 1 — GT A1.1 missing (PDF p.201–204, book p.190–193)

**Subagent prompt:**
```
Extract verbatim text from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 201–204 (book pp.190–193, section A1.1 Covering and Partitioning).

Step 2: Extract these entries which are MISSING R-files:
  GT5  | ACHROMATIC NUMBER         | from: MINIMUM MAXIMAL MATCHING
  GT6  | MONOCHROMATIC TRIANGLE    | from: 3SAT
  GT9  | PARTIAL FEEDBACK EDGE SET | from: VERTEX COVER
  GT10 | MINIMUM MAXIMAL MATCHING  | from: VERTEX COVER (for cubic graphs)
  GT12 | PARTITION INTO ISOMORPHIC SUBGRAPHS   | from: 3DM
  GT13 | PARTITION INTO HAMILTONIAN SUBGRAPHS  | from: 3SAT
  GT14 | PARTITION INTO FORESTS    | from: GRAPH 3-COLORABILITY
  GT15 | PARTITION INTO CLIQUES    | from: GRAPH K-COLORABILITY
  GT16 | PARTITION INTO PERFECT MATCHINGS      | from: NOT-ALL-EQUAL 3SAT
  GT17 | COVERING BY CLIQUES       | from: PARTITION INTO CLIQUES
  GT18 | COVERING BY COMPLETE BIPARTITE SUBGRAPHS | from: PARTITION INTO CLIQUES

Do NOT write files for GT1, GT2, GT3, GT4, GT7, GT8, GT11 — those already exist.

Step 3: For each entry, create a JSON file:
  Path: /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json
  where <from> and <to> are lowercase, no spaces (e.g. R_minimalmaximalmatching_achromaticnumber.json)

  Fields:
    id: "R_<from>_<to>"
    from_problem: exact text from book's "Transformation from:" line
    to_problem: exact problem name from book header
    source_location: e.g. "A1.1 GT5"
    text: full verbatim text of the appendix entry (INSTANCE, QUESTION, Reference, Comment)

Rules:
- Verbatim only. No paraphrasing.
- If "Transformation from:" lists multiple sources, use the primary one.
- Skip figure images. Include figure captions if they appear as inline text.
- Use the Write tool to create each file.

Return a list of files created.
```

---

## Task 2 — GT A1.2 missing (PDF p.205–210, book p.194–199)

**Subagent prompt:**
```
Extract verbatim text from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 205–210 (book pp.194–199, section A1.2 Subgraphs and Supergraphs).

Step 2: Extract these MISSING entries:
  GT21 | INDUCED SUBGRAPH WITH PROPERTY PI (*)         | from: 3SAT
  GT22 | INDUCED CONNECTED SUBGRAPH WITH PROPERTY PI (*) | from: 3SAT
  GT23 | INDUCED PATH                                   | from: 3SAT
  GT25 | BIPARTITE SUBGRAPH                             | from: MAXIMUM 2-SATISFIABILITY
  GT26 | DEGREE-BOUNDED CONNECTED SUBGRAPH              | from: HAMILTONIAN PATH
  GT27 | PLANAR SUBGRAPH                                | from: HAMILTONIAN PATH
  GT28 | EDGE-SUBGRAPH                                  | from: 3SAT
  GT29 | TRANSITIVE SUBGRAPH                            | from: BIPARTITE SUBGRAPH
  GT30 | UNICONNECTED SUBGRAPH                          | from: VERTEX COVER

Do NOT write files for GT19, GT20, GT24 — those already exist.

Step 3: For each entry, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json

  Same 5-field schema and verbatim rules as described.
  source_location: e.g. "A1.2 GT21"

Return a list of files created.
```

---

## Task 3 — GT A1.2–A1.3 missing (PDF p.209–214, book p.198–203)

**Subagent prompt:**
```
Extract verbatim text from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 209–214 (book pp.198–203, sections A1.2 continued + A1.3 Vertex Ordering).

Step 2: Extract these MISSING entries:
  GT31 | MINIMUM K-CONNECTED SUBGRAPH       | from: HAMILTONIAN CIRCUIT
  GT32 | CUBIC SUBGRAPH                     | from: GRAPH 3-COLORABILITY
  GT34 | HAMILTONIAN COMPLETION             | from: HAMILTONIAN CIRCUIT
  GT35 | INTERVAL GRAPH COMPLETION          | from: OPTIMAL LINEAR ARRANGEMENT
  GT36 | PATH GRAPH COMPLETION              | from: INTERVAL GRAPH COMPLETION
  GT38 | DIRECTED HAMILTONIAN CIRCUIT       | from: VERTEX COVER
  GT39 | HAMILTONIAN PATH                   | from: VERTEX COVER
  GT40 | BANDWIDTH                          | from: 3-PARTITION
  GT41 | DIRECTED BANDWIDTH                 | from: 3-PARTITION
  GT42 | OPTIMAL LINEAR ARRANGEMENT         | from: SIMPLE MAX CUT
  GT43 | DIRECTED OPTIMAL LINEAR ARRANGEMENT| from: OPTIMAL LINEAR ARRANGEMENT
  GT44 | MINIMUM CUT LINEAR ARRANGEMENT     | from: SIMPLE MAX CUT
  GT45 | ROOTED TREE ARRANGEMENT            | from: OPTIMAL LINEAR ARRANGEMENT
  GT46 | DIRECTED ELIMINATION ORDERING      | from: 3SAT
  GT47 | ELIMINATION DEGREE SEQUENCE        | from: EXACT COVER BY 3-SETS

Do NOT write files for GT33, GT37 — those already exist.

Note: GT38 and GT39 are different from existing files R09 (HC→DHC) and R08 (HC→HP). The appendix GT38/GT39 entries reference VC→DHC and VC→HP.

Step 3: For each entry, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json

  Same 5-field schema and verbatim rules.
  source_location: e.g. "A1.3 GT38"

Return a list of files created.
```

---

## Task 4 — GT A1.4–A1.5 missing (PDF p.213–217, book p.202–206)

**Subagent prompt:**
```
Extract verbatim text from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 213–217 (book pp.202–206, sections A1.4 Morphisms + A1.5 Miscellaneous).

Step 2: Extract these MISSING entries:
  GT49 | LARGEST COMMON SUBGRAPH           | from: CLIQUE
  GT50 | MAXIMUM SUBGRAPH MATCHING         | from: CLIQUE
  GT51 | GRAPH CONTRACTABILITY             | from: 3SAT
  GT52 | GRAPH HOMOMORPHISM                | from: GRAPH K-COLORABILITY
  GT53 | DIGRAPH D-MORPHISM                | from: GRAPH GRUNDY NUMBERING
  GT54 | PATH WITH FORBIDDEN PAIRS         | from: 3SAT
  GT55 | MULTIPLE CHOICE MATCHING          | from: 3SAT
  GT56 | GRAPH GRUNDY NUMBERING            | from: 3SAT
  GT57 | KERNEL                            | from: 3SAT
  GT58 | K-CLOSURE                         | from: CLIQUE
  GT59 | INTERSECTION GRAPH BASIS          | from: COVERING BY CLIQUES
  GT60 | PATH DISTINGUISHERS               | from: VERTEX COVER
  GT61 | METRIC DIMENSION                  | from: 3DM
  GT62 | NESETRIL-RODL DIMENSION           | from: GRAPH 3-COLORABILITY
  GT63 | THRESHOLD NUMBER                  | from: INDEPENDENT SET
  GT64 | ORIENTED DIAMETER                 | from: SET SPLITTING
  GT65 | WEIGHTED DIAMETER                 | from: 3-PARTITION

Do NOT write files for GT48 — that already exists (R13).

Step 3: For each entry, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json

  Same 5-field schema and verbatim rules.
  source_location: e.g. "A1.4 GT49"

Return a list of files created.
```

---

## Task 5 — ND and SP gap check (PDF p.206–237, book p.195–226)

**Subagent prompt:**
```
Audit and fill gaps in the A2 Network Design and A3 Sets & Partitions sections.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf
R-files: /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/

Step 1: List all existing R-files in the directory. Note which ones cover ND and SP entries (filenames contain network/spanning/flow/steiner/traveling/path/knapsack/subset/partition/set/bin etc.).

Step 2: Read PDF pages 206–217 (ND1–ND27, book pp.195–206). Then read pages 218–237 (ND28–ND51 + SP section, book pp.207–226).
  Known ND entries: ND1–ND51 (51 total).
  Known SP entries: SP1–SP21 roughly (consult what you find in the PDF).

Step 3: For each ND and SP entry in the PDF:
  - Check if a matching R-file already exists (look at existing filenames for a match on to_problem)
  - If no match found, create a new R_ file with verbatim text

  Path: /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json
  Same 5-field schema. source_location: e.g. "A2.1 ND5" or "A3 SP7"

Step 4: Return a summary of:
  - Total ND entries found in PDF
  - How many already have R-files
  - How many new R_ files were created (list them)
  - Same for SP entries
```

---

## Task 6 — AL7–AL21 missing (PDF p.277–282, book p.266–271)

**Subagent prompt:**
```
Extract verbatim text from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 277–282 (book pp.266–271, section A10 Automata AL7–AL21).

Step 2: Extract these MISSING entries (AL1–AL6 already exist as R216–R221, AL9 exists as R224):
  AL7  | REDUCTION OF INCOMPLETELY SPECIFIED AUTOMATA | (find "Transformation from:" in book)
  AL8  | MINIMUM INFERRED FINITE STATE AUTOMATON      | from: MONOTONE 3SAT
  AL10 | MINIMUM INFERRED REGULAR EXPRESSION         | from: 3SAT
  AL11 | REYNOLDS COVERING FOR CONTEXT-FREE GRAMMARS | from: 3SAT
  AL12 | COVERING FOR LINEAR GRAMMARS                | from: REGULAR EXPRESSION NON-UNIVERSALITY
  AL13 | STRUCTURAL INEQUIVALENCE FOR LINEAR GRAMMARS| from: REGULAR EXPRESSION NON-UNIVERSALITY
  AL14 | REGULAR GRAMMAR INEQUIVALENCE               | from: FINITE STATE AUTOMATON INEQUIVALENCE
  AL15 | NON-LR(K) CONTEXT-FREE GRAMMAR              | from: generic
  AL16 | ETOL GRAMMAR NON-EMPTINESS                  | from: REGULAR EXPRESSION NON-UNIVERSALITY
  AL17 | CONTEXT-FREE PROGRAMMED LANGUAGE MEMBERSHIP | from: 3SAT
  AL18 | QUASI-REAL-TIME LANGUAGE MEMBERSHIP         | from: 3SAT
  AL19 | ETOL LANGUAGE MEMBERSHIP                    | from: 3SAT
  AL20 | CONTEXT-SENSITIVE LANGUAGE MEMBERSHIP       | from: LINEAR BOUNDED AUTOMATON ACCEPTANCE
  AL21 | TREE TRANSDUCER LANGUAGE MEMBERSHIP         | from: generic

Step 3: For each entry, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json

  Same 5-field schema and verbatim rules.
  source_location: e.g. "A10 AL7"

Return a list of files created.
```

---

## Task 7 — PO2–PO20 missing (PDF p.283–289, book p.272–278)

**Subagent prompt:**
```
Extract verbatim text from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 283–289 (book pp.272–278, section A11 Program Optimization PO1–PO20).

Step 2: Extract these MISSING entries (PO1 exists as R225, PO3 exists as R226, PO9 exists as R18):
  PO2  | FEASIBLE REGISTER ASSIGNMENT             | from: 3SAT
  PO4  | CODE GENERATION ON A ONE-REGISTER MACHINE| from: 3SAT
  PO5  | CODE GENERATION WITH UNLIMITED REGISTERS | from: FEEDBACK VERTEX SET
  PO6  | CODE GENERATION FOR PARALLEL ASSIGNMENTS | from: FEEDBACK VERTEX SET
  PO7  | CODE GENERATION WITH ADDRESS EXPRESSIONS | from: 3SAT
  PO8  | CODE GENERATION WITH UNFIXED VARIABLE LOCATIONS | from: 3SAT
  PO10 | MICROCODE BIT OPTIMIZATION               | from: 3DM
  PO11 | INEQUIVALENCE OF PROGRAMS WITH ARRAYS    | from: 3SAT
  PO12 | INEQUIVALENCE OF PROGRAMS WITH ASSIGNMENTS | from: 3SAT
  PO13 | INEQUIVALENCE OF FINITE MEMORY PROGRAMS  | from: LINEAR BOUNDED AUTOMATON ACCEPTANCE
  PO14 | INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING | from: 3SAT
  PO15 | INEQUIVALENCE OF SIMPLE FUNCTIONS        | from: INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING
  PO16 | STRONG INEQUIVALENCE OF IANOV SCHEMES    | from: 3SAT
  PO17 | STRONG INEQUIVALENCE FOR MONADIC RECURSION SCHEMES | from: STRONG INEQUIVALENCE OF IANOV SCHEMES
  PO18 | NON-CONTAINMENT FOR FREE B-SCHEMES       | from: 3SAT
  PO19 | NON-FREEDOM FOR LOOP-FREE PROGRAM SCHEMES| from: 3SAT
  PO20 | PROGRAMS WITH FORMALLY RECURSIVE PROCEDURES | from: 3SAT

Also fix R226 (PO3): its `from_problem` field says "GRAPH 3-COLORABILITY" but the book says "permutation generation" — overwrite it with the correct `from_problem` using the verbatim book text.

Step 3: For each entry, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json

  Same 5-field schema and verbatim rules.
  source_location: e.g. "A11 PO2"

Return a list of files created and the R226 fix.
```

---

## Task 8 — MS2–MS19 missing + MS1 fix (PDF p.290–295, book p.279–284)

**Subagent prompt:**
```
Extract verbatim text from a book PDF into JSON files.

PDF: /Users/xiweipan/Codes/problem-reductions/Computers and intractability  a guide to the theory of NP-completeness.pdf

Step 1: Read PDF pages 290–295 (book pp.279–284, section A12 Miscellaneous MS1–MS19).

Step 2: Fix R229 first: read /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R229_3sat_betweenness.json
  The `from_problem` field says "3SAT" but the book says "Transformation from SET SPLITTING". Overwrite with correct from_problem and verbatim text.

Step 3: Extract these MISSING entries (MS1 exists as R229):
  MS2  | CYCLIC ORDERING             | from: 3SAT
  MS3  | NON-LIVENESS OF FREE CHOICE PETRI NETS | from: 3SAT
  MS4  | REACHABILITY FOR 1-CONSERVATIVE PETRI NETS | from: LINEAR BOUNDED AUTOMATON ACCEPTANCE
  MS5  | FINITE FUNCTION GENERATION  | from: FINITE STATE AUTOMATA INTERSECTION
  MS6  | PERMUTATION GENERATION      | from: X3C
  MS7  | DECODING OF LINEAR CODES    | from: 3DM
  MS8  | SHAPLEY-SHUBIK VOTING POWER | from: PARTITION
  MS9  | CLUSTERING                  | from: GRAPH 3-COLORABILITY
  MS10 | RANDOMIZATION TEST FOR MATCHED PAIRS | from: PARTITION
  MS11 | MAXIMUM LIKELIHOOD RANKING  | from: FEEDBACK ARC SET
  MS12 | MATRIX DOMINATION           | from: MINIMUM MAXIMAL MATCHING
  MS13 | MATRIX COVER                | from: MAX CUT
  MS14 | SIMPLY DEVIATED DISJUNCTION | from: MAX CUT
  MS15 | DECISION TREE               | from: X3C
  MS16 | MINIMUM WEIGHT AND/OR GRAPH SOLUTION | from: X3C
  MS17 | FAULT DETECTION IN LOGIC CIRCUITS | from: 3SAT
  MS18 | FAULT DETECTION IN DIRECTED GRAPHS | from: X3C
  MS19 | FAULT DETECTION WITH TEST POINTS | from: X3C

Step 4: For each entry, create:
  /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/R_<from>_<to>.json

  Same 5-field schema and verbatim rules.
  source_location: e.g. "A12 MS2"

Return a list of files created and the R229 fix.
```

---

## Task 9 — ID Consolidation

After Tasks 0–8 complete, assign sequential IDs to all `R_*.json` files.

**Step 1:** List all files matching `R_*.json` in `references/Garey&Johnson/reductions/`. Sort them by section order (GT first, then ND, SP, SR, SS, MP, AN, GP, LO, AL, PO, MS).

**Step 2:** Determine the next available ID after the current maximum. The current highest is R229 (plus R230+ from Tasks 1–8). Assign sequentially starting from R230.

**Step 3:** For each `R_*.json` file:
- Assign new ID (e.g., R230, R231, ...)
- Rename the file to `R{N}_{from}_{to}.json` (keeping the descriptive suffix)
- Update the `id` field inside the JSON

Use Python/Bash to do the renaming:
```bash
cd /Users/xiweipan/Codes/problem-reductions/references/Garey&Johnson/reductions/
ls R_*.json | sort  # verify the list
```

**Step 4:** Verify no duplicate IDs exist:
```bash
ls R[0-9]*.json | sed 's/_.*$//' | sort | uniq -d
```
Expected: no output (no duplicates).

**Step 5:** Commit:
```bash
git add references/Garey&Johnson/
git commit -m "feat: fill missing appendix R-files (GT, AL, PO, MS, ND, SP gaps)"
```

---

## After All Tasks Complete

Spot-check 5 files across different sections:
1. One GT entry — compare `text` against PDF
2. One AL entry — compare `text` against PDF
3. One PO entry — compare `text` against PDF
4. One MS entry — compare `text` against PDF
5. One newly-assigned sequential ID — verify rename was correct

Total expected new R-files: ~112 (51 GT + 6 ND + 6 SP + 14 AL + 17 PO + 18 MS)
