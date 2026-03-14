//! Canonical example database assembly.
//!
//! `rule_builders` and `model_builders` are the canonical in-memory sources for
//! all example data. This module assembles, validates, and looks up structural
//! records from those builders — no filesystem round-trip or legacy bridge.

use crate::error::{ProblemError, Result};
use crate::export::{
    examples_output_dir, ModelDb, ModelExample, ProblemRef, RuleDb, RuleExample, EXAMPLE_DB_VERSION,
};
use std::collections::BTreeSet;
use std::path::PathBuf;

mod model_builders;
mod rule_builders;

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

pub fn build_rule_db() -> Result<RuleDb> {
    let mut rules = rule_builders::build_rule_examples();
    rules.sort_by_key(rule_key);
    validate_rule_uniqueness(&rules)?;
    Ok(RuleDb {
        version: EXAMPLE_DB_VERSION,
        rules,
    })
}

pub fn build_model_db() -> Result<ModelDb> {
    let mut models = model_builders::build_model_examples();
    models.sort_by_key(model_key);
    validate_model_uniqueness(&models)?;
    Ok(ModelDb {
        version: EXAMPLE_DB_VERSION,
        models,
    })
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

pub fn default_generated_dir() -> PathBuf {
    examples_output_dir()
}

#[cfg(test)]
#[path = "../unit_tests/example_db.rs"]
mod tests;
