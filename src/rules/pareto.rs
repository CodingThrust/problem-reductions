//! Pareto label-setting search over the reduction graph.
//!
//! This module replaces the old scalar Dijkstra (`ReductionGraph::dijkstra`) with a
//! generic multi-label search. The core motivation (issue #788, design doc
//! `docs/design/symbolic-growth-domain.md`, section M3/F3b) is that edge costs are
//! **path-dependent**: the cost of a reduction depends on the size of the problem
//! accumulated along the path so far. Scalar Dijkstra keeps only the cheapest-so-far
//! label per node, so a cheaper-but-larger intermediate state can poison downstream
//! choices — it can miss the path whose *final* target is smallest.
//!
//! The fix is the standard algorithm for partial-order path costs — **multi-label
//! Pareto search** (Martins 1984; McRAPTOR-style per-node label bags). Each node keeps
//! an antichain of non-dominated labels (a "bag"); a label is only pruned when another
//! label at the same node dominates it. See [`ReductionGraph::pareto_search`].
//!
//! Two search domains are provided:
//! - [`CostLabel`]: a scalar formula label that reproduces Dijkstra's behavior for the
//!   existing `PathCostFn` cost functions (used by `find_cheapest_path*`). It carries the
//!   accumulated `ProblemSize` (from overhead formulas) and an additive scalar cost.
//! - [`MeasuredLabel`]: concrete-instance state used by a separate exhaustive simple-path
//!   search. It *actually executes* each reduction and measures the real constructed target
//!   size. Asymptotic overhead formulas are not used as concrete budget bounds.

use crate::expr::Expr;
use crate::growth::Growth;
use crate::rules::cost::PathCostFn;
use crate::rules::registry::{EdgeCapabilities, ReduceFn, ReductionOverhead};
use crate::rules::traits::DynReductionResult;
use crate::types::ProblemSize;
use std::any::Any;
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::panic;
use std::rc::Rc;
use std::sync::Once;

thread_local! {
    /// When set, the installed panic hook suppresses output on the current thread.
    static SILENCE_PANIC: Cell<bool> = const { Cell::new(false) };
}

static HOOK_INIT: Once = Once::new();

/// Run `f`, catching any panic and returning `None`, without printing the panic to
/// stderr on this thread.
///
/// During the measured search we deliberately execute candidate reductions to measure
/// their real output size. A reduction whose preconditions the current instance violates
/// panics (its macro-generated dispatch downcasts and unwraps); such an edge is simply
/// not a viable path, so we treat the panic as "edge infeasible" and prune it — the
/// design's guarantee that path selection never crashes. The thread-local silencer keeps
/// this expected, recovered panic from spamming stderr while leaving genuine panics on
/// other threads untouched.
pub(crate) fn catch_reduction<R>(f: impl FnOnce() -> R) -> Option<R> {
    HOOK_INIT.call_once(|| {
        let prev = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if SILENCE_PANIC.with(|s| s.get()) {
                return;
            }
            prev(info);
        }));
    });
    SILENCE_PANIC.with(|s| s.set(true));
    let result = panic::catch_unwind(panic::AssertUnwindSafe(f));
    SILENCE_PANIC.with(|s| s.set(false));
    result.ok()
}

/// Default post-construction total-size budget for the measured search (in "size units",
/// i.e. the sum of all `ProblemSize` components).
///
/// A reduction's target must exist before it can be measured, so this limits which
/// constructed instances remain eligible for further search; it cannot prevent the
/// construction itself from exhausting memory.
pub const DEFAULT_SIZE_BUDGET: usize = 10_000_000;

/// Maximum number of reduction steps (hops) explored along any path.
pub const HOP_CAP: usize = 16;

/// Maximum number of non-dominated labels retained per node. On overflow, the bag is
/// truncated by a deterministic tie-break (never by iteration order).
pub const BAG_CAP: usize = 32;

/// A borrowed view of one reduction edge, handed to [`PathLabel::extend`].
///
/// It exposes exactly what a label needs to advance: the overhead formula (for symbolic
/// and formula-based labels), the executable reduction function (for measured execution),
/// the edge capabilities, and the target node's identity (for measuring the constructed
/// target's size by name).
pub struct ReductionEdge<'g> {
    /// Overhead expressions mapping source size fields to target size fields.
    pub overhead: &'g ReductionOverhead,
    /// Type-erased witness reduction executor, if this edge supports witness/config mode.
    pub reduce_fn: Option<ReduceFn>,
    /// Capability metadata for the edge.
    pub capabilities: EdgeCapabilities,
    /// Target problem name (e.g. "ILP").
    pub target_name: &'static str,
    /// Target problem variant.
    pub target_variant: &'g BTreeMap<String, String>,
}

/// A path cost that composes along reduction edges under a partial order.
///
/// **Isotonicity invariant (correctness condition for dominance pruning):** if label
/// `A` dominates label `B`, then for any edge `e`, `A.extend(e)` dominates `B.extend(e)`
/// (when both are `Some`). This follows from the monotonicity of overhead / reduction
/// size in the source size. The Pareto search relies on it to safely discard dominated
/// labels.
///
/// The kernel prunes by [`dominates`](PathLabel::dominates) alone — it does **not**
/// branch-and-bound on [`cost`](PathLabel::cost). Dominance is exact for every label
/// domain, whereas a scalar B&B bound would only be sound for a monotone `cost`; a label's
/// scalar summary may shrink across an edge or summarize an incomparable growth vector.
/// `cost` is used only for frontier ordering and the deterministic final tie-break.
pub trait PathLabel: Clone {
    /// Advance this label across `edge`. Returns `None` when a label-domain guard rejects
    /// the edge. A `None` must be *isotone*:
    /// if `A` dominates `B` and `A.extend(e)` is `None`, that is fine, but a guard must
    /// never prune a dominating label while keeping a dominated one.
    fn extend(&self, edge: &ReductionEdge) -> Option<Self>;

    /// Partial order used to keep each node's bag an antichain. Implementations must
    /// satisfy the isotonicity invariant above.
    fn dominates(&self, other: &Self) -> bool;

    /// Scalar summary used only for frontier ordering and the deterministic final
    /// tie-break — never for pruning (the kernel prunes by [`dominates`] alone). Smaller
    /// is better. It need not be monotone along `extend`.
    ///
    /// [`dominates`]: PathLabel::dominates
    fn cost(&self) -> f64;
}

/// Formula-based label for a [`PathCostFn`].
///
/// Carries the accumulated `ProblemSize` (advanced through overhead formulas) and the
/// additive scalar cost. Because a future edge's [`edge_cost`](PathCostFn::edge_cost)
/// depends on the carried size, dominance is **componentwise Pareto over `(cost, size)`**,
/// not scalar: a cheaper-but-larger prefix must not evict a costlier-but-smaller one whose
/// continuation is globally cheapest. Each node therefore keeps the antichain of
/// non-dominated `(cost, size)` labels rather than a single minimum-cost representative.
pub struct CostLabel<'c, C: PathCostFn> {
    size: ProblemSize,
    cost: f64,
    cost_fn: &'c C,
}

// Manual `Clone` (the derive would wrongly require `C: Clone`; `cost_fn` is a reference).
impl<C: PathCostFn> Clone for CostLabel<'_, C> {
    fn clone(&self) -> Self {
        Self {
            size: self.size.clone(),
            cost: self.cost,
            cost_fn: self.cost_fn,
        }
    }
}

impl<'c, C: PathCostFn> CostLabel<'c, C> {
    /// Create the initial label at the source node.
    pub fn new(input_size: ProblemSize, cost_fn: &'c C) -> Self {
        Self {
            size: input_size,
            cost: 0.0,
            cost_fn,
        }
    }
}

impl<C: PathCostFn> PathLabel for CostLabel<'_, C> {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        let increment = self.cost_fn.edge_cost(edge.overhead, &self.size);
        let new_size = edge.overhead.evaluate_output_size(&self.size);
        Some(Self {
            size: new_size,
            cost: self.cost + increment,
            cost_fn: self.cost_fn,
        })
    }

    fn dominates(&self, other: &Self) -> bool {
        // Path-dependent costs: a future edge's `edge_cost` depends on the carried size,
        // so `self` may only evict `other` when it is componentwise no worse in BOTH the
        // accumulated cost and the carried size. Scalar `cost <= other.cost` alone would
        // let a cheap-but-large prefix evict the globally optimal small one.
        self.cost <= other.cost && size_le(&self.size, &other.size)
    }

    fn cost(&self) -> f64 {
        self.cost
    }
}

/// The current constructed position of a [`MeasuredLabel`].
#[derive(Clone)]
enum MeasuredPos<'a> {
    /// At the source node: the original, un-reduced source instance.
    Source(&'a dyn Any),
    /// At a reduced node: the last reduction step's result. The current problem instance
    /// is `result.target_problem_any()`.
    Reduced(Rc<dyn DynReductionResult>),
}

/// The concrete-instance measured label (design doc M3/F3b).
///
/// For a concrete source instance, the **measured** target size is authoritative.
/// Asymptotic overhead formulas are deliberately not consulted: evaluating a Big-O
/// expression at one input does not produce a certified concrete upper bound.
/// `extend` runs this stack, in order:
///
/// 1. **Execute + measure:** run `reduce_to()`, measure the real target size; over budget
///    → `None`.
/// 2. **No comparative pruning:** measured states are enumerated by a separate exhaustive
///    simple-path search. Neither size vectors nor serialized representations discard a
///    constructed route before its downstream reductions are measured, and Pareto bag/hop
///    caps do not apply.
///
/// **Memory.** The budget is checked only after a reduction has constructed its target,
/// so it cannot prevent a reduction itself from exhausting memory. It limits which
/// constructed instances remain eligible for further search. Exhaustive simple-path
/// enumeration can take exponential time and retain large constructed chains.
#[derive(Clone)]
pub struct MeasuredLabel<'a> {
    /// Measured size of the problem instance at the current node.
    size: ProblemSize,
    /// The reduction steps executed so far (empty at the source). Shared via `Rc` so
    /// cloning a label is cheap and never re-executes a reduction.
    chain: Vec<Rc<dyn DynReductionResult>>,
    /// Current constructed position.
    pos: MeasuredPos<'a>,
    /// Hard total-size budget.
    budget: usize,
}

impl<'a> MeasuredLabel<'a> {
    /// Create the initial measured label at the source node.
    ///
    /// `source_size` is the measured size of `source` (typically
    /// `ReductionGraph::compute_source_size`).
    pub fn new(source: &'a dyn Any, source_size: ProblemSize, budget: usize) -> Self {
        Self {
            size: source_size,
            chain: Vec::new(),
            pos: MeasuredPos::Source(source),
            budget,
        }
    }

    /// The reduction chain executed to reach this label (one entry per hop).
    pub(crate) fn chain(&self) -> &[Rc<dyn DynReductionResult>] {
        &self.chain
    }

    /// The measured problem size at this label's node.
    pub(crate) fn measured_size(&self) -> &ProblemSize {
        &self.size
    }

    /// Execute one reduction and retain the state only when its measured target is
    /// within the post-construction budget.
    pub(crate) fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        // Execute the reduction and measure the real target size. Executing a
        // reduction whose preconditions the current instance violates panics; such an
        // edge is not a viable path, so a caught panic prunes it (returns `None`). The
        // measurement (`compute_source_size`) probes every same-name size function, so
        // mismatched-variant probes panic internally too — both are wrapped in one
        // silenced `catch_reduction`.
        let reduce_fn = edge.reduce_fn?;
        let current: &dyn Any = match &self.pos {
            MeasuredPos::Source(s) => *s,
            MeasuredPos::Reduced(r) => r.target_problem_any(),
        };
        let target_name = edge.target_name;
        let (result, measured) = catch_reduction(|| {
            let result: Rc<dyn DynReductionResult> = Rc::from(reduce_fn(current));
            let measured = crate::rules::ReductionGraph::compute_source_size(
                target_name,
                result.target_problem_any(),
            );
            (result, measured)
        })?;
        if measured.total() > self.budget {
            return None;
        }

        let mut chain = self.chain.clone();
        chain.push(result.clone());
        Some(Self {
            size: measured,
            chain,
            pos: MeasuredPos::Reduced(result),
            budget: self.budget,
        })
    }
}

/// Componentwise "less-or-equal in every field" test between two sizes.
/// Missing fields are treated as `0`.
fn size_le(a: &ProblemSize, b: &ProblemSize) -> bool {
    a.components
        .iter()
        .all(|(name, av)| *av <= b.get(name).unwrap_or(0))
}

/// Asymptotic, **instance-free** label domain (design doc M3/F3a).
///
/// Each entry maps one size field of the **current** node to its
/// [`Growth`](crate::growth::Growth) expressed in the **source problem's** size
/// variables. The initial label at source `S` maps every one of `S`'s size fields
/// `f` to `Growth::from_expr(Var(f))` — "field `f` grows like itself".
///
/// [`extend`](PathLabel::extend) composes an edge's overhead into the label: each
/// target size-field's overhead `Expr` is written over the *current* node's field
/// names, so we substitute each current field's rendered growth
/// ([`Growth::to_expr`](crate::growth::Growth::to_expr)) into it and run
/// [`Growth::from_expr`](crate::growth::Growth::from_expr) on the result. This reuses
/// the whole M1+M2 growth pipeline and needs no new growth-domain primitive. A field
/// whose growth is [`Growth::Unknown`](crate::growth::Growth::Unknown) (nonlinear
/// exponent, factorial) has no `Expr`; any target field depending on it becomes
/// `Unknown` too — the bound is never fabricated.
///
/// [`dominates`](PathLabel::dominates) is componentwise in the **search** sense
/// (smaller growth = better): `self` dominates `other` iff for *every* field `self`
/// grows no faster than `other`, and strictly slower on at least one. Because
/// `Unknown` is the top of the growth order, a label with an `Unknown` field is
/// dominated by any fully-known label — undecidable paths rank last, the honest
/// ranking.
///
/// **Isotonicity** (the correctness condition for the kernel's dominance pruning)
/// follows from the growth domain's monotonicity axiom: `from_expr` composed with
/// substitution into weakly-monotone overhead expressions preserves the growth
/// order, so `A ⪰ B ⇒ extend(A,e) ⪰ extend(B,e)`.
#[derive(Clone, Debug, PartialEq)]
pub struct GrowthLabel {
    /// Current node's size fields → growth in the source problem's variables.
    fields: BTreeMap<&'static str, Growth>,
}

impl GrowthLabel {
    /// The initial label at a source node: each size field grows like itself.
    ///
    /// `source_fields` is the source problem's list of size-field names (e.g. from
    /// [`ReductionGraph::size_field_names`](crate::rules::ReductionGraph::size_field_names)).
    pub fn source(source_fields: &[&'static str]) -> Self {
        let fields = source_fields
            .iter()
            .map(|&f| (f, Growth::from_expr(&Expr::Var(f))))
            .collect();
        GrowthLabel { fields }
    }

    /// Construct directly from a field → growth map (test/introspection helper).
    pub fn from_fields(fields: BTreeMap<&'static str, Growth>) -> Self {
        GrowthLabel { fields }
    }

    /// The current node's size fields mapped to their growth in source variables.
    pub fn fields(&self) -> &BTreeMap<&'static str, Growth> {
        &self.fields
    }
}

impl PathLabel for GrowthLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        // Render each current field's growth back to a display `Expr` in the source
        // variables. `Unknown` growth has no `Expr` (`None`) and taints any target
        // field that references it.
        let rendered: BTreeMap<&'static str, Option<Expr>> =
            self.fields.iter().map(|(k, g)| (*k, g.to_expr())).collect();

        // Substitution map from current field name to its rendered growth `Expr` (in
        // source variables). Depends only on `rendered`, so build it once for all edges'
        // output fields rather than per target field. Only present-and-known fields are
        // mapped. Unlike `ReductionOverhead::compose`, an overhead variable ABSENT from
        // this map is NOT a passthrough source variable: in the asymptotic label it is an
        // intermediate-only field with no source-variable growth, so any target field that
        // references it must be tainted (see below) rather than leaked verbatim.
        let mapping: HashMap<&str, &Expr> = rendered
            .iter()
            .filter_map(|(k, opt)| opt.as_ref().map(|e| (*k, e)))
            .collect();

        let mut new_fields: BTreeMap<&'static str, Growth> = BTreeMap::new();
        for (target_field, expr) in &edge.overhead.output_size {
            // Taint the target field if this overhead references any variable we cannot
            // express in the source's variables: either a present-but-`Unknown` current
            // field, or a variable absent from the label entirely (an intermediate-only
            // field that would otherwise leak through `substitute` as a fake source
            // variable). Both cases are exactly "not in `mapping`".
            let taints = expr.variables().iter().any(|v| !mapping.contains_key(v));
            if taints {
                new_fields.insert(target_field, Growth::Unknown);
                continue;
            }
            // Substitute rendered growths into the overhead, then reduce in the growth
            // domain.
            let substituted = expr.substitute(&mapping);
            new_fields.insert(target_field, Growth::from_expr(&substituted));
        }
        // Asymptotic mode has no budget, so `extend` never prunes.
        Some(GrowthLabel { fields: new_fields })
    }

    fn dominates(&self, other: &Self) -> bool {
        // Search-sense componentwise dominance over the union of fields (labels
        // compared are at the same node, so their field sets coincide; the union is
        // defensive). `self` dominates `other` iff `self` grows no faster on every
        // field and strictly slower on at least one.
        //
        // `Growth::dominates(a, b)` means "a grows ≥ b", with `Unknown` as top. So:
        //   self ≤ other on field f  ⟺  other_f.dominates(self_f)
        // and self is strictly better on f iff additionally NOT self_f.dominates(other_f).
        let o1 = Growth::Terms(Vec::new()); // O(1): the bottom, for absent fields.
        let keys: BTreeSet<&'static str> = self
            .fields
            .keys()
            .chain(other.fields.keys())
            .copied()
            .collect();
        let mut strict = false;
        for k in keys {
            let s = self.fields.get(k).unwrap_or(&o1);
            let o = other.fields.get(k).unwrap_or(&o1);
            if !o.dominates(s) {
                // self grows strictly faster than other here → self does not dominate.
                return false;
            }
            if !s.dominates(o) {
                // other ≥ self but self ⋡ other ⇒ self strictly slower on this field.
                strict = true;
            }
        }
        strict
    }

    fn cost(&self) -> f64 {
        // Heuristic scalar summary for frontier ordering and the deterministic final
        // tie-break ONLY — never for pruning (dominance is the exact partial order, and
        // asymptotic growth is incomparable so no scalar bound could separate front
        // members). Summed field magnitudes; `Unknown` fields dominate the sum, ranking
        // undecidable paths last.
        self.fields.values().map(|g| g.magnitude()).sum()
    }
}
