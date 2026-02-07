//! Reduction from CircuitSAT to SpinGlass.
//!
//! This module implements the reduction from boolean circuit satisfiability
//! to the Spin Glass (Ising model) problem using logic gadgets.
//!
//! Each logic gate is encoded as a SpinGlass Hamiltonian where the ground
//! states correspond to valid input/output combinations.

use crate::models::optimization::SpinGlass;
use crate::models::specialized::{Assignment, BooleanExpr, BooleanOp, CircuitSAT};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::collections::HashMap;
use std::ops::AddAssign;

/// A logic gadget represented as a SpinGlass problem.
///
/// Each gadget encodes a logic gate where the ground states of the
/// Hamiltonian correspond to valid input/output combinations.
///
/// # References
/// - [What are the cost function for NAND and NOR gates?](https://support.dwavesys.com/hc/en-us/community/posts/1500000470701-What-are-the-cost-function-for-NAND-and-NOR-gates)
/// - Nguyen, M.-T., Liu, J.-G., et al., PRX Quantum 4, 010316 (2023)
#[derive(Debug, Clone)]
pub struct LogicGadget<W> {
    /// The SpinGlass problem encoding the gate.
    pub problem: SpinGlass<SimpleGraph, W>,
    /// Input spin indices (0-indexed within the gadget).
    pub inputs: Vec<usize>,
    /// Output spin indices (0-indexed within the gadget).
    pub outputs: Vec<usize>,
}

impl<W> LogicGadget<W> {
    /// Create a new logic gadget.
    pub fn new(problem: SpinGlass<SimpleGraph, W>, inputs: Vec<usize>, outputs: Vec<usize>) -> Self {
        Self {
            problem,
            inputs,
            outputs,
        }
    }
}

impl<W: Clone + Default> LogicGadget<W> {
    /// Get the number of spins in this gadget.
    pub fn num_spins(&self) -> usize {
        self.problem.num_spins()
    }
}

/// Create an AND gate gadget.
///
/// 3-variable SpinGlass: inputs at indices 0, 1; output at index 2.
/// Ground states: (0,0,0), (0,1,0), (1,0,0), (1,1,1) corresponding to
/// all valid AND truth table entries.
///
/// J = [1, -2, -2] for edges (0,1), (0,2), (1,2)
/// h = [-1, -1, 2] (negated from Julia to account for different spin convention)
///
/// Note: Julia uses config 0 -> spin +1, 1 -> spin -1
///       Rust uses config 0 -> spin -1, 1 -> spin +1
///       So h values are negated to produce equivalent ground states.
pub fn and_gadget<W>() -> LogicGadget<W>
where
    W: Clone + Default + From<i32>,
{
    let interactions = vec![
        ((0, 1), W::from(1)),
        ((0, 2), W::from(-2)),
        ((1, 2), W::from(-2)),
    ];
    let fields = vec![W::from(-1), W::from(-1), W::from(2)];
    let sg = SpinGlass::new(3, interactions, fields);
    LogicGadget::new(sg, vec![0, 1], vec![2])
}

/// Create an OR gate gadget.
///
/// 3-variable SpinGlass: inputs at indices 0, 1; output at index 2.
/// Ground states: (0,0,0), (0,1,1), (1,0,1), (1,1,1) corresponding to
/// all valid OR truth table entries.
///
/// J = [1, -2, -2] for edges (0,1), (0,2), (1,2)
/// h = [1, 1, -2] (negated from Julia to account for different spin convention)
pub fn or_gadget<W>() -> LogicGadget<W>
where
    W: Clone + Default + From<i32>,
{
    let interactions = vec![
        ((0, 1), W::from(1)),
        ((0, 2), W::from(-2)),
        ((1, 2), W::from(-2)),
    ];
    let fields = vec![W::from(1), W::from(1), W::from(-2)];
    let sg = SpinGlass::new(3, interactions, fields);
    LogicGadget::new(sg, vec![0, 1], vec![2])
}

/// Create a NOT gate gadget.
///
/// 2-variable SpinGlass: input at index 0; output at index 1.
/// Ground states: (0,1), (1,0) corresponding to valid NOT.
///
/// J = [1] for edge (0,1)
/// h = [0, 0]
pub fn not_gadget<W>() -> LogicGadget<W>
where
    W: Clone + Default + From<i32> + Zero,
{
    let interactions = vec![((0, 1), W::from(1))];
    let fields = vec![W::zero(), W::zero()];
    let sg = SpinGlass::new(2, interactions, fields);
    LogicGadget::new(sg, vec![0], vec![1])
}

/// Create an XOR gate gadget.
///
/// 4-variable SpinGlass: inputs at indices 0, 1; output at 2; auxiliary at 3.
/// Ground states correspond to valid XOR truth table entries.
///
/// J = [1, -1, -2, -1, -2, 2] for edges (0,1), (0,2), (0,3), (1,2), (1,3), (2,3)
/// h = [-1, -1, 1, 2] (negated from Julia to account for different spin convention)
pub fn xor_gadget<W>() -> LogicGadget<W>
where
    W: Clone + Default + From<i32>,
{
    let interactions = vec![
        ((0, 1), W::from(1)),
        ((0, 2), W::from(-1)),
        ((0, 3), W::from(-2)),
        ((1, 2), W::from(-1)),
        ((1, 3), W::from(-2)),
        ((2, 3), W::from(2)),
    ];
    let fields = vec![W::from(-1), W::from(-1), W::from(1), W::from(2)];
    let sg = SpinGlass::new(4, interactions, fields);
    // Note: output is at index 2 (not 3) according to Julia code
    // The Julia code has: LogicGadget(sg, [1, 2], [3]) which is 1-indexed
    // In 0-indexed: inputs [0, 1], output [2]
    LogicGadget::new(sg, vec![0, 1], vec![2])
}

/// Create a SET0 gadget (constant false).
///
/// 1-variable SpinGlass that prefers config 0 (spin -1 in Rust convention).
/// h = [1] (negated from Julia's [-1] to account for different spin convention)
pub fn set0_gadget<W>() -> LogicGadget<W>
where
    W: Clone + Default + From<i32>,
{
    let interactions = vec![];
    let fields = vec![W::from(1)];
    let sg = SpinGlass::new(1, interactions, fields);
    LogicGadget::new(sg, vec![], vec![0])
}

/// Create a SET1 gadget (constant true).
///
/// 1-variable SpinGlass that prefers config 1 (spin +1 in Rust convention).
/// h = [-1] (negated from Julia's [1] to account for different spin convention)
pub fn set1_gadget<W>() -> LogicGadget<W>
where
    W: Clone + Default + From<i32>,
{
    let interactions = vec![];
    let fields = vec![W::from(-1)];
    let sg = SpinGlass::new(1, interactions, fields);
    LogicGadget::new(sg, vec![], vec![0])
}

/// Result of reducing CircuitSAT to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionCircuitToSG<W> {
    /// The target SpinGlass problem.
    target: SpinGlass<SimpleGraph, W>,
    /// Mapping from source variable names to spin indices.
    variable_map: HashMap<String, usize>,
    /// Source variable names in order.
    source_variables: Vec<String>,
    /// Source problem size.
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionCircuitToSG<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Source = CircuitSAT<W>;
    type Target = SpinGlass<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.source_variables
            .iter()
            .map(|var| {
                self.variable_map
                    .get(var)
                    .and_then(|&idx| target_solution.get(idx).copied())
                    .unwrap_or(0)
            })
            .collect()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

/// Builder for constructing the combined SpinGlass from circuit gadgets.
struct SpinGlassBuilder<W> {
    /// Current number of spins.
    num_spins: usize,
    /// Accumulated interactions.
    interactions: HashMap<(usize, usize), W>,
    /// Accumulated fields.
    fields: Vec<W>,
    /// Variable name to spin index mapping.
    variable_map: HashMap<String, usize>,
}

impl<W> SpinGlassBuilder<W>
where
    W: Clone + Default + Zero + AddAssign + From<i32>,
{
    fn new() -> Self {
        Self {
            num_spins: 0,
            interactions: HashMap::new(),
            fields: Vec::new(),
            variable_map: HashMap::new(),
        }
    }

    /// Allocate a new spin and return its index.
    fn allocate_spin(&mut self) -> usize {
        let idx = self.num_spins;
        self.num_spins += 1;
        self.fields.push(W::zero());
        idx
    }

    /// Get or create a spin index for a variable.
    fn get_or_create_variable(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.variable_map.get(name) {
            idx
        } else {
            let idx = self.allocate_spin();
            self.variable_map.insert(name.to_string(), idx);
            idx
        }
    }

    /// Add a gadget to the builder with the given spin mapping.
    fn add_gadget(&mut self, gadget: &LogicGadget<W>, spin_map: &[usize]) {
        // Add interactions
        for ((i, j), weight) in gadget.problem.interactions() {
            let global_i = spin_map[i];
            let global_j = spin_map[j];
            let key = if global_i < global_j {
                (global_i, global_j)
            } else {
                (global_j, global_i)
            };
            self.interactions
                .entry(key)
                .or_insert_with(W::zero)
                .add_assign(weight.clone());
        }

        // Add fields
        for (local_idx, field) in gadget.problem.fields().iter().enumerate() {
            let global_idx = spin_map[local_idx];
            self.fields[global_idx].add_assign(field.clone());
        }
    }

    /// Build the final SpinGlass.
    fn build(self) -> (SpinGlass<SimpleGraph, W>, HashMap<String, usize>) {
        let interactions: Vec<((usize, usize), W)> = self.interactions.into_iter().collect();
        let sg = SpinGlass::new(self.num_spins, interactions, self.fields);
        (sg, self.variable_map)
    }
}

/// Process a boolean expression and return the spin index of its output.
fn process_expression<W>(expr: &BooleanExpr, builder: &mut SpinGlassBuilder<W>) -> usize
where
    W: Clone + Default + Zero + AddAssign + From<i32>,
{
    match &expr.op {
        BooleanOp::Var(name) => builder.get_or_create_variable(name),

        BooleanOp::Const(value) => {
            let gadget: LogicGadget<W> = if *value { set1_gadget() } else { set0_gadget() };
            let output_spin = builder.allocate_spin();
            let spin_map = vec![output_spin];
            builder.add_gadget(&gadget, &spin_map);
            output_spin
        }

        BooleanOp::Not(inner) => {
            let input_spin = process_expression(inner, builder);
            let gadget: LogicGadget<W> = not_gadget();
            let output_spin = builder.allocate_spin();
            let spin_map = vec![input_spin, output_spin];
            builder.add_gadget(&gadget, &spin_map);
            output_spin
        }

        BooleanOp::And(args) => process_binary_chain(args, builder, and_gadget),

        BooleanOp::Or(args) => process_binary_chain(args, builder, or_gadget),

        BooleanOp::Xor(args) => process_binary_chain(args, builder, xor_gadget),
    }
}

/// Process a multi-input gate by chaining binary gates.
fn process_binary_chain<W, F>(
    args: &[BooleanExpr],
    builder: &mut SpinGlassBuilder<W>,
    gadget_fn: F,
) -> usize
where
    W: Clone + Default + Zero + AddAssign + From<i32>,
    F: Fn() -> LogicGadget<W>,
{
    assert!(
        !args.is_empty(),
        "Binary gate must have at least one argument"
    );

    if args.len() == 1 {
        // Single argument - just return its output
        return process_expression(&args[0], builder);
    }

    // Process first two arguments
    let mut result_spin = {
        let input0 = process_expression(&args[0], builder);
        let input1 = process_expression(&args[1], builder);
        let gadget = gadget_fn();
        let output_spin = builder.allocate_spin();

        // For XOR gadget, we need to allocate the auxiliary spin too
        let spin_map = if gadget.num_spins() == 4 {
            // XOR: inputs [0, 1], aux at 3, output at 2
            let aux_spin = builder.allocate_spin();
            vec![input0, input1, output_spin, aux_spin]
        } else {
            // AND/OR: inputs [0, 1], output at 2
            vec![input0, input1, output_spin]
        };

        builder.add_gadget(&gadget, &spin_map);
        output_spin
    };

    // Chain remaining arguments
    for arg in args.iter().skip(2) {
        let next_input = process_expression(arg, builder);
        let gadget = gadget_fn();
        let output_spin = builder.allocate_spin();

        let spin_map = if gadget.num_spins() == 4 {
            let aux_spin = builder.allocate_spin();
            vec![result_spin, next_input, output_spin, aux_spin]
        } else {
            vec![result_spin, next_input, output_spin]
        };

        builder.add_gadget(&gadget, &spin_map);
        result_spin = output_spin;
    }

    result_spin
}

/// Process a circuit assignment.
fn process_assignment<W>(assignment: &Assignment, builder: &mut SpinGlassBuilder<W>)
where
    W: Clone + Default + Zero + AddAssign + From<i32>,
{
    // Process the expression to get the output spin
    let expr_output = process_expression(&assignment.expr, builder);

    // For each output variable, we need to constrain it to equal the expression output
    // This is done by adding a NOT gadget constraint (with J=1) to enforce equality
    for output_name in &assignment.outputs {
        let output_spin = builder.get_or_create_variable(output_name);

        // If the output spin is different from expr_output, add equality constraint
        if output_spin != expr_output {
            // Add ferromagnetic coupling to enforce s_i = s_j
            // J = -1 means aligned spins have lower energy
            // Actually, we want to use a strong negative coupling
            let key = if output_spin < expr_output {
                (output_spin, expr_output)
            } else {
                (expr_output, output_spin)
            };
            builder
                .interactions
                .entry(key)
                .or_insert_with(W::zero)
                .add_assign(W::from(-4)); // Strong ferromagnetic coupling
        }
    }
}

#[reduction(
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_spins", poly!(num_assignments)),
            ("num_interactions", poly!(num_assignments)),
        ])
    }
)]
impl<W> ReduceTo<SpinGlass<SimpleGraph, W>> for CircuitSAT<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionCircuitToSG<W>;

    fn reduce_to(&self) -> Self::Result {
        let mut builder = SpinGlassBuilder::new();

        // Process each assignment in the circuit
        for assignment in &self.circuit().assignments {
            process_assignment(assignment, &mut builder);
        }

        let (target, variable_map) = builder.build();
        let source_variables = self.variable_names().to_vec();
        let source_size = self.problem_size();

        ReductionCircuitToSG {
            target,
            variable_map,
            source_variables,
            source_size,
        }
    }
}

#[cfg(test)]
#[path = "../tests_unit/rules/circuit_spinglass.rs"]
mod tests;
