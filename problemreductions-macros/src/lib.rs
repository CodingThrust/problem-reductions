//! Procedural macros for problemreductions.
//!
//! This crate provides the `#[reduction]` attribute macro that automatically
//! generates `ReductionEntry` registrations from `ReduceTo` impl blocks,
//! and the `declare_variants!` proc macro for compile-time validated variant
//! registration.

pub(crate) mod parser;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{parse_macro_input, GenericArgument, ItemImpl, Path, PathArguments, Type};

/// Attribute macro for automatic reduction registration.
///
/// Parses a `ReduceTo` impl block and generates the corresponding `inventory::submit!`
/// call. Variant fields are derived from `Problem::variant()`.
///
/// **Type generics are not supported** — all `ReduceTo` impls must use concrete types.
/// If you need a reduction for a generic problem, write separate impls for each concrete
/// type combination.
///
/// # Attributes
///
/// - `overhead = { expr }` — overhead specification
///
/// ## New syntax (preferred):
/// ```ignore
/// #[reduction(overhead = {
///     num_vars = "num_vertices^2",
///     num_constraints = "num_edges",
/// })]
/// ```
///
/// ## Legacy syntax (still supported):
/// ```ignore
/// #[reduction(overhead = { ReductionOverhead::new(vec![...]) })]
/// ```
#[proc_macro_attribute]
pub fn reduction(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as ReductionAttrs);
    let impl_block = parse_macro_input!(item as ItemImpl);

    match generate_reduction_entry(&attrs, &impl_block) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Overhead specification: either new parsed syntax or legacy raw tokens.
enum OverheadSpec {
    /// Legacy syntax: raw token stream (e.g., `ReductionOverhead::new(...)`)
    Legacy(TokenStream2),
    /// New syntax: list of (field_name, expression_string) pairs
    Parsed(Vec<(String, String)>),
}

/// Parsed attributes from #[reduction(...)]
struct ReductionAttrs {
    overhead: Option<OverheadSpec>,
}

impl syn::parse::Parse for ReductionAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = ReductionAttrs { overhead: None };

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "overhead" => {
                    let content;
                    syn::braced!(content in input);
                    attrs.overhead = Some(parse_overhead_content(&content)?);
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

/// Detect and parse the overhead content as either new or legacy syntax.
///
/// New syntax detection: the first tokens are `ident = "string_literal"`.
/// Legacy syntax: everything else (starts with a path like `ReductionOverhead::...`).
fn parse_overhead_content(content: syn::parse::ParseStream) -> syn::Result<OverheadSpec> {
    // Fork to peek ahead without consuming
    let fork = content.fork();

    // Try to detect new syntax: ident = "string"
    let is_new_syntax = fork.parse::<syn::Ident>().is_ok()
        && fork.parse::<syn::Token![=]>().is_ok()
        && fork.parse::<syn::LitStr>().is_ok();

    if is_new_syntax {
        // Parse new syntax: field_name = "expression", ...
        let mut fields = Vec::new();
        while !content.is_empty() {
            let field_name: syn::Ident = content.parse()?;
            content.parse::<syn::Token![=]>()?;
            let expr_str: syn::LitStr = content.parse()?;
            fields.push((field_name.to_string(), expr_str.value()));

            if content.peek(syn::Token![,]) {
                content.parse::<syn::Token![,]>()?;
            }
        }
        Ok(OverheadSpec::Parsed(fields))
    } else {
        // Legacy syntax: parse as raw token stream
        let tokens: TokenStream2 = content.parse()?;
        Ok(OverheadSpec::Legacy(tokens))
    }
}

/// Extract the base type name from a Type (e.g., "IndependentSet" from "IndependentSet<i32>").
/// Special-cases `Decision<T>` to produce `DecisionT`.
fn extract_type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            let ident = segment.ident.to_string();

            if ident == "Decision" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    let inner_ty = args.args.iter().find_map(|arg| match arg {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None,
                    })?;
                    let inner_name = extract_type_name(inner_ty)?;
                    return Some(format!("Decision{inner_name}"));
                }
            }

            Some(ident)
        }
        _ => None,
    }
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

/// Generate the variant fn body for a type.
///
/// Calls `Problem::variant()` on the concrete type.
/// Errors if the type uses any type generics — all `ReduceTo` impls must be concrete.
fn make_variant_fn_body(ty: &Type, type_generics: &HashSet<String>) -> syn::Result<TokenStream2> {
    if type_uses_type_generics(ty, type_generics) {
        let used: Vec<_> = type_generics.iter().cloned().collect();
        return Err(syn::Error::new_spanned(
            ty,
            format!(
                "#[reduction] does not support type generics (found: {}). \
                 Make the ReduceTo impl concrete by specifying explicit types.",
                used.join(", ")
            ),
        ));
    }
    Ok(quote! { <#ty as crate::traits::Problem>::variant() })
}

/// Generate overhead code from the new parsed syntax.
///
/// Produces a `ReductionOverhead` constructor that uses `Expr` AST values.
fn generate_parsed_overhead(fields: &[(String, String)]) -> syn::Result<TokenStream2> {
    let mut field_tokens = Vec::new();

    for (field_name, expr_str) in fields {
        let parsed = parser::parse_expr(expr_str).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("error parsing overhead expression \"{expr_str}\": {e}"),
            )
        })?;

        let expr_ast = parsed.to_expr_tokens();
        let name_lit = field_name.as_str();
        field_tokens.push(quote! { (#name_lit, #expr_ast) });
    }

    Ok(quote! {
        crate::rules::registry::ReductionOverhead::new(vec![#(#field_tokens),*])
    })
}

/// Generate a compiled overhead evaluation function from parsed overhead fields.
///
/// Produces a closure that downcasts `&dyn Any` to `&SourceType`, calls getter methods
/// for each variable in the expressions, and returns a `ProblemSize`.
fn generate_overhead_eval_fn(
    fields: &[(String, String)],
    source_type: &Type,
) -> syn::Result<TokenStream2> {
    let src_ident = syn::Ident::new("__src", proc_macro2::Span::call_site());

    let mut field_eval_tokens = Vec::new();
    for (field_name, expr_str) in fields {
        let parsed = parser::parse_expr(expr_str).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("error parsing overhead expression \"{expr_str}\": {e}"),
            )
        })?;

        let eval_tokens = parsed.to_eval_tokens(&src_ident);
        let name_lit = field_name.as_str();
        field_eval_tokens.push(quote! { (#name_lit, (#eval_tokens).round() as usize) });
    }

    Ok(quote! {
        |__any_src: &dyn std::any::Any| -> crate::types::ProblemSize {
            let #src_ident = __any_src.downcast_ref::<#source_type>().unwrap();
            crate::types::ProblemSize::new(vec![#(#field_eval_tokens),*])
        }
    })
}

/// Generate a function that extracts the source problem's size fields from `&dyn Any`.
///
/// Collects all variable names referenced in the overhead expressions, generates
/// getter calls for each, and returns a `ProblemSize`.
fn generate_source_size_fn(
    fields: &[(String, String)],
    source_type: &Type,
) -> syn::Result<TokenStream2> {
    let src_ident = syn::Ident::new("__src", proc_macro2::Span::call_site());

    // Collect all unique variable names from overhead expressions
    let mut var_names = std::collections::BTreeSet::new();
    for (_, expr_str) in fields {
        let parsed = parser::parse_expr(expr_str).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("error parsing overhead expression \"{expr_str}\": {e}"),
            )
        })?;
        for v in parsed.variables() {
            var_names.insert(v.to_string());
        }
    }

    let getter_tokens: Vec<_> = var_names
        .iter()
        .map(|var| {
            let getter = syn::Ident::new(var, proc_macro2::Span::call_site());
            let name_lit = var.as_str();
            quote! { (#name_lit, #src_ident.#getter() as usize) }
        })
        .collect();

    Ok(quote! {
        |__any_src: &dyn std::any::Any| -> crate::types::ProblemSize {
            let #src_ident = __any_src.downcast_ref::<#source_type>().unwrap();
            crate::types::ProblemSize::new(vec![#(#getter_tokens),*])
        }
    })
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
    let capabilities = if source_name == target_name {
        quote! { crate::rules::EdgeCapabilities::both() }
    } else {
        quote! { crate::rules::EdgeCapabilities::witness_only() }
    };

    // Collect generic parameter info from the impl block
    let type_generics = collect_type_generic_names(&impl_block.generics);

    // Generate variant fn bodies
    let source_variant_body = make_variant_fn_body(source_type, &type_generics)?;
    let target_variant_body = make_variant_fn_body(&target_type, &type_generics)?;

    // Generate overhead, eval fn, and source size fn
    let (overhead, overhead_eval_fn, source_size_fn) = match &attrs.overhead {
        Some(OverheadSpec::Legacy(tokens)) => {
            let eval_fn = quote! {
                |_: &dyn std::any::Any| -> crate::types::ProblemSize {
                    panic!("overhead_eval_fn not available for legacy overhead syntax; \
                            migrate to parsed syntax: field = \"expression\"")
                }
            };
            let size_fn = quote! {
                |_: &dyn std::any::Any| -> crate::types::ProblemSize {
                    crate::types::ProblemSize::new(vec![])
                }
            };
            (tokens.clone(), eval_fn, size_fn)
        }
        Some(OverheadSpec::Parsed(fields)) => {
            let overhead_tokens = generate_parsed_overhead(fields)?;
            let eval_fn = generate_overhead_eval_fn(fields, source_type)?;
            let size_fn = generate_source_size_fn(fields, source_type)?;
            (overhead_tokens, eval_fn, size_fn)
        }
        None => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Missing overhead specification. Use #[reduction(overhead = { ... })] and specify overhead expressions for all target problem size fields.",
            ));
        }
    };

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
                reduce_fn: Some(|src: &dyn std::any::Any| -> Box<dyn crate::rules::traits::DynReductionResult> {
                    let src = src.downcast_ref::<#source_type>().unwrap_or_else(|| {
                        panic!(
                            "DynReductionResult: source type mismatch: expected `{}`, got `{}`",
                            std::any::type_name::<#source_type>(),
                            std::any::type_name_of_val(src),
                        )
                    });
                    Box::new(<#source_type as crate::rules::ReduceTo<#target_type>>::reduce_to(src))
                }),
                reduce_aggregate_fn: None,
                capabilities: #capabilities,
                overhead_eval_fn: #overhead_eval_fn,
                source_size_fn: #source_size_fn,
            }
        }

        const _: () = {
            fn _assert_declared_variant<T: crate::traits::DeclaredVariant>() {}
            fn _check() {
                _assert_declared_variant::<#source_type>();
                _assert_declared_variant::<#target_type>();
            }
        };
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

// --- declare_variants! proc macro ---

/// Input for the `declare_variants!` proc macro.
struct DeclareVariantsInput {
    entries: Vec<DeclareVariantEntry>,
}

/// A single entry: `[default] Type => "complexity_string" [aliases ["X", ...]]`.
struct DeclareVariantEntry {
    is_default: bool,
    ty: Type,
    complexity: syn::LitStr,
    aliases: Vec<syn::LitStr>,
}

impl syn::parse::Parse for DeclareVariantsInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();
        while !input.is_empty() {
            // Optionally accept a `default` keyword before the type
            let is_default = input.peek(syn::Token![default]);
            if is_default {
                input.parse::<syn::Token![default]>()?;
            }

            let ty: Type = input.parse()?;
            input.parse::<syn::Token![=>]>()?;
            let complexity: syn::LitStr = input.parse()?;

            // Optional: `aliases ["X", "Y", ...]`
            let aliases = if input.peek(syn::Ident) {
                let fork = input.fork();
                let ident: syn::Ident = fork.parse()?;
                if ident == "aliases" {
                    input.parse::<syn::Ident>()?;
                    let content;
                    syn::bracketed!(content in input);
                    let mut out = Vec::new();
                    while !content.is_empty() {
                        let lit: syn::LitStr = content.parse()?;
                        if lit.value().trim().is_empty() {
                            return Err(syn::Error::new(
                                lit.span(),
                                "variant alias must not be empty or whitespace-only",
                            ));
                        }
                        out.push(lit);
                        if content.peek(syn::Token![,]) {
                            content.parse::<syn::Token![,]>()?;
                        }
                    }
                    out
                } else if fork.peek(syn::token::Bracket) {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("expected 'aliases', found '{ident}'"),
                    ));
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            entries.push(DeclareVariantEntry {
                is_default,
                ty,
                complexity,
                aliases,
            });

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(DeclareVariantsInput { entries })
    }
}

/// Declare explicit problem variants with per-variant complexity metadata.
///
/// Each entry generates:
/// 1. A `DeclaredVariant` trait impl for compile-time checking
/// 2. A `VariantEntry` inventory submission for runtime graph building
/// 3. A compiled `complexity_eval_fn` that calls getter methods
/// 4. A const validation block verifying all variable names are valid getters
///
/// Complexity strings must use only numeric literals and getter method names.
/// Mathematical constants (epsilon, omega, etc.) should be inlined as numbers
/// and documented in comments or docstrings.
///
/// # Example
///
/// ```text
/// declare_variants! {
///     MaximumIndependentSet<SimpleGraph, i32>   => "1.1996^num_vertices",
///     MaximumIndependentSet<KingsSubgraph, i32> => "2^sqrt(num_vertices)",
/// }
/// ```
#[proc_macro]
pub fn declare_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeclareVariantsInput);
    match generate_declare_variants(&input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Generate code for all `declare_variants!` entries.
fn generate_declare_variants(input: &DeclareVariantsInput) -> syn::Result<TokenStream2> {
    // Validate default markers per problem name.
    // Group entries by their base type name (e.g., "MaximumIndependentSet").
    let mut defaults_per_problem: HashMap<String, Vec<usize>> = HashMap::new();
    let mut problem_names = HashSet::new();
    for (i, entry) in input.entries.iter().enumerate() {
        let base_name = extract_type_name(&entry.ty).unwrap_or_default();
        problem_names.insert(base_name.clone());
        if entry.is_default {
            defaults_per_problem.entry(base_name).or_default().push(i);
        }
    }

    // Check for multiple defaults for the same problem
    for (name, indices) in &defaults_per_problem {
        if indices.len() > 1 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "`{name}` has more than one default variant; \
                     only one entry per problem may be marked `default`"
                ),
            ));
        }
    }

    for name in problem_names {
        if !defaults_per_problem.contains_key(&name) {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "`{name}` must declare exactly one default variant; \
                     mark one entry with `default`"
                ),
            ));
        }
    }

    let mut output = TokenStream2::new();

    for entry in &input.entries {
        let ty = &entry.ty;
        let complexity_str = entry.complexity.value();
        let is_default = entry.is_default;
        let alias_lits: Vec<_> = entry.aliases.iter().map(|s| s.value()).collect();

        // Parse the complexity expression to validate syntax
        let parsed = parser::parse_expr(&complexity_str).map_err(|e| {
            syn::Error::new(
                entry.complexity.span(),
                format!("invalid complexity expression \"{complexity_str}\": {e}"),
            )
        })?;

        // Generate getter validation for all variables
        let vars = parsed.variables();
        let validation = if vars.is_empty() {
            quote! {}
        } else {
            let src_ident = syn::Ident::new("__src", proc_macro2::Span::call_site());
            let getter_checks: Vec<_> = vars
                .iter()
                .map(|var| {
                    let getter = syn::Ident::new(var, proc_macro2::Span::call_site());
                    quote! { let _ = #src_ident.#getter(); }
                })
                .collect();

            quote! {
                const _: () = {
                    #[allow(unused)]
                    fn _validate_complexity(#src_ident: &#ty) {
                        #(#getter_checks)*
                    }
                };
            }
        };

        // Generate compiled complexity eval fn
        let complexity_eval_fn = generate_complexity_eval_fn(&parsed, ty)?;

        // Generate dispatch fields based on aggregate value solving plus optional witnesses.
        let solve_value_body = quote! {
            let total = <crate::solvers::BruteForce as crate::solvers::Solver>::solve(&solver, p);
            crate::registry::format_metric(&total)
        };

        let solve_witness_body = quote! {
            let config = crate::solvers::BruteForce::find_witness(&solver, p)?;
        };

        let dispatch_fields = quote! {
            factory: |data: serde_json::Value| -> Result<Box<dyn crate::registry::DynProblem>, serde_json::Error> {
                let p: #ty = serde_json::from_value(data)?;
                Ok(Box::new(p))
            },
            serialize_fn: |any: &dyn std::any::Any| -> Option<serde_json::Value> {
                let p = any.downcast_ref::<#ty>()?;
                Some(serde_json::to_value(p).expect("serialize failed"))
            },
            solve_value_fn: |any: &dyn std::any::Any| -> String {
                let p = any
                    .downcast_ref::<#ty>()
                    .expect("type-erased solve_value downcast failed");
                let solver = crate::solvers::BruteForce::new();
                #solve_value_body
            },
            solve_witness_fn: |any: &dyn std::any::Any| -> Option<(Vec<usize>, String)> {
                let p = any.downcast_ref::<#ty>()?;
                let solver = crate::solvers::BruteForce::new();
                #solve_witness_body
                let evaluation = crate::registry::format_metric(&crate::traits::Problem::evaluate(p, &config));
                Some((config, evaluation))
            },
        };

        output.extend(quote! {
            impl crate::traits::DeclaredVariant for #ty {}

            crate::inventory::submit! {
                crate::registry::VariantEntry {
                    name: <#ty as crate::traits::Problem>::NAME,
                    variant_fn: || <#ty as crate::traits::Problem>::variant(),
                    complexity: #complexity_str,
                    complexity_eval_fn: #complexity_eval_fn,
                    is_default: #is_default,
                    aliases: &[#(#alias_lits),*],
                    #dispatch_fields
                }
            }

            #validation
        });
    }

    Ok(output)
}

/// Generate a compiled complexity evaluation function.
///
/// Produces a closure that downcasts `&dyn Any` to the problem type, calls getter
/// methods for all variables, and returns the worst-case time complexity as f64.
fn generate_complexity_eval_fn(
    parsed: &parser::ParsedExpr,
    ty: &Type,
) -> syn::Result<TokenStream2> {
    let src_ident = syn::Ident::new("__src", proc_macro2::Span::call_site());
    let eval_tokens = parsed.to_eval_tokens(&src_ident);

    Ok(quote! {
        |__any_src: &dyn std::any::Any| -> f64 {
            let #src_ident = __any_src.downcast_ref::<#ty>().unwrap();
            #eval_tokens
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_str, Type};

    #[test]
    fn extract_type_name_strips_non_decision_generics() {
        let ty: Type = parse_str("MinimumVertexCover<SimpleGraph, i32>").unwrap();
        assert_eq!(
            extract_type_name(&ty).as_deref(),
            Some("MinimumVertexCover")
        );
    }

    #[test]
    fn extract_type_name_unwraps_decision_inner_type() {
        let ty: Type = parse_str("Decision<MinimumVertexCover<SimpleGraph, i32>>").unwrap();
        assert_eq!(
            extract_type_name(&ty).as_deref(),
            Some("DecisionMinimumVertexCover")
        );
    }

    #[test]
    fn declare_variants_accepts_single_default() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            default Foo => "1",
        };
        assert!(generate_declare_variants(&input).is_ok());
    }

    #[test]
    fn declare_variants_requires_one_default_per_problem() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            Foo => "1",
        };
        let err = generate_declare_variants(&input).unwrap_err();
        assert!(
            err.to_string().contains("exactly one default"),
            "expected 'exactly one default' in error, got: {}",
            err
        );
    }

    #[test]
    fn declare_variants_rejects_multiple_defaults_for_one_problem() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            default Foo => "1",
            default Foo => "2",
        };
        let err = generate_declare_variants(&input).unwrap_err();
        assert!(
            err.to_string().contains("more than one default"),
            "expected 'more than one default' in error, got: {}",
            err
        );
    }

    #[test]
    fn declare_variants_rejects_missing_default_marker() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            Foo => "1",
        };
        let err = generate_declare_variants(&input).unwrap_err();
        assert!(
            err.to_string().contains("exactly one default"),
            "expected 'exactly one default' in error, got: {}",
            err
        );
    }

    #[test]
    fn declare_variants_marks_only_explicit_default() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            Foo => "1",
            default Foo => "2",
        };
        let result = generate_declare_variants(&input);
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        let true_count = tokens.matches("is_default : true").count();
        let false_count = tokens.matches("is_default : false").count();
        assert_eq!(true_count, 1, "should have exactly one default");
        assert_eq!(false_count, 1, "should have exactly one non-default");
    }

    #[test]
    fn declare_variants_accepts_entries_without_solver_kind_markers() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            default Foo => "1",
            default Bar => "2",
        };
        assert!(generate_declare_variants(&input).is_ok());
    }

    #[test]
    fn declare_variants_rejects_legacy_solver_kind_markers() {
        let result = syn::parse_str::<DeclareVariantsInput>("default opt Foo => \"1\"");
        assert!(
            result.is_err(),
            "expected parse error for legacy solver kind marker"
        );
    }

    #[test]
    fn declare_variants_rejects_empty_alias_literal() {
        let err =
            match syn::parse_str::<DeclareVariantsInput>("default Foo => \"1\" aliases [\"\"]") {
                Ok(_) => panic!("empty alias literal should be rejected"),
                Err(err) => err,
            };
        assert!(
            err.to_string().contains("empty or whitespace-only"),
            "expected empty-alias error, got: {err}"
        );
    }

    #[test]
    fn declare_variants_rejects_whitespace_only_alias_literal() {
        let err = match syn::parse_str::<DeclareVariantsInput>(
            "default Foo => \"1\" aliases [\"  \\t\"]",
        ) {
            Ok(_) => panic!("whitespace-only alias literal should be rejected"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains("empty or whitespace-only"),
            "expected whitespace-only alias error, got: {err}"
        );
    }

    #[test]
    fn declare_variants_rejects_unknown_alias_keyword_before_bracket() {
        let err = match syn::parse_str::<DeclareVariantsInput>(
            "default Foo => \"1\" nicknames [\"Foo\"]",
        ) {
            Ok(_) => panic!("unknown aliases keyword should be rejected"),
            Err(err) => err,
        };
        assert_eq!(err.to_string(), "expected 'aliases', found 'nicknames'");
    }

    #[test]
    fn declare_variants_generates_aggregate_value_and_witness_dispatch() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            default Foo => "1",
        };
        let tokens = generate_declare_variants(&input).unwrap().to_string();
        assert!(tokens.contains("factory :"), "expected factory field");
        assert!(
            tokens.contains("serialize_fn :"),
            "expected serialize_fn field"
        );
        assert!(
            tokens.contains("solve_value_fn :"),
            "expected solve_value_fn field"
        );
        assert!(
            tokens.contains("solve_witness_fn :"),
            "expected solve_witness_fn field"
        );
        assert!(
            !tokens.contains("factory : None"),
            "factory should not be None"
        );
        assert!(
            !tokens.contains("serialize_fn : None"),
            "serialize_fn should not be None"
        );
        assert!(
            !tokens.contains("solve_value_fn : None"),
            "solve_value_fn should not be None"
        );
        assert!(
            !tokens.contains("solve_witness_fn : None"),
            "solve_witness_fn should not be None"
        );
        assert!(
            tokens.contains("let total ="),
            "expected aggregate value solve"
        );
        assert!(
            tokens.contains("find_witness"),
            "expected find_witness in tokens"
        );
        assert!(
            !tokens.contains("find_best"),
            "did not expect legacy find_best in tokens"
        );
        assert!(
            !tokens.contains("SolutionSize :: Invalid"),
            "did not expect legacy invalid fallback in tokens"
        );
    }

    #[test]
    fn reduction_rejects_unexpected_attribute() {
        let extra_attr = syn::Ident::new("extra", proc_macro2::Span::call_site());
        let parse_result = syn::parse2::<ReductionAttrs>(quote! {
            #extra_attr = "unexpected", overhead = { num_vertices = "num_vertices" }
        });
        let err = match parse_result {
            Ok(_) => panic!("unexpected reduction attribute should be rejected"),
            Err(err) => err,
        };
        assert!(err.to_string().contains("unknown attribute: extra"));
    }

    #[test]
    fn reduction_accepts_overhead_attribute() {
        let attrs: ReductionAttrs = syn::parse_quote! {
            overhead = { n = "n" }
        };
        assert!(attrs.overhead.is_some());
    }

    #[test]
    fn declare_variants_codegen_uses_required_dispatch_fields() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            default Foo => "1",
        };
        let tokens = generate_declare_variants(&input).unwrap().to_string();
        assert!(tokens.contains("factory :"));
        assert!(tokens.contains("serialize_fn :"));
        assert!(tokens.contains("solve_value_fn :"));
        assert!(tokens.contains("solve_witness_fn :"));
        assert!(!tokens.contains("factory : None"));
        assert!(!tokens.contains("serialize_fn : None"));
        assert!(!tokens.contains("solve_value_fn : None"));
        assert!(!tokens.contains("solve_witness_fn : None"));
    }
}
