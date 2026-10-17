# Polynomial comparator V1

`pred path MIS QUBO --compare` compares the listed paths' chosen polynomial
certificates in a **generic, batch-relative order**. Smaller certificates are
better. The default view shows the semantics once, pair counts and verdicts,
unresolved reasons, and one candidate front with field formulas. Add `--proofs`
for exact proofs and witness curves, or `--json` for structured certificates.
Path numbers refer to the original listing. Dominance names the winner and loser;
proven incomparability and `UNRESOLVED` remain distinct.

This order ignores exceptional exact-cancellation families, such as complete
source graphs with zero complement edges. It can be wrong on structured inputs,
including nearly complete graphs, whose complements are sparse. A generic Yes
is a statement about the defined families, not a worst-case guarantee or an
average runtime bound. No distribution of instances is assumed.

## Definition in plain language

Fix a comparison batch of paths sharing one source domain. Let `S` contain every
nonzero polynomial certificate of every field of every compared path. Nominal
signed provenance is audit data; `S` uses the chosen exact or upper certificates.
Unavailable fields have no polynomial certificate and return an explicit reason.
The comparator trusts that every declared certificate is a nonnegative size on
valid source instances; it does not try to prove nonnegativity.

A growth family is `x_i = c_i*t^e_i`, with real `c_i >= 2`, real `e_i >= 0`, and
at least one growing coordinate. Every nonempty choice of growing variables is
its own **stratum**. Within a stratum, the common refinement of the Newton normal
fans of `S` supplies the exponent cells. Every relative-interior cell has its
own Lebesgue measure on scales and exponents, including lower-dimensional
exponent cells. Bounded coordinates retain their scale-coordinate measure:
`n` fixed while `m` grows is a full-measure case of its own stratum.

For a nonzero polynomial `p`, its support height `h_p(e)` is the largest dot
product of a monomial's exponent with `e`. The **top face** `p_e(c)` sums all terms
attaining that maximum, before cancellation. A family is regular if these values
are nonzero for every member of `S`, and valid if they are all positive. Zero
certificates impose no validity condition and have height minus infinity.
Non-regular families form a null set in each cell. Negative top faces exclude
families because declared sizes cannot become negative on real instances.
Positive top faces are necessary for realizability; they do not prove that a
family obeys all source-instance constraints.

`f <=gen g` means `h_f(e) <= h_g(e)` for almost every valid family in **each cell
of each stratum**. Along a regular valid family, this is exactly eventual
boundedness of the ratio. For any probability measure equivalent to Lebesgue on
the valid part of a cell, and every epsilon greater than zero, there are `C` and
`N` such that `f <= C*g` for every `t >= N` outside an epsilon-fraction of
families. No particular probability distribution is selected. Empty valid parts
make comparisons vacuous. Zero against zero is bounded; a nonzero certificate
against zero is unbounded on valid families.

The order is a preorder within a **fixed batch**. Adding paths may shrink the
valid set and change verdicts. Changing the CLI listing limit can therefore
change a pair's verdict. Comparisons from different batches cannot be chained.

## API and pipeline

The library entry points are `poly_compare::{compare, compare_profiles,
compare_all, compare_summary}` and `ReductionGraph::path_polynomial_profile`:

- `compare(f, g, domain) -> FieldComparison { forward, reverse }` uses a batch
  consisting of its two field certificates. `verdict()` combines the directions.
- `compare_profiles(f, g) -> ProfileComparison { fields, verdict }` uses every
  certificate of both profiles, including fields outside the compared direction.
- `compare_all(&[Profile]) -> PathComparison { pairs, front }` collects `S` once
  for the whole batch. `counts()` follows `Verdict::ALL` order: Dominated,
  Equivalent, Incomparable, Unknown. Identical profiles share comparisons.
- `compare_summary(&[Profile]) -> ComparisonSummary { counts, front }` retains
  every original certificate in `S`, groups equal upper growth supports, and
  counts group-pair multiplicities without allocating every pair record. It
  returns the original route indices, including every equivalent route.

Library pair/front indices are zero-based; CLI indices are one-based. Callers
must compare the same source domain and target parameter schema. Graph endpoints
establish these. Graph profiles preserve all source coordinates, including unused
ones. `Profile::new(fields)` infers coordinates from its certificates.

1. `polynomial::{Poly, Monomial}` provides the shared exact algebra. Supported
   `Expr` nodes expand into a sorted `BTreeMap<Monomial, BigRational>`, combining
   equal exponents and deleting zeros. Exponents are arbitrary-precision
   `BigUint`. `Poly::from_expr` returns `None` for unsupported operations or
   variable denominators; field conversion reports `Reason::NotPolynomial`.
2. `FieldCertificate` keeps `original_signed: Option<Poly>` separately from
   `certificate: Result<(Poly, ParameterRelation), Reason>`.
   `ParameterRelation::{Exact, UpperBound}` is shared with parameter transforms.
   Nominal substitution through upper inputs is provenance, not an equality or
   bound. No registered formula or accuracy contract is changed.
3. `ParameterTransform::compose` owns composition. Expansion removes canceled
   dependencies before checking them. Exact inputs preserve the outer contract.
   For upper inputs the partial hull deletes negative monomials involving any
   upper input, retaining negatives over exact inputs, including constants.
   Native substitution then expands exactly. Missing inputs affect only their
   dependent fields. Composition uses the nonnegative orthant, including 0 and 1,
   independently of the asymptotic scale box. `Profile::from_transform` projects
   the result; `Profile::from_path` also reconstructs nominal signed formulas.
4. Comparison first checks identity after expansion and a zero numerator. It
   then covers every numerator positive-hull monomial by a convex combination
   of denominator positive-hull support, coordinatewise. This proves support
   dominance on the entire nonnegative direction cone. Every valid top face
   contains a positive term, so signed and positive-hull support heights agree.
   There is **no denominator lower-tightness obligation**.
5. Coordinatewise dominance supplies one-weight proofs; axis-simplex coverage
   supplies exact weights `a_i / b_i`, allocating any remaining weight to an
   axis vertex. Other coverage uses exact Fourier–Motzkin feasibility. Redundant
   support points covered by axis vertices or coordinatewise dominance are
   removed before constructing the LP. LPs have at most
   24 variables and 512 elimination rows; exceeding a limit returns Unknown
   unless a separately verified witness decides the direction.
6. Refutation uses integer scales and exponents. It tries exact LP separating
   directions, then a small deterministic catalogue of scales and exponent
   directions. The single witness checker verifies both exact substitutions,
   a strict support-height gap, and a positive original top face for **every
   nonzero batch certificate**. Checking only post-cancellation leading terms,
   or only the compared field, is insufficient. The curve must escape and its
   scales must be at least 2.
7. Fields combine by strong Kleene conjunction: any No refutes a profile
   direction, all Yes prove it, otherwise it is Unknown. Both directions give
   `Equivalent`, `Dominated { worse }`, proven `Incomparable` (both refuted), or
   `Unknown`. Final-front filtering removes only certified strict dominance.
   Equivalent paths retain their provenance. **No prefix pruning** is performed.

The summary compares positive support projections with unit coefficients. On
every regular valid original family, the original top face contains a positive
term, so its height equals the positive-support height. Removing coordinatewise
dominated or convexly covered points preserves that height for every nonnegative
exponent vector. Consequently, these projections have the same growth order on
the original valid families. They impose no additional validity restriction,
because their coefficients are positive. All original signed certificates stay
in the batch, including certificates belonging to grouped routes. Source domains
and unavailable-field positions remain part of each group key. Native full
comparisons continue to produce proofs for the original certificates.

Polynomial IDs, positive hulls, the curve catalogue, exact curve evaluations and
batch regularity checks are cached within one comparison call. These caches are
never reused across batches with different path limits. Scale changes cannot
alter a support-height gap, so alternative scales are considered only after a
strict gap is established. Public proof and witness verifiers remain available
for independent checking.

`growth.rs` and `big_o.rs` remain terminal complexity-display projections; they
do not rank paths. The comparator compares chosen certificates, including upper
bounds. Equal upper certificates do not establish equal actual target sizes.

## Proofs and examples

Each direction is `Direction::{Yes(GenericProof), No(Witness), Unknown(Reason)}`:

- `GenericProof::Identity` verifies equality of the expanded polynomials.
- `GenericProof::Zero` verifies that the numerator is identically zero.
- `GenericProof::Newton(Vec<Coverage>)` verifies complete, unique coverage of
  the numerator's positive support. Each `Coverage` records a monomial and
  nonnegative rational convex weights on positive denominator monomials.
- `Witness` records integer scales/exponents and the complete exact univariate
  substitutions. `Witness::verify(f, g, S)` checks regularity, positivity, and
  the strict gap across the supplied batch. `Witness::on(curve, f, g, S)`
  evaluates and returns only a verified witness.
- `Unknown(Reason)` retains one typed missing obligation: unsupported polynomial,
  missing field, unavailable transform, resource limit, or unsuccessful coverage
  and regular-valid witness search. Unavailable errors retain
  `ParameterTransformError`.

All Yes objects are re-verified by `GenericProof::verify`; every No passes the
batch-aware witness checker. Numerical samples are regression evidence, never
the universal proof.

For `f=n`, `g=(n-m)^2+1`, the exceptional curve `n=m=2*t` has `f=2*t, g=1`.
Its denominator's original top face cancels, so the witness is rejected, and
Newton coverage proves Yes. Similarly `n` against `n*(n-1)/2-m` gives Yes even
though complete graphs have zero complement edges. These are explicit examples
of why this result must not be read as a worst-case guarantee.

`n*m` against `n^2` is refuted by `n=2, m=2*t`; bounded strata are retained.
For the batch `[n^2, 0, n]`, the zero path dominates both nonzero paths: zero
certificates do not empty the valid set. Finally, `[m, n]` is proven incomparable,
while adding the certificate `n-m` excludes the original forward witnesses. The
finite implementation reports Unknown for that pair in the larger batch rather
than pretending the old witness is still valid.

## JSON proof schema (version 5)

`--json` and `-o` include `polynomial_comparison` with `schema_version: 5`,
`mode: "generic"`, `domain`, `scope`, `profiles`, `front`, `counts`,
and `pairs`. Each pair contains `paths: [left, right]`, one `verdict`, one
`description`, and `fields`. Dominated pairs add `dominates: {winner, loser}`.
Indices are one-based. Each field contains exactly `forward` and `reverse`.
There are no parallel result fields or an implied-proof wrapper.

Direction objects have this shape:

```json
{"status":"Yes","certificate":{"kind":"identity"}}
```

- Yes certificates have `kind: identity | zero | newton`. Identity and zero have
  no further fields. Newton has `coverage`, an array of `{exponent, weights}`.
  Each weight is `{exponent, weight}`; exponents are coordinate-to-exponent
  objects, with `{}` denoting the constant monomial. There is no lower-bound
  certificate or source-box obligation in a Yes proof.
- No certificates have `kind: witness`, a `curve` mapping coordinates to
  `{scale, exponent}`, and `numerator`/`denominator` arrays of
  `{exponent, coefficient}` for the complete univariate substitutions. An empty
  coefficient array represents zero. Reconstruct `S` from **all profiles'**
  polynomial certificates when checking the witness.
- Unknown has one `missing_obligation` object with
  `kind: not_polynomial | missing_field | unavailable | validity | resource_limit`.
  Reasons with explanatory text include `detail`. `validity` means coverage
  failed and the finite search found no witness regular and positive across
  the batch; it does not prove that valid families are absent.

All exact certificate integers and rational numbers are strings: decimal
integers or reduced `p/q` fractions. Profile fields retain display-formatted
`original_signed` and `certificate` formulas (or null), plus
`accuracy: {kind: exact | upper_bound | unavailable}`. Unavailable accuracy has
its typed `reason`. Proofs are always present in JSON; `--proofs` affects only
human output. Consumers should use typed fields, not parse descriptions.

This is the CLI/full-proof schema. The browser's `summarize_paths` binding returns
profiles, counts and the front; its compatible `compare_paths` binding also
returns every pair verdict. Neither browser payload transports field proof
records. The native `compare_all` proof API is unchanged.

## Limitations

The result is **batch-relative** and ignores exact top-face cancellations.
Structured inputs may lie entirely on excluded families; nearly complete graphs
with sparse complements require separate analysis. Bounded strata and exponent
equality cells are retained, not discarded as null. Positive regular faces are
only a relaxation of realizability; source promises and parameter constraints
are not inferred. Comparisons can be vacuous in cells with no valid families.

The witness catalogue tries LP separators, scales 2, 3 and 4 uniformly and one
coordinate at a time, all-growing and single-growing directions, and exposed
two-coordinate ties from batch supports. It inspects at most 128 support pairs
and retains at most 32 ties. It does not enumerate the full common fan. Coverage
on the whole cone can fail even when a batch-restricted Yes is true; a valid
refutation may also lie beyond the catalogue. Both cases can remain Unknown.
Unknown is never labeled proven incomparability.

Exact expansion can grow exponentially. There is no silent truncation of a
polynomial certificate; large expansions may exhaust time or memory. Pair
output is quadratic in the listed paths. Summary comparison uses growth-support
groups and retains no pair matrix; all-distinct groups still require quadratic
comparison work. A capped CLI listing fixes only
that batch; its truncation indicator reports omitted paths, and the front makes
no claim of exhaustive optimality. Advanced cancellation solving, declared
constraints, exhaustive fans, SOS, SONC, CAD and Gröbner bases remain deferred.

## Graph audit and measured snapshot

The example enumerates simple paths of 1–4 non-Turing hops. Nodes are problem
names with complete variants; no node repeats, but different variants of one
problem may occur. Variant casts count as hops. Uncomposable paths are counted
as skipped. Each variant endpoint group is one batch containing every field
certificate, with unordered pair counts and a summed final-front size. This
measures symbolic certificates, not concrete construction execution.

```sh
cargo run --release --example polynomial_compare_audit
```

Snapshot measured **11 October 2026**, using the release command above. Counts
are not promises for future graph revisions.

| Measurement | Count |
| --- | ---: |
| Simple paths, 1–4 non-Turing hops | 3,169 |
| Paths without a composable transform | 0 |
| Variant endpoint groups | 2,100 |
| Unordered path-pair comparisons | 4,274 |
| Dominated | 1,883 |
| Equivalent | 218 |
| Proven incomparable | 2,170 |
| Unknown | 3 |
| Candidate front, summed over endpoint groups | 2,544 |

The distribution and front match the preceding batch-relative implementation
exactly. Within Unknown pairs, the unresolved-direction breakdown is:

| Reason | Field | Directions |
| --- | --- | ---: |
| Validity: no regular-valid witness found after coverage fails | num_quadratic_terms | 1 |
| Unavailable | num_edges | 2 |
| Unavailable | num_sets | 2 |
| Unavailable | num_vertices | 2 |
| Unavailable | universe_size | 2 |

Enumeration and composition took 743.12 ms; total time was 7.95 s, excluding
compilation. These are single-run measurements, not performance guarantees.
