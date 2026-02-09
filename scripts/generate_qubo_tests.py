"""Generate QUBO test datasets using qubogen.

For each supported problem type, creates a small instance, reduces it to QUBO
via qubogen, brute-force solves both sides, and exports JSON ground truth
to tests/data/qubo/.

Usage:
    uv run python scripts/generate_qubo_tests.py
"""

import json
from itertools import product
from pathlib import Path

import numpy as np

# Monkey-patch for qubogen compatibility with numpy >= 1.24
np.float = np.float64
np.int = np.int_
np.bool = np.bool_

import qubogen


def brute_force_qubo(Q: np.ndarray) -> dict:
    """Brute-force solve a QUBO: minimize x^T Q x over binary x."""
    n = Q.shape[0]
    best_val = float("inf")
    best_configs = []
    for bits in product(range(2), repeat=n):
        x = np.array(bits, dtype=float)
        val = float(x @ Q @ x)
        if val < best_val - 1e-9:
            best_val = val
            best_configs = [list(bits)]
        elif abs(val - best_val) < 1e-9:
            best_configs.append(list(bits))
    return {"value": best_val, "configs": best_configs}


def save_test(name: str, data: dict, outdir: Path):
    """Save test data as compact JSON."""
    path = outdir / f"{name}.json"
    with open(path, "w") as f:
        json.dump(data, f, separators=(",", ":"))
    print(f"  wrote {path} ({path.stat().st_size} bytes)")


def generate_vertex_covering(outdir: Path):
    """Minimum Vertex Cover on a small graph (4 nodes, 5 edges)."""
    edges = [(0, 1), (1, 2), (2, 3), (0, 3), (0, 2)]
    n_nodes = 4
    penalty = 8.0
    g = qubogen.Graph(edges=np.array(edges), n_nodes=n_nodes)
    Q = qubogen.qubo_mvc(g, penalty=penalty)

    qubo_result = brute_force_qubo(Q)

    save_test("vertexcovering_to_qubo", {
        "problem": "VertexCovering",
        "source": {"num_vertices": n_nodes, "edges": edges, "penalty": penalty},
        "qubo_matrix": Q.tolist(),
        "qubo_num_vars": int(Q.shape[0]),
        "qubo_optimal": qubo_result,
    }, outdir)


def generate_independent_set(outdir: Path):
    """Independent Set on a small graph.

    IndependentSet is the complement of VertexCover: maximize |S| s.t. no
    two adjacent vertices are in S. We formulate as QUBO by negating the
    linear terms of MVC (minimize -|S| + penalty * constraint violations).
    """
    edges = [(0, 1), (1, 2), (2, 3), (0, 3)]
    n_nodes = 4
    penalty = 8.0

    # Independent set QUBO: maximize sum(x_i) s.t. x_i*x_j = 0 for edges
    # = minimize -sum(x_i) + P * sum_{(i,j)} x_i*x_j
    Q = np.zeros((n_nodes, n_nodes))
    for i in range(n_nodes):
        Q[i][i] = -1.0
    for i, j in edges:
        Q[i][j] += penalty

    qubo_result = brute_force_qubo(Q)

    save_test("independentset_to_qubo", {
        "problem": "IndependentSet",
        "source": {"num_vertices": n_nodes, "edges": edges, "penalty": penalty},
        "qubo_matrix": Q.tolist(),
        "qubo_num_vars": int(Q.shape[0]),
        "qubo_optimal": qubo_result,
    }, outdir)


def generate_graph_coloring(outdir: Path):
    """Graph Coloring on a small graph (3 nodes triangle, 3 colors)."""
    edges = [(0, 1), (1, 2), (0, 2)]
    n_nodes = 3
    n_color = 3
    penalty = 10.0
    g = qubogen.Graph(edges=np.array(edges), n_nodes=n_nodes)
    Q = qubogen.qubo_graph_coloring(g, n_color=n_color, penalty=penalty)

    qubo_result = brute_force_qubo(Q)

    # QUBO variables: n_nodes * n_color (one-hot encoding)
    save_test("coloring_to_qubo", {
        "problem": "Coloring",
        "source": {
            "num_vertices": n_nodes,
            "edges": edges,
            "num_colors": n_color,
            "penalty": penalty,
        },
        "qubo_matrix": Q.tolist(),
        "qubo_num_vars": int(Q.shape[0]),
        "qubo_optimal": qubo_result,
    }, outdir)


def generate_set_packing(outdir: Path):
    """Set Packing: select maximum-weight non-overlapping sets."""
    # 3 sets over 4 elements
    # set 0: {0, 2}
    # set 1: {1, 2}
    # set 2: {0, 3}
    sets = [[0, 2], [1, 2], [0, 3]]
    n_elements = 4
    n_sets = len(sets)
    weights = [1.0, 2.0, 1.5]
    penalty = 8.0

    # Build incidence matrix (elements x sets)
    a = np.zeros((n_elements, n_sets))
    for j, s in enumerate(sets):
        for i in s:
            a[i][j] = 1

    Q = qubogen.qubo_set_pack(a, np.array(weights), penalty=penalty)

    qubo_result = brute_force_qubo(Q)

    save_test("setpacking_to_qubo", {
        "problem": "SetPacking",
        "source": {
            "sets": sets,
            "num_elements": n_elements,
            "weights": weights,
            "penalty": penalty,
        },
        "qubo_matrix": Q.tolist(),
        "qubo_num_vars": int(Q.shape[0]),
        "qubo_optimal": qubo_result,
    }, outdir)


def generate_max2sat(outdir: Path):
    """Max 2-SAT: maximize satisfied clauses."""
    # 3 variables, 4 clauses:
    # (x0 OR x1), (NOT x0 OR x2), (x1 OR NOT x2), (NOT x1 OR NOT x2)
    literals = np.array([[0, 1], [0, 2], [1, 2], [1, 2]])
    signs = np.array(
        [[True, True], [False, True], [True, False], [False, False]]
    )

    c = qubogen.Clauses(literals=literals, signs=signs)
    Q = qubogen.qubo_max2sat(c)

    qubo_result = brute_force_qubo(Q)

    # Convert to list-of-clauses format matching our KSatisfiability model
    clauses = []
    for i in range(len(literals)):
        clause = []
        for j in range(2):
            var = int(literals[i][j])
            negated = not bool(signs[i][j])
            clause.append({"variable": var, "negated": negated})
        clauses.append(clause)

    save_test("ksatisfiability_to_qubo", {
        "problem": "KSatisfiability",
        "source": {"num_variables": 3, "clauses": clauses},
        "qubo_matrix": Q.tolist(),
        "qubo_num_vars": int(Q.shape[0]),
        "qubo_optimal": qubo_result,
    }, outdir)


def generate_ilp(outdir: Path):
    """Binary ILP (General 0/1 Programming): min c^T x, s.t. Ax <= b."""
    # 3 variables
    # minimize: x0 + 2*x1 + 3*x2
    # s.t.: x0 + x1 <= 1
    #        x1 + x2 <= 1
    cost = np.array([1.0, 2.0, 3.0])
    A = np.array([[1.0, 1.0, 0.0], [0.0, 1.0, 1.0]])
    b = np.array([1.0, 1.0])
    sign = np.array([-1, -1])  # -1 means <=
    penalty = 10.0

    Q = qubogen.qubo_general01(cost, A, b, sign, penalty=penalty)

    qubo_result = brute_force_qubo(Q)

    save_test("ilp_to_qubo", {
        "problem": "ILP",
        "source": {
            "num_variables": 3,
            "objective": cost.tolist(),
            "constraints_lhs": A.tolist(),
            "constraints_rhs": b.tolist(),
            "constraint_signs": sign.tolist(),
            "penalty": penalty,
        },
        "qubo_matrix": Q.tolist(),
        "qubo_num_vars": int(Q.shape[0]),
        "qubo_optimal": qubo_result,
    }, outdir)


def main():
    outdir = Path(__file__).resolve().parent.parent / "tests" / "data" / "qubo"
    outdir.mkdir(parents=True, exist_ok=True)

    print("Generating QUBO test datasets...")
    generate_vertex_covering(outdir)
    generate_independent_set(outdir)
    generate_graph_coloring(outdir)
    generate_set_packing(outdir)
    generate_max2sat(outdir)
    generate_ilp(outdir)
    print("Done.")


if __name__ == "__main__":
    main()
