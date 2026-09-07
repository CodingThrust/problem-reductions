//! Shared utilities for CLI and MCP: parsing helpers and random generation.

use anyhow::{bail, Result};
use num_bigint::BigUint;

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

/// Parse semicolon-separated x,y pairs from a string.
pub fn parse_positions<T: std::str::FromStr>(pos_str: &str, example: &str) -> Result<Vec<(T, T)>>
where
    T::Err: std::fmt::Display,
{
    pos_str
        .split(';')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split(',').collect();
            if parts.len() != 2 {
                bail!(
                    "Invalid position '{}': expected format x,y (e.g., {example})",
                    pair.trim()
                );
            }
            let x: T = parts[0]
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid x in '{}': {e}", pair.trim()))?;
            let y: T = parts[1]
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid y in '{}': {e}", pair.trim()))?;
            Ok((x, y))
        })
        .collect()
}

// ---------------------------------------------------------------------------
/// Parse a comma-separated list of values.
pub fn parse_comma_list<T: std::str::FromStr>(s: &str) -> Result<Vec<T>>
where
    T::Err: std::fmt::Display,
{
    s.split(',')
        .map(|v| {
            v.trim()
                .parse::<T>()
                .map_err(|e| anyhow::anyhow!("Invalid value '{}': {e}", v.trim()))
        })
        .collect()
}

pub fn parse_decimal_biguint(s: &str) -> Result<BigUint> {
    BigUint::parse_bytes(s.trim().as_bytes(), 10)
        .ok_or_else(|| anyhow::anyhow!("Invalid decimal integer '{}'", s.trim()))
}

pub fn parse_biguint_list(s: &str) -> Result<Vec<BigUint>> {
    s.split(',')
        .map(|value| parse_decimal_biguint(value.trim()))
        .collect()
}

/// Parse edge pairs like "0-1,1-2,2-3" into Vec<(usize, usize)>.
pub fn parse_edge_pairs(s: &str) -> Result<Vec<(usize, usize)>> {
    s.split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split('-').collect();
            if parts.len() != 2 {
                bail!("Invalid edge '{}': expected format u-v", pair.trim());
            }
            let u: usize = parts[0].trim().parse()?;
            let v: usize = parts[1].trim().parse()?;
            Ok((u, v))
        })
        .collect()
}
