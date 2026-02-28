//! Explicit variant registration via inventory.

/// A registered problem variant entry.
///
/// Submitted by [`declare_variants!`] for each concrete problem type.
/// The reduction graph uses these entries to build nodes with complexity metadata.
pub struct VariantEntry {
    /// Problem name (from `Problem::NAME`).
    pub name: &'static str,
    /// Function returning variant key-value pairs (from `Problem::variant()`).
    pub variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Worst-case time complexity expression (e.g., `"2^num_vertices"`).
    pub complexity: &'static str,
}

impl VariantEntry {
    /// Get the variant by calling the function.
    pub fn variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.variant_fn)()
    }
}

impl std::fmt::Debug for VariantEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantEntry")
            .field("name", &self.name)
            .field("variant", &self.variant())
            .field("complexity", &self.complexity)
            .finish()
    }
}

inventory::collect!(VariantEntry);
