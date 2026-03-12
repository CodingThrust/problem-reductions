use std::process::Command;

fn pred_sym() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pred-sym"))
}

#[test]
fn test_pred_sym_parse() {
    let output = pred_sym().args(["parse", "n + m"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "n + m");
}

#[test]
fn test_pred_sym_canon_merge_terms() {
    let output = pred_sym().args(["canon", "n + n"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "2 * n");
}

#[test]
fn test_pred_sym_big_o() {
    let output = pred_sym().args(["big-o", "3 * n^2 + n"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "O(n^2)");
}

#[test]
fn test_pred_sym_eval() {
    let output = pred_sym()
        .args(["eval", "n + m", "--vars", "n=3,m=4"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "7");
}

#[test]
fn test_pred_sym_big_o_signed_polynomial() {
    let output = pred_sym()
        .args(["big-o", "n^3 - n^2 + 2*n + 4*n*m"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // n^3 dominates n^2 and n; n*m is incomparable
    assert!(stdout.contains("n^3"), "got: {}", stdout.trim());
}

#[test]
fn test_pred_sym_big_o_sqrt_display() {
    let output = pred_sym().args(["big-o", "2^(n^(1/2))"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("sqrt"),
        "expected sqrt notation, got: {}",
        stdout.trim()
    );
}

#[test]
fn test_pred_sym_compare() {
    let output = pred_sym()
        .args(["compare", "n + n", "2 * n"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("true"),
        "expected exact equality, got: {}",
        stdout.trim()
    );
}
