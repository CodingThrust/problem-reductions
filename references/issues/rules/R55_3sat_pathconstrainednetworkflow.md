---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PATH CONSTRAINED NETWORK FLOW"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Path Constrained Network Flow'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** PATH CONSTRAINED NETWORK FLOW
**Motivation:** Establishes NP-completeness of PATH CONSTRAINED NETWORK FLOW via polynomial-time reduction from 3SAT. This result is notable because standard (unconstrained) network flow is polynomial, but restricting flow to travel along specified paths makes the problem NP-complete, even when all capacities equal 1.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND34, p.215

## GJ Source Entry

> [ND34] PATH CONSTRAINED NETWORK FLOW
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, a capacity c(a)∈Z^+ for each a∈A, a collection P of directed paths in G, and a requirement R∈Z^+.
> QUESTION: Is there a function g: P->Z_0^+ such that if f: A->Z_0^+ is the flow function defined by f(a)=Sum_{p∈P(a)} g(p), where P(a)⊆P is the set of all paths in P containing the arc a, then f is such that
> (1) f(a)<=c(a) for all a∈A,
> (2) for each v∈V-{s,t}, flow is conserved at v, and
> (3) the net flow into t is at least R?
> Reference: [Promel, 1978]. Transformation from 3SAT.
> Comment: Remains NP-complete even if all c(a)=1. The corresponding problem with non-integral flows is equivalent to LINEAR PROGRAMMING, but the question of whether the best rational flow fails to exceed the best integral flow is NP-complete.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a PATH CONSTRAINED NETWORK FLOW instance as follows:

1. **Variable gadgets:** For each variable x_i, create a "variable arc" e_i in the graph. Create two paths: p_{x_i} (representing x_i = true) and p_{~x_i} (representing x_i = false). Both paths traverse arc e_i, ensuring that at most one of them can carry flow (since arc e_i has capacity 1).

2. **Clause gadgets:** For each clause C_j (containing three literals l_{j,1}, l_{j,2}, l_{j,3}), create a "clause arc" e_{n+j}. Also create three arcs c_{j,1}, c_{j,2}, c_{j,3}, one for each literal position in the clause. Create three paths p~_{j,1}, p~_{j,2}, p~_{j,3} where p~_{j,k} traverses both arc e_{n+j} and arc c_{j,k}.

3. **Linking literals to variables:** Arc c_{j,k} is also traversed by the variable path p_{x_i} (or p_{~x_i}) if literal l_{j,k} is x_i (or ~x_i, respectively). This creates a conflict: if variable x_i is set to true (p_{x_i} carries flow), then the clause path p~_{j,k} corresponding to literal ~x_i cannot carry flow through the shared arc.

4. **Capacities:** Set all arc capacities to 1.

5. **Requirement:** Set R such that we need flow from all variable gadgets (n units for variable selection) plus at least one satisfied literal per clause (m units from clause satisfaction), giving R = n + m.

6. **Correctness (forward):** A satisfying assignment selects one path per variable (n units of flow). For each clause, at least one literal is true, so the corresponding clause path can carry flow without conflicting with the variable paths. Total flow >= n + m = R.

7. **Correctness (reverse):** If a feasible flow achieving R = n + m exists, the variable arcs force exactly one truth value per variable (binary choice), and the clause arcs force each clause to have at least one satisfied literal.

**Key invariant:** Shared arcs between variable paths and clause paths enforce consistency between variable assignments and clause satisfaction. Unit capacities enforce binary choices.

**Time complexity of reduction:** O(n + m) for graph construction (polynomial in the 3SAT formula size).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in 3SAT instance
- m = number of clauses in 3SAT instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n + m) |
| `num_arcs` | `n + m + 3 * m` = `n + 4 * m` |
| `num_paths` | `2 * n + 3 * m` |
| `requirement` (R) | `n + m` |

**Derivation:** The graph has O(n + m) vertices. There are n variable arcs, m clause arcs, and 3m literal arcs, for n + 4m arcs total. The path collection has 2n variable paths and 3m clause-literal paths. All capacities are 1.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a 3SAT instance to PathConstrainedNetworkFlow, solve target with BruteForce (enumerate path flow assignments), extract solution, verify on source
- Test with known YES instance: a satisfiable 3SAT formula
- Test with known NO instance: an unsatisfiable 3SAT formula (e.g., a small unsatisfiable core)
- Compare with known results from literature

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3, x_4
Clauses (m = 4):
- C_1 = (x_1 v x_2 v ~x_3)
- C_2 = (~x_1 v x_3 v x_4)
- C_3 = (x_2 v ~x_3 v ~x_4)
- C_4 = (~x_1 v ~x_2 v x_4)

Satisfying assignment: x_1 = T, x_2 = T, x_3 = F, x_4 = T
- C_1: x_1=T -> satisfied
- C_2: x_4=T -> satisfied
- C_3: ~x_3=T -> satisfied
- C_4: x_4=T -> satisfied

**Constructed target instance (PathConstrainedNetworkFlow):**
- Variable arcs: e_1, e_2, e_3, e_4 (capacity 1 each)
- Clause arcs: e_5, e_6, e_7, e_8 (capacity 1 each)
- Literal arcs: c_{1,1}, c_{1,2}, c_{1,3}, c_{2,1}, c_{2,2}, c_{2,3}, c_{3,1}, c_{3,2}, c_{3,3}, c_{4,1}, c_{4,2}, c_{4,3} (capacity 1 each)
- Variable paths (8 total): p_{x_1}, p_{~x_1}, p_{x_2}, p_{~x_2}, p_{x_3}, p_{~x_3}, p_{x_4}, p_{~x_4}
- Clause paths (12 total): 3 per clause
- R = 4 + 4 = 8

**Solution mapping:**
- Assignment x_1=T, x_2=T, x_3=F, x_4=T:
  - Select paths p_{x_1}, p_{x_2}, p_{~x_3}, p_{x_4} (flow = 1 each, 4 units)
  - For C_1: x_1 satisfies it, select clause path p~_{1,1} (1 unit)
  - For C_2: x_4 satisfies it, select clause path p~_{2,3} (1 unit)
  - For C_3: ~x_3 satisfies it, select clause path p~_{3,2} (1 unit)
  - For C_4: x_4 satisfies it, select clause path p~_{4,3} (1 unit)
  - Total flow into t = 4 + 4 = 8 = R


## References

- **[Promel, 1978]**: [`Promel1978`] H. J. Pr{\"o}mel (1978). "".
