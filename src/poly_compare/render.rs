//! Human-readable certificates and the version-5 exact JSON proof schema.
use super::*;
use serde::{Serialize, Serializer};
use serde_json::{Value, json};
use std::fmt;

fn monomial(m: &Monomial) -> String {
    Poly::from_terms([(m.clone(), BigRational::one())])
        .to_expr()
        .to_string()
}
fn powers(m: &Monomial) -> Value {
    json!(
        m.iter()
            .map(|(v, e)| (v.to_string(), e.to_string()))
            .collect::<BTreeMap<_, _>>()
    )
}
fn domain(d: &BTreeSet<Symbol>) -> Vec<String> {
    d.iter().map(ToString::to_string).collect()
}
fn power_term(coefficient: impl fmt::Display, degree: &BigUint) -> String {
    if degree.is_zero() {
        coefficient.to_string()
    } else if degree.is_one() {
        format!("{coefficient}·t")
    } else {
        format!("{coefficient}·t^{degree}")
    }
}
fn leading(p: &BTreeMap<BigUint, BigRational>) -> String {
    p.last_key_value()
        .map(|(e, c)| power_term(c, e))
        .unwrap_or_else(|| "0".into())
}
fn univariate(p: &BTreeMap<BigUint, BigRational>) -> Value {
    json!(
        p.iter()
            .map(|(e, c)| json!({"exponent":e.to_string(), "coefficient":c.to_string()}))
            .collect::<Vec<_>>()
    )
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotPolynomial(s) => write!(f, "not polynomial: {s}"),
            Self::Unavailable(e) => write!(f, "unavailable: {e}"),
            Self::MissingField => write!(f, "field missing on one path"),
            Self::Validity => write!(
                f,
                "Newton coverage fails; no regular witness found positive for every batch certificate"
            ),
            Self::ResourceLimit(s) => write!(f, "resource limit: {s}"),
        }
    }
}
/// `= formula`, `<= formula`, or `unavailable: reason`.
impl fmt::Display for FieldCertificate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.certificate {
            Ok((poly, ParameterRelation::Exact)) => write!(f, "= {}", poly.to_expr()),
            Ok((poly, ParameterRelation::UpperBound)) => {
                write!(f, "<= {}", poly.to_expr())
            }
            Err(reason) => write!(f, "unavailable: {reason}"),
        }
    }
}
impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
impl fmt::Display for Coverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terms = self
            .weights
            .iter()
            .map(|(m, w)| format!("{w}·({})", monomial(m)))
            .collect::<Vec<_>>();
        write!(f, "{} ≤ {}", monomial(&self.exponent), terms.join(" + "))
    }
}
impl fmt::Display for GenericProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identity => write!(f, "f ≡ g after expansion"),
            Self::Zero => write!(f, "f ≡ 0 after expansion"),
            Self::Newton(coverage) => {
                write!(f, "every monomial of f⁺ lies in the Newton hull of g⁺")?;
                for c in coverage {
                    write!(f, "\n  {c}")?;
                }
                Ok(())
            }
        }
    }
}
impl fmt::Display for Witness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let curve = self
            .curve
            .iter()
            .map(|(v, (c, e))| format!("{v} = {}", power_term(c, e)))
            .collect::<Vec<_>>();
        write!(
            f,
            "{}; f ~ {}, ",
            curve.join(", "),
            leading(&self.numerator)
        )?;
        if self.denominator.is_empty() {
            write!(f, "g ≡ 0; positive f cannot be bounded by g")
        } else {
            write!(f, "g ~ {}; f/g → ∞", leading(&self.denominator))
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yes(p) => write!(f, "Yes: {p}"),
            Self::No(w) => write!(f, "No (regular and positive across the batch): {w}"),
            Self::Unknown(reason) => write!(f, "Unknown: missing obligation: {reason}"),
        }
    }
}

// Exact numbers are decimal strings (rationals use reduced p/q), independent of JSON numeric precision.
macro_rules! serialize_view {
    ($ty:ty, $value:ident, $body:expr) => {
        impl Serialize for $ty {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let $value = self;
                let view: Value = $body;
                view.serialize(serializer)
            }
        }
    };
}
serialize_view!(
    Reason,
    r,
    match r {
        Reason::NotPolynomial(s) => json!({"kind":"not_polynomial", "detail":s}),
        Reason::Unavailable(e) => json!({"kind":"unavailable", "detail":e.to_string()}),
        Reason::MissingField => json!({"kind":"missing_field"}),
        Reason::Validity => json!({"kind":"validity"}),
        Reason::ResourceLimit(s) => json!({"kind":"resource_limit", "detail":s}),
    }
);
serialize_view!(
    FieldCertificate,
    c,
    json!({
        "original_signed": c.original_signed.as_ref().map(|p| p.to_expr().to_string()),
        "certificate": c.certificate.as_ref().ok().map(|(p, _)| p.to_expr().to_string()),
        "accuracy": match &c.certificate {
            Ok((_, relation)) => json!({"kind": relation}),
            Err(reason) => json!({"kind":"unavailable", "reason":reason}),
        },
    })
);
serialize_view!(
    Profile,
    p,
    json!({"source_coordinates":domain(&p.domain), "fields":p.fields})
);
serialize_view!(Verdict, v, json!(v.name()));
serialize_view!(
    FieldComparison,
    c,
    json!({"forward":c.forward, "reverse":c.reverse})
);
serialize_view!(
    Coverage,
    c,
    json!({"exponent":powers(&c.exponent), "weights":c.weights.iter().map(|(m,w)| json!({"exponent":powers(m), "weight":w.to_string()})).collect::<Vec<_>>()})
);
serialize_view!(
    GenericProof,
    p,
    match p {
        GenericProof::Identity => json!({"kind":"identity"}),
        GenericProof::Zero => json!({"kind":"zero"}),
        GenericProof::Newton(coverage) => json!({"kind":"newton", "coverage":coverage}),
    }
);
serialize_view!(
    Witness,
    w,
    json!({"kind":"witness", "curve":w.curve.iter().map(|(v,(c,e))| (v.to_string(),json!({"scale":c.to_string(),"exponent":e.to_string()}))).collect::<BTreeMap<_,_>>(),"numerator":univariate(&w.numerator),"denominator":univariate(&w.denominator)})
);
serialize_view!(
    Direction,
    d,
    match d {
        Direction::Yes(p) => json!({"status":"Yes","certificate":p}),
        Direction::No(w) => json!({"status":"No","certificate":w}),
        Direction::Unknown(reason) => json!({"status":"Unknown","missing_obligation":reason}),
    }
);
#[cfg(test)]
#[path = "../unit_tests/poly_compare_render.rs"]
mod tests;
