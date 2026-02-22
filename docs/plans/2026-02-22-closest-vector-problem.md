# ClosestVectorProblem Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the Closest Vector Problem (CVP) as a new optimization model, following the `add-model` skill steps 1-7.

**Architecture:** CVP is a lattice problem parameterized by element type `T` (i32 or f64). It minimizes ‖Bx - t‖₂ over integer vectors x with explicit bounds per variable. The struct stores the basis matrix, target vector, and variable bounds (reusing `VarBounds` from ILP). Configuration encoding follows the ILP pattern: config indices are offsets from lower bounds.

**Tech Stack:** Rust, serde, inventory crate for schema registration

---

### Task 1: Create the CVP model file with struct and constructor

**Files:**
- Create: `src/models/optimization/closest_vector_problem.rs`

**Step 1: Write the failing test**

Create `src/unit_tests/models/optimization/closest_vector_problem.rs`:

```rust
use super::*;
use crate::models::optimization::VarBounds;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_cvp_creation() {
    // 3D integer lattice: b1=(2,0,0), b2=(1,2,0), b3=(0,1,2)
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);
    assert_eq!(cvp.num_variables(), 3);
    assert_eq!(cvp.ambient_dimension(), 3);
    assert_eq!(cvp.num_basis_vectors(), 3);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_cvp_creation -- --no-capture 2>&1 | tail -20`
Expected: FAIL (module not found)

**Step 3: Write minimal implementation**

Create `src/models/optimization/closest_vector_problem.rs` with:

```rust
use crate::models::optimization::VarBounds;
use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestVectorProblem",
        module_path: module_path!(),
        description: "Find the closest lattice point to a target vector",
        fields: &[
            FieldInfo { name: "basis", type_name: "Vec<Vec<T>>", description: "Basis matrix B as column vectors" },
            FieldInfo { name: "target", type_name: "Vec<f64>", description: "Target vector t" },
            FieldInfo { name: "bounds", type_name: "Vec<VarBounds>", description: "Integer bounds per variable" },
        ],
    }
}

/// Closest Vector Problem (CVP).
///
/// Given a lattice basis B ∈ R^{m×n} and target t ∈ R^m,
/// find integer x ∈ Z^n minimizing ‖Bx - t‖₂.
///
/// Variables are integer coefficients with explicit bounds for enumeration.
/// The configuration encoding follows ILP: config[i] is an offset from bounds[i].lower.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosestVectorProblem<T> {
    /// Basis matrix B stored as n column vectors, each of dimension m.
    basis: Vec<Vec<T>>,
    /// Target vector t ∈ R^m.
    target: Vec<f64>,
    /// Integer bounds per variable for enumeration.
    bounds: Vec<VarBounds>,
}

impl<T> ClosestVectorProblem<T> {
    /// Create a new CVP instance.
    ///
    /// # Arguments
    /// * `basis` - n column vectors of dimension m
    /// * `target` - target vector of dimension m
    /// * `bounds` - integer bounds per variable (length n)
    ///
    /// # Panics
    /// Panics if basis/bounds lengths mismatch or dimensions are inconsistent.
    pub fn new(basis: Vec<Vec<T>>, target: Vec<f64>, bounds: Vec<VarBounds>) -> Self {
        let n = basis.len();
        assert_eq!(bounds.len(), n, "bounds length must match number of basis vectors");
        let m = target.len();
        for (i, col) in basis.iter().enumerate() {
            assert_eq!(col.len(), m, "basis vector {i} has length {}, expected {m}", col.len());
        }
        Self { basis, target, bounds }
    }

    /// Number of basis vectors (lattice dimension n).
    pub fn num_basis_vectors(&self) -> usize {
        self.basis.len()
    }

    /// Dimension of the ambient space (m).
    pub fn ambient_dimension(&self) -> usize {
        self.target.len()
    }

    /// Access the basis matrix.
    pub fn basis(&self) -> &[Vec<T>] {
        &self.basis
    }

    /// Access the target vector.
    pub fn target(&self) -> &[f64] {
        &self.target
    }

    /// Access the variable bounds.
    pub fn bounds(&self) -> &[VarBounds] {
        &self.bounds
    }

    /// Convert a configuration (offsets from lower bounds) to integer values.
    fn config_to_values(&self, config: &[usize]) -> Vec<i64> {
        config
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let lo = self.bounds.get(i).and_then(|b| b.lower).unwrap_or(0);
                lo + c as i64
            })
            .collect()
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_cvp_creation -- --no-capture 2>&1 | tail -20`
Expected: FAIL (not yet registered in mod.rs — will pass after Task 2)

**Step 5: Commit**

```bash
git add src/models/optimization/closest_vector_problem.rs src/unit_tests/models/optimization/closest_vector_problem.rs
git commit -m "feat: add ClosestVectorProblem struct and constructor"
```

---

### Task 2: Implement Problem and OptimizationProblem traits

**Files:**
- Modify: `src/models/optimization/closest_vector_problem.rs`

**Step 1: Write the failing tests**

Add to `src/unit_tests/models/optimization/closest_vector_problem.rs`:

```rust
#[test]
fn test_cvp_evaluate() {
    // b1=(2,0,0), b2=(1,2,0), b3=(0,1,2), target=(3,3,3)
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    // x=(1,1,1) -> Bx=(3,3,2), distance=1.0
    // config offset: x_i - lower = 1 - (-2) = 3
    let config_111 = vec![3, 3, 3]; // maps to x=(1,1,1)
    let result = Problem::evaluate(&cvp, &config_111);
    assert_eq!(result, SolutionSize::Valid(1.0));
}

#[test]
fn test_cvp_direction() {
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(0, 2), VarBounds::bounded(0, 2)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);
    assert_eq!(cvp.direction(), Direction::Minimize);
}

#[test]
fn test_cvp_dims() {
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(-1, 3), VarBounds::bounded(0, 5)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);
    assert_eq!(cvp.dims(), vec![5, 6]); // (-1..3)=5 values, (0..5)=6 values
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test test_cvp -- --no-capture 2>&1 | tail -20`
Expected: FAIL (trait not implemented)

**Step 3: Implement the traits**

Add to `src/models/optimization/closest_vector_problem.rs`, requiring `T: Into<f64> + Clone`:

```rust
impl<T: Clone + Into<f64> + Serialize + for<'de> Deserialize<'de> + std::fmt::Debug + 'static> Problem for ClosestVectorProblem<T> {
    const NAME: &'static str = "ClosestVectorProblem";
    type Metric = SolutionSize<f64>;

    fn dims(&self) -> Vec<usize> {
        self.bounds
            .iter()
            .map(|b| {
                b.num_values().expect(
                    "CVP brute-force enumeration requires all variables to have finite bounds",
                )
            })
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<f64> {
        let values = self.config_to_values(config);
        // Compute Bx - t, then ‖Bx - t‖₂
        let m = self.ambient_dimension();
        let mut diff = vec![0.0f64; m];
        // Bx = sum_i x_i * basis[i]
        for (i, &x_i) in values.iter().enumerate() {
            for (j, b_ji) in self.basis[i].iter().enumerate() {
                diff[j] += x_i as f64 * (*b_ji).clone().into();
            }
        }
        // diff = Bx - t
        for j in 0..m {
            diff[j] -= self.target[j];
        }
        let norm = diff.iter().map(|d| d * d).sum::<f64>().sqrt();
        SolutionSize::Valid(norm)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn problem_size_names() -> &'static [&'static str] {
        &["num_basis_vectors", "ambient_dimension"]
    }

    fn problem_size_values(&self) -> Vec<usize> {
        vec![self.num_basis_vectors(), self.ambient_dimension()]
    }
}

impl<T: Clone + Into<f64> + Serialize + for<'de> Deserialize<'de> + std::fmt::Debug + 'static> OptimizationProblem for ClosestVectorProblem<T> {
    type Value = f64;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/optimization/closest_vector_problem.rs"]
mod tests;
```

**Step 4: Run tests to verify they pass**

Run: `cargo test test_cvp -- --no-capture 2>&1 | tail -20`
Expected: PASS (all 4 tests)

**Step 5: Commit**

```bash
git add src/models/optimization/closest_vector_problem.rs src/unit_tests/models/optimization/closest_vector_problem.rs
git commit -m "feat: implement Problem and OptimizationProblem traits for CVP"
```

---

### Task 3: Register CVP in module exports and prelude

**Files:**
- Modify: `src/models/optimization/mod.rs`
- Modify: `src/models/mod.rs`

**Step 1: Update `src/models/optimization/mod.rs`**

Add module declaration and re-export:

```rust
mod closest_vector_problem;
// add to existing pub use line:
pub use closest_vector_problem::ClosestVectorProblem;
```

**Step 2: Update `src/models/mod.rs`**

Add `ClosestVectorProblem` to the optimization re-export line:

```rust
pub use optimization::{ClosestVectorProblem, SpinGlass, ILP, QUBO};
```

**Step 3: Verify compilation**

Run: `cargo build 2>&1 | tail -20`
Expected: successful compilation

**Step 4: Commit**

```bash
git add src/models/optimization/mod.rs src/models/mod.rs
git commit -m "feat: register ClosestVectorProblem in module exports"
```

---

### Task 4: Register CVP in CLI dispatch

**Files:**
- Modify: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/problem_name.rs`

**Step 1: Update `problemreductions-cli/src/dispatch.rs`**

Add import at top:
```rust
use problemreductions::models::optimization::ClosestVectorProblem;
```

Add match arm in `load_problem()` (after the `"ILP"` arm):
```rust
"ClosestVectorProblem" => deser_opt::<ClosestVectorProblem<i32>>(data),
```

Add match arm in `serialize_any_problem()` (after the `"ILP"` arm):
```rust
"ClosestVectorProblem" => try_ser::<ClosestVectorProblem<i32>>(data),
```

**Step 2: Update `problemreductions-cli/src/problem_name.rs`**

Add alias in `resolve_alias()`:
```rust
"closestvectorproblem" | "cvp" => "ClosestVectorProblem".to_string(),
```

Add to `ALIASES` array:
```rust
("CVP", "ClosestVectorProblem"),
```

**Step 3: Verify CLI builds**

Run: `cargo build -p problemreductions-cli 2>&1 | tail -20`
Expected: successful build

**Step 4: Commit**

```bash
git add problemreductions-cli/src/dispatch.rs problemreductions-cli/src/problem_name.rs
git commit -m "feat: register ClosestVectorProblem in CLI dispatch"
```

---

### Task 5: Write comprehensive unit tests

**Files:**
- Modify: `src/unit_tests/models/optimization/closest_vector_problem.rs`

**Step 1: Add solver and serialization tests**

Append to the test file:

```rust
use crate::solvers::BruteForce;

#[test]
fn test_cvp_brute_force() {
    // b1=(2,0,0), b2=(1,2,0), b3=(0,1,2), target=(3,3,3)
    // Optimal: x=(1,1,1), Bx=(3,3,2), distance=1.0
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-1, 3),
        VarBounds::bounded(-1, 3),
        VarBounds::bounded(-1, 3),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    let solver = BruteForce::new();
    let solution = solver.find_best(&cvp).expect("should find a solution");
    let values: Vec<i64> = solution.iter().enumerate().map(|(i, &c)| {
        cvp.bounds()[i].lower.unwrap() + c as i64
    }).collect();
    assert_eq!(values, vec![1, 1, 1]);
}

#[test]
fn test_cvp_serialization() {
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    let json = serde_json::to_string(&cvp).expect("serialize");
    let cvp2: ClosestVectorProblem<i32> = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(cvp2.num_basis_vectors(), 3);
    assert_eq!(cvp2.ambient_dimension(), 3);
}

#[test]
fn test_cvp_2d_identity() {
    // Identity basis in 2D, target=(0.3, 0.7)
    // Closest: x=(0,1), Bx=(0,1), distance=sqrt(0.09+0.09)=0.3*sqrt(2)
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.3, 0.7];
    let bounds = vec![VarBounds::bounded(-2, 2), VarBounds::bounded(-2, 2)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    let solver = BruteForce::new();
    let solution = solver.find_best(&cvp).expect("should find a solution");
    let values: Vec<i64> = solution.iter().enumerate().map(|(i, &c)| {
        cvp.bounds()[i].lower.unwrap() + c as i64
    }).collect();
    assert_eq!(values, vec![0, 1]);
}

#[test]
fn test_cvp_problem_size() {
    let basis = vec![vec![1, 0, 0], vec![0, 1, 0]]; // 2 vectors in R^3
    let target = vec![0.5, 0.5, 0.5];
    let bounds = vec![VarBounds::bounded(0, 2), VarBounds::bounded(0, 2)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);
    assert_eq!(ClosestVectorProblem::<i32>::problem_size_names(), &["num_basis_vectors", "ambient_dimension"]);
    assert_eq!(cvp.problem_size_values(), vec![2, 3]);
}

#[test]
fn test_cvp_evaluate_exact_solution() {
    // Target is exactly a lattice point: t = (2, 2), basis = identity
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![2.0, 2.0];
    let bounds = vec![VarBounds::bounded(0, 4), VarBounds::bounded(0, 4)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    // x=(2,2), Bx=(2,2), distance=0
    let config = vec![2, 2]; // offset from lower=0
    let result = Problem::evaluate(&cvp, &config);
    assert_eq!(result, SolutionSize::Valid(0.0));
}

#[test]
#[should_panic(expected = "bounds length must match")]
fn test_cvp_mismatched_bounds() {
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(0, 1)]; // only 1 bound for 2 vars
    ClosestVectorProblem::new(basis, target, bounds);
}

#[test]
#[should_panic(expected = "basis vector")]
fn test_cvp_inconsistent_dimensions() {
    let basis = vec![vec![1, 0], vec![0]]; // second vector has wrong dim
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(0, 1), VarBounds::bounded(0, 1)];
    ClosestVectorProblem::new(basis, target, bounds);
}
```

**Step 2: Run all tests**

Run: `cargo test test_cvp -- --no-capture 2>&1 | tail -30`
Expected: PASS (all tests)

**Step 3: Commit**

```bash
git add src/unit_tests/models/optimization/closest_vector_problem.rs
git commit -m "test: add comprehensive CVP unit tests"
```

---

### Task 6: Add paper documentation

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add display name**

Add to the `display-name` dictionary (after the `"BicliqueCover"` entry):

```typst
"ClosestVectorProblem": [Closest Vector Problem],
```

**Step 2: Add problem definition**

Add a `#problem-def` block (after the ILP definition, in the optimization section):

```typst
#problem-def("ClosestVectorProblem")[
  Given a lattice basis $bold(B) in RR^(m times n)$ (columns $bold(b)_1, ..., bold(b)_n in RR^m$ spanning lattice $cal(L)(bold(B)) = {bold(B) bold(x) : bold(x) in ZZ^n}$) and target $bold(t) in RR^m$, find $bold(x) in ZZ^n$ minimizing $norm(bold(B) bold(x) - bold(t))_2$.
]
```

**Step 3: Verify paper builds**

Run: `make doc 2>&1 | tail -10`
Expected: successful build (warnings about missing reductions are OK for a new problem with no reduction rules yet)

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add ClosestVectorProblem definition to paper"
```

---

### Task 7: Final verification

**Step 1: Run full check**

Run: `make test clippy`
Expected: all tests pass, no clippy warnings

**Step 2: Run review-implementation skill**

Verify all structural checks pass for the new model.

**Step 3: Squash or tidy commits if needed**

Ensure all commits are clean and ready for PR.
