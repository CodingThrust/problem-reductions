use num_traits::ToPrimitive;
use problemreductions_expr::Expr;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) trait ExprCodegen {
    fn to_expr_tokens(&self) -> TokenStream;
    fn to_eval_tokens(&self, source: &syn::Ident) -> TokenStream;
}

impl ExprCodegen for Expr {
    fn to_expr_tokens(&self) -> TokenStream {
        match self {
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
            Expr::Var(name) => quote! { crate::expr::Expr::variable(#name) },
            Expr::Add(left, right) => {
                binary_tokens(left, right, |left, right| quote! { (#left) + (#right) })
            }
            Expr::Sub(left, right) => {
                binary_tokens(left, right, |left, right| quote! { (#left) - (#right) })
            }
            Expr::Mul(left, right) => {
                binary_tokens(left, right, |left, right| quote! { (#left) * (#right) })
            }
            Expr::Div(left, right) => {
                binary_tokens(left, right, |left, right| quote! { (#left) / (#right) })
            }
            Expr::Pow(base, exponent) => {
                let base = base.to_expr_tokens();
                let exponent = exponent.to_expr_tokens();
                quote! { crate::expr::Expr::pow(#base, #exponent) }
            }
            Expr::Neg(value) => {
                let value = value.to_expr_tokens();
                quote! { -(#value) }
            }
            Expr::Exp(value) => unary_tokens(
                value,
                |value| quote! { crate::expr::Expr::Exp(Box::new(#value)) },
            ),
            Expr::Log(value) => unary_tokens(
                value,
                |value| quote! { crate::expr::Expr::Log(Box::new(#value)) },
            ),
            Expr::Sqrt(value) => unary_tokens(
                value,
                |value| quote! { crate::expr::Expr::Sqrt(Box::new(#value)) },
            ),
            Expr::Factorial(value) => unary_tokens(
                value,
                |value| quote! { crate::expr::Expr::Factorial(Box::new(#value)) },
            ),
        }
    }

    fn to_eval_tokens(&self, source: &syn::Ident) -> TokenStream {
        match self {
            Expr::Const(value) => {
                let value = value
                    .to_f64()
                    .expect("expression constant must fit the temporary f64 evaluator");
                quote! { #value }
            }
            Expr::Var(name) => {
                let getter = syn::Ident::new(name, proc_macro2::Span::call_site());
                quote! { (#source.#getter() as f64) }
            }
            Expr::Add(left, right) => eval_binary_tokens(
                left,
                right,
                source,
                |left, right| quote! { (#left + #right) },
            ),
            Expr::Sub(left, right) => eval_binary_tokens(
                left,
                right,
                source,
                |left, right| quote! { (#left - #right) },
            ),
            Expr::Mul(left, right) => eval_binary_tokens(
                left,
                right,
                source,
                |left, right| quote! { ::std::ops::Mul::mul(#left, #right) },
            ),
            Expr::Div(left, right) => eval_binary_tokens(
                left,
                right,
                source,
                |left, right| quote! { (#left / #right) },
            ),
            Expr::Pow(base, exponent) => eval_binary_tokens(
                base,
                exponent,
                source,
                |base, exponent| quote! { f64::powf(#base, #exponent) },
            ),
            Expr::Neg(value) => {
                let value = value.to_eval_tokens(source);
                quote! { -(#value) }
            }
            Expr::Exp(value) => {
                eval_unary_tokens(value, source, |value| quote! { f64::exp(#value) })
            }
            Expr::Log(value) => {
                eval_unary_tokens(value, source, |value| quote! { f64::ln(#value) })
            }
            Expr::Sqrt(value) => {
                eval_unary_tokens(value, source, |value| quote! { f64::sqrt(#value) })
            }
            Expr::Factorial(value) => {
                let value = value.to_eval_tokens(source);
                quote! {{
                    let __n = #value;
                    let __rounded = __n.round();
                    if (__n - __rounded).abs() < 1e-10 && __rounded >= 0.0 {
                        (2..=(__rounded as u64)).fold(1.0f64, |product, factor| product * factor as f64)
                    } else {
                        (2.0 * ::std::f64::consts::PI * __n).sqrt()
                            * (__n / ::std::f64::consts::E).powf(__n)
                    }
                }}
            }
        }
    }
}

fn binary_tokens(
    left: &Expr,
    right: &Expr,
    build: impl FnOnce(TokenStream, TokenStream) -> TokenStream,
) -> TokenStream {
    build(left.to_expr_tokens(), right.to_expr_tokens())
}

fn unary_tokens(value: &Expr, build: impl FnOnce(TokenStream) -> TokenStream) -> TokenStream {
    build(value.to_expr_tokens())
}

fn eval_binary_tokens(
    left: &Expr,
    right: &Expr,
    source: &syn::Ident,
    build: impl FnOnce(TokenStream, TokenStream) -> TokenStream,
) -> TokenStream {
    build(left.to_eval_tokens(source), right.to_eval_tokens(source))
}

fn eval_unary_tokens(
    value: &Expr,
    source: &syn::Ident,
    build: impl FnOnce(TokenStream) -> TokenStream,
) -> TokenStream {
    build(value.to_eval_tokens(source))
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
        assert!(!expression.to_expr_tokens().is_empty());
    }
}
