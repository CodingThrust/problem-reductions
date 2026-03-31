//! Square Tiling (Wang Tiling) problem implementation.
//!
//! Given a set C of colors, a collection T of tiles (each with four colored edges:
//! top, right, bottom, left), and a positive integer N, determine whether there
//! exists a tiling of an N x N grid using tiles from T such that adjacent tiles
//! have matching edge colors. Tiles may be reused but not rotated or reflected.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SquareTiling",
        display_name: "Square Tiling",
        aliases: &["WangTiling"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Place colored square tiles on an N x N grid with matching edge colors",
        fields: &[
            FieldInfo { name: "num_colors", type_name: "usize", description: "Number of colors" },
            FieldInfo { name: "tiles", type_name: "Vec<(usize, usize, usize, usize)>", description: "Collection of tile types (top, right, bottom, left)" },
            FieldInfo { name: "grid_size", type_name: "usize", description: "Grid dimension N for N x N tiling" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "SquareTiling",
        fields: &["num_colors", "num_tiles", "grid_size"],
    }
}

/// A tile with four colored edges: (top, right, bottom, left).
/// Each color is an index in `0..num_colors`.
pub type Tile = (usize, usize, usize, usize);

#[derive(Debug, Clone, Serialize)]
pub struct SquareTiling {
    num_colors: usize,
    tiles: Vec<Tile>,
    grid_size: usize,
}

impl SquareTiling {
    fn validate_inputs(num_colors: usize, tiles: &[Tile], grid_size: usize) -> Result<(), String> {
        if num_colors == 0 {
            return Err("SquareTiling requires at least one color".to_string());
        }
        if tiles.is_empty() {
            return Err("SquareTiling requires at least one tile".to_string());
        }
        if grid_size == 0 {
            return Err("SquareTiling requires grid_size >= 1".to_string());
        }
        for (i, &(top, right, bottom, left)) in tiles.iter().enumerate() {
            if top >= num_colors
                || right >= num_colors
                || bottom >= num_colors
                || left >= num_colors
            {
                return Err(format!(
                    "Tile {} has color(s) out of range 0..{}",
                    i, num_colors
                ));
            }
        }
        Ok(())
    }

    /// Create a new `SquareTiling` instance, returning an error if inputs are invalid.
    pub fn try_new(num_colors: usize, tiles: Vec<Tile>, grid_size: usize) -> Result<Self, String> {
        Self::validate_inputs(num_colors, &tiles, grid_size)?;
        Ok(Self {
            num_colors,
            tiles,
            grid_size,
        })
    }

    /// Create a new `SquareTiling` instance.
    ///
    /// # Panics
    ///
    /// Panics if `num_colors` is 0, `tiles` is empty, `grid_size` is 0,
    /// or any tile color is out of range.
    pub fn new(num_colors: usize, tiles: Vec<Tile>, grid_size: usize) -> Self {
        Self::try_new(num_colors, tiles, grid_size).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Number of colors.
    pub fn num_colors(&self) -> usize {
        self.num_colors
    }

    /// Number of tile types.
    pub fn num_tiles(&self) -> usize {
        self.tiles.len()
    }

    /// Grid dimension N (for N x N grid).
    pub fn grid_size(&self) -> usize {
        self.grid_size
    }

    /// The collection of tile types.
    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    /// Check whether a configuration represents a valid tiling.
    ///
    /// The configuration is a flat array of tile indices of length `grid_size^2`,
    /// laid out in row-major order: position `i * grid_size + j` corresponds
    /// to grid cell `(i, j)` (row i, column j).
    fn is_valid_tiling(&self, config: &[usize]) -> bool {
        let n = self.grid_size;
        if config.len() != n * n {
            return false;
        }

        // Check all tile indices are valid
        for &tile_idx in config {
            if tile_idx >= self.tiles.len() {
                return false;
            }
        }

        // Check horizontal adjacency: right of (i,j) must match left of (i,j+1)
        for i in 0..n {
            for j in 0..n - 1 {
                let left_tile = self.tiles[config[i * n + j]];
                let right_tile = self.tiles[config[i * n + j + 1]];
                if left_tile.1 != right_tile.3 {
                    return false;
                }
            }
        }

        // Check vertical adjacency: bottom of (i,j) must match top of (i+1,j)
        for i in 0..n - 1 {
            for j in 0..n {
                let upper_tile = self.tiles[config[i * n + j]];
                let lower_tile = self.tiles[config[(i + 1) * n + j]];
                if upper_tile.2 != lower_tile.0 {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Deserialize)]
struct SquareTilingData {
    num_colors: usize,
    tiles: Vec<Tile>,
    grid_size: usize,
}

impl<'de> Deserialize<'de> for SquareTiling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = SquareTilingData::deserialize(deserializer)?;
        Self::try_new(data.num_colors, data.tiles, data.grid_size).map_err(D::Error::custom)
    }
}

impl Problem for SquareTiling {
    const NAME: &'static str = "SquareTiling";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.tiles.len(); self.grid_size * self.grid_size]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(self.is_valid_tiling(config))
    }
}

crate::declare_variants! {
    default SquareTiling => "num_tiles^(grid_size^2)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "square_tiling",
        instance: Box::new(SquareTiling::new(
            3,
            vec![(0, 1, 2, 0), (0, 0, 2, 1), (2, 1, 0, 0), (2, 0, 0, 1)],
            2,
        )),
        optimal_config: vec![0, 1, 2, 3],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/square_tiling.rs"]
mod tests;
