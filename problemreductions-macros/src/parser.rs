//! Pratt parser for overhead expression strings.
//!
//! Parses expressions like:
//! - `"num_vertices"`
//! - `"num_vertices^2"`
//! - `"num_edges + num_vertices^2"`
//! - `"3 * num_vertices"`
//! - `"exp(num_vertices^2)"`
//! - `"sqrt(num_edges)"`
//!
//! Grammar:
//!   expr     = term (('+' | '-') term)*
//!   term     = factor (('*' | '/') factor)*
//!   factor   = unary ('^' factor)?        // right-associative
//!   unary    = '-' unary | primary
//!   primary  = NUMBER | IDENT | func_call | '(' expr ')'
//!   func_call = ('exp' | 'log' | 'sqrt' | 'factorial') '(' expr ')'

use proc_macro2::TokenStream;
use quote::quote;

/// Parsed expression node (intermediate representation before codegen).
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedExpr {
    Const(f64),
    Var(String),
    Add(Box<ParsedExpr>, Box<ParsedExpr>),
    Sub(Box<ParsedExpr>, Box<ParsedExpr>),
    Mul(Box<ParsedExpr>, Box<ParsedExpr>),
    Div(Box<ParsedExpr>, Box<ParsedExpr>),
    Pow(Box<ParsedExpr>, Box<ParsedExpr>),
    Neg(Box<ParsedExpr>),
    Exp(Box<ParsedExpr>),
    Log(Box<ParsedExpr>),
    Sqrt(Box<ParsedExpr>),
    Factorial(Box<ParsedExpr>),
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '^' => {
                chars.next();
                tokens.push(Token::Caret);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            c if c.is_ascii_digit() || c == '.' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val: f64 = num.parse().map_err(|_| format!("invalid number: {num}"))?;
                tokens.push(Token::Number(val));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(ident));
            }
            _ => return Err(format!("unexpected character: '{ch}'")),
        }
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.advance() {
            Some(ref tok) if tok == expected => Ok(()),
            Some(tok) => Err(format!("expected {expected:?}, got {tok:?}")),
            None => Err(format!("expected {expected:?}, got end of input")),
        }
    }

    fn parse_expr(&mut self) -> Result<ParsedExpr, String> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = self.advance().unwrap();
            let right = self.parse_term()?;
            left = match op {
                Token::Plus => ParsedExpr::Add(Box::new(left), Box::new(right)),
                Token::Minus => ParsedExpr::Sub(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<ParsedExpr, String> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek(), Some(Token::Star) | Some(Token::Slash)) {
            let op = self.advance().unwrap();
            let right = self.parse_factor()?;
            left = match op {
                Token::Star => ParsedExpr::Mul(Box::new(left), Box::new(right)),
                Token::Slash => ParsedExpr::Div(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<ParsedExpr, String> {
        let base = self.parse_unary()?;
        if matches!(self.peek(), Some(Token::Caret)) {
            self.advance();
            let exp = self.parse_factor()?; // right-associative
            Ok(ParsedExpr::Pow(Box::new(base), Box::new(exp)))
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<ParsedExpr, String> {
        if matches!(self.peek(), Some(Token::Minus)) {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(ParsedExpr::Neg(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<ParsedExpr, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(ParsedExpr::Const(n)),
            Some(Token::Ident(name)) => {
                // Check for function call: exp(...), log(...), sqrt(...)
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.advance(); // consume '('
                    let arg = self.parse_expr()?;
                    self.expect(&Token::RParen)?;
                    match name.as_str() {
                        "exp" => Ok(ParsedExpr::Exp(Box::new(arg))),
                        "log" => Ok(ParsedExpr::Log(Box::new(arg))),
                        "sqrt" => Ok(ParsedExpr::Sqrt(Box::new(arg))),
                        "factorial" => Ok(ParsedExpr::Factorial(Box::new(arg))),
                        _ => Err(format!("unknown function: {name}")),
                    }
                } else {
                    Ok(ParsedExpr::Var(name))
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Some(tok) => Err(format!("unexpected token: {tok:?}")),
            None => Err("unexpected end of input".to_string()),
        }
    }
}

/// Parse an expression string into a ParsedExpr.
pub fn parse_expr(input: &str) -> Result<ParsedExpr, String> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr()?;
    if parser.pos != parser.tokens.len() {
        return Err(format!(
            "unexpected trailing tokens at position {}",
            parser.pos
        ));
    }
    Ok(expr)
}

impl ParsedExpr {
    /// Generate TokenStream that constructs an `Expr` value.
    pub fn to_expr_tokens(&self) -> TokenStream {
        match self {
            ParsedExpr::Const(c) => quote! { crate::expr::Expr::Const(#c) },
            ParsedExpr::Var(name) => quote! { crate::expr::Expr::Var(#name) },
            ParsedExpr::Add(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { (#a) + (#b) }
            }
            ParsedExpr::Sub(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { (#a) - (#b) }
            }
            ParsedExpr::Mul(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { (#a) * (#b) }
            }
            ParsedExpr::Div(a, b) => {
                let a = a.to_expr_tokens();
                let b = b.to_expr_tokens();
                quote! { (#a) / (#b) }
            }
            ParsedExpr::Pow(base, exp) => {
                let base = base.to_expr_tokens();
                let exp = exp.to_expr_tokens();
                quote! { crate::expr::Expr::pow(#base, #exp) }
            }
            ParsedExpr::Neg(a) => {
                let a = a.to_expr_tokens();
                quote! { -(#a) }
            }
            ParsedExpr::Exp(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::Exp(Box::new(#a)) }
            }
            ParsedExpr::Log(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::Log(Box::new(#a)) }
            }
            ParsedExpr::Sqrt(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::Sqrt(Box::new(#a)) }
            }
            ParsedExpr::Factorial(a) => {
                let a = a.to_expr_tokens();
                quote! { crate::expr::Expr::Factorial(Box::new(#a)) }
            }
        }
    }

    /// Generate TokenStream that evaluates the expression by calling getter methods
    /// on a source variable `src`.
    pub fn to_eval_tokens(&self, src_ident: &syn::Ident) -> TokenStream {
        match self {
            ParsedExpr::Const(c) => quote! { (#c as f64) },
            ParsedExpr::Var(name) => {
                let getter = syn::Ident::new(name, proc_macro2::Span::call_site());
                quote! { (#src_ident.#getter() as f64) }
            }
            ParsedExpr::Add(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a + #b) }
            }
            ParsedExpr::Sub(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a - #b) }
            }
            ParsedExpr::Mul(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a * #b) }
            }
            ParsedExpr::Div(a, b) => {
                let a = a.to_eval_tokens(src_ident);
                let b = b.to_eval_tokens(src_ident);
                quote! { (#a / #b) }
            }
            ParsedExpr::Pow(base, exp) => {
                let base = base.to_eval_tokens(src_ident);
                let exp = exp.to_eval_tokens(src_ident);
                quote! { f64::powf(#base, #exp) }
            }
            ParsedExpr::Neg(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { (-(#a)) }
            }
            ParsedExpr::Exp(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { f64::exp(#a) }
            }
            ParsedExpr::Log(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { f64::ln(#a) }
            }
            ParsedExpr::Sqrt(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { f64::sqrt(#a) }
            }
            ParsedExpr::Factorial(a) => {
                let a = a.to_eval_tokens(src_ident);
                quote! { {
                    let __n = #a;
                    let __r = __n.round();
                    if (__n - __r).abs() < 1e-10 && __r >= 0.0 {
                        let mut __f = 1u64;
                        let __k = __r as u64;
                        let mut __i = 2u64;
                        while __i <= __k { __f = __f.saturating_mul(__i); __i += 1; }
                        __f as f64
                    } else {
                        (2.0 * ::std::f64::consts::PI * __n).sqrt() * (__n / ::std::f64::consts::E).powf(__n)
                    }
                } }
            }
        }
    }

    /// Collect all variable names in the expression.
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }

    fn collect_vars(&self, vars: &mut Vec<String>) {
        match self {
            ParsedExpr::Const(_) => {}
            ParsedExpr::Var(name) => vars.push(name.clone()),
            ParsedExpr::Add(a, b)
            | ParsedExpr::Sub(a, b)
            | ParsedExpr::Mul(a, b)
            | ParsedExpr::Div(a, b)
            | ParsedExpr::Pow(a, b) => {
                a.collect_vars(vars);
                b.collect_vars(vars);
            }
            ParsedExpr::Neg(a)
            | ParsedExpr::Exp(a)
            | ParsedExpr::Log(a)
            | ParsedExpr::Sqrt(a)
            | ParsedExpr::Factorial(a) => {
                a.collect_vars(vars);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_var() {
        assert_eq!(
            parse_expr("num_vertices").unwrap(),
            ParsedExpr::Var("num_vertices".into())
        );
    }

    #[test]
    fn test_parse_const() {
        assert_eq!(parse_expr("42").unwrap(), ParsedExpr::Const(42.0));
    }

    #[test]
    fn test_parse_pow() {
        let e = parse_expr("n^2").unwrap();
        assert_eq!(
            e,
            ParsedExpr::Pow(
                Box::new(ParsedExpr::Var("n".into())),
                Box::new(ParsedExpr::Const(2.0)),
            )
        );
    }

    #[test]
    fn test_parse_add_mul() {
        // n + 3 * m  →  n + (3*m)
        let e = parse_expr("n + 3 * m").unwrap();
        assert_eq!(
            e,
            ParsedExpr::Add(
                Box::new(ParsedExpr::Var("n".into())),
                Box::new(ParsedExpr::Mul(
                    Box::new(ParsedExpr::Const(3.0)),
                    Box::new(ParsedExpr::Var("m".into())),
                )),
            )
        );
    }

    #[test]
    fn test_parse_exp() {
        let e = parse_expr("exp(n^2)").unwrap();
        assert_eq!(
            e,
            ParsedExpr::Exp(Box::new(ParsedExpr::Pow(
                Box::new(ParsedExpr::Var("n".into())),
                Box::new(ParsedExpr::Const(2.0)),
            )))
        );
    }

    #[test]
    fn test_parse_complex() {
        // 3 * n^2 + exp(m) — should parse correctly
        let e = parse_expr("3 * n^2 + exp(m)").unwrap();
        assert!(matches!(e, ParsedExpr::Add(_, _)));
    }

    #[test]
    fn test_parse_parens() {
        let e = parse_expr("(n + m)^2").unwrap();
        assert!(matches!(e, ParsedExpr::Pow(_, _)));
    }

    #[test]
    fn test_variables() {
        let e = parse_expr("n^2 + 3 * m + exp(k)").unwrap();
        assert_eq!(e.variables(), vec!["k", "m", "n"]);
    }

    #[test]
    fn test_parse_neg() {
        let e = parse_expr("-n").unwrap();
        assert_eq!(e, ParsedExpr::Neg(Box::new(ParsedExpr::Var("n".into()))));
    }

    #[test]
    fn test_parse_sub() {
        let e = parse_expr("n - m").unwrap();
        assert_eq!(
            e,
            ParsedExpr::Sub(
                Box::new(ParsedExpr::Var("n".into())),
                Box::new(ParsedExpr::Var("m".into())),
            )
        );
    }
}
