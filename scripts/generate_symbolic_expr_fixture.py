"""Generate the symbolic-expression conformance fixture with SymPy.

The committed fixture lets Rust tests use SymPy as an independent semantic
oracle without adding Python to the Rust build or test environment.

Usage:
    uv run --project scripts python scripts/generate_symbolic_expr_fixture.py
"""

import json
from pathlib import Path

import sympy
from sympy.parsing.sympy_parser import (
    convert_xor,
    parse_expr,
    rationalize,
    standard_transformations,
)


OUTPUT = (
    Path(__file__).resolve().parents[1]
    / "problemreductions-expr"
    / "tests"
    / "fixtures"
    / "sympy_oracle.json"
)
TRANSFORMATIONS = standard_transformations + (convert_xor, rationalize)


# The final boolean selects cases where SymPy's mathematical polynomial
# predicate and this crate's deliberately syntactic predicate have the same
# contract. Every case still participates in variable and exact-value checks.
CASES = [
    ("zero", "0", {}, True),
    ("integer", "42", {}, True),
    ("exact_decimal", "2.372", {}, True),
    ("leading_decimal_point", ".125", {}, True),
    (
        "arbitrary_precision_integer",
        "100000000000000000000000000000000000000000000000001",
        {},
        True,
    ),
    ("variable", "n", {"n": 7}, True),
    ("negation", "-n", {"n": 7}, True),
    ("addition", "n + m", {"n": 3, "m": 4}, True),
    ("subtraction", "n - m", {"n": 3, "m": 7}, True),
    ("multiplication", "n * m", {"n": 6, "m": 7}, True),
    ("rational_coefficient", "n / 2", {"n": 3}, True),
    ("variable_divisor", "n / m", {"n": 12, "m": 5}, True),
    ("nested_divisor", "n / (m + 1)", {"n": 10, "m": 4}, True),
    ("exact_size_formula", "n * (n - 1) / 2 - m", {"n": 5, "m": 4}, True),
    ("zero_power", "n^0", {"n": 9}, True),
    ("integer_power", "n^3", {"n": 4}, True),
    ("negative_power", "2^-3", {}, False),
    ("symbolic_exponent", "2^n", {"n": 10}, True),
    ("unary_precedence", "-n^2", {"n": 3}, True),
    ("parenthesized_negative_base", "(-n)^2", {"n": 3}, True),
    ("fractional_power", "n^0.5", {"n": 81}, True),
    ("square_root", "sqrt(n)", {"n": 81}, True),
    ("pythagorean_root", "sqrt(n^2 + m^2)", {"n": 3, "m": 4}, True),
    ("exponential_identity", "exp(n)", {"n": 0}, True),
    ("logarithm_identity", "log(n)", {"n": 1}, True),
    ("factorial", "factorial(n)", {"n": 6}, True),
    ("factorial_subexpression", "factorial(n - 1)", {"n": 6}, True),
    ("decimal_scaling", "2.372 * n", {"n": 1000}, True),
    ("difference_of_squares", "(n + m) * (n - m)", {"n": 10, "m": 3}, True),
    (
        "multivariate_polynomial",
        "n^2 + 2 * n * m + m^2",
        {"n": 3, "m": 4},
        True,
    ),
    ("nested_rational", "n / (2 * m)", {"n": 12, "m": 3}, True),
    (
        "long_decimal",
        "1.0000000000000000000000000000000000000001",
        {},
        True,
    ),
    ("nested_subtraction", "n - (m - k)", {"n": 10, "m": 7, "k": 2}, True),
    ("left_subtraction", "(n - m) - k", {"n": 10, "m": 7, "k": 2}, True),
    ("nested_division", "n / (m / k)", {"n": 12, "m": 6, "k": 3}, True),
    ("left_division", "(n / m) / k", {"n": 12, "m": 6, "k": 2}, True),
    ("right_associative_power", "n^(m^k)", {"n": 2, "m": 3, "k": 2}, True),
    ("parenthesized_power", "(n^m)^k", {"n": 2, "m": 3, "k": 2}, True),
    ("double_negation", "--n", {"n": 7}, True),
    ("zero_factorial", "factorial(0)", {}, False),
    ("zero_square_root", "sqrt(0)", {}, False),
    (
        "constant_functions",
        "exp(0) + log(1) + factorial(5)",
        {},
        False,
    ),
    ("zero_product", "n * 0 + 7", {"n": 999}, True),
    ("self_division", "n / n", {"n": 5}, True),
    ("identity_power", "n^1", {"n": 13}, True),
    ("decimal_integer_power", "n^2.0", {"n": 9}, True),
    ("decimal_sum", "0.1 + 0.2", {}, True),
    (
        "large_mixed_decimal",
        "99999999999999999999.00000000000000000001",
        {},
        True,
    ),
    ("identifier_shapes", "n_1 + size2", {"n_1": 8, "size2": 9}, True),
    ("mixed_precedence", "n + m * k^2", {"n": 1, "m": 2, "k": 3}, True),
]


# These cases exercise the production f64 boundary. Expected values are emitted
# at 80 decimal digits so the Rust test, rather than Python's float conversion,
# performs the final rounding to f64.
APPROXIMATE_CASES = [
    ("exp_one", "exp(1)", {}),
    ("exp_fraction", "exp(n / 3)", {"n": 5}),
    ("log_two", "log(2)", {}),
    ("log_large", "log(1000000)", {}),
    ("sqrt_two", "sqrt(2)", {}),
    ("sqrt_large", "sqrt(1234567)", {}),
    ("fractional_power", "7^2.372", {}),
    ("mixed_transcendental", "exp(log(n)) + sqrt(m)", {"n": 13, "m": 2}),
    ("complexity_formula", "2^(2.372 * n / 3)", {"n": 19}),
    ("factorial_ten", "factorial(10)", {}),
    ("factorial_f64_boundary", "factorial(170)", {}),
    ("factorial_f64_overflow", "factorial(171)", {}),
]


def parse(source: str) -> sympy.Expr:
    return parse_expr(source, transformations=TRANSFORMATIONS, evaluate=False)


def exact_fraction(value: sympy.Expr) -> str:
    value = value.doit()
    if value.is_Rational is not True:
        raise ValueError(f"fixture result is not exact rational: {value!r}")
    numerator, denominator = value.as_numer_denom()
    return f"{numerator}/{denominator}"


def generate_case(
    name: str,
    source: str,
    bindings: dict[str, int],
    compare_polynomial: bool,
) -> dict:
    expression = parse(source)
    symbols = sorted(str(symbol) for symbol in expression.free_symbols)
    if set(symbols) != set(bindings):
        raise ValueError(f"{name} bindings do not match free symbols")
    substitutions = {sympy.Symbol(name): value for name, value in bindings.items()}
    result = expression.subs(substitutions)
    polynomial = expression.is_polynomial(
        *(sympy.Symbol(name) for name in symbols)
    )
    return {
        "name": name,
        "source": source,
        "variables": symbols,
        "bindings": bindings,
        "exact_result": exact_fraction(result),
        "compare_polynomial": compare_polynomial,
        "is_polynomial": polynomial is True,
    }


def generate_approximate_case(
    name: str,
    source: str,
    bindings: dict[str, int],
) -> dict:
    expression = parse(source)
    symbols = sorted(str(symbol) for symbol in expression.free_symbols)
    if set(symbols) != set(bindings):
        raise ValueError(f"{name} bindings do not match free symbols")
    substitutions = {sympy.Symbol(name): value for name, value in bindings.items()}
    result = expression.subs(substitutions).doit()
    if result.is_real is not True or result.is_finite is not True:
        raise ValueError(f"{name} result is not a finite real number: {result!r}")
    return {
        "name": name,
        "source": source,
        "bindings": bindings,
        "decimal_result": str(sympy.N(result, 80)),
    }


def main() -> None:
    if sympy.__version__ != "1.14.0":
        raise RuntimeError(f"expected SymPy 1.14.0, found {sympy.__version__}")
    fixture = {
        "oracle": {
            "engine": "SymPy",
            "version": sympy.__version__,
            "parse_evaluate": False,
            "decimal_mode": "rationalize base-10 spelling",
            "documentation": {
                "parser": "https://docs.sympy.org/latest/modules/parsing.html",
                "expression_core": "https://docs.sympy.org/latest/modules/core.html",
            },
        },
        "cases": [generate_case(*case) for case in CASES],
        "approximate_cases": [
            generate_approximate_case(*case) for case in APPROXIMATE_CASES
        ],
    }
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(fixture, indent=2) + "\n", encoding="utf-8")
    print(
        f"wrote {len(fixture['cases'])} exact and "
        f"{len(fixture['approximate_cases'])} approximate cases to {OUTPUT}"
    )


if __name__ == "__main__":
    main()
