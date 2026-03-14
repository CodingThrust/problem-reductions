//! Canonical example database assembly.
//!
//! This module currently builds the canonical `RuleDb` through a temporary
//! compatibility bridge that reuses the legacy `examples/reduction_*.rs`
//! exporters. The intended end state is pure in-memory builders with no
//! filesystem round-trip.

use crate::error::{ProblemError, Result};
use crate::export::{
    examples_output_dir, ModelDb, ModelExample, ProblemRef, RuleDb, RuleExample, EXAMPLE_DB_VERSION,
};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

mod model_builders;
mod rule_builders;

struct LegacyRuleEntry {
    file_stem: &'static str,
    run: fn(),
}

macro_rules! legacy_rule {
    ($name:ident) => {
        #[allow(dead_code)]
        mod $name {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/",
                stringify!($name),
                ".rs"
            ));
        }
    };
}

legacy_rule!(reduction_binpacking_to_ilp);
legacy_rule!(reduction_circuitsat_to_ilp);
legacy_rule!(reduction_circuitsat_to_spinglass);
legacy_rule!(reduction_factoring_to_circuitsat);
legacy_rule!(reduction_factoring_to_ilp);
legacy_rule!(reduction_ilp_to_qubo);
legacy_rule!(reduction_kcoloring_to_ilp);
legacy_rule!(reduction_kcoloring_to_qubo);
legacy_rule!(reduction_ksatisfiability_to_qubo);
legacy_rule!(reduction_ksatisfiability_to_satisfiability);
legacy_rule!(reduction_ksatisfiability_to_subsetsum);
legacy_rule!(reduction_longestcommonsubsequence_to_ilp);
legacy_rule!(reduction_maxcut_to_spinglass);
legacy_rule!(reduction_maximumclique_to_ilp);
legacy_rule!(reduction_maximumclique_to_maximumindependentset);
legacy_rule!(reduction_maximumindependentset_to_ilp);
legacy_rule!(reduction_maximumindependentset_to_maximumclique);
legacy_rule!(reduction_maximumindependentset_to_maximumsetpacking);
legacy_rule!(reduction_maximumindependentset_to_minimumvertexcover);
legacy_rule!(reduction_maximumindependentset_to_qubo);
legacy_rule!(reduction_maximummatching_to_ilp);
legacy_rule!(reduction_maximummatching_to_maximumsetpacking);
legacy_rule!(reduction_maximumsetpacking_to_ilp);
legacy_rule!(reduction_maximumsetpacking_to_maximumindependentset);
legacy_rule!(reduction_maximumsetpacking_to_qubo);
legacy_rule!(reduction_minimumdominatingset_to_ilp);
legacy_rule!(reduction_minimumsetcovering_to_ilp);
legacy_rule!(reduction_minimumvertexcover_to_ilp);
legacy_rule!(reduction_minimumvertexcover_to_maximumindependentset);
legacy_rule!(reduction_minimumvertexcover_to_minimumsetcovering);
legacy_rule!(reduction_minimumvertexcover_to_qubo);
legacy_rule!(reduction_qubo_to_ilp);
legacy_rule!(reduction_qubo_to_spinglass);
legacy_rule!(reduction_satisfiability_to_circuitsat);
legacy_rule!(reduction_satisfiability_to_kcoloring);
legacy_rule!(reduction_satisfiability_to_ksatisfiability);
legacy_rule!(reduction_satisfiability_to_maximumindependentset);
legacy_rule!(reduction_satisfiability_to_minimumdominatingset);
legacy_rule!(reduction_spinglass_to_maxcut);
legacy_rule!(reduction_spinglass_to_qubo);
legacy_rule!(reduction_travelingsalesman_to_ilp);
legacy_rule!(reduction_travelingsalesman_to_qubo);

const LEGACY_RULES: &[LegacyRuleEntry] = &[
    LegacyRuleEntry {
        file_stem: "binpacking_to_ilp",
        run: reduction_binpacking_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "circuitsat_to_ilp",
        run: reduction_circuitsat_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "circuitsat_to_spinglass",
        run: reduction_circuitsat_to_spinglass::run,
    },
    LegacyRuleEntry {
        file_stem: "factoring_to_circuitsat",
        run: reduction_factoring_to_circuitsat::run,
    },
    LegacyRuleEntry {
        file_stem: "factoring_to_ilp",
        run: reduction_factoring_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "ilp_to_qubo",
        run: reduction_ilp_to_qubo::run,
    },
    LegacyRuleEntry {
        file_stem: "kcoloring_to_ilp",
        run: reduction_kcoloring_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "kcoloring_to_qubo",
        run: reduction_kcoloring_to_qubo::run,
    },
    LegacyRuleEntry {
        file_stem: "ksatisfiability_to_qubo",
        run: reduction_ksatisfiability_to_qubo::run,
    },
    LegacyRuleEntry {
        file_stem: "ksatisfiability_to_satisfiability",
        run: reduction_ksatisfiability_to_satisfiability::run,
    },
    LegacyRuleEntry {
        file_stem: "ksatisfiability_to_subsetsum",
        run: reduction_ksatisfiability_to_subsetsum::run,
    },
    LegacyRuleEntry {
        file_stem: "longestcommonsubsequence_to_ilp",
        run: reduction_longestcommonsubsequence_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "maxcut_to_spinglass",
        run: reduction_maxcut_to_spinglass::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumclique_to_ilp",
        run: reduction_maximumclique_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumclique_to_maximumindependentset",
        run: reduction_maximumclique_to_maximumindependentset::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumindependentset_to_ilp",
        run: reduction_maximumindependentset_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumindependentset_to_maximumclique",
        run: reduction_maximumindependentset_to_maximumclique::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumindependentset_to_maximumsetpacking",
        run: reduction_maximumindependentset_to_maximumsetpacking::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumindependentset_to_minimumvertexcover",
        run: reduction_maximumindependentset_to_minimumvertexcover::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumindependentset_to_qubo",
        run: reduction_maximumindependentset_to_qubo::run,
    },
    LegacyRuleEntry {
        file_stem: "maximummatching_to_ilp",
        run: reduction_maximummatching_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "maximummatching_to_maximumsetpacking",
        run: reduction_maximummatching_to_maximumsetpacking::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumsetpacking_to_ilp",
        run: reduction_maximumsetpacking_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumsetpacking_to_maximumindependentset",
        run: reduction_maximumsetpacking_to_maximumindependentset::run,
    },
    LegacyRuleEntry {
        file_stem: "maximumsetpacking_to_qubo",
        run: reduction_maximumsetpacking_to_qubo::run,
    },
    LegacyRuleEntry {
        file_stem: "minimumdominatingset_to_ilp",
        run: reduction_minimumdominatingset_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "minimumsetcovering_to_ilp",
        run: reduction_minimumsetcovering_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "minimumvertexcover_to_ilp",
        run: reduction_minimumvertexcover_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "minimumvertexcover_to_maximumindependentset",
        run: reduction_minimumvertexcover_to_maximumindependentset::run,
    },
    LegacyRuleEntry {
        file_stem: "minimumvertexcover_to_minimumsetcovering",
        run: reduction_minimumvertexcover_to_minimumsetcovering::run,
    },
    LegacyRuleEntry {
        file_stem: "minimumvertexcover_to_qubo",
        run: reduction_minimumvertexcover_to_qubo::run,
    },
    LegacyRuleEntry {
        file_stem: "qubo_to_ilp",
        run: reduction_qubo_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "qubo_to_spinglass",
        run: reduction_qubo_to_spinglass::run,
    },
    LegacyRuleEntry {
        file_stem: "satisfiability_to_circuitsat",
        run: reduction_satisfiability_to_circuitsat::run,
    },
    LegacyRuleEntry {
        file_stem: "satisfiability_to_kcoloring",
        run: reduction_satisfiability_to_kcoloring::run,
    },
    LegacyRuleEntry {
        file_stem: "satisfiability_to_ksatisfiability",
        run: reduction_satisfiability_to_ksatisfiability::run,
    },
    LegacyRuleEntry {
        file_stem: "satisfiability_to_maximumindependentset",
        run: reduction_satisfiability_to_maximumindependentset::run,
    },
    LegacyRuleEntry {
        file_stem: "satisfiability_to_minimumdominatingset",
        run: reduction_satisfiability_to_minimumdominatingset::run,
    },
    LegacyRuleEntry {
        file_stem: "spinglass_to_maxcut",
        run: reduction_spinglass_to_maxcut::run,
    },
    LegacyRuleEntry {
        file_stem: "spinglass_to_qubo",
        run: reduction_spinglass_to_qubo::run,
    },
    LegacyRuleEntry {
        file_stem: "travelingsalesman_to_ilp",
        run: reduction_travelingsalesman_to_ilp::run,
    },
    LegacyRuleEntry {
        file_stem: "travelingsalesman_to_qubo",
        run: reduction_travelingsalesman_to_qubo::run,
    },
];

static BUILD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn build_lock() -> &'static Mutex<()> {
    BUILD_LOCK.get_or_init(|| Mutex::new(()))
}

fn unique_temp_dir(file_stem: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "problemreductions-example-db-{}-{}-{}",
        file_stem,
        std::process::id(),
        nanos
    ))
}

struct EnvVarGuard {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &std::path::Path) -> Self {
        let previous = std::env::var_os(key);
        std::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            std::env::set_var(self.key, previous);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

#[cfg(unix)]
struct StdoutSilencer {
    saved_fd: std::os::fd::OwnedFd,
}

#[cfg(unix)]
impl StdoutSilencer {
    fn new() -> Result<Self> {
        use std::fs::File;
        use std::os::fd::{AsRawFd, FromRawFd};

        unsafe extern "C" {
            fn dup(oldfd: i32) -> i32;
            fn dup2(oldfd: i32, newfd: i32) -> i32;
            fn close(fd: i32) -> i32;
        }

        std::io::stdout()
            .flush()
            .map_err(|e| ProblemError::IoError(e.to_string()))?;

        let saved = unsafe { dup(1) };
        if saved < 0 {
            return Err(ProblemError::IoError(
                "Failed to duplicate stdout".to_string(),
            ));
        }

        let dev_null = File::options()
            .write(true)
            .open("/dev/null")
            .map_err(|e| ProblemError::IoError(e.to_string()))?;

        if unsafe { dup2(dev_null.as_raw_fd(), 1) } < 0 {
            unsafe {
                close(saved);
            }
            return Err(ProblemError::IoError(
                "Failed to redirect stdout".to_string(),
            ));
        }

        Ok(Self {
            saved_fd: unsafe { std::os::fd::OwnedFd::from_raw_fd(saved) },
        })
    }
}

#[cfg(unix)]
impl Drop for StdoutSilencer {
    fn drop(&mut self) {
        use std::os::fd::AsRawFd;

        unsafe extern "C" {
            fn dup2(oldfd: i32, newfd: i32) -> i32;
        }

        let _ = std::io::stdout().flush();
        let _ = unsafe { dup2(self.saved_fd.as_raw_fd(), 1) };
    }
}

#[cfg(not(unix))]
struct StdoutSilencer;

#[cfg(not(unix))]
impl StdoutSilencer {
    fn new() -> Result<Self> {
        Ok(Self)
    }
}

fn build_legacy_rule(entry: &LegacyRuleEntry) -> Result<RuleExample> {
    let _guard = build_lock().lock().expect("example build mutex poisoned");
    let dir = unique_temp_dir(entry.file_stem);
    fs::create_dir_all(&dir).map_err(|e| ProblemError::IoError(e.to_string()))?;
    let _env_guard = EnvVarGuard::set(crate::export::EXAMPLES_DIR_ENV, &dir);
    let _stdout_guard = StdoutSilencer::new()?;

    (entry.run)();

    let path = dir.join(format!("{}.json", entry.file_stem));
    let json = fs::read_to_string(&path).map_err(|e| ProblemError::IoError(e.to_string()))?;
    let example =
        serde_json::from_str(&json).map_err(|e| ProblemError::SerializationError(e.to_string()))?;
    let _ = fs::remove_dir_all(&dir);
    Ok(example)
}

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
