//! Variant attribute utilities.

use std::any::type_name;

/// Extract short type name from full path.
/// e.g., "problemreductions::graph_types::SimpleGraph" -> "SimpleGraph"
pub fn short_type_name<T: 'static>() -> &'static str {
    let full = type_name::<T>();
    full.rsplit("::").next().unwrap_or(full)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_type_name_primitive() {
        assert_eq!(short_type_name::<i32>(), "i32");
        assert_eq!(short_type_name::<f64>(), "f64");
    }

    #[test]
    fn test_short_type_name_struct() {
        struct MyStruct;
        assert_eq!(short_type_name::<MyStruct>(), "MyStruct");
    }
}
