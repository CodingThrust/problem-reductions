use super::*;
use crate::parameters::{ParameterRelation, ParameterTransform, ParameterTransformError};

fn poly(s: &str) -> Poly {
    Poly::from_expr(&Expr::parse(s)).unwrap()
}
fn field(s: &str) -> FieldCertificate {
    FieldCertificate::new(&Expr::parse(s), ParameterRelation::Exact)
}
fn pair(f: &str, g: &str) -> FieldComparison {
    compare(&field(f), &field(g), &BTreeSet::new())
}
fn certificate(p: &Profile, name: &str) -> Option<(Poly, ParameterRelation)> {
    p.fields[name].certificate.clone().ok()
}

#[test]
fn signed_polynomials_with_matching_positive_supports_are_equivalent() {
    for (f, g) in [
        ("n^2+m^2-n*m", "n^2+m^2"),
        ("n*(n-1)/2-m", "n^2"), // Zero on complete graphs; its positive part is quadratic.
    ] {
        let c = pair(f, g);
        assert_eq!(c.verdict(), Verdict::Equivalent);
        let Direction::Yes(proof) = c.forward else {
            panic!("the positive denominator covers the signed numerator")
        };
        assert!(proof.verify(&poly(f), &poly(g)));
    }
}

#[test]
fn identical_signed_formulas_and_zero_have_direct_proofs() {
    let c = pair("n-m", "n-m");
    assert_eq!(c.verdict(), Verdict::Equivalent);
    for d in [c.forward, c.reverse] {
        let Direction::Yes(proof @ GenericProof::Identity) = d else {
            panic!("signed identity must be certified")
        };
        assert!(proof.verify(&poly("n-m"), &poly("n-m")));
        assert!(!proof.verify(&poly("n-m"), &poly("n+m")));
    }
    let Direction::Yes(proof @ GenericProof::Zero) = pair("0", "n-m").forward else {
        panic!("zero is bounded by every declared nonnegative certificate")
    };
    assert!(proof.verify(&poly("0"), &poly("n-m")));
    assert!(!proof.verify(&poly("n"), &poly("n-m")));
}

#[test]
fn witnesses_reject_negative_and_cancelled_top_faces() {
    let curve = BTreeMap::from([
        (Symbol::new("n").unwrap(), (2u8.into(), 1u8.into())),
        (Symbol::new("m").unwrap(), (2u8.into(), 2u8.into())),
    ]);
    for (f, g) in [("n-m", "1"), ("n^3", "n-m")] {
        let f = poly(f);
        let g = poly(g);
        let witness = Witness {
            curve: curve.clone(),
            numerator: on_curve(&f, &curve).unwrap(),
            denominator: on_curve(&g, &curve).unwrap(),
        };
        assert!(!witness.verify(&f, &g, &[]));
        assert!(Witness::on(curve.clone(), &f, &g, &[]).is_none());
    }
    let f = poly("m^2-n");
    let g = poly("m-n");
    assert!(Witness::on(curve.clone(), &f, &g, &[])
        .unwrap()
        .verify(&f, &g, &[]));
    let zero = poly("m-n^2/2");
    assert!(Witness::on(curve.clone(), &f, &zero, &[]).is_none());
    let zero = poly("0");
    assert!(Witness::on(curve, &f, &zero, &[])
        .unwrap()
        .verify(&f, &zero, &[]));
    let c = pair("n^3-m", "n-m");
    let Direction::No(w) = c.forward else {
        panic!("n grows while m stays bounded, with both leading coefficients positive")
    };
    assert!(w.verify(&poly("n^3-m"), &poly("n-m"), &[]));
    // The constant term has no separating ray; a later mixed term does.
    let c = pair("1+n*m^2", "n^2+m^2-n*m");
    let Direction::No(w) = c.forward else {
        panic!("search all positive numerator monomials")
    };
    assert!(w.verify(&poly("1+n*m^2"), &poly("n^2+m^2-n*m"), &[]));
}

#[test]
fn exact_expansion_preserves_cancellation_and_rational_coefficients() {
    assert_eq!(poly("n + m - m"), poly("n"));
    assert_eq!(poly("(n+m)^2/2"), poly("n^2/2+n*m+m^2/2"));
    for s in ["n/m", "sqrt(n)", "log(n)", "2^n", "factorial(n)"] {
        assert_eq!(Poly::from_expr(&Expr::parse(s)), None);
        assert!(matches!(
            field(s).certificate,
            Err(Reason::NotPolynomial(_))
        ));
    }
}

#[test]
fn running_map_composes_exactly_and_keeps_signed_provenance() {
    let f = ParameterTransform::new(
        "map",
        ParameterRelation::Exact,
        [("n", Expr::parse("n+m")), ("m", Expr::parse("n*m"))],
    )
    .unwrap();
    let source = ["n".to_string(), "m".to_string()];
    let p = Profile::from_path(
        &source,
        &[f.clone(), f.clone()],
        &f.compose(&f, "c").unwrap(),
    );
    assert_eq!(p.fields["n"].original_signed, Some(poly("n+m+n*m")));
    assert_eq!(
        certificate(&p, "m"),
        Some((poly("n^2*m+n*m^2"), ParameterRelation::Exact))
    );
    let bounded = ParameterTransform::new(
        "bound",
        ParameterRelation::UpperBound,
        [("n", Expr::parse("n")), ("m", Expr::parse("m"))],
    )
    .unwrap();
    let outer = ParameterTransform::new(
        "signed",
        ParameterRelation::Exact,
        [("size", Expr::parse("n*m-n"))],
    )
    .unwrap();
    let composed = bounded.compose(&outer, "c").unwrap();
    let p = Profile::from_path(&source, &[bounded, outer], &composed);
    assert_eq!(p.fields["size"].original_signed, Some(poly("n*m-n")));
    assert_eq!(
        certificate(&p, "size"),
        Some((poly("n*m"), ParameterRelation::UpperBound))
    );
}

#[test]
fn partial_hull_keeps_negative_terms_over_exact_inputs() {
    let first = ParameterTransform::from_fields(
        "first",
        [
            ("v", ParameterRelation::Exact, Expr::parse("n")),
            ("e", ParameterRelation::UpperBound, Expr::parse("n^2")),
        ],
    )
    .unwrap();
    let next = ParameterTransform::new(
        "next",
        ParameterRelation::Exact,
        [
            ("kept", Expr::parse("e + v^3 - v")),
            ("dropped", Expr::parse("v^2 - e")),
        ],
    )
    .unwrap();
    let p = Profile::from_transform(&first.compose(&next, "c").unwrap());
    assert_eq!(
        certificate(&p, "kept"),
        Some((poly("n^2+n^3-n"), ParameterRelation::UpperBound))
    );
    assert_eq!(
        certificate(&p, "dropped"),
        Some((poly("n^2"), ParameterRelation::UpperBound))
    );
}

#[test]
fn normalized_dependencies_do_not_lose_exactness() {
    let first = ParameterTransform::new(
        "first",
        ParameterRelation::UpperBound,
        [("m", Expr::parse("n^2"))],
    )
    .unwrap();
    let next = ParameterTransform::new(
        "next",
        ParameterRelation::Exact,
        [("size", Expr::parse("(m+1)^2-m^2-2*m"))],
    )
    .unwrap();
    let p = Profile::from_transform(&first.compose(&next, "composed").unwrap());
    assert_eq!(
        certificate(&p, "size"),
        Some((poly("1"), ParameterRelation::Exact))
    );
}

#[test]
fn newton_lp_decides_convex_supports_and_proven_incomparability() {
    assert_eq!(
        pair("n*m", "n^2+m^2").verdict(),
        Verdict::Dominated { worse: Side::Right }
    );
    let c = pair("n", "m");
    assert_eq!(c.verdict(), Verdict::Incomparable);
    for (d, f, g) in [
        (&c.forward, poly("n"), poly("m")),
        (&c.reverse, poly("m"), poly("n")),
    ] {
        let Direction::No(w) = d else {
            panic!("expected witness")
        };
        assert!(w.verify(&f, &g, &[]));
    }
}

#[test]
fn signed_support_coverage_ignores_exceptional_cancellations() {
    assert_eq!(pair("n*m-n*m/4", "n*m").verdict(), Verdict::Equivalent);
    assert_eq!(pair("n*m-n", "n*m").verdict(), Verdict::Equivalent);
    assert_eq!(
        pair("n^2*m^2-100*n*m", "n^2*m^2").verdict(),
        Verdict::Equivalent
    );
    assert_eq!(
        pair("n^2+m^2-n*m", "n^2+m^2").verdict(),
        Verdict::Equivalent
    );
    assert_eq!(pair("n^2-m", "n^2").verdict(), Verdict::Equivalent);
    let c = pair("n*m-2*n", "n*m");
    assert_eq!(c.verdict(), Verdict::Equivalent);
}

#[test]
fn signed_monotone_suffix_reversal_keeps_all_prefixes() {
    let prefix = |y: &str, z: &str| {
        ParameterTransform::new(
            "prefix",
            ParameterRelation::Exact,
            [("y", Expr::parse(y)), ("z", Expr::parse(z))],
        )
        .unwrap()
    };
    let (u, v) = (prefix("3", "n"), prefix("2", "n^2"));
    assert_eq!(
        compare_profiles(&Profile::from_transform(&u), &Profile::from_transform(&v)).verdict,
        Verdict::Dominated { worse: Side::Right }
    );
    let suffix = ParameterTransform::new(
        "suffix",
        ParameterRelation::Exact,
        [("size", Expr::parse("(y-2)*z"))],
    )
    .unwrap();
    let completed = [&u, &v].map(|p| Profile::from_transform(&p.compose(&suffix, "c").unwrap()));
    assert_eq!(compare_all(&completed).front, vec![1]);
    // Both prefixes must be extended: pruning v would delete the optimal zero completion.
    assert_eq!(
        certificate(&completed[1], "size"),
        Some((poly("0"), ParameterRelation::Exact))
    );
}

#[test]
fn kleene_profiles_preserve_unknown_and_equivalent_candidates() {
    let p = Profile::new(BTreeMap::from([("a".into(), field("n"))]));
    let q = Profile::new(BTreeMap::from([
        ("a".into(), field("m")),
        ("b".into(), field("n")),
    ]));
    assert_eq!(compare_profiles(&p, &q).verdict, Verdict::Incomparable);
    let all = compare_all(&[p.clone(), p.clone(), q]);
    assert_eq!(all.front, vec![0, 1, 2]);
    assert_eq!(all.counts(), [0, 1, 2, 0]);
    assert!(matches!(
        pair("n", "log(n)").forward,
        Direction::Unknown(Reason::NotPolynomial(_))
    ));
}

#[test]
fn unused_source_coordinates_remain_in_signed_profiles() {
    let p = Profile::from_transform(
        &ParameterTransform::new(
            "source",
            ParameterRelation::Exact,
            [("a", Expr::parse("n^2-100*n")), ("b", Expr::parse("m"))],
        )
        .unwrap(),
    );
    let q = Profile::from_transform(
        &ParameterTransform::new(
            "source",
            ParameterRelation::Exact,
            [("a", Expr::parse("n^2")), ("b", Expr::parse("m"))],
        )
        .unwrap(),
    );
    assert_eq!(compare_profiles(&p, &q).verdict, Verdict::Equivalent);
    assert!(p.domain.contains(&Symbol::new("m").unwrap()));
}

#[test]
fn unavailable_dependencies_are_checked_after_cancellation() {
    let mut first = ParameterTransform::new(
        "missing",
        ParameterRelation::Exact,
        [("n", Expr::parse("n"))],
    )
    .unwrap();
    first.declare_unavailable("m", "not registered");
    let next = ParameterTransform::new(
        "outer",
        ParameterRelation::Exact,
        [
            ("a", Expr::parse("n-m")),
            ("b", Expr::parse("(m+1)^2-m^2-2*m")),
        ],
    )
    .unwrap();
    let result = Profile::from_transform(&first.compose(&next, "c").unwrap());
    assert!(matches!(
        result.fields["a"].certificate,
        Err(Reason::Unavailable(
            ParameterTransformError::UnavailableInput { .. }
        ))
    ));
    assert_eq!(
        certificate(&result, "b"),
        Some((poly("1"), ParameterRelation::Exact))
    );
}

#[test]
fn proof_verifiers_reject_tampered_weights_and_curves() {
    let c = pair("n*m", "n^2+m^2");
    let Direction::Yes(mut proof) = c.forward else {
        panic!("convex proof")
    };
    assert!(proof.verify(&poly("n*m"), &poly("n^2+m^2")));
    if let GenericProof::Newton(coverage) = &mut proof {
        coverage[0].weights[0].1 += BigRational::one();
    }
    assert!(!proof.verify(&poly("n*m"), &poly("n^2+m^2")));
    let c = pair("n", "m");
    let Direction::No(mut w) = c.forward else {
        panic!("curve")
    };
    w.numerator.clear();
    assert!(!w.verify(&poly("n"), &poly("m"), &[]));
}

#[test]
fn exact_lp_finds_fractional_points_and_rejects_infeasibility() {
    let r = |v: i64| BigRational::from_integer(v.into());
    let feasible = lp::feasible(vec![vec![r(-3)], vec![r(2)]], vec![r(-1), r(1)])
        .unwrap()
        .unwrap();
    assert_eq!(feasible, vec![BigRational::new(1.into(), 3.into())]);
    assert_eq!(
        lp::feasible(vec![vec![r(-1)], vec![r(1)]], vec![r(-2), r(1)]).unwrap(),
        None
    );
    assert!(matches!(
        lp::feasible(vec![vec![r(1); 25]], vec![r(1)]),
        Err(Reason::ResourceLimit(_))
    ));
}

#[test]
fn real_graph_path_comparison_runs_from_registered_formulas() {
    let graph = crate::rules::ReductionGraph::new();
    let paths = graph.find_paths_up_to(
        "MaximumIndependentSet",
        &graph.variants_for("MaximumIndependentSet")[0],
        "QUBO",
        &graph.variants_for("QUBO")[0],
        4,
    );
    assert!(!paths.is_empty());
    let profiles: Vec<_> = paths
        .iter()
        .map(|p| graph.path_polynomial_profile(p).unwrap())
        .collect();
    for profile in &profiles {
        assert!(!profile.domain.is_empty());
        assert_eq!(
            compare_profiles(profile, profile).verdict,
            Verdict::Equivalent
        );
    }
    let result = compare_all(&profiles);
    assert!(!result.front.is_empty());
    // Front comparisons must use the same full batch, not standalone two-path scopes.
    for (i, j, c) in &result.pairs {
        if result.front.contains(i) && result.front.contains(j) {
            assert!(!matches!(c.verdict, Verdict::Dominated { .. }));
        }
    }
}

proptest::proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(64))]
    #[test]
    fn generated_decisions_have_exact_certificates_and_sample_bounds(
        a in 0u8..4, b in 0u8..4, c in 0u8..4, d in 0u8..4,
        coefficient in 1u8..6, n in 2u64..9, m in 2u64..9) {
        let f = format!("{coefficient}*n^{a}*m^{b}+1");
        let g = format!("n^{c}*m^{d}+n^2+m^2+1");
        let pf=poly(&f); let pg=poly(&g);
        let comparison=pair(&f,&g);
        let values=crate::types::ProblemParameters::new(vec![("n",n),("m",m)]);
        let evaluate = |s:&str| ParameterTransform::new("sample",ParameterRelation::Exact,[("value",Expr::parse(s))]).unwrap().evaluate(&values).unwrap().get("value").unwrap();
        for (direction,p,q,ps,qs) in [(&comparison.forward,&pf,&pg,f.as_str(),g.as_str()),(&comparison.reverse,&pg,&pf,g.as_str(),f.as_str())] {
            match direction {
                Direction::Yes(proof) => {
                    proptest::prop_assert!(proof.verify(p,q));
                    // All generated denominator coefficients are >= 1. Weighted AM-GM bounds each covered monomial by q.
                    let bound: BigRational=p.terms().values().sum();
                    proptest::prop_assert!(BigRational::from_integer(evaluate(ps).into()) <= bound*BigRational::from_integer(evaluate(qs).into()));
                }
                Direction::No(w) => proptest::prop_assert!(w.verify(p,q,&[pf.clone(),pg.clone()])),
                Direction::Unknown(_) => {}
            }

        }
    }
}

fn profile(s: &str) -> Profile {
    Profile::new(BTreeMap::from([("size".into(), field(s))]))
}
fn curve(n: (u8, u8), m: (u8, u8)) -> Curve {
    BTreeMap::from([
        (Symbol::new("n").unwrap(), (n.0.into(), n.1.into())),
        (Symbol::new("m").unwrap(), (m.0.into(), m.1.into())),
    ])
}

#[test]
fn complement_edges_have_direct_newton_coverage() {
    let f = poly("n");
    let g = poly("n*(n-1)/2-m");
    let c = pair("n", "n*(n-1)/2-m");
    let Direction::Yes(proof @ GenericProof::Newton(_)) = c.forward else {
        panic!("generic coverage")
    };
    assert!(proof.verify(&f, &g));
    assert!(!proof.verify(&poly("n^3"), &g));
}

#[test]
fn exceptional_worst_case_witness_is_not_a_generic_refutation() {
    let f = poly("n");
    let g = poly("(n-m)^2+1");
    let witness = Witness {
        curve: curve((2, 1), (2, 1)),
        numerator: BTreeMap::from([(1u8.into(), BigRational::from_integer(2.into()))]),
        denominator: BTreeMap::from([(0u8.into(), BigRational::one())]),
    };
    assert!(!witness.verify(&f, &g, &[f.clone(), g.clone()]));
    let Direction::Yes(proof) = pair("n", "(n-m)^2+1").forward else {
        panic!("exceptional unbounded ratios must allow generic Yes")
    };
    assert!(proof.verify(&f, &g));
}

#[test]
fn regular_witnesses_keep_bounded_strata_and_reject_invalid_batch_faces() {
    let f = poly("n*m");
    let g = poly("n^2");
    let w = Witness::on(curve((2, 0), (2, 1)), &f, &g, &[]).unwrap();
    assert!(w.verify(&f, &g, &[f.clone(), g.clone()]));
    assert!(!w.verify(&f, &g, &[poly("n-m")]));
    assert!(!w.verify(&f, &g, &[poly("n-2")]));
    assert!(!w.verify(&f, &g, &[poly("k")]));
    assert!(matches!(pair("n*m", "n^2").forward, Direction::No(_)));
    let mut tampered = w;
    tampered.numerator.clear();
    assert!(!tampered.verify(&f, &g, &[]));
}

#[test]
fn transitivity_and_zero_certificates_preserve_the_front() {
    let profiles = [profile("n"), profile("n^2"), profile("n^3")];
    let result = compare_all(&profiles);
    assert_eq!(result.counts(), [3, 0, 0, 0]);
    assert_eq!(result.front, vec![0]);
    for (i, j, c) in &result.pairs {
        let Direction::Yes(p) = &c.fields["size"].forward else {
            panic!("transitive exponent order")
        };
        assert!(p.verify(
            profiles[*i].fields["size"].poly().unwrap(),
            profiles[*j].fields["size"].poly().unwrap()
        ));
    }
    let result = compare_all(&[profile("n^2"), profile("0"), profile("n")]);
    assert_eq!(result.front, vec![1]);
    let c = &result.pairs[0].2.fields["size"];
    assert!(matches!(c.forward, Direction::No(_)));
    assert!(matches!(c.reverse, Direction::Yes(_)));
    assert_eq!(
        compare_all(&[profile("0"), profile("0")]).counts(),
        [0, 1, 0, 0]
    );
}

#[test]
fn adding_a_path_changes_the_common_valid_family_set() {
    let profiles = [profile("m"), profile("n"), profile("n-m")];
    assert_eq!(
        compare_profiles(&profiles[0], &profiles[1]).verdict,
        Verdict::Incomparable
    );
    let result = compare_all(&profiles);
    assert_eq!(result.pairs[0].2.verdict, Verdict::Unknown);
    assert_eq!(
        result.pairs[0].2.fields["size"].forward,
        Direction::Unknown(Reason::Validity)
    );
}

#[test]
fn alternative_scales_find_a_regular_family_with_a_bounded_parameter() {
    let profile = |size| {
        Profile::new(BTreeMap::from([
            ("size".into(), field(size)),
            ("guard".into(), field("k-3")),
        ]))
    };
    let c = compare_profiles(&profile("n^2"), &profile("n*k^2"));
    let Direction::No(w) = &c.fields["size"].forward else {
        panic!("perturb the bounded scale away from an invalid face")
    };
    assert!(w.verify(&poly("n^2"), &poly("n*k^2"), &[poly("k-3")]));
}

#[test]
fn lower_dimensional_cell_search_checks_the_whole_batch_fan() {
    let profile = |size| {
        Profile::new(BTreeMap::from([
            ("size".into(), field(size)),
            ("guard".into(), field("512*m^5-n^13")),
        ]))
    };
    let c = compare_profiles(&profile("n^3*m^3"), &profile("n^10+m^4"));
    let Direction::No(w) = &c.fields["size"].forward else {
        panic!("the batch face 5*e_m=13*e_n has a regular refutation")
    };
    assert!(w.verify(&poly("n^3*m^3"), &poly("n^10+m^4"), &[poly("512*m^5-n^13")]));
}
