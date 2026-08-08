//! Multi-label elementary-path search over the reduction graph.
//!
//! The search keeps multiple path states per node and filters the Pareto front only at
//! the destination. Intermediate strict dominance is deliberately forbidden: arbitrary
//! reduction overheads may shrink, subtract, or otherwise reverse an apparent order.
//! The current labels do not carry complete constructed instances, so even equal labels
//! are retained as distinct intermediate states. See [`ReductionGraph::pareto_search`].
//!
//! Two search domains are provided:
//! - [`GrowthLabel`]: symbolic componentwise growth for the asymptotic front.
//! - [`MeasuredLabel`]: concrete-instance state used by a separate simple-path search. It
//!   *actually executes* each reduction and measures the real constructed target size.
//!   Asymptotic overhead formulas are not used as concrete budget bounds.

use crate::expr::Expr;
use crate::growth::Growth;
use crate::rules::registry::{ReduceFn, ReductionOverhead};
use crate::rules::traits::DynReductionResult;
use crate::types::ProblemSize;
use serde::Serialize;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

/// Per-field post-construction limits for measured search.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SizeBudget {
    limits: BTreeMap<String, usize>,
}

impl SizeBudget {
    /// Create limits keyed by registered [`ProblemSize`] field name.
    pub fn new(limits: BTreeMap<String, usize>) -> Self {
        Self { limits }
    }

    pub(crate) fn fields(&self) -> impl Iterator<Item = &str> {
        self.limits.keys().map(String::as_str)
    }

    pub(crate) fn permits(&self, size: &ProblemSize) -> bool {
        size.components
            .iter()
            .all(|(field, value)| self.limits.get(field).is_none_or(|limit| value <= limit))
    }
}

/// A configured measured-budget field does not exist in the registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownSizeField(pub String);

impl std::fmt::Display for UnknownSizeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown problem-size field: {}", self.0)
    }
}

impl std::error::Error for UnknownSizeField {}

/// Coverage of symbolic analysis, independent of graph-search completeness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AnalysisCoverage {
    pub analyzed_paths: usize,
    pub excluded_paths: usize,
}

/// Why a searched path could not participate in the symbolic front.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AnalysisFailure {
    pub fields: Vec<String>,
    pub reason: &'static str,
}

/// A borrowed view of one reduction edge, handed to [`PathLabel::extend`].
///
/// It exposes exactly what a label needs to advance: the overhead formula (for symbolic
/// and formula-based labels), the executable reduction function (for measured execution),
/// and the target node's identity (for measuring the constructed target's size by name).
pub struct ReductionEdge<'g> {
    /// Overhead expressions mapping source size fields to target size fields.
    pub overhead: &'g ReductionOverhead,
    /// Type-erased witness reduction executor, if this edge supports witness/config mode.
    pub reduce_fn: Option<ReduceFn>,
    /// Target problem name (e.g. "ILP").
    pub target_name: &'static str,
    /// Target problem variant.
    pub target_variant: &'g BTreeMap<String, String>,
}

/// Abstract state carried along a reduction path.
///
/// The kernel never prunes or coalesces an intermediate state: the built-in labels do
/// not contain enough information to prove that two constructed problems are identical.
/// Terminal dominance is applied only after a path reaches the destination, where no
/// future extension can reverse the order. Agenda and result ordering use only hops and
/// the stable path key; they do not select an objective winner.
pub trait PathLabel: Clone {
    /// Advance this label across `edge`. Returns `None` when a label-domain guard rejects
    /// the edge.
    fn extend(&self, edge: &ReductionEdge) -> Option<Self>;

    /// Weak Pareto order used only to filter completed labels at the destination.
    ///
    /// Implementations must provide a reflexive and transitive relation. Mutual
    /// dominance denotes the same terminal objective vector; the kernel then retains one
    /// deterministic representative.
    fn final_dominates(&self, other: &Self) -> bool;
}

/// The current constructed position of a [`MeasuredLabel`].
#[derive(Clone)]
enum MeasuredPos<'a> {
    /// At the source node: the original, un-reduced source instance.
    Source(&'a dyn Any),
    /// At a reduced node: the last reduction step's result. The current problem instance
    /// is `result.target_problem_any()`.
    Reduced(Rc<MeasuredStep>),
}

/// One persistent reduction-chain link. Sharing predecessors makes label extension O(1)
/// in path depth while keeping every constructed intermediate alive as long as needed.
struct MeasuredStep {
    result: Rc<dyn DynReductionResult>,
    previous: Option<Rc<MeasuredStep>>,
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
/// 2. **No comparative pruning:** measured states are enumerated by a separate
///    simple-path search. Neither size vectors nor serialized representations discard a
///    constructed route before its downstream reductions are measured. Exact mode has no
///    search caps; approximate mode applies only its explicit reported limits.
///
/// **Memory.** The budget is checked only after a reduction has constructed its target,
/// so it cannot prevent a reduction itself from exhausting memory. It limits which
/// constructed instances remain eligible for further search. Exact simple-path
/// enumeration can take exponential time; persistent chain links release completed
/// branches instead of copying every prefix.
#[derive(Clone)]
pub struct MeasuredLabel<'a> {
    /// Measured size of the problem instance at the current node.
    size: ProblemSize,
    /// Current constructed position.
    pos: MeasuredPos<'a>,
    /// Per-field post-construction budget.
    budget: Rc<SizeBudget>,
}

impl<'a> MeasuredLabel<'a> {
    /// Create the initial measured label at the source node.
    ///
    /// `source_size` is the measured size of `source` (typically
    /// `ReductionGraph::compute_source_size`).
    pub fn new(source: &'a dyn Any, source_size: ProblemSize, budget: SizeBudget) -> Self {
        Self {
            size: source_size,
            pos: MeasuredPos::Source(source),
            budget: Rc::new(budget),
        }
    }

    /// Reconstruct the reduction chain executed to reach this label.
    pub(crate) fn chain(&self) -> Vec<Rc<dyn DynReductionResult>> {
        let mut chain = Vec::new();
        let mut step = match &self.pos {
            MeasuredPos::Source(_) => None,
            MeasuredPos::Reduced(step) => Some(Rc::clone(step)),
        };
        while let Some(current) = step {
            chain.push(Rc::clone(&current.result));
            step = current.previous.as_ref().map(Rc::clone);
        }
        chain.reverse();
        chain
    }

    /// The measured problem size at this label's node.
    pub(crate) fn measured_size(&self) -> &ProblemSize {
        &self.size
    }

    /// Execute one reduction and retain the state only when its measured target is
    /// within the post-construction budget.
    pub(crate) fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        // Execute the reduction and measure the real target size. The graph has already
        // selected the exact source variant, so any panic is a reduction defect and must
        // remain visible.
        let reduce_fn = edge.reduce_fn?;
        let current: &dyn Any = match &self.pos {
            MeasuredPos::Source(s) => *s,
            MeasuredPos::Reduced(step) => step.result.target_problem_any(),
        };
        let result: Rc<dyn DynReductionResult> = Rc::from(reduce_fn(current));
        let measured = crate::rules::ReductionGraph::compute_source_size(
            edge.target_name,
            edge.target_variant,
            result.target_problem_any(),
        );
        if !self.budget.permits(&measured) {
            return None;
        }

        let previous = match &self.pos {
            MeasuredPos::Source(_) => None,
            MeasuredPos::Reduced(step) => Some(Rc::clone(step)),
        };
        let step = Rc::new(MeasuredStep { result, previous });
        Some(Self {
            size: measured,
            pos: MeasuredPos::Reduced(step),
            budget: Rc::clone(&self.budget),
        })
    }
}

impl PathLabel for MeasuredLabel<'_> {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        MeasuredLabel::extend(self, edge)
    }

    fn final_dominates(&self, other: &Self) -> bool {
        self.size.components.len() == other.size.components.len()
            && self.size.components.iter().all(|(field, value)| {
                other
                    .size
                    .get(field)
                    .is_some_and(|other_value| *value <= other_value)
            })
    }
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
/// [`final_dominates`](PathLabel::final_dominates) is componentwise in the **search**
/// sense (smaller growth = better): `self` terminally dominates `other` iff for every field
/// `self` grows no faster than `other`. It is used only at the destination. A label
/// containing `Unknown` is outside this dominance relation. Such a path is
/// reported as an analysis failure and excluded from the symbolic Pareto front.
///
#[derive(Clone, Debug, PartialEq)]
pub struct GrowthLabel {
    /// Current node's size fields → growth in the source problem's variables.
    fields: BTreeMap<String, Growth>,
}

impl GrowthLabel {
    /// The initial label at a source node: each size field grows like itself.
    ///
    /// `source_fields` is the source problem's list of size-field names (e.g. from
    /// [`ReductionGraph::size_field_names`](crate::rules::ReductionGraph::size_field_names)).
    pub fn source(source_fields: &[String]) -> Self {
        let fields = source_fields
            .iter()
            .map(|field| {
                (
                    field.clone(),
                    Growth::from_expr(&Expr::variable(field.as_str())),
                )
            })
            .collect();
        GrowthLabel { fields }
    }

    /// Construct directly from a field → growth map (test/introspection helper).
    pub fn from_fields<K>(fields: BTreeMap<K, Growth>) -> Self
    where
        K: Into<String> + Ord,
    {
        GrowthLabel {
            fields: fields
                .into_iter()
                .map(|(field, growth)| (field.into(), growth))
                .collect(),
        }
    }

    /// The current node's size fields mapped to their growth in source variables.
    pub fn fields(&self) -> &BTreeMap<String, Growth> {
        &self.fields
    }

    /// Return the explicit failure boundary when any field is unanalyzable.
    pub fn analysis_failure(&self) -> Option<AnalysisFailure> {
        let fields: Vec<_> = self
            .fields
            .iter()
            .filter(|(_, growth)| matches!(growth, Growth::Unknown))
            .map(|(field, _)| field.clone())
            .collect();
        (!fields.is_empty()).then_some(AnalysisFailure {
            fields,
            reason: "symbolic growth analysis returned Unknown",
        })
    }
}

impl PathLabel for GrowthLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        // Render each current field's growth back to a display `Expr` in the source
        // variables. `Unknown` growth has no `Expr` (`None`) and taints any target
        // field that references it.
        let rendered: BTreeMap<&str, Option<Expr>> = self
            .fields
            .iter()
            .map(|(field, growth)| (field.as_str(), growth.to_expr()))
            .collect();

        // Substitution map from current field name to its rendered growth `Expr` (in
        // source variables). Depends only on `rendered`, so build it once for all edges'
        // output fields rather than per target field. Only present-and-known fields are
        // mapped. Unlike `ReductionOverhead::compose`, an overhead variable ABSENT from
        // this map is NOT a passthrough source variable: in the asymptotic label it is an
        // intermediate-only field with no source-variable growth, so any target field that
        // references it must be tainted (see below) rather than leaked verbatim.
        let mapping: HashMap<&str, &Expr> = rendered
            .iter()
            .filter_map(|(field, expression)| expression.as_ref().map(|value| (*field, value)))
            .collect();

        let mut new_fields: BTreeMap<String, Growth> = BTreeMap::new();
        for (target_field, expr) in &edge.overhead.output_size {
            // Taint the target field if this overhead references any variable we cannot
            // express in the source's variables: either a present-but-`Unknown` current
            // field, or a variable absent from the label entirely (an intermediate-only
            // field that would otherwise leak through `substitute` as a fake source
            // variable). Both cases are exactly "not in `mapping`".
            let taints = expr.variables().iter().any(|v| !mapping.contains_key(v));
            if taints {
                new_fields.insert((*target_field).to_string(), Growth::Unknown);
                continue;
            }
            // Substitute rendered growths into the overhead, then reduce in the growth
            // domain.
            let substituted = expr.substitute(&mapping);
            new_fields.insert((*target_field).to_string(), Growth::from_expr(&substituted));
        }
        // Asymptotic mode has no budget, so `extend` never prunes.
        Some(GrowthLabel { fields: new_fields })
    }

    fn final_dominates(&self, other: &Self) -> bool {
        if self
            .fields
            .values()
            .chain(other.fields.values())
            .any(|growth| matches!(growth, Growth::Unknown))
        {
            return false;
        }
        // Labels compared at the same terminal node have the same field set. Equality
        // counts so the terminal front has one
        // deterministic representative per growth vector.
        //
        // `Growth::dominates(a, b)` means "a grows ≥ b", with `Unknown` as top. So:
        //   self ≤ other on field f  ⟺  other_f.dominates(self_f)
        assert_eq!(
            self.fields.len(),
            other.fields.len(),
            "terminal growth fields differ"
        );
        for ((self_field, self_growth), (other_field, other_growth)) in
            self.fields.iter().zip(&other.fields)
        {
            assert_eq!(self_field, other_field, "terminal growth fields differ");
            if !other_growth.dominates(self_growth) {
                // self grows strictly faster than other here → self does not dominate.
                return false;
            }
        }
        true
    }
}
