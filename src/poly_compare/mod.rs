//! Certified V1 generic polynomial comparison relative to a fixed path batch.
//! See `docs/src/polynomial-comparator.md` for semantics and limitations.
use crate::expr::{Expr, Symbol};
use crate::parameters::{ParameterRelation, ParameterTransform, ParameterTransformError};
use crate::polynomial::pow_rational;
pub use crate::polynomial::{Monomial, Poly};
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{BTreeMap, BTreeSet, HashMap};

mod lp;
mod render;

/// Why one comparison direction is unresolved; each is a missing proof obligation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reason {
    NotPolynomial(String),
    Unavailable(ParameterTransformError),
    MissingField,
    Validity,
    ResourceLimit(&'static str),
}

/// One target field of a path: its certified bound plus the nominal signed composition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldCertificate {
    /// Exact substitution of the registered signed formulas; audit data only.
    pub original_signed: Option<Poly>,
    /// The bound comparisons use, with its accuracy.
    pub certificate: Result<(Poly, ParameterRelation), Reason>,
}
impl FieldCertificate {
    pub fn new(expression: &Expr, relation: ParameterRelation) -> Self {
        let certificate = Poly::from_expr(expression)
            .map(|poly| (poly, relation))
            .ok_or_else(|| Reason::NotPolynomial(expression.to_string()));
        Self {
            original_signed: certificate.as_ref().ok().map(|(poly, _)| poly.clone()),
            certificate,
        }
    }
    fn poly(&self) -> Result<&Poly, Reason> {
        self.certificate
            .as_ref()
            .map(|(poly, _)| poly)
            .map_err(Clone::clone)
    }
}

/// Target fields of one path as functions of the source parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Profile {
    /// Full source box, including coordinates absent from every formula.
    pub domain: BTreeSet<Symbol>,
    pub fields: BTreeMap<String, FieldCertificate>,
}
impl Profile {
    /// Profile over the variables its certificates read.
    pub fn new(fields: BTreeMap<String, FieldCertificate>) -> Self {
        let domain = fields
            .values()
            .filter_map(|f| f.poly().ok())
            .flat_map(Poly::variables)
            .collect();
        Self { domain, fields }
    }

    /// Project a composed transform; composition itself is `ParameterTransform::compose`.
    pub fn from_transform(transform: &ParameterTransform) -> Self {
        let mut fields: BTreeMap<_, _> = transform
            .expressions()
            .map(|(name, expr)| {
                let relation = transform.relation(name).expect("listed field");
                (name.to_string(), FieldCertificate::new(expr, relation))
            })
            .collect();
        for (name, error) in transform.unavailable_fields() {
            fields.insert(
                name.to_string(),
                FieldCertificate {
                    original_signed: None,
                    certificate: Err(Reason::Unavailable(error.clone())),
                },
            );
        }
        Self::new(fields)
    }

    /// Path profile: `composed` supplies certificates; `steps` are substituted
    /// exactly, without hulls, to recover the nominal signed formulas.
    pub fn from_path(
        source_parameters: &[String],
        steps: &[ParameterTransform],
        composed: &ParameterTransform,
    ) -> Self {
        let mut values: BTreeMap<Symbol, Poly> = source_parameters
            .iter()
            .filter_map(|name| Symbol::new(name.as_str()).ok())
            .map(|symbol| (symbol.clone(), Poly::variable(symbol)))
            .collect();
        let mut profile = Self::from_transform(composed);
        profile.domain.extend(values.keys().cloned());
        for step in steps {
            values = step
                .expressions()
                .filter_map(|(name, expr)| {
                    let poly = Poly::from_expr(expr)?.substitute(&values)?;
                    Some((Symbol::new(name).ok()?, poly))
                })
                .collect();
        }
        for (name, field) in &mut profile.fields {
            field.original_signed = Symbol::new(name.as_str())
                .ok()
                .and_then(|symbol| values.remove(&symbol));
        }
        profile
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coverage {
    pub exponent: Monomial,
    pub weights: Vec<(Monomial, BigRational)>,
}
/// Re-verified proofs of the batch-relative generic order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenericProof {
    Identity,
    Zero,
    Newton(Vec<Coverage>),
}
impl GenericProof {
    pub fn verify(&self, f: &Poly, g: &Poly) -> bool {
        match self {
            Self::Identity => f == g,
            Self::Zero => f.is_zero(),
            Self::Newton(coverage) => verify_coverage(coverage, f, g),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Witness {
    pub curve: Curve,
    pub numerator: BTreeMap<BigUint, BigRational>,
    pub denominator: BTreeMap<BigUint, BigRational>,
}
type Curve = BTreeMap<Symbol, (BigUint, BigUint)>;
fn on_curve(poly: &Poly, curve: &Curve) -> Option<BTreeMap<BigUint, BigRational>> {
    let mut result = BTreeMap::new();
    for (m, c) in poly.terms() {
        let mut degree = BigUint::zero();
        let mut coefficient = c.clone();
        for (v, a) in m {
            let (scale, e) = curve.get(v)?;
            degree += a * e;
            coefficient *= pow_rational(
                BigRational::from_integer(BigInt::from(scale.clone())),
                &BigInt::from(a.clone()),
            );
        }
        *result.entry(degree).or_insert_with(BigRational::zero) += coefficient;
    }
    result.retain(|_, c| !c.is_zero());
    Some(result)
}
impl Witness {
    /// Evaluate both sides; accept only a regular-valid refutation across the batch.
    pub fn on(curve: Curve, f: &Poly, g: &Poly, batch: &[Poly]) -> Option<Self> {
        let w = Self {
            numerator: on_curve(f, &curve)?,
            denominator: on_curve(g, &curve)?,
            curve,
        };
        w.verify(f, g, batch).then_some(w)
    }
    /// Verify exact substitutions and every original top face, before cancellation.
    pub fn verify(&self, f: &Poly, g: &Poly, batch: &[Poly]) -> bool {
        self.curve.values().all(|(c, _)| c >= &BigUint::from(2u8))
            && self.curve.values().any(|(_, e)| !e.is_zero())
            && on_curve(f, &self.curve).as_ref() == Some(&self.numerator)
            && on_curve(g, &self.curve).as_ref() == Some(&self.denominator)
            && batch
                .iter()
                .chain([f, g])
                .filter(|p| !p.is_zero())
                .all(|p| {
                    height(p, &self.curve).is_some_and(|h| {
                        on_curve(p, &self.curve)
                            .and_then(|terms| terms.get(&h).cloned())
                            .is_some_and(|c| c.is_positive())
                    })
                })
            && height(f, &self.curve) > height(g, &self.curve)
    }
}

/// One generic direction: a checkable proof, a regular-valid witness, or a missing obligation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Yes(GenericProof),
    No(Witness),
    Unknown(Reason),
}
impl Direction {
    /// Kleene truth value.
    fn holds(&self) -> Option<bool> {
        match self {
            Self::Yes(_) => Some(true),
            Self::No(_) => Some(false),
            Self::Unknown(_) => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Dominated {
        worse: Side,
    },
    Equivalent,
    /// Both directions refuted with verified witnesses.
    Incomparable,
    /// At least one direction unresolved and none refuted on both sides.
    Unknown,
}
impl Verdict {
    pub const ALL: [&'static str; 4] = ["Dominated", "Equivalent", "Incomparable", "Unknown"];
    fn from_directions(forward: Option<bool>, reverse: Option<bool>) -> Self {
        match (forward, reverse) {
            (Some(true), Some(true)) => Self::Equivalent,
            (Some(true), Some(false)) => Self::Dominated { worse: Side::Right },
            (Some(false), Some(true)) => Self::Dominated { worse: Side::Left },
            (Some(false), Some(false)) => Self::Incomparable,
            _ => Self::Unknown,
        }
    }
    /// Position in [`Verdict::ALL`].
    pub fn index(self) -> usize {
        match self {
            Self::Dominated { .. } => 0,
            Self::Equivalent => 1,
            Self::Incomparable => 2,
            Self::Unknown => 3,
        }
    }
    pub fn name(self) -> &'static str {
        Self::ALL[self.index()]
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldComparison {
    pub forward: Direction,
    pub reverse: Direction,
}
impl FieldComparison {
    pub fn verdict(&self) -> Verdict {
        Verdict::from_directions(self.forward.holds(), self.reverse.holds())
    }
}
fn exponent(m: &Monomial, v: &Symbol) -> BigUint {
    m.get(v).cloned().unwrap_or_default()
}
fn dominates(a: &Monomial, b: &Monomial) -> bool {
    b.iter().all(|(v, e)| exponent(a, v) >= *e)
}
impl Coverage {
    pub fn verify(&self, denominator: &Poly) -> bool {
        self.weights.iter().all(|(b, w)| {
            !w.is_negative() && denominator.terms().get(b).is_some_and(|c| c.is_positive())
        }) && self.weights.iter().map(|(_, w)| w).sum::<BigRational>() == BigRational::one()
            && self.exponent.iter().all(|(v, e)| {
                self.weights
                    .iter()
                    .map(|(b, w)| w * BigRational::from_integer(BigInt::from(exponent(b, v))))
                    .sum::<BigRational>()
                    >= BigRational::from_integer(BigInt::from(e.clone()))
            })
    }
}
fn verify_coverage(coverage: &[Coverage], f: &Poly, g: &Poly) -> bool {
    let upper = f.positive_hull();
    coverage.len() == upper.terms().len()
        && coverage
            .iter()
            .map(|c| &c.exponent)
            .collect::<BTreeSet<_>>()
            .len()
            == coverage.len()
        && coverage
            .iter()
            .all(|c| upper.terms().contains_key(&c.exponent) && c.verify(g))
}
/// Newton-hull LP over the positive denominator support; built once per denominator.
struct CoverageLp<'a> {
    support: Vec<&'a Monomial>,
    rows: Vec<Vec<BigRational>>,
}
impl<'a> CoverageLp<'a> {
    fn new(hull: &'a Poly, vars: &[Symbol]) -> Self {
        let support: Vec<_> = hull.terms().keys().collect();
        let mut rows: Vec<Vec<_>> = vars
            .iter()
            .map(|v| {
                support
                    .iter()
                    .map(|b| -BigRational::from_integer(BigInt::from(exponent(b, v))))
                    .collect()
            })
            .collect();
        rows.push(vec![BigRational::one(); support.len()]);
        rows.push(vec![-BigRational::one(); support.len()]);
        Self { support, rows }
    }
    fn cover(&self, a: &Monomial, vars: &[Symbol]) -> Result<Option<Coverage>, Reason> {
        if let Some(dominator) = self.support.iter().find(|b| dominates(b, a)) {
            return Ok(Some(Coverage {
                exponent: a.clone(),
                weights: vec![((*dominator).clone(), BigRational::one())],
            }));
        }
        let mut rhs: Vec<_> = vars
            .iter()
            .map(|v| -BigRational::from_integer(BigInt::from(exponent(a, v))))
            .collect();
        rhs.extend([BigRational::one(), -BigRational::one()]);
        Ok(
            lp::feasible(self.rows.clone(), rhs)?.map(|weights| Coverage {
                exponent: a.clone(),
                weights: self
                    .support
                    .iter()
                    .zip(weights)
                    .filter(|(_, w)| !w.is_zero())
                    .map(|(b, w)| ((*b).clone(), w))
                    .collect(),
            }),
        )
    }
}
/// A curve along which `x^a` outgrows every monomial of `lower`.
fn separating_curve(vars: &[Symbol], a: &Monomial, lower: &Poly) -> Result<Option<Curve>, Reason> {
    let direction = if lower.is_zero() {
        Some(vec![BigRational::one(); vars.len()])
    } else {
        let rows = lower
            .terms()
            .keys()
            .map(|b| {
                vars.iter()
                    .map(|v| {
                        BigRational::from_integer(
                            BigInt::from(exponent(b, v)) - BigInt::from(exponent(a, v)),
                        )
                    })
                    .collect()
            })
            .collect();
        lp::feasible(rows, vec![-BigRational::one(); lower.terms().len()])?
    };
    Ok(direction.map(|direction| {
        let denominator: BigInt = direction.iter().map(|e| e.denom()).product();
        vars.iter()
            .zip(direction)
            .map(|(v, e)| {
                let e = (e * BigRational::from_integer(denominator.clone())).to_integer();
                (v.clone(), (BigUint::from(2u8), e.magnitude().clone()))
            })
            .collect()
    }))
}

/// Facts about one field shared by both directions of a comparison.
struct Prepared<'a> {
    poly: &'a Poly,
    /// Positive part: an upper bound for the numerator, the Newton support for the denominator.
    hull: Poly,
}
fn monomial_degree(m: &Monomial, curve: &Curve) -> Option<BigUint> {
    m.iter()
        .try_fold(BigUint::zero(), |d, (v, a)| Some(d + a * &curve.get(v)?.1))
}
fn height(p: &Poly, curve: &Curve) -> Option<BigUint> {
    p.terms()
        .keys()
        .map(|m| monomial_degree(m, curve))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .max()
}

fn refute(base: &Curve, f: &Poly, g: &Poly, batch: &[Poly]) -> Option<Witness> {
    let mut scales = vec![base.clone()];
    for scale in [3u8, 4] {
        scales.push(
            base.iter()
                .map(|(v, (_, e))| (v.clone(), (scale.into(), e.clone())))
                .collect(),
        );
        for v in base.keys() {
            let mut curve = base.clone();
            curve.get_mut(v)?.0 = scale.into();
            scales.push(curve);
        }
    }
    scales.into_iter().find_map(|curve| {
        // A strict support-height gap is required even if actual leading terms cancel.
        if height(f, &curve) <= height(g, &curve) {
            return None;
        }
        Witness::on(curve, f, g, batch)
    })
}

fn cell_curves(vars: &[Symbol], batch: &[Poly]) -> Vec<Curve> {
    let curve = |exponents: Vec<BigUint>| {
        vars.iter()
            .cloned()
            .zip(exponents)
            .map(|(v, e)| (v, (2u8.into(), e)))
            .collect::<Curve>()
    };
    let mut curves = vec![curve(vec![BigUint::one(); vars.len()])];
    curves.extend((0..vars.len()).map(|i| {
        curve(
            (0..vars.len())
                .map(|j| BigUint::from(u8::from(i == j)))
                .collect(),
        )
    }));
    // shortcut: inspect at most 128 support pairs and keep 32 exposed tie directions; exhaustive fans are deferred.
    let mut inspected = 0;
    let mut ties = 0;
    for p in batch {
        let support: Vec<_> = p.terms().keys().collect();
        for (i, a) in support.iter().enumerate() {
            for b in &support[i + 1..] {
                inspected += 1;
                if inspected > 128 {
                    return curves;
                }
                for (i, v) in vars.iter().enumerate() {
                    let dv = BigInt::from(exponent(a, v)) - BigInt::from(exponent(b, v));
                    for (j, w) in vars.iter().enumerate().skip(i + 1) {
                        let dw = BigInt::from(exponent(a, w)) - BigInt::from(exponent(b, w));
                        if !(&dv * &dw).is_negative() {
                            continue;
                        }
                        let mut exponents = vec![BigUint::zero(); vars.len()];
                        exponents[i] = dw.magnitude().clone();
                        exponents[j] = dv.magnitude().clone();
                        let c = curve(exponents);
                        let h = height(p, &c);
                        if h == monomial_degree(a, &c)
                            && h == monomial_degree(b, &c)
                            && !curves.contains(&c)
                        {
                            curves.push(c);
                            ties += 1;
                            if ties == 32 {
                                return curves;
                            }
                        }
                    }
                }
            }
        }
    }
    curves
}

fn direction(f: &Prepared, g: &Prepared, ambient: &BTreeSet<Symbol>, batch: &[Poly]) -> Direction {
    let immediate = if f.poly == g.poly {
        Some(GenericProof::Identity)
    } else if f.poly.is_zero() {
        Some(GenericProof::Zero)
    } else {
        None
    };
    if let Some(proof) = immediate {
        if proof.verify(f.poly, g.poly) {
            return Direction::Yes(proof);
        }
    }
    let mut domain = ambient.clone();
    domain.extend(batch.iter().flat_map(Poly::variables));
    let vars: Vec<_> = domain.into_iter().collect();
    let lp = CoverageLp::new(&g.hull, &vars);
    let coverage: Result<Option<Vec<_>>, _> =
        f.hull.terms().keys().map(|a| lp.cover(a, &vars)).collect();
    let mut reason = match coverage {
        Ok(Some(coverage)) => {
            let proof = GenericProof::Newton(coverage);
            if proof.verify(f.poly, g.poly) {
                return Direction::Yes(proof);
            }
            Reason::Validity
        }
        Ok(None) => Reason::Validity,
        Err(reason) => reason,
    };
    for a in f.hull.terms().keys() {
        match separating_curve(&vars, a, &g.hull) {
            Ok(Some(curve)) => {
                if let Some(w) = refute(&curve, f.poly, g.poly, batch) {
                    return Direction::No(w);
                }
            }
            Err(error) => reason = error,
            Ok(None) => {}
        }
    }
    for curve in cell_curves(&vars, batch) {
        if let Some(w) = refute(&curve, f.poly, g.poly, batch) {
            return Direction::No(w);
        }
    }
    Direction::Unknown(reason)
}
/// Compare chosen certificates in the generic order, with a batch of these two fields, over `domain`
/// plus every variable either side reads. Unused coordinates matter:
/// bounded-variable slices still escape to infinity.
pub fn compare(
    f: &FieldCertificate,
    g: &FieldCertificate,
    domain: &BTreeSet<Symbol>,
) -> FieldComparison {
    let batch: Vec<_> = [f, g]
        .iter()
        .filter_map(|f| f.poly().ok())
        .filter(|p| !p.is_zero())
        .cloned()
        .collect();
    compare_in_batch(f, g, domain, &batch)
}
fn compare_in_batch(
    f: &FieldCertificate,
    g: &FieldCertificate,
    domain: &BTreeSet<Symbol>,
    batch: &[Poly],
) -> FieldComparison {
    let both = |reason: Reason| FieldComparison {
        forward: Direction::Unknown(reason.clone()),
        reverse: Direction::Unknown(reason),
    };
    let (fp, gp) = match (f.poly(), g.poly()) {
        (Ok(fp), Ok(gp)) => (fp, gp),
        (Err(reason), _) | (_, Err(reason)) => return both(reason),
    };
    let mut ambient = domain.clone();
    ambient.extend(fp.variables());
    ambient.extend(gp.variables());
    let [fs, gs] = [fp, gp].map(|poly| Prepared {
        poly,
        hull: poly.positive_hull(),
    });
    FieldComparison {
        forward: direction(&fs, &gs, &ambient, batch),
        reverse: direction(&gs, &fs, &ambient, batch),
    }
}

/// Field-wise comparison of two paths under the product order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileComparison {
    pub fields: BTreeMap<String, FieldComparison>,
    pub verdict: Verdict,
}
pub fn compare_profiles(f: &Profile, g: &Profile) -> ProfileComparison {
    compare_profiles_in_batch(f, g, &batch_certificates([f, g]))
}
fn batch_certificates<'a>(profiles: impl IntoIterator<Item = &'a Profile>) -> Vec<Poly> {
    let mut batch = Vec::new();
    for p in profiles
        .into_iter()
        .flat_map(|p| p.fields.values())
        .filter_map(|f| f.poly().ok())
    {
        if !p.is_zero() && !batch.contains(p) {
            batch.push(p.clone());
        }
    }
    batch
}
fn compare_profiles_in_batch(f: &Profile, g: &Profile, batch: &[Poly]) -> ProfileComparison {
    let missing = FieldCertificate {
        original_signed: None,
        certificate: Err(Reason::MissingField),
    };
    let domain: BTreeSet<_> = f.domain.union(&g.domain).cloned().collect();
    let fields: BTreeMap<_, _> = f
        .fields
        .keys()
        .chain(g.fields.keys())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|name| {
            let comparison = compare_in_batch(
                f.fields.get(name).unwrap_or(&missing),
                g.fields.get(name).unwrap_or(&missing),
                &domain,
                batch,
            );
            (name.clone(), comparison)
        })
        .collect();
    // Kleene conjunction over fields: any refutation wins, otherwise any unknown.
    let all = |pick: fn(&FieldComparison) -> Option<bool>| {
        let mut result = Some(true);
        for c in fields.values() {
            match pick(c) {
                Some(false) => return Some(false),
                None => result = None,
                Some(true) => {}
            }
        }
        result
    };
    let verdict = Verdict::from_directions(all(|c| c.forward.holds()), all(|c| c.reverse.holds()));
    ProfileComparison { fields, verdict }
}

/// All pairwise comparisons of a path batch and its candidate front.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathComparison {
    /// `(i, j, comparison)` for every `i < j`, in order.
    pub pairs: Vec<(usize, usize, ProfileComparison)>,
    /// Paths that no other path strictly dominates. Final-front filtering only:
    /// prefixes are never pruned, and equivalent paths are all kept.
    pub front: Vec<usize>,
}
impl PathComparison {
    /// Pair counts per verdict, ordered as [`Verdict::ALL`].
    pub fn counts(&self) -> [usize; 4] {
        let mut counts = [0; 4];
        for (_, _, c) in &self.pairs {
            counts[c.verdict.index()] += 1;
        }
        counts
    }
}
pub fn compare_all(profiles: &[Profile]) -> PathComparison {
    let batch = batch_certificates(profiles);
    // Identical profiles share one comparison result.
    let mut representatives: Vec<usize> = Vec::new();
    let class: Vec<usize> = profiles
        .iter()
        .enumerate()
        .map(|(i, p)| {
            representatives
                .iter()
                .position(|&r| profiles[r] == *p)
                .unwrap_or_else(|| {
                    representatives.push(i);
                    representatives.len() - 1
                })
        })
        .collect();
    let mut cache = HashMap::new();
    let mut dominated = BTreeSet::new();
    let mut pairs = Vec::new();
    for i in 0..profiles.len() {
        for j in i + 1..profiles.len() {
            let comparison = cache
                .entry((class[i], class[j]))
                .or_insert_with(|| {
                    compare_profiles_in_batch(
                        &profiles[representatives[class[i]]],
                        &profiles[representatives[class[j]]],
                        &batch,
                    )
                })
                .clone();
            if let Verdict::Dominated { worse } = comparison.verdict {
                dominated.insert(if worse == Side::Left { i } else { j });
            }
            pairs.push((i, j, comparison));
        }
    }
    PathComparison {
        pairs,
        front: (0..profiles.len())
            .filter(|i| !dominated.contains(i))
            .collect(),
    }
}

#[cfg(test)]
#[path = "../unit_tests/poly_compare.rs"]
mod tests;
