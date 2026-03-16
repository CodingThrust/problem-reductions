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

#[test]
fn test_pred_sym_parse_invalid_input_returns_error() {
    let output = pred_sym().args(["parse", "n +"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("failed to parse expression"),
        "got: {stderr}"
    );
}

#[test]
fn test_pred_sym_big_o_rejects_division() {
    let output = pred_sym().args(["big-o", "n / m"]).output().unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_pred_sym_version() {
    let output = pred_sym().args(["--version"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("pred-sym"), "got: {stdout}");
}

#[test]
fn test_pred_sym_big_o_raw() {
    let output = pred_sym()
        .args(["big-o", "--raw", "3 * n^2 + n"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "n^2");
}

#[test]
fn test_pred_sym_big_o_exp_dominates_poly() {
    let output = pred_sym().args(["big-o", "2^n + n^3"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2^n"), "got: {}", stdout.trim());
    assert!(
        !stdout.contains("n^3"),
        "n^3 should be dominated, got: {}",
        stdout.trim()
    );
}

#[test]
fn test_pred_sym_big_o_larger_base_dominates() {
    let output = pred_sym().args(["big-o", "3^n + 2^n"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("3^n"), "got: {}", stdout.trim());
    assert!(
        !stdout.contains("2^n"),
        "2^n should be dominated, got: {}",
        stdout.trim()
    );
}

#[test]
fn test_pred_sym_big_o_poly_log_dominates_poly() {
    let output = pred_sym()
        .args(["big-o", "n * log(n) + n"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("log"),
        "expected n*log(n) to survive, got: {}",
        stdout.trim()
    );
}

#[test]
fn test_pred_sym_eval_unbound_variable_error() {
    let output = pred_sym()
        .args(["eval", "n^2", "--vars", "m=10"])
        .output()
        .unwrap();
    assert!(!output.status.success(), "should fail for unbound variable");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("unbound variable"),
        "expected unbound variable error, got: {stderr}"
    );
    assert!(
        stderr.contains("n"),
        "should mention variable 'n', got: {stderr}"
    );
}

#[test]
fn test_pred_sym_compare_unequal_exits_nonzero() {
    let output = pred_sym().args(["compare", "n^2", "n^3"]).output().unwrap();
    assert!(
        !output.status.success(),
        "compare of unequal expressions should exit non-zero"
    );
}

#[test]
fn test_pred_sym_compare_big_o_equal_exits_zero() {
    let output = pred_sym()
        .args(["compare", "2*n^2 + n", "n^2"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Big-O equal expressions should exit 0"
    );
}
