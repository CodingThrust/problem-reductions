//! Shared functional-dependency branch-and-bound backend.
//!
//! Provides a generic attribute-subset search that exploits FD closure
//! to prune branches, shared across MinimumCardinalityKey, AdditionalKey,
//! PrimeAttributeName, and BoyceCoddNormalFormViolation.

/// Decision returned by the partial-acceptance predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BranchDecision {
    /// Continue branching.
    Continue,
    /// Prune this branch (no extension can succeed).
    Prune,
}

/// Search for a subset of a universe that satisfies model-specific predicates,
/// using deterministic branch-and-bound with closure-based pruning.
///
/// - `universe_size`: total number of elements to branch over
/// - `initially_forced`: indices that must be included in any solution
/// - `branch_order`: order in which to branch (indices into 0..universe_size)
/// - `accept_partial`: called at each branch point; returns `Prune` to skip
/// - `accept_complete`: called when all branch decisions are made; returns true for a witness
///
/// Returns the selected subset as a sorted `Vec<usize>` of indices.
pub(crate) fn search_fd_subset(
    universe_size: usize,
    initially_forced: &[usize],
    branch_order: &[usize],
    accept_partial: impl Fn(&[bool], usize) -> BranchDecision,
    accept_complete: impl Fn(&[bool]) -> bool,
) -> Option<Vec<usize>> {
    let mut selected = vec![false; universe_size];
    for &i in initially_forced {
        selected[i] = true;
    }

    let mut result = None;
    dfs(
        &mut selected,
        branch_order,
        0,
        &accept_partial,
        &accept_complete,
        &mut result,
    );
    result
}

fn dfs(
    selected: &mut Vec<bool>,
    branch_order: &[usize],
    depth: usize,
    accept_partial: &impl Fn(&[bool], usize) -> BranchDecision,
    accept_complete: &impl Fn(&[bool]) -> bool,
    result: &mut Option<Vec<usize>>,
) {
    if result.is_some() {
        return;
    }

    if depth == branch_order.len() {
        if accept_complete(selected) {
            *result = Some(
                selected
                    .iter()
                    .enumerate()
                    .filter(|(_, &v)| v)
                    .map(|(i, _)| i)
                    .collect(),
            );
        }
        return;
    }

    let idx = branch_order[depth];

    // If this index is already forced, skip branching
    if selected[idx] {
        if accept_partial(selected, depth) == BranchDecision::Prune {
            return;
        }
        dfs(
            selected,
            branch_order,
            depth + 1,
            accept_partial,
            accept_complete,
            result,
        );
        return;
    }

    // Try including the attribute first (more likely to find solutions)
    selected[idx] = true;
    if accept_partial(selected, depth) != BranchDecision::Prune {
        dfs(
            selected,
            branch_order,
            depth + 1,
            accept_partial,
            accept_complete,
            result,
        );
    }
    selected[idx] = false;

    if result.is_some() {
        return;
    }

    // Try excluding the attribute
    if accept_partial(selected, depth) != BranchDecision::Prune {
        dfs(
            selected,
            branch_order,
            depth + 1,
            accept_partial,
            accept_complete,
            result,
        );
    }
}

/// Compute the closure of an attribute set under functional dependencies.
///
/// Each FD is `(lhs, rhs)`. If all lhs attributes are in the set,
/// all rhs attributes are added. Repeats until fixpoint.
pub(crate) fn compute_closure(
    attrs: &[bool],
    dependencies: &[(Vec<usize>, Vec<usize>)],
) -> Vec<bool> {
    let mut closure = attrs.to_vec();
    let mut changed = true;
    while changed {
        changed = false;
        for (lhs, rhs) in dependencies {
            if lhs.iter().all(|&a| closure[a]) {
                for &a in rhs {
                    if !closure[a] {
                        closure[a] = true;
                        changed = true;
                    }
                }
            }
        }
    }
    closure
}

/// Check whether a set of attributes is a superkey (closure covers all attributes).
pub(crate) fn is_superkey(attrs: &[bool], dependencies: &[(Vec<usize>, Vec<usize>)]) -> bool {
    let closure = compute_closure(attrs, dependencies);
    closure.iter().all(|&v| v)
}

/// Check whether a set of attributes is a minimal key (superkey, and removing
/// any single attribute breaks the superkey property).
pub(crate) fn is_minimal_key(attrs: &[bool], dependencies: &[(Vec<usize>, Vec<usize>)]) -> bool {
    if !is_superkey(attrs, dependencies) {
        return false;
    }
    for i in 0..attrs.len() {
        if attrs[i] {
            let mut reduced = attrs.to_vec();
            reduced[i] = false;
            if is_superkey(&reduced, dependencies) {
                return false;
            }
        }
    }
    true
}

/// Find essential attributes — those that must be in every candidate key.
/// An attribute is essential if removing it from the full set means the
/// closure of the remaining attributes no longer covers everything.
pub(crate) fn find_essential_attributes(
    universe_size: usize,
    dependencies: &[(Vec<usize>, Vec<usize>)],
) -> Vec<usize> {
    let mut essential = Vec::new();
    for i in 0..universe_size {
        let mut without_i = vec![true; universe_size];
        without_i[i] = false;
        let closure = compute_closure(&without_i, dependencies);
        if !closure.iter().all(|&v| v) {
            essential.push(i);
        }
    }
    essential
}

/// Find essential attributes for a restricted attribute set (e.g., relation_attrs
/// in AdditionalKey). An attribute in `target_attrs` is essential if removing it
/// from the full target set means the closure no longer covers all target_attrs.
pub(crate) fn find_essential_attributes_restricted(
    universe_size: usize,
    dependencies: &[(Vec<usize>, Vec<usize>)],
    target_attrs: &[usize],
) -> Vec<usize> {
    let mut essential = Vec::new();
    for &attr in target_attrs {
        let mut without = vec![false; universe_size];
        for &a in target_attrs {
            if a != attr {
                without[a] = true;
            }
        }
        let closure = compute_closure(&without, dependencies);
        if !target_attrs.iter().all(|&a| closure[a]) {
            essential.push(attr);
        }
    }
    essential
}
