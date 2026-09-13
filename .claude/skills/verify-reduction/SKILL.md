---
name: verify-reduction
description: Verify a reduction mathematically using a Typst proof and independent constructor/adversary scripts, with coverage chosen from the construction's risks. Report findings without committing artifacts.
---

# Verify Reduction

Verify a reduction before implementation, standalone or as the default mathematical
verification step of `/add-rule`. Produce a proof and independent executable
checks in a temporary directory. Report what was established and any limitations;
finite checks support the proof but do not replace it.

## Invocation

```
/verify-reduction 868
/verify-reduction SubsetSum Partition
```

## Step 1: Read the Definition and Resolve the API

For an issue, read it with `gh issue view <number> --json title,body`. Inspect both
concrete models with `pred show <Problem> --json` and read their implementations.
Extract the construction, mathematical domain, correctness argument, witness
mapping, parameter formulas, worked example, and references. Consult the cited
literature when needed to resolve a mathematical claim.

Read the canonical [witness/aggregate contract](../../../docs/src/design.md#witness-and-aggregate-reductions),
[arithmetic policy](../../../docs/src/design.md#arithmetic), and
[validation policy](../../../docs/src/design.md#validation-evidence).

Locate `Solution` and `Value` definitions with `rg`, substitute concrete generic
arguments, and follow associated types to their implementations. Record the
resolved types and source evidence. If resolution remains unclear, use a temporary
compile-backed Rust probe with a path dependency on the repository. Do not infer
Rust types from problem names, unit-weight terminology, or Python integers.

Check the actual operation:

- A witness reduction maps solutions and justifies feasibility and, where claimed,
  optimality preservation. Different objective directions or Rust value types are
  not automatic failures. For example, complementing an independent set of size
  `k` gives a vertex cover of size `n-k` and reverses optimization direction.
- An aggregate reduction must justify its actual value conversion. Check the
  domain of arithmetic the construction or mapping performs, not a hypothetical
  conversion between all source and target objective values.
- A multi-query algorithm needs the existing Turing capability. An arbitrary
  feasibility witness does not establish a source optimum without an argument.

Report a concrete mathematical/API mismatch before implementation if one exists.
Do not replace that analysis with a wrapper-pair whitelist or backend range gate.

## Step 2: Write the Proof

Write a standalone Typst proof in the temporary directory containing:

- Source/target definitions and the precise applicability domain.
- Construction steps with symbols defined before use.
- Independent forward and reverse correctness arguments. For optimization,
  state the objective relationship and why target optima yield source optima.
- Witness extraction, including its mathematical preconditions.
- Target parameter formulas, distinguishing equalities from upper bounds.
- Small worked examples that exercise the construction. Include YES and NO
  examples where both exist; for always-feasible optimization problems, show
  the relevant objective relationship instead of inventing an infeasible case.

Use enough detail to make the argument checkable. Do not substitute phrases such
as “obviously” or “the converse is similar” for a missing proof. Example size is
chosen for clarity and coverage, not a minimum vertex count.

## Step 3: Implement Constructor Checks

Write a temporary Python script with independent source/target feasibility and
objective oracles. Cover the claims relevant to this construction:

| Claim | Evidence |
|-------|----------|
| Forward/reverse correctness | Small exhaustive instances or justified sampling; compare feasibility and the stated optimum relationship |
| Witness extraction | Target witnesses satisfying the mapping's preconditions produce valid source witnesses; check optimal mappings where claimed |
| Parameter formulas | Measure constructed targets and compare with equalities or upper bounds; use symbolic checking when it adds evidence |
| Target structure | Check the actual target invariants and gadget interactions |
| Worked examples | Reproduce the proof's values and witnesses |
| Arithmetic/case splits | Exercise concrete branches and representation risks in the construction |

Choose exhaustive bounds and sampling from the construction's risks and cost.
Record bounds, seeds, counts, and omissions so the evidence is reproducible.
There is no universal minimum generated-check count. Do not duplicate solver
precision tests or require a backend to establish mathematical equivalence.
Python's arbitrary-precision arithmetic is not evidence that Rust construction
arithmetic cannot overflow; inspect the actual stored representation separately.

## Step 4: Run Checks and Analyze Gaps

Run the script and investigate failures. Correct the proof, construction, or
checker according to the evidence, then rerun affected checks. Map each proof
claim to its executable evidence or explain why it is established by proof alone.
Report untested areas rather than increasing check counts without new coverage.

If a backend integration run is included, identify it separately. Record whether
failure occurs in construction, solving, extraction, or source validation. A
backend timeout, numerical rejection, or non-optimal termination is not itself a
counterexample to the reduction theorem and must not be reported as a pass.

## Step 5: Independent Adversary Verification

Dispatch an independent subagent with the problem definitions and Typst proof,
without the constructor script. Ask it to implement its own construction,
extraction, feasibility, and objective checks. It must not import the constructor
implementation. Have it challenge the proof's actual risks:

- Complement/identity mappings: objective direction and witness correspondence.
- Algebraic mappings: case boundaries, coefficients, and extraction per case.
- Gadget mappings: unintended paths, gadget interactions, and target invariants.

Use exhaustive checks or property-based strategies where they provide useful
independent coverage, not to satisfy a count. Reproduce applicable worked examples.
Compare both implementations on shared instances. Investigate disagreements;
structurally different but equivalent encodings may be valid. One checker passing
does not establish that the other checker is at fault.

## Step 6: Review and Report

Before reporting, confirm:

- The concrete Rust types and actual witness/aggregate contract were checked.
- The proof covers construction, both directions, extraction, and parameters.
- Independent checks exercise relevant branches and mappings with reproducible
  bounds/seeds; remaining gaps are stated.
- Disagreements and failures are resolved or explicitly reported.
- Mathematical evidence and backend integration results are distinguished.

Report:

```text
VERIFICATION RESULT: VERIFIED / FAILED / INCOMPLETE
  Source and target: <concrete variants>
  Mathematical claim and applicability domain: <summary>
  Constructor coverage: <bounds, cases, counts>
  Independent coverage: <bounds, cases, counts>
  Cross-comparison: <result>
  Remaining gaps or counterexamples: <details or none>
  Backend integration, if run: <separate result>
```

Use VERIFIED only when the proof and independent checks support the stated claim;
use FAILED for an established defect and INCOMPLETE for unresolved evidence.
When called by `/add-rule`, provide the checked construction, extraction, and
examples for the Rust implementation. Keep proof/scripts/results temporary; do
not commit generated verification artifacts.

## Reduction lifecycle responsibilities

Apply the canonical [executed lifecycle](../../../docs/src/design.md#executed-reduction-lifecycle).
State the rule's instance domain, qualifying-witness premise, source guarantee,
and infeasibility interpretation. Check every qualifying tied optimum in small
exhaustive cases where ties are relevant. A witness flag alone does not prove
complete solvability or that adjacent path premises compose.

Construct each executed result once and share target, witness, value, and
completion state. Outcome interpretation uses the rule's mathematical relation;
ordinary extraction assumes its premises. Keep necessary dynamic/JSON conversion
and reachable representation failures, but no checked/unchecked extraction or
pure forwarding wrappers. Do not add `SolutionAggregate` bounds to models or
mathematical mappings; it belongs to brute-force witness selection.
