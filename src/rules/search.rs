//! Completeness policy and accounting for reduction-path search.

use serde::Serialize;
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

/// Whether a path search must be complete or may use explicit resource limits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchMode {
    /// Search every elementary path allowed by the selected label semantics.
    Exact,
    /// Return valid partial results under an approximation policy.
    Approximate(ApproximationPolicy),
}

/// Policy used by an approximate search.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApproximationPolicy {
    /// Deterministic count limits and/or a wall-clock timeout.
    Bounded(SearchLimits),
}

/// Optional limits for bounded approximate search.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SearchLimits {
    /// Maximum number of edges in an explored path.
    pub max_hops: Option<usize>,
    /// Maximum number of live labels retained at one graph node.
    pub max_labels_per_node: Option<usize>,
    /// Maximum number of states whose outgoing edges are expanded.
    pub max_expanded_states: Option<usize>,
    /// Wall-clock duration checked between state expansions.
    pub timeout: Option<Duration>,
}

impl SearchLimits {
    /// Legacy interactive bounds, now made explicit at the caller boundary.
    pub fn interactive() -> Self {
        Self {
            max_hops: Some(16),
            max_labels_per_node: Some(32),
            max_expanded_states: None,
            timeout: None,
        }
    }
}

/// A resource limit that made a search incomplete.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitReached {
    HopLimit,
    LabelsPerNodeLimit,
    ExpandedStatesLimit,
    Timeout,
}

/// Whether the returned value is complete for the declared search semantics.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SearchCompleteness {
    Exact,
    Approximate { reasons: BTreeSet<LimitReached> },
}

impl SearchCompleteness {
    /// Whether no configured approximation limit affected exploration.
    pub fn is_exact(&self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Limits that affected exploration, empty for an exact outcome.
    pub fn reasons(&self) -> BTreeSet<LimitReached> {
        match self {
            Self::Exact => BTreeSet::new(),
            Self::Approximate { reasons } => reasons.clone(),
        }
    }
}

/// Search work and pruning statistics.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchStats {
    /// Initial and successfully extended states created by the search.
    pub generated_states: usize,
    /// States whose outgoing edges were examined.
    pub expanded_states: usize,
    /// Completed target states removed by terminal Pareto dominance.
    pub dominated_states: usize,
    /// Label extensions rejected by domain feasibility checks.
    pub infeasible_extensions: usize,
    /// Largest number of simultaneously retained states at one node. Exact DFS retains
    /// at most the current branch; bounded search may retain a per-node candidate bag.
    pub peak_labels_per_node: usize,
    /// Elapsed wall-clock time for the whole public request.
    ///
    /// This diagnostic is intentionally omitted from serialized output so
    /// count-limited responses remain byte-stable across runs and platforms.
    #[serde(skip_serializing)]
    pub elapsed: Duration,
}

/// A search value together with its completeness guarantee and work statistics.
#[must_use]
#[derive(Debug)]
pub struct SearchOutcome<T> {
    /// Complete result or valid partial result.
    pub value: T,
    /// Whether configured limits affected the explored search space.
    pub completeness: SearchCompleteness,
    /// Work performed across the whole request.
    pub stats: SearchStats,
}

/// Per-request mutable accounting shared by all traversals for that request.
pub(crate) struct SearchTracker {
    limits: Option<SearchLimits>,
    reached: BTreeSet<LimitReached>,
    stats: SearchStats,
    completed_states: usize,
    started: Instant,
}

impl SearchTracker {
    pub(crate) fn new(mode: &SearchMode) -> Self {
        let limits = match mode {
            SearchMode::Exact => None,
            SearchMode::Approximate(ApproximationPolicy::Bounded(limits)) => Some(limits.clone()),
        };
        Self {
            limits,
            reached: BTreeSet::new(),
            stats: SearchStats::default(),
            completed_states: 0,
            started: Instant::now(),
        }
    }

    pub(crate) fn record_generated(&mut self) {
        self.stats.generated_states += 1;
    }

    pub(crate) fn is_exact_mode(&self) -> bool {
        self.limits.is_none()
    }

    pub(crate) fn record_expanded(&mut self) {
        self.stats.expanded_states += 1;
    }

    pub(crate) fn record_dominated(&mut self, count: usize) {
        self.stats.dominated_states += count;
    }

    pub(crate) fn record_completed(&mut self, count: usize) {
        self.completed_states += count;
    }

    pub(crate) fn completed_states(&self) -> usize {
        self.completed_states
    }

    pub(crate) fn record_infeasible(&mut self) {
        self.stats.infeasible_extensions += 1;
    }

    pub(crate) fn observe_bag(&mut self, size: usize) {
        self.stats.peak_labels_per_node = self.stats.peak_labels_per_node.max(size);
    }

    pub(crate) fn reach(&mut self, reason: LimitReached) {
        self.reached.insert(reason);
    }

    pub(crate) fn hop_limited(&mut self, hops: usize) -> bool {
        let limited = self
            .limits
            .as_ref()
            .and_then(|limits| limits.max_hops)
            .is_some_and(|limit| hops >= limit);
        if limited {
            self.reach(LimitReached::HopLimit);
        }
        limited
    }

    pub(crate) fn expansion_limited(&mut self) -> bool {
        let limited = self
            .limits
            .as_ref()
            .and_then(|limits| limits.max_expanded_states)
            .is_some_and(|limit| self.stats.expanded_states >= limit);
        if limited {
            self.reach(LimitReached::ExpandedStatesLimit);
        }
        limited
    }

    pub(crate) fn timed_out(&mut self) -> bool {
        let timed_out = self
            .limits
            .as_ref()
            .and_then(|limits| limits.timeout)
            .is_some_and(|timeout| self.started.elapsed() >= timeout);
        if timed_out {
            self.reach(LimitReached::Timeout);
        }
        timed_out
    }

    pub(crate) fn label_limit(&self) -> Option<usize> {
        self.limits
            .as_ref()
            .and_then(|limits| limits.max_labels_per_node)
    }

    pub(crate) fn finish<T>(mut self, value: T) -> SearchOutcome<T> {
        self.stats.elapsed = self.started.elapsed();
        let completeness = if self.reached.is_empty() {
            SearchCompleteness::Exact
        } else {
            SearchCompleteness::Approximate {
                reasons: self.reached,
            }
        };
        SearchOutcome {
            value,
            completeness,
            stats: self.stats,
        }
    }
}
