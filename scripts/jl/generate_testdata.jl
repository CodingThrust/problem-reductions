#!/usr/bin/env julia
# Generate JSON test fixtures from ProblemReductions.jl for Rust parity testing.
# Run: cd scripts/jl && julia --project=. generate_testdata.jl

using ProblemReductions, Graphs, JSON

const OUTDIR = joinpath(@__DIR__, "..", "..", "tests", "data", "jl")
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
        JSON.print(f, data)
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

function export_dominatingset(ds, graph, label)
    inst = serialize_graph_problem(ds, graph)
    return make_instance(label, inst, ds)
end

function export_maximalis(mis, graph, label)
    inst = serialize_graph_problem(mis, graph)
    return make_instance(label, inst, mis)
end

function export_paintshop(ps, label)
    inst = Dict(
        "sequence" => ps.sequence,
        "num_cars" => length(unique(ps.sequence)),
    )
    return make_instance(label, inst, ps)
end

function export_coloring(col, graph, k, label)
    inst = Dict(
        "num_vertices" => nv(graph),
        "edges" => graph_to_edges(graph),
        "k" => k,
    )
    return make_instance(label, inst, col)
end

function export_setcovering(sc, label)
    # convert sets to 0-based
    sets_0 = [sort([e - 1 for e in s]) for s in sc.sets]
    w = sc.weights
    if w isa UnitWeight
        wts = ones(Int, w.n)
    else
        wts = collect(w)
    end
    inst = Dict(
        "universe_size" => length(sc.elements),
        "sets" => sets_0,
        "weights" => wts,
    )
    return make_instance(label, inst, sc)
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

    # ── Doc example instances ──
    # IndependentSet docstring: 4-vertex graph
    doc_is_graph = SimpleGraph(Graphs.SimpleEdge.([(1, 2), (1, 3), (3, 4), (2, 3)]))
    doc_is = IndependentSet(doc_is_graph)
    # Tutorial: diamond graph
    doc_diamond = smallgraph(:diamond)
    doc_is_diamond = IndependentSet(doc_diamond)

    # SpinGlass docstring: 4-vertex graph
    doc_sg_graph = SimpleGraph(Graphs.SimpleEdge.([(1, 2), (1, 3), (3, 4), (2, 3)]))
    doc_sg = SpinGlass(doc_sg_graph, [1, -1, 1, -1], [1, -1, -1, 1])

    # MaxCut docstring: complete_graph(3) with weights
    doc_mc_graph = complete_graph(3)
    doc_mc = MaxCut(doc_mc_graph, [1, 2, 3])

    # QUBO docstring: identity matrix
    doc_qubo = QUBO([1. 0 0; 0 1 0; 0 0 1])

    # VertexCovering docstring: 4-vertex 5-edge graph with weights
    doc_vc_graph = SimpleGraph(Graphs.SimpleEdge.([(1,2), (1,3), (3,4), (2,3), (1,4)]))
    doc_vc = VertexCovering(doc_vc_graph, [1, 3, 1, 4])

    # Factoring docstring + Ising example: Factoring(2,2,6)
    doc_fact = Factoring(2, 2, 6)

    # DominatingSet docstring: path_graph(5)
    doc_ds_graph = path_graph(5)
    doc_ds = DominatingSet(doc_ds_graph)

    # MaximalIS docstring: 4-vertex 5-edge graph
    doc_mis_graph = SimpleGraph(Graphs.SimpleEdge.([(1, 2), (1, 3), (3, 4), (2, 3), (1, 4)]))
    doc_mis = MaximalIS(doc_mis_graph)

    # PaintShop docstring
    doc_ps = PaintShop(["a", "b", "a", "c", "c", "b"])

    # Coloring docstring: petersen graph, 3 colors
    doc_col = Coloring{3}(graph)

    # SetCovering docstring
    doc_sc = SetCovering([[1, 2, 3], [2, 4], [1, 4]], [1, 2, 3])

    # ── Individual rule test instances (from test/rules/*.jl) ──
    rule_graph4 = SimpleGraph(Graphs.SimpleEdge.([(1, 2), (1, 3), (3, 4), (2, 3)]))

    # spinglass_maxcut.jl: MaxCut with specific weights
    rule_mc = MaxCut(rule_graph4, [1, 3, 1, 4])
    # spinglass_maxcut.jl: SpinGlass with same weights
    rule_sg = SpinGlass(rule_graph4, [1, 3, 1, 4], zeros(Int, 4))

    # spinglass_qubo.jl: different QUBO matrix
    rule_qubo = QUBO([2 1 -2; 1 2 -2; -2 -2 2])

    # vertexcovering_setcovering.jl: VC with specific weights
    rule_vc = VertexCovering(rule_graph4, [1, 3, 1, 4])

    # independentset_setpacking.jl: g02 variant (different edge insertion order, same graph)
    rule_is_g02 = IndependentSet(SimpleGraph(Graphs.SimpleEdge.([(1, 3), (1, 2), (2, 3), (3, 4)])))

    # matching_setpacking.jl: 4-vertex matching (unweighted + weighted)
    rule_match_uw = Matching(rule_graph4)
    rule_match_w = Matching(rule_graph4, [1, 2, 3, 4])

    # sat_3sat.jl: multi-clause SAT
    rule_sat_3sat = Satisfiability(CNF([
        CNFClause([BoolVar(:x)]),
        CNFClause([BoolVar(:y, true), BoolVar(:z)]),
        CNFClause([BoolVar(:x), BoolVar(:y, true), BoolVar(:z), BoolVar(:w)]),
    ]))

    # sat_independentset.jl / sat_dominatingset.jl / circuit_sat.jl: 3-variable SAT instances
    x1, nx1 = BoolVar(:x1), BoolVar(:x1, true)
    x2, nx2 = BoolVar(:x2), BoolVar(:x2, true)
    x3, nx3 = BoolVar(:x3), BoolVar(:x3, true)

    rule_sat01 = Satisfiability(CNF([
        CNFClause([x1, nx2, x3]),
        CNFClause([nx1, x2, nx3]),
        CNFClause([x1, nx2, nx3]),
        CNFClause([nx1, x2, x3]),
    ]))
    rule_sat02 = Satisfiability(CNF([
        CNFClause([nx1, x2, x3]),
        CNFClause([x1, nx2, x3]),
        CNFClause([x1, x2, nx3]),
    ]))
    rule_sat03 = Satisfiability(CNF([
        CNFClause([x1, x2, x3]),
        CNFClause([nx1, nx2, nx3]),
    ]))
    # Unsatisfiable instances
    rule_sat04 = Satisfiability(CNF([
        CNFClause([x1, x1, x1]),
        CNFClause([nx1, nx1, nx1]),
    ]))
    rule_sat05 = Satisfiability(CNF([CNFClause([x1]), CNFClause([nx1])]))
    rule_sat06 = Satisfiability(CNF([
        CNFClause([x1, x2]),
        CNFClause([x1, nx2]),
        CNFClause([nx1, x2]),
        CNFClause([nx1, nx2]),
    ]))
    rule_sat07 = Satisfiability(CNF([
        CNFClause([x1, x2]),
        CNFClause([x1, nx2]),
        CNFClause([nx1, x2]),
    ]))

    # sat_coloring.jl
    rule_sat_col = Satisfiability(CNF([
        CNFClause([BoolVar(:X), BoolVar(:Y)]),
        CNFClause([BoolVar(:X), BoolVar(:Y, true)]),
    ]))

    # ── Export model fixtures ──
    println("Exporting model fixtures...")

    # IndependentSet (SimpleGraph)
    write_fixture("independentset.json", model_fixture("IndependentSet", [
        export_independentset(graph, "petersen"),
        export_independentset(doc_is_graph, "doc_4vertex"),
        export_independentset(doc_diamond, "doc_diamond"),
    ]))

    # SpinGlass
    write_fixture("spinglass.json", model_fixture("SpinGlass", [
        export_spinglass(spinglass, graph, "petersen"),
        export_spinglass(doc_sg, doc_sg_graph, "doc_4vertex"),
        export_spinglass(rule_sg, rule_graph4, "rule_4vertex"),
    ]))

    # MaxCut
    write_fixture("maxcut.json", model_fixture("MaxCut", [
        export_maxcut(maxcut, graph, "petersen"),
        export_maxcut(doc_mc, doc_mc_graph, "doc_k3"),
        export_maxcut(rule_mc, rule_graph4, "rule_4vertex"),
    ]))

    # QUBO
    write_fixture("qubo.json", model_fixture("QUBO", [
        export_qubo(qubo, "3x3_matrix"),
        export_qubo(doc_qubo, "doc_identity"),
        export_qubo(rule_qubo, "rule_3x3"),
    ]))

    # Satisfiability
    write_fixture("satisfiability.json", model_fixture("Satisfiability", [
        export_sat(sat, "simple_clause"),
        export_sat(rule_sat_3sat, "rule_3sat_multi"),
        export_sat(rule_sat01, "rule_sat01"),
        export_sat(rule_sat02, "rule_sat02"),
        export_sat(rule_sat03, "rule_sat03"),
        export_sat(rule_sat04, "rule_sat04_unsat"),
        export_sat(rule_sat05, "rule_sat05_unsat"),
        export_sat(rule_sat06, "rule_sat06_unsat"),
        export_sat(rule_sat07, "rule_sat07"),
        export_sat(rule_sat_col, "rule_sat_coloring"),
    ]))

    # KSatisfiability
    write_fixture("ksatisfiability.json", model_fixture("KSatisfiability", [
        export_ksat(ksat, 3, "simple_3sat"),
    ]))

    # VertexCovering
    write_fixture("vertexcovering.json", model_fixture("VertexCovering", [
        export_vertexcovering(vertexcovering, graph, "petersen"),
        export_vertexcovering(doc_vc, doc_vc_graph, "doc_4vertex"),
        export_vertexcovering(rule_vc, rule_graph4, "rule_4vertex"),
    ]))

    # SetPacking
    write_fixture("setpacking.json", model_fixture("SetPacking", [
        export_setpacking(setpacking, "five_sets"),
    ]))

    # Matching
    write_fixture("matching.json", model_fixture("Matching", [
        export_matching(matching, graph, "petersen"),
        export_matching(rule_match_uw, rule_graph4, "rule_4vertex"),
        export_matching(rule_match_w, rule_graph4, "rule_4vertex_weighted"),
    ]))

    # Factoring
    fact1 = Factoring(1, 1, 1)
    fact2 = Factoring(2, 1, 2)
    fact3 = Factoring(2, 1, 3)
    write_fixture("factoring.json", model_fixture("Factoring", [
        export_factoring(fact1, "1x1_factor_1"),
        export_factoring(fact2, "2x1_factor_2"),
        export_factoring(fact3, "2x1_factor_3"),
        export_factoring(doc_fact, "doc_factor6"),
    ]))

    # DominatingSet (doc example)
    write_fixture("dominatingset.json", model_fixture("DominatingSet", [
        export_dominatingset(doc_ds, doc_ds_graph, "doc_path5"),
    ]))

    # MaximalIS (doc example)
    write_fixture("maximalis.json", model_fixture("MaximalIS", [
        export_maximalis(doc_mis, doc_mis_graph, "doc_4vertex"),
    ]))

    # PaintShop (doc example)
    write_fixture("paintshop.json", model_fixture("PaintShop", [
        export_paintshop(doc_ps, "doc_abaccb"),
    ]))

    # Coloring (doc example)
    write_fixture("coloring.json", model_fixture("Coloring", [
        export_coloring(doc_col, graph, 3, "doc_petersen_3color"),
    ]))

    # SetCovering (doc example)
    write_fixture("setcovering.json", model_fixture("SetCovering", [
        export_setcovering(doc_sc, "doc_3subsets"),
    ]))

    # ── Export reduction fixtures ──
    println("Exporting reduction fixtures...")

    # ── Reduction pairs: rules.jl round-trip + doc examples ──
    reduction_pairs = Any[
        (doc_is,         SetPacking,               "doc_is",         "doc_IndependentSet", "SetPacking"),
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

    # ── Reduction pairs: individual rule test instances (test/rules/*.jl) ──
    rule_reduction_pairs = Any[
        # spinglass_maxcut.jl
        (rule_mc,        SpinGlass{<:SimpleGraph}, "rule_mc",        "rule_MaxCut",       "SpinGlass"),
        (rule_sg,        MaxCut,                   "rule_sg",        "rule_SpinGlass",    "MaxCut"),
        # spinglass_qubo.jl
        (rule_qubo,      SpinGlass{<:SimpleGraph}, "rule_qubo",      "rule_QUBO",         "SpinGlass"),
        # vertexcovering_setcovering.jl
        (rule_vc,        SetCovering,              "rule_vc",        "rule_VertexCovering", "SetCovering"),
        # independentset_setpacking.jl
        (rule_is_g02,    SetPacking,               "rule_is_g02",    "rule_IndependentSet", "SetPacking"),
        # vertexcovering_independentset.jl
        (doc_is,         VertexCovering,            "rule_is_vc",    "rule2_IndependentSet", "VertexCovering"),
        # matching_setpacking.jl (unweighted + weighted)
        (rule_match_uw,  SetPacking,               "rule_match_uw",  "rule_Matching",     "SetPacking"),
        (rule_match_w,   SetPacking,               "rule_match_w",   "rule_MatchingW",    "SetPacking"),
        # sat_3sat.jl
        (rule_sat_3sat,  KSatisfiability{3},        "rule_sat_3sat",  "rule_Satisfiability", "KSatisfiability3"),
        # circuit_sat.jl (SAT → CircuitSAT)
        (rule_sat01,     CircuitSAT,               "rule_sat01",     "rule_SAT01",        "CircuitSAT"),
        (rule_sat02,     CircuitSAT,               "rule_sat02",     "rule_SAT02",        "CircuitSAT"),
        (rule_sat03,     CircuitSAT,               "rule_sat03",     "rule_SAT03",        "CircuitSAT"),
        # sat_coloring.jl
        (rule_sat_col,   Coloring{3},              "rule_sat_col",   "rule_Satisfiability2", "Coloring3"),
        # sat_independentset.jl
        (rule_sat01,     IndependentSet{<:SimpleGraph}, "rule_sat01", "rule_SAT01",       "IndependentSet"),
        (rule_sat02,     IndependentSet{<:SimpleGraph}, "rule_sat02", "rule_SAT02",       "IndependentSet"),
        (rule_sat03,     IndependentSet{<:SimpleGraph}, "rule_sat03", "rule_SAT03",       "IndependentSet"),
        (rule_sat04,     IndependentSet{<:SimpleGraph}, "rule_sat04", "rule_SAT04_unsat", "IndependentSet"),
        (rule_sat07,     IndependentSet{<:SimpleGraph}, "rule_sat07", "rule_SAT07",       "IndependentSet"),
        # sat_dominatingset.jl
        (rule_sat01,     DominatingSet{<:SimpleGraph}, "rule_sat01",  "rule_SAT01",       "DominatingSet"),
        (rule_sat02,     DominatingSet{<:SimpleGraph}, "rule_sat02",  "rule_SAT02",       "DominatingSet"),
        (rule_sat03,     DominatingSet{<:SimpleGraph}, "rule_sat03",  "rule_SAT03",       "DominatingSet"),
        (rule_sat04,     DominatingSet{<:SimpleGraph}, "rule_sat04",  "rule_SAT04_unsat", "DominatingSet"),
        (rule_sat07,     DominatingSet{<:SimpleGraph}, "rule_sat07",  "rule_SAT07",       "DominatingSet"),
    ]
    append!(reduction_pairs, rule_reduction_pairs)

    for (source, target_type, source_label, src_name, tgt_name) in reduction_pairs
        filename = "$(lowercase(src_name))_to_$(lowercase(tgt_name)).json"
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
