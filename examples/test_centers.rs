use problemreductions::rules::unitdiskmapping::map_graph;
use problemreductions::topology::{smallgraph, Graph};
use std::collections::HashSet;

fn main() {
    let (n, edges) = smallgraph("petersen").unwrap();
    let result = map_graph(n, &edges);

    println!("=== Petersen Graph Mapping ===");
    println!("Vertices: {}", n);
    println!("Grid nodes: {}", result.grid_graph.num_vertices());

    // Build position set
    let positions: HashSet<(i32, i32)> = result.grid_graph.nodes()
        .iter()
        .map(|n| (n.row, n.col))
        .collect();

    println!("\n=== Copy Line Centers ===");
    for line in &result.lines {
        let (row, col) = line.center_location(result.padding, result.spacing);
        let exists = positions.contains(&(row as i32, col as i32));
        println!(
            "Vertex {}: center=({}, {}), exists_in_grid={}",
            line.vertex, row, col, exists
        );
    }

    println!("\n=== Dense Locations vs Grid ===");
    for line in &result.lines {
        let locs = line.dense_locations(result.padding, result.spacing);
        let in_grid: Vec<_> = locs.iter()
            .filter(|(r, c, _)| positions.contains(&(*r as i32, *c as i32)))
            .collect();
        println!(
            "Vertex {}: dense_locs={}, in_grid={}",
            line.vertex, locs.len(), in_grid.len()
        );
    }
}
