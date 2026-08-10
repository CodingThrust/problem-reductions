//! Concrete-instance state used by measured simple-path search.
//!
//! Exact-size and certified-bound ranking have separate APIs and result types in
//! [`ReductionGraph`](crate::rules::ReductionGraph).

use crate::rules::registry::{ReduceFn, ReductionSizeContract, SizeContractError};
use crate::rules::traits::DynReductionResult;
use crate::types::ProblemSize;
use std::any::Any;
use std::collections::BTreeMap;
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

/// A borrowed view of one reduction edge used by measured execution.
pub struct ReductionEdge<'g> {
    /// Validated exact/bound/unavailable size metadata for this edge.
    pub size_contract: &'g Result<ReductionSizeContract, SizeContractError>,
    /// Type-erased witness reduction executor, if this edge supports witness/config mode.
    pub reduce_fn: Option<ReduceFn>,
    /// Target problem name (e.g. "ILP").
    pub target_name: &'static str,
    /// Target problem variant.
    pub target_variant: &'g BTreeMap<String, String>,
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
/// Asymptotic growth formulas are deliberately not consulted: evaluating a Big-O
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

    pub(crate) fn final_dominates(&self, other: &Self) -> bool {
        self.size.components.len() == other.size.components.len()
            && self.size.components.iter().all(|(field, value)| {
                other
                    .size
                    .get(field)
                    .is_some_and(|other_value| *value <= other_value)
            })
    }
}
