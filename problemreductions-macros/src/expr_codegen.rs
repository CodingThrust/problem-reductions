use num_traits::ToPrimitive;
use problemreductions_expr::Expr;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn expr_tokens(expression: &Expr) -> TokenStream {
    match expression {
        Expr::Const(value) => {
            let numerator = value.numer().to_string();
            let denominator = value.denom().to_string();
            quote! {
                crate::expr::Expr::rational(
                    #numerator.parse::<crate::expr::BigInt>().expect("macro-generated numerator must be valid"),
                    #denominator.parse::<crate::expr::BigInt>().expect("macro-generated denominator must be valid"),
                )
            }
        }
        Expr::Var(name) => {
            let name = name.as_str();
            quote! { crate::expr::Expr::variable(#name) }
        }
        Expr::Add(left, right) => {
            binary_expr_tokens(left, right, |left, right| quote! { (#left) + (#right) })
        }
        Expr::Sub(left, right) => {
            binary_expr_tokens(left, right, |left, right| quote! { (#left) - (#right) })
        }
        Expr::Mul(left, right) => {
            binary_expr_tokens(left, right, |left, right| quote! { (#left) * (#right) })
        }
        Expr::Div(left, right) => {
            binary_expr_tokens(left, right, |left, right| quote! { (#left) / (#right) })
        }
        Expr::Pow(base, exponent) => {
            let base = expr_tokens(base);
            let exponent = expr_tokens(exponent);
            quote! { crate::expr::Expr::pow(#base, #exponent) }
        }
        Expr::Neg(value) => {
            let value = expr_tokens(value);
            quote! { -(#value) }
        }
        Expr::Exp(value) => unary_expr_tokens(
            value,
            |value| quote! { crate::expr::Expr::Exp(Box::new(#value)) },
        ),
        Expr::Log(value) => unary_expr_tokens(
            value,
            |value| quote! { crate::expr::Expr::Log(Box::new(#value)) },
        ),
        Expr::Sqrt(value) => unary_expr_tokens(
            value,
            |value| quote! { crate::expr::Expr::Sqrt(Box::new(#value)) },
        ),
        Expr::Factorial(value) => unary_expr_tokens(
            value,
            |value| quote! { crate::expr::Expr::Factorial(Box::new(#value)) },
        ),
    }
}

pub(crate) fn eval_tokens(expression: &Expr, source: &syn::Ident) -> syn::Result<TokenStream> {
    Ok(match expression {
        Expr::Const(value) => {
            let value = value.to_f64().ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("exact expression constant {value} is outside the f64 evaluator"),
                )
            })?;
            quote! { #value }
        }
        Expr::Var(name) => {
            let getter = syn::parse_str::<syn::Ident>(name.as_str()).map_err(|_| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("expression variable {name:?} is not a valid Rust getter name"),
                )
            })?;
            quote! { (#source.#getter() as f64) }
        }
        Expr::Add(left, right) => binary_eval_tokens(
            left,
            right,
            source,
            |left, right| quote! { (#left + #right) },
        )?,
        Expr::Sub(left, right) => binary_eval_tokens(
            left,
            right,
            source,
            |left, right| quote! { (#left - #right) },
        )?,
        Expr::Mul(left, right) => binary_eval_tokens(
            left,
            right,
            source,
            |left, right| quote! { ::std::ops::Mul::mul(#left, #right) },
        )?,
        Expr::Div(left, right) => binary_eval_tokens(
            left,
            right,
            source,
            |left, right| quote! { (#left / #right) },
        )?,
        Expr::Pow(base, exponent) => binary_eval_tokens(
            base,
            exponent,
            source,
            |base, exponent| quote! { f64::powf(#base, #exponent) },
        )?,
        Expr::Neg(value) => {
            let value = eval_tokens(value, source)?;
            quote! { -(#value) }
        }
        Expr::Exp(value) => unary_eval_tokens(value, source, |value| quote! { f64::exp(#value) })?,
        Expr::Log(value) => unary_eval_tokens(value, source, |value| quote! { f64::ln(#value) })?,
        Expr::Sqrt(value) => {
            unary_eval_tokens(value, source, |value| quote! { f64::sqrt(#value) })?
        }
        Expr::Factorial(value) => {
            let value = eval_tokens(value, source)?;
            quote! {
                crate::expr::approximate_factorial(#value)
                    .expect("factorial argument must evaluate to a non-negative integer")
            }
        }
    })
}

fn binary_expr_tokens(
    left: &Expr,
    right: &Expr,
    build: impl FnOnce(TokenStream, TokenStream) -> TokenStream,
) -> TokenStream {
    build(expr_tokens(left), expr_tokens(right))
}

fn unary_expr_tokens(value: &Expr, build: impl FnOnce(TokenStream) -> TokenStream) -> TokenStream {
    build(expr_tokens(value))
}

fn binary_eval_tokens(
    left: &Expr,
    right: &Expr,
    source: &syn::Ident,
    build: impl FnOnce(TokenStream, TokenStream) -> TokenStream,
) -> syn::Result<TokenStream> {
    Ok(build(
        eval_tokens(left, source)?,
        eval_tokens(right, source)?,
    ))
}

fn unary_eval_tokens(
    value: &Expr,
    source: &syn::Ident,
    build: impl FnOnce(TokenStream) -> TokenStream,
) -> syn::Result<TokenStream> {
    Ok(build(eval_tokens(value, source)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_parser_drives_codegen() {
        let expression = Expr::parse("n * (n - 1) / 2 - m");
        assert!(matches!(expression, Expr::Sub(_, _)));
        assert_eq!(
            expression.variables().into_iter().collect::<Vec<_>>(),
            vec!["m", "n"]
        );
        assert!(!expr_tokens(&expression).is_empty());
        let source = syn::Ident::new("source", proc_macro2::Span::call_site());
        assert!(!eval_tokens(&expression, &source).unwrap().is_empty());
    }
}
