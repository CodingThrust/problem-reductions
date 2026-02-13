#!/usr/bin/env julia
# Generate JSON test fixtures from ProblemReductions.jl for Rust parity testing.
# Run: cd scripts/jl && julia --project=. generate_testdata.jl

using ProblemReductions, Graphs, JSON

const OUTDIR = joinpath(@__DIR__, "..", "..", "tests", "data")
mkpath(OUTDIR)

# ── helpers ──────────────────────────────────────────────────────────

"""Convert a SimpleGraph to a sorted list of 0-based edges [[u,v], ...]."""
function graph_to_edges(g::SimpleGraph)
    return [[src(e) - 1, dst(e) - 1] for e in edges(g)]
end

"""Convert HyperGraph hyperedges to 0-based lists."""
function hypergraph_to_hyperedges(g::HyperGraph)
    return [sort([v - 1 for v in he]) for he in g.edges]
end

"""Write a JSON dict to tests/data/<filename>."""
function write_fixture(filename, data)
    path = joinpath(OUTDIR, filename)
    open(path, "w") do f
        JSON.print(f, data, 2)
    end
    println("  wrote $path")
end

"""Evaluate several configs on a problem and return evaluation dicts."""
function evaluate_configs(problem, configs)
    results = []
    for config in configs
        ss = solution_size(problem, config)
        d = Dict(
            "config" => config,
            "is_valid" => ss.is_valid,
            "size" => ss.size,
        )
        push!(results, d)
    end
    return results
end

"""Generate random binary configs for a problem."""
function sample_configs(problem; n=8)
    nv = num_variables(problem)
    nf = num_flavors(problem)
    total_configs = nf^nv
    n = min(n, total_configs)  # cap at total possible configs
    configs = Set{Vector{Int}}()
    # always include all-zeros and all-ones
    push!(configs, zeros(Int, nv))
    if nf == 2 && nv > 0
        push!(configs, ones(Int, nv))
    end
    # random samples
    while length(configs) < n
        push!(configs, [rand(0:nf-1) for _ in 1:nv])
    end
    return collect(configs)
end

# ── model serializers ────────────────────────────────────────────────

function serialize_graph_problem(problem, graph::SimpleGraph; weight_field=:weights)
    d = Dict(
        "num_vertices" => nv(graph),
        "edges" => graph_to_edges(graph),
    )
    w = getfield(problem, weight_field)
    if w isa UnitWeight
        d["weights"] = ones(Int, w.n)
    else
        d["weights"] = collect(w)
    end
    return d
end

function model_fixture(problem_type::String, instances)
    return Dict(
        "problem_type" => problem_type,
        "instances" => instances,
    )
end

function make_instance(label, instance_data, problem; extra=Dict())
    configs = sample_configs(problem; n=10)
    evals = evaluate_configs(problem, configs)
    best = findbest(problem, BruteForce())
    d = Dict(
        "label" => label,
        "instance" => instance_data,
        "evaluations" => evals,
        "best_solutions" => best,
    )
    merge!(d, extra)
    return d
end

# ── model exports ────────────────────────────────────────────────────

function export_independentset(graph, label, weights=nothing)
    if weights === nothing
        is = IndependentSet(graph)
    else
        is = IndependentSet(graph, weights)
    end
    inst = serialize_graph_problem(is, graph)
    return make_instance(label, inst, is)
end

function export_spinglass(sg, graph, label)
    inst = Dict(
        "num_vertices" => nv(graph),
        "edges" => graph_to_edges(graph),
        "J" => collect(sg.J),
        "h" => collect(sg.h),
    )
    return make_instance(label, inst, sg)
end

function export_maxcut(mc, graph, label)
    inst = Dict(
        "num_vertices" => nv(graph),
        "edges" => graph_to_edges(graph),
    )
    w = mc.weights
    if w isa UnitWeight
        inst["weights"] = ones(Int, w.n)
    else
        inst["weights"] = collect(w)
    end
    return make_instance(label, inst, mc)
end

function export_qubo(q, label)
    inst = Dict(
        "matrix" => [collect(q.matrix[i, :]) for i in 1:size(q.matrix, 1)],
    )
    return make_instance(label, inst, q)
end

function export_sat(sat, label)
    # Map symbols to 0-based integer indices
    syms = sat.symbols
    sym_to_idx = Dict(s => i - 1 for (i, s) in enumerate(syms))
    clauses = []
    for clause in sat.cnf.clauses
        lits = []
        for bv in clause.vars
            push!(lits, Dict(
                "variable" => sym_to_idx[bv.name],
                "negated" => bv.neg,
            ))
        end
        push!(clauses, Dict("literals" => lits))
    end
    inst = Dict(
        "num_variables" => length(syms),
        "clauses" => clauses,
    )
    return make_instance(label, inst, sat)
end

function export_ksat(ksat, k, label)
    syms = ksat.symbols
    sym_to_idx = Dict(s => i - 1 for (i, s) in enumerate(syms))
    clauses = []
    for clause in ksat.cnf.clauses
        lits = []
        for bv in clause.vars
            push!(lits, Dict(
                "variable" => sym_to_idx[bv.name],
                "negated" => bv.neg,
            ))
        end
        push!(clauses, Dict("literals" => lits))
    end
    inst = Dict(
        "num_variables" => length(syms),
        "clauses" => clauses,
        "k" => k,
    )
    return make_instance(label, inst, ksat)
end

function export_vertexcovering(vc, graph, label)
    inst = serialize_graph_problem(vc, graph)
    return make_instance(label, inst, vc)
end

function export_setpacking(sp, label)
    # convert sets to 0-based
    sets_0 = [sort([e - 1 for e in s]) for s in sp.sets]
    w = sp.weights
    if w isa UnitWeight
        wts = ones(Int, w.n)
    else
        wts = collect(w)
    end
    inst = Dict(
        "sets" => sets_0,
        "weights" => wts,
    )
    return make_instance(label, inst, sp)
end

function export_matching(m, graph, label)
    inst = Dict(
        "num_vertices" => nv(graph),
        "edges" => graph_to_edges(graph),
    )
    w = m.weights
    if w isa UnitWeight
        inst["weights"] = ones(Int, w.n)
    else
        inst["weights"] = collect(w)
    end
    return make_instance(label, inst, m)
end

function export_factoring(f, label)
    inst = Dict(
        "m" => f.m,
        "n" => f.n,
        "input" => f.input,
    )
    return make_instance(label, inst, f)
end

# ── reduction exports ────────────────────────────────────────────────

function export_reduction(source, target_type, source_label)
    println("  reducing $(typeof(source)) => $target_type [$source_label]")
    # direct solve source
    best_source = findbest(source, BruteForce())

    # reduce
    result = reduceto(target_type, source)
    target = target_problem(result)

    # solve target
    best_target = findbest(target, BruteForce())

    # extract solutions
    extracted_single = unique(extract_solution.(Ref(result), best_target))
    extracted_multiple = extract_multiple_solutions(result, best_target)

    return Dict(
        "label" => source_label,
        "best_source" => best_source,
        "best_target" => best_target,
        "extracted_single" => extracted_single,
        "extracted_multiple" => extracted_multiple,
    )
end

# ── main ─────────────────────────────────────────────────────────────

function main()
    println("Generating Julia parity test data...")

    # ── Build test instances (matching Julia test/rules/rules.jl) ──
    graph = smallgraph(:petersen)
    circuit = CircuitSAT(@circuit begin
        x = a ∨ ¬b
        y = ¬c ∨ b
        z = x ∧ y ∧ a
    end)
    maxcut = MaxCut(graph)
    spinglass = SpinGlass(graph, [1,2,1,2,1,2,1,2,1,2,1,2,1,2,1], zeros(Int, nv(graph)))
    vertexcovering = VertexCovering(graph, [1,2,1,2,1,2,1,2,1,2])
    sat = Satisfiability(CNF([CNFClause([BoolVar(:a), BoolVar(:b)])]))
    ksat = KSatisfiability{3}(CNF([CNFClause([BoolVar(:a), BoolVar(:b), BoolVar(:c)])]))
    graph2 = HyperGraph(3, [[1, 2], [1], [2,3], [2]])
    qubo = QUBO([0 1 -2; 1 0 -2; -2 -2 6])
    is = IndependentSet(graph)
    is2 = IndependentSet(graph2)
    setpacking = SetPacking([[1, 2, 5], [1, 3], [2, 4], [3, 6], [2, 3, 6]])
    matching = Matching(graph)

    # ── Export model fixtures ──
    println("Exporting model fixtures...")

    # IndependentSet (SimpleGraph)
    write_fixture("jl_independentset.json", model_fixture("IndependentSet", [
        export_independentset(graph, "petersen"),
    ]))

    # SpinGlass
    write_fixture("jl_spinglass.json", model_fixture("SpinGlass", [
        export_spinglass(spinglass, graph, "petersen"),
    ]))

    # MaxCut
    write_fixture("jl_maxcut.json", model_fixture("MaxCut", [
        export_maxcut(maxcut, graph, "petersen"),
    ]))

    # QUBO
    write_fixture("jl_qubo.json", model_fixture("QUBO", [
        export_qubo(qubo, "3x3_matrix"),
    ]))

    # Satisfiability
    write_fixture("jl_satisfiability.json", model_fixture("Satisfiability", [
        export_sat(sat, "simple_clause"),
    ]))

    # KSatisfiability
    write_fixture("jl_ksatisfiability.json", model_fixture("KSatisfiability", [
        export_ksat(ksat, 3, "simple_3sat"),
    ]))

    # VertexCovering
    write_fixture("jl_vertexcovering.json", model_fixture("VertexCovering", [
        export_vertexcovering(vertexcovering, graph, "petersen"),
    ]))

    # SetPacking
    write_fixture("jl_setpacking.json", model_fixture("SetPacking", [
        export_setpacking(setpacking, "five_sets"),
    ]))

    # Matching
    write_fixture("jl_matching.json", model_fixture("Matching", [
        export_matching(matching, graph, "petersen"),
    ]))

    # Factoring
    fact1 = Factoring(1, 1, 1)
    fact2 = Factoring(2, 1, 2)
    fact3 = Factoring(2, 1, 3)
    write_fixture("jl_factoring.json", model_fixture("Factoring", [
        export_factoring(fact1, "1x1_factor_1"),
        export_factoring(fact2, "2x1_factor_2"),
        export_factoring(fact3, "2x1_factor_3"),
    ]))

    # ── Export reduction fixtures ──
    println("Exporting reduction fixtures...")

    reduction_pairs = Any[
        (circuit,        SpinGlass{<:SimpleGraph}, "circuit",        "CircuitSAT",       "SpinGlass"),
        (maxcut,         SpinGlass{<:SimpleGraph}, "maxcut",         "MaxCut",            "SpinGlass"),
        (spinglass,      MaxCut,                   "spinglass",      "SpinGlass",         "MaxCut"),
        (vertexcovering, SetCovering,              "vertexcovering", "VertexCovering",    "SetCovering"),
        (sat,            Coloring{3},              "sat_col",        "Satisfiability",    "Coloring3"),
        (qubo,           SpinGlass{<:SimpleGraph}, "qubo",           "QUBO",              "SpinGlass"),
        (spinglass,      QUBO,                     "spinglass_qubo", "SpinGlass",         "QUBO"),
        (sat,            KSatisfiability{3},        "sat_ksat",       "Satisfiability",    "KSatisfiability3"),
        (ksat,           Satisfiability,            "ksat_sat",       "KSatisfiability",   "Satisfiability"),
        (sat,            IndependentSet{<:SimpleGraph}, "sat_is",    "Satisfiability",    "IndependentSet"),
        (sat,            DominatingSet{<:SimpleGraph},  "sat_ds",    "Satisfiability",    "DominatingSet"),
        (is,             SetPacking,               "is",             "IndependentSet",    "SetPacking"),
        (is2,            SetPacking,               "is2_hyper",      "IndependentSet_HyperGraph", "SetPacking"),
        (setpacking,     IndependentSet{<:SimpleGraph}, "sp",        "SetPacking",        "IndependentSet"),
        (is,             VertexCovering,            "is_vc",         "IndependentSet",    "VertexCovering"),
        (matching,       SetPacking,               "matching",       "Matching",          "SetPacking"),
        (fact1,          CircuitSAT,               "factoring",      "Factoring",         "CircuitSAT"),
    ]

    for (source, target_type, source_label, src_name, tgt_name) in reduction_pairs
        filename = "jl_$(lowercase(src_name))_to_$(lowercase(tgt_name)).json"
        case = export_reduction(source, target_type, source_label)
        data = Dict(
            "source_type" => src_name,
            "target_type" => tgt_name,
            "cases" => [case],
        )
        write_fixture(filename, data)
    end

    println("Done! Generated fixtures in $OUTDIR")
end

main()
