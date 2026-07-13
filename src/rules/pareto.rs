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
//! Two label domains are provided:
//! - [`CostLabel`]: a scalar formula label that reproduces Dijkstra's behavior for the
//!   existing `PathCostFn` cost functions (used by `find_cheapest_path*`). It carries the
//!   accumulated `ProblemSize` (from overhead formulas) and an additive scalar cost.
//! - [`MeasuredLabel`]: the concrete-instance label. For a concrete source instance, it
//!   *actually executes* each reduction and measures the real constructed target size.
//!   Formulas are only used as a pre-flight guard, never to arbitrate between candidates.

use crate::rules::cost::PathCostFn;
use crate::rules::registry::{EdgeCapabilities, ReduceFn, ReductionOverhead};
use crate::rules::traits::DynReductionResult;
use crate::types::ProblemSize;
use std::any::Any;
use std::cell::Cell;
use std::collections::BTreeMap;
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
fn catch_reduction<R>(f: impl FnOnce() -> R) -> Option<R> {
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

/// Default hard total-size budget for the measured search (in "size units", i.e. the
/// sum of all `ProblemSize` components). Generous by design: the point is to refuse
/// astronomic constructions (e.g. a `2^num_vertices` blow-up), not to micro-manage.
pub const DEFAULT_SIZE_BUDGET: usize = 10_000_000;

/// Maximum number of reduction steps (hops) explored along any path.
pub const HOP_CAP: usize = 16;

/// Maximum number of non-dominated labels retained per node. On overflow, the bag is
/// truncated by a deterministic tie-break (never by iteration order).
pub const BAG_CAP: usize = 32;

/// A borrowed view of one reduction edge, handed to [`PathLabel::extend`].
///
/// It exposes exactly what a label needs to advance: the overhead formula (for the
/// symbolic pre-flight guard and formula-based sizing), the executable reduction
/// function (for measured execution), the edge capabilities, and the target node's
/// identity (for measuring the constructed target's size by name).
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
/// **B&B soundness:** [`cost`](PathLabel::cost) must be non-decreasing along `extend`
/// (a reduction never shrinks the tracked cost below the current value). Every concrete
/// cost function and the measured-size total satisfy this.
pub trait PathLabel: Clone {
    /// Advance this label across `edge`. Returns `None` when a guard prunes the edge
    /// (e.g. the measured label's pre-flight size guard). A `None` must be *isotone*:
    /// if `A` dominates `B` and `A.extend(e)` is `None`, that is fine, but a guard must
    /// never prune a dominating label while keeping a dominated one.
    fn extend(&self, edge: &ReductionEdge) -> Option<Self>;

    /// Partial order: `true` iff `self` is at least as good as `other` in every
    /// component (and strictly better in at least one, or equal). Used to keep each
    /// node's bag an antichain.
    fn dominates(&self, other: &Self) -> bool;

    /// Scalar summary used for branch-and-bound pruning, frontier ordering, and the
    /// deterministic final tie-break. Smaller is better. Must be non-decreasing along
    /// `extend` (see trait docs).
    fn cost(&self) -> f64;
}

/// Formula-based scalar label reproducing Dijkstra behavior for a [`PathCostFn`].
///
/// Carries the accumulated `ProblemSize` (advanced through overhead formulas) and the
/// additive scalar cost. Dominance is scalar (`self.cost <= other.cost`), so each node
/// keeps only its minimum-cost label — exactly the classic single-objective shortest
/// path, but expressed in the generic kernel.
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
        self.cost <= other.cost
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
/// For a concrete source instance, formulas are advisory — the **measured** target size
/// is authoritative. `extend` runs this four-part pruning stack, in order:
///
/// 1. **Symbolic pre-flight guard:** evaluate the edge's overhead formula at the current
///    *measured* size. If the (upper-bound) prediction already exceeds the budget, return
///    `None` **without executing** — so a catastrophic construction (e.g. a
///    `2^num_vertices` blow-up) is never even started. This is what makes OOM
///    structurally impossible during path selection.
/// 2. **Execute + measure:** run `reduce_to()`, measure the real target size; over budget
///    → `None`.
/// 3. **Branch-and-bound:** handled by the kernel using [`cost`](PathLabel::cost) against
///    the best completed path's final size.
/// 4. **Componentwise measured-size dominance:** [`dominates`](PathLabel::dominates), a
///    heuristic under a documented size-monotone-future assumption. The kernel's
///    `exhaustive` flag disables *only* this guard, keeping 1–3 (which are sound).
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
}

/// Componentwise "less-or-equal in every field" test between two measured sizes.
///
/// `a` covers `b` iff every field of `b` is present in `a` with a value `>=` b's — i.e.
/// `a` is componentwise `<=` `b`. Missing fields are treated as `0`.
fn size_le(a: &ProblemSize, b: &ProblemSize) -> bool {
    // a <= b componentwise: for each field in either, a[f] <= b[f].
    a.components.iter().all(|(name, av)| {
        let bv = b.get(name).unwrap_or(0);
        *av <= bv
    }) && b.components.iter().all(|(name, bv)| {
        let av = a.get(name).unwrap_or(0);
        av <= *bv
    })
}

impl PathLabel for MeasuredLabel<'_> {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        // Guard 1: symbolic pre-flight. Predict the target size from the overhead
        // formula evaluated at the *measured* current size. Because formulas are upper
        // bounds, a prediction over budget means we must not even start the construction.
        // Computed in `f64` so an astronomic prediction (e.g. `2^num_vertices`) is flagged
        // rather than overflowing `usize`.
        let predicted_total = edge.overhead.evaluate_output_total_f64(&self.size);
        if predicted_total > self.budget as f64 {
            return None;
        }

        // Guard 2: execute the reduction and measure the real target size. Executing a
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

    fn dominates(&self, other: &Self) -> bool {
        // Componentwise measured-size dominance. Labels compared here are always at the
        // same node (same problem variant), so their size fields coincide.
        size_le(&self.size, &other.size)
    }

    fn cost(&self) -> f64 {
        self.size.total() as f64
    }
}
