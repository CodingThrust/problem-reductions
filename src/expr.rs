//! Symbolic expression system for reduction overhead.
//!
//! Provides a DSL for expressing how problem sizes transform during reductions.
//! Supports arithmetic, exponentiation, and built-in math functions.

use crate::types::ProblemSize;
use std::fmt;

/// A symbolic expression over named variables.
#[derive(Clone, Debug)]
pub enum Expr {
    /// Numeric literal.
    Num(f64),
    /// Named variable (e.g., `num_vertices`).
    Var(Box<str>),
    /// Binary operation.
    BinOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// Unary negation.
    Neg(Box<Expr>),
    /// Built-in function call.
    Call { func: Func, args: Vec<Expr> },
}

/// Binary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// Built-in functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Func {
    Log2,
    Log10,
    Ln,
    Exp,
    Sqrt,
    Min,
    Max,
    Floor,
    Ceil,
    Abs,
}

/// Evaluation error.
#[derive(Debug)]
pub enum EvalError {
    UnknownVar(Box<str>),
    DivideByZero,
    Arity {
        func: Func,
        expected: usize,
        got: usize,
    },
    Domain {
        func: Func,
        detail: Box<str>,
    },
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UnknownVar(name) => write!(f, "unknown variable: {name}"),
            EvalError::DivideByZero => write!(f, "division by zero"),
            EvalError::Arity {
                func,
                expected,
                got,
            } => write!(f, "{func:?} expects {expected} args, got {got}"),
            EvalError::Domain { func, detail } => write!(f, "{func:?}: {detail}"),
        }
    }
}

impl std::error::Error for EvalError {}

// ── AST construction ──

impl Expr {
    /// Convenience constructor for binary operations.
    pub fn binop(op: BinOp, lhs: Expr, rhs: Expr) -> Self {
        Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

// ── Evaluator ──

impl Expr {
    /// Evaluate the expression given variable bindings from `ProblemSize`.
    pub fn evaluate(&self, size: &ProblemSize) -> Result<f64, EvalError> {
        match self {
            Expr::Num(v) => Ok(*v),
            Expr::Var(name) => size
                .get(name)
                .map(|v| v as f64)
                .ok_or_else(|| EvalError::UnknownVar(name.clone())),
            Expr::Neg(inner) => Ok(-inner.evaluate(size)?),
            Expr::BinOp { op, lhs, rhs } => {
                let l = lhs.evaluate(size)?;
                let r = rhs.evaluate(size)?;
                match op {
                    BinOp::Add => Ok(l + r),
                    BinOp::Sub => Ok(l - r),
                    BinOp::Mul => Ok(l * r),
                    BinOp::Div => {
                        if r == 0.0 {
                            return Err(EvalError::DivideByZero);
                        }
                        Ok(l / r)
                    }
                    BinOp::Pow => {
                        if l < 0.0 && r.fract() != 0.0 {
                            return Err(EvalError::Domain {
                                func: Func::Sqrt,
                                detail: "negative base with non-integer exponent".into(),
                            });
                        }
                        let result = l.powf(r);
                        if result.is_nan() || result.is_infinite() {
                            return Err(EvalError::Domain {
                                func: Func::Exp,
                                detail: format!("{l} ^ {r} produced non-finite result").into(),
                            });
                        }
                        Ok(result)
                    }
                }
            }
            Expr::Call { func, args } => eval_func(*func, args, size),
        }
    }
}

fn eval_func(func: Func, args: &[Expr], size: &ProblemSize) -> Result<f64, EvalError> {
    let (min_args, max_args) = match func {
        Func::Min | Func::Max => (2, 2),
        _ => (1, 1),
    };
    if args.len() < min_args || args.len() > max_args {
        return Err(EvalError::Arity {
            func,
            expected: min_args,
            got: args.len(),
        });
    }

    let a = args[0].evaluate(size)?;
    match func {
        Func::Log2 => {
            if a <= 0.0 {
                return Err(EvalError::Domain {
                    func,
                    detail: "log2 of non-positive".into(),
                });
            }
            Ok(a.log2())
        }
        Func::Log10 => {
            if a <= 0.0 {
                return Err(EvalError::Domain {
                    func,
                    detail: "log10 of non-positive".into(),
                });
            }
            Ok(a.log10())
        }
        Func::Ln => {
            if a <= 0.0 {
                return Err(EvalError::Domain {
                    func,
                    detail: "ln of non-positive".into(),
                });
            }
            Ok(a.ln())
        }
        Func::Exp => {
            let result = a.exp();
            if result.is_infinite() {
                return Err(EvalError::Domain {
                    func,
                    detail: "exp overflow".into(),
                });
            }
            Ok(result)
        }
        Func::Sqrt => {
            if a < 0.0 {
                return Err(EvalError::Domain {
                    func,
                    detail: "sqrt of negative".into(),
                });
            }
            Ok(a.sqrt())
        }
        Func::Abs => Ok(a.abs()),
        Func::Floor => Ok(a.floor()),
        Func::Ceil => Ok(a.ceil()),
        Func::Min => {
            let b = args[1].evaluate(size)?;
            Ok(a.min(b))
        }
        Func::Max => {
            let b = args[1].evaluate(size)?;
            Ok(a.max(b))
        }
    }
}

// ── Parser ──

/// A source span for error reporting.
#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Parse error.
#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar { ch: char, pos: usize },
    UnexpectedEof,
    UnexpectedToken { span: Span, detail: String },
    UnknownFunction { name: String, span: Span },
    TrailingInput { span: Span },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedChar { ch, pos } => {
                write!(f, "unexpected character '{ch}' at position {pos}")
            }
            ParseError::UnexpectedEof => write!(f, "unexpected end of input"),
            ParseError::UnexpectedToken { span, detail } => {
                write!(f, "{detail} at {}..{}", span.start, span.end)
            }
            ParseError::UnknownFunction { name, span } => {
                write!(
                    f,
                    "unknown function '{name}' at {}..{}",
                    span.start, span.end
                )
            }
            ParseError::TrailingInput { span } => {
                write!(f, "trailing input at {}..{}", span.start, span.end)
            }
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Clone, Debug)]
enum Token {
    Num(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    Eof,
}

#[derive(Clone, Debug)]
struct SpannedToken {
    token: Token,
    span: Span,
}

struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn next_token(&mut self) -> Result<SpannedToken, ParseError> {
        self.skip_whitespace();
        let start = self.pos;

        if self.pos >= self.input.len() {
            return Ok(SpannedToken {
                token: Token::Eof,
                span: Span { start, end: start },
            });
        }

        let ch = self.input[self.pos] as char;
        match ch {
            '+' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::Plus,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            '-' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::Minus,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            '*' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::Star,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            '/' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::Slash,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            '^' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::Caret,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            '(' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::LParen,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            ')' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::RParen,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            ',' => {
                self.pos += 1;
                Ok(SpannedToken {
                    token: Token::Comma,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                })
            }
            c if c.is_ascii_digit() || c == '.' => self.lex_number(start),
            c if c.is_ascii_alphabetic() || c == '_' => self.lex_ident(start),
            _ => Err(ParseError::UnexpectedChar { ch, pos: start }),
        }
    }

    fn lex_number(&mut self, start: usize) -> Result<SpannedToken, ParseError> {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos < self.input.len() && self.input[self.pos] == b'.' {
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }
        let s = std::str::from_utf8(&self.input[start..self.pos]).unwrap();
        let val: f64 = s.parse().map_err(|_| ParseError::UnexpectedChar {
            ch: s.chars().next().unwrap_or('?'),
            pos: start,
        })?;
        Ok(SpannedToken {
            token: Token::Num(val),
            span: Span {
                start,
                end: self.pos,
            },
        })
    }

    fn lex_ident(&mut self, start: usize) -> Result<SpannedToken, ParseError> {
        while self.pos < self.input.len()
            && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == b'_')
        {
            self.pos += 1;
        }
        let s = std::str::from_utf8(&self.input[start..self.pos]).unwrap();
        Ok(SpannedToken {
            token: Token::Ident(s.to_string()),
            span: Span {
                start,
                end: self.pos,
            },
        })
    }
}

struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token()?;
            let is_eof = matches!(tok.token, Token::Eof);
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(Self { tokens, pos: 0 })
    }

    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &SpannedToken {
        let tok = &self.tokens[self.pos];
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    fn expect_rparen(&mut self) -> Result<(), ParseError> {
        match &self.peek().token {
            Token::RParen => {
                self.advance();
                Ok(())
            }
            _ => Err(ParseError::UnexpectedToken {
                span: self.peek().span,
                detail: "expected ')'".to_string(),
            }),
        }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;

        loop {
            let (op, l_bp, r_bp) = match &self.peek().token {
                Token::Plus => (BinOp::Add, 1, 2),
                Token::Minus => (BinOp::Sub, 1, 2),
                Token::Star => (BinOp::Mul, 3, 4),
                Token::Slash => (BinOp::Div, 3, 4),
                Token::Caret => (BinOp::Pow, 7, 6), // right-assoc
                _ => break,
            };

            if l_bp < min_bp {
                break;
            }

            self.advance();
            let rhs = self.parse_expr(r_bp)?;
            lhs = Expr::binop(op, lhs, rhs);
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        let tok = self.peek().clone();
        match &tok.token {
            Token::Num(v) => {
                let v = *v;
                self.advance();
                Ok(Expr::Num(v))
            }
            Token::Minus => {
                self.advance();
                let inner = self.parse_expr(5)?; // unary minus bp
                Ok(Expr::Neg(Box::new(inner)))
            }
            Token::LParen => {
                self.advance();
                let inner = self.parse_expr(0)?;
                self.expect_rparen()?;
                Ok(inner)
            }
            Token::Ident(name) => {
                let name = name.clone();
                let span = tok.span;
                self.advance();

                // Check if followed by '(' — function call
                if matches!(self.peek().token, Token::LParen) {
                    self.advance(); // consume '('
                    let func = resolve_func(&name, span)?;
                    let mut args = Vec::new();
                    if !matches!(self.peek().token, Token::RParen) {
                        args.push(self.parse_expr(0)?);
                        while matches!(self.peek().token, Token::Comma) {
                            self.advance();
                            args.push(self.parse_expr(0)?);
                        }
                    }
                    self.expect_rparen()?;
                    Ok(Expr::Call { func, args })
                } else {
                    Ok(Expr::Var(name.into()))
                }
            }
            Token::Eof => Err(ParseError::UnexpectedEof),
            _ => Err(ParseError::UnexpectedToken {
                span: tok.span,
                detail: "expected expression".to_string(),
            }),
        }
    }
}

fn resolve_func(name: &str, span: Span) -> Result<Func, ParseError> {
    match name.to_ascii_lowercase().as_str() {
        "log2" => Ok(Func::Log2),
        "log10" => Ok(Func::Log10),
        "ln" => Ok(Func::Ln),
        "exp" => Ok(Func::Exp),
        "sqrt" => Ok(Func::Sqrt),
        "min" => Ok(Func::Min),
        "max" => Ok(Func::Max),
        "floor" => Ok(Func::Floor),
        "ceil" => Ok(Func::Ceil),
        "abs" => Ok(Func::Abs),
        _ => Err(ParseError::UnknownFunction {
            name: name.to_string(),
            span,
        }),
    }
}

impl Expr {
    /// Parse an expression from a string.
    pub fn parse(input: &str) -> Result<Expr, ParseError> {
        if input.trim().is_empty() {
            return Err(ParseError::UnexpectedEof);
        }
        let mut parser = Parser::new(input)?;
        let expr = parser.parse_expr(0)?;
        if !matches!(parser.peek().token, Token::Eof) {
            return Err(ParseError::TrailingInput {
                span: parser.peek().span,
            });
        }
        Ok(expr)
    }
}

// ── Display ──

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_expr(self, f, 0)
    }
}

/// Display with parent context to decide parenthesization.
/// `parent_bp` is the binding power of the parent context.
fn display_expr(expr: &Expr, f: &mut fmt::Formatter<'_>, parent_bp: u8) -> fmt::Result {
    match expr {
        Expr::Num(v) => {
            let rounded = v.round() as i64;
            if (*v - rounded as f64).abs() < 1e-10 && v.is_finite() {
                write!(f, "{rounded}")
            } else {
                write!(f, "{v}")
            }
        }
        Expr::Var(name) => write!(f, "{name}"),
        Expr::Neg(inner) => {
            write!(f, "-")?;
            // Wrap compound inner expressions
            let needs_parens = matches!(inner.as_ref(), Expr::BinOp { .. });
            if needs_parens {
                write!(f, "(")?;
                display_expr(inner, f, 0)?;
                write!(f, ")")
            } else {
                display_expr(inner, f, 5)
            }
        }
        Expr::BinOp { op, lhs, rhs } => {
            let (l_bp, r_bp) = match op {
                BinOp::Add | BinOp::Sub => (1, 2),
                BinOp::Mul | BinOp::Div => (3, 4),
                BinOp::Pow => (7, 6),
            };

            let needs_parens = l_bp < parent_bp;
            if needs_parens {
                write!(f, "(")?;
            }

            display_expr(lhs, f, l_bp)?;
            let op_str = match op {
                BinOp::Add => " + ",
                BinOp::Sub => " - ",
                BinOp::Mul => " * ",
                BinOp::Div => " / ",
                BinOp::Pow => " ^ ",
            };
            write!(f, "{op_str}")?;
            display_expr(rhs, f, r_bp)?;

            if needs_parens {
                write!(f, ")")?;
            }
            Ok(())
        }
        Expr::Call { func, args } => {
            let name = match func {
                Func::Log2 => "log2",
                Func::Log10 => "log10",
                Func::Ln => "ln",
                Func::Exp => "exp",
                Func::Sqrt => "sqrt",
                Func::Min => "min",
                Func::Max => "max",
                Func::Floor => "floor",
                Func::Ceil => "ceil",
                Func::Abs => "abs",
            };
            write!(f, "{name}(")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                display_expr(arg, f, 0)?;
            }
            write!(f, ")")
        }
    }
}

// ── Serde ──

impl serde::Serialize for Expr {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Expr {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Expr::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[path = "unit_tests/expr.rs"]
mod tests;
