//! Canonical example database assembly.
//!
//! Each model and rule has a canonical example spec that stores both the
//! problem instance and its known optimal solution. The database is computed
//! from these specs on demand — no static fixture file.
//!
//! Model specs are pure data (`Box<dyn DynProblem>` + optimal config + value).
//! Rule specs run `reduce_to()` (fast, no solver) with pre-stored solution pairs.

use crate::error::{ProblemError, Result};
use crate::export::{ExampleDb, ModelDb, ModelExample, ProblemRef, RuleDb, RuleExample};
use std::collections::BTreeSet;

mod model_builders;
mod rule_builders;
pub(crate) mod specs;

fn rule_key(example: &RuleExample) -> (ProblemRef, ProblemRef) {
    (example.source.problem_ref(), example.target.problem_ref())
}

fn model_key(example: &ModelExample) -> ProblemRef {
    example.problem_ref()
}

fn validate_rule_uniqueness(rules: &[RuleExample]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for rule in rules {
        let key = rule_key(rule);
        if !seen.insert(key.clone()) {
            return Err(ProblemError::InvalidProblem(format!(
                "Duplicate canonical rule example for {} {:?} -> {} {:?}",
                key.0.name, key.0.variant, key.1.name, key.1.variant
            )));
        }
    }
    Ok(())
}

fn validate_model_uniqueness(models: &[ModelExample]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for model in models {
        let key = model_key(model);
        if !seen.insert(key.clone()) {
            return Err(ProblemError::InvalidProblem(format!(
                "Duplicate canonical model example for {} {:?}",
                key.name, key.variant
            )));
        }
    }
    Ok(())
}

/// Build the full example database from specs.
///
/// Fast — specs store concrete instances and pre-computed solutions,
/// no solver is called.
pub fn build_example_db() -> Result<ExampleDb> {
    compute_example_db()
}

/// Build the model database from specs.
pub fn build_model_db() -> Result<ModelDb> {
    compute_model_db()
}

/// Build the rule database from specs.
pub fn build_rule_db() -> Result<RuleDb> {
    compute_rule_db()
}

/// Compute the full example database from builder code.
pub fn compute_example_db() -> Result<ExampleDb> {
    let model_db = compute_model_db()?;
    let rule_db = compute_rule_db()?;
    Ok(ExampleDb {
        models: model_db.models,
        rules: rule_db.rules,
    })
}

/// Compute the model database from specs.
pub fn compute_model_db() -> Result<ModelDb> {
    let mut models = model_builders::build_model_examples();
    models.sort_by_key(model_key);
    validate_model_uniqueness(&models)?;
    Ok(ModelDb { models })
}

/// Compute the rule database from specs.
pub fn compute_rule_db() -> Result<RuleDb> {
    let mut rules = rule_builders::build_rule_examples();
    rules.sort_by_key(rule_key);
    validate_rule_uniqueness(&rules)?;
    Ok(RuleDb { rules })
}

pub fn find_rule_example(source: &ProblemRef, target: &ProblemRef) -> Result<RuleExample> {
    let db = build_rule_db()?;
    db.rules
        .into_iter()
        .find(|rule| &rule.source.problem_ref() == source && &rule.target.problem_ref() == target)
        .ok_or_else(|| {
            ProblemError::InvalidProblem(format!(
                "No canonical rule example exists for {} {:?} -> {} {:?}",
                source.name, source.variant, target.name, target.variant
            ))
        })
}

pub fn find_model_example(problem: &ProblemRef) -> Result<ModelExample> {
    let db = build_model_db()?;
    db.models
        .into_iter()
        .find(|model| &model.problem_ref() == problem)
        .ok_or_else(|| {
            ProblemError::InvalidProblem(format!(
                "No canonical model example exists for {} {:?}",
                problem.name, problem.variant
            ))
        })
}
#[cfg(test)]
#[path = "../unit_tests/example_db.rs"]
mod tests;
