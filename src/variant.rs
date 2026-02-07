//! Variant attribute utilities.

use std::any::type_name;

/// Convert const generic usize to static str (for common values).
///
/// This is useful for including const generic parameters in problem variant IDs.
/// For values 1-10, returns the string representation. For other values, returns "N".
///
/// # Example
///
/// ```
/// use problemreductions::variant::const_usize_str;
///
/// assert_eq!(const_usize_str::<3>(), "3");
/// assert_eq!(const_usize_str::<10>(), "10");
/// assert_eq!(const_usize_str::<100>(), "N");
/// ```
pub const fn const_usize_str<const N: usize>() -> &'static str {
    match N {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        _ => "N",
    }
}

/// Extract short type name from full path.
/// e.g., "problemreductions::graph_types::SimpleGraph" -> "SimpleGraph"
pub fn short_type_name<T: 'static>() -> &'static str {
    let full = type_name::<T>();
    full.rsplit("::").next().unwrap_or(full)
}

#[cfg(test)]
#[path = "tests_unit/variant.rs"]
mod tests;
