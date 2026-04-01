# Verification Methodology for Reduction Rule Proofs

This document describes the multi-layer verification approach used to validate
the reduction rules in `proposed-reductions.typ`. The methodology is designed
to be reproducible by future contributors and applicable to new reductions.

## Overview

Each reduction rule is verified at three independent layers:

| Layer | Tool | What it catches | Confidence level |
|-------|------|----------------|-----------------|
| **Mathematical proof** | Typst PDF | Logical gaps, wrong case analysis | High (if reviewer is careful) |
| **Computational verification** | Python scripts | Wrong formulas, off-by-one errors, construction bugs | Very high (exhaustive for small n) |
| **Machine-checked algebra** | Lean 4 + Mathlib | Arithmetic errors, identity mistakes | Absolute (for proved lemmas) |

No single layer is sufficient. The Python scripts caught 2 bugs that survived
careful mathematical proofs (X3C→AP quotient cycles, VC→HC edge count). The
Lean proofs verify identities that Python checks only numerically.

## Layer 1: Mathematical Proofs (Typst)

### Structure per reduction

Every reduction in `proposed-reductions.typ` follows the same template:

1. **Theorem statement** — 1-3 sentence intuition with citation
2. **Proof** with exactly three subsections:
   - *Construction:* numbered algorithm steps, all symbols defined before use
   - *Correctness:* bidirectional (⟹ and ⟸), each direction a separate paragraph
   - *Solution extraction:* how to map target solution back to source
3. **Overhead table** — target size fields as functions of source size fields
4. **Worked example** — concrete small instance with full numerical verification

### Quality criteria

- No hand-waving ("clearly", "obviously", "it is easy to see")
- No scratch work or failed attempts visible
- Every claim backed by algebraic computation or structural argument
- Bidirectional proofs must be genuinely independent (not "the converse is similar")

### Common failure modes caught by this layer

- Missing cases in case analysis (e.g., Σ = 2T, Σ > 2T, Σ < 2T)
- Incorrect direction of inequalities
- Forgetting to handle infeasible instances
- Wrong overhead formulas

## Layer 2: Computational Verification (Python)

### Design principles

Each reduction gets its own `verify_<source>_<target>.py` script that:

1. **Defines the reduction construction** as a Python function
2. **Checks the forward direction**: source has solution → target has solution
3. **Checks the backward direction**: target has solution → extracted source solution is valid
4. **Verifies overhead formulas**: compare formula output vs actual constructed sizes
5. **Verifies structural properties**: girth, cycle counts, graph connectivity, etc.

### Verification strategies by reduction type

#### Identity/trivial reductions (DS → Multicenter)

- **Exhaustive enumeration**: test ALL graphs up to n=6
- For each graph and each parameter K: check the equivalence holds
- This is feasible because the source and target are the same graph

#### Algebraic reductions (SubsetSum → Partition, MaxCut → OLA)

- **Symbolic verification** (sympy): verify key identities for general n
- **Exhaustive enumeration**: all instances up to n=6, all parameter values
- **Solution extraction**: verify the extracted solution actually solves the source

#### Gadget-based reductions (VC → HC, VC → PFES)

- **Construction verification**: build the gadget graph, verify vertex/edge counts
- **Structural properties**: girth, widget internal edge count, pendant degrees
- **Forward/backward on small instances**: limited by exponential cost of HC/HP check
- **Formula verification on larger instances**: verify counts match formulas even when
  we can't afford to check HC/HP

#### Composition reductions (VC → HP)

- **Test each step independently**: verify HC → HP transformation on many graphs
- **End-to-end on small instances**: VC → HC → HP where affordable
- **Arithmetic verification**: vertex/edge count formulas for the composed construction

### Test count targets

| Category | Target | Rationale |
|----------|--------|-----------|
| Exhaustive (all graphs n≤6) | 10,000+ | Covers all small cases |
| Algebraic (all instances n≤6) | 20,000+ | Multiple parameters per instance |
| Gadget construction | 1,000+ | Formula checks on all graphs, HC/HP on small ones |
| Structural (girth, dominance) | 100+ | Per-graph structural property checks |

### Running the verification suite

```bash
# Run all scripts
for f in docs/paper/verify-reductions/verify_*.py; do
    echo "=== $(basename $f) ==="
    python3 "$f"
    echo
done

# Run a single script
python3 docs/paper/verify-reductions/verify_subsetsum_partition.py
```

### Dependencies

- Python 3.8+
- `sympy` — symbolic algebra for identity verification
- `networkx` — graph construction, girth computation, connectivity checks
- No other dependencies (standard library only otherwise)

Install: `pip install sympy networkx`

### Common failure modes caught by this layer

- **Construction bugs**: wrong edge connections, missing vertices
- **Formula errors**: edge count overcounts for isolated vertices (caught for VC→HC)
- **Fundamental design flaws**: quotient-graph cycles from 2-cycle encoding (caught for X3C→AP)
- **Off-by-one errors**: in padding elements, vertex indices, etc.

## Layer 3: Machine-Checked Proofs (Lean 4)

### What we formalize

The Lean proofs target **algebraic identities and structural invariants** that
the reductions depend on:

| Category | Example | Tactic |
|----------|---------|--------|
| Arithmetic identities | $14m + (2m-n) + 2nK = 16m - n + 2nK$ | `omega` |
| Sum formulas | $L_{K_n} = n(n^2-1)/6$ for $n \leq 12$ | `native_decide` |
| Graph lattice | $G \sqcup G^c = \top$, $G \sqcap G^c = \bot$ | `sup_compl_eq_top` |
| Edge-set decomposition | $\sum_{e \in E_1} w(e) + \sum_{e \in E_2} w(e) = \sum_{e \in E_1 \cup E_2} w(e)$ | `Finset.sum_union` |
| Padding algebra | $T + (\Sigma - 2T) = \Sigma - T$ | `omega` |

### What we don't formalize (and why)

| Argument | Difficulty | Mathlib status |
|----------|-----------|---------------|
| Widget traversal patterns | Very hard | No Hamiltonian path enumeration |
| Quotient-graph acyclicity | Very hard | No DAG quotient formalization |
| Girth ≥ 6 for specific graph | Hard | Possible but requires building the specific graph in Lean |
| Consecutive placement in OLA→RTA | Hard | Needs arrangement cost formalization |

These are verified computationally (Python) instead.

### Building the Lean proofs

```bash
cd docs/paper/verify-reductions/lean
export PATH="$HOME/.elan/bin:$PATH"

# First time: install Lean and fetch Mathlib
curl -sSf https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh | sh -s -- -y
lake update  # downloads Mathlib + cached oleans

# Build
lake build

# Run
lake exe reductionproofs
```

### Lean proof status

- **Proved (no sorry)**: complement partition, arithmetic identities, L_{K_n} formula,
  padding algebra, edge/vertex counts
- **Admitted (1 sorry)**: SubsetSum ↔ Partition list-level equivalence (needs
  List.zip/filter/sum decomposition — tedious but not a structural gap)

## Methodology for Adding a New Reduction

When adding a new reduction rule, follow this process:

### Step 1: Write the mathematical proof (Typst)

Add a new section to `proposed-reductions.typ` with:
- Theorem statement (with citation)
- Construction (numbered steps)
- Correctness (⟹ and ⟸)
- Solution extraction
- Overhead table
- Worked example

Compile: `python3 -c "import typst; typst.compile('docs/paper/proposed-reductions.typ', output='docs/paper/proposed-reductions.pdf', root='.')"`

### Step 2: Write computational verification (Python)

Create `verify-reductions/verify_<source>_<target>.py`:

```python
#!/usr/bin/env python3
"""§X.Y Source → Target: exhaustive + structural verification."""
import itertools, sys

def reduce(source_instance):
    """Apply the reduction construction."""
    ...
    return target_instance

def extract_solution(target_solution, reduction_data):
    """Extract source solution from target solution."""
    ...
    return source_solution

def main():
    passed = failed = 0

    # 1. Symbolic checks (sympy) for key identities
    ...

    # 2. Exhaustive forward/backward for n ≤ 6
    for n in range(2, 7):
        for instance in all_instances(n):
            source_feasible = check_source(instance)
            target = reduce(instance)
            target_feasible = check_target(target)
            assert source_feasible == target_feasible  # forward + backward

    # 3. Solution extraction verification
    ...

    # 4. Overhead formula verification
    ...

    # 5. Structural properties (girth, connectivity, etc.)
    ...

    print(f"Source → Target: {passed} passed, {failed} failed")
    return 1 if failed else 0

if __name__ == "__main__":
    sys.exit(main())
```

### Step 3: Add Lean lemma (if applicable)

Add to `lean/ReductionProofs/Basic.lean`:
- Arithmetic identities used in the proof
- Structural invariants (e.g., edge-set decomposition)
- Build: `cd lean && lake build`

### Step 4: Run full suite

```bash
for f in docs/paper/verify-reductions/verify_*.py; do
    python3 "$f" || echo "FAILED: $f"
done
```

## File Listing

```
docs/paper/
├── proposed-reductions.typ          # Mathematical proofs (19 pages)
├── proposed-reductions.pdf          # Compiled PDF
└── verify-reductions/
    ├── METHODOLOGY.md               # This document
    ├── verify_all.py                # Original monolithic script (legacy)
    ├── verify_subsetsum_partition.py # §2.1 — 32,580 checks
    ├── verify_vc_hc.py              # §2.2 — 11,358 checks
    ├── verify_vc_hp.py              # §2.3 — 118+ checks
    ├── verify_maxcut_ola.py         # §3.1 — 21,520 checks
    ├── verify_ola_rta.py            # §3.2 — 120+ checks
    ├── verify_ds_multicenter.py     # §4.1/4.2 — 23,922 checks
    ├── verify_x3c_ap.py            # §4.3 — expected failures (known bug)
    ├── verify_vc_pfes.py            # §5.1 — 147+ checks
    └── lean/
        ├── lakefile.toml            # Lean project config (requires Mathlib)
        ├── lean-toolchain           # Lean 4.29.0
        ├── ReductionProofs.lean     # Module root
        ├── ReductionProofs/
        │   └── Basic.lean           # All Lean proofs
        └── Main.lean                # Entry point
```

## Lessons Learned

1. **Python verification catches bugs that proofs miss.** The X3C→AP construction
   survived careful proof-writing but failed on the very first test instance.
   The VC→HC edge count formula was wrong for isolated vertices — a case the
   proof hand-waved with "since Σ d(v) = 2m."

2. **Lean proofs for arithmetic are trivially true.** `14m + (2m-n) + 2nK = 16m - n + 2nK`
   is proved by `omega` — this adds no real confidence. The structural arguments
   (widget traversals, girth bounds) are where Lean could add value but are
   hardest to formalize.

3. **The graph lattice structure in Mathlib is powerful.** `G ⊔ Gᶜ = ⊤` via
   `sup_compl_eq_top` proves the complement identity in one line — this is a
   genuinely meaningful machine-checked proof.

4. **Exhaustive verification on small instances (n ≤ 6) catches most bugs.**
   In our experience, if a construction is wrong, it fails on n = 3 or n = 4.
   Testing up to n = 6 provides very high confidence.

5. **Mark failures honestly.** The X3C→AP entry is marked OPEN in red in the PDF
   with a clear explanation of the failure mode. This is more valuable than a
   wrong proof.
