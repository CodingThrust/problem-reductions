// Verification Note: NAESatisfiability → SetSplitting (#841)
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show link: set text(blue)
#show: thmrules.with(qed-symbol: $square$)

#let theorem = thmbox("theorem", "Theorem", fill: rgb("#e8f4f8"))
#let proof = thmproof("proof", "Proof")

#align(center)[
  #text(size: 14pt, weight: "bold")[Verification Note: NAESatisfiability $arrow.r$ SetSplitting]

  #v(0.3em)
  #text(size: 10pt, style: "italic")[Issue #link("https://github.com/CodingThrust/problem-reductions/issues/841")[#841]]
]

#v(1em)

== NAESatisfiability $arrow.r$ Set Splitting <sec:naesat-setsplitting>

#theorem[
  Not-All-Equal Satisfiability (NAE-SAT) reduces to Set Splitting via a direct reinterpretation. Each variable becomes a pair of complementary universe elements, each clause becomes a subset, and a NAE-satisfying assignment corresponds exactly to a set splitting (2-coloring of the universe where no subset is monochromatic). Reference: Lovász (1973); Garey & Johnson (1979), SP4.
] <thm:naesat-setsplitting>

#proof[
  _Construction._ Given a NAE-SAT instance with $n$ variables $x_1, dots, x_n$ and $m$ clauses $C_1, dots, C_m$ (each clause is a set of literals, where a literal is $x_i$ or $overline(x_i)$):

  + *Universe.* Create $2n$ elements: for each variable $x_i$, create a positive element $p_i$ (representing $x_i$) and a negative element $q_i$ (representing $overline(x_i)$). The universe is $S = {p_1, q_1, p_2, q_2, dots, p_n, q_n}$.

  + *Complementarity subsets.* For each variable $x_i$ ($i = 1, dots, n$), add the subset ${p_i, q_i}$ to the collection. This forces $p_i$ and $q_i$ into different partition halves, encoding Boolean complementarity.

  + *Clause subsets.* For each clause $C_j$ ($j = 1, dots, m$), create the subset $D_j$ containing the universe element for each literal in $C_j$: if literal $x_i$ appears, include $p_i$; if literal $overline(x_i)$ appears, include $q_i$.

  + Output the Set Splitting instance $(S, cal(C))$ where $|S| = 2n$ and $cal(C)$ contains $n$ complementarity subsets and $m$ clause subsets.

  _Correctness._

  ($arrow.r.double$) Suppose the NAE-SAT instance is satisfiable with assignment $alpha$. Define the partition: $S_1 = {p_i : alpha(x_i) = top} union {q_i : alpha(x_i) = bot}$ and $S_2 = S backslash S_1$. Each complementarity subset ${p_i, q_i}$ has one element in $S_1$ and one in $S_2$ (since exactly one of $x_i, overline(x_i)$ is true). Each clause subset $D_j$ is not monochromatic: since $alpha$ is NAE-satisfying, clause $C_j$ has at least one true literal (element in $S_1$) and at least one false literal (element in $S_2$). $checkmark$

  ($arrow.l.double$) Suppose a set splitting $(S_1, S_2)$ exists. The complementarity subsets force $p_i$ and $q_i$ into different halves for all $i$. Define $alpha(x_i) = top$ if $p_i in S_1$, $alpha(x_i) = bot$ if $p_i in S_2$. This is consistent (each variable gets exactly one value). For each clause $C_j$: the clause subset $D_j$ has elements in both $S_1$ and $S_2$, meaning at least one literal maps to $S_1$ (true under $alpha$) and at least one maps to $S_2$ (false under $alpha$). Thus no clause is all-true or all-false: $alpha$ is NAE-satisfying. $checkmark$

  _Solution extraction._ Given a set splitting $(S_1, S_2)$: for each variable $x_i$, set $x_i = top$ if $p_i in S_1$, $x_i = bot$ if $p_i in S_2$. The complementarity subsets guarantee this is well-defined.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`universe_size`], [$2 dot n$ where $n =$ `num_vars`],
  [`num_subsets`], [$n + m$ where $m =$ `num_clauses`],
)

*Example.* NAE-SAT with $n = 3$ variables ${x_1, x_2, x_3}$ and $m = 2$ clauses:
- $C_1 = (x_1, overline(x_2), x_3)$
- $C_2 = (overline(x_1), x_2, overline(x_3))$

Universe: $S = {p_1, q_1, p_2, q_2, p_3, q_3}$ (6 elements).

Subsets:
- Complementarity: ${p_1, q_1}, {p_2, q_2}, {p_3, q_3}$
- Clause $C_1 = (x_1, overline(x_2), x_3)$: ${p_1, q_2, p_3}$
- Clause $C_2 = (overline(x_1), x_2, overline(x_3))$: ${q_1, p_2, q_3}$

Total: 5 subsets. Overhead: $|S| = 6 = 2 dot 3$, $|cal(C)| = 5 = 3 + 2$. $checkmark$

NAE-satisfying assignment: $x_1 = top, x_2 = top, x_3 = bot$.
- $C_1 = (top, bot, bot)$: not all equal. $checkmark$
- $C_2 = (bot, top, top)$: not all equal. $checkmark$

Partition: $S_1 = {p_1, q_2, p_2, q_3} = {p_1, p_2, q_2, q_3}$... Actually: $S_1 = {p_i : x_i = top} union {q_i : x_i = bot} = {p_1, p_2, q_3}$, $S_2 = {q_1, q_2, p_3}$.
- ${p_1, q_1}$: $p_1 in S_1, q_1 in S_2$. $checkmark$
- ${p_2, q_2}$: $p_2 in S_1, q_2 in S_2$. $checkmark$
- ${p_3, q_3}$: $p_3 in S_2, q_3 in S_1$. $checkmark$
- ${p_1, q_2, p_3}$: $p_1 in S_1, q_2 in S_2, p_3 in S_2$. Not monochromatic. $checkmark$
- ${q_1, p_2, q_3}$: $q_1 in S_2, p_2 in S_1, q_3 in S_1$. Not monochromatic. $checkmark$

= References

+ Garey, M. R. and Johnson, D. S. (1979). _Computers and Intractability._ W.H. Freeman. SP4.
+ Lovász, L. (1973). "Coverings and colourings of hypergraphs." _Proc. 4th Southeastern Conference on Combinatorics_, pp. 3--12.
