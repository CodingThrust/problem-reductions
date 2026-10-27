use super::*;

fn field(s: &str) -> FieldCertificate {
    FieldCertificate::new(&Expr::parse(s), ParameterRelation::Exact)
}

#[test]
fn identity_and_zero_explain_expansion() {
    let compare = |f: &FieldCertificate, g: &FieldCertificate| compare(f, g, &BTreeSet::new());
    let identity = compare(&field("n+m-m-n*m"), &field("n-n*m")).forward;
    assert_eq!(identity.to_string(), "Yes: f ≡ g after expansion");
    let zero = compare(&field("0"), &field("n-m")).forward;
    assert_eq!(zero.to_string(), "Yes: f ≡ 0 after expansion");
    assert_eq!(
        serde_json::to_value(identity).unwrap()["certificate"],
        json!({"kind":"identity"})
    );
    assert_eq!(
        serde_json::to_value(zero).unwrap()["certificate"],
        json!({"kind":"zero"})
    );
}

#[test]
fn newton_explains_positive_hull_weights() {
    let compare = |f: &FieldCertificate, g: &FieldCertificate| compare(f, g, &BTreeSet::new());
    let direction = compare(&field("n*m"), &field("n^2+m^2")).forward;
    let text = direction.to_string();
    assert!(text.contains("every monomial of f⁺ lies in the Newton hull of g⁺"));
    assert!(text.contains("1/2·("));
    let json = serde_json::to_value(direction).unwrap();
    assert_eq!(json["certificate"]["kind"], "newton");
    assert_eq!(json["certificate"].as_object().unwrap().len(), 2);
    assert_eq!(
        json["certificate"]["coverage"][0]["weights"][0]["weight"],
        "1/2"
    );
    assert!(json["certificate"]["coverage"][0]["exponent"].is_object());
}

#[test]
fn witness_shows_integer_curve_and_exact_leading_terms() {
    let witness = Witness {
        curve: BTreeMap::from([
            (Symbol::new("num_edges").unwrap(), (2u8.into(), 0u8.into())),
            (
                Symbol::new("num_vertices").unwrap(),
                (2u8.into(), 1u8.into()),
            ),
        ]),
        numerator: BTreeMap::from([(6u8.into(), BigRational::from_integer(64.into()))]),
        denominator: BTreeMap::from([(2u8.into(), BigRational::from_integer(4.into()))]),
    };
    assert_eq!(
        witness.to_string(),
        "num_edges = 2, num_vertices = 2·t; f ~ 64·t^6, g ~ 4·t^2; f/g → ∞"
    );
    let json = serde_json::to_value(&witness).unwrap();
    assert_eq!(json["curve"]["num_vertices"]["exponent"], "1");
    assert_eq!(json["numerator"][0]["coefficient"], "64");
    let zero = Witness {
        denominator: BTreeMap::new(),
        ..witness
    };
    assert!(zero.to_string().contains("g ≡ 0"));
}

#[test]
fn unknown_and_field_certificates_use_typed_reasons() {
    let unknown = Direction::Unknown(Reason::Validity);
    assert_eq!(
        unknown.to_string(),
        "Unknown: missing obligation: Newton coverage fails; no regular witness found positive for every batch certificate"
    );
    let json = serde_json::to_value(unknown).unwrap();
    assert_eq!(json["missing_obligation"]["kind"], "validity");
    for reason in [
        Reason::NotPolynomial("log(n)".into()),
        Reason::MissingField,
        Reason::ResourceLimit("too large"),
    ] {
        assert!(!reason.to_string().is_empty());
        assert!(serde_json::to_value(&reason).unwrap()["kind"].is_string());
    }
    let upper = FieldCertificate::new(&Expr::parse("n"), ParameterRelation::UpperBound);
    assert_eq!(upper.to_string(), "<= n");
    assert_eq!(field("n").to_string(), "= n");
    let missing = field("log(n)");
    assert!(missing
        .to_string()
        .starts_with("unavailable: not polynomial"));
    let json = serde_json::to_value(&missing).unwrap();
    assert_eq!(json["accuracy"]["reason"]["kind"], "not_polynomial");
    assert_eq!(
        serde_json::to_value(&upper).unwrap()["accuracy"]["kind"],
        "upper_bound"
    );
    assert_eq!(Verdict::Incomparable.to_string(), "Incomparable");
}

#[test]
fn signed_coverage_and_regular_refutations_serialize_as_single_directions() {
    let c = compare(&field("n"), &field("n*(n-1)/2-m"), &BTreeSet::new());
    assert!(c.forward.to_string().contains("n ≤ 1·(n^2)"));
    let json = serde_json::to_value(&c).unwrap();
    assert_eq!(json.as_object().unwrap().len(), 2);
    assert_eq!(json["forward"]["certificate"]["kind"], "newton");
    let c = compare(&field("n"), &field("n^2"), &BTreeSet::new());
    assert!(c
        .reverse
        .to_string()
        .contains("regular and positive across the batch"));
    let json = serde_json::to_value(c).unwrap();
    assert_eq!(json["forward"]["certificate"]["kind"], "newton");
    assert_eq!(json["reverse"]["certificate"]["kind"], "witness");
}
