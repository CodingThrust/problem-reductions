//! Procedural macros for problemreductions.
//!
//! This crate provides the `#[reduction]` attribute macro that automatically
//! generates `ReductionEntry` registrations from `ReduceTo` impl blocks.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, GenericArgument, ItemImpl, Path, PathArguments, Type};

/// Attribute macro for automatic reduction registration.
///
/// This macro parses a `ReduceTo` impl block and automatically generates
/// the corresponding `inventory::submit!` call with the correct metadata.
///
/// # Type Parameter Convention
///
/// The macro extracts graph and weight type information from type parameters:
/// - `Problem<G>` where `G` is a graph type - extracts graph type name
/// - `Problem<G, W>` where `W` is a weight type - weighted if W != Unweighted
///
/// # Example
///
/// ```ignore
/// #[reduction(
///     source_graph = "SimpleGraph",
///     target_graph = "GridGraph",
///     source_weighted = false,
///     target_weighted = true,
/// )]
/// impl ReduceTo<IndependentSet<i32, GridGraph>> for IndependentSet<Unweighted, SimpleGraph> {
///     type Result = ReductionISToGridIS;
///     fn reduce_to(&self) -> Self::Result { ... }
/// }
/// ```
///
/// The macro also supports inferring from type names when explicit attributes aren't provided.
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

/// Extract graph type from type parameters (first parameter in `Problem<G, W>` order)
fn extract_graph_type(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                // Get the first type argument which is the graph type
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
                            return None; // Generic param, let it default
                        }
                        // Skip known weight types - for single-param problems like QUBO<W>
                        if is_weight_type(&name) {
                            return None; // Weight type in first position, not a graph type
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
/// For `Problem<G, W>` (two params): returns W (second param).
/// For `Problem<W>` (single weight param): returns W (first param).
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
                        // Single param - check if it's a weight type
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
                        // Two params: Problem<G, W> - return second
                        Some(type_args[1].clone())
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
/// and default to "Unweighted" since they're not concrete types.
fn get_weight_name(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let name = type_path
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_else(|| "Unweighted".to_string());
            // Treat single uppercase letters as generic params, default to Unweighted
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

    // Determine weight type names
    let source_weight_name = attrs
        .source_weighted
        .map(|w| {
            if w {
                "i32".to_string()
            } else {
                "Unweighted".to_string()
            }
        })
        .unwrap_or_else(|| {
            extract_weight_type(source_type)
                .map(|t| get_weight_name(&t))
                .unwrap_or_else(|| "Unweighted".to_string())
        });
    let target_weight_name = attrs
        .target_weighted
        .map(|w| {
            if w {
                "i32".to_string()
            } else {
                "Unweighted".to_string()
            }
        })
        .unwrap_or_else(|| {
            extract_weight_type(&target_type)
                .map(|t| get_weight_name(&t))
                .unwrap_or_else(|| "Unweighted".to_string())
        });

    // Determine graph types
    let source_graph = attrs
        .source_graph
        .clone()
        .or_else(|| extract_graph_type(source_type))
        .unwrap_or_else(|| "SimpleGraph".to_string());
    let target_graph = attrs
        .target_graph
        .clone()
        .or_else(|| extract_graph_type(&target_type))
        .unwrap_or_else(|| "SimpleGraph".to_string());

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
                source_variant: &[("graph", #source_graph), ("weight", #source_weight_name)],
                target_variant: &[("graph", #target_graph), ("weight", #target_weight_name)],
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
