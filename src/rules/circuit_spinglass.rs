//! Reduction from CircuitSAT to SpinGlass.
//!
//! This module implements the reduction from boolean circuit satisfiability
//! to the Spin Glass (Ising model) problem using logic gadgets.
//!
//! Each logic gate is encoded as a SpinGlass Hamiltonian where the ground
//! states correspond to valid input/output combinations.

use crate::models::formula::{Assignment, BooleanExpr, BooleanOp, CircuitSAT};
use crate::models::graph::SpinGlass;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::WeightElement;
use num_traits::Zero;
use std::collections::HashMap;
#[cfg(test)]
use std::ops::AddAssign;

type BuiltSpinGlass = (SpinGlass<SimpleGraph, i64>, HashMap<String, usize>);

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
    #[allow(dead_code)] // read in tests
    pub inputs: Vec<usize>,
    /// Output spin indices (0-indexed within the gadget).
    #[allow(dead_code)] // read in tests
    pub outputs: Vec<usize>,
}

impl<W> LogicGadget<W> {
    /// Create a new logic gadget.
    pub fn new(
        problem: SpinGlass<SimpleGraph, W>,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
    ) -> Self {
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
    W: WeightElement + From<i64>,
{
    let interactions = vec![
        ((0, 1), W::from(1)),
        ((0, 2), W::from(-2)),
        ((1, 2), W::from(-2)),
    ];
    let fields = vec![W::from(-1), W::from(-1), W::from(2)];
    let sg = SpinGlass::new(3, interactions, fields);
    LogicGadget::new(
        sg.expect("static AND gadget must be valid"),
        vec![0, 1],
        vec![2],
    )
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
    W: WeightElement + From<i64>,
{
    let interactions = vec![
        ((0, 1), W::from(1)),
        ((0, 2), W::from(-2)),
        ((1, 2), W::from(-2)),
    ];
    let fields = vec![W::from(1), W::from(1), W::from(-2)];
    let sg = SpinGlass::new(3, interactions, fields);
    LogicGadget::new(
        sg.expect("static OR gadget must be valid"),
        vec![0, 1],
        vec![2],
    )
}

/// Create a NOT gate gadget.
///
/// 2-variable SpinGlass: input at index 0; output at index 1.
/// Ground states: (0,1), (1,0) corresponding to valid NOT.
///
/// J = \[1\] for edge (0,1)
/// h = \[0, 0\]
pub fn not_gadget<W>() -> LogicGadget<W>
where
    W: WeightElement + From<i64> + Zero,
{
    let interactions = vec![((0, 1), W::from(1))];
    let fields = vec![W::zero(), W::zero()];
    let sg = SpinGlass::new(2, interactions, fields);
    LogicGadget::new(
        sg.expect("static NOT gadget must be valid"),
        vec![0],
        vec![1],
    )
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
    W: WeightElement + From<i64>,
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
    LogicGadget::new(
        sg.expect("static XOR gadget must be valid"),
        vec![0, 1],
        vec![2],
    )
}

/// Create a SET0 gadget (constant false).
///
/// 1-variable SpinGlass that prefers config 0 (spin -1 in Rust convention).
/// h = \[1\] (negated from Julia's \[-1\] to account for different spin convention)
pub fn set0_gadget<W>() -> LogicGadget<W>
where
    W: WeightElement + From<i64>,
{
    let interactions = vec![];
    let fields = vec![W::from(1)];
    let sg = SpinGlass::new(1, interactions, fields);
    LogicGadget::new(
        sg.expect("static SET0 gadget must be valid"),
        vec![],
        vec![0],
    )
}

/// Create a SET1 gadget (constant true).
///
/// 1-variable SpinGlass that prefers config 1 (spin +1 in Rust convention).
/// h = \[-1\] (negated from Julia's \[1\] to account for different spin convention)
pub fn set1_gadget<W>() -> LogicGadget<W>
where
    W: WeightElement + From<i64>,
{
    let interactions = vec![];
    let fields = vec![W::from(-1)];
    let sg = SpinGlass::new(1, interactions, fields);
    LogicGadget::new(
        sg.expect("static SET1 gadget must be valid"),
        vec![],
        vec![0],
    )
}

/// Result of reducing CircuitSAT to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionCircuitToSG {
    /// The target SpinGlass problem.
    target: SpinGlass<SimpleGraph, i64>,
    /// Mapping from source variable names to spin indices.
    variable_map: HashMap<String, usize>,
    /// Source variable names in order.
    source_variables: Vec<String>,
}

impl ReductionResult for ReductionCircuitToSG {
    type Source = CircuitSAT;
    type Target = SpinGlass<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(self
            .source_variables
            .iter()
            .map(|variable| target_solution[self.variable_map[variable]] == 1)
            .collect())
    }
}

/// Builder for constructing the combined SpinGlass from circuit gadgets.
struct SpinGlassBuilder {
    /// Current number of spins.
    num_spins: usize,
    /// Accumulated interactions.
    interactions: HashMap<(usize, usize), i64>,
    /// Accumulated fields.
    fields: Vec<i64>,
    /// Variable name to spin index mapping.
    variable_map: HashMap<String, usize>,
}

impl SpinGlassBuilder {
    fn new() -> Self {
        Self {
            num_spins: 0,
            interactions: HashMap::new(),
            fields: Vec::new(),
            variable_map: HashMap::new(),
        }
    }

    /// Allocate a new spin and return its index.
    fn allocate_spin(&mut self) -> Result<usize, crate::registry::ConstructionError> {
        let idx = self.num_spins;
        self.num_spins = self
            .num_spins
            .checked_add(1)
            .ok_or("spin count exceeds usize")?;
        self.fields.push(0);
        Ok(idx)
    }

    /// Get or create a spin index for a variable.
    fn get_or_create_variable(
        &mut self,
        name: &str,
    ) -> Result<usize, crate::registry::ConstructionError> {
        if let Some(&idx) = self.variable_map.get(name) {
            Ok(idx)
        } else {
            let idx = self.allocate_spin()?;
            self.variable_map.insert(name.to_string(), idx);
            Ok(idx)
        }
    }

    /// Add a gadget to the builder with the given spin mapping.
    fn add_gadget(
        &mut self,
        gadget: &LogicGadget<i64>,
        spin_map: &[usize],
    ) -> Result<(), crate::registry::ConstructionError> {
        // Add interactions
        for ((i, j), weight) in gadget.problem.interactions() {
            let global_i = spin_map[i];
            let global_j = spin_map[j];
            let key = if global_i < global_j {
                (global_i, global_j)
            } else {
                (global_j, global_i)
            };
            let entry = self.interactions.entry(key).or_insert(0);
            *entry = entry
                .checked_add(weight)
                .ok_or("circuit SpinGlass coupling overflow")?;
        }

        // Add fields
        for (local_idx, field) in gadget.problem.fields().iter().enumerate() {
            let global_idx = spin_map[local_idx];
            self.fields[global_idx] = self.fields[global_idx]
                .checked_add(*field)
                .ok_or("circuit SpinGlass field overflow")?;
        }
        Ok(())
    }

    /// Build the final SpinGlass.
    fn build(self) -> Result<BuiltSpinGlass, crate::registry::ConstructionError> {
        let mut interactions: Vec<((usize, usize), i64)> = self.interactions.into_iter().collect();
        interactions.sort_by_key(|((u, v), _)| (*u, *v));
        let sg = SpinGlass::new(self.num_spins, interactions, self.fields);
        Ok((sg?, self.variable_map))
    }
}

/// Process a boolean expression and return the spin index of its output.
fn process_expression(
    expr: &BooleanExpr,
    builder: &mut SpinGlassBuilder,
) -> Result<usize, crate::registry::ConstructionError> {
    match &expr.op {
        BooleanOp::Var(name) => builder.get_or_create_variable(name),

        BooleanOp::Const(value) => {
            let gadget: LogicGadget<i64> = if *value { set1_gadget() } else { set0_gadget() };
            let output_spin = builder.allocate_spin()?;
            let spin_map = vec![output_spin];
            builder.add_gadget(&gadget, &spin_map)?;
            Ok(output_spin)
        }

        BooleanOp::Not(inner) => {
            let input_spin = process_expression(inner, builder)?;
            let gadget: LogicGadget<i64> = not_gadget();
            let output_spin = builder.allocate_spin()?;
            let spin_map = vec![input_spin, output_spin];
            builder.add_gadget(&gadget, &spin_map)?;
            Ok(output_spin)
        }

        BooleanOp::And(args) => process_binary_chain(args, builder, and_gadget),

        BooleanOp::Or(args) => process_binary_chain(args, builder, or_gadget),

        BooleanOp::Xor(args) => process_binary_chain(args, builder, xor_gadget),
    }
}

/// Process a multi-input gate by chaining binary gates.
fn process_binary_chain<F>(
    args: &[BooleanExpr],
    builder: &mut SpinGlassBuilder,
    gadget_fn: F,
) -> Result<usize, crate::registry::ConstructionError>
where
    F: Fn() -> LogicGadget<i64>,
{
    if args.is_empty() {
        return Err("binary gate must have at least one argument".into());
    }

    if args.len() == 1 {
        // Single argument - just return its output
        return process_expression(&args[0], builder);
    }

    // Process first two arguments
    let mut result_spin = {
        let input0 = process_expression(&args[0], builder)?;
        let input1 = process_expression(&args[1], builder)?;
        let gadget = gadget_fn();
        let output_spin = builder.allocate_spin()?;

        // For XOR gadget, we need to allocate the auxiliary spin too
        let spin_map = if gadget.num_spins() == 4 {
            // XOR: inputs [0, 1], aux at 3, output at 2
            let aux_spin = builder.allocate_spin()?;
            vec![input0, input1, output_spin, aux_spin]
        } else {
            // AND/OR: inputs [0, 1], output at 2
            vec![input0, input1, output_spin]
        };

        builder.add_gadget(&gadget, &spin_map)?;
        output_spin
    };

    // Chain remaining arguments
    for arg in args.iter().skip(2) {
        let next_input = process_expression(arg, builder)?;
        let gadget = gadget_fn();
        let output_spin = builder.allocate_spin()?;

        let spin_map = if gadget.num_spins() == 4 {
            let aux_spin = builder.allocate_spin()?;
            vec![result_spin, next_input, output_spin, aux_spin]
        } else {
            vec![result_spin, next_input, output_spin]
        };

        builder.add_gadget(&gadget, &spin_map)?;
        result_spin = output_spin;
    }

    Ok(result_spin)
}

/// Process a circuit assignment.
fn process_assignment(
    assignment: &Assignment,
    builder: &mut SpinGlassBuilder,
) -> Result<(), crate::registry::ConstructionError> {
    // Process the expression to get the output spin
    let expr_output = process_expression(&assignment.expr, builder)?;

    // For each output variable, we need to constrain it to equal the expression output
    // This is done by adding a NOT gadget constraint (with J=1) to enforce equality
    for output_name in &assignment.outputs {
        let output_spin = builder.get_or_create_variable(output_name)?;

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
            let entry = builder.interactions.entry(key).or_insert(0);
            *entry = entry
                .checked_add(-4)
                .ok_or("circuit SpinGlass equality coupling overflow")?;
        }
    }
    Ok(())
}

#[reduction(
    size = upper_bound {
        num_spins = "num_variables + 2 * num_expression_nodes",
        num_interactions = "6 * num_expression_nodes + num_assignment_outputs",
    }
)]
impl ReduceTo<SpinGlass<SimpleGraph, i64>> for CircuitSAT {
    type Result = ReductionCircuitToSG;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let mut builder = SpinGlassBuilder::new();

        // Process each assignment in the circuit
        for assignment in &self.circuit().assignments {
            process_assignment(assignment, &mut builder).map_err(
                crate::rules::ReductionError::construction::<
                    CircuitSAT,
                    SpinGlass<SimpleGraph, i64>,
                >,
            )?;
        }

        let (target, variable_map) = builder.build().map_err(
            crate::rules::ReductionError::construction::<CircuitSAT, SpinGlass<SimpleGraph, i64>>,
        )?;
        let source_variables = self.variable_names().to_vec();

        Ok(ReductionCircuitToSG {
            target,
            variable_map,
            source_variables,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT};

    fn full_adder_circuit_sat() -> CircuitSAT {
        let circuit = Circuit::new(vec![
            Assignment::new(
                vec!["t".to_string()],
                BooleanExpr::xor(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
            ),
            Assignment::new(
                vec!["sum".to_string()],
                BooleanExpr::xor(vec![BooleanExpr::var("t"), BooleanExpr::var("cin")]),
            ),
            Assignment::new(
                vec!["ab".to_string()],
                BooleanExpr::and(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
            ),
            Assignment::new(
                vec!["cin_t".to_string()],
                BooleanExpr::and(vec![BooleanExpr::var("cin"), BooleanExpr::var("t")]),
            ),
            Assignment::new(
                vec!["cout".to_string()],
                BooleanExpr::or(vec![BooleanExpr::var("ab"), BooleanExpr::var("cin_t")]),
            ),
        ]);
        CircuitSAT::new(circuit)
    }

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "circuitsat_to_spinglass",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, SpinGlass<SimpleGraph, i64>>(
                full_adder_circuit_sat(),
                SolutionPair {
                    source_config: serde_json::json!(vec![
                        false, false, false, false, false, false, false, false
                    ]),
                    target_config: serde_json::json!(vec![
                        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/circuit_spinglass.rs"]
mod tests;
