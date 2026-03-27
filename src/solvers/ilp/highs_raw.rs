//! Thin safe wrapper around the `highs-sys` C API.
//!
//! This module provides a minimal safe Rust interface to the HiGHS solver,
//! calling the C API directly without intermediate Rust wrapper crates.

use std::ffi::CString;
use std::os::raw::c_int;

/// Status returned by HiGHS C API calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HiGHSStatus {
    Ok,
    Warning,
    Error,
}

impl HiGHSStatus {
    fn from_raw(status: c_int) -> Self {
        match status {
            highs_sys::STATUS_OK => Self::Ok,
            highs_sys::STATUS_WARNING => Self::Warning,
            _ => Self::Error,
        }
    }

    pub(crate) fn is_err(self) -> bool {
        self == Self::Error
    }
}

/// Model status after solving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModelStatus {
    Optimal,
    Infeasible,
    Unbounded,
    TimeLimitOrOther,
    Error,
}

impl ModelStatus {
    fn from_raw(status: c_int) -> Self {
        match status {
            highs_sys::MODEL_STATUS_OPTIMAL
            | highs_sys::MODEL_STATUS_OBJECTIVE_BOUND
            | highs_sys::MODEL_STATUS_OBJECTIVE_TARGET => Self::Optimal,

            highs_sys::MODEL_STATUS_INFEASIBLE
            | highs_sys::MODEL_STATUS_UNBOUNDED_OR_INFEASIBLE => Self::Infeasible,

            highs_sys::MODEL_STATUS_UNBOUNDED => Self::Unbounded,

            highs_sys::MODEL_STATUS_REACHED_TIME_LIMIT
            | highs_sys::MODEL_STATUS_REACHED_ITERATION_LIMIT
            | highs_sys::MODEL_STATUS_REACHED_SOLUTION_LIMIT
            | highs_sys::MODEL_STATUS_REACHED_INTERRUPT
            | highs_sys::MODEL_STATUS_REACHED_MEMORY_LIMIT => Self::TimeLimitOrOther,

            _ => Self::Error,
        }
    }
}

/// Primal solution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SolutionStatus {
    None,
    Infeasible,
    Feasible,
}

impl SolutionStatus {
    fn from_raw(status: c_int) -> Self {
        match status {
            highs_sys::SOLUTION_STATUS_FEASIBLE => Self::Feasible,
            highs_sys::SOLUTION_STATUS_INFEASIBLE => Self::Infeasible,
            _ => Self::None,
        }
    }
}

/// Assert that a HiGHS C API call did not return an error.
fn assert_option_ok(status: c_int, option: &str) {
    debug_assert!(
        status != highs_sys::STATUS_ERROR,
        "HiGHS option '{option}' failed with error status"
    );
}

/// RAII handle to a HiGHS model instance.
pub(crate) struct HiGHSModel {
    ptr: *mut std::ffi::c_void,
}

impl Drop for HiGHSModel {
    fn drop(&mut self) {
        unsafe { highs_sys::Highs_destroy(self.ptr) }
    }
}

impl HiGHSModel {
    /// Create a new HiGHS model with output suppressed.
    pub(crate) fn new() -> Self {
        let ptr = unsafe { highs_sys::Highs_create() };
        let mut model = Self { ptr };
        model.set_bool_option("output_flag", false);
        model.set_bool_option("log_to_console", false);
        model
    }

    /// Pass a full MIP problem to HiGHS using column-wise sparse matrix format.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn pass_mip(
        &mut self,
        num_vars: usize,
        num_constraints: usize,
        sense: c_int,
        col_cost: &[f64],
        col_lower: &[f64],
        col_upper: &[f64],
        row_lower: &[f64],
        row_upper: &[f64],
        a_start: &[c_int],
        a_index: &[c_int],
        a_value: &[f64],
        integrality: &[c_int],
    ) -> HiGHSStatus {
        let nnz = a_value.len();
        let status = unsafe {
            highs_sys::Highs_passMip(
                self.ptr,
                num_vars as c_int,
                num_constraints as c_int,
                nnz as c_int,
                highs_sys::MATRIX_FORMAT_COLUMN_WISE,
                sense,
                0.0, // offset
                col_cost.as_ptr(),
                col_lower.as_ptr(),
                col_upper.as_ptr(),
                row_lower.as_ptr(),
                row_upper.as_ptr(),
                a_start.as_ptr(),
                a_index.as_ptr(),
                a_value.as_ptr(),
                integrality.as_ptr(),
            )
        };
        HiGHSStatus::from_raw(status)
    }

    /// Set a boolean option.
    pub(crate) fn set_bool_option(&mut self, name: &str, value: bool) {
        let c_name = CString::new(name).expect("invalid option name");
        let status = unsafe {
            highs_sys::Highs_setBoolOptionValue(
                self.ptr,
                c_name.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
        assert_option_ok(status, name);
    }

    /// Set an integer option.
    pub(crate) fn set_int_option(&mut self, name: &str, value: i32) {
        let c_name = CString::new(name).expect("invalid option name");
        let status =
            unsafe { highs_sys::Highs_setIntOptionValue(self.ptr, c_name.as_ptr(), value) };
        assert_option_ok(status, name);
    }

    /// Set a double option.
    pub(crate) fn set_double_option(&mut self, name: &str, value: f64) {
        let c_name = CString::new(name).expect("invalid option name");
        let status =
            unsafe { highs_sys::Highs_setDoubleOptionValue(self.ptr, c_name.as_ptr(), value) };
        assert_option_ok(status, name);
    }

    /// Set a string option.
    pub(crate) fn set_string_option(&mut self, name: &str, value: &str) {
        let c_name = CString::new(name).expect("invalid option name");
        let c_value = CString::new(value).expect("invalid option value");
        let status = unsafe {
            highs_sys::Highs_setStringOptionValue(self.ptr, c_name.as_ptr(), c_value.as_ptr())
        };
        assert_option_ok(status, name);
    }

    /// Run the solver.
    pub(crate) fn solve(&mut self) -> HiGHSStatus {
        let status = unsafe { highs_sys::Highs_run(self.ptr) };
        HiGHSStatus::from_raw(status)
    }

    /// Get the model status after solving.
    pub(crate) fn model_status(&self) -> ModelStatus {
        let status = unsafe { highs_sys::Highs_getModelStatus(self.ptr) };
        ModelStatus::from_raw(status)
    }

    /// Get the primal solution status.
    pub(crate) fn primal_solution_status(&self) -> SolutionStatus {
        let c_name = CString::new("primal_solution_status").unwrap();
        let mut value: c_int = -1;
        unsafe {
            highs_sys::Highs_getIntInfoValue(self.ptr, c_name.as_ptr(), &mut value);
        }
        SolutionStatus::from_raw(value)
    }

    /// Extract column (variable) values from the solution.
    pub(crate) fn solution_values(&self, num_vars: usize) -> Vec<f64> {
        let mut col_value = vec![0.0f64; num_vars];
        // HiGHS C API null-checks each pointer; pass null for unused arrays.
        unsafe {
            highs_sys::Highs_getSolution(
                self.ptr,
                col_value.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
        col_value
    }
}
