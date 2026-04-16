use crate::registry::variant::validate_variant_aliases;

#[test]
fn variant_alias_inventory_is_valid() {
    if let Err(conflicts) = validate_variant_aliases() {
        panic!("variant alias validation failed:\n{}", conflicts.join("\n"));
    }
}
