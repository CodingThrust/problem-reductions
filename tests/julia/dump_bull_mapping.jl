"""
Dump graph mapping information for comparison with Rust implementation.
Follows the testing pattern from UnitDiskMapping/test/mapping.jl.

Supports modes: UnWeighted(), Weighted(), TriangularWeighted()
"""

using Pkg
Pkg.activate(@__DIR__)
Pkg.develop(path="/Users/liujinguo/.julia/dev/UnitDiskMapping")
Pkg.instantiate()

using UnitDiskMapping
using Graphs
using GenericTensorNetworks
using UnitDiskMapping: is_independent_set

# Manual JSON serialization (no external deps)
function to_json(obj, indent=0)
    ind = "  " ^ indent
    ind1 = "  " ^ (indent + 1)

    if obj isa Dict
        isempty(obj) && return "{}"
        parts = ["$(ind1)\"$k\": $(to_json(v, indent + 1))" for (k, v) in pairs(obj)]
        return "{\n$(join(parts, ",\n"))\n$ind}"
    elseif obj isa Vector
        isempty(obj) && return "[]"
        all(x -> x isa Number, obj) && return "[" * join(obj, ", ") * "]"
        parts = [to_json(x, indent + 1) for x in obj]
        return "[\n$(ind1)" * join(parts, ",\n$(ind1)") * "\n$ind]"
    elseif obj isa String
        return "\"$(escape_string(obj))\""
    elseif obj isa Number
        return string(obj)
    elseif obj isa Bool
        return obj ? "true" : "false"
    elseif obj === nothing
        return "null"
    else
        return "\"$(escape_string(string(obj)))\""
    end
end

function dump_mapping_info(mode, g, name)
    # Get mapping result using the specified mode
    res = map_graph(mode, g)
    mode_name = string(typeof(mode))

    # Capture grid state at different stages for comparison
    # Stage 1: After embed_graph (copylines only, includes Connected cells at crossings)
    ug_copylines_only = UnitDiskMapping.embed_graph(mode, g)

    # Stage 2: After crossing gadgets (before simplifiers)
    ug_before_simplify = UnitDiskMapping.embed_graph(mode, g)
    ug_before_simplify, crossing_tape = UnitDiskMapping.apply_crossing_gadgets!(mode, ug_before_simplify)

    info = Dict{String, Any}()
    info["graph_name"] = name
    info["mode"] = mode_name
    info["num_vertices"] = nv(g)
    info["num_edges"] = ne(g)
    info["edges"] = [[src(e), dst(e)] for e in edges(g)]

    # Grid graph info
    m, n = size(res.grid_graph)
    info["grid_size"] = [m, n]
    info["padding"] = res.padding
    info["mis_overhead"] = res.mis_overhead
    info["num_grid_nodes"] = length(res.grid_graph.nodes)

    # Grid nodes with positions and weights (AFTER simplifiers - final result)
    nodes_info = [Dict(
        "index" => i,
        "row" => node.loc[1],
        "col" => node.loc[2],
        "weight" => node.weight isa Number ? node.weight : 1
    ) for (i, node) in enumerate(res.grid_graph.nodes)]
    info["grid_nodes"] = nodes_info

    # Grid nodes BEFORE simplifiers (after copylines + crossing gadgets)
    # Extract from ug_before_simplify.content which is a matrix of cells
    # Include cell state: O=Occupied, D=Doubled, C=Connected
    nodes_before_simplifiers = []
    for i in 1:size(ug_before_simplify.content, 1)
        for j in 1:size(ug_before_simplify.content, 2)
            cell = ug_before_simplify.content[i, j]
            if !isempty(cell)
                state = if cell.doubled
                    "D"
                elseif cell.connected
                    "C"
                else
                    "O"
                end
                push!(nodes_before_simplifiers, Dict("row" => i, "col" => j, "state" => state))
            end
        end
    end
    info["grid_nodes_before_simplifiers"] = nodes_before_simplifiers
    info["num_grid_nodes_before_simplifiers"] = length(nodes_before_simplifiers)

    # Grid nodes AFTER copylines only (before crossing gadgets)
    # Include cell state: O=Occupied, D=Doubled, C=Connected
    nodes_copylines_only = []
    for i in 1:size(ug_copylines_only.content, 1)
        for j in 1:size(ug_copylines_only.content, 2)
            cell = ug_copylines_only.content[i, j]
            if !isempty(cell)
                state = if cell.doubled
                    "D"
                elseif cell.connected
                    "C"
                else
                    "O"
                end
                push!(nodes_copylines_only, Dict("row" => i, "col" => j, "state" => state))
            end
        end
    end
    info["grid_nodes_copylines_only"] = nodes_copylines_only
    info["num_grid_nodes_copylines_only"] = length(nodes_copylines_only)

    # Tape entries (mapping_history)
    tape_info = [Dict(
        "index" => i,
        "type" => string(typeof(entry[1])),
        "row" => entry[2],
        "col" => entry[3]
    ) for (i, entry) in enumerate(res.mapping_history)]
    info["tape"] = tape_info
    info["num_tape_entries"] = length(tape_info)

    # Copy lines info with locations
    spacing = res.spacing
    lines_info = []
    for (i, line) in enumerate(res.lines)
        # Get copyline_locations for this line
        locs = UnitDiskMapping.copyline_locations(UnitDiskMapping.nodetype(mode), line, res.padding, spacing)
        locs_info = [Dict("row" => loc.loc[1], "col" => loc.loc[2]) for loc in locs]

        push!(lines_info, Dict(
            "index" => i,
            "vertex" => line.vertex,
            "vslot" => line.vslot,
            "hslot" => line.hslot,
            "vstart" => line.vstart,
            "vstop" => line.vstop,
            "hstop" => line.hstop,
            "locations" => locs_info
        ))
    end
    info["copy_lines"] = lines_info

    # Solve optimal MIS/WMIS using GenericTensorNetworks
    missize_original = solve(GenericTensorNetwork(IndependentSet(g)), SizeMax())[].n
    info["original_mis_size"] = missize_original

    # For weighted modes (Weighted, TriangularWeighted), solve weighted MIS
    # For unweighted mode, solve standard MIS
    if mode isa UnitDiskMapping.UnWeighted
        # Unweighted: use SimpleGraph directly
        gp = GenericTensorNetwork(IndependentSet(SimpleGraph(res.grid_graph));
                                  optimizer=TreeSA(ntrials=1, niters=10))
        missize_map = solve(gp, SizeMax())[].n
        info["mapped_mis_size"] = missize_map
        info["overhead_check"] = res.mis_overhead + missize_original == missize_map

        # Get optimal MIS configuration
        misconfig = solve(gp, SingleConfigMax())[].c
        selected_positions = [Dict(
            "node_index" => i,
            "row" => res.grid_graph.nodes[i].loc[1],
            "col" => res.grid_graph.nodes[i].loc[2]
        ) for i in 1:length(misconfig.data) if misconfig.data[i] > 0]
        info["mis_selected_positions"] = selected_positions
        info["mis_selected_count"] = length(selected_positions)

        # Map config back
        original_configs = map_config_back(res, collect(misconfig.data))
        info["original_config"] = collect(original_configs)
        info["mapped_back_size"] = count(isone, original_configs)
        info["is_valid_is"] = is_independent_set(g, original_configs)
        info["size_matches"] = count(isone, original_configs) == missize_original
    else
        # Weighted modes: use unitdisk_graph to construct edges, then solve weighted MIS
        try
            # Get unit disk graph edges from grid positions
            grid_graph = res.grid_graph
            nodes = grid_graph.nodes
            n_nodes = length(nodes)

            # Construct SimpleGraph for topology
            sg = SimpleGraph(n_nodes)
            unit = mode isa UnitDiskMapping.TriangularWeighted ? 1.1 : 1.5
            grid_type = mode isa UnitDiskMapping.TriangularWeighted ? TriangularGrid() : SquareGrid()

            # Compute physical positions and add edges
            physical_locs = [UnitDiskMapping.physical_position(node, grid_type) for node in nodes]
            for i in 1:n_nodes
                for j in i+1:n_nodes
                    dist_sq = sum(abs2, physical_locs[i] .- physical_locs[j])
                    if dist_sq < unit^2
                        add_edge!(sg, i, j)
                    end
                end
            end

            # Get weights
            weights = [node.weight for node in nodes]

            # Solve weighted MIS
            gp = GenericTensorNetwork(IndependentSet(sg, weights);
                                      optimizer=TreeSA(ntrials=1, niters=10))
            wmis_result = solve(gp, SizeMax())[]
            missize_map = wmis_result.n

            info["mapped_mis_size"] = missize_map
            info["num_grid_edges"] = ne(sg)
            info["overhead_check"] = res.mis_overhead + missize_original == missize_map

            # Get optimal configuration
            misconfig = solve(gp, SingleConfigMax())[].c
            selected_positions = [Dict(
                "node_index" => i,
                "row" => nodes[i].loc[1],
                "col" => nodes[i].loc[2],
                "weight" => weights[i]
            ) for i in 1:length(misconfig.data) if misconfig.data[i] > 0]
            info["mis_selected_positions"] = selected_positions
            info["mis_selected_count"] = length(selected_positions)
            info["mis_selected_weight"] = sum(weights[i] for i in 1:length(misconfig.data) if misconfig.data[i] > 0; init=0)

            # Map config back
            original_configs = map_config_back(res, collect(misconfig.data))
            info["original_config"] = collect(original_configs)
            info["mapped_back_size"] = count(isone, original_configs)
            info["is_valid_is"] = is_independent_set(g, original_configs)
            info["size_matches"] = count(isone, original_configs) == missize_original
        catch e
            println("  Note: Error solving weighted MIS: $e")
            info["mapped_mis_size"] = nothing
            info["overhead_check"] = nothing
            info["mis_selected_positions"] = []
            info["mis_selected_count"] = 0
            info["original_config"] = []
            info["mapped_back_size"] = 0
            info["is_valid_is"] = nothing
            info["size_matches"] = nothing
        end
    end

    return info
end

# Test multiple graphs with different modes
function main()
    modes = [
        ("unweighted", UnitDiskMapping.UnWeighted()),
        ("weighted", UnitDiskMapping.Weighted()),
        ("triangular", UnitDiskMapping.TriangularWeighted()),
    ]

    graphs = [
        ("diamond", smallgraph(:diamond)),
        ("bull", smallgraph(:bull)),
        ("house", smallgraph(:house)),
        ("petersen", smallgraph(:petersen)),
    ]

    for (mode_name, mode) in modes
        println("\n" * "="^60)
        println("MODE: $mode_name")
        println("="^60)

        for (graph_name, g) in graphs
            println("\n--- $graph_name graph ($mode_name) ---")

            try
                info = dump_mapping_info(mode, g, graph_name)

                # Save to JSON
                output_path = joinpath(@__DIR__, "$(graph_name)_$(mode_name)_trace.json")
                open(output_path, "w") do f
                    write(f, to_json(info))
                end
                println("Saved to: $output_path")

                # Print summary
                println("  Vertices: $(info["num_vertices"])")
                println("  Grid size: $(info["grid_size"][1]) x $(info["grid_size"][2])")
                println("  Grid nodes: $(info["num_grid_nodes"])")
                println("  Tape entries: $(info["num_tape_entries"])")
                println("  Original MIS: $(info["original_mis_size"])")
                println("  Mapped MIS: $(info["mapped_mis_size"])")
                println("  MIS overhead: $(info["mis_overhead"])")
                println("  Overhead check: $(info["overhead_check"])")
                println("  Original config: $(info["original_config"])")
                println("  Is valid IS: $(info["is_valid_is"])")
                println("  Size matches: $(info["size_matches"])")
            catch e
                println("  ERROR: $e")
            end
        end
    end
end

main()
