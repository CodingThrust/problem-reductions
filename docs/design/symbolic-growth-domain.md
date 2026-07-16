# Symbolic Growth Domain & Pareto Path Search — Product Design

Status: approved design, ready for decomposition into issues.
Origin: issue #1069 (`pred path --all` OOMs/hangs in `big_o_normal_form`). The acute
symptom is already mitigated on `main` by a stopgap: `MAX_CANONICAL_TERMS = 50_000`
in `canonical.rs` aborts oversized expansions, and the CLI falls back to printing the
*unreduced* composed expression as `O(<raw expr>)` on failure
(`problemreductions-cli/src/commands/graph.rs:349`). This design replaces
refuse-or-bluff with a system that answers.

## Need

The symbolic overhead system conflates exact expressions with asymptotic queries:
`big_o_normal_form` (src/big_o.rs) fully expands composed path overheads to monomial
normal form (src/canonical.rs) before projecting to Big-O. Expansion of nested
`(sum)^2 * (sum)^2` structures is exponential in nesting depth — the root cause of
issue #1069. The stopgap cap prevents the OOM but leaves three structural defects:

1. **Refuse-or-bluff answers.** Paths whose composed overhead exceeds the expansion
   cap get no normalized Big-O; the CLI falls back to printing the raw unreduced
   expression disguised as `O(...)`. The exponential-expansion algorithm is still
   there, merely fenced.
2. **Heuristic dominance.** Asymptotic comparison relies on a foolable two-point
   numerical sampling heuristic (`numerical_dominance_check`) — e.g. `n^100` vs
   `1.001^n` is decided wrongly because the crossover lies beyond the sampled range.
3. **Unsound search.** The scalar Dijkstra in `ReductionGraph::find_cheapest_path`
   has a latent correctness hole: edge costs depend on the size accumulated along the
   path, which violates Dijkstra's assumptions — a cheaper-so-far path with a larger
   intermediate size can be wrongly preferred. And there is no instance-free
   (asymptotic) search mode at all.

We need a **trustworthy** (explicit semantic axioms, bounded termination, per-rule
verifiability) and **extensible** (new functions/variables without touching the core)
symbolic system: an exact `Expr` layer separated from an asymptotic growth domain,
with both Big-O rendering and path search running in the asymptotic domain at
polynomial cost. Occam's razor is a hard constraint: no new entities beyond what the
selected features require.

**Users:** library maintainers adding models/rules; CLI/MCP consumers of
`pred path` / `find_path`; the Typst paper's auto-derivation pipeline.

**Success criteria** (the stopgap already prevents OOM; these measure what the
principled system adds):
- **Answers, not refusals:** every enumerable path gets a genuine normalized Big-O.
  The `MAX_CANONICAL_TERMS` bail-out and the `O(<raw expr>)` CLI fallback are
  deleted; the only remaining "cannot normalize" sources are nonlinear exponents
  and factorials, rendered as an explicit annotation (the one `2^num_vertices`
  overhead edge gets a real exponential bound via the linear `exp` field).
  Regression: issue #1069's exploding path (KSat → … → QuadraticAssignment → ILP →
  QUBO) asserts a real normalized Big-O, not an error or fallback.
- **Trustworthy comparison:** the numerical sampling heuristic is replaced by a
  symbolic decision procedure, property-tested against numeric evaluation.
- **Correct search:** Pareto label search fixes the path-dependent-cost hole and adds
  an instance-free asymptotic mode.
- Big-O for all enumerated paths across the whole reduction graph completes within a
  CI time budget (each test < 5 s per repo policy).
- Output is byte-identical across Linux/macOS (no inventory-order dependence).

**Constraints:**
- The `#[reduction]` macro and overhead declaration syntax stay unchanged (dozens of
  rule files untouched).
- Internal APIs and CLI output format may break (0.x semver).
- No new external dependencies.

## Prior art & landscape

Surveyed via four research passes (CAS systems; compiler symbolic-cost systems;
e-graph engines; asymptotics theory and formalization). Borrow-vs-build verdict:

| Candidate | Verdict | Why |
|---|---|---|
| Albert–Alonso–Arenas–Genaim–Puebla, *Asymptotic Resource Usage Bounds* (APLAS 2009) | **Adopt as spec** | Published normal form (sums of products of `2^(r·A)`, `A^r`, `log A`) with a soundness theorem `e ∈ Θ(asymp(e))` — our correctness contract |
| SageMath `AsymptoticRing` / growth groups | **Borrow the design, not the code** | GPL; the core (exponent-vector arithmetic + poset of summands with O-term absorption) is small enough to reimplement cleanly |
| KoAT weakly-monotone bound grammar (Brockschmidt et al., TOPLAS 2016) | **Adopt as axiom** | Weak monotonicity ⇒ composition-by-substitution is sound ⇒ Pareto label search is correct (isotonicity) |
| LLVM SCEV / GCC chrec | **Adopt patterns** | Construction-time canonicalization, explicit budgets with graceful degradation, absorbing "don't know" sentinel (`SCEVCouldNotCompute`, `chrec_dont_know`) |
| Multivariate Big-O semantics: Howell (KSU TR 2007-4); Guéneau–Charguéraud–Pottier (ESOP 2018) | **Adopt definition** | Naive multivariate O is inconsistent (Howell Thm 2.3/2.4); the product-filter definition restricted to nonnegative weakly-monotone functions is the trustworthy one |
| McRAPTOR / OpenTripPlanner `ParetoSet` / nigiri `pareto_set.h`; Martins 1984; NAMOA* | **Adopt algorithm** | Per-node label bags (antichains) with dominance pruning are the industry and literature standard for partial-order path costs; enumerate-then-filter appears nowhere as a recommended method |
| ProblemReductions.jl `reduction_paths` | **Anti-pattern baseline** | `all_simple_paths` with no cost model, no ranking, no filter; survives only because its graph is tiny |
| egg / egglog e-graphs | **Dropped** | Directional normalization doesn't need equality saturation (Cranelift aegraph retrospective: mean e-class size 1.13); egglog API unstable |
| SymPy / GiNaC / Symbolica | **Concepts only** | Never auto-expand; deterministic total order on atoms; function-registry extensibility (deferred with F6) |

Nothing is directly reusable as a dependency; this is a build against published specs.

**Empirical inventory scan** (drives the grammar decision): registered overhead
expressions are overwhelmingly polynomial with subtraction and constant division.
Exceptions: one `log` factor (`ksatisfiability_*`: `(num_vars + num_clauses)^2 *
log(num_vars + num_clauses + 1)`), one genuine exponential
(`highlyconnecteddeletion_ilp.rs`: `num_vars = "2^num_vertices"`), and one
`sqrt((x)^2)` used as an absolute-value idiom. `declare_variants!` complexity strings
are heavily exponential, but they are consumed only by `pred list/show` display and
the dropped F8 — outside this design's data path.

## Features

Selected (rough, agentic-coding-adjusted estimates):

| # | Feature | Effort |
|---|---|---|
| F1 | Growth domain: `GrowthTerm`/`Growth` antichain, symbolic dominance, pruning, absorbing `Unknown`, caps with upward widening | ~2–3 days |
| F2 | Replace the `big_o.rs` pipeline with the growth domain; delete `canonical.rs`; issue-1069 regression + whole-graph CI budget tests | ~1–2 days |
| F3 | Pareto label search kernel replacing `dijkstra`, with two label domains: F3a asymptotic (`Growth` per size field) and F3b concrete instance (**measured**: execute reductions and apply post-construction measured budgets) | ~3–4 days |
| F12 | Per-edge overhead calibration test: canonical examples run through `reduce_to()`, measured sizes must not exceed formula predictions | ~0.5–1 day |
| F4 | CLI/MCP surface: Pareto-front output, deterministic ordering, `--json` no longer renders text | ~1–2 days |
| F5+F11 (merged support work, folded into F1/F3/F4) | Redundancy check (`find_dominated_rules`) rewired to the same dominance order; `Growth` serde + `Display` consumed by CLI JSON and paper export | ~1.5 days |

Total: ~10–14 days.

Deferred / dropped, with reasons:

- **F6 `Expr::Func(FuncKind)` registry** and **F7 shared parser crate** — deferred to a
  later milestone. Genuine extensibility improvements, but independent of this
  milestone's goal; the growth domain consumes `Expr` as-is.
- **F8 effective-complexity ranking** (target complexity ∘ overhead) — deferred until a
  concrete find-problem need; requires an exponential part in `GrowthTerm` (see
  Extensibility).
- **F9 convex-hull/AM-GM pruning** — deferred until antichain sizes measurably hurt;
  Pareto pruning suffices at current variable counts.
- **F10 egg-based display simplification** — dropped per survey (directional ruleset
  does not need equality saturation).

## Semantic foundation (normative)

These definitions and axioms are the trust contract; tests enforce them.

- **Definition (multivariate Big-O, product filter).** For size functions
  `f, g : ℕ_{≥2}^k → ℝ_{≥0}`: `g ∈ O(f)` iff `∃ c > 0, N` such that
  `g(x) ≤ c·f(x)` whenever **all** variables `x_i ≥ N`. (Howell's `O_∀`;
  Guéneau et al.'s product filter.)
- **Domain axioms.** Every expression admitted to the growth domain is nonnegative
  and weakly monotone (nondecreasing in each variable) on `vars ≥ 2`. Under these
  axioms Howell's inconsistencies vanish and `f + g ≍ max(f, g)` up to a constant
  factor, which licenses `add = antichain union + prune`.
- **Widening rules (always upward, i.e. toward a valid upper bound):**
  - Subtraction: `a − b ⇝ a + b` (sound since `b ≥ 0`; also covers the
    `sqrt((a−b)^2)` absolute-value idiom because `|a−b| ≤ a+b`).
  - Constant division and all multiplicative constants: dropped on entry.
  - Exponentials with **linear** exponents (`c^x`, `c^(r·x)`, `exp(x)`) are
    first-class (see M1's `exp` field). Nonlinear exponents (`2^(n*k)`,
    `2^sqrt(n)`, double exponentials), `factorial(·)`, and negative exponents:
    `Growth::Unknown` (absorbing).
- **Forbidden moves (documented + tested):** never specialize a variable to a
  constant inside an O-fact; never rescale coefficients of exponents
  (`2^(2n) ∉ O(2^n)` — exp rates compare coefficientwise, exactly).
- **Isotonicity invariant (for search):** if label `A` dominates label `B`, then for
  any edge `e`, `extend(A, e)` dominates `extend(B, e)`. This follows from the
  monotonicity axiom (composition by substitution into monotone expressions) and is
  the correctness condition for dominance pruning in M3.

## Modules

Only one new file. Everything else is in-place replacement; net LOC is expected
near zero or negative (`canonical.rs`, 431 lines, is deleted).

### M1 — `src/growth.rs` (the one new entity)

```rust
/// One growth monomial, e.g. 2^(3k)·n^2·m·log(n) →
/// { exp: {k:3.0}, poly: {n:2.0, m:1.0}, logs: {n:1} }.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrowthTerm {
    exp:  BTreeMap<&'static str, f64>, // variable → rate, base normalized to 2
                                       // (3^n → {n: log2(3)}); linear forms only
    poly: BTreeMap<&'static str, f64>, // variable → degree (0.5 covers sqrt)
    logs: BTreeMap<&'static str, u32>, // variable → log power
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Growth {
    /// Antichain of pairwise-incomparable dominant terms, sorted by a
    /// deterministic total order (for stable output/serialization).
    Terms(Vec<GrowthTerm>),
    /// Absorbing sentinel: exp/factorial/negative exponents, or cap overflow
    /// that even widening cannot represent. Absorbs through all operations.
    Unknown,
}
```

Operations (each prunes back to an antichain immediately):

- `Growth::from_expr(&Expr) -> Growth` — single bottom-up pass, linear in tree size.
  `Var → {poly:{v:1}}`; `Const → O(1)` (empty term); `Add → union + prune`;
  `Mul → pairwise map-merge + prune`; `Pow(base, const k ≥ 0) →` compute base's
  antichain, then pairwise products (never expands the underlying sums);
  `Log(a) → log(dominant(a))` using `log(n^a·m^b) ≍ log n + log m`;
  `Sqrt = Pow 0.5`; everything else → `Unknown`.
- `dominates(&GrowthTerm, &GrowthTerm) -> bool` — per variable, lexicographic on
  (exp rate, poly degree, log power); dominated iff ≤ on every variable and < on at
  least one. This decides e.g. `1.001^n ≻ n^100` correctly, which the sampling
  heuristic gets wrong.
  Purely symbolic; replaces `numerical_dominance_check`.
- Caps: antichain length cap (default 32). On overflow, **widen upward** to the
  single term taking the componentwise max of all exponents (a valid upper bound),
  never truncate by order.
- Axiom guards: `debug_assert!` nonnegativity/monotonicity preconditions at entry.

Deps: read-only on `expr.rs`. Serde derive here is the whole of former F11.

### M2 — `big_o.rs` pipeline replacement

`big_o_normal_form(&Expr) -> Result<Expr, AsymptoticAnalysisError>` keeps its
signature: internally `Growth::from_expr` → render `Growth` back to a display `Expr`
(`Unknown` maps to the existing `Unsupported` error). CLI callers (`big_o_of`,
`overhead_to_json`, `format_path_text`) are untouched. `compose_path_overhead`
continues to produce the compact nested `Expr` (≤ ~2 KB in the worst observed case);
`from_expr` walks it in microseconds — **no caching, no registry changes**.
`canonical.rs` and the `asymptotic_normal_form` compatibility wrapper are deleted
along with their unit tests (internal API breakage is in-scope).

`pred-sym` (the standalone symbolic CLI, used by the find-problem skills for
`big-o` and `eval`) follows suit: the `canon` subcommand is deleted (no live
consumers), and `compare` narrows its semantics to Big-O equivalence via the growth
domain. `big-o` keeps working on the skills' effective-complexity inputs
(`1.5^n * n^2`) thanks to the linear `exp` field; nonlinear-exponent inputs report
`Unknown` and the skills fall back to `pred-sym eval`.

Alternatives considered: capped expansion (rejected: keeps the exponential algorithm
and reintroduces order-dependent truncation); per-edge growth caching in
`ReductionEntry` with per-path folding (rejected for now: YAGNI at current graph
size; revisit if profiling ever shows `from_expr` on composed paths as hot).

### M3 — Pareto label search kernel (`src/rules/graph.rs`, in-place)

Replace `dijkstra` (~60 lines) with one generic label-setting search (~100 lines)
plus a minimal trait:

```rust
pub trait PathLabel: Clone {
    fn extend(&self, edge: &ReductionEdge) -> Self;   // must be isotone
    fn dominates(&self, other: &Self) -> bool;        // partial order
}
```

- Per-node **bag** = antichain of non-dominated labels, each with a predecessor
  pointer for path reconstruction (McRAPTOR structure).
- Deterministic bounding, in the style of transit routers: hop cap (default 16) and
  per-node bag cap with a **deterministic tie-break** (fewest hops, then
  lexicographic node-name order) — never iteration-order truncation. A label evicted
  from a bag (dominated or cap-truncated) has its arena slot's label freed immediately,
  so the bag cap genuinely bounds retained per-node label memory.
- Label domains:
  - **F3a asymptotic:** label = `BTreeMap<field, Growth>` mapping each size field of
    the current node to its growth in the source's variables; `extend` substitutes
    the edge's overhead expressions; `dominates` is componentwise. Exponential
    growth is comparable via the `exp` field (polynomial paths dominate exponential
    ones); `Unknown` fields make a label dominated by any known label — undecidable
    paths rank last, which is the honest ranking.
  - **F3b instance (measured):** for a concrete instance, formulas are advisory —
    **measured sizes are authoritative**. Overhead formulas are scaling upper bounds
    over the declared size fields and can be arbitrarily loose on
    structure-dependent constructions (see #107), so they must never arbitrate
    between concrete candidates. Label = the actual `ProblemSize` measured on the
    constructed intermediate problem (plus the reduction chain itself, reused for
    solving/witness extraction by the winner); `extend` executes the edge's
    `reduce_to()` and measures. The only instance-budget guard is the **measured
    budget check after execution**. Evaluating an asymptotic expression at one point
    is not a certified concrete bound, so overhead formulas do not prune measured
    candidates. This also means the budget cannot prevent the construction itself
    from exhausting memory.

    Measured search uses **no dominance pruning**. `ProblemSize` omits instance
    structure, and equal-size intermediate instances can produce different sizes under
    a later structure-dependent reduction. Even serialized-state equivalence is not
    used to discard a route. It is therefore a separate exhaustive simple-path
    enumeration, not a label domain in the capped Pareto kernel.

    Note the measured label deliberately does **not** use branch-and-bound: a
    reduction can *shrink* the measured size, so the cost is non-monotone and a
    B&B bound could prune a partial route that would still finish smallest.
    No hop or bag cap truncates this enumeration, so its time and retained constructed
    state can grow exponentially with the number of simple paths. This also does not
    bound temporary memory used inside `reduce_to()`.
    This fixes the path-dependent-cost hole in the current Dijkstra *and* removes
    the dependency on formula accuracy for concrete decisions.
- `find_cheapest_path*` become thin wrappers returning the front (instance mode
  typically collapses to a single optimum after the numeric tie-break).
- `find_dominated_rules` / `compare_overhead` (`src/rules/analysis.rs`) are rewired
  to the same `dominates` order, deleting their bespoke comparison heuristics —
  one trusted comparison everywhere (former F5).
- `all_simple_paths`-based enumeration remains the explicit `--all` listing mechanism;
  measured optimum-finding now performs its own execution-aware simple-path enumeration
  because no sound state-level dominance relation is available.

Alternatives considered: enumerate-then-filter (rejected: combinatorial growth as the
graph densifies, and any truncation limit is iteration-order-dependent — the sibling
package ProblemReductions.jl does exactly this, with no cost model, and it is the
baseline we are improving on); a generic semiring algebraic-path framework (rejected:
over-engineering for two label domains); formula-evaluated instance labels (rejected
after review: overhead formulas are upper bounds over declared size fields and can be
arbitrarily loose on structure-dependent constructions, so a formula-ranked front may
not contain the true winner — measured sizes are the ground truth and affordable at
interactive scales; formulas remain available for asymptotic analysis but do not
decide concrete feasibility).

### M4 — CLI/MCP surface (`problemreductions-cli/src/commands/graph.rs`, in-place)

- Asymptotic `pred path S T`: print the Pareto front (typically 1–3 paths), each with
  its Big-O per size field; paths whose composed growth is `Unknown` (nonlinear
  exponents, factorial) are annotated explicitly instead of showing a fake bound.
- Instance mode (`--size …`): output shape unchanged (single best path).
- `path --all`: keep enumeration; Big-O per path now via M2 (fast); **`--json` mode
  no longer builds the text rendering** (the unconditional `format_path_text` call
  named in issue #1069).
- All path lists sorted by (hops, lexicographic names). JSON emits the structured
  `Growth` serialization. (The paper export consumes raw overhead expressions, not
  Big-O strings — verified unaffected.)

## Quality requirements

- **Reliability:** every public function terminates with an answer or `Unknown` —
  no input can hang or OOM. Regression: issue #1069 path #34; a whole-graph test
  enumerating paths (bounded length) between hot pairs asserts Big-O completion
  within the CI budget (< 5 s per test).
- **Trustworthiness testing:** each `from_expr` transfer function and the dominance
  order get randomized property tests (≥ 5000 checks, matching the repo's
  verify-reduction culture): `eval(expr) ≤ C · eval(render(growth(expr)))` at large
  sizes; `growth` idempotent on its own rendering; `dominates(a,b)` ⟹ sampled
  `eval(b)/eval(a)` grows. Isotonicity of both `PathLabel` impls is property-tested.
- **Determinism:** identical output across platforms; a test compares `pred path`
  output against golden files (antichain and front ordering are total and
  deterministic by construction).
- **Performance:** `pred path KSat QUBO --all` end-to-end < 1 s (currently OOM).
- **Extensibility:** the linear `exp` field ships in M1 (required by the
  find-problem skills' use of `pred-sym big-o` on effective-complexity
  expressions). The remaining upgrade path — nonlinear exponents (a polynomial
  exponent instead of a linear form), needed only if F8-style effective-complexity
  ranking over complexity strings like `2^(num_edges * k)` is ever built — touches
  only `dominates`, `mul`, and `from_expr`'s `Pow/Exp` arms; antichain machinery,
  caps, search kernel, and serialization are unaffected.

## Out of scope

- `#[reduction]` macro, overhead declaration syntax, and all rule files.
- `declare_variants!` complexity strings and their validation
  (`is_valid_complexity_notation`) — untouched; they are display-only in this design.
- FuncKind registry, shared parser crate, effective-complexity ranking, hull pruning,
  egg display layer (deferred/dropped as listed under Features).

## References

- E. Albert, D. Alonso, P. Arenas, S. Genaim, G. Puebla. *Asymptotic Resource Usage
  Bounds.* APLAS 2009. (Normal form + `Θ`-preservation theorem.)
- R. Howell. *On Asymptotic Notation with Multiple Variables.* Kansas State
  University TR 2007-4. (Multivariate O inconsistencies; `O_∀` definition.)
- A. Guéneau, A. Charguéraud, F. Pottier. *A Fistful of Dollars: Formalizing
  Asymptotic Complexity Claims via Deductive Program Verification.* ESOP 2018.
  (Filter-based O; nonnegative-monotone cost discipline; documented pitfalls.)
- M. Brockschmidt, F. Emmes, S. Falke, C. Fuhs, J. Giesl. *Analyzing Runtime and Size
  Complexity of Integer Programs.* TOPLAS 2016. (Weakly monotone bounds compose.)
- SageMath `sage.rings.asymptotic` (growth groups, O-term absorption) — design
  reference only (GPL).
- LLVM `ScalarEvolution` / GCC `tree-chrec` — budgets, sentinels, construction-time
  canonicalization.
- D. Delling, T. Pajor, R. Werneck. *Round-Based Public Transit Routing.* ALENEX
  2012 (McRAPTOR bags); E. Martins. *On a Multicriteria Shortest Path Problem.* EJOR
  1984; L. Mandow, J.-L. Pérez de la Cruz. *Multiobjective A\* with Consistent
  Heuristics.* JACM 2010.
- D. Gruntz. *On Computing Limits in a Symbolic Manipulation System.* ETH 1996
  (dominance ordering; relevant when the `exp` field is added).
- Issue #1069 — root-cause analysis this design responds to.
