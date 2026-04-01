// Verification proof: CircuitSAT -> Satisfiability (Tseitin transformation)
// Standalone document for the verify-reduction pipeline.

#set page(margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

= CircuitSAT $arrow.r$ Satisfiability (Tseitin Transformation)

== Problem Definitions

*CircuitSAT.* Given a boolean circuit $C$ consisting of assignments
${o_i = f_i ("inputs")}$ where each $f_i$ is a boolean expression tree
over gates ${and, or, not, xor}$ and variables,
find a truth assignment to all variables such that every assignment $o_i = f_i$ is satisfied.

*Satisfiability (CNF-SAT).* Given a boolean formula in conjunctive normal form
$F = C_1 and C_2 and dots and C_m$ over variables $x_1, dots, x_n$,
find a truth assignment making all clauses true.

== Construction (Tseitin Transformation)

Given a circuit $C$ with variable set $V$ and assignments $A_1, dots, A_k$
where $A_j: o_j = f_j$, we construct a CNF formula $F$ as follows.

*Step 1: Variable mapping.*
For each circuit variable $v in V$, create a SAT variable $x_v$ with a unique index
(1-indexed). Let $n_0 = |V|$.

*Step 2: Gate decomposition.*
Walk each assignment's expression tree. For each non-leaf subexpression $g$
(i.e., each gate), introduce a fresh SAT variable $x_g$.
Add definitional clauses enforcing $x_g equiv g(x_(a_1), x_(a_2), dots)$:

#table(
  columns: (auto, auto),
  inset: 8pt,
  stroke: 0.5pt,
  [*Gate*], [*Definitional clauses for $v equiv "gate"(a, b)$*],
  [$v = not a$],
  [$( overline(v) or overline(a) ) and ( v or a )$],
  [$v = a and b$],
  [$( overline(v) or a ) and ( overline(v) or b ) and ( v or overline(a) or overline(b) )$],
  [$v = a or b$],
  [$( v or overline(a) ) and ( v or overline(b) ) and ( overline(v) or a or b )$],
  [$v = a xor b$],
  [$( overline(v) or overline(a) or overline(b) ) and ( overline(v) or a or b ) and ( v or overline(a) or b ) and ( v or a or overline(b) )$],
)

For $n$-ary $and, or, xor$ with children $c_1, dots, c_k$ ($k > 2$):
build a balanced binary tree of $(k - 1)$ binary gates with fresh intermediate variables.

*Step 3: Output equivalence.*
For each assignment $o_j = f_j$, let $r_j$ be the SAT variable for the root of $f_j$'s expression tree.
Add equivalence clauses:
$ ( overline(x_(o_j)) or r_j ) and ( x_(o_j) or overline(r_j) ) $

*Step 4: Constant handling.*
For a constant node `true` (resp. `false`), create a fresh variable $x_c$ and add a unit clause
$(x_c)$ (resp. $(overline(x_c))$).

== Correctness

=== Forward direction ($C "satisfiable" arrow.r.double F "satisfiable"$)

Let $alpha: V arrow {0, 1}$ be a satisfying assignment for $C$.
Construct a SAT assignment $beta$ as follows:
- For each circuit variable $v$, set $beta(x_v) = alpha(v)$.
- For each auxiliary gate variable $x_g$, set $beta(x_g)$ to the value obtained
  by evaluating gate $g$ under $alpha$.

Since $alpha$ satisfies the circuit, each assignment $o_j = f_j$ holds,
so $alpha(o_j) = f_j(alpha)$.
The equivalence clauses for output $o_j$ are satisfied because
$beta(x_(o_j)) = alpha(o_j) = f_j(alpha) = beta(r_j)$.

Each definitional clause is satisfied because $beta(x_g)$ equals the gate's
actual output under the input values. For example, for an AND gate
$v = a and b$: if $beta(v) = 1$ then $beta(a) = beta(b) = 1$, satisfying
all three clauses. If $beta(v) = 0$ then at least one of $beta(a), beta(b) = 0$,
satisfying $(v or overline(a) or overline(b))$, and the other two clauses
are satisfied by $overline(v)$ being true.

=== Backward direction ($F "satisfiable" arrow.r.double C "satisfiable"$)

Let $beta$ be a satisfying assignment for $F$.
The definitional clauses for each gate enforce that auxiliary variables
take the correct functional values. Specifically, for an AND gate $v = a and b$:
- Clause $( overline(v) or a )$ forces $beta(v) = 1 arrow.r.double beta(a) = 1$.
- Clause $( overline(v) or b )$ forces $beta(v) = 1 arrow.r.double beta(b) = 1$.
- Clause $( v or overline(a) or overline(b) )$ forces $beta(a) = beta(b) = 1 arrow.r.double beta(v) = 1$.

Thus $beta(v) = beta(a) and beta(b)$. Similar reasoning applies to NOT, OR, XOR gates.

By induction on the expression tree depth, $beta(r_j) = f_j(beta|_V)$ for each root $r_j$.
The equivalence clauses force $beta(x_(o_j)) = beta(r_j) = f_j(beta|_V)$,
so the restriction $alpha = beta|_V$ satisfies every assignment $o_j = f_j$ in $C$.

== Solution Extraction

Given a satisfying assignment $beta$ for $F$, extract the circuit solution:
$ alpha(v) = beta(x_v) quad "for each" v in V $

The first $|V|$ SAT variables (by their mapped indices) correspond directly
to circuit variables. This extraction is valid by the backward direction proof.

== Overhead

#table(
  columns: (auto, auto),
  inset: 8pt,
  stroke: 0.5pt,
  [*Quantity*], [*Bound*],
  [SAT variables],
  [$|V| + G + K$ where $G$ = number of binary gates, $K$ = number of constants],
  [SAT clauses],
  [$<= 4G + 2K + 2|A|$ where $|A|$ = number of assignment outputs],
  [Time complexity],
  [$O(|C|)$ — single pass over the circuit],
)

The reduction is linear in the size of the circuit.
Each NOT gate produces 2 clauses, each AND/OR gate produces 3 clauses,
each XOR gate produces 4 clauses, and each output equivalence produces 2 clauses.

== YES Example

Circuit with 5 variables ($x, y, z, c, d$):
- $c = x and y$
- $d = c or z$

*Satisfying assignment:* $x = 1, y = 1, z = 0, c = 1, d = 1$.

Verification: $c = 1 and 1 = 1$ #sym.checkmark, $d = 1 or 0 = 1$ #sym.checkmark.

After Tseitin transformation, the CNF formula has 7 SAT variables
(5 circuit + 2 gate auxiliary) and is satisfiable.
Extracting the circuit variable values from any SAT solution recovers
a valid circuit assignment.

== NO Example

Circuit with 4 variables ($x, y, c, d$):
- $c = x and y$
- $d = not(c)$
- $c = d$ (forces $c = d$)

This is unsatisfiable: the second assignment requires $d = not(c)$,
but the third assignment requires $c = d$.
Together they imply $c = not(c)$, which is impossible.

After Tseitin transformation, the resulting CNF formula is also unsatisfiable,
confirming that the reduction preserves unsatisfiability.

== Verification Summary

The Python verification script (`verify_circuitsat_satisfiability.py`) performs:
- 14,000+ automated checks across 7 sections
- Forward check: random circuits preserve satisfiability through reduction
- Backward check: every SAT solution extracts to a valid circuit solution
- Overhead verification: variable/clause counts match predicted formulas
- Exhaustive testing for circuits with $n <= 5$ variables

All checks pass with 0 failures.
