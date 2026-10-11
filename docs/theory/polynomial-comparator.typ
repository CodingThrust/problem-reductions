#set document(title: "Signed polynomial parameter transformations", author: "problem-reductions project")
#set page(paper: "a4", margin: 21mm, numbering: "1")
#set text(font: "New Computer Modern", size: 11pt)
#show math.equation: set text(font: "New Computer Modern Math")
#set par(justify: true, leading: 0.65em)
#set heading(numbering: "1.1.")
#set outline(indent: 1em)
#show table.cell: set par(justify: false)
#set table(align: left, inset: 5pt, stroke: 0.35pt + rgb("cad2dc"))
#let palette = (Definition: rgb("eef5fc"), Theorem: rgb("edf6ee"), Proposition: rgb("edf6ee"), Lemma: rgb("edf6ee"), Counterexample: rgb("fff0ef"), Example: rgb("f3f4f5"), Remark: rgb("f3f4f5"))
#show heading.where(level: 1): it => {
  if it.numbering != none { counter(figure.where(kind: "statement")).update(0) }
  it
}
#show figure.where(kind: "statement"): set block(breakable: true)
#let stmt(kind, title, body) = figure(
  block(fill: palette.at(kind), inset: 10pt, radius: 3pt, breakable: true)[
    #set align(left)
    #block(sticky: true, below: 3pt)[#strong[#kind #context {
      let sec = counter(heading).get().first()
      let n = counter(figure.where(kind: "statement")).get().first()
      [#sec.#n.]
    } #title]]
    #v(5pt)
    #body
  ],
  kind: "statement", supplement: kind, numbering: "1",
)
#show ref: it => {
  let el = it.element
  if el != none and el.func() == figure and el.kind == "statement" {
    context {
      let sec = counter(heading).at(el.location()).first()
      let n = counter(figure.where(kind: "statement")).at(el.location()).first()
      link(it.target, [#el.supplement #sec.#n])
    }
  } else { it }
}
#let overview(body) = [#emph(body)]
#let hyp(n, body) = block(above: 4pt, below: 4pt)[*\(H#n\)* #body]
#let step(n, title, body) = block(breakable: true, above: 7pt)[*Step #n (#title).* #body]
#let proof(body) = [
  #block(sticky: true, below: 0pt)[*Proof.*]
  #body
  #align(right)[$qed$]
]
#let yes = [Yes]
#let no = [No]
#let unk = [Unknown]

#align(center)[
 #text(19pt, weight: "bold")[Signed polynomial parameter transformations]
 #v(4pt)
 Composition, certified comparison and Pareto search \
 problem-reductions project · 9 October 2026
]

*Implementation scope (11 October 2026).* The Rust symbolic comparator implements only the generic, fixed-batch order in @sec-generic. The earlier worst-case sections remain mathematical background; the API, CLI and candidate front report one generic result.

*Reading guide.* This note extends the accepted 14-page theory from positive polynomials to rational signed polynomials. Exact simplification, sound upper propagation and comparison are separate operations. Complete procedures are identified by fragment; a failed certificate search is never called incomparability.

#outline(depth: 1)
#table(columns: (1fr,3fr),
 table.header([*Notation*],[*Meaning*]),
 [$D,pi,F_a,U,L$],[Realizable domain, registered projection, fiber and envelopes (@d-semantics).],
 [$B_p,omega_p$],[Chosen path certificate and actual output parameter.],
 [$O_D,Theta_D,prec.eq$],[Outside-bounded-set order, equivalence, fieldwise product.],
 [$E,J,H_J P$],[Exact inputs, upper inputs and partial polynomial hull (@t-compose).],
 [$P,Q,h_A,K$],[Positive/negative parts, support function, log-recession cone.],
 [$cal(C),epsilon$],[Finite rational certificate catalogue and strict budget margin.],
 [$sigma,H$],[Full continuation state and hop bound.],
)
#table(columns: (1fr,2.4fr,1.4fr),
 table.header([*Result*],[*Conclusion*],[*Status*]),
 [@t-compose],[No signed-polynomial syntax rejection; exact/upper accuracy retained.],[Proved],
 [@t-newton],[Positive envelopes compare by cone LP under coverage.],[Proved],
 [@t-certificates],[Derivative, absorption and rational circuit budgets are sound.],[Proved],
 [@t-fragments],[Complete tight-envelope, shifted-face and univariate fragments.],[Proved],
 [@t-integer-boundary],[General integer completeness already fails for quartic sums of squares.],[Undecidable],
 [@t-pruning; @t-search],[Safe pruning and complete finite fronts on decided fragments.],[Proved],
)

= Scope, parameters and comparison <sec-scope>
#overview[Certificates concern envelopes of registered output parameters, not construction costs.]

Reductions are deterministic one-instance maps, total on stated promises. Correct solution/value mappings are assumed and not costed; construction cost, approximation and randomization are excluded. Queries exclude Turing edges or compare only their per-query parameter maps. Only polynomials are reduction certificates here: rational coefficients and nonnegative integer exponents. Logarithms used in proofs are coordinates, not permitted edge functions. Coefficient magnitude may itself be registered.

#stmt("Definition", [Realized relations and field accuracy])[
A problem has valid instances $I$ and total parameters $pi:I->NN_0^k$. Invalid strings lie outside $I$. For $f:A->I_t$, $A subset.eq I_s$, define
$ D=pi_(s)(A), quad F_a={x in A:pi_(s)(x)=a}, quad
K_f={(pi_(s)(x),pi_(t)(f(x))):x in A}. $
Each target coordinate has envelopes $U_(j)(a)=sup_(x in F_a)pi_(t)(f(x))_j$ and $L_(j)(a)=inf_(x in F_a)pi_(t)(f(x))_j$. They are defined only for $a in D$, in $[0,infinity]$. Fibers may be infinite; empty fibers are undefined, not zero.

A finite polynomial certificate $B_j$ is *exact* when $L_j=U_j=B_j$ and *upper* when $U_j<=B_j$. Actual values and sound certificates are nonnegative on $D$, even if a displayed polynomial is negative off $D$. Missing fields remain unavailable. A field factors exactly through parameters iff it is constant on each fiber: the inequalities $L_j<=pi_(t)(f(x))_j<=U_j$ prove both directions.
] <d-semantics>

Soundness and composition concern parameters in $NN_0$, including $0,1$. The asymptotic query in this note is explicitly restricted to integer $D subset.eq {2,3,dots}^k$, optionally constrained by polynomial or monomial conditions. Other strata require separate treatment; they cannot be silently removed from a larger query. A real relaxation $D_R$ is always distinguished from $D$. No measurability assumption is needed.

For finite nonnegative $f,g$ on $D$, with $r(a)=max_i a_i$, fix Howell's outside-bounded-set convention [H08, §2]:
$ f in O_(D)(g) <==> exists c>0, exists N: forall a in D,
 r(a)>=N ==> f(a)<=c g(a). $
Write $f scripts(approx)_D g$ or $Theta_D$ for both directions. Apply $scripts(prec.eq)_D$ fieldwise to maps; $f scripts(prec)_D g$ means $f scripts(prec.eq)_D g$ and $g scripts(prec.eq)_D f$ fails. Reflexivity uses $c=1$; transitivity multiplies constants and takes the maximum threshold. Thus this is a preorder and its $Theta$ quotient is a partial order. Bounded-domain comparisons are vacuous. Bounded exceptions are finite on integer domains; zero denominators remain significant on unbounded slices.

*Guarantee mode* compares chosen $B_p,B_q$. If $B_p<=c B_q$ eventually and $B_q<=C U_q$ on the same fibers, then $U_p<=c C U_q$. With $B_q<=C L_q$, the same chain gives an instance-wise bound. Taking suprema proves that instance-wise bounds imply envelope bounds. Equal certificates alone do not imply equal actual behavior: both $n$ and $n^2$ have upper certificate $n^2$.

= Signed composition without syntax rejection <sec-composition>
#overview[Normalize first; preserve exact information before constructing any majorant.]

A polynomial is stored in its unique sparse expansion $P=sum_a c_a x^a$ with rational coefficients, combined identical exponents and deleted zero terms. Substitution and expansion are exact finite operations; for example $n+m-m=n$. The running transformation and its second iterate are
$ F(n,m)=(n+m,n m), quad F compose F(n,m)=(n+m+n m,n^2 m+n m^2). $

#stmt("Theorem", [Exact composition, partial hulls and derivative certificates])[
#hyp(1)[At a source vector $a$, intermediate coordinates indexed by $E$ are exactly $e(a)$; those indexed by $J$ have actual values $z$ with $0<=z<=u(a)$. Exact and upper vectors are finite and nonnegative on the source domain.]
#hyp(2)[A downstream exact or upper coordinate is represented by a polynomial $P(e,z)$, valid on reachable instances.]
#enum(numbering: "(a)")[Exact substitution composes exactly. If all inputs used by the normalized outer expression are exact, its outer exact/upper contract is preserved.][Delete only negative monomials containing a variable indexed by $J$; call the result $H_J P$. On nonnegative inputs,
$ P(e,z)<=H_J P(e,z)<=H_J P(e,u)<=P^+(e,u), $
where $P^+$ discards all negative monomials. Thus the partial hull is a sound polynomial upper certificate, no worse pointwise than the full hull.][Alternatively retain $P$ if each $frac(partial P,partial z_j)>=0$ on every segment obtained by increasing $z$ to $u$ with $e$ fixed. A sufficient test is derivative nonnegativity on the entire interpolation box. Exact inputs need no monotonicity.][Every finite path with complete valid polynomial edge fields and satisfied promises has finite polynomial certificates. No path is rejected merely for a signed or non-monotone intermediate formula. Unavailable input fields and unproved semantic eligibility are distinct obstacles.]
] <t-compose>
#proof[
#step(1, [Exactness and normalization])[
Substituting actual equalities gives $pi_(t)(f_(2)(f_(1)(x)))_j=P(e(pi_(s)(x)))$ for an exact outer field; an outer upper inequality gives the same substitution as an upper bound. Polynomial distributivity proves expansion preserves every value. A variable canceled before substitution imposes no dependency or accuracy loss.]
#step(2, [Partial hull])[
Removed monomials are nonpositive on the orthant. Every remaining negative monomial is independent of $z$; all terms that depend on $z$ have positive coefficients and nonnegative exponents. Hence $H_J P$ is isotone in precisely those coordinates. The last inequality discards its remaining nonpositive terms. Combining this chain with the downstream bound proves upper soundness and, on realizable $a$, nonnegativity of the resulting bound.]
#step(3, [Derivative alternative])[
Raise one coordinate at a time. The fundamental theorem of calculus gives
$ P(e,z+t e_j)-P(e,z)=integral_0^t (frac(partial P,partial z_j))(e,z+s e_j) dif s>=0. $
Every segment must be inside the certified derivative domain; intermediate points need not be realizable parameter vectors. The finite sequence of coordinate increases proves the upper substitution.]
#step(4, [All finite paths])[
Start with exact identity coordinates. At each edge, normalize, use exact substitution when possible, otherwise a certified derivative rule or the always-available partial hull. Polynomial substitution stays finite; induction proves the claim. The grade is upper after any inexact substitution, unless an independent equality proof upgrades it.

Even a polynomial negative on a larger relaxation can be sound on the actual domain; failed relaxation validation yields a domain obligation, not a syntax rejection.]
]

The read-only `src/parameters.rs` uses a full positive hull when any used input is upper-bounded. The partial hull improves this policy: for $P(e,z)=e z-e$, retaining $-e$ is safe; the full hull returns $e z$. Domain-sensitive derivative checks can do better still. They must cover actual values $0,1$: $(z-2)^2$ is increasing for $z>=2$, but actual $z=0$ and upper value $2$ give $4>0$. An asymptotic sign proof alone is not a global composition certificate.

For a fixed path, compare its final normalized polynomials on the original source domain. Intermediate images remain relevant for sound substitution and promises, but are not additional variables in that final comparison. Keep formula provenance and accuracy separately from any growth-class representative.

= Newton comparison for positive envelopes <sec-newton>
#overview[The positive theorem remains the engine, now applied to certified envelopes and strata.]

For a nonempty positive polynomial $P=sum_(a in A)c_a x^a$, put $h_(A)(y)=max_(a in A)a dot y$. Let $c_*=min c_a$ and $C=sum c_a$. For all positive $x$,
$ c_*exp(h_(A)(log x))<=P(x)<=C exp(h_(A)(log x)). $
An attaining monomial proves the lower bound; bounding every monomial by the maximum and summing proves the upper bound. Thus the logarithmic error lies in $[log c_*,log C]$, uniformly even when some variables remain bounded.

For positive maps $f,g$, with inner supports $A_i$ and outer supports $B_j$, distributivity gives
$ S_j=union_(b in B_j){sum_i sum_(r=1)^(b_i) a_(i r):a_(i r) in A_i}, quad
h_(S_j)=h_(B_j) compose (h_(A_1),dots,h_(A_l)). $
Indeed, nonnegative $b_i$ allow independently attaining each inner maximum. If absolute inner log errors are bounded by $e_i>=0$ and absolute outer errors by $eta_j>=0$, the composite error is at most $eta_j+max_b sum_i b_i e_i$, by the triangle inequality and the fact that maxima of termwise-$M$-close lists differ by at most $M$. Signed formulas generally lose this homomorphism through cancellation.

#stmt("Theorem", [Cone comparison, coverage and the exact LP])[
#hyp(1)[$Y={y:C y<=b} subset.eq [1,infinity)^k$ is a nonempty closed polyhedron; $K={d:C d<=0}$ and $D_R={2^y:y in Y}$.]
#hyp(2)[For integer completeness, the specified $D subset.eq D_R inter NN^k$ shadows every escaping ray: for each $y_0 in Y$, nonzero $d in K$, there exist $x_j in D$, $t_j->infinity$ and bounded $e_j$ with $log_2 x_j=y_0+t_j d+e_j$.]
#enum(numbering: "(a)")[For positive polynomials $P,Q$ with supports $A,B$,
$ P in O_(D_R)(Q) <==> h_A|_K<=h_B|_K. $
The same iff holds on $D$ under (H2); a real Yes always restricts. Mutual comparison is equality of restricted support functions.][For rational $C,A,B$, test each $a in A$ by
$ "maximize " delta: C d<=0, quad sum_i d_i=1, quad delta<=(a-b')dot d quad(b' in B). $
A positive optimum gives No; all nonpositive optima give Yes. The empty direction set means $K={0}$ and bounded $Y$.][The restricted support function is canonical, not its exponent list. A term is redundant exactly when its support is dominated by the remaining nonempty support on $K$, tested by the same LP.]
] <t-newton>
#proof[
#step(1, [Escaping rays])[
Every $d in K$ is nonnegative, since $y_0+t d in Y subset.eq[1,infinity)^k$ for all $t>=0$. If $delta=h_(A)(d)-h_(B)(d)>0$, the uniform estimate with fixed offset $y_0$ gives
$ P(2^(y_0+t d))/Q(2^(y_0+t d))>=c 2^(t delta)->infinity. $
A bounded log perturbation changes each monomial by a bounded factor, uniformly over its finite support. Thus a shadow preserves divergence and escapes as well. This proves necessity in both cases.]
#step(2, [Sufficiency, including cone boundaries])[
Use the exact Farkas/LP alternative [BV04, Chapter 5]: on a nonempty polyhedron $Z={y:M y<=v}$, $ell dot y$ is bounded above iff $ell=M^T z$ for some $z>=0$, equivalently iff $ell dot d<=0$ whenever $M d<=0$.

Partition $Y$ into nonempty closed cells $Y_(b')$ on which $b'$ maximizes $B$. Their recession cones are $K inter {d:(b''-b')dot d<=0, b'' in B}$. The support inequality makes $(a-b')dot d<=0$ there for every $a$. The alternative supplies finite $M_(a b')$ with $(a-b')dot y<=M_(a b')$ on the cell. Therefore
$ P(2^y)<=sum_a c_a 2^(M_(a b'))2^(b' dot y)<=C_(b')Q(2^y). $
Taking the maximum of finitely many $C_(b')$ proves a global bound, which restricts to $D$.]
#step(3, [LP and canonical form])[
Nonzero cone vectors normalize into the simplex. On that compact set, $min_(b' in B)(a-b')dot d$ is continuous and attains its maximum; a positive value is exactly a support violation. If $K={0}$, applying the alternative to each coordinate bounds $Y$.

Equality of supports follows by two comparisons. A convex combination cannot exceed the maximum of its constituent linear forms, so Newton vertices suffice. Deleting a term preserves the class exactly when its linear form lies below the remaining maximum. Delete sequentially: on the diagonal cone the distinct singleton exponents $(1,0)$ and $(0,1)$ coincide, so simultaneous deletion can erase every term.]
]

A rational Yes certificate can give, for each tested $a$, weights $lambda_b>=0$, $sum_b lambda_b=1$, and $z>=0$ such that $a-sum_b lambda_b b=C^T z$. Dotting with $d in K$ verifies the inequality. A rational positive-gap direction certifies No on the real domain; integer No still needs coverage. Rational LP feasibility and bounded optimization are polynomial in explicit input bit length [K79]. There are $|A|$ LPs of $k+1$ variables and $O("rows"(C)+|B|)$ constraints, not a bound on expanded path size.

Declared constraints can add comparisons: on $2<=m<=n$, $n m<=n^2$, while $n^2/(n m)->infinity$ along $m=2$. On the full box they are incomparable. Round the real $n$ upward and $m$ downward to verify integer shadowing for this constrained domain. Arbitrary polynomial constraints need not have any such log-polyhedral coverage.

= Signed positivity and tight envelopes <sec-certificates>
#overview[Upper bounds alone do not decide comparison; denominator lower evidence is essential.]

Write $f=P-Q$, where $P=sum_i c_i x^(a_i)$ and $Q=sum_j d_j x^(b_j)$ have disjoint supports and strictly positive coefficients. Either part may be empty. The universal upper bound is $f<=P$. For asymptotic use, establish $f>=epsilon G>=0$ eventually, with a positive polynomial $G$ and $epsilon>0$. Comparison $P in O_(D)(G)$ then proves the desired upper relation. To obtain a complete Newton comparison of original functions, establish *two-sided* tight envelopes for both.

#stmt("Theorem", [Rational positivity, directional absorption and circuit budgets])[
#hyp(1)[All polynomial data and certificate coefficients are rational. Polynomial domain conditions are $s_(j)(x)>=0$ and $r_(l)(x)=0$.]
#enum(numbering: "(a)")[An identity
$ q=sum_J sigma_J product_(j in J)s_j + sum_l h_l r_l $
with each $sigma_J$ an explicitly given rational sum of polynomial squares proves $q>=0$ on the domain. Nonnegative coefficients after a nonnegative variable shift are another such sign certificate. Apply these to $q=g-c f$, partial derivatives or lower-envelope residuals.][For nonempty supports $A,B$, on the polyhedron of @t-newton with rational $C,b$, if
$ h_(A)(d)>h_(B)(d) quad(d in K, sum_i d_i=1), $
then $Q/P->0$ uniformly outside bounded sets. Thus $P-Q approx P$ eventually, with any prescribed strict margin after a computable threshold from the rational LP certificates.][Suppose finitely many verified modes satisfy
$ x^(b_j)<=sum_i kappa_(i j r)x^(a_i), quad kappa_(i j r)>=0. $
If rational allocations $w_(j r)>=0$ and rational $epsilon>0$ satisfy
$ sum_r w_(j r)>=d_j, quad
sum_(j,r)kappa_(i j r)w_(j r)<=(1-epsilon)c_i, $
then globally $epsilon P<=P-Q<=P$. With $epsilon=0$ the same certificate proves nonnegativity only.][Dominance and rational weighted AM–GM provide such modes. If $lambda_i>0$, $sum_i lambda_i=1$, $overline(a)=sum_i lambda_i a_i$, $q_i>0$, and $x^(b-overline(a))<=R$ on the domain, choose rational $kappa>0$ satisfying
$ kappa product_i q_i^(lambda_i)>=R. $
Then a mode is $kappa_i=kappa lambda_i q_i$. All product comparisons are exact after raising to a common denominator of the rational weights.]
] <t-certificates>
#proof[
#step(1, [Sign identities])[
Squares and the constrained $s_j$ are nonnegative, products and nonnegative sums remain so, and all ideal terms $h_l r_l$ vanish. Verify the displayed identity by sparse rational expansion. Shifted coefficient checks apply because each shifted variable is nonnegative. For $q=g-c f>=0$ with $c>0$, divide by $c$ to obtain $f<=g/c$; identities are a special case.]
#step(2, [Compact directional gap and dual witnesses])[
For nonzero $K$, its normalized simplex is compact. The continuous positive gap therefore has minimum $delta_0>0$. For each $b' in B$, minimize $z$ with
$ C d<=0, quad sum_i d_i=1, quad z>=(a-b')dot d quad(a in A). $
All optima are positive. Explicit dual witnesses are rational $lambda_a>=0$, $sum_a lambda_a=1$, $mu>=0$, $delta>0$ with
$ sum_a lambda_(a)(a-b')+C^T mu=delta bold(1). $
Dotting with a normalized feasible $d$ shows that its convex average of the gaps is at least $delta$. LP duality gives such witnesses; use a common rational $0<delta<delta_0$ for the finitely many $b'$.

Partition $Y$ into cells on which a fixed $a$ maximizes $A$. For every recession vector $d$ of such a cell,
$ (b'-a+delta bold(1))dot d<=0. $
The Farkas alternative supplies nonnegative rational multipliers proving
$ (b'-a)dot y<=M_(a b')-delta sum_i y_i. $
Set $M=max M_(a b')$. Then, with $c_*=min c_a$ and $D_0=sum d_b$,
$ Q(2^y)/P(2^y)<=D_0/c_* 2^M 2^(-delta sum_i y_i). $
The sum of logarithmic coordinates tends uniformly to infinity with radius, including bounded-variable directions. The bound gives a threshold for any desired error. If $K={0}$, $Y$ is bounded and the eventual claim is vacuous. For rational $M,delta$, raising to their denominators makes the threshold check rational integer arithmetic.]
#step(3, [Budget summation])[
Multiply each mode by $w_(j r)$ and sum. Nonnegativity and the column coverage give
$ Q=sum_j d_j x^(b_j)<=sum_(j,r)w_(j r)x^(b_j)
 <=sum_i (sum_(j,r)kappa_(i j r)w_(j r))x^(a_i)<=(1-epsilon)P. $
Subtract from $P$; $Q>=0$ gives the other inequality. For a finite catalogue, maximizing $epsilon$ under these constraints is a rational LP. A positive feasible margin, rather than a floating optimum alone, certifies tightness.]
#step(4, [AM–GM and constraint factors])[
For positive $t_i$, put $S=sum_i lambda_i t_i$. The elementary inequality $log u<=u-1$ follows by integrating $1-1/u$ from $1$ to $u$; the integral is nonnegative on either side of $1$. Consequently
$ sum_i lambda_i log(t_i/S)<=sum_i lambda_(i)(t_i/S-1)=0, $
so $product_i t_i^(lambda_i)<=S$. Substitute $t_i=q_i x^(a_i)$ and multiply by $kappa$ to obtain the stated mode.

For box dominance $a>=b$, take $x^b<=2^(sum_(i)(b_i-a_i))x^a$. For monomial constraints, use $b-overline(a)=C^T z$, $z>=0$: then $x^(b-overline(a))<=2^(z dot b_0)$, where $b_0$ is the log-polyhedron's right side. A rational $R$ above this factor is verified by integer powers when $z dot b_0$ is rational. More generally constraints $x^(v_l)<=r_l$ with rational $r_l>0$ yield products of rational powers, checked the same way. The exponent identity and nonnegative multipliers are part of the proof object.]
]

These are positive-orthant circuit certificates, related to SONC [IDW16], not a claim that every nonnegative polynomial has such a decomposition. For $n^2+m^2-n m$, use $n m<=(n^2+m^2)/2$ to get margin $1/2$. For $n^2+m^2-2n m$, the same mode saturates the budget and yields only $(n-m)^2>=0$. A strict margin cannot be replaced by zero.

The directional test must include boundary directions: $P=n m$, $Q=2n$ has gap $d_m$, positive in the simplex interior but zero at $(1,0)$; $P-Q$ vanishes on the unbounded slice $m=2$. Declared cone constraints can make a strict gap possible, but do not remove the need to verify their real enclosure and integer coverage.

A rational positivity identity proves a global interpolation-domain claim, even at parameters $0,1$. Eventual absorption cannot certify such a claim by itself. For integer asymptotic domains its excluded region is finite and must be separately checked if global nonnegativity is required; for real domains use a separate bounded-region certificate or quantifier elimination.

= Refutation and complete polynomial fragments <sec-completeness>
#overview[Integer witnesses detect cancellation; real decision procedures need an explicit transfer argument.]

#stmt("Theorem", [Polynomial witness curves and initial forms])[
#hyp(1)[$gamma_(i)(t) in QQ[t]$ take integer values on a specified infinite arithmetic progression, eventually satisfy $gamma(t) in D$, and escape bounded sets. Domain membership is certified by exact substitution into every required polynomial condition.]
#hyp(2)[$f,g$ are finite nonnegative on $D$. Put $F=f compose gamma$, $G=g compose gamma$.]
#enum(numbering: "(a)")[If $F$ has positive leading coefficient and either $G=0$ or $deg F>deg G$, then $f in.not O_(D)(g)$. When $G!=0$, its leading coefficient is positive by (H2).][For a monomial curve $x_i=c_i t^(e_i)$, let $w=max_a a dot e$. The leading candidate coefficient is
$ "in"_(e)(f)(c)=sum_(a:a dot e=w)f_a c^a. $
If this vanishes, combine terms at the next exponent level; repeating computes the exact univariate leading term. Newton support alone does not detect this cancellation.]
] <t-curves>
#proof[
#step(1, [Exact verification of the curve])[
Substitution yields univariate rational polynomials. A nonzero polynomial $p(t)=a_d t^d+sum_(i<d)a_i t^i$ has the sign of $a_d$ once
$ t>1+sum_(i<d)abs(a_i/a_d), $
because the sum of lower absolute terms is smaller than $abs(a_d)t^d$. Equalities must vanish identically; strict or nonstrict inequalities use this sign test, with zero allowed only for the latter. Check a feasible Boolean branch of domain conditions.

Integrality needs a separate exact check: for example, substitute $t=M u+r$ and verify that every resulting coefficient is integral. A nonintegral constant cannot be repaired by taking powers. One positive-degree coordinate proves escape.]
#step(2, [Refutation])[
If $G=0$, eventually $F>0$ violates every upper constant. Otherwise leading-term bounds give $F/G>=c t^(deg F-deg G)->infinity$ on the progression. Together with escape this violates every proposed pair of constant and threshold.]
#step(3, [Initial-form calculation])[
Each monomial becomes $f_a c^a t^(a dot e)$. Group equal powers exactly. The largest surviving coefficient is the univariate leading coefficient, proving (b). Rational exponents can be made integer by a common power substitution, but integrality of constant coordinates and the other coefficients must still be checked.]
]

Examples include $g=(m-n)^2$ along $(t,t)$ and $g=(2m-n^2)^2$ along $(2t,2t^2)$: with $f=1$, both give No. For the first polynomial the initial coefficient is $(c_m-c_n)^2$, vanishing exactly on the positive diagonal. A negative leading value instead refutes a claimed nonnegative certificate; it is not a valid $O_D$ verdict until the domain or certificate is repaired.

#stmt("Theorem", [Complete subclasses])[
#hyp(1)[All comparisons have established nonnegative functions on their specified integer domains.]
#enum(numbering: "(a)")[*Tight positive envelopes.* If each field is zero or has proved eventual bounds $epsilon P<=f<=C P$ with a nonempty positive polynomial $P$, on an admissible covered domain, @t-newton decides every pair. This includes strict absorption and positive-margin budget/circuit certificates.][*Shifted-face boxes.* On $NN_(>=2)^k$, if each field's expansion $f(z+2)$ has nonnegative coefficients, comparisons are complete by at most $2^k-1$ face tests. Ordinary positive polynomials and tight positive representatives can be mixed with this class.][*Finite partitions.* The same conclusion holds when a finite, verified cover partitions the domain into cells, and on each unbounded cell every field is zero or has a tight positive representative with a complete covered-domain test.][*One variable.* Rational polynomials nonnegative on a univariate domain given by rational polynomial conditions intersected with integers admit complete comparison.]
] <t-fragments>
#proof[
#step(1, [Tight representatives])[
From $epsilon P<=f<=C P$ and $eta Q<=g<=M Q$ beyond a common threshold, $P<=c Q$ implies $f<=C c g/eta$. Conversely $f<=c g$ implies $P<=c M Q/epsilon$. Thus $f in O_(D)(g)$ iff $P in O_(D)(Q)$. A zero numerator always compares; a positive representative is not $O_(D)(0)$ on an unbounded domain.]
#step(2, [Integer faces of shifted variables])[
Set $z_i=x_i-2$. Every integer $z_i$ is either $0$ or at least $1$. For each nonempty free-index set $J$, set other coordinates to zero and keep $z_j>=1$. Kill monomials containing fixed zero coordinates. Each remaining polynomial is zero or a positive sum on that face.

The transformation $z_j |-> x_j=z_j+2$ preserves escape. For a positive representative originally in $x$, substitute fixed $x_i=2$; its free variables $z_j+2$ satisfy $z_j<=z_j+2<=3z_j$, so the support growth is unchanged. Apply the full-box case of @t-newton on $z_j>=1$ (equivalently rescale its box; the proof is identical). Integer monomial directions realize every rational separating cone direction. The empty free set is one bounded point.]
#step(3, [Finite covers])[
A global comparison restricts to every cell. Conversely, take the largest of finitely many valid constants and thresholds to combine all cell comparisons. Bounded cells can be excluded by one threshold. Thus a failed unbounded-cell test refutes the global comparison, and successful tests prove it. Steps 1 and 2 supply the claimed special cases.]
#step(4, [Univariate domains])[
For every nonzero atomic polynomial, the coefficient bound in @t-curves gives a rational bound beyond which its sign is fixed and it has no roots. Past the maximum of these bounds, every Boolean domain formula is constant. The integer domain therefore has either a full tail or no tail. In the latter case comparison is vacuous; in the former, a nonzero nonnegative polynomial has positive leading coefficient. Two nonzero such polynomials compare exactly by degree. Zero cases are as in Step 1. Every step is finite rational arithmetic.]
]

For shifted faces, $n-2 approx n$ in one variable, but not on the two-variable box: the face $n=2$, $m->infinity$ distinguishes them. Finite integer partitions extend the class: $(m-5/2)^2$ is at least $1/4$ on integers; splitting $m=2$ from $m>=3$ yields positive shifted expansions on both cells. No corresponding unrestricted real conclusion follows.

#stmt("Theorem", [Real fallback and the integer boundary])[
#hyp(1)[$D_R$ is rational semialgebraic and $f,g$ are rational polynomials; nonnegativity and definedness are checked on the chosen domain.]
#enum(numbering: "(a)")[Real quantifier elimination decides $f in O_(D_R)(g)$ completely via
$ exists c>0, exists N>=0, forall x:
 (x in D_R and max_i x_i>=N) ==> f(x)<=c g(x). $
A Yes transfers to every integer subset. A No transfers only with integer refutation evidence, for example @t-curves, ray coverage for tight positive representatives, or the rounding condition below.][If an escaping real witness $x(t)$ has feasible integer shadows $z(t)$ with bounded additive error, and along the connecting segments
$ norm(z-x)_infinity sup norm(nabla f)_1<=f(x)/2, $
$ norm(z-x)_infinity sup norm(nabla g)_1<=M g(x) $
for fixed $M$, then its unbounded ratio transfers to the integer shadows. For $g(x)=0$, the second condition requires exact zero preservation.][No sound terminating procedure decides every integer-box polynomial comparison, even for $f=1$ and a denominator that is a sum of squares of quadratic polynomials, in unbounded dimension.]
] <t-integer-boundary>
#proof[
#step(1, [Real decision and witness shape])[
Effective Tarski–Seidenberg elimination turns any first-order formula over a real closed field with rational coefficients into an equivalent quantifier-free formula [B14, Theorem 2.1]. Replace the maximum by a finite disjunction to obtain (a); the same theorem checks signs. Restriction proves transfer of Yes.

Curve selection says that every point in the closure of a semialgebraic set is reached by a continuous semialgebraic arc lying in that set away from its endpoint [BR21]. Apply it after bounded coordinate compactification to the set of $(u,x)$ with $u>0$, $r(x)>u$, $f(x)>u g(x)$. A real No makes its $u$-projection unbounded; a compactified closure point with $1/u=0$ gives an escaping real witness with arbitrarily large ratio. The arc need not be polynomial, rational, or integer-valued.]
#step(2, [Quantitative rounding])[
Integrating gradients along a segment gives
$ abs(f(z)-f(x))<=norm(z-x)_infinity sup norm(nabla f)_1, $
and likewise for $g$. Hence $f(z)>=f(x)/2$ and $g(z)<=(1+M)g(x)$. The ratio still diverges, or the denominator remains zero; bounded displacement preserves escape. Mere density or bounded log error does not imply these inequalities for a cancelling polynomial.]
#step(3, [Undecidability already for quartic sums of squares])[
Matiyasevich's theorem implies that no algorithm decides natural-number zeros of all integer polynomials [M70]. Write such an equation as equality of its positive and negative coefficient parts. Arithmetic circuits for these parts use only addition, multiplication and nonnegative integer constants. Introduce a nonnegative integer variable for every gate, and a quadratic or linear equation for each gate and for equality of the outputs.

This finite system $q_(i)(u)=0$ has a natural-number solution exactly when the original equation does. Set $a=u+2$, add a free integer coordinate $t>=2$, and put
$ f(a,t)=1, quad g(a,t)=sum_i q_(i)(a-2)^2. $
The denominator is a nonnegative quartic sum of squares. Without a solution it is an integer at least one everywhere, so $f<=g$. With a solution, its fixed $a$ and arbitrary $t$ give an unbounded zero slice, refuting comparison. A complete comparison algorithm would decide the original solvability problem, a contradiction.]
]

The real/integer obstruction is explicit: for $f=1$, $g=(m-5/2)^2$ on $n,m>=2$, integer comparison holds with constant $4$, while the real slice $m=5/2$ gives No. Even full-box integer points do not automatically shadow a real cancellation witness in the required quantitative sense.

The proved subclasses are sufficient and effectively recognizable when their certificates or finite partitions are supplied. The budget LP is complete for its finite mode catalogue, not for all nonnegative polynomials or all SONC decompositions. No general two-variable signed-integer completeness theorem is claimed. The quartic obstruction uses growing dimension and does not settle fixed dimension. Finding useful broader subclasses and effective integer witnesses remains open; “generic coefficients” without a quantitative uniform noncancellation condition is not such a theorem.

= Certified decisions and Pareto search <sec-search>
#overview[Unknown preserves candidates; exact composition does not imply safe prefix pruning.]

#stmt("Definition", [Layered comparison and proof objects])[
For each direction $f in O_(D)(g)$, return *Yes* only with a verified universal certificate, *No* only with a verified integer obstruction, and *Unknown* otherwise. First establish nonnegativity on the query domain; a sign failure requests a corrected domain or certificate.

Run these layers, retaining exact expressions throughout:
+ Expand rationally; test identity, zero cases and certificates for $c g-f>=0$.
+ Generate upper and lower envelopes using rational sign identities, shifts, strict absorption, coefficient budgets and circuit modes.
+ Compare tight representatives by cone LP or finite shifted-face/partition tests.
+ Search integer polynomial curves and verify leading terms after cancellation.
+ Use real quantifier elimination. Accept its Yes; convert No only with integer coverage or an independently verified integer witness. Otherwise return Unknown with the failed obligations.

Proof objects are sparse identities and rational square lists; rational feasible budget/dual multipliers; support-separating directions plus integer coverage; or curves, progression data and univariate coefficients. Every verifier checks identities and inequalities exactly. Finite numerical samples are not proofs.
] <d-procedure>

Every Yes layer is sound by @t-certificates, @t-newton or @t-fragments and transitivity. Every curve No is sound by @t-curves; real No without transfer is not accepted. Since the procedure performs no other definitive step, induction over its sequence of tests proves soundness. Complete fragments terminate at their designated finite layer; resource-bounded certificate searches may instead stop with Unknown.

For a profile, combine directional field decisions by strong Kleene conjunction: all Yes gives Yes, any No gives No, otherwise Unknown. For the two resulting directions use
#block(breakable: false)[
#table(columns: (1fr,1fr,2fr),
 table.header([$p prec.eq q$],[$q prec.eq p$],[*Profile verdict*]),
 [Yes],[Yes],[Equivalent],
 [Yes],[No],[Dominated: $q$ is dominated by $p$],
 [No],[Yes],[Dominated: $p$ is dominated by $q$],
 [No],[No],[Incomparable],
 [Any other pair],[—],[Unknown, retaining the proved one-sided information],
)
]
All Yes fields prove the product relation, and one No disproves it, proving soundness of this table. Incomparable is a mathematical result, not a synonym for Unknown. Exact/upper grades do not change guarantee verdicts; an upper grade prevents interpreting them as actual output comparisons without tightness.

#stmt("Theorem", [Sufficient signed-prefix pruning conditions])[
#hyp(1)[Two prefixes have nonnegative certificate vectors $u,v$ on the same source domain, with $u prec.eq v$. Full state includes accuracy, selected formula policy, promises, remaining hops, and visited vertices for simple paths. Every discarded continuation remains eligible from the retained prefix.]
#hyp(2)[Every suffix update has a proved uniform pair-order law on all its admissible certificate pairs. A sufficient law for a fixed update $T$ is isotonicity together with global scaling control
$ T(lambda y)<=C_lambda T(y) quad(lambda>=1), $
on an extension containing $u,v,lambda v$. Alternatively, global two-sided bounds $a G<=T<=b G$ for a positive polynomial map $G$ on every evaluated pair suffice.]
#enum(numbering: "(a)")[Every same-suffix completion through the retained prefix is no worse in guarantee mode. Optimal growth classes are preserved.][Derivative positivity alone proves neither scaling control nor safe $Theta$-pruning. Local eventual tightness on the target domain is insufficient if prefix values can stay in its exceptional set.]
] <t-pruning>
#proof[
#step(1, [One extension])[
Choose one $c>=1$ and source threshold with $u<=c v$ in all fields. Isotonicity and scaling give
$ T(u)<=T(c v)<=C_c T(v). $
For tight positive representatives of maximal total degree $d$ per field, use
$ T(u)<=b G(u)<=b c^d G(v)<=b c^d T(v)/a. $
All bounds must hold at these actual evaluation vectors, not only on an escaping target tail. Accuracy-dependent hull choices must therefore be part of the fixed update policy or separately compared.]
#step(2, [Suffixes and state])[
Induct over suffix length using Step 1 and preserved eligibility. A retained prefix using no more hops preserves a hop budget; a subset of visited vertices preserves simple-path eligibility. Other promises need their own inclusion. If a discarded completion was optimal, its no-worse replacement cannot be strictly better and must be equivalent. Transitivity preserves coverage through repeated deletions.]
#step(3, [A polynomial obstruction])[
On $y,z>=2$, $T(y,z)=(y-2)z$ is nonnegative and isotone: its derivatives are $z$ and $y-2$. Yet prefixes $u(n)=(3,n)$ and $v(n)=(2,n^2)$ satisfy $u prec v$, while
$ T(u(n))=n, quad T(v(n))=0. $
Thus strict pruning fails without the pair-order/scaling condition. At $y=2$ scaling also directly fails. All formulas in this counterexample are polynomials.]
]

#stmt("Counterexample", [Scaling alone does not permit pruning])[
The nonnegative homogeneous polynomial $T(y,z)=(y-2z)^2$ has exact scaling $T(lambda y,lambda z)=lambda^2 T(y,z)$. Pointwise ordered, equivalent prefixes $u(n)=(n,n)$ and $v(n)=(2n,n)$ give $T(u(n))=n^2$ and $T(v(n))=0$. The missing condition is isotonicity; hence both alternatives in @t-pruning require their full hypotheses.
] <x-scaling>

The conservative algorithm enumerates full paths through unsafe states. It may prune at states where *all* remaining suffixes have verified order-preserving updates; mere signed-polynomial membership is insufficient. The implemented comparator enumerates full paths, avoiding reliance on unverified prefix tests.

#stmt("Theorem", [Finite-front correctness and completeness])[
#hyp(1)[The graph is finite, the hop bound is $H$, and semantic eligibility/state checks are exact and terminating. Comparison certificates are sound. Only proved strictly dominated paths are removed; equivalent paths may share one representative with provenance.]
#enum(numbering: "(a)")[All removals are safe: every original Pareto-optimal class remains represented. With Unknown pairs the result is a candidate front, not necessarily a true front.][If every profile comparison belongs to a complete fragment, the resulting equivalence classes are exactly the Pareto-minimal classes.][The same conclusions hold with prefix pruning under @t-pruning.]
] <t-search>
#proof[
#step(1, [Finite coverage])[
There are finitely many edge sequences of length at most $H$. A strict dominator cannot itself be removed along a cycle: transitivity would contradict strictness. Following successive dominators therefore ends at a retained class. An optimal class has no strict dominator, and equivalence merging preserves a representative. Prefix deletions preserve this same coverage by @t-pruning.]
#step(2, [Exactness when comparisons are complete])[
If a returned class $p$ were dominated by an original $q$, coverage would give retained $r prec.eq q prec p$. Were $p prec.eq r$, transitivity would contradict $q prec p$. Thus $r$ strictly dominates $p$, which complete final filtering would remove. Conversely Step 1 preserves every optimal class. With Unknown, this argument lacks a certified final deletion, so only candidate coverage is asserted.]
]

#raw("Initialize the exact source identity label at hop 0.
For h = 0,...,H-1:
    extend every eligible retained label by each allowed edge
    expand exactly; propagate exact/upper grades by Theorem 2.1
    retain signed formulas and all semantic/domain obligations
    prune a prefix only with a suffix-closure and eligibility proof
Gather terminal-eligible target paths from all layers.
Compare both directions, field by field, with the layered procedure.
Delete certified strictly dominated paths; group proved equivalents.
Return witnesses, proof objects, and every unresolved pair/reason.", block: true)

With outdegree $Delta$, at most $sum_(h=0)^H Delta^h$ paths start at a fixed source. A group of $M$ terminal paths needs at most $M(M-1)/2$ paired comparisons. A budget catalogue with $R$ modes, $p$ positive terms and $q$ negative terms has $R+1$ variables and $p+q+1$ inequalities. Rational LP verification is polynomial in certificate bit length; mode discovery, $2^k$ shifted faces, symbolic expansion and real quantifier elimination have no polynomial global guarantee here.

Exponential output is unavoidable: retain variables $x_1,dots,x_h>=2$ and at stage $i$ multiply one of two accumulators by $x_i$. Subset $S$ gives $(product_(i in S)x_i,product_(i in.not S)x_i)$. The first field for $S$ is $O$ of that for $T$ iff $S subset.eq T$ (grow a missing variable for necessity); the second requires $T subset.eq S$. Hence the $2^h$ classes are pairwise incomparable. Caps preserve individual certificate validity but may omit an optimum or its dominator; report incompleteness explicitly.

= Generic comparison relative to a fixed batch <sec-generic>
#overview[The implemented comparator uses this order; worst-case comparison remains theoretical background.]

This notion is inspired by the distinction between worst-case and generic-case
complexity in [KMSS03]; its family measures are specified here independently of
that paper's counting-density model. Fix a finite batch of paths sharing one real
source domain $D$. Let $S$ be the set of all nonzero chosen polynomial certificates
of all fields of all paths in that batch. Identically zero fields are allowed.

#stmt("Definition", [Strata, cells, regularity and the generic order])[
A family is $phi_i(t)=c_i t^(e_i)$ with real $c_i>=2$, real $e_i>=0$, and
$e!=0$. Each nonempty growing-coordinate set $I={i:e_i>0}$ is its own stratum.
Within it take the common refinement of the normal fans of the Newton polytopes
of $S$, restricted to that stratum. Every relative-interior cell has its own
Lebesgue measure on $(c,e)$ in its affine span, including lower-dimensional
exponent cells. Bounded coordinates retain their full scale-coordinate measure.

For $p!=0$, set $h_p(e)=max_(a in "supp" p)a dot e$ and
$p_e(c)=sum_(a:a dot e=h_p(e))p_a c^a$; set $h_0=-infinity$.
A family is *regular* if $p_e(c)!=0$ for every $p in S$, and *valid* if all these
values are strictly positive. Zero certificates impose no condition.
Write $f prec.eq_("gen",S) g$ if $h_f(e)<=h_g(e)$ for almost every valid family
in every cell of every stratum. Empty valid sets give vacuous comparisons.
] <d-generic>

#stmt("Theorem", [Regular families, probability bounds and the batch preorder])[
#enum(numbering: "(a)")[Non-regular families are null in each cell. On a regular
family, $p(phi(t))=p_e(c)t^(h_p(e))(1+o(1))$.][On a valid family, eventual
boundedness of $f/g$ is equivalent to $h_f<=h_g$, interpreting a zero numerator
as bounded and a nonzero numerator against zero as unbounded. Almost-everywhere
boundedness is equivalent to the following: for every probability measure
equivalent to Lebesgue on the valid part of a cell and every $epsilon>0$, there
exist $C>0,N$ such that $f(phi(t))<=C g(phi(t))$ for all $t>=N$ outside a set of
families of probability less than $epsilon$. The same statement holds for any
one such measure; no distribution is selected by the order.][For fixed $S$ the
order is reflexive and transitive, and extends fieldwise to profiles. Verdicts
are relative to the batch; adding paths may shrink its valid set.]
] <t-generic-order>
#proof[
Within a cell each exposed face is fixed. Its face polynomial is a nonzero
polynomial in the independent scale coordinates: distinct monomials cannot
cancel identically. The zero set of a nonzero polynomial is Lebesgue-null,
by induction on the number of coordinates and Fubini's theorem, using finitely
many roots in one variable. A finite union over $S$ remains null. Integrating
over the exponent cell preserves nullity. Grouping powers gives the claimed
asymptotic expansion; its surviving lower powers have strictly smaller exponent.

For nonzero valid $f,g$, the ratio is asymptotic to
$(f_e(c)/g_e(c))t^(h_f-h_g)$. This has a finite limit precisely when $h_f<=h_g$.
The stated zero conventions give the same equivalence for zero fields.
Let $E_j$ be the measurable set of valid families on which $f(phi(t))<=j g(phi(t))$
for every $t>=j$. Measurability follows by checking rational $t$ and continuity.
The sets increase, and their union is exactly the families with finite limiting
ratio, including the zero conventions. Continuity of probability from below
therefore gives a uniform $C=N=j$ outside an arbitrarily small exceptional set.
Conversely, every family with unbounded ratio is outside every such bounded set;
letting $epsilon$ tend to zero makes their measure zero. Equivalence to Lebesgue
transfers precisely the null sets. This is boundedness in probability, not a
uniform bound on every valid family, nor an average-cost claim.

Reflexivity is the pointwise inequality $h_f<=h_f$. For transitivity, discard
the two exceptional null sets in each cell; on the remaining *common* valid set,
$h_f<=h_g<=h_h$. Extended-real order includes $-infinity$. Refine cells when
needed; the batch fan is fixed. Finite conjunction over fields uses the same
finite-union argument. Changing $S$ changes the quantified set; comparisons
proved under different batches cannot be chained without checking that scope.
]

#stmt("Theorem", [Generic certificates and regular refutations])[
#enum(numbering: "(a)")[An eventual worst-case inequality proved throughout the
real box implies the generic inequality. Identity and zero are direct generic certificates. The former worst-case Newton
certificates also imply it on every valid family. A worst-case No alone has no
such implication.][Positive-hull Newton coverage over the whole nonnegative
direction cone proves $f prec.eq_("gen",S) g$ without denominator lower
tightness.][A regular valid family with $h_f>h_g$ proves generic No. Its positive
face values and strict height gap persist on a positive-measure relative
neighbourhood in its cell. The checker must test every member of $S$, including
certificates absent from the compared field.]
] <t-generic-certificates>
#proof[
A box inequality holds along every escaping family eventually. Identity handles
equal signed formulas; zero is bounded by every eventually nonnegative valid
denominator. Newton's envelope inequality and its verified lower bound hold on
the real box, hence along these families.

If $p_e(c)>0$, its exposed face contains a positive coefficient, since $c_i>0$.
Thus $h_p(e)=h_(p^+)(e)$ on valid families. Coverage @t-newton gives
$h_(f^+)(e)<=h_(g^+)(e)$ on the whole cone and therefore $h_f<=h_g$ on valid
families. If $f=0$ the result is immediate; positive-hull coverage of a nonzero
numerator by a zero denominator is impossible.

Within a relative-interior cell the faces are fixed. Their values are continuous
polynomials in $c$, and the height difference is linear in $e$. Strict positivity
of every face and the strict gap are open relative to that cell and the scale
orthant. A neighbourhood of the witness therefore has positive product measure
and unbounded ratio throughout. For $g=0$ and nonzero $f$, the strict extended
height gap is automatic; both zero cannot refute. Actual leading coefficients
after cancellation do not suffice: regularity concerns the *top* face before
cancellation. Rational scales and exponents can be checked exactly, with integer
exponents obtained by a common power substitution. The V1 search uses integer
scales and exponents and is intentionally incomplete.
]

#stmt("Example", [Exceptional failures and bounded strata])[
For $f=n$, $g=(n-m)^2+1$, the curve $n=m=2t$ gives $f=2t$, $g=1$ and a valid
worst-case No. It is non-regular because the top face of $g$ vanishes. Outside
$c_n=c_m$ in the cell $e_n=e_m$, the denominator has quadratic growth; the other
cells do as well. Hence generic Yes and worst-case No coexist.

For the complement-edge certificate $g=n(n-1)/2-m$, $f=n$ has generic Yes by
coverage $n<=_h n^2$, which the implemented comparator certifies. Worst-case
comparison still cannot ignore complete graphs. Complete
graphs have $m=n(n-1)/2$ and $g=0$; their two-term parameter curve is not a
single monomial family, and cannot be ignored by worst-case comparison.
For $f=n m$, $g=n^2$, the stratum $e_n=0,e_m>0$ gives a regular generic No:
take $n=2,m=2t$. Keeping the exponent equality cell and every bounded-variable
stratum is essential.

For $(f,g,h)=(n^2,0,n)$, $f prec.eq_("gen",S) g$ is false on every valid family,
while $g prec.eq_("gen",S) h$ is true. Zero fields do not empty the valid set.
Batch dependence is visible with $f=m,h=n$: alone they are generically
incomparable; adding $p=n-m$ excludes families where $m$ grows faster than $n$,
and the actual batch-relative relation becomes $m prec.eq_("gen",S) n$.
The conservative whole-cone coverage algorithm may report Unknown there.
] <x-generic>

The implemented comparator reports only the generic verdict; it is not a
worst-case guarantee. Algebraic scale coincidences
may be null, but structured inputs can lie entirely on them. Sparse source
graphs have nearly complete complements; complete source graphs have empty
complements. Neither structured family is justified as ignorable by a generic
verdict. Lower-dimensional exponent cells and bounded strata are retained, not
thrown away. Regular positivity is only a relaxation of realizability in $D$;
unregistered source promises remain outside the comparator.

V1 returns Unknown when whole-cone coverage fails and its finite deterministic
catalogue finds no regular valid witness, when certificates are unavailable or
non-polynomial, or when exact LP resource limits are reached. Validity is checked
against one fixed batch. Negative leading cells are excluded as invalid rather
than ignored as null. No domain inference, general fan enumeration, SOS, CAD,
Gröbner bases or floating-point solver is added.

= References <sec-references>
#overview[External results are stated precisely where invoked.]

- *[KMSS03]* Ilya Kapovich, Alexei Myasnikov, Paul Schupp and Vladimir Shpilrain.
  _Generic-case complexity, decision problems in group theory, and random walks._
  Journal of Algebra 264(2), 2003, pp. 665–694.
  #link("https://arxiv.org/abs/math/0203239")[Author preprint].

- *[H08]* Rodney R. Howell. _On Asymptotic Notation with Multiple Variables._ Kansas State University Technical Report 2007-4, revised 2008, §2. #link("https://people.cs.ksu.edu/~rhowell/asymptotic.pdf")[Author PDF].
- *[BV04]* Stephen Boyd and Lieven Vandenberghe. _Convex Optimization._ Cambridge University Press, 2004, Chapter 5. #link("https://web.stanford.edu/~boyd/cvxbook/")[Book].
- *[K79]* Leonid G. Khachiyan. _A Polynomial Algorithm in Linear Programming._ Doklady Akademii Nauk SSSR 244(5), 1979, pp. 1093–1096. #link("https://www.mathnet.ru/eng/dan42319")[Journal record].
- *[B14]* Saugata Basu. _Algorithms in Real Algebraic Geometry: A Survey._ arXiv:1409.1534, 2014, Theorem 2.1. #link("https://arxiv.org/pdf/1409.1534")[Author manuscript].
- *[M70]* Yuri V. Matiyasevich. _The Diophantineness of Enumerable Sets._ Doklady Akademii Nauk SSSR 191(2), 1970, pp. 279–282. #link("https://www.mathnet.ru/eng/dan35274")[Journal archive].
- *[IDW16]* Sadik Iliman and Timo de Wolff. _Amoebas, Nonnegative Polynomials and Sums of Squares Supported on Circuits._ Research in the Mathematical Sciences 3, article 9, 2016. #link("https://doi.org/10.1186/s40687-016-0052-2")[Open article]. Our rational positive-orthant AM–GM certificate is proved above; no general SONC completeness theorem is imported.
- *[BR21]* Saugata Basu and Marie-Françoise Roy. _Quantitative Curve Selection Lemma._ arXiv:1803.00505v3, 2021. #link("https://arxiv.org/abs/1803.00505")[Author preprint]. Its curve-selection statement does not promise polynomial or integer arcs.
