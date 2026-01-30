using Pkg
Pkg.activate("/Users/liujinguo/.julia/dev/UnitDiskMapping")
using UnitDiskMapping
using Graphs

g = smallgraph(:diamond)
result = map_graph(g)

# Use the same MIS positions as Rust
mis_positions = Set([
    (4, 6), (4, 8), (4, 10), (6, 7), (6, 11), (7, 15), (8, 9), (8, 11), 
    (8, 13), (9, 15), (10, 11), (11, 15), (12, 13)
])

# Build config matrix
m, n = size(result.grid_graph)
config = zeros(Int, m, n)
for node in result.grid_graph.nodes
    if (node.loc[1], node.loc[2]) in mis_positions
        config[node.loc...] = 1
    end
end

println("=== JULIA DEBUG ===")
println("Config total before: $(sum(config))")
println("Grid size: $m x $n")

# Map config back
original = map_config_back(result, config)
println("\nMAP CONFIG BACK RESULT: $original")
println("Sum: $(sum(original))")
