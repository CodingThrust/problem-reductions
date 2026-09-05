use num_traits::ToPrimitive;
use problemreductions_expr::{Expr, ExprNode};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn expr_tokens(expression: &Expr) -> TokenStream {
    match expression.node() {
        ExprNode::Const(value) => {
            let numerator = value.numer().to_string();
            let denominator = value.denom().to_string();
            quote! {
                crate::expr::Expr::rational(
                    #numerator.parse::<crate::expr::BigInt>().expect("macro-generated numerator must be valid"),
                    #denominator.parse::<crate::expr::BigInt>().expect("macro-generated denominator must be valid"),
                )
            }
        }
        ExprNode::Var(name) => {
            let name = name.as_str();
            quote! { crate::expr::Expr::variable(#name) }
        }
        ExprNode::Add(values) => {
            nary_expr_tokens(values, |left, right| quote! { (#left) + (#right) })
        }
        ExprNode::Mul(values) => {
            nary_expr_tokens(values, |left, right| quote! { (#left) * (#right) })
        }
        ExprNode::Pow(base, exponent) => {
            let base = expr_tokens(base);
            let exponent = expr_tokens(exponent);
            quote! { crate::expr::Expr::pow(#base, #exponent) }
        }
        ExprNode::Exp(value) => {
            unary_expr_tokens(value, |value| quote! { crate::expr::Expr::exp(#value) })
        }
        ExprNode::Log(value) => {
            unary_expr_tokens(value, |value| quote! { crate::expr::Expr::log(#value) })
        }
        ExprNode::Factorial(value) => unary_expr_tokens(
            value,
            |value| quote! { crate::expr::Expr::factorial(#value) },
        ),
    }
}

pub(crate) fn complexity_estimate_tokens(
    expression: &Expr,
    parameters: &syn::Ident,
) -> syn::Result<TokenStream> {
    Ok(match expression.node() {
        ExprNode::Const(value) => {
            let value =
                value
                    .to_f64()
                    .filter(|value| value.is_finite())
                    .ok_or_else(|| {
                        syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("exact expression constant {value} is outside complexity estimation"),
                )
                    })?;
            quote! { #value }
        }
        ExprNode::Var(name) => {
            let name = name.as_str();
            quote! {
                (#parameters
                    .get(#name)
                    .expect("validated complexity parameter must be present") as f64)
            }
        }
        ExprNode::Add(values) => nary_estimate_tokens(
            values,
            parameters,
            |left, right| quote! { (#left + #right) },
        )?,
        ExprNode::Mul(values) => nary_estimate_tokens(
            values,
            parameters,
            |left, right| quote! { (#left * #right) },
        )?,
        ExprNode::Pow(base, exponent) => {
            let base = complexity_estimate_tokens(base, parameters)?;
            let exponent = complexity_estimate_tokens(exponent, parameters)?;
            quote! { f64::powf(#base, #exponent) }
        }
        ExprNode::Exp(value) => {
            let value = complexity_estimate_tokens(value, parameters)?;
            quote! { f64::exp(#value) }
        }
        ExprNode::Log(value) => {
            let value = complexity_estimate_tokens(value, parameters)?;
            quote! { f64::ln(#value) }
        }
        ExprNode::Factorial(value) => {
            let value = complexity_estimate_tokens(value, parameters)?;
            quote! {
                crate::expr::approximate_factorial(#value)
                    .expect("complexity factorial requires a non-negative integer")
            }
        }
    })
}

fn nary_estimate_tokens(
    values: &[Expr],
    parameters: &syn::Ident,
    build: impl Fn(TokenStream, TokenStream) -> TokenStream,
) -> syn::Result<TokenStream> {
    let mut values = values.iter();
    let first = complexity_estimate_tokens(
        values
            .next()
            .expect("canonical n-ary expression has operands"),
        parameters,
    )?;
    values.try_fold(first, |left, value| {
        Ok(build(left, complexity_estimate_tokens(value, parameters)?))
    })
}

fn nary_expr_tokens(
    values: &[Expr],
    build: impl Fn(TokenStream, TokenStream) -> TokenStream,
) -> TokenStream {
    let mut values = values.iter().map(expr_tokens);
    let first = values
        .next()
        .expect("normalized n-ary expression has at least two operands");
    values.fold(first, build)
}

fn unary_expr_tokens(value: &Expr, build: impl FnOnce(TokenStream) -> TokenStream) -> TokenStream {
    build(expr_tokens(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_parser_drives_codegen() {
        let expression = Expr::parse("n * (n - 1) / 2 - m");
        assert!(matches!(expression.node(), ExprNode::Add(_)));
        assert_eq!(
            expression.variables().into_iter().collect::<Vec<_>>(),
            vec!["m", "n"]
        );
        assert!(!expr_tokens(&expression).is_empty());
    }

    #[test]
    fn codegen_covers_every_semantic_operator() {
        let expression = Expr::parse("exp(n) + log(n) + factorial(n) + n^2");
        let constructed = expr_tokens(&expression).to_string();
        assert!(constructed.contains("Expr :: exp"));
        assert!(constructed.contains("Expr :: log"));
        assert!(constructed.contains("Expr :: factorial"));
        assert!(constructed.contains("Expr :: pow"));
    }
}
