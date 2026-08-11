//! Procedural macros for problemreductions.
//!
//! This crate provides the `#[reduction]` attribute macro that automatically
//! generates `ReductionEntry` registrations from `ReduceTo` impl blocks,
//! and the `declare_variants!` proc macro for compile-time validated variant
//! registration.

mod expr_codegen;

use expr_codegen::{complexity_estimate_tokens, expr_tokens};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{parse_macro_input, DeriveInput, GenericArgument, ItemImpl, Path, PathArguments, Type};

/// Generate static construction-input metadata from a typed create spec.
#[proc_macro_derive(CreateSpec, attributes(create))]
pub fn derive_create_spec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match generate_create_spec(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn generate_create_spec(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "CreateSpec can only be derived for structs",
        ));
    };
    let syn::Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &data.fields,
            "CreateSpec requires named fields",
        ));
    };

    let mut field_entries = Vec::new();
    let mut input_entries = Vec::new();
    let mut input_renames = Vec::new();
    for field in &fields.named {
        let ident = field.ident.as_ref().expect("named field");
        let rust_name = ident.to_string();
        let mut input_name = rust_name.clone();
        let mut codec = quote!(crate::registry::CreateInputCodec::Auto);
        for attribute in &field.attrs {
            if attribute.path().is_ident("create") {
                attribute.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        input_name = meta.value()?.parse::<syn::LitStr>()?.value();
                        return Ok(());
                    }
                    if meta.path.is_ident("codec") {
                        let value = meta.value()?.parse::<syn::LitStr>()?;
                        codec = create_codec_tokens(&value)?;
                        return Ok(());
                    }
                    Err(meta.error("expected `name` or `codec`"))
                })?;
            }
        }
        if input_name.is_empty()
            || !input_name
                .bytes()
                .all(|byte| byte == b'_' || byte.is_ascii_lowercase() || byte.is_ascii_digit())
        {
            return Err(syn::Error::new(
                ident.span(),
                "construction input names must use non-empty snake_case",
            ));
        }

        let (value_type, required) = option_inner_type(&field.ty)
            .map(|inner| (inner, false))
            .unwrap_or((&field.ty, true));
        let type_name = quote!(#value_type).to_string().replace(' ', "");
        let description = field
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident("doc"))
            .filter_map(|attribute| match &attribute.meta {
                syn::Meta::NameValue(value) => match &value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(text),
                        ..
                    }) => Some(text.value().trim().to_string()),
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        if input_name != rust_name {
            let external_name = syn::LitStr::new(&input_name, ident.span());
            let rust_name = syn::LitStr::new(&rust_name, ident.span());
            input_renames.push(quote! {
                if let Some(value) = object.remove(#external_name) {
                    object.insert(#rust_name.to_string(), value);
                }
            });
        }
        let input_name = syn::LitStr::new(&input_name, ident.span());
        let type_name = syn::LitStr::new(&type_name, ident.span());
        let description = syn::LitStr::new(&description, ident.span());
        field_entries.push(quote! {
            crate::registry::FieldInfo {
                name: #input_name,
                type_name: #type_name,
                description: #description,
            }
        });
        input_entries.push(quote! {
            crate::registry::CreateInputInfo {
                name: #input_name,
                type_name: #type_name,
                description: #description,
                required: #required,
                codec: #codec,
            }
        });
    }

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics crate::registry::CreateSpec for #name #type_generics #where_clause {
            const FIELDS: &'static [crate::registry::FieldInfo] = &[
                #(#field_entries),*
            ];
            const INPUTS: &'static [crate::registry::CreateInputInfo] = &[
                #(#input_entries),*
            ];

            fn deserialize_inputs(
                mut data: serde_json::Value,
            ) -> Result<Self, serde_json::Error>
            where
                Self: serde::de::DeserializeOwned,
            {
                let object = data
                    .as_object_mut()
                    .expect("construction inputs were validated as an object");
                #(#input_renames)*
                serde_json::from_value(data)
            }
        }
    })
}

fn create_codec_tokens(value: &syn::LitStr) -> syn::Result<TokenStream2> {
    let variant = match value.value().as_str() {
        "auto" => quote!(Auto),
        "scalar" => quote!(Scalar),
        "json" => quote!(Json),
        "comma-separated" => quote!(CommaSeparated),
        "semicolon-separated" => quote!(SemicolonSeparated),
        "edge-list" => quote!(EdgeList),
        "arc-list" => quote!(ArcList),
        "bipartite-edge-list" => quote!(BipartiteEdgeList),
        "equality-pair-list" => quote!(EqualityPairList),
        "functional-dependency-list" => quote!(FunctionalDependencyList),
        "character-rows" => quote!(CharacterRows),
        _ => {
            return Err(syn::Error::new(
                value.span(),
                "unknown construction codec; expected one of: auto, scalar, json, comma-separated, semicolon-separated, edge-list, arc-list, bipartite-edge-list, equality-pair-list, functional-dependency-list, character-rows",
            ))
        }
    };
    Ok(quote!(crate::registry::CreateInputCodec::#variant))
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    let Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };
    arguments.args.iter().find_map(|argument| match argument {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    })
}

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
/// - `exact = { field = expression, ... }` — exact target-size equalities
/// - `bound = { field = expression, ... }` — certified monotone upper bounds
/// - `unavailable = { field = "reason", ... }` — fields that cannot be propagated
/// - `aggregate = identity` — explicitly register an aggregate executor; compilation
///   requires the reduction result to prove source/target value-type equality
///
/// ## Syntax
/// ```ignore
/// #[reduction(exact = {
///     num_vars = "num_vertices^2",
///     num_constraints = num_edges,
/// })]
/// ```
///
#[proc_macro_attribute]
pub fn reduction(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as ReductionAttrs);
    let impl_block = parse_macro_input!(item as ItemImpl);

    match generate_reduction_entry(&attrs, &impl_block) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[derive(Clone)]
struct ParsedExpressionField {
    name: String,
    expression: problemreductions_expr::Expr,
}

/// Parsed attributes from #[reduction(...)]
struct ReductionAttrs {
    exact: Option<Vec<(String, String)>>,
    bound: Option<Vec<(String, String)>>,
    unavailable: Option<Vec<(String, String)>>,
    identity_aggregate: bool,
}

impl syn::parse::Parse for ReductionAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = ReductionAttrs {
            exact: None,
            bound: None,
            unavailable: None,
            identity_aggregate: false,
        };

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "exact" => {
                    let content;
                    syn::braced!(content in input);
                    attrs.exact = Some(parse_expression_fields(&content)?);
                }
                "bound" => {
                    let content;
                    syn::braced!(content in input);
                    attrs.bound = Some(parse_expression_fields(&content)?);
                }
                "unavailable" => {
                    let content;
                    syn::braced!(content in input);
                    attrs.unavailable = Some(parse_unavailable_fields(&content)?);
                }
                "aggregate" => {
                    let value: syn::Ident = input.parse()?;
                    if value != "identity" {
                        return Err(syn::Error::new(value.span(), "expected `identity`"));
                    }
                    attrs.identity_aggregate = true;
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

fn parse_expression_fields(content: syn::parse::ParseStream) -> syn::Result<Vec<(String, String)>> {
    let mut fields = Vec::new();
    while !content.is_empty() {
        let field_name: syn::Ident = content.parse()?;
        content.parse::<syn::Token![=]>()?;
        let expression = if content.peek(syn::LitStr) {
            content.parse::<syn::LitStr>()?.value()
        } else {
            content.parse::<syn::Ident>()?.to_string()
        };
        fields.push((field_name.to_string(), expression));

        if content.peek(syn::Token![,]) {
            content.parse::<syn::Token![,]>()?;
        }
    }
    Ok(fields)
}

fn parse_unavailable_fields(
    content: syn::parse::ParseStream,
) -> syn::Result<Vec<(String, String)>> {
    let mut fields = Vec::new();
    while !content.is_empty() {
        let field_name: syn::Ident = content.parse()?;
        content.parse::<syn::Token![=]>()?;
        let reason = content.parse::<syn::LitStr>()?.value();
        if reason.trim().is_empty() {
            return Err(syn::Error::new(
                field_name.span(),
                "unavailable size field requires a non-empty reason",
            ));
        }
        fields.push((field_name.to_string(), reason));
        if content.peek(syn::Token![,]) {
            content.parse::<syn::Token![,]>()?;
        }
    }
    Ok(fields)
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

/// Parse one explicit exact or bound field declaration into the canonical expression DAG.
fn parse_expression_fields_to_expr(
    fields: &[(String, String)],
) -> syn::Result<Vec<ParsedExpressionField>> {
    fields
        .iter()
        .map(|(name, source)| {
            let expression = problemreductions_expr::Expr::try_parse(source).map_err(|error| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("error parsing size expression \"{source}\": {error}"),
                )
            })?;
            Ok(ParsedExpressionField {
                name: name.clone(),
                expression,
            })
        })
        .collect()
}

fn generate_expression_fields(fields: &[ParsedExpressionField]) -> TokenStream2 {
    let field_tokens = fields.iter().map(|field| {
        let expression = expr_tokens(&field.expression);
        let name = field.name.as_str();
        quote! { (#name, #expression) }
    });

    quote! { vec![#(#field_tokens),*] }
}

/// Generate a function that extracts the source problem's size fields from `&dyn Any`.
///
/// Collects all variable names referenced in the size expressions, generates
/// getter calls for each, and returns a `ProblemSize`.
fn generate_source_size_fn(fields: &[ParsedExpressionField], source_type: &Type) -> TokenStream2 {
    let src_ident = syn::Ident::new("__src", proc_macro2::Span::call_site());
    let var_names: std::collections::BTreeSet<_> = fields
        .iter()
        .flat_map(|field| field.expression.variables())
        .collect();
    let getter_tokens = var_names
        .into_iter()
        .map(|name| {
            let getter = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { (#name, #src_ident.#getter() as usize) }
        })
        .collect::<Vec<_>>();

    quote! {
        |__any_src: &dyn std::any::Any| -> crate::types::ProblemSize {
            let #src_ident = __any_src.downcast_ref::<#source_type>().unwrap();
            crate::types::ProblemSize::new(vec![#(#getter_tokens),*])
        }
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
    let reduce_aggregate_fn = if attrs.identity_aggregate {
        quote! {
            Some(|src: &dyn std::any::Any| -> Box<dyn crate::rules::traits::DynAggregateReductionResult> {
                let src = src.downcast_ref::<#source_type>().unwrap_or_else(|| {
                    panic!(
                        "DynAggregateReductionResult: source type mismatch: expected `{}`, got `{}`",
                        std::any::type_name::<#source_type>(),
                        std::any::type_name_of_val(src),
                    )
                });
                Box::new(<#source_type as crate::rules::ReduceTo<#target_type>>::reduce_to(src))
            })
        }
    } else {
        quote! { None }
    };

    // Collect generic parameter info from the impl block
    let type_generics = collect_type_generic_names(&impl_block.generics);

    // Generate variant fn bodies
    let source_variant_body = make_variant_fn_body(source_type, &type_generics)?;
    let target_variant_body = make_variant_fn_body(&target_type, &type_generics)?;

    if attrs.exact.is_none() && attrs.bound.is_none() && attrs.unavailable.is_none() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Missing size contract. Classify every target field with `exact`, `bound`, or `unavailable`.",
        ));
    }
    let exact = parse_expression_fields_to_expr(attrs.exact.as_deref().unwrap_or_default())?;
    let bounds = parse_expression_fields_to_expr(attrs.bound.as_deref().unwrap_or_default())?;
    let exact_tokens = generate_expression_fields(&exact);
    let bound_tokens = generate_expression_fields(&bounds);
    let unavailable_tokens = attrs
        .unavailable
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|(field, reason)| quote! { crate::rules::registry::UnavailableSizeField { field: #field, reason: #reason } });
    let source_fields = exact.iter().chain(&bounds).cloned().collect::<Vec<_>>();
    let source_size_fn = generate_source_size_fn(&source_fields, source_type);

    // Generate the combined output
    let output = quote! {
        #impl_block

        inventory::submit! {
            crate::rules::registry::ReductionEntry {
                source_name: #source_name,
                target_name: #target_name,
                source_variant_fn: || { #source_variant_body },
                target_variant_fn: || { #target_variant_body },
                size_declarations_fn: || crate::rules::registry::ReductionSizeDeclarations {
                    exact: #exact_tokens,
                    bounds: #bound_tokens,
                    unavailable: vec![#(#unavailable_tokens),*],
                },
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
                reduce_aggregate_fn: #reduce_aggregate_fn,
                turing: false,
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
    create_spec: Option<Type>,
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

            let mut aliases = Vec::new();
            let mut create_spec = None;
            while input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                if ident == "aliases" {
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        let lit: syn::LitStr = content.parse()?;
                        if lit.value().trim().is_empty() {
                            return Err(syn::Error::new(
                                lit.span(),
                                "variant alias must not be empty or whitespace-only",
                            ));
                        }
                        aliases.push(lit);
                        if content.peek(syn::Token![,]) {
                            content.parse::<syn::Token![,]>()?;
                        }
                    }
                } else if ident == "create" {
                    if create_spec.is_some() {
                        return Err(syn::Error::new(ident.span(), "duplicate `create` clause"));
                    }
                    create_spec = Some(input.parse()?);
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("expected `aliases` or `create`, found `{ident}`"),
                    ));
                }
            }

            entries.push(DeclareVariantEntry {
                is_default,
                ty,
                complexity,
                aliases,
                create_spec,
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
        let create_spec = &entry.create_spec;
        let complexity_str = entry.complexity.value();
        let is_default = entry.is_default;
        let alias_lits: Vec<_> = entry.aliases.iter().map(|s| s.value()).collect();

        // Parse the complexity expression to validate syntax
        let parsed = problemreductions_expr::Expr::try_parse(&complexity_str).map_err(|e| {
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

        let construction_fields = if let Some(create_spec) = create_spec {
            quote! {
                create_inputs: Some(<#create_spec as crate::registry::CreateSpec>::INPUTS),
                construct_fn: |data: serde_json::Value| -> Result<Box<dyn crate::registry::DynProblem>, crate::registry::ConstructionError> {
                    crate::registry::validate_create_inputs(
                        <#create_spec as crate::registry::CreateSpec>::INPUTS,
                        &data,
                    )?;
                    let spec: #create_spec = <#create_spec as crate::registry::CreateSpec>::deserialize_inputs(data)
                        .map_err(|error| crate::registry::ConstructionError::InvalidInput(error.to_string()))?;
                    let problem: #ty = <#ty as std::convert::TryFrom<#create_spec>>::try_from(spec)
                        .map_err(|error| crate::registry::ConstructionError::Conversion(error.to_string()))?;
                    Ok(Box::new(problem))
                },
            }
        } else {
            quote! {
                create_inputs: None,
                construct_fn: |data: serde_json::Value| -> Result<Box<dyn crate::registry::DynProblem>, crate::registry::ConstructionError> {
                    let problem_type = <#ty as crate::traits::Problem>::problem_type();
                    crate::registry::validate_direct_create_inputs(problem_type.fields, &data)?;
                    let problem: #ty = serde_json::from_value(data)
                        .map_err(|error| crate::registry::ConstructionError::InvalidInput(error.to_string()))?;
                    Ok(Box::new(problem))
                },
            }
        };

        let dispatch_fields = quote! {
            #construction_fields
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
    parsed: &problemreductions_expr::Expr,
    ty: &Type,
) -> syn::Result<TokenStream2> {
    let src_ident = syn::Ident::new("__src", proc_macro2::Span::call_site());
    let eval_tokens = complexity_estimate_tokens(parsed, &src_ident)?;

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
    fn size_fields_report_expression_domain_errors() {
        let fields = vec![("num_vertices".to_string(), "0 / 0".to_string())];
        let Err(error) = parse_expression_fields_to_expr(&fields) else {
            panic!("invalid size expression was accepted");
        };
        assert!(error.to_string().contains("division by zero"));
    }

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
        assert_eq!(
            err.to_string(),
            "expected `aliases` or `create`, found `nicknames`"
        );
    }

    #[test]
    fn create_spec_derive_generates_required_optional_and_codec_metadata() {
        let input: DeriveInput = syn::parse_quote! {
            struct ExampleCreateSpec {
                /// Required edge data.
                #[create(name = "edges", codec = "edge-list")]
                graph_edges: Vec<(usize, usize)>,
                /// Optional limit.
                limit: Option<usize>,
            }
        };
        let tokens = generate_create_spec(&input).unwrap().to_string();
        assert!(tokens.contains("CreateSpec for ExampleCreateSpec"));
        assert!(tokens.contains("const FIELDS"));
        assert!(tokens.contains("crate :: registry :: FieldInfo"));
        assert!(tokens.contains("name : \"edges\""));
        assert!(tokens.contains("type_name : \"Vec<(usize,usize)>\""));
        assert!(tokens.contains("required : true"));
        assert!(tokens.contains("required : false"));
        assert!(tokens.contains("CreateInputCodec :: EdgeList"));
        assert!(tokens.contains("Required edge data."));
    }

    #[test]
    fn create_spec_derive_rejects_unknown_codec() {
        let input: DeriveInput = syn::parse_quote! {
            struct ExampleCreateSpec {
                #[create(codec = "model-specific")]
                value: usize,
            }
        };
        let error = generate_create_spec(&input).unwrap_err();
        assert!(error.to_string().contains("unknown construction codec"));
    }

    #[test]
    fn create_spec_derive_supports_generics() {
        let input: DeriveInput = syn::parse_quote! {
            struct ExampleCreateSpec<T>
            where
                T: Clone,
            {
                /// Generic value.
                value: T,
            }
        };
        let tokens = generate_create_spec(&input).unwrap().to_string();
        assert!(tokens.contains("impl < T > crate :: registry :: CreateSpec"));
        assert!(tokens.contains("for ExampleCreateSpec < T >"));
        assert!(tokens.contains("where T : Clone"));
    }

    #[test]
    fn declare_variants_generates_custom_constructor() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            default Foo => "1" create FooCreateSpec aliases ["F"],
        };
        let tokens = generate_declare_variants(&input).unwrap().to_string();
        assert!(tokens.contains("create_inputs : Some"));
        assert!(tokens.contains("FooCreateSpec as crate :: registry :: CreateSpec"));
        assert!(tokens.contains("TryFrom < FooCreateSpec >"));
        assert!(tokens.contains("validate_create_inputs"));
    }

    #[test]
    fn declare_variants_generates_direct_constructor_by_default() {
        let input: DeclareVariantsInput = syn::parse_quote! {
            default Foo => "1",
        };
        let tokens = generate_declare_variants(&input).unwrap().to_string();
        assert!(tokens.contains("create_inputs : None"));
        assert!(tokens.contains("validate_direct_create_inputs"));
        assert!(tokens.contains("construct_fn :"));
    }

    #[test]
    fn declare_variants_rejects_duplicate_create_clause() {
        let error = syn::parse_str::<DeclareVariantsInput>(
            "default Foo => \"1\" create First create Second",
        )
        .err()
        .expect("duplicate create clause must fail");
        assert_eq!(error.to_string(), "duplicate `create` clause");
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
            #extra_attr = "unexpected", exact = { num_vertices = "num_vertices" }
        });
        let err = match parse_result {
            Ok(_) => panic!("unexpected reduction attribute should be rejected"),
            Err(err) => err,
        };
        assert!(err.to_string().contains("unknown attribute: extra"));
    }

    #[test]
    fn reduction_accepts_explicit_size_attributes() {
        let attrs: ReductionAttrs = syn::parse_quote! {
            exact = { n = n, squared = "n^2" },
            bound = { squared = "n^2" },
            unavailable = { encoding_bits = "coefficient magnitudes are not tracked" }
        };
        assert_eq!(
            attrs.exact,
            Some(vec![
                ("n".to_string(), "n".to_string()),
                ("squared".to_string(), "n^2".to_string()),
            ])
        );
        assert_eq!(attrs.bound, Some(vec![("squared".into(), "n^2".into())]));
        assert_eq!(
            attrs.unavailable,
            Some(vec![(
                "encoding_bits".into(),
                "coefficient magnitudes are not tracked".into()
            )])
        );
    }

    #[test]
    fn reduction_rejects_legacy_overhead_attribute() {
        let result = syn::parse2::<ReductionAttrs>(quote! {
            overhead = { ReductionOverhead::default() }
        });
        assert!(result.is_err());
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
