//! Lossless symbolic expressions shared by the runtime library and proc macros.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

/// A validated problem-size variable name.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
#[serde(transparent)]
pub struct Symbol(Box<str>);

impl Symbol {
    pub fn new(name: impl Into<Box<str>>) -> Result<Self, InvalidSymbol> {
        let name = name.into();
        if is_valid_symbol(&name) {
            Ok(Self(name))
        } else {
            Err(InvalidSymbol(name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Symbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = Box::<str>::deserialize(deserializer)?;
        Self::new(name).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("invalid expression variable name {0:?}")]
pub struct InvalidSymbol(Box<str>);

fn is_valid_symbol(name: &str) -> bool {
    let mut bytes = name.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == b'_')
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        || name == "_"
    {
        return false;
    }
    !matches!(
        name,
        "abstract"
            | "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "gen"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "union"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}

/// One immutable node in a symbolic expression DAG.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExprNode {
    Const(BigRational),
    Var(Symbol),
    Add(Box<[Expr]>),
    Mul(Box<[Expr]>),
    Pow(Expr, Expr),
    Exp(Expr),
    Log(Expr),
    Factorial(Expr),
}

/// A cheap, immutable handle to a shared symbolic expression node.
#[derive(Clone, Debug)]
pub struct Expr(Arc<ExprNode>);

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || self.node() == other.node()
    }
}

impl Eq for Expr {}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.node(), state);
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if Arc::ptr_eq(&self.0, &other.0) {
            std::cmp::Ordering::Equal
        } else {
            self.node().cmp(other.node())
        }
    }
}

/// Opaque identity used to memoize one traversal of an expression DAG.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExprNodeId(usize);

impl serde::Serialize for Expr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ExprDocument::from_expression(self).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Expr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ExprDocument::deserialize(deserializer)?
            .into_expression()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ExprDocument {
    nodes: Vec<SerializedNode>,
    root: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
enum SerializedNode {
    Const(BigRational),
    Var(Symbol),
    Add(Vec<usize>),
    Mul(Vec<usize>),
    Pow(usize, usize),
    Exp(usize),
    Log(usize),
    Factorial(usize),
}

impl ExprDocument {
    fn from_expression(root: &Expr) -> Self {
        let mut ids = HashMap::new();
        let mut nodes = Vec::new();
        let mut pending = vec![(root, false)];
        while let Some((expression, expanded)) = pending.pop() {
            if ids.contains_key(&expression.node_identity()) {
                continue;
            }
            if !expanded {
                pending.push((expression, true));
                match expression.node() {
                    ExprNode::Add(values) | ExprNode::Mul(values) => {
                        pending.extend(values.iter().rev().map(|value| (value, false)));
                    }
                    ExprNode::Pow(base, exponent) => {
                        pending.push((exponent, false));
                        pending.push((base, false));
                    }
                    ExprNode::Exp(value) | ExprNode::Log(value) | ExprNode::Factorial(value) => {
                        pending.push((value, false))
                    }
                    ExprNode::Const(_) | ExprNode::Var(_) => {}
                }
                continue;
            }

            let child_id = |child: &Expr| ids[&child.node_identity()];
            let node = match expression.node() {
                ExprNode::Const(value) => SerializedNode::Const(value.clone()),
                ExprNode::Var(symbol) => SerializedNode::Var(symbol.clone()),
                ExprNode::Add(values) => SerializedNode::Add(values.iter().map(child_id).collect()),
                ExprNode::Mul(values) => SerializedNode::Mul(values.iter().map(child_id).collect()),
                ExprNode::Pow(base, exponent) => {
                    SerializedNode::Pow(child_id(base), child_id(exponent))
                }
                ExprNode::Exp(value) => SerializedNode::Exp(child_id(value)),
                ExprNode::Log(value) => SerializedNode::Log(child_id(value)),
                ExprNode::Factorial(value) => SerializedNode::Factorial(child_id(value)),
            };
            let id = nodes.len();
            nodes.push(node);
            ids.insert(expression.node_identity(), id);
        }
        Self {
            nodes,
            root: ids[&root.node_identity()],
        }
    }

    fn into_expression(self) -> Result<Expr, InvalidExpressionDocument> {
        let mut expressions = Vec::with_capacity(self.nodes.len());
        for (node_id, node) in self.nodes.into_iter().enumerate() {
            let child = |id: usize| {
                expressions
                    .get(id)
                    .cloned()
                    .ok_or(InvalidExpressionDocument::UnavailableChild { node_id, id })
            };
            let expression = match node {
                SerializedNode::Const(value) => Expr::constant(value),
                SerializedNode::Var(symbol) => Expr::from_node(ExprNode::Var(symbol)),
                SerializedNode::Add(ids) => {
                    Expr::add_all(ids.into_iter().map(child).collect::<Result<_, _>>()?)
                }
                SerializedNode::Mul(ids) => {
                    Expr::mul_all(ids.into_iter().map(child).collect::<Result<_, _>>()?)
                }
                SerializedNode::Pow(base, exponent) => Expr::pow(child(base)?, child(exponent)?),
                SerializedNode::Exp(value) => Expr::exp(child(value)?),
                SerializedNode::Log(value) => Expr::log(child(value)?),
                SerializedNode::Factorial(value) => Expr::factorial(child(value)?),
            };
            expressions.push(expression);
        }
        expressions
            .get(self.root)
            .cloned()
            .ok_or(InvalidExpressionDocument::UnavailableRoot(self.root))
    }
}

#[derive(Debug, thiserror::Error)]
enum InvalidExpressionDocument {
    #[error("expression node {node_id} references unavailable child node {id}")]
    UnavailableChild { node_id: usize, id: usize },
    #[error("expression root references unavailable node {0}")]
    UnavailableRoot(usize),
}

impl Expr {
    fn from_node(node: ExprNode) -> Self {
        Self(Arc::new(node))
    }

    pub fn node(&self) -> &ExprNode {
        &self.0
    }

    /// Identity of this allocation for operation-local DAG memoization.
    /// The value is process-local and remains valid while any clone of the node lives.
    pub fn node_identity(&self) -> ExprNodeId {
        ExprNodeId(Arc::as_ptr(&self.0) as usize)
    }

    pub fn integer(value: impl Into<BigInt>) -> Self {
        Self::constant(BigRational::from_integer(value.into()))
    }

    pub fn rational(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>) -> Self {
        Self::constant(BigRational::new(numerator.into(), denominator.into()))
    }

    pub fn constant(value: BigRational) -> Self {
        Self::from_node(ExprNode::Const(value))
    }

    pub fn variable(name: impl Into<Box<str>>) -> Self {
        Self::try_variable(name).unwrap_or_else(|error| panic!("{error}"))
    }

    pub fn try_variable(name: impl Into<Box<str>>) -> Result<Self, InvalidSymbol> {
        Symbol::new(name).map(|symbol| Self::from_node(ExprNode::Var(symbol)))
    }

    pub fn pow(base: Expr, exponent: Expr) -> Self {
        if exponent.is_exact_integer(0) || base.is_exact_integer(1) {
            return Self::integer(1);
        }
        if exponent.is_exact_integer(1) {
            return base;
        }
        Self::from_node(ExprNode::Pow(base, exponent))
    }

    pub fn exp(value: Expr) -> Self {
        Self::from_node(ExprNode::Exp(value))
    }

    pub fn log(value: Expr) -> Self {
        Self::from_node(ExprNode::Log(value))
    }

    pub fn sqrt(value: Expr) -> Self {
        Self::pow(value, Self::rational(1, 2))
    }

    pub fn factorial(value: Expr) -> Self {
        Self::from_node(ExprNode::Factorial(value))
    }

    pub fn parse(input: &str) -> Self {
        Self::try_parse(input)
            .unwrap_or_else(|error| panic!("failed to parse expression {input:?}: {error}"))
    }

    pub fn try_parse(input: &str) -> Result<Self, ParseError> {
        Parser::new(tokenize(input)?).parse()
    }

    pub fn variables(&self) -> BTreeSet<&str> {
        let mut variables = BTreeSet::new();
        let mut visited = HashSet::new();
        self.collect_variables(&mut variables, &mut visited);
        variables
    }

    fn collect_variables<'a>(
        &'a self,
        variables: &mut BTreeSet<&'a str>,
        visited: &mut HashSet<ExprNodeId>,
    ) {
        if !visited.insert(self.node_identity()) {
            return;
        }
        match self.node() {
            ExprNode::Const(_) => {}
            ExprNode::Var(name) => {
                variables.insert(name.as_str());
            }
            ExprNode::Add(values) | ExprNode::Mul(values) => {
                for value in values {
                    value.collect_variables(variables, visited);
                }
            }
            ExprNode::Pow(base, exponent) => {
                base.collect_variables(variables, visited);
                exponent.collect_variables(variables, visited);
            }
            ExprNode::Exp(value) | ExprNode::Log(value) | ExprNode::Factorial(value) => {
                value.collect_variables(variables, visited);
            }
        }
    }

    /// Replace every variable or report the complete set of missing replacements.
    pub fn substitute_complete(
        &self,
        replacements: &HashMap<&str, &Expr>,
    ) -> Result<Expr, SubstitutionError> {
        self.substitute_inner(replacements, &mut HashMap::new())
            .map_err(SubstitutionError::new)
    }

    fn substitute_inner(
        &self,
        replacements: &HashMap<&str, &Expr>,
        memo: &mut HashMap<ExprNodeId, Result<Expr, BTreeSet<Box<str>>>>,
    ) -> Result<Expr, BTreeSet<Box<str>>> {
        let identity = self.node_identity();
        if let Some(result) = memo.get(&identity) {
            return result.clone();
        }
        let result = match self.node() {
            ExprNode::Const(_) => Ok(self.clone()),
            ExprNode::Var(name) => match replacements.get(name.as_ref()) {
                Some(replacement) => Ok((*replacement).clone()),
                None => Err(BTreeSet::from([name.as_str().into()])),
            },
            ExprNode::Add(values) => {
                Self::substitute_values(values, replacements, memo).map(Self::add_all)
            }
            ExprNode::Mul(values) => {
                Self::substitute_values(values, replacements, memo).map(Self::mul_all)
            }
            ExprNode::Pow(base, exponent) => {
                let base = base.substitute_inner(replacements, memo);
                let exponent = exponent.substitute_inner(replacements, memo);
                match (base, exponent) {
                    (Ok(base), Ok(exponent)) => Ok(Self::pow(base, exponent)),
                    (Err(mut left), Err(right)) => {
                        left.extend(right);
                        Err(left)
                    }
                    (Err(missing), _) | (_, Err(missing)) => Err(missing),
                }
            }
            ExprNode::Exp(value) => value.substitute_inner(replacements, memo).map(Self::exp),
            ExprNode::Log(value) => value.substitute_inner(replacements, memo).map(Self::log),
            ExprNode::Factorial(value) => value
                .substitute_inner(replacements, memo)
                .map(Self::factorial),
        };
        memo.insert(identity, result.clone());
        result
    }

    fn substitute_values(
        values: &[Expr],
        replacements: &HashMap<&str, &Expr>,
        memo: &mut HashMap<ExprNodeId, Result<Expr, BTreeSet<Box<str>>>>,
    ) -> Result<Vec<Expr>, BTreeSet<Box<str>>> {
        let mut substituted = Vec::with_capacity(values.len());
        let mut missing = BTreeSet::new();
        for value in values {
            match value.substitute_inner(replacements, memo) {
                Ok(value) => substituted.push(value),
                Err(variables) => missing.extend(variables),
            }
        }
        if missing.is_empty() {
            Ok(substituted)
        } else {
            Err(missing)
        }
    }

    pub fn is_constant(&self) -> bool {
        self.is_constant_inner(&mut HashMap::new())
    }

    fn is_constant_inner(&self, memo: &mut HashMap<ExprNodeId, bool>) -> bool {
        if let Some(result) = memo.get(&self.node_identity()) {
            return *result;
        }
        let result = match self.node() {
            ExprNode::Const(_) => true,
            ExprNode::Var(_) => false,
            ExprNode::Add(values) | ExprNode::Mul(values) => {
                values.iter().all(|value| value.is_constant_inner(memo))
            }
            ExprNode::Pow(base, exponent) => {
                base.is_constant_inner(memo) && exponent.is_constant_inner(memo)
            }
            ExprNode::Exp(value) | ExprNode::Log(value) | ExprNode::Factorial(value) => {
                value.is_constant_inner(memo)
            }
        };
        memo.insert(self.node_identity(), result);
        result
    }

    pub fn is_polynomial(&self) -> bool {
        self.is_polynomial_inner(&mut HashMap::new())
    }

    fn is_polynomial_inner(&self, polynomial_memo: &mut HashMap<ExprNodeId, bool>) -> bool {
        if let Some(result) = polynomial_memo.get(&self.node_identity()) {
            return *result;
        }
        let result = match self.node() {
            ExprNode::Const(_) | ExprNode::Var(_) => true,
            ExprNode::Add(values) | ExprNode::Mul(values) => values
                .iter()
                .all(|value| value.is_polynomial_inner(polynomial_memo)),
            ExprNode::Pow(base, exponent) => {
                (matches!((base.node(), exponent.node()),
                    (ExprNode::Const(base), ExprNode::Const(exponent))
                        if exponent.is_integer()
                            && (!exponent.is_negative() || !base.is_zero())))
                    || (base.is_polynomial_inner(polynomial_memo)
                        && matches!(exponent.node(), ExprNode::Const(value) if value.is_integer() && !value.is_negative()))
            }
            ExprNode::Exp(_) | ExprNode::Log(_) | ExprNode::Factorial(_) => false,
        };
        polynomial_memo.insert(self.node_identity(), result);
        result
    }

    pub fn is_valid_complexity_notation(&self) -> bool {
        self.complexity_notation_analysis(&mut HashMap::new()).1
    }

    fn complexity_notation_analysis(
        &self,
        memo: &mut HashMap<ExprNodeId, (bool, bool)>,
    ) -> (bool, bool) {
        if let Some(analysis) = memo.get(&self.node_identity()) {
            return *analysis;
        }
        let analysis = match self.node() {
            ExprNode::Const(value) => (true, value.is_one()),
            ExprNode::Var(_) => (false, true),
            ExprNode::Add(values) | ExprNode::Mul(values) => {
                let mut all_constant = true;
                let mut all_valid_nonconstant = true;
                for value in values {
                    let (constant, valid) = value.complexity_notation_analysis(memo);
                    all_constant &= constant;
                    all_valid_nonconstant &= !constant && valid;
                }
                (all_constant, all_valid_nonconstant)
            }
            ExprNode::Pow(base, exponent) => {
                let base_analysis = base.complexity_notation_analysis(memo);
                let exponent_analysis = exponent.complexity_notation_analysis(memo);
                let base_valid = match base.node() {
                    ExprNode::Const(value) => value.is_positive(),
                    _ => base_analysis.1,
                };
                (
                    base_analysis.0 && exponent_analysis.0,
                    base_valid && (exponent_analysis.0 || exponent_analysis.1),
                )
            }
            ExprNode::Exp(value) | ExprNode::Log(value) | ExprNode::Factorial(value) => {
                value.complexity_notation_analysis(memo)
            }
        };
        memo.insert(self.node_identity(), analysis);
        analysis
    }

    pub fn unique_node_count(&self) -> usize {
        let mut visited = HashSet::new();
        let mut pending = vec![self];
        while let Some(expression) = pending.pop() {
            if !visited.insert(expression.node_identity()) {
                continue;
            }
            match expression.node() {
                ExprNode::Add(values) | ExprNode::Mul(values) => pending.extend(values),
                ExprNode::Pow(base, exponent) => {
                    pending.push(base);
                    pending.push(exponent);
                }
                ExprNode::Exp(value) | ExprNode::Log(value) | ExprNode::Factorial(value) => {
                    pending.push(value);
                }
                ExprNode::Const(_) | ExprNode::Var(_) => {}
            }
        }
        visited.len()
    }

    fn is_exact_integer(&self, expected: i64) -> bool {
        matches!(self.node(), ExprNode::Const(value) if *value == BigRational::from_integer(expected.into()))
    }

    fn add_all(values: Vec<Expr>) -> Expr {
        let mut constant = BigRational::zero();
        let mut coefficients: std::collections::BTreeMap<Expr, BigRational> =
            std::collections::BTreeMap::new();
        let mut pending = values;
        while let Some(value) = pending.pop() {
            match value.node() {
                ExprNode::Add(nested) => pending.extend(nested.iter().cloned()),
                ExprNode::Const(value) => constant += value,
                ExprNode::Mul(factors)
                    if matches!(factors.first().map(Expr::node), Some(ExprNode::Const(_))) =>
                {
                    let ExprNode::Const(coefficient) = factors[0].node() else {
                        unreachable!()
                    };
                    let base = Self::mul_all(factors[1..].to_vec());
                    *coefficients.entry(base).or_insert_with(BigRational::zero) += coefficient;
                }
                _ => {
                    *coefficients.entry(value).or_insert_with(BigRational::zero) +=
                        BigRational::one();
                }
            }
        }
        let mut terms = Vec::with_capacity(coefficients.len() + usize::from(!constant.is_zero()));
        for (base, coefficient) in coefficients {
            if coefficient.is_zero() {
                continue;
            }
            if coefficient.is_one() {
                terms.push(base);
            } else {
                terms.push(Self::mul_all(vec![Self::constant(coefficient), base]));
            }
        }
        if !constant.is_zero() {
            terms.push(Self::constant(constant));
        }
        terms.sort();
        match terms.len() {
            0 => Self::integer(0),
            1 => terms.pop().expect("single normalized sum term"),
            _ => Self::from_node(ExprNode::Add(terms.into_boxed_slice())),
        }
    }

    fn mul_all(values: Vec<Expr>) -> Expr {
        let mut constant = BigRational::one();
        let mut powers: std::collections::BTreeMap<Expr, Vec<Expr>> =
            std::collections::BTreeMap::new();
        let mut pending = values;
        while let Some(value) = pending.pop() {
            match value.node() {
                ExprNode::Mul(nested) => pending.extend(nested.iter().cloned()),
                ExprNode::Const(value) => constant *= value,
                ExprNode::Pow(base, exponent) => {
                    powers
                        .entry(base.clone())
                        .or_default()
                        .push(exponent.clone());
                }
                _ => powers.entry(value).or_default().push(Self::integer(1)),
            }
        }
        let mut factors = Vec::with_capacity(powers.len() + usize::from(!constant.is_one()));
        for (base, exponents) in powers {
            factors.push(Self::pow(base, Self::add_all(exponents)));
        }
        if !constant.is_one() {
            factors.push(Self::constant(constant));
        }
        factors.sort();
        match factors.len() {
            0 => Self::integer(1),
            1 => factors.pop().expect("single normalized product factor"),
            _ => Self::from_node(ExprNode::Mul(factors.into_boxed_slice())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubstitutionError {
    missing: BTreeSet<Box<str>>,
}

impl SubstitutionError {
    fn new(missing: BTreeSet<Box<str>>) -> Self {
        Self { missing }
    }

    pub fn missing_variables(&self) -> impl Iterator<Item = &str> {
        self.missing.iter().map(AsRef::as_ref)
    }
}

impl fmt::Display for SubstitutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "missing substitutions for {}",
            self.missing_variables().collect::<Vec<_>>().join(", ")
        )
    }
}

impl std::error::Error for SubstitutionError {}

impl std::ops::Add for Expr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::add_all(vec![self, rhs])
    }
}

impl std::ops::Sub for Expr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::add_all(vec![self, -rhs])
    }
}

impl std::ops::Mul for Expr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul_all(vec![self, rhs])
    }
}

impl std::ops::Div for Expr {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::mul_all(vec![self, Self::pow(rhs, Self::integer(-1))])
    }
}

impl std::ops::Neg for Expr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::mul_all(vec![Self::integer(-1), self])
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_precedence(formatter, 0, false)
    }
}

impl Expr {
    fn precedence(&self) -> u8 {
        match self.node() {
            ExprNode::Add(_) => 1,
            ExprNode::Mul(_) => 2,
            ExprNode::Pow(_, _) => 4,
            _ => 5,
        }
    }

    fn fmt_with_precedence(
        &self,
        formatter: &mut fmt::Formatter<'_>,
        parent_precedence: u8,
        right_child: bool,
    ) -> fmt::Result {
        let precedence = self.precedence();
        let needs_parentheses = precedence < parent_precedence
            || (right_child
                && precedence == parent_precedence
                && matches!(self.node(), ExprNode::Add(_) | ExprNode::Mul(_)))
            || (!right_child
                && precedence == parent_precedence
                && matches!(self.node(), ExprNode::Pow(_, _)));
        if needs_parentheses {
            write!(formatter, "(")?;
        }
        match self.node() {
            ExprNode::Const(value) => fmt_rational(value, formatter)?,
            ExprNode::Var(name) => write!(formatter, "{name}")?,
            ExprNode::Add(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, " + ")?;
                    }
                    value.fmt_with_precedence(formatter, precedence, index > 0)?;
                }
            }
            ExprNode::Mul(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, " * ")?;
                    }
                    value.fmt_with_precedence(formatter, precedence, index > 0)?;
                }
            }
            ExprNode::Pow(base, exponent) => {
                base.fmt_with_precedence(formatter, precedence, false)?;
                write!(formatter, "^")?;
                exponent.fmt_with_precedence(formatter, precedence, true)?;
            }
            ExprNode::Exp(value) => write!(formatter, "exp({value})")?,
            ExprNode::Log(value) => write!(formatter, "log({value})")?,
            ExprNode::Factorial(value) => write!(formatter, "factorial({value})")?,
        }
        if needs_parentheses {
            write!(formatter, ")")?;
        }
        Ok(())
    }
}

fn fmt_rational(value: &BigRational, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if value.is_integer() {
        return write!(formatter, "{}", value.to_integer());
    }
    let negative = value.is_negative();
    let numerator = value.numer().abs();
    let mut denominator = value.denom().clone();
    let mut twos = 0usize;
    let mut fives = 0usize;
    while (&denominator % 2u8).is_zero() {
        denominator /= 2u8;
        twos += 1;
    }
    while (&denominator % 5u8).is_zero() {
        denominator /= 5u8;
        fives += 1;
    }
    if !denominator.is_one() {
        return write!(formatter, "{}/{}", value.numer(), value.denom());
    }
    let scale = twos.max(fives);
    let scaled = numerator
        * BigInt::from(2u8).pow((scale - twos) as u32)
        * BigInt::from(5u8).pow((scale - fives) as u32);
    let digits = scaled.to_string();
    let sign = if negative { "-" } else { "" };
    if digits.len() <= scale {
        write!(formatter, "{sign}0.{:0>width$}", digits, width = scale)
    } else {
        let split = digits.len() - scale;
        write!(formatter, "{sign}{}.{}", &digits[..split], &digits[split..])
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{message} at byte {position}")]
pub struct ParseError {
    position: usize,
    message: String,
}

impl ParseError {
    fn new(position: usize, message: impl Into<String>) -> Self {
        Self {
            position,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    position: usize,
    kind: TokenKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenKind {
    Number(BigRational),
    Ident(Box<str>),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut position = 0;
    while position < bytes.len() {
        match bytes[position] {
            b' ' | b'\t' | b'\n' | b'\r' => position += 1,
            b'+' => push_token(&mut tokens, &mut position, TokenKind::Plus),
            b'-' => push_token(&mut tokens, &mut position, TokenKind::Minus),
            b'*' => push_token(&mut tokens, &mut position, TokenKind::Star),
            b'/' => push_token(&mut tokens, &mut position, TokenKind::Slash),
            b'^' => push_token(&mut tokens, &mut position, TokenKind::Caret),
            b'(' => push_token(&mut tokens, &mut position, TokenKind::LeftParen),
            b')' => push_token(&mut tokens, &mut position, TokenKind::RightParen),
            byte if byte.is_ascii_digit() || byte == b'.' => {
                let start = position;
                while position < bytes.len()
                    && (bytes[position].is_ascii_digit() || bytes[position] == b'.')
                {
                    position += 1;
                }
                let spelling = &input[start..position];
                let value = parse_decimal(spelling).ok_or_else(|| {
                    ParseError::new(start, format!("invalid number {spelling:?}"))
                })?;
                tokens.push(Token {
                    position: start,
                    kind: TokenKind::Number(value),
                });
            }
            byte if byte.is_ascii_alphabetic() || byte == b'_' => {
                let start = position;
                while position < bytes.len()
                    && (bytes[position].is_ascii_alphanumeric() || bytes[position] == b'_')
                {
                    position += 1;
                }
                tokens.push(Token {
                    position: start,
                    kind: TokenKind::Ident(input[start..position].into()),
                });
            }
            _ => {
                let character = input[position..].chars().next().unwrap();
                return Err(ParseError::new(
                    position,
                    format!("unexpected character {character:?}"),
                ));
            }
        }
    }
    Ok(tokens)
}

fn push_token(tokens: &mut Vec<Token>, position: &mut usize, kind: TokenKind) {
    tokens.push(Token {
        position: *position,
        kind,
    });
    *position += 1;
}

fn parse_decimal(spelling: &str) -> Option<BigRational> {
    let mut parts = spelling.split('.');
    let integer = parts.next()?;
    let fractional = parts.next();
    if parts.next().is_some() || (integer.is_empty() && fractional.is_none()) {
        return None;
    }
    match fractional {
        None => BigInt::from_str(integer)
            .ok()
            .map(BigRational::from_integer),
        Some(fractional) if !integer.is_empty() || !fractional.is_empty() => {
            let combined = format!("{integer}{fractional}");
            let numerator = BigInt::from_str(&combined).ok()?;
            let denominator = BigInt::from(10u8).pow(fractional.len() as u32);
            Some(BigRational::new(numerator, denominator))
        }
        Some(_) => None,
    }
}

struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
    end_position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let end_position = tokens.last().map_or(0, |token| token.position + 1);
        Self {
            tokens: tokens.into_iter().peekable(),
            end_position,
        }
    }

    fn parse(mut self) -> Result<Expr, ParseError> {
        if self.tokens.peek().is_none() {
            return Err(ParseError::new(0, "expected expression"));
        }
        let expression = self.parse_additive()?;
        if let Some(token) = self.peek() {
            return Err(ParseError::new(token.position, "unexpected trailing token"));
        }
        Ok(expression)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn consume(&mut self, kind: &TokenKind) -> bool {
        if self.peek().is_some_and(|token| &token.kind == kind) {
            self.tokens.next();
            true
        } else {
            false
        }
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.parse_multiplicative()?;
        loop {
            if self.consume(&TokenKind::Plus) {
                expression = expression + self.parse_multiplicative()?;
            } else if self.consume(&TokenKind::Minus) {
                expression = expression - self.parse_multiplicative()?;
            } else {
                return Ok(expression);
            }
        }
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.parse_unary()?;
        loop {
            if self.consume(&TokenKind::Star) {
                expression = expression * self.parse_unary()?;
            } else if self
                .peek()
                .is_some_and(|token| token.kind == TokenKind::Slash)
            {
                let position = self.advance().expect("peeked division token").position;
                let denominator = self.parse_unary()?;
                if denominator.is_exact_integer(0) {
                    return Err(ParseError::new(position, "division by zero"));
                }
                expression = expression / denominator;
            } else {
                return Ok(expression);
            }
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.consume(&TokenKind::Minus) {
            Ok(-self.parse_unary()?)
        } else {
            self.parse_power()
        }
    }

    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let base = self.parse_primary()?;
        if self
            .peek()
            .is_some_and(|token| token.kind == TokenKind::Caret)
        {
            let position = self.advance().expect("peeked power token").position;
            let exponent = self.parse_unary()?;
            if matches!((base.node(), exponent.node()),
                (ExprNode::Const(base), ExprNode::Const(exponent))
                    if base.is_zero() && exponent.is_negative())
            {
                return Err(ParseError::new(
                    position,
                    "zero cannot have a negative power",
                ));
            }
            Ok(Expr::pow(base, exponent))
        } else {
            Ok(base)
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self
            .advance()
            .ok_or_else(|| ParseError::new(self.end_position(), "expected expression"))?;
        match token.kind {
            TokenKind::Number(value) => Ok(Expr::constant(value)),
            TokenKind::Ident(name) => {
                if !self.consume(&TokenKind::LeftParen) {
                    return Expr::try_variable(name)
                        .map_err(|error| ParseError::new(token.position, error.to_string()));
                }
                let argument = self.parse_additive()?;
                self.expect_right_paren()?;
                match name.as_ref() {
                    "exp" => Ok(Expr::exp(argument)),
                    "log" => {
                        if matches!(argument.node(), ExprNode::Const(value) if !value.is_positive())
                        {
                            Err(ParseError::new(
                                token.position,
                                "logarithm argument must be positive",
                            ))
                        } else {
                            Ok(Expr::log(argument))
                        }
                    }
                    "sqrt" => {
                        if matches!(argument.node(), ExprNode::Const(value) if value.is_negative())
                        {
                            Err(ParseError::new(
                                token.position,
                                "square-root argument must be non-negative",
                            ))
                        } else {
                            Ok(Expr::sqrt(argument))
                        }
                    }
                    "factorial" => {
                        if matches!(argument.node(), ExprNode::Const(value)
                            if !value.is_integer() || value.is_negative())
                        {
                            Err(ParseError::new(
                                token.position,
                                "factorial argument must be a non-negative integer",
                            ))
                        } else {
                            Ok(Expr::factorial(argument))
                        }
                    }
                    _ => Err(ParseError::new(
                        token.position,
                        format!("unknown function {name:?}"),
                    )),
                }
            }
            TokenKind::LeftParen => {
                let expression = self.parse_additive()?;
                self.expect_right_paren()?;
                Ok(expression)
            }
            _ => Err(ParseError::new(token.position, "expected expression")),
        }
    }

    fn expect_right_paren(&mut self) -> Result<(), ParseError> {
        if self.consume(&TokenKind::RightParen) {
            Ok(())
        } else {
            Err(ParseError::new(
                self.end_position(),
                "expected closing parenthesis",
            ))
        }
    }

    fn end_position(&self) -> usize {
        self.end_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_literals_are_exact() {
        assert_eq!(Expr::parse("2.372"), Expr::rational(593, 250));
    }

    #[test]
    fn parser_normalizes_source_operators() {
        let expression = Expr::parse("n * (n - 1) / 2 - m");
        assert!(matches!(expression.node(), ExprNode::Add(_)));
        assert_eq!(expression.variables(), BTreeSet::from(["m", "n"]));
    }

    #[test]
    fn parser_rejects_statically_undefined_expressions() {
        for source in [
            "0 / 0",
            "0^-1",
            "log(0)",
            "log(-1)",
            "sqrt(-1)",
            "factorial(-1)",
            "factorial(3.5)",
        ] {
            assert!(Expr::try_parse(source).is_err(), "accepted {source}");
        }
    }

    #[test]
    fn variables_are_owned() {
        let name = String::from("dynamic_size");
        let expression = Expr::parse(&name);
        drop(name);
        assert_eq!(expression.variables(), BTreeSet::from(["dynamic_size"]));
    }

    #[test]
    fn variables_enforce_one_identifier_grammar() {
        for invalid in ["", "_", "1n", "n-m", "type"] {
            assert!(Expr::try_variable(invalid).is_err(), "accepted {invalid:?}");
        }
        for invalid_expression in ["", "_", "1n", "type"] {
            assert!(
                Expr::try_parse(invalid_expression).is_err(),
                "parsed {invalid_expression:?}"
            );
        }
        assert!(matches!(Expr::parse("n-m").node(), ExprNode::Add(_)));
        for valid in ["n", "_n", "n_1", "num_vertices"] {
            let expression = Expr::try_variable(valid).unwrap();
            assert_eq!(
                Expr::try_parse(&expression.to_string()).unwrap(),
                expression
            );
        }
    }

    #[test]
    fn deserialization_rejects_invalid_variable_names() {
        assert!(serde_json::from_str::<Expr>(r#"{"nodes":[{"Var":"n-m"}],"root":0}"#).is_err());
    }

    #[test]
    fn serialization_preserves_shared_nodes() {
        let shared = Expr::variable("a") + Expr::variable("b");
        let expression = Expr::pow(shared.clone(), shared);
        let encoded = serde_json::to_value(&expression).unwrap();
        assert_eq!(encoded["nodes"].as_array().unwrap().len(), 4);

        let decoded: Expr = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded, expression);
        assert_eq!(decoded.unique_node_count(), 4);
    }

    #[test]
    fn deserialization_rejects_forward_node_references() {
        let error =
            serde_json::from_str::<Expr>(r#"{"nodes":[{"Pow":[1,1]},{"Var":"n"}],"root":0}"#)
                .unwrap_err();
        assert!(error.to_string().contains("unavailable child node 1"));
    }

    #[test]
    fn complete_substitution_rejects_missing_variables() {
        let expression = Expr::parse("n + m");
        let n = Expr::integer(3);
        let replacements = HashMap::from([("n", &n)]);
        let error = expression.substitute_complete(&replacements).unwrap_err();
        assert_eq!(error.missing_variables().collect::<Vec<_>>(), ["m"]);

        let m = Expr::integer(4);
        let replacements = HashMap::from([("n", &n), ("m", &m)]);
        assert_eq!(
            expression.substitute_complete(&replacements),
            Ok(Expr::integer(3) + Expr::integer(4))
        );
    }

    #[test]
    fn polynomial_accepts_exact_rational_coefficients() {
        assert!(Expr::parse("-n / 2").is_polynomial());
        assert!(
            !(Expr::variable("n") * Expr::pow(Expr::integer(0), Expr::integer(-1))).is_polynomial()
        );
        assert!(!Expr::parse("n / m").is_polynomial());
    }

    #[test]
    fn exponentiation_precedes_unary_minus() {
        assert_eq!(
            Expr::parse("-n^2"),
            -Expr::pow(Expr::variable("n"), Expr::integer(2))
        );
        assert_eq!(
            Expr::parse("2^-3"),
            Expr::pow(Expr::integer(2), -Expr::integer(3))
        );
    }

    #[test]
    fn display_preserves_grouping() {
        let expression = Expr::parse("n * (n - 1) / 2 - m");
        assert_eq!(expression.to_string(), "-1 * m + n * (-1 + n) * 2^-1");
        assert_eq!(Expr::parse(&expression.to_string()), expression);
    }

    #[test]
    fn repeated_substitution_keeps_a_constant_number_of_nodes() {
        let template = Expr::parse("x + x");
        let mut expression = Expr::variable("n");
        for _ in 0..100 {
            let replacements = HashMap::from([("x", &expression)]);
            expression = template
                .substitute_complete(&replacements)
                .expect("x has an exact replacement");
        }

        assert_eq!(expression.unique_node_count(), 3);
        assert_eq!(expression.variables(), BTreeSet::from(["n"]));
    }

    #[test]
    fn constructors_combine_coefficients_and_exponents() {
        assert_eq!(Expr::parse("2*x + 3*x"), Expr::parse("5*x"));
        assert_eq!(Expr::parse("x^2 * x^3"), Expr::parse("x^5"));
        assert_eq!(Expr::parse("x * x^-1"), Expr::integer(1));
    }

    #[test]
    fn canonicalization_preserves_deep_shared_subexpressions() {
        let mut expression = Expr::variable("n");
        for _ in 0..100 {
            expression = Expr::pow(expression.clone(), Expr::integer(2)) + expression;
        }

        assert_eq!(expression.unique_node_count(), 301);
    }

    #[test]
    fn serialization_preserves_every_operator() {
        let expression = Expr::parse("-factorial(n - 1) + exp(m) / log(sqrt(k))^2");
        let encoded = serde_json::to_string(&expression).unwrap();
        let decoded: Expr = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, expression);
    }

    #[test]
    fn display_does_not_normalize_half_power_to_sqrt() {
        let power = Expr::pow(Expr::variable("n"), Expr::rational(1, 2));
        assert_eq!(power.to_string(), "n^0.5");
        assert_eq!(Expr::parse(&power.to_string()), power);
    }

    #[test]
    fn shared_dag_queries_reuse_nodes_without_losing_errors() {
        let shared = Expr::variable("n") + Expr::variable("m");
        let expression = Expr::pow(shared.clone(), shared);

        assert_eq!(expression.variables(), BTreeSet::from(["m", "n"]));
        assert!(!expression.is_constant());
        assert!(!expression.is_polynomial());
        assert!(expression.is_valid_complexity_notation());
        assert_eq!(expression.unique_node_count(), 4);

        let error = expression.substitute_complete(&HashMap::new()).unwrap_err();
        assert_eq!(
            error.missing_variables().collect::<Vec<_>>(),
            vec!["m", "n"]
        );

        let mut expressions = HashSet::new();
        assert!(expressions.insert(expression.clone()));
        assert!(!expressions.insert(expression));
    }

    #[test]
    fn display_and_parser_cover_non_decimal_rationals() {
        assert_eq!(Expr::rational(1, 3).to_string(), "1/3");
        assert!(Expr::try_parse(".").is_err());
    }
}
