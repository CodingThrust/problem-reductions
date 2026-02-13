//! Procedural macros for problemreductions.
//!
//! This crate provides the `#[reduction]` attribute macro that automatically
//! generates `ReductionEntry` registrations from `ReduceTo` impl blocks.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_macro_input, GenericArgument, ItemImpl, Path, PathArguments, Type};

/// Attribute macro for automatic reduction registration.
///
/// Parses a `ReduceTo` impl block and generates the corresponding `inventory::submit!`
/// call. Variant fields are derived from `Problem::variant()` when possible.
///
/// # Variant Derivation
///
/// - **Types without type generics** (e.g., `KColoring<K, SimpleGraph>`): calls
///   `Problem::variant()` at runtime. Const generics like `K` are substituted with
///   `usize::MAX` (maps to `"N"` via `const_usize_str`).
/// - **Types with type generics** (e.g., `MaxCut<SimpleGraph, W>`): falls back to
///   constructing `("graph", ...), ("weight", ...)` from type parameter analysis.
///
/// # Attributes
///
/// - `overhead = { expr }` — overhead specification (required for non-trivial reductions)
/// - `source_graph = "..."` — override source graph type (fallback path only)
/// - `target_graph = "..."` — override target graph type (fallback path only)
/// - `source_weighted = bool` — override source weight (fallback path only)
/// - `target_weighted = bool` — override target weight (fallback path only)
#[proc_macro_attribute]
pub fn reduction(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as ReductionAttrs);
    let impl_block = parse_macro_input!(item as ItemImpl);

    match generate_reduction_entry(&attrs, &impl_block) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Parsed attributes from #[reduction(...)]
struct ReductionAttrs {
    source_graph: Option<String>,
    target_graph: Option<String>,
    source_weighted: Option<bool>,
    target_weighted: Option<bool>,
    overhead: Option<TokenStream2>,
}

impl syn::parse::Parse for ReductionAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = ReductionAttrs {
            source_graph: None,
            target_graph: None,
            source_weighted: None,
            target_weighted: None,
            overhead: None,
        };

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "source_graph" => {
                    let lit: syn::LitStr = input.parse()?;
                    attrs.source_graph = Some(lit.value());
                }
                "target_graph" => {
                    let lit: syn::LitStr = input.parse()?;
                    attrs.target_graph = Some(lit.value());
                }
                "source_weighted" => {
                    let lit: syn::LitBool = input.parse()?;
                    attrs.source_weighted = Some(lit.value());
                }
                "target_weighted" => {
                    let lit: syn::LitBool = input.parse()?;
                    attrs.target_weighted = Some(lit.value());
                }
                "overhead" => {
                    let content;
                    syn::braced!(content in input);
                    attrs.overhead = Some(content.parse()?);
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown attribute: {}", ident),
                    ));
                }
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(attrs)
    }
}

/// Extract the base type name from a Type (e.g., "IndependentSet" from "IndependentSet<i32>")
fn extract_type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            Some(segment.ident.to_string())
        }
        _ => None,
    }
}

/// Collect const generic parameter names from impl generics.
/// e.g., `impl<const K: usize>` → {"K"}
fn collect_const_generic_names(generics: &syn::Generics) -> HashSet<String> {
    generics
        .params
        .iter()
        .filter_map(|p| {
            if let syn::GenericParam::Const(c) = p {
                Some(c.ident.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Collect type generic parameter names from impl generics.
/// e.g., `impl<G: Graph, W: NumericSize>` → {"G", "W"}
fn collect_type_generic_names(generics: &syn::Generics) -> HashSet<String> {
    generics
        .params
        .iter()
        .filter_map(|p| {
            if let syn::GenericParam::Type(t) = p {
                Some(t.ident.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Check if a type uses any of the given type generic parameters.
fn type_uses_type_generics(ty: &Type, type_generics: &HashSet<String>) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in args.args.iter() {
                        if let GenericArgument::Type(Type::Path(inner)) = arg {
                            if let Some(ident) = inner.path.get_ident() {
                                if type_generics.contains(&ident.to_string()) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        }
        _ => false,
    }
}

/// Rewrite a type by substituting const generic names with `{usize::MAX}`.
///
/// e.g., `KColoring<K, SimpleGraph>` with const_generics={"K"}
/// → `KColoring<{usize::MAX}, SimpleGraph>`
fn rewrite_const_generics(ty: &Type, const_generics: &HashSet<String>) -> Type {
    match ty {
        Type::Path(type_path) => {
            let mut new_path = type_path.clone();
            if let Some(segment) = new_path.path.segments.last_mut() {
                if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in args.args.iter_mut() {
                        if let GenericArgument::Type(Type::Path(inner)) = arg {
                            if let Some(ident) = inner.path.get_ident() {
                                if const_generics.contains(&ident.to_string()) {
                                    // Replace const generic with sentinel value
                                    *arg = syn::parse_quote!({ usize::MAX });
                                }
                            }
                        }
                    }
                }
            }
            Type::Path(new_path)
        }
        _ => ty.clone(),
    }
}

/// Generate the variant fn body for a type.
///
/// If the type has no type generics: calls `Problem::variant()` with const generic
/// sentinels. Otherwise falls back to manual `("graph", ...), ("weight", ...)` construction.
fn make_variant_fn_body(
    ty: &Type,
    const_generics: &HashSet<String>,
    type_generics: &HashSet<String>,
    graph_override: Option<&str>,
    weighted_override: Option<bool>,
) -> TokenStream2 {
    if type_uses_type_generics(ty, type_generics) {
        // Fallback: construct variant manually from type parameter analysis
        let graph = graph_override
            .map(|s| s.to_string())
            .or_else(|| extract_graph_type(ty))
            .unwrap_or_else(|| "SimpleGraph".to_string());
        let weight = weighted_override
            .map(|w| {
                if w {
                    "i32".to_string()
                } else {
                    "Unweighted".to_string()
                }
            })
            .unwrap_or_else(|| {
                extract_weight_type(ty)
                    .map(|t| get_weight_name(&t))
                    .unwrap_or_else(|| "Unweighted".to_string())
            });
        quote! { vec![("graph", #graph), ("weight", #weight)] }
    } else {
        // Call Problem::variant() with const generic sentinels
        let rewritten = rewrite_const_generics(ty, const_generics);
        quote! { <#rewritten as crate::traits::Problem>::variant() }
    }
}

/// Generate the reduction entry code
fn generate_reduction_entry(
    attrs: &ReductionAttrs,
    impl_block: &ItemImpl,
) -> syn::Result<TokenStream2> {
    // Extract the trait path (should be ReduceTo<Target>)
    let trait_path = impl_block
        .trait_
        .as_ref()
        .map(|(_, path, _)| path)
        .ok_or_else(|| syn::Error::new_spanned(impl_block, "Expected impl ReduceTo<T> for S"))?;

    // Extract target type from ReduceTo<Target>
    let target_type = extract_target_from_trait(trait_path)?;

    // Extract source type (Self type)
    let source_type = &impl_block.self_ty;

    // Get type names
    let source_name = extract_type_name(source_type)
        .ok_or_else(|| syn::Error::new_spanned(source_type, "Cannot extract source type name"))?;
    let target_name = extract_type_name(&target_type)
        .ok_or_else(|| syn::Error::new_spanned(&target_type, "Cannot extract target type name"))?;

    // Collect generic parameter info from the impl block
    let const_generics = collect_const_generic_names(&impl_block.generics);
    let type_generics = collect_type_generic_names(&impl_block.generics);

    // Generate variant fn bodies
    let source_variant_body = make_variant_fn_body(
        source_type,
        &const_generics,
        &type_generics,
        attrs.source_graph.as_deref(),
        attrs.source_weighted,
    );
    let target_variant_body = make_variant_fn_body(
        &target_type,
        &const_generics,
        &type_generics,
        attrs.target_graph.as_deref(),
        attrs.target_weighted,
    );

    // Generate overhead or use default
    let overhead = attrs.overhead.clone().unwrap_or_else(|| {
        quote! {
            crate::rules::registry::ReductionOverhead::default()
        }
    });

    // Generate the combined output
    let output = quote! {
        #impl_block

        inventory::submit! {
            crate::rules::registry::ReductionEntry {
                source_name: #source_name,
                target_name: #target_name,
                source_variant_fn: || { #source_variant_body },
                target_variant_fn: || { #target_variant_body },
                overhead_fn: || { #overhead },
                module_path: module_path!(),
            }
        }
    };

    Ok(output)
}

/// Extract the target type from ReduceTo<Target> trait path
fn extract_target_from_trait(path: &Path) -> syn::Result<Type> {
    let segment = path
        .segments
        .last()
        .ok_or_else(|| syn::Error::new_spanned(path, "Empty trait path"))?;

    if segment.ident != "ReduceTo" {
        return Err(syn::Error::new_spanned(segment, "Expected ReduceTo trait"));
    }

    if let PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(GenericArgument::Type(ty)) = args.args.first() {
            return Ok(ty.clone());
        }
    }

    Err(syn::Error::new_spanned(
        segment,
        "Expected ReduceTo<Target> with type parameter",
    ))
}

// --- Fallback helpers for types with type generics ---

/// Extract graph type from type parameters (first parameter in `Problem<G, W>` order)
fn extract_graph_type(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                for arg in args.args.iter() {
                    if let GenericArgument::Type(Type::Path(inner_path)) = arg {
                        let name = inner_path
                            .path
                            .segments
                            .last()
                            .map(|s| s.ident.to_string())?;
                        // Skip generic params (single uppercase letter)
                        if name.len() == 1
                            && name
                                .chars()
                                .next()
                                .map(|c| c.is_ascii_uppercase())
                                .unwrap_or(false)
                        {
                            return None;
                        }
                        // Skip known weight types
                        if is_weight_type(&name) {
                            return None;
                        }
                        return Some(name);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

/// Check if a type name is a known weight type
fn is_weight_type(name: &str) -> bool {
    ["i32", "i64", "f32", "f64", "Unweighted"].contains(&name)
}

/// Extract weight type from type parameters.
fn extract_weight_type(ty: &Type) -> Option<Type> {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                let type_args: Vec<_> = args
                    .args
                    .iter()
                    .filter_map(|arg| {
                        if let GenericArgument::Type(t) = arg {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect();

                match type_args.len() {
                    1 => {
                        let first = type_args[0];
                        if let Type::Path(inner_path) = first {
                            let name = inner_path.path.segments.last()?.ident.to_string();
                            if is_weight_type(&name) {
                                return Some(first.clone());
                            }
                        }
                        None
                    }
                    2 => {
                        let second = type_args[1];
                        if let Type::Path(inner_path) = second {
                            let name = inner_path.path.segments.last()?.ident.to_string();
                            if is_weight_type(&name) {
                                return Some(second.clone());
                            }
                        }
                        None
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Get weight type name as a string for the variant.
/// Single-letter uppercase names are treated as generic type parameters
/// and default to "Unweighted".
fn get_weight_name(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let name = type_path
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_else(|| "Unweighted".to_string());
            if name.len() == 1
                && name
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false)
            {
                "Unweighted".to_string()
            } else {
                name
            }
        }
        _ => "Unweighted".to_string(),
    }
}
