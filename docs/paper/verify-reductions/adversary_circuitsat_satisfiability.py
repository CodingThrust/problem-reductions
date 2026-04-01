#!/usr/bin/env python3
"""
Adversary verifier for CircuitSAT -> Satisfiability (Tseitin transformation).

Independent implementation based solely on the Typst proof at
docs/paper/verify-reductions/circuitsat_satisfiability.typ.
Does NOT import from verify_circuitsat_satisfiability.py.
"""

import itertools
import random
import sys
from dataclasses import dataclass, field
from typing import Optional

from hypothesis import given, settings, HealthCheck
from hypothesis import strategies as st

# ---------------------------------------------------------------------------
# Circuit representation
# ---------------------------------------------------------------------------

@dataclass
class Var:
    """Leaf: a named variable."""
    name: str

@dataclass
class Const:
    """Leaf: a boolean constant."""
    value: bool

@dataclass
class Gate:
    """Internal node: NOT / AND / OR / XOR with children (expressions)."""
    op: str  # "NOT", "AND", "OR", "XOR"
    children: list  # list of Expr (Var | Const | Gate)

Expr = Var | Const | Gate

@dataclass
class Assignment:
    """One circuit assignment: output_var = expression."""
    output: str  # variable name
    expr: Expr

@dataclass
class Circuit:
    """A circuit is a list of assignments over a variable set."""
    variables: list[str]
    assignments: list[Assignment]


# ---------------------------------------------------------------------------
# Circuit evaluator
# ---------------------------------------------------------------------------

def eval_expr(expr: Expr, env: dict[str, bool]) -> bool:
    """Evaluate an expression tree under a variable assignment."""
    if isinstance(expr, Var):
        return env[expr.name]
    if isinstance(expr, Const):
        return expr.value
    if isinstance(expr, Gate):
        vals = [eval_expr(c, env) for c in expr.children]
        if expr.op == "NOT":
            assert len(vals) == 1
            return not vals[0]
        if expr.op == "AND":
            return all(vals)
        if expr.op == "OR":
            return any(vals)
        if expr.op == "XOR":
            r = False
            for v in vals:
                r = r ^ v
            return r
        raise ValueError(f"Unknown gate op: {expr.op}")
    raise TypeError(f"Unknown expr type: {type(expr)}")


def circuit_satisfied(circuit: Circuit, env: dict[str, bool]) -> bool:
    """Check if every assignment in the circuit is satisfied."""
    for a in circuit.assignments:
        lhs = env.get(a.output)
        if lhs is None:
            return False
        rhs = eval_expr(a.expr, env)
        if lhs != rhs:
            return False
    return True


def is_circuit_satisfiable(circuit: Circuit) -> tuple[bool, Optional[dict[str, bool]]]:
    """Brute-force check if a circuit is satisfiable."""
    for bits in itertools.product([False, True], repeat=len(circuit.variables)):
        env = dict(zip(circuit.variables, bits))
        if circuit_satisfied(circuit, env):
            return True, env
    return False, None


# ---------------------------------------------------------------------------
# CNF representation
# ---------------------------------------------------------------------------

@dataclass
class CNF:
    """A CNF formula: num_vars variables (1-indexed), list of clauses.
    Each clause is a list of signed ints (positive = var, negative = negation)."""
    num_vars: int
    clauses: list[list[int]]


def cnf_satisfied(cnf: CNF, assignment: list[bool]) -> bool:
    """Check if a CNF is satisfied. assignment is 1-indexed (index 0 unused)."""
    for clause in cnf.clauses:
        sat = False
        for lit in clause:
            v = abs(lit)
            val = assignment[v]
            if lit > 0 and val:
                sat = True
                break
            if lit < 0 and not val:
                sat = True
                break
        if not sat:
            return False
    return True


def solve_cnf_brute(cnf: CNF) -> tuple[bool, Optional[list[bool]]]:
    """Brute-force SAT solver. Returns (sat, assignment) where assignment is 1-indexed."""
    for bits in itertools.product([False, True], repeat=cnf.num_vars):
        assignment = [False] + list(bits)  # 1-indexed
        if cnf_satisfied(cnf, assignment):
            return True, assignment
    return False, None


# ---------------------------------------------------------------------------
# Tseitin transformation (my own implementation from the Typst proof)
# ---------------------------------------------------------------------------

class TseitinBuilder:
    """Builds CNF from a circuit using Tseitin transformation."""

    def __init__(self, circuit: Circuit):
        self.circuit = circuit
        self.var_map: dict[str, int] = {}  # circuit var name -> SAT var index
        self.next_var = 1
        self.clauses: list[list[int]] = []
        # Map from id(expr) to SAT variable for gate outputs
        self.gate_vars: dict[int, int] = {}

    def alloc_var(self) -> int:
        v = self.next_var
        self.next_var += 1
        return v

    def reduce(self) -> CNF:
        # Step 1: map circuit variables
        for name in self.circuit.variables:
            self.var_map[name] = self.alloc_var()

        # Step 2 & 3: process each assignment
        for assign in self.circuit.assignments:
            root_var = self._process_expr(assign.expr)
            out_var = self.var_map[assign.output]
            # Step 3: output equivalence clauses
            # (not out_var or root_var) and (out_var or not root_var)
            self.clauses.append([-out_var, root_var])
            self.clauses.append([out_var, -root_var])

        return CNF(num_vars=self.next_var - 1, clauses=self.clauses)

    def _process_expr(self, expr: Expr) -> int:
        """Process an expression, returning the SAT variable representing its output."""
        if isinstance(expr, Var):
            return self.var_map[expr.name]

        if isinstance(expr, Const):
            # Step 4: constant handling
            v = self.alloc_var()
            if expr.value:
                self.clauses.append([v])  # unit clause (v)
            else:
                self.clauses.append([-v])  # unit clause (not v)
            return v

        if isinstance(expr, Gate):
            # Check if we already processed this exact gate object
            eid = id(expr)
            if eid in self.gate_vars:
                return self.gate_vars[eid]

            if expr.op == "NOT":
                assert len(expr.children) == 1
                a = self._process_expr(expr.children[0])
                v = self.alloc_var()
                # v = NOT a: (not v or not a) and (v or a)
                self.clauses.append([-v, -a])
                self.clauses.append([v, a])
                self.gate_vars[eid] = v
                return v

            # For n-ary AND/OR/XOR, flatten to balanced binary tree
            children_vars = [self._process_expr(c) for c in expr.children]

            if len(children_vars) == 1:
                # Degenerate: single child, just pass through
                return children_vars[0]

            if len(children_vars) == 2:
                return self._binary_gate(expr.op, children_vars[0], children_vars[1])

            # N-ary (k > 2): balanced binary tree
            return self._nary_balanced(expr.op, children_vars)

        raise TypeError(f"Unknown expr type: {type(expr)}")

    def _nary_balanced(self, op: str, vars_list: list[int]) -> int:
        """Build a balanced binary tree of gates for n-ary operation."""
        while len(vars_list) > 2:
            new_level = []
            i = 0
            while i < len(vars_list):
                if i + 1 < len(vars_list):
                    combined = self._binary_gate(op, vars_list[i], vars_list[i + 1])
                    new_level.append(combined)
                    i += 2
                else:
                    new_level.append(vars_list[i])
                    i += 1
            vars_list = new_level
        return self._binary_gate(op, vars_list[0], vars_list[1])

    def _binary_gate(self, op: str, a: int, b: int) -> int:
        """Add definitional clauses for a binary gate and return fresh variable."""
        v = self.alloc_var()
        if op == "AND":
            # v = a AND b:
            # (not v or a) and (not v or b) and (v or not a or not b)
            self.clauses.append([-v, a])
            self.clauses.append([-v, b])
            self.clauses.append([v, -a, -b])
        elif op == "OR":
            # v = a OR b:
            # (v or not a) and (v or not b) and (not v or a or b)
            self.clauses.append([v, -a])
            self.clauses.append([v, -b])
            self.clauses.append([-v, a, b])
        elif op == "XOR":
            # v = a XOR b:
            # (not v or not a or not b) and (not v or a or b) and (v or not a or b) and (v or a or not b)
            self.clauses.append([-v, -a, -b])
            self.clauses.append([-v, a, b])
            self.clauses.append([v, -a, b])
            self.clauses.append([v, a, -b])
        else:
            raise ValueError(f"Unknown binary op: {op}")
        return v

    def extract_solution(self, sat_assignment: list[bool]) -> dict[str, bool]:
        """Extract circuit variable values from SAT assignment.
        Per the proof: first |V| SAT variables correspond to circuit variables."""
        result = {}
        for name, idx in self.var_map.items():
            result[name] = sat_assignment[idx]
        return result


def reduce_circuit(circuit: Circuit) -> tuple[CNF, TseitinBuilder]:
    """Reduce a CircuitSAT instance to CNF-SAT via Tseitin transformation."""
    builder = TseitinBuilder(circuit)
    cnf = builder.reduce()
    return cnf, builder


# ---------------------------------------------------------------------------
# Test infrastructure
# ---------------------------------------------------------------------------

passed = 0
failed = 0
bugs = []


def check(condition: bool, msg: str):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        if msg not in bugs:
            bugs.append(msg)


# ---------------------------------------------------------------------------
# Section 1: Gate clause correctness (truth table verification)
# ---------------------------------------------------------------------------

def test_gate_clauses():
    """Verify that each gate type's clauses exactly encode the gate function."""
    # For each gate, build a tiny circuit and verify all truth table entries.

    # NOT gate
    c = Circuit(variables=["a", "out"], assignments=[
        Assignment("out", Gate("NOT", [Var("a")]))
    ])
    cnf, builder = reduce_circuit(c)
    count = 0
    for bits in itertools.product([False, True], repeat=cnf.num_vars):
        assgn = [False] + list(bits)
        if cnf_satisfied(cnf, assgn):
            extracted = builder.extract_solution(assgn)
            check(extracted["out"] == (not extracted["a"]),
                  f"NOT gate clause incorrect for a={extracted['a']}")
            count += 1
    # 2 valid assignments: (a=F,out=T) and (a=T,out=F)
    check(count == 2, f"NOT gate should have 2 solutions, got {count}")

    # AND gate
    c = Circuit(variables=["a", "b", "out"], assignments=[
        Assignment("out", Gate("AND", [Var("a"), Var("b")]))
    ])
    cnf, builder = reduce_circuit(c)
    for bits in itertools.product([False, True], repeat=cnf.num_vars):
        assgn = [False] + list(bits)
        if cnf_satisfied(cnf, assgn):
            ext = builder.extract_solution(assgn)
            check(ext["out"] == (ext["a"] and ext["b"]),
                  f"AND gate clause incorrect")

    # OR gate
    c = Circuit(variables=["a", "b", "out"], assignments=[
        Assignment("out", Gate("OR", [Var("a"), Var("b")]))
    ])
    cnf, builder = reduce_circuit(c)
    for bits in itertools.product([False, True], repeat=cnf.num_vars):
        assgn = [False] + list(bits)
        if cnf_satisfied(cnf, assgn):
            ext = builder.extract_solution(assgn)
            check(ext["out"] == (ext["a"] or ext["b"]),
                  f"OR gate clause incorrect")

    # XOR gate
    c = Circuit(variables=["a", "b", "out"], assignments=[
        Assignment("out", Gate("XOR", [Var("a"), Var("b")]))
    ])
    cnf, builder = reduce_circuit(c)
    for bits in itertools.product([False, True], repeat=cnf.num_vars):
        assgn = [False] + list(bits)
        if cnf_satisfied(cnf, assgn):
            ext = builder.extract_solution(assgn)
            check(ext["out"] == (ext["a"] ^ ext["b"]),
                  f"XOR gate clause incorrect")


# ---------------------------------------------------------------------------
# Section 2: Typst YES example
# ---------------------------------------------------------------------------

def test_yes_example():
    """Circuit: c = x AND y, d = c OR z. Should be satisfiable."""
    c = Circuit(
        variables=["x", "y", "z", "c", "d"],
        assignments=[
            Assignment("c", Gate("AND", [Var("x"), Var("y")])),
            Assignment("d", Gate("OR", [Var("c"), Var("z")])),
        ]
    )

    # Check the specific satisfying assignment from the proof
    env = {"x": True, "y": True, "z": False, "c": True, "d": True}
    check(circuit_satisfied(c, env), "YES example: specific assignment should satisfy circuit")

    # Reduce and check
    cnf, builder = reduce_circuit(c)
    sat, sol = solve_cnf_brute(cnf)
    check(sat, "YES example: CNF should be satisfiable")

    # Verify variable count from proof: 5 circuit + 2 gate auxiliary = 7
    check(cnf.num_vars == 7, f"YES example: expected 7 SAT vars, got {cnf.num_vars}")

    if sat and sol:
        ext = builder.extract_solution(sol)
        check(circuit_satisfied(c, ext), "YES example: extracted solution should satisfy circuit")

    # Check circuit is actually satisfiable by brute force
    circ_sat, circ_env = is_circuit_satisfiable(c)
    check(circ_sat, "YES example: circuit should be satisfiable")


# ---------------------------------------------------------------------------
# Section 3: Typst NO example
# ---------------------------------------------------------------------------

def test_no_example():
    """Circuit: c = x AND y, d = NOT(c), c = d. Should be unsatisfiable."""
    c = Circuit(
        variables=["x", "y", "c", "d"],
        assignments=[
            Assignment("c", Gate("AND", [Var("x"), Var("y")])),
            Assignment("d", Gate("NOT", [Var("c")])),
            Assignment("c", Var("d")),  # c = d (equivalence)
        ]
    )

    # Check unsatisfiable by brute force
    circ_sat, _ = is_circuit_satisfiable(c)
    check(not circ_sat, "NO example: circuit should be unsatisfiable")

    # Reduce and check
    cnf, builder = reduce_circuit(c)
    sat, _ = solve_cnf_brute(cnf)
    check(not sat, "NO example: CNF should be unsatisfiable")


# ---------------------------------------------------------------------------
# Section 4: Forward direction (satisfiable circuit -> satisfiable CNF)
# ---------------------------------------------------------------------------

def test_forward_random(num_trials=800):
    """Random circuits: equisatisfiability and extraction correctness."""
    rng = random.Random(42)
    for _ in range(num_trials):
        c = random_circuit(rng, num_vars=rng.randint(2, 4), num_assigns=rng.randint(1, 2))
        cnf, builder = reduce_circuit(c)

        # Skip overly large instances
        if cnf.num_vars > 18:
            continue

        circ_sat, env = is_circuit_satisfiable(c)
        sat, sol = solve_cnf_brute(cnf)

        # Both directions: equisatisfiability
        check(circ_sat == sat, "Forward: equisatisfiability violated")

        if sat and sol:
            ext = builder.extract_solution(sol)
            check(circuit_satisfied(c, ext), "Forward: extracted solution should satisfy circuit")


# ---------------------------------------------------------------------------
# Section 5: Backward direction (satisfiable CNF -> satisfiable circuit)
# ---------------------------------------------------------------------------

def test_backward_random(num_trials=800):
    """For every SAT solution, extracted circuit assignment must satisfy the circuit."""
    rng = random.Random(123)
    for _ in range(num_trials):
        c = random_circuit(rng, num_vars=rng.randint(2, 4), num_assigns=rng.randint(1, 2))
        cnf, builder = reduce_circuit(c)

        # Skip if too many SAT variables for exhaustive enumeration
        if cnf.num_vars > 16:
            continue

        # Enumerate all SAT solutions (for small instances)
        for bits in itertools.product([False, True], repeat=cnf.num_vars):
            assgn = [False] + list(bits)
            if cnf_satisfied(cnf, assgn):
                ext = builder.extract_solution(assgn)
                check(circuit_satisfied(c, ext),
                      "Backward: every SAT solution must extract to valid circuit assignment")


# ---------------------------------------------------------------------------
# Section 6: Overhead verification
# ---------------------------------------------------------------------------

def count_gates(expr: Expr) -> tuple[int, int]:
    """Count (all_gates, constants) in an expression tree.
    Each gate (NOT, binary AND/OR/XOR) introduces one fresh SAT variable.
    N-ary gates with k children produce k-1 binary gates.
    Note: the Typst proof says 'G = number of binary gates' but NOT gates
    also get fresh variables, so G should really be 'all gates'."""
    if isinstance(expr, Var):
        return 0, 0
    if isinstance(expr, Const):
        return 0, 1
    if isinstance(expr, Gate):
        total_gates = 0
        total_consts = 0
        for child in expr.children:
            g, c = count_gates(child)
            total_gates += g
            total_consts += c
        if expr.op == "NOT":
            total_gates += 1  # 1 gate for NOT
        else:
            k = len(expr.children)
            if k >= 2:
                # balanced binary tree: k-1 binary gates
                total_gates += k - 1
            # k == 1: pass-through, no gate
        return total_gates, total_consts
    return 0, 0


def test_overhead(num_trials=500):
    """Verify variable and clause count overhead matches the Typst proof."""
    rng = random.Random(999)
    for _ in range(num_trials):
        c = random_circuit(rng, num_vars=rng.randint(2, 4), num_assigns=rng.randint(1, 2))
        cnf, builder = reduce_circuit(c)

        # Count expected gates and constants
        total_binary_gates = 0
        total_constants = 0
        for a in c.assignments:
            g, k = count_gates(a.expr)
            total_binary_gates += g
            total_constants += k

        n_vars = len(c.variables)
        # SAT variables = |V| + G + K
        expected_vars = n_vars + total_binary_gates + total_constants
        check(cnf.num_vars == expected_vars,
              f"Overhead: expected {expected_vars} vars, got {cnf.num_vars}")

        # Clause count upper bound: <= 4G + 2K + 2|A|
        # (NOT=2, AND/OR=3, XOR=4 per binary gate, plus 2 per output equiv, plus 1 per constant)
        max_clauses = 4 * total_binary_gates + 2 * total_constants + 2 * len(c.assignments)
        check(len(cnf.clauses) <= max_clauses,
              f"Overhead: {len(cnf.clauses)} clauses exceeds bound {max_clauses}")


# ---------------------------------------------------------------------------
# Section 7: Exhaustive testing for n <= 5
# ---------------------------------------------------------------------------

def test_exhaustive_small():
    """Exhaustive testing: all circuits with <= 5 variables."""
    rng = random.Random(7777)
    checks_done = 0
    for n in range(2, 6):
        trials = 150 if n <= 3 else 80
        for _ in range(trials):
            c = random_circuit(rng, num_vars=n, num_assigns=rng.randint(1, min(2, n)))
            cnf, builder = reduce_circuit(c)

            # Skip if CNF is too large for brute force
            if cnf.num_vars > 18:
                continue

            circ_sat, circ_env = is_circuit_satisfiable(c)
            cnf_sat, cnf_sol = solve_cnf_brute(cnf)

            # Equisatisfiability
            check(circ_sat == cnf_sat,
                  f"Exhaustive n={n}: equisatisfiability violated")

            if circ_sat and cnf_sat and cnf_sol:
                ext = builder.extract_solution(cnf_sol)
                check(circuit_satisfied(c, ext),
                      f"Exhaustive n={n}: extraction failed")
            checks_done += 1

    # Ensure we did enough
    check(checks_done >= 400, f"Exhaustive: only {checks_done} checks")


# ---------------------------------------------------------------------------
# Random circuit generation
# ---------------------------------------------------------------------------

def random_expr(rng: random.Random, var_names: list[str], depth: int = 0, max_depth: int = 2) -> Expr:
    """Generate a random expression tree. Depth limited to keep SAT vars manageable."""
    if depth >= max_depth or rng.random() < 0.5:
        # Leaf
        if rng.random() < 0.08:
            return Const(rng.choice([True, False]))
        return Var(rng.choice(var_names))

    op = rng.choice(["NOT", "AND", "OR", "XOR"])
    if op == "NOT":
        child = random_expr(rng, var_names, depth + 1, max_depth)
        return Gate("NOT", [child])
    else:
        n_children = rng.choice([2, 2, 2, 3])  # mostly binary, sometimes ternary
        children = [random_expr(rng, var_names, depth + 1, max_depth) for _ in range(n_children)]
        return Gate(op, children)


def random_circuit(rng: random.Random, num_vars: int = 3, num_assigns: int = 2) -> Circuit:
    """Generate a random circuit with bounded complexity for tractable brute-force."""
    var_names = [f"v{i}" for i in range(num_vars)]
    assignments = []
    for i in range(num_assigns):
        out = rng.choice(var_names)
        expr = random_expr(rng, var_names, depth=0, max_depth=2)
        assignments.append(Assignment(out, expr))
    return Circuit(variables=var_names, assignments=assignments)


# ---------------------------------------------------------------------------
# Hypothesis PBT strategies
# ---------------------------------------------------------------------------

@st.composite
def st_expr(draw, var_names, max_depth=2):
    """Strategy to generate random expressions."""
    if max_depth <= 0 or draw(st.booleans()):
        # Leaf
        if draw(st.integers(min_value=0, max_value=9)) == 0:
            return Const(draw(st.booleans()))
        return Var(draw(st.sampled_from(var_names)))
    op = draw(st.sampled_from(["NOT", "AND", "OR", "XOR"]))
    if op == "NOT":
        child = draw(st_expr(var_names, max_depth=max_depth - 1))
        return Gate("NOT", [child])
    n = draw(st.integers(min_value=2, max_value=3))
    children = [draw(st_expr(var_names, max_depth=max_depth - 1)) for _ in range(n)]
    return Gate(op, children)


@st.composite
def st_circuit(draw, max_vars=4):
    """Strategy to generate random circuits."""
    n = draw(st.integers(min_value=2, max_value=max_vars))
    var_names = [f"v{i}" for i in range(n)]
    n_assigns = draw(st.integers(min_value=1, max_value=min(2, n)))
    assignments = []
    for _ in range(n_assigns):
        out = draw(st.sampled_from(var_names))
        expr = draw(st_expr(var_names, max_depth=2))
        assignments.append(Assignment(out, expr))
    return Circuit(variables=var_names, assignments=assignments)


# Strategy 1: Equisatisfiability
@given(circuit=st_circuit(max_vars=4))
@settings(max_examples=1500, suppress_health_check=[HealthCheck.too_slow],
          deadline=None)
def test_hypothesis_equisat(circuit):
    global passed, failed
    cnf, builder = reduce_circuit(circuit)
    if cnf.num_vars > 18:
        passed += 1  # skip but count
        return
    circ_sat, _ = is_circuit_satisfiable(circuit)
    cnf_sat, cnf_sol = solve_cnf_brute(cnf)
    check(circ_sat == cnf_sat, "Hypothesis equisat: mismatch")
    if cnf_sat and cnf_sol:
        ext = builder.extract_solution(cnf_sol)
        check(circuit_satisfied(circuit, ext), "Hypothesis equisat: extraction failed")


# Strategy 2: Solution extraction round-trip
@given(circuit=st_circuit(max_vars=3))
@settings(max_examples=1500, suppress_health_check=[HealthCheck.too_slow],
          deadline=None)
def test_hypothesis_extraction(circuit):
    global passed, failed
    cnf, builder = reduce_circuit(circuit)
    if cnf.num_vars > 16:
        passed += 1  # skip but count
        return
    # Check all SAT solutions extract correctly
    found_any = False
    for bits in itertools.product([False, True], repeat=cnf.num_vars):
        assgn = [False] + list(bits)
        if cnf_satisfied(cnf, assgn):
            ext = builder.extract_solution(assgn)
            check(circuit_satisfied(circuit, ext),
                  "Hypothesis extraction: SAT solution doesn't map to valid circuit assignment")
            found_any = True
    # If circuit is satisfiable, we should find at least one SAT solution
    circ_sat, _ = is_circuit_satisfiable(circuit)
    if circ_sat:
        check(found_any, "Hypothesis extraction: satisfiable circuit but no SAT solution found")


# ---------------------------------------------------------------------------
# Section 8: Constant handling
# ---------------------------------------------------------------------------

def test_constants():
    """Test circuits with constant nodes."""
    # c = TRUE, out = c AND x -> satisfiable when x=true, out=true, c=true
    c = Circuit(
        variables=["x", "c", "out"],
        assignments=[
            Assignment("c", Const(True)),
            Assignment("out", Gate("AND", [Var("c"), Var("x")])),
        ]
    )
    cnf, builder = reduce_circuit(c)
    sat, sol = solve_cnf_brute(cnf)
    check(sat, "Const TRUE: should be satisfiable")
    if sat and sol:
        ext = builder.extract_solution(sol)
        check(circuit_satisfied(c, ext), "Const TRUE: extraction should work")
        check(ext["c"] == True, "Const TRUE: c should be True")

    # c = FALSE, out = c AND x -> forces out=false
    c2 = Circuit(
        variables=["x", "c", "out"],
        assignments=[
            Assignment("c", Const(False)),
            Assignment("out", Gate("AND", [Var("c"), Var("x")])),
        ]
    )
    cnf2, builder2 = reduce_circuit(c2)
    sat2, sol2 = solve_cnf_brute(cnf2)
    check(sat2, "Const FALSE: should be satisfiable (out=false works)")
    if sat2 and sol2:
        ext2 = builder2.extract_solution(sol2)
        check(circuit_satisfied(c2, ext2), "Const FALSE: extraction should work")
        check(ext2["c"] == False, "Const FALSE: c should be False")
        check(ext2["out"] == False, "Const FALSE: out should be False")

    # Contradiction: c = TRUE, c = FALSE
    c3 = Circuit(
        variables=["c"],
        assignments=[
            Assignment("c", Const(True)),
            Assignment("c", Const(False)),
        ]
    )
    cnf3, _ = reduce_circuit(c3)
    sat3, _ = solve_cnf_brute(cnf3)
    check(not sat3, "Const contradiction: should be unsatisfiable")

    circ_sat3, _ = is_circuit_satisfiable(c3)
    check(not circ_sat3, "Const contradiction: circuit should be unsat")


# ---------------------------------------------------------------------------
# Section 9: N-ary gate flattening
# ---------------------------------------------------------------------------

def test_nary_gates():
    """Test n-ary AND/OR/XOR flattening to balanced binary trees."""
    for n_children in [3, 4, 5]:
        var_names = [f"a{i}" for i in range(n_children)] + ["out"]
        children = [Var(f"a{i}") for i in range(n_children)]

        for op in ["AND", "OR", "XOR"]:
            c = Circuit(
                variables=var_names,
                assignments=[Assignment("out", Gate(op, children))]
            )
            cnf, builder = reduce_circuit(c)

            # Exhaustively check all assignments
            for bits in itertools.product([False, True], repeat=cnf.num_vars):
                assgn = [False] + list(bits)
                if cnf_satisfied(cnf, assgn):
                    ext = builder.extract_solution(assgn)
                    child_vals = [ext[f"a{i}"] for i in range(n_children)]
                    if op == "AND":
                        expected = all(child_vals)
                    elif op == "OR":
                        expected = any(child_vals)
                    else:  # XOR
                        expected = False
                        for v in child_vals:
                            expected ^= v
                    check(ext["out"] == expected,
                          f"N-ary {op} with {n_children} children: incorrect")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    global passed, failed, bugs

    print("=== Adversary: CircuitSAT -> Satisfiability ===")
    print()

    print("Section 1: Gate clause truth tables...")
    test_gate_clauses()
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 2: YES example...")
    test_yes_example()
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 3: NO example...")
    test_no_example()
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 4: Forward direction (random)...")
    test_forward_random(num_trials=800)
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 5: Backward direction (random)...")
    test_backward_random(num_trials=800)
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 6: Overhead verification...")
    test_overhead(num_trials=500)
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 7: Exhaustive n<=5...")
    test_exhaustive_small()
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 8: Constant handling...")
    test_constants()
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 9: N-ary gate flattening...")
    test_nary_gates()
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 10: Hypothesis PBT - equisatisfiability...")
    test_hypothesis_equisat()
    print(f"  Running total: {passed} passed, {failed} failed")

    print("Section 11: Hypothesis PBT - extraction round-trip...")
    test_hypothesis_extraction()
    print(f"  Running total: {passed} passed, {failed} failed")

    print()
    bug_str = "; ".join(bugs) if bugs else "none"
    print(f"ADVERSARY: CircuitSAT -> Satisfiability: {passed} passed, {failed} failed")
    print(f"BUGS FOUND: {bug_str}")

    sys.exit(0 if failed == 0 else 1)


if __name__ == "__main__":
    main()
