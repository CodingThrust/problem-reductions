//! Reduction from KSatisfiability (3-SAT) to TimetableDesign.
//!
//! The issue sketch for #486 is not directly implementable against this
//! repository's `TimetableDesign` model: the sketch relies on pair-specific
//! availability and per-clause optional work, while the model exposes only
//! craftsman/task availability sets and exact pairwise requirements.
//!
//! This implementation uses a fully specified chain instead:
//! 1. Eliminate pure literals from the source formula.
//! 2. Apply Tovey's bounded-occurrence cloning so every remaining variable
//!    appears at most three times and every clause has length two or three.
//! 3. Build the explicit bipartite list-edge-coloring instance from Marx's
//!    outerplanar reduction, padding missing literal-occurrence colors with
//!    dummy colors when a variable occurs only `1+1`, `2+1`, or `1+2` times.
//! 4. Compile the list instance to a core  edge-coloring instance with blocked
//!    colors on auxiliary vertices, following Marx's precoloring gadgets.
//! 5. Encode blocked colors as dedicated dummy assignments in `TimetableDesign`.
//!
//! A timetable witness therefore yields a proper coloring of the core gadget
//! graph; the colors on the two special variable edges recover the satisfying
//! truth assignment.

use crate::models::formula::{CNFClause, KSatisfiability};
use crate::models::misc::TimetableDesign;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
#[cfg(any(test, feature = "example-db"))]
use crate::traits::Problem;
use crate::variant::K3;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
struct NormalizedFormula {
    clauses: Vec<CNFClause>,
    transformed_to_original: Vec<usize>,
    pure_assignments: Vec<Option<usize>>,
    source_num_vars: usize,
}

#[derive(Debug, Clone)]
struct CoreGraph {
    edges: Vec<(usize, usize)>,
    blocked_colors: Vec<Vec<usize>>,
}

impl CoreGraph {
    fn new() -> Self {
        Self {
            edges: Vec::new(),
            blocked_colors: Vec::new(),
        }
    }

    fn add_vertex(&mut self) -> usize {
        let id = self.blocked_colors.len();
        self.blocked_colors.push(Vec::new());
        id
    }

    fn add_edge(&mut self, u: usize, v: usize) -> usize {
        self.edges.push((u, v));
        self.edges.len() - 1
    }

    fn block_color(&mut self, vertex: usize, color: usize) {
        let blocked = &mut self.blocked_colors[vertex];
        if !blocked.contains(&color) {
            blocked.push(color);
        }
    }
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
enum EdgeEncoding {
    Direct {
        edge: usize,
        allowed: Vec<usize>,
    },
    TwoList {
        left_outer: usize,
        middle: usize,
        right_outer: usize,
        first: usize,
        second: usize,
    },
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct VariableEncoding {
    vb: EdgeEncoding,
    vd: EdgeEncoding,
    ab: EdgeEncoding,
    bc: EdgeEncoding,
    cd: EdgeEncoding,
    de: EdgeEncoding,
    neg2: usize,
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct ClauseEncoding {
    edge: EdgeEncoding,
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct ReductionLayout {
    source_num_vars: usize,
    num_periods: usize,
    craftsman_avail: Vec<Vec<bool>>,
    task_avail: Vec<Vec<bool>>,
    requirements: Vec<Vec<u64>>,
    pure_assignments: Vec<Option<usize>>,
    transformed_to_original: Vec<usize>,
    normalized_clauses: Vec<CNFClause>,
    variable_encodings: Vec<VariableEncoding>,
    clause_encodings: Vec<ClauseEncoding>,
    edge_pairs: Vec<(usize, usize)>,
    blocker_pairs: HashMap<(usize, usize), (usize, usize)>,
}

/// Result of reducing KSatisfiability<K3> to TimetableDesign.
#[derive(Debug, Clone)]
pub struct Reduction3SATToTimetableDesign {
    target: TimetableDesign,
    layout: ReductionLayout,
}

fn literal_var_index(literal: i32) -> usize {
    literal.unsigned_abs() as usize - 1
}

#[cfg(any(test, feature = "example-db"))]
fn evaluate_clause(clause: &CNFClause, assignment: &[usize]) -> bool {
    clause.literals.iter().any(|&literal| {
        let value = assignment[literal_var_index(literal)] == 1;
        if literal > 0 { value } else { !value }
    })
}

fn eliminate_pure_literals(source: &KSatisfiability<K3>) -> (Vec<CNFClause>, Vec<Option<usize>>) {
    let mut clauses = source.clauses().to_vec();
    let mut assignments = vec![None; source.num_vars()];

    loop {
        let mut positive = vec![0usize; source.num_vars()];
        let mut negative = vec![0usize; source.num_vars()];
        for clause in &clauses {
            for &literal in &clause.literals {
                let var = literal_var_index(literal);
                if literal > 0 {
                    positive[var] += 1;
                } else {
                    negative[var] += 1;
                }
            }
        }

        let mut changed = false;
        for var in 0..source.num_vars() {
            if assignments[var].is_some() {
                continue;
            }
            match (positive[var] > 0, negative[var] > 0) {
                (true, false) => {
                    assignments[var] = Some(1);
                    changed = true;
                }
                (false, true) => {
                    assignments[var] = Some(0);
                    changed = true;
                }
                _ => {}
            }
        }

        if !changed {
            break;
        }

        clauses.retain(|clause| {
            !clause.literals.iter().any(|&literal| {
                let var = literal_var_index(literal);
                match assignments[var] {
                    Some(1) => literal > 0,
                    Some(0) => literal < 0,
                    None => false,
                    Some(_) => unreachable!(),
                }
            })
        });
    }

    (clauses, assignments)
}

fn normalize_formula(source: &KSatisfiability<K3>) -> NormalizedFormula {
    let (mut clauses, pure_assignments) = eliminate_pure_literals(source);
    let source_num_vars = source.num_vars();
    let mut transformed_to_original = Vec::new();
    let mut next_var = source_num_vars + 1;

    for original_var in 1..=source_num_vars {
        let mut occurrences = Vec::new();
        for (clause_idx, clause) in clauses.iter().enumerate() {
            for (lit_idx, &literal) in clause.literals.iter().enumerate() {
                if literal_var_index(literal) + 1 == original_var {
                    occurrences.push((clause_idx, lit_idx, literal > 0));
                }
            }
        }

        if occurrences.is_empty() {
            continue;
        }

        if occurrences.len() <= 3 {
            let replacement = next_var;
            next_var += 1;
            transformed_to_original.push(original_var - 1);
            for (clause_idx, lit_idx, is_positive) in occurrences {
                clauses[clause_idx].literals[lit_idx] =
                    if is_positive { replacement as i32 } else { -(replacement as i32) };
            }
            continue;
        }

        let replacements: Vec<usize> = (0..occurrences.len())
            .map(|_| {
                let id = next_var;
                next_var += 1;
                transformed_to_original.push(original_var - 1);
                id
            })
            .collect();

        for ((clause_idx, lit_idx, is_positive), replacement) in
            occurrences.into_iter().zip(replacements.iter().copied())
        {
            clauses[clause_idx].literals[lit_idx] =
                if is_positive { replacement as i32 } else { -(replacement as i32) };
        }

        for idx in 0..replacements.len() {
            let current = replacements[idx] as i32;
            let next = replacements[(idx + 1) % replacements.len()] as i32;
            clauses.push(CNFClause::new(vec![current, -next]));
        }
    }

    for clause in &mut clauses {
        for literal in &mut clause.literals {
            let sign = if *literal < 0 { -1 } else { 1 };
            let temp_var = literal.unsigned_abs() as usize;
            debug_assert!(
                temp_var > source_num_vars,
                "all residual literals should have been replaced by transformed variables"
            );
            let compact_var = temp_var - source_num_vars;
            *literal = sign * compact_var as i32;
        }
    }

    for transformed_var in 1..=transformed_to_original.len() {
        let mut positive = 0usize;
        let mut negative = 0usize;
        for clause in &clauses {
            for &literal in &clause.literals {
                if literal_var_index(literal) + 1 == transformed_var {
                    if literal > 0 {
                        positive += 1;
                    } else {
                        negative += 1;
                    }
                }
            }
        }
        debug_assert!(positive <= 2, "normalized variable {transformed_var} has {positive} positive occurrences");
        debug_assert!(negative <= 2, "normalized variable {transformed_var} has {negative} negative occurrences");
        debug_assert!(
            positive > 0 && negative > 0,
            "pure literals should have been eliminated before gadget construction"
        );
    }

    NormalizedFormula {
        clauses,
        transformed_to_original,
        pure_assignments,
        source_num_vars,
    }
}

#[cfg(any(test, feature = "example-db"))]
fn choose_clause_edge_color(clause: &CNFClause, assignment: &[usize], colors: &[usize]) -> Option<usize> {
    clause
        .literals
        .iter()
        .zip(colors.iter().copied())
        .find_map(|(&literal, color)| {
            let value = assignment[literal_var_index(literal)] == 1;
            let satisfied = if literal > 0 { value } else { !value };
            satisfied.then_some(color)
        })
}

fn add_two_list_edge(
    graph: &mut CoreGraph,
    all_colors: &[usize],
    x: usize,
    y: usize,
    first: usize,
    second: usize,
) -> EdgeEncoding {
    let x_prime = graph.add_vertex();
    let y_prime = graph.add_vertex();

    let left_outer = graph.add_edge(x, x_prime);
    let middle = graph.add_edge(x_prime, y_prime);
    let right_outer = graph.add_edge(y_prime, y);

    for &color in all_colors {
        if color != first && color != second {
            graph.block_color(x_prime, color);
            graph.block_color(y_prime, color);
        }
    }

    EdgeEncoding::TwoList {
        left_outer,
        middle,
        right_outer,
        first,
        second,
    }
}

fn add_direct_clause_edge(
    graph: &mut CoreGraph,
    all_colors: &[usize],
    x: usize,
    y: usize,
    allowed: Vec<usize>,
) -> EdgeEncoding {
    let edge = graph.add_edge(x, y);
    for &color in all_colors {
        if !allowed.contains(&color) {
            graph.block_color(y, color);
        }
    }
    EdgeEncoding::Direct { edge, allowed }
}

fn core_edge_color(solution: &[usize], pair: (usize, usize), num_tasks: usize, num_periods: usize) -> usize {
    (0..num_periods)
        .find(|&period| solution[((pair.0 * num_tasks) + pair.1) * num_periods + period] == 1)
        .expect("each required pair should be scheduled exactly once")
}

#[cfg(any(test, feature = "example-db"))]
fn encode_edge_color(
    colors: &mut [Option<usize>],
    encoding: &EdgeEncoding,
    chosen_color: usize,
) {
    match encoding {
        EdgeEncoding::Direct { edge, allowed } => {
            assert!(
                allowed.contains(&chosen_color),
                "chosen color {chosen_color} must belong to direct edge list {allowed:?}"
            );
            colors[*edge] = Some(chosen_color);
        }
        EdgeEncoding::TwoList {
            left_outer,
            middle,
            right_outer,
            first,
            second,
        } => {
            assert!(
                chosen_color == *first || chosen_color == *second,
                "chosen color {chosen_color} must belong to two-list edge {{{first}, {second}}}"
            );
            let other = if chosen_color == *first { *second } else { *first };
            colors[*left_outer] = Some(chosen_color);
            colors[*right_outer] = Some(chosen_color);
            colors[*middle] = Some(other);
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
fn edge_from_assignment(encoding: &EdgeEncoding, choose_first: bool) -> usize {
    match encoding {
        EdgeEncoding::Direct { allowed, .. } => allowed[usize::from(!choose_first)],
        EdgeEncoding::TwoList { first, second, .. } => {
            if choose_first { *first } else { *second }
        }
    }
}

fn bipartition(graph: &CoreGraph) -> Vec<bool> {
    let mut side = vec![None; graph.blocked_colors.len()];
    let mut adjacency = vec![Vec::new(); graph.blocked_colors.len()];
    for &(u, v) in &graph.edges {
        adjacency[u].push(v);
        adjacency[v].push(u);
    }

    for start in 0..graph.blocked_colors.len() {
        if side[start].is_some() {
            continue;
        }
        side[start] = Some(false);
        let mut queue = VecDeque::from([start]);
        while let Some(vertex) = queue.pop_front() {
            let current = side[vertex].expect("vertex has been assigned a side");
            for &next in &adjacency[vertex] {
                match side[next] {
                    Some(existing) => {
                        assert_ne!(existing, current, "core graph must remain bipartite");
                    }
                    None => {
                        side[next] = Some(!current);
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    side.into_iter()
        .map(|entry| entry.expect("every vertex should receive a bipartition side"))
        .collect()
}

fn build_layout(source: &KSatisfiability<K3>) -> ReductionLayout {
    let normalized = normalize_formula(source);
    let num_transformed_vars = normalized.transformed_to_original.len();
    if num_transformed_vars == 0 && normalized.clauses.is_empty() {
        return ReductionLayout {
            source_num_vars: normalized.source_num_vars,
            num_periods: 1,
            craftsman_avail: Vec::new(),
            task_avail: Vec::new(),
            requirements: Vec::new(),
            pure_assignments: normalized.pure_assignments,
            transformed_to_original: normalized.transformed_to_original,
            normalized_clauses: normalized.clauses,
            variable_encodings: Vec::new(),
            clause_encodings: Vec::new(),
            edge_pairs: Vec::new(),
            blocker_pairs: HashMap::new(),
        };
    }

    let num_periods = 4 * num_transformed_vars.max(1);
    let all_colors: Vec<usize> = (0..num_periods).collect();

    let mut occurrences_by_var: Vec<Vec<(usize, usize, bool)>> = vec![Vec::new(); num_transformed_vars];
    for (clause_idx, clause) in normalized.clauses.iter().enumerate() {
        for (lit_idx, &literal) in clause.literals.iter().enumerate() {
            occurrences_by_var[literal_var_index(literal)].push((clause_idx, lit_idx, literal > 0));
        }
    }

    let mut clause_literal_colors: Vec<Vec<usize>> = normalized
        .clauses
        .iter()
        .map(|clause| vec![usize::MAX; clause.literals.len()])
        .collect();

    let mut variable_colors = Vec::with_capacity(num_transformed_vars);
    for (index, occurrences) in occurrences_by_var.iter().enumerate() {
        let base = 4 * index;
        let neg2 = base;
        let neg1 = base + 1;
        let pos2 = base + 2;
        let pos1 = base + 3;

        let mut positive_occurrences = Vec::new();
        let mut negative_occurrences = Vec::new();
        for &(clause_idx, lit_idx, is_positive) in occurrences {
            if is_positive {
                positive_occurrences.push((clause_idx, lit_idx));
            } else {
                negative_occurrences.push((clause_idx, lit_idx));
            }
        }

        if let Some(&(clause_idx, lit_idx)) = positive_occurrences.first() {
            clause_literal_colors[clause_idx][lit_idx] = pos1;
        }
        if let Some(&(clause_idx, lit_idx)) = positive_occurrences.get(1) {
            clause_literal_colors[clause_idx][lit_idx] = pos2;
        }
        if let Some(&(clause_idx, lit_idx)) = negative_occurrences.first() {
            clause_literal_colors[clause_idx][lit_idx] = neg1;
        }
        if let Some(&(clause_idx, lit_idx)) = negative_occurrences.get(1) {
            clause_literal_colors[clause_idx][lit_idx] = neg2;
        }

        variable_colors.push((pos1, pos2, neg1, neg2));
    }

    let mut graph = CoreGraph::new();
    let center = graph.add_vertex();
    let mut variable_encodings = Vec::with_capacity(num_transformed_vars);

    for &(pos1, pos2, neg1, neg2) in &variable_colors {
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();

        let ab = add_two_list_edge(&mut graph, &all_colors, a, b, pos1, neg2);
        let bc = add_two_list_edge(&mut graph, &all_colors, b, c, pos2, pos1);
        let cd = add_two_list_edge(&mut graph, &all_colors, c, d, pos1, pos2);
        let de = add_two_list_edge(&mut graph, &all_colors, d, e, pos2, neg1);
        let vb = add_two_list_edge(&mut graph, &all_colors, center, b, neg2, pos2);
        let vd = add_two_list_edge(&mut graph, &all_colors, center, d, neg1, pos1);

        variable_encodings.push(VariableEncoding {
            vb,
            vd,
            ab,
            bc,
            cd,
            de,
            neg2,
        });
    }

    let mut clause_encodings = Vec::with_capacity(normalized.clauses.len());
    for (clause_idx, _clause) in normalized.clauses.iter().enumerate() {
        let clause_vertex = graph.add_vertex();
        let colors = clause_literal_colors[clause_idx].clone();
        debug_assert!(colors.iter().all(|&color| color != usize::MAX));

        let edge = match colors.len() {
            1 => add_direct_clause_edge(&mut graph, &all_colors, center, clause_vertex, colors),
            2 => add_two_list_edge(
                &mut graph,
                &all_colors,
                center,
                clause_vertex,
                colors[0],
                colors[1],
            ),
            3 => add_direct_clause_edge(&mut graph, &all_colors, center, clause_vertex, colors),
            len => panic!("expected clause size 1, 2, or 3 after normalization, got {len}"),
        };

        clause_encodings.push(ClauseEncoding { edge });
    }

    let side = bipartition(&graph);
    let mut vertex_to_craftsman = vec![None; graph.blocked_colors.len()];
    let mut vertex_to_task = vec![None; graph.blocked_colors.len()];

    let mut num_craftsmen = 0usize;
    let mut num_tasks = 0usize;
    for (vertex, is_task_side) in side.iter().copied().enumerate() {
        if is_task_side {
            vertex_to_task[vertex] = Some(num_tasks);
            num_tasks += 1;
        } else {
            vertex_to_craftsman[vertex] = Some(num_craftsmen);
            num_craftsmen += 1;
        }
    }

    let mut craftsman_avail = vec![vec![true; num_periods]; num_craftsmen];
    let mut task_avail = vec![vec![true; num_periods]; num_tasks];
    let mut requirements = vec![vec![0u64; num_tasks]; num_craftsmen];
    let mut edge_pairs = vec![(usize::MAX, usize::MAX); graph.edges.len()];

    for (edge_idx, &(u, v)) in graph.edges.iter().enumerate() {
        let (craft, task) = if !side[u] {
            (
                vertex_to_craftsman[u].expect("left vertex has craftsman index"),
                vertex_to_task[v].expect("right vertex has task index"),
            )
        } else {
            (
                vertex_to_craftsman[v].expect("left vertex has craftsman index"),
                vertex_to_task[u].expect("right vertex has task index"),
            )
        };
        requirements[craft][task] = 1;
        edge_pairs[edge_idx] = (craft, task);
    }

    let mut blocker_pairs = HashMap::new();
    for (vertex, blocked_colors) in graph.blocked_colors.iter().enumerate() {
        for &color in blocked_colors {
            if side[vertex] {
                let task = vertex_to_task[vertex].expect("right vertex has task index");
                craftsman_avail.push({
                    let mut row = vec![false; num_periods];
                    row[color] = true;
                    row
                });
                requirements.push(vec![0u64; task_avail.len()]);
                let craft = requirements.len() - 1;
                requirements[craft][task] = 1;
                blocker_pairs.insert((vertex, color), (craft, task));
            } else {
                let craft = vertex_to_craftsman[vertex].expect("left vertex has craftsman index");
                task_avail.push({
                    let mut row = vec![false; num_periods];
                    row[color] = true;
                    row
                });
                for requirement_row in &mut requirements {
                    requirement_row.push(0);
                }
                let task = task_avail.len() - 1;
                requirements[craft][task] = 1;
                blocker_pairs.insert((vertex, color), (craft, task));
            }
        }
    }

    ReductionLayout {
        source_num_vars: normalized.source_num_vars,
        num_periods,
        craftsman_avail,
        task_avail,
        requirements,
        pure_assignments: normalized.pure_assignments,
        transformed_to_original: normalized.transformed_to_original,
        normalized_clauses: normalized.clauses,
        variable_encodings,
        clause_encodings,
        edge_pairs,
        blocker_pairs,
    }
}

impl Reduction3SATToTimetableDesign {
    #[cfg(any(test, feature = "example-db"))]
    fn construct_target_solution(&self, source_assignment: &[usize]) -> Option<Vec<usize>> {
        if source_assignment.len() != self.layout.source_num_vars {
            return None;
        }
        if self
            .layout
            .pure_assignments
            .iter()
            .enumerate()
            .any(|(var, fixed)| fixed.is_some_and(|value| source_assignment[var] != value))
        {
            return None;
        }

        let transformed_assignment: Vec<usize> = self
            .layout
            .transformed_to_original
            .iter()
            .map(|&original| source_assignment[original])
            .collect();

        if self
            .layout
            .normalized_clauses
            .iter()
            .any(|clause| !evaluate_clause(clause, &transformed_assignment))
        {
            return None;
        }

        let mut colors = vec![None; self.layout.edge_pairs.len()];

        for (transformed_var, encoding) in self.layout.variable_encodings.iter().enumerate() {
            let choose_first = transformed_assignment[transformed_var] == 1;
            for edge in [&encoding.ab, &encoding.bc, &encoding.cd, &encoding.de, &encoding.vb, &encoding.vd] {
                let chosen = edge_from_assignment(edge, choose_first);
                encode_edge_color(&mut colors, edge, chosen);
            }
        }

        for (clause, encoding) in self
            .layout
            .normalized_clauses
            .iter()
            .zip(self.layout.clause_encodings.iter())
        {
            let allowed = match &encoding.edge {
                EdgeEncoding::Direct { allowed, .. } => allowed.clone(),
                EdgeEncoding::TwoList { first, second, .. } => vec![*first, *second],
            };
            let chosen = choose_clause_edge_color(clause, &transformed_assignment, &allowed)?;
            encode_edge_color(&mut colors, &encoding.edge, chosen);
        }

        if colors.iter().any(Option::is_none) {
            return None;
        }

        let num_tasks = self.target.num_tasks();
        let num_periods = self.target.num_periods();
        let mut config = vec![0usize; self.target.dims().len()];

        for (edge_idx, color) in colors.into_iter().enumerate() {
            let (craft, task) = self.layout.edge_pairs[edge_idx];
            let color = color.expect("all core edges should be colored");
            config[((craft * num_tasks) + task) * num_periods + color] = 1;
        }

        for (&(_vertex, color), &(craft, task)) in &self.layout.blocker_pairs {
            config[((craft * num_tasks) + task) * num_periods + color] = 1;
        }

        Some(config)
    }
}

impl ReductionResult for Reduction3SATToTimetableDesign {
    type Source = KSatisfiability<K3>;
    type Target = TimetableDesign;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_tasks = self.target.num_tasks();
        let num_periods = self.target.num_periods();

        let mut transformed_assignment = vec![0usize; self.layout.transformed_to_original.len()];
        for (index, encoding) in self.layout.variable_encodings.iter().enumerate() {
            let vb_pair = match &encoding.vb {
                EdgeEncoding::Direct { edge, .. } => self.layout.edge_pairs[*edge],
                EdgeEncoding::TwoList { left_outer, .. } => self.layout.edge_pairs[*left_outer],
            };
            let vb_color = core_edge_color(target_solution, vb_pair, num_tasks, num_periods);
            transformed_assignment[index] = usize::from(vb_color == encoding.neg2);
        }

        let mut source_assignment = vec![0usize; self.layout.source_num_vars];
        for (var, fixed) in self.layout.pure_assignments.iter().copied().enumerate() {
            if let Some(value) = fixed {
                source_assignment[var] = value;
            }
        }

        let mut seen_transformed = vec![false; self.layout.source_num_vars];
        for (value, &original_var) in transformed_assignment
            .iter()
            .zip(self.layout.transformed_to_original.iter())
        {
            if !seen_transformed[original_var] {
                source_assignment[original_var] = *value;
                seen_transformed[original_var] = true;
            }
        }

        source_assignment
    }
}

#[reduction(overhead = {
    num_periods = "4 * num_literals",
    num_craftsmen = "64 * num_literals^2 + 32 * num_literals + 1",
    num_tasks = "64 * num_literals^2 + 32 * num_literals + 1",
})]
impl ReduceTo<TimetableDesign> for KSatisfiability<K3> {
    type Result = Reduction3SATToTimetableDesign;

    fn reduce_to(&self) -> Self::Result {
        let layout = build_layout(self);
        let target = TimetableDesign::new(
            layout.num_periods,
            layout.craftsman_avail.len(),
            layout.task_avail.len(),
            layout.craftsman_avail.clone(),
            layout.task_avail.clone(),
            layout.requirements.clone(),
        );

        Reduction3SATToTimetableDesign { target, layout }
    }
}

#[cfg(any(test, feature = "example-db"))]
#[allow(dead_code)]
pub(super) fn construct_timetable_from_assignment(
    target: &TimetableDesign,
    assignment: &[usize],
    source: &KSatisfiability<K3>,
) -> Option<Vec<usize>> {
    let reduction = ReduceTo::<TimetableDesign>::reduce_to(source);
    if reduction.target_problem().num_periods() != target.num_periods()
        || reduction.target_problem().num_craftsmen() != target.num_craftsmen()
        || reduction.target_problem().num_tasks() != target.num_tasks()
        || reduction.target_problem().craftsman_avail() != target.craftsman_avail()
        || reduction.target_problem().task_avail() != target.task_avail()
        || reduction.target_problem().requirements() != target.requirements()
    {
        return None;
    }
    reduction.construct_target_solution(assignment)
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_timetabledesign",
        build: || {
            let source = KSatisfiability::<K3>::new(
                2,
                vec![
                    CNFClause::new(vec![1, 2, 2]),
                    CNFClause::new(vec![-1, -2, -2]),
                ],
            );
            let reduction = ReduceTo::<TimetableDesign>::reduce_to(&source);
            let source_config = vec![1, 0];
            let target_config = reduction
                .construct_target_solution(&source_config)
                .expect("canonical satisfying assignment should lift to timetable");

            crate::example_db::specs::rule_example_with_witness::<_, TimetableDesign>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_timetabledesign.rs"]
mod tests;
