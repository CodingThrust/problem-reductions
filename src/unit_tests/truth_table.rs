use super::*;

#[test]
fn test_and_gate() {
    let and = TruthTable::and(2);
    assert!(!and.evaluate(&[false, false]));
    assert!(!and.evaluate(&[true, false]));
    assert!(!and.evaluate(&[false, true]));
    assert!(and.evaluate(&[true, true]));
}

#[test]
fn test_or_gate() {
    let or = TruthTable::or(2);
    assert!(!or.evaluate(&[false, false]));
    assert!(or.evaluate(&[true, false]));
    assert!(or.evaluate(&[false, true]));
    assert!(or.evaluate(&[true, true]));
}

#[test]
fn test_not_gate() {
    let not = TruthTable::not();
    assert!(not.evaluate(&[false]));
    assert!(!not.evaluate(&[true]));
}

#[test]
fn test_xor_gate() {
    let xor = TruthTable::xor(2);
    assert!(!xor.evaluate(&[false, false]));
    assert!(xor.evaluate(&[true, false]));
    assert!(xor.evaluate(&[false, true]));
    assert!(!xor.evaluate(&[true, true]));
}

#[test]
fn test_nand_gate() {
    let nand = TruthTable::nand(2);
    assert!(nand.evaluate(&[false, false]));
    assert!(nand.evaluate(&[true, false]));
    assert!(nand.evaluate(&[false, true]));
    assert!(!nand.evaluate(&[true, true]));
}

#[test]
fn test_implies() {
    let imp = TruthTable::implies();
    assert!(imp.evaluate(&[false, false])); // F -> F = T
    assert!(imp.evaluate(&[false, true])); // F -> T = T
    assert!(!imp.evaluate(&[true, false])); // T -> F = F
    assert!(imp.evaluate(&[true, true])); // T -> T = T
}

#[test]
fn test_from_function() {
    let majority =
        TruthTable::from_function(3, |input| input.iter().filter(|&&b| b).count() >= 2);
    assert!(!majority.evaluate(&[false, false, false]));
    assert!(!majority.evaluate(&[true, false, false]));
    assert!(majority.evaluate(&[true, true, false]));
    assert!(majority.evaluate(&[true, true, true]));
}

#[test]
fn test_evaluate_config() {
    let and = TruthTable::and(2);
    assert!(!and.evaluate_config(&[0, 0]));
    assert!(!and.evaluate_config(&[1, 0]));
    assert!(and.evaluate_config(&[1, 1]));
}

#[test]
fn test_satisfiable() {
    let or = TruthTable::or(2);
    assert!(or.is_satisfiable());

    let contradiction = TruthTable::from_outputs(2, vec![false, false, false, false]);
    assert!(!contradiction.is_satisfiable());
    assert!(contradiction.is_contradiction());
}

#[test]
fn test_tautology() {
    let tautology = TruthTable::from_outputs(2, vec![true, true, true, true]);
    assert!(tautology.is_tautology());

    let or = TruthTable::or(2);
    assert!(!or.is_tautology());
}

#[test]
fn test_satisfying_assignments() {
    let xor = TruthTable::xor(2);
    let sat = xor.satisfying_assignments();
    assert_eq!(sat.len(), 2);
    assert!(sat.contains(&vec![true, false]));
    assert!(sat.contains(&vec![false, true]));
}

#[test]
fn test_count() {
    let and = TruthTable::and(2);
    assert_eq!(and.count_ones(), 1);
    assert_eq!(and.count_zeros(), 3);
}

#[test]
fn test_index_to_input() {
    let tt = TruthTable::and(3);
    assert_eq!(tt.index_to_input(0), vec![false, false, false]);
    assert_eq!(tt.index_to_input(1), vec![true, false, false]);
    assert_eq!(tt.index_to_input(7), vec![true, true, true]);
}

#[test]
fn test_outputs_vec() {
    let and = TruthTable::and(2);
    assert_eq!(and.outputs_vec(), vec![false, false, false, true]);
}

#[test]
fn test_and_with() {
    let a = TruthTable::from_outputs(1, vec![false, true]);
    let b = TruthTable::from_outputs(1, vec![true, false]);
    let result = a.and_with(&b);
    assert_eq!(result.outputs_vec(), vec![false, false]);
}

#[test]
fn test_or_with() {
    let a = TruthTable::from_outputs(1, vec![false, true]);
    let b = TruthTable::from_outputs(1, vec![true, false]);
    let result = a.or_with(&b);
    assert_eq!(result.outputs_vec(), vec![true, true]);
}

#[test]
fn test_negate() {
    let and = TruthTable::and(2);
    let nand = and.negate();
    assert_eq!(nand.outputs_vec(), vec![true, true, true, false]);
}

#[test]
fn test_num_rows() {
    let tt = TruthTable::and(3);
    assert_eq!(tt.num_rows(), 8);
}

#[test]
fn test_3_input_and() {
    let and3 = TruthTable::and(3);
    assert!(!and3.evaluate(&[true, true, false]));
    assert!(and3.evaluate(&[true, true, true]));
}

#[test]
fn test_xnor() {
    let xnor = TruthTable::xnor(2);
    assert!(xnor.evaluate(&[false, false]));
    assert!(!xnor.evaluate(&[true, false]));
    assert!(!xnor.evaluate(&[false, true]));
    assert!(xnor.evaluate(&[true, true]));
}

#[test]
fn test_nor() {
    let nor = TruthTable::nor(2);
    assert!(nor.evaluate(&[false, false]));
    assert!(!nor.evaluate(&[true, false]));
    assert!(!nor.evaluate(&[false, true]));
    assert!(!nor.evaluate(&[true, true]));
}

#[test]
fn test_serialization() {
    let and = TruthTable::and(2);
    let json = serde_json::to_string(&and).unwrap();
    let deserialized: TruthTable = serde_json::from_str(&json).unwrap();
    assert_eq!(and, deserialized);
}

#[test]
fn test_outputs() {
    let and = TruthTable::and(2);
    let outputs = and.outputs();
    assert_eq!(outputs.len(), 4);
}

#[test]
fn test_num_inputs() {
    let and = TruthTable::and(3);
    assert_eq!(and.num_inputs(), 3);
}
