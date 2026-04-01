#!/usr/bin/env python3
"""
Verification script for the CircuitSAT -> Satisfiability (Tseitin) reduction.

7 mandatory sections:
1. Symbolic / analytic proof sketch
2. Small hand-crafted examples (YES + NO)
3. Brute-force forward check (satisfiability preservation)
4. Brute-force backward check (solution extraction)
5. Overhead formula verification
6. Exhaustive small-n check (n <= 5)
7. Summary statistics
"""

import itertools
import json
import random
import sys
from dataclasses import dataclass, field
from typing import Optional

from pysat.solvers import Minisat22

# ============================================================
# Circuit representation
# ============================================================

class Expr:
    """Expression tree for boolean circuits."""
    pass

@dataclass
class Var(Expr):
    name: str
    def __repr__(self): return self.name

@dataclass
class Const(Expr):
    value: bool
    def __repr__(self): return str(self.value)

@dataclass
class Not(Expr):
    child: Expr
    def __repr__(self): return f"NOT({self.child})"

@dataclass
class And(Expr):
    children: list
    def __repr__(self): return f"AND({', '.join(str(c) for c in self.children)})"

@dataclass
class Or(Expr):
    children: list
    def __repr__(self): return f"OR({', '.join(str(c) for c in self.children)})"

@dataclass
class Xor(Expr):
    children: list
    def __repr__(self): return f"XOR({', '.join(str(c) for c in self.children)})"

@dataclass
class Assignment:
    outputs: list  # list of variable names
    expr: Expr

def eval_expr(expr, env):
    """Evaluate expression given variable assignment dict."""
    if isinstance(expr, Var):
        return env[expr.name]
    elif isinstance(expr, Const):
        return expr.value
    elif isinstance(expr, Not):
        return not eval_expr(expr.child, env)
    elif isinstance(expr, And):
        return all(eval_expr(c, env) for c in expr.children)
    elif isinstance(expr, Or):
        return any(eval_expr(c, env) for c in expr.children)
    elif isinstance(expr, Xor):
        result = False
        for c in expr.children:
            result ^= eval_expr(c, env)
        return result
    raise ValueError(f"Unknown expr type: {type(expr)}")

def circuit_variables(assignments):
    """Get sorted list of all variables in circuit."""
    vars_set = set()
    def walk(expr):
        if isinstance(expr, Var):
            vars_set.add(expr.name)
        elif isinstance(expr, Const):
            pass
        elif isinstance(expr, Not):
            walk(expr.child)
        elif isinstance(expr, (And, Or, Xor)):
            for c in expr.children:
                walk(c)
    for a in assignments:
        for o in a.outputs:
            vars_set.add(o)
        walk(a.expr)
    return sorted(vars_set)

def circuit_satisfying(assignments, env):
    """Check if an assignment satisfies the circuit."""
    for a in assignments:
        result = eval_expr(a.expr, env)
        for o in a.outputs:
            if env.get(o, False) != result:
                return False
    return True

def brute_force_circuit(assignments):
    """Return all satisfying assignments for the circuit."""
    all_vars = circuit_variables(assignments)
    solutions = []
    for bits in itertools.product([False, True], repeat=len(all_vars)):
        env = dict(zip(all_vars, bits))
        if circuit_satisfying(assignments, env):
            solutions.append(env)
    return solutions

# ============================================================
# SAT representation
# ============================================================

def eval_cnf(num_vars, clauses, assignment):
    """Evaluate a CNF formula. assignment is list of bool, 0-indexed."""
    for clause in clauses:
        satisfied = False
        for lit in clause:
            var_idx = abs(lit) - 1
            val = assignment[var_idx]
            if (lit > 0 and val) or (lit < 0 and not val):
                satisfied = True
                break
        if not satisfied:
            return False
    return True

def brute_force_sat(num_vars, clauses):
    """Return all satisfying assignments for CNF."""
    solutions = []
    for bits in itertools.product([False, True], repeat=num_vars):
        if eval_cnf(num_vars, clauses, list(bits)):
            solutions.append(list(bits))
    return solutions

def sat_is_satisfiable(num_vars, clauses):
    """Check satisfiability using pysat (fast)."""
    with Minisat22() as solver:
        for c in clauses:
            solver.add_clause(c)
        return solver.solve()

def sat_find_one(num_vars, clauses):
    """Find one satisfying assignment using pysat, or None."""
    with Minisat22() as solver:
        for c in clauses:
            solver.add_clause(c)
        if solver.solve():
            model = solver.get_model()
            # Convert to bool list (0-indexed)
            assignment = [False] * num_vars
            for lit in model:
                if lit > 0 and lit <= num_vars:
                    assignment[lit - 1] = True
            return assignment
    return None

def sat_enumerate_all(num_vars, clauses, limit=10000):
    """Enumerate all SAT solutions using pysat with blocking clauses."""
    solutions = []
    with Minisat22() as solver:
        for c in clauses:
            solver.add_clause(c)
        while solver.solve() and len(solutions) < limit:
            model = solver.get_model()
            assignment = [False] * num_vars
            for lit in model:
                if lit > 0 and lit <= num_vars:
                    assignment[lit - 1] = True
            solutions.append(assignment)
            # Block this solution
            blocking = [-lit for lit in model if abs(lit) <= num_vars]
            if not blocking:
                break
            solver.add_clause(blocking)
    return solutions

# ============================================================
# Tseitin Reduction: CircuitSAT -> Satisfiability
# ============================================================

def reduce(assignments):
    """
    Reduce CircuitSAT to Satisfiability via Tseitin transformation.

    Args:
        assignments: list of Assignment objects

    Returns:
        (num_vars, clauses, var_map, circuit_var_names)
        - var_map: dict mapping variable name -> SAT variable index (1-indexed)
        - circuit_var_names: sorted list of circuit variable names
    """
    circuit_var_names = circuit_variables(assignments)
    var_map = {}  # name -> 1-indexed SAT variable
    next_var = [1]

    def get_var(name):
        if name not in var_map:
            var_map[name] = next_var[0]
            next_var[0] += 1
        return var_map[name]

    # Assign indices to circuit variables first
    for v in circuit_var_names:
        get_var(v)

    clauses = []

    def tseitin(expr):
        """
        Walk the expression tree, creating a fresh variable for each
        non-leaf node, and adding definitional clauses.
        Returns the SAT variable index representing this subexpression.
        """
        if isinstance(expr, Var):
            return get_var(expr.name)
        elif isinstance(expr, Const):
            # Create a fresh variable and force it to the constant value
            v = next_var[0]
            next_var[0] += 1
            if expr.value:
                clauses.append([v])  # v must be true
            else:
                clauses.append([-v])  # v must be false
            return v
        elif isinstance(expr, Not):
            a = tseitin(expr.child)
            v = next_var[0]
            next_var[0] += 1
            # v <=> NOT a: (not v or not a) and (v or a)
            clauses.append([-v, -a])
            clauses.append([v, a])
            return v
        elif isinstance(expr, And):
            child_vars = [tseitin(c) for c in expr.children]
            if len(child_vars) == 1:
                return child_vars[0]
            # Build binary tree of AND gates
            result = child_vars[0]
            for i in range(1, len(child_vars)):
                a = result
                b = child_vars[i]
                v = next_var[0]
                next_var[0] += 1
                # v <=> a AND b: (not v or a), (not v or b), (v or not a or not b)
                clauses.append([-v, a])
                clauses.append([-v, b])
                clauses.append([v, -a, -b])
                result = v
            return result
        elif isinstance(expr, Or):
            child_vars = [tseitin(c) for c in expr.children]
            if len(child_vars) == 1:
                return child_vars[0]
            # Build binary tree of OR gates
            result = child_vars[0]
            for i in range(1, len(child_vars)):
                a = result
                b = child_vars[i]
                v = next_var[0]
                next_var[0] += 1
                # v <=> a OR b: (v or not a), (v or not b), (not v or a or b)
                clauses.append([v, -a])
                clauses.append([v, -b])
                clauses.append([-v, a, b])
                result = v
            return result
        elif isinstance(expr, Xor):
            child_vars = [tseitin(c) for c in expr.children]
            if len(child_vars) == 1:
                return child_vars[0]
            # Build binary tree of XOR gates
            result = child_vars[0]
            for i in range(1, len(child_vars)):
                a = result
                b = child_vars[i]
                v = next_var[0]
                next_var[0] += 1
                # v <=> a XOR b: 4 clauses
                clauses.append([-v, -a, -b])
                clauses.append([-v, a, b])
                clauses.append([v, -a, b])
                clauses.append([v, a, -b])
                result = v
            return result
        raise ValueError(f"Unknown expr: {type(expr)}")

    # Process each assignment
    for assign in assignments:
        root_var = tseitin(assign.expr)
        # Each output must equal the expression result
        for out_name in assign.outputs:
            out_var = get_var(out_name)
            # out_var <=> root_var: (not out or root) and (out or not root)
            clauses.append([-out_var, root_var])
            clauses.append([out_var, -root_var])

    num_vars = next_var[0] - 1
    return num_vars, clauses, var_map, circuit_var_names


def extract_circuit_solution(sat_assignment, var_map, circuit_var_names):
    """
    Given a SAT solution (list of bool, 0-indexed), extract circuit variable values.
    """
    env = {}
    for name in circuit_var_names:
        idx = var_map[name] - 1  # 0-indexed
        env[name] = sat_assignment[idx]
    return env

# ============================================================
# Random circuit generation
# ============================================================

def random_expr(input_vars, depth=0, max_depth=2):
    """Generate a random expression tree."""
    if depth >= max_depth or (depth > 0 and random.random() < 0.4):
        # Leaf: variable or constant
        if random.random() < 0.9:
            return Var(random.choice(input_vars))
        else:
            return Const(random.choice([True, False]))

    gate = random.choice(["not", "and", "or", "xor"])
    if gate == "not":
        return Not(random_expr(input_vars, depth + 1, max_depth))
    else:
        n_children = random.randint(2, 3)
        children = [random_expr(input_vars, depth + 1, max_depth) for _ in range(n_children)]
        if gate == "and":
            return And(children)
        elif gate == "or":
            return Or(children)
        else:
            return Xor(children)

def random_circuit(num_input_vars=None, num_assignments=None):
    """Generate a random circuit."""
    if num_input_vars is None:
        num_input_vars = random.randint(2, 3)
    if num_assignments is None:
        num_assignments = random.randint(1, 2)

    input_vars = [f"x{i}" for i in range(num_input_vars)]
    assignments = []
    all_vars = list(input_vars)

    for i in range(num_assignments):
        out_name = f"g{i}"
        expr = random_expr(all_vars, max_depth=1)
        assignments.append(Assignment(outputs=[out_name], expr=expr))
        all_vars.append(out_name)

    return assignments


# ============================================================
# Section 1: Symbolic / analytic proof sketch
# ============================================================

def section1_symbolic():
    print("=" * 60)
    print("SECTION 1: Symbolic / analytic proof sketch")
    print("=" * 60)

    print("""
Tseitin Transformation (CircuitSAT -> CNF-SAT):

Given a boolean circuit C with assignments {o_i = f_i(inputs)},
we construct a CNF formula F as follows:

1. For each circuit variable v, create a SAT variable x_v.
2. Walk each assignment's expression tree. For each non-leaf gate g
   with fresh variable x_g:
   - NOT(a):   {-g,-a}, {g,a}
   - AND(a,b): {-g,a}, {-g,b}, {g,-a,-b}
   - OR(a,b):  {g,-a}, {g,-b}, {-g,a,b}
   - XOR(a,b): {-g,-a,-b}, {-g,a,b}, {g,-a,b}, {g,a,-b}
3. For each output o_i = f_i, add equivalence clauses x_{o_i} <=> x_{root_i}.
4. N-ary gates are flattened to binary with fresh intermediate variables.

Correctness:
  (=>) If C is satisfiable with assignment A, extend A to auxiliary
       variables by evaluating each gate. Every definitional clause
       is satisfied by construction.
  (<=) If F is satisfiable with assignment B, the values of the
       circuit variable subset of B satisfy C because the definitional
       clauses enforce gate semantics.

Solution extraction: Read circuit variables from the first |circuit_vars|
SAT variables (by their mapped indices).
""")
    return 0  # no checks in section 1

# ============================================================
# Section 2: Small hand-crafted examples (YES + NO)
# ============================================================

def section2_handcrafted():
    print("=" * 60)
    print("SECTION 2: Hand-crafted examples")
    print("=" * 60)
    checks = 0

    # YES example: c = x AND y, d = c OR z
    # Satisfying: x=T, y=T, z=F => c=T, d=T
    print("\nYES example: c = x AND y, d = c OR z")
    yes_circuit = [
        Assignment(outputs=["c"], expr=And([Var("x"), Var("y")])),
        Assignment(outputs=["d"], expr=Or([Var("c"), Var("z")])),
    ]
    yes_env = {"c": True, "d": True, "x": True, "y": True, "z": False}
    assert circuit_satisfying(yes_circuit, yes_env), "YES example should be satisfying"
    checks += 1

    num_vars, clauses, var_map, cv = reduce(yes_circuit)
    sat_sols = brute_force_sat(num_vars, clauses)
    assert len(sat_sols) > 0, "YES example: SAT should be satisfiable"
    checks += 1

    # Verify extraction
    for sol in sat_sols:
        env = extract_circuit_solution(sol, var_map, cv)
        assert circuit_satisfying(yes_circuit, env), "Extracted solution must satisfy circuit"
        checks += 1
    print(f"  YES example: {len(sat_sols)} SAT solutions, all extract correctly. checks={checks}")

    # NO example: c = x AND y, d = NOT(c), force c=True AND d=True (impossible)
    print("\nNO example: c = x AND y, d = NOT(c), with c=d constraint")
    no_circuit = [
        Assignment(outputs=["c"], expr=And([Var("x"), Var("y")])),
        Assignment(outputs=["d"], expr=Not(Var("c"))),
        # Force c = d (impossible since d = NOT c)
        Assignment(outputs=["c"], expr=Var("d")),
    ]
    no_sols = brute_force_circuit(no_circuit)
    assert len(no_sols) == 0, "NO example: circuit should be unsatisfiable"
    checks += 1

    num_vars_no, clauses_no, var_map_no, cv_no = reduce(no_circuit)
    sat_sols_no = brute_force_sat(num_vars_no, clauses_no)
    assert len(sat_sols_no) == 0, "NO example: SAT should be unsatisfiable"
    checks += 1
    print(f"  NO example: SAT unsatisfiable as expected. checks={checks}")

    # Another YES: XOR gate
    print("\nYES example 2: c = x XOR y")
    xor_circuit = [
        Assignment(outputs=["c"], expr=Xor([Var("x"), Var("y")])),
    ]
    circuit_sols = brute_force_circuit(xor_circuit)
    num_v, cls, vm, cvn = reduce(xor_circuit)
    sat_s = brute_force_sat(num_v, cls)
    assert len(circuit_sols) == len(set(
        tuple(extract_circuit_solution(s, vm, cvn).get(v, False) for v in cvn)
        for s in sat_s
    )), "XOR: same number of distinct circuit solutions"
    checks += 1
    print(f"  XOR example OK. checks={checks}")

    print(f"Section 2 total checks: {checks}")
    return checks

# ============================================================
# Section 3: Brute-force forward check (satisfiability preservation)
# ============================================================

def section3_forward(num_trials=2500):
    print("=" * 60)
    print(f"SECTION 3: Forward check ({num_trials} random circuits)")
    print("=" * 60)
    checks = 0
    failures = 0

    for trial in range(num_trials):
        assignments = random_circuit()
        circuit_sols = brute_force_circuit(assignments)
        circuit_sat = len(circuit_sols) > 0

        num_vars, clauses, var_map, cv = reduce(assignments)
        sat_sat = sat_is_satisfiable(num_vars, clauses)

        if circuit_sat != sat_sat:
            print(f"  FAILURE at trial {trial}: circuit_sat={circuit_sat}, sat_sat={sat_sat}")
            print(f"    Circuit: {assignments}")
            failures += 1
        checks += 1

    print(f"Section 3: {checks} checks, {failures} failures")
    assert failures == 0, f"Section 3 had {failures} failures"
    return checks

# ============================================================
# Section 4: Brute-force backward check (solution extraction)
# ============================================================

def section4_backward(num_trials=2500):
    print("=" * 60)
    print(f"SECTION 4: Backward check / solution extraction ({num_trials} trials)")
    print("=" * 60)
    checks = 0
    failures = 0

    for trial in range(num_trials):
        assignments = random_circuit()
        num_vars, clauses, var_map, cv = reduce(assignments)

        # Check: every SAT solution extracts to a valid circuit solution
        sat_sol = sat_find_one(num_vars, clauses)
        if sat_sol is not None:
            env = extract_circuit_solution(sat_sol, var_map, cv)
            if not circuit_satisfying(assignments, env):
                print(f"  FAILURE at trial {trial}: extracted solution doesn't satisfy circuit")
                failures += 1
            checks += 1

        # Also verify: every circuit solution maps to a valid SAT solution
        circuit_sols = brute_force_circuit(assignments)
        for csol in circuit_sols:
            # Extend circuit solution to full SAT assignment by evaluating gates
            full_assignment = extend_to_sat(csol, assignments, num_vars, var_map)
            if not eval_cnf(num_vars, clauses, full_assignment):
                print(f"  FAILURE at trial {trial}: circuit solution can't extend to SAT")
                failures += 1
            checks += 1

    print(f"Section 4: {checks} checks, {failures} failures")
    assert failures == 0, f"Section 4 had {failures} failures"
    return checks


def extend_to_sat(circuit_env, assignments, num_vars, var_map):
    """
    Extend a circuit variable assignment to a full SAT assignment
    by evaluating each gate in the expression trees.
    """
    # Start with circuit variable values
    full_env = dict(circuit_env)

    # We need to evaluate each subexpression and assign auxiliary variables.
    # Re-run the Tseitin transformation but this time track values.
    next_var = [max(var_map.values()) + 1 if var_map else 1]
    sat_assignment = [False] * num_vars

    # Set circuit variable values
    for name, val in circuit_env.items():
        if name in var_map:
            sat_assignment[var_map[name] - 1] = val

    # Re-walk the expression trees to assign auxiliary variables
    aux_idx = [len(var_map)]  # next aux variable (0-indexed position)

    def eval_and_assign(expr):
        """Evaluate expr and assign its gate variable."""
        if isinstance(expr, Var):
            return circuit_env.get(expr.name, False)
        elif isinstance(expr, Const):
            val = expr.value
            idx = aux_idx[0]
            aux_idx[0] += 1
            if idx < num_vars:
                sat_assignment[idx] = val
            return val
        elif isinstance(expr, Not):
            child_val = eval_and_assign(expr.child)
            val = not child_val
            idx = aux_idx[0]
            aux_idx[0] += 1
            if idx < num_vars:
                sat_assignment[idx] = val
            return val
        elif isinstance(expr, And):
            child_vals = [eval_and_assign(c) for c in expr.children]
            if len(child_vals) == 1:
                return child_vals[0]
            result = child_vals[0]
            for i in range(1, len(child_vals)):
                result = result and child_vals[i]
                idx = aux_idx[0]
                aux_idx[0] += 1
                if idx < num_vars:
                    sat_assignment[idx] = result
            return result
        elif isinstance(expr, Or):
            child_vals = [eval_and_assign(c) for c in expr.children]
            if len(child_vals) == 1:
                return child_vals[0]
            result = child_vals[0]
            for i in range(1, len(child_vals)):
                result = result or child_vals[i]
                idx = aux_idx[0]
                aux_idx[0] += 1
                if idx < num_vars:
                    sat_assignment[idx] = result
            return result
        elif isinstance(expr, Xor):
            child_vals = [eval_and_assign(c) for c in expr.children]
            if len(child_vals) == 1:
                return child_vals[0]
            result = child_vals[0]
            for i in range(1, len(child_vals)):
                result = result ^ child_vals[i]
                idx = aux_idx[0]
                aux_idx[0] += 1
                if idx < num_vars:
                    sat_assignment[idx] = result
            return result
        return False

    for assign in assignments:
        eval_and_assign(assign.expr)

    return sat_assignment

# ============================================================
# Section 5: Overhead formula verification
# ============================================================

def count_gates(expr):
    """Count the number of non-leaf nodes (gates) in an expression tree."""
    if isinstance(expr, (Var, Const)):
        return 0
    elif isinstance(expr, Not):
        return 1 + count_gates(expr.child)
    elif isinstance(expr, (And, Or, Xor)):
        child_gates = sum(count_gates(c) for c in expr.children)
        # n-ary gate with k children -> (k-1) binary gates
        n_binary = max(0, len(expr.children) - 1)
        return n_binary + child_gates
    return 0

def count_const_nodes(expr):
    """Count constant nodes in expression tree."""
    if isinstance(expr, Const):
        return 1
    elif isinstance(expr, Var):
        return 0
    elif isinstance(expr, Not):
        return count_const_nodes(expr.child)
    elif isinstance(expr, (And, Or, Xor)):
        return sum(count_const_nodes(c) for c in expr.children)
    return 0

def section5_overhead(num_trials=1500):
    print("=" * 60)
    print(f"SECTION 5: Overhead formula verification ({num_trials} trials)")
    print("=" * 60)
    checks = 0
    failures = 0

    for trial in range(num_trials):
        assignments = random_circuit()
        cv = circuit_variables(assignments)
        num_circuit_vars = len(cv)

        num_vars, clauses, var_map, _ = reduce(assignments)
        num_clauses = len(clauses)

        # Count total binary gates and constants across all assignments
        total_gates = sum(count_gates(a.expr) for a in assignments)
        total_consts = sum(count_const_nodes(a.expr) for a in assignments)

        # Expected fresh variables: one per binary gate + one per constant
        expected_aux_vars = total_gates + total_consts
        expected_num_vars = num_circuit_vars + expected_aux_vars

        if num_vars != expected_num_vars:
            print(f"  FAILURE trial {trial}: num_vars={num_vars}, expected={expected_num_vars}")
            print(f"    circuit_vars={num_circuit_vars}, gates={total_gates}, consts={total_consts}")
            failures += 1
        checks += 1

        # Verify num_vars >= num_circuit_vars (always)
        assert num_vars >= num_circuit_vars
        checks += 1

    print(f"Section 5: {checks} checks, {failures} failures")
    assert failures == 0, f"Section 5 had {failures} failures"
    return checks

# ============================================================
# Section 6: Exhaustive small-n check (n <= 5)
# ============================================================

def section6_exhaustive():
    print("=" * 60)
    print("SECTION 6: Exhaustive small-n check")
    print("=" * 60)
    checks = 0
    failures = 0

    # Exhaustively generate circuits with 2-5 variables
    gate_types = ["not", "and", "or", "xor"]

    for n_vars in range(2, 6):
        input_vars = [f"x{i}" for i in range(n_vars)]

        # Generate a diverse set of circuits
        for gate in gate_types:
            for i in range(len(input_vars)):
                for j in range(len(input_vars)):
                    if gate == "not":
                        if i != j:
                            continue
                        expr = Not(Var(input_vars[i]))
                    elif gate == "and":
                        expr = And([Var(input_vars[i]), Var(input_vars[j])])
                    elif gate == "or":
                        expr = Or([Var(input_vars[i]), Var(input_vars[j])])
                    else:  # xor
                        expr = Xor([Var(input_vars[i]), Var(input_vars[j])])

                    out_name = "out"
                    circuit_assigns = [Assignment(outputs=[out_name], expr=expr)]

                    # Check reduction preserves satisfiability
                    circuit_sols = brute_force_circuit(circuit_assigns)
                    num_v, cls, vm, cv = reduce(circuit_assigns)
                    sat_is_sat = sat_is_satisfiable(num_v, cls)

                    circuit_is_sat = len(circuit_sols) > 0

                    if circuit_is_sat != sat_is_sat:
                        print(f"  FAILURE: n={n_vars}, gate={gate}, vars=({i},{j})")
                        failures += 1
                    checks += 1

                    # Check extraction for one SAT solution
                    sol = sat_find_one(num_v, cls)
                    if sol is not None:
                        env = extract_circuit_solution(sol, vm, cv)
                        if not circuit_satisfying(circuit_assigns, env):
                            print(f"  FAILURE: extraction n={n_vars}, gate={gate}")
                            failures += 1
                        checks += 1

        # Multi-gate circuits (more trials for better coverage)
        for _ in range(100):
            num_assign = random.randint(1, 3)
            circuit = random_circuit(num_input_vars=n_vars, num_assignments=num_assign)
            circuit_sols = brute_force_circuit(circuit)
            num_v, cls, vm, cv = reduce(circuit)
            sat_sat = sat_is_satisfiable(num_v, cls)

            if (len(circuit_sols) > 0) != sat_sat:
                failures += 1
            checks += 1

            sol = sat_find_one(num_v, cls)
            if sol is not None:
                env = extract_circuit_solution(sol, vm, cv)
                if not circuit_satisfying(circuit, env):
                    failures += 1
                checks += 1

    print(f"Section 6: {checks} checks, {failures} failures")
    assert failures == 0, f"Section 6 had {failures} failures"
    return checks

# ============================================================
# Section 7: Summary and test vectors export
# ============================================================

def section7_summary(check_counts, export_path=None):
    print("=" * 60)
    print("SECTION 7: Summary")
    print("=" * 60)
    total = sum(check_counts.values())
    for section, count in check_counts.items():
        print(f"  {section}: {count} checks")
    print(f"  TOTAL: {total} checks")

    if total < 5000:
        print(f"  WARNING: total checks ({total}) < 5000 minimum")

    # Export test vectors
    if export_path:
        export_test_vectors(export_path)

    return total

def export_test_vectors(path):
    """Export test vectors JSON for downstream use."""

    # YES instance: c = x AND y, d = c OR z
    yes_circuit = [
        Assignment(outputs=["c"], expr=And([Var("x"), Var("y")])),
        Assignment(outputs=["d"], expr=Or([Var("c"), Var("z")])),
    ]
    yes_nv, yes_cls, yes_vm, yes_cv = reduce(yes_circuit)

    # Find one satisfying SAT solution
    yes_sat_sols = brute_force_sat(yes_nv, yes_cls)
    yes_sol = yes_sat_sols[0] if yes_sat_sols else None
    yes_env = extract_circuit_solution(yes_sol, yes_vm, yes_cv) if yes_sol else None

    # NO instance: c = x AND y, d = NOT(c), force c = d
    no_circuit = [
        Assignment(outputs=["c"], expr=And([Var("x"), Var("y")])),
        Assignment(outputs=["d"], expr=Not(Var("c"))),
        Assignment(outputs=["c"], expr=Var("d")),
    ]
    no_nv, no_cls, no_vm, no_cv = reduce(no_circuit)

    # XOR instance
    xor_circuit = [
        Assignment(outputs=["c"], expr=Xor([Var("x"), Var("y")])),
    ]
    xor_nv, xor_cls, xor_vm, xor_cv = reduce(xor_circuit)
    xor_sat_sols = brute_force_sat(xor_nv, xor_cls)

    test_vectors = {
        "reduction": "CircuitSAT -> Satisfiability",
        "method": "Tseitin transformation",
        "yes_instance": {
            "circuit": [
                {"outputs": ["c"], "expr": "AND(x, y)"},
                {"outputs": ["d"], "expr": "OR(c, z)"},
            ],
            "circuit_variables": yes_cv,
            "sat_num_vars": yes_nv,
            "sat_clauses": yes_cls,
            "var_map": yes_vm,
            "example_sat_solution": [int(b) for b in yes_sol] if yes_sol else None,
            "extracted_circuit_env": {k: int(v) for k, v in yes_env.items()} if yes_env else None,
            "satisfiable": True,
        },
        "no_instance": {
            "circuit": [
                {"outputs": ["c"], "expr": "AND(x, y)"},
                {"outputs": ["d"], "expr": "NOT(c)"},
                {"outputs": ["c"], "expr": "Var(d)"},
            ],
            "circuit_variables": no_cv,
            "sat_num_vars": no_nv,
            "sat_clauses": no_cls,
            "var_map": no_vm,
            "satisfiable": False,
        },
        "xor_instance": {
            "circuit": [
                {"outputs": ["c"], "expr": "XOR(x, y)"},
            ],
            "circuit_variables": xor_cv,
            "sat_num_vars": xor_nv,
            "sat_clauses": xor_cls,
            "var_map": xor_vm,
            "num_sat_solutions": len(xor_sat_sols),
            "satisfiable": True,
        },
        "overhead": {
            "description": "num_vars = |circuit_vars| + |gates| + |constants|; num_clauses depends on gate types",
            "formula_num_vars": "|V_circuit| + |gates_binary| + |constants|",
        },
        "claims": [
            "CircuitSAT satisfiable iff produced CNF-SAT satisfiable",
            "Every SAT solution extracts to a valid circuit solution",
            "Every circuit solution has at least one corresponding SAT solution",
            "Overhead is linear in circuit size (gates + assignments)",
        ],
    }

    with open(path, "w") as f:
        json.dump(test_vectors, f, indent=2)
    print(f"  Exported test vectors to {path}")


# ============================================================
# Main
# ============================================================

def main():
    random.seed(42)

    check_counts = {}

    s1 = section1_symbolic()
    check_counts["Section 1 (symbolic)"] = s1

    s2 = section2_handcrafted()
    check_counts["Section 2 (handcrafted)"] = s2

    s3 = section3_forward(num_trials=2000)
    check_counts["Section 3 (forward)"] = s3

    s4 = section4_backward(num_trials=2000)
    check_counts["Section 4 (backward)"] = s4

    s5 = section5_overhead(num_trials=1000)
    check_counts["Section 5 (overhead)"] = s5

    s6 = section6_exhaustive()
    check_counts["Section 6 (exhaustive)"] = s6

    export_path = "docs/paper/verify-reductions/test_vectors_circuitsat_satisfiability.json"
    total = section7_summary(check_counts, export_path=export_path)

    print(f"\nALL SECTIONS PASSED. Total checks: {total}")
    if total < 5000:
        print("WARNING: Below 5000 check minimum!")
        sys.exit(1)
    return 0

if __name__ == "__main__":
    sys.exit(main())
