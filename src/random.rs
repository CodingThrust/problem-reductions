//! Shared deterministic building blocks for model-owned random generators.

use crate::registry::ConstructionError;
use crate::topology::SimpleGraph;
use serde::Deserialize;

/// Inputs shared by models generated from an Erdős–Rényi simple graph.
#[derive(Debug, Deserialize, crate::CreateSpec)]
pub struct SimpleGraphRandomSpec {
    /// Number of graph vertices.
    pub num_vertices: usize,
    /// Independent probability of including each possible edge (default: 0.5).
    pub edge_prob: Option<f64>,
    /// Seed for reproducible generation.
    pub seed: Option<u64>,
}

/// Inputs shared by integer-lattice graph generators.
#[derive(Debug, Deserialize, crate::CreateSpec)]
pub struct IntegerGeometryRandomSpec {
    /// Number of graph vertices.
    pub num_vertices: usize,
    /// Seed for reproducible generation.
    pub seed: Option<u64>,
}

/// Inputs shared by unit-disk graph generators.
#[derive(Debug, Deserialize, crate::CreateSpec)]
pub struct UnitDiskRandomSpec {
    /// Number of graph vertices.
    pub num_vertices: usize,
    /// Disk radius used to derive edges (default: 1.0).
    pub radius: Option<f64>,
    /// Seed for reproducible generation.
    pub seed: Option<u64>,
}

/// Random simple-graph inputs with a required clique size.
#[derive(Debug, Deserialize, crate::CreateSpec)]
pub struct CliqueRandomSpec {
    /// Number of graph vertices.
    pub num_vertices: usize,
    /// Independent edge probability (default: 0.5).
    pub edge_prob: Option<f64>,
    /// Seed for reproducible generation.
    pub seed: Option<u64>,
    /// Required clique size.
    pub k: usize,
}

impl CliqueRandomSpec {
    /// Generate the graph using the common graph inputs.
    pub fn graph(&self) -> Result<SimpleGraph, String> {
        SimpleGraphRandomSpec {
            num_vertices: self.num_vertices,
            edge_prob: self.edge_prob,
            seed: self.seed,
        }
        .graph()
    }
}

/// Random simple-graph inputs with optional source and sink vertices.
#[derive(Debug, Deserialize, crate::CreateSpec)]
pub struct EndpointRandomSpec {
    /// Number of graph vertices.
    pub num_vertices: usize,
    /// Independent edge probability (default: 0.5).
    pub edge_prob: Option<f64>,
    /// Seed for reproducible generation.
    pub seed: Option<u64>,
    /// Source vertex (default: 0).
    pub source: Option<usize>,
    /// Sink vertex (default: the final vertex).
    pub sink: Option<usize>,
}

/// Random simple-graph inputs with an optional runtime color count.
#[derive(Debug, Deserialize, crate::CreateSpec)]
pub struct ColoringRandomSpec {
    /// Number of graph vertices.
    pub num_vertices: usize,
    /// Independent edge probability (default: 0.5).
    pub edge_prob: Option<f64>,
    /// Seed for reproducible generation.
    pub seed: Option<u64>,
    /// Runtime color count (default: 3).
    pub k: Option<usize>,
}

impl ColoringRandomSpec {
    /// Generate the graph using the common graph inputs.
    pub fn graph(&self) -> Result<SimpleGraph, String> {
        SimpleGraphRandomSpec {
            num_vertices: self.num_vertices,
            edge_prob: self.edge_prob,
            seed: self.seed,
        }
        .graph()
    }
}

impl EndpointRandomSpec {
    /// Generate the graph using the common graph inputs.
    pub fn graph(&self) -> Result<SimpleGraph, String> {
        SimpleGraphRandomSpec {
            num_vertices: self.num_vertices,
            edge_prob: self.edge_prob,
            seed: self.seed,
        }
        .graph()
    }

    /// Validate and return distinct source and sink vertices.
    pub fn endpoints(&self) -> Result<(usize, usize), String> {
        if self.num_vertices < 2 {
            return Err("num_vertices must be at least 2".to_string());
        }
        let source = self.source.unwrap_or(0);
        let sink = self.sink.unwrap_or(self.num_vertices - 1);
        if source >= self.num_vertices || sink >= self.num_vertices {
            return Err(format!(
                "source and sink must be below num_vertices ({})",
                self.num_vertices
            ));
        }
        if source == sink {
            return Err("source and sink must be distinct".to_string());
        }
        Ok((source, sink))
    }
}

impl SimpleGraphRandomSpec {
    /// Generate the requested graph after validating its probability.
    pub fn graph(&self) -> Result<SimpleGraph, String> {
        let edge_prob = self.edge_prob.unwrap_or(0.5);
        if !(0.0..=1.0).contains(&edge_prob) {
            return Err(format!(
                "edge_prob must be between 0 and 1, got {edge_prob}"
            ));
        }
        Ok(create_random_graph(self.num_vertices, edge_prob, self.seed))
    }
}

/// Implement a typed, model-owned random generator using a typed input spec.
#[macro_export]
macro_rules! impl_random_generate {
    ($target:ty, $spec:ty, |$input:ident| $body:block) => {
        impl $crate::registry::RandomGenerate for $target {
            const INPUTS: &'static [$crate::registry::CreateInputInfo] =
                <$spec as $crate::registry::CreateSpec>::INPUTS;

            fn generate(
                data: serde_json::Value,
            ) -> Result<Self, $crate::registry::ConstructionError> {
                $crate::registry::validate_create_inputs(Self::INPUTS, &data)?;
                let $input: $spec = <$spec as $crate::registry::CreateSpec>::deserialize_inputs(
                    data,
                )
                .map_err(|error| {
                    $crate::registry::ConstructionError::InvalidInput(error.to_string())
                })?;
                let generate = || -> Result<Self, String> { $body };
                let result = generate();
                result.map_err($crate::registry::ConstructionError::Conversion)
            }
        }
    };
}

/// LCG PRNG step returning a uniform value in `[0, 1)`.
pub fn lcg_step(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*state >> 33) as f64 / (1u64 << 31) as f64
}

/// Initialize LCG state from a seed or the current time.
pub fn lcg_init(seed: Option<u64>) -> u64 {
    seed.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock must be after the Unix epoch")
            .as_nanos() as u64
    })
}

/// Generate an Erdős–Rényi simple graph.
pub fn create_random_graph(num_vertices: usize, edge_prob: f64, seed: Option<u64>) -> SimpleGraph {
    let mut state = lcg_init(seed);
    let edges = (0..num_vertices)
        .flat_map(|u| ((u + 1)..num_vertices).map(move |v| (u, v)))
        .filter(|_| lcg_step(&mut state) < edge_prob)
        .collect();
    SimpleGraph::new(num_vertices, edges)
}

/// Generate unique integer positions on a square grid.
pub fn create_random_int_positions(num_vertices: usize, seed: Option<u64>) -> Vec<(i32, i32)> {
    let mut state = lcg_init(seed);
    let grid_size = (num_vertices as f64).sqrt().ceil() as i32 + 1;
    let capacity = (grid_size * grid_size) as usize;
    lcg_choose(&mut state, capacity, num_vertices)
        .expect("grid capacity exceeds the requested position count")
        .into_iter()
        .map(|index| (index as i32 / grid_size, index as i32 % grid_size))
        .collect()
}

/// Generate float positions in `[0, sqrt(N)]²`.
pub fn create_random_float_positions(num_vertices: usize, seed: Option<u64>) -> Vec<(f64, f64)> {
    let mut state = lcg_init(seed);
    let side = (num_vertices as f64).sqrt();
    (0..num_vertices)
        .map(|_| (lcg_step(&mut state) * side, lcg_step(&mut state) * side))
        .collect()
}

/// Choose `k` distinct sorted indices from `0..n`.
pub fn lcg_choose(state: &mut u64, n: usize, k: usize) -> Result<Vec<usize>, ConstructionError> {
    if k > n {
        return Err(ConstructionError::Conversion(format!(
            "cannot choose {k} elements from {n}"
        )));
    }
    let mut indices = (0..n).collect::<Vec<_>>();
    for i in 0..k {
        let j = i + (lcg_step(state) * (n - i) as f64) as usize % (n - i);
        indices.swap(i, j);
    }
    let mut chosen = indices[..k].to_vec();
    chosen.sort_unstable();
    Ok(chosen)
}
