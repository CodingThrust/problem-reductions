---
name: Problem
about: Propose a new problem type
title: "[Model] IntegralFlowWithHomologousArcs"
labels: model
assignees: ''
---

## Motivation

INTEGRAL FLOW WITH HOMOLOGOUS ARCS (P111) from Garey & Johnson, A2 ND35. A classical NP-complete problem that generalizes standard network flow by requiring equal flow on designated pairs of "homologous" arcs. The integrality constraint combined with the equal-flow requirement makes the problem intractable, even though the non-integral variant reduces to linear programming.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None in current set.
- **As target:** R56: 3SAT -> INTEGRAL FLOW WITH HOMOLOGOUS ARCS

## Definition

**Name:** `IntegralFlowHomologousArcs`
<!-- ⚠️ Unverified -->
**Canonical name:** INTEGRAL FLOW WITH HOMOLOGOUS ARCS
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND35

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+, set H ⊆ A × A of "homologous" pairs of arcs.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, flow is conserved at v,
(3) for all pairs <a,a'> ∈ H, f(a) = f(a'), and
(4) the net flow into t is at least R?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |A| (one variable per arc in the directed graph).
- **Per-variable domain:** {0, 1, ..., c(a)} where c(a) is the capacity of arc a. In the unit-capacity case, domain is {0, 1}.
- **Meaning:** Each variable f(a) represents the integer flow on arc a. A configuration is a valid integral flow if it satisfies capacity constraints, flow conservation at all non-terminal vertices, equal-flow constraints on homologous pairs, and achieves total flow into t of at least R.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `IntegralFlowHomologousArcs`
**Variants:** None (single variant; problem is always on a directed graph with integer capacities).

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices \|V\| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs (u, v) in the graph |
| `capacities` | `Vec<u64>` | Capacity c(a) for each arc |
| `source` | `usize` | Source vertex s |
| `sink` | `usize` | Sink vertex t |
| `requirement` | `u64` | Flow requirement R |
| `homologous_pairs` | `Vec<(usize, usize)>` | Pairs of arc indices that must carry equal flow |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The problem asks whether there exists an integral flow meeting all constraints.
- NP-complete even with unit capacities (c(a) = 1 for all a).
- The non-integral version is polynomially equivalent to LINEAR PROGRAMMING.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The problem is NP-complete (Sahni, 1974). Brute-force enumeration over all possible integer flow assignments takes O(prod_{a in A} (c(a)+1)) time. With unit capacities, this is O(2^|A|). No significantly better exact algorithm is known for the general case.
- **NP-completeness:** Proved by Sahni (1974) via reduction from 3SAT. Remains NP-complete with unit capacities (Even, Itai, and Shamir, 1976).
- **Special cases:** The variant without integrality constraint (allowing real-valued flows with equal-flow constraints) is polynomially equivalent to LINEAR PROGRAMMING (Itai, 1977).
- **References:**
  - S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262-279.
  - S. Even, A. Itai, A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM J. Comput.* 5, pp. 691-703.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+, set H ⊆ A × A of "homologous" pairs of arcs.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, flow is conserved at v,
(3) for all pairs <a,a'> ∈ H, f(a) = f(a'), and
(4) the net flow into t is at least R?
Reference: [Sahni, 1974]. Transformation from 3SAT.
Comment: Remains NP-complete if c(a) = 1 for all a ∈ A (by modifying the construction in [Even, Itai, and Shamir, 1976]). Corresponding problem with non-integral flows is polynomially equivalent to LINEAR PROGRAMMING [Itai, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [x] It can be solved by reducing to integer programming.
- [x] Other: Enumerate all integer flow assignments on arcs (exponential), check capacity, conservation, homologous-pair, and requirement constraints. Alternatively, formulate as an ILP with flow variables, capacity constraints, conservation constraints, and equality constraints for homologous pairs.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance (YES):**
Directed graph with 6 vertices {0=s, 1, 2, 3, 4, 5=t} and 8 arcs:
- a_0 = (0,1) cap 1, a_1 = (0,2) cap 1, a_2 = (1,3) cap 1, a_3 = (2,3) cap 1
- a_4 = (1,4) cap 1, a_5 = (2,4) cap 1, a_6 = (3,5) cap 1, a_7 = (4,5) cap 1
- Homologous pairs: H = {(a_2, a_5), (a_4, a_3)} meaning f(a_2)=f(a_5) and f(a_4)=f(a_3).
- Requirement R = 2.

Solution: f(a_0)=1, f(a_1)=1, f(a_2)=1, f(a_5)=1, f(a_4)=0, f(a_3)=0, f(a_6)=1, f(a_7)=1.
- Capacity: all flows <= 1.
- Conservation: vertex 1: in=1(a_0), out=1(a_2)+0(a_4)=1. vertex 2: in=1(a_1), out=0(a_3)+1(a_5)=1. vertex 3: in=1(a_2)+0(a_3)=1, out=1(a_6). vertex 4: in=0(a_4)+1(a_5)=1, out=1(a_7).
- Homologous: f(a_2)=f(a_5)=1, f(a_4)=f(a_3)=0.
- Net flow into t=5: f(a_6)+f(a_7) = 2 >= R=2. Answer: YES.

**Instance (NO):**
Same graph but H = {(a_0, a_1), (a_6, a_7)} and R = 2.
- f(a_0)=f(a_1) forced, so both must equal 1 (to get flow 2 into the network). Then vertex 1 has in=1, must send out 1 total. Vertex 2 has in=1, must send out 1 total. But f(a_6)=f(a_7) is also forced.
- For flow 2 into t, we need f(a_6)+f(a_7)=2, so f(a_6)=f(a_7)=1. Node 3 needs in=1=out, so exactly one of a_2, a_3 has flow 1. Node 4 needs in=1=out, so exactly one of a_4, a_5 has flow 1. This is feasible, so this is actually YES as well.
- Change R = 3 with same structure: impossible since max flow is 2. Answer: NO.
