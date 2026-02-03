//! Benchmarks for the BruteForce solver on various problem types.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use problemreductions::models::graph::*;
use problemreductions::topology::SimpleGraph;
use problemreductions::models::optimization::*;
use problemreductions::models::satisfiability::*;
use problemreductions::models::set::*;
use problemreductions::models::specialized::*;
use problemreductions::prelude::*;

/// Benchmark IndependentSet on graphs of varying sizes.
fn bench_independent_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("IndependentSet");

    for n in [4, 6, 8, 10].iter() {
        // Create a path graph with n vertices
        let edges: Vec<(usize, usize)> = (0..*n - 1).map(|i| (i, i + 1)).collect();
        let problem = IndependentSet::<SimpleGraph, i32>::new(*n, edges);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("path", n), n, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Benchmark VertexCovering on graphs of varying sizes.
fn bench_vertex_covering(c: &mut Criterion) {
    let mut group = c.benchmark_group("VertexCovering");

    for n in [4, 6, 8, 10].iter() {
        let edges: Vec<(usize, usize)> = (0..*n - 1).map(|i| (i, i + 1)).collect();
        let problem = VertexCovering::<SimpleGraph, i32>::new(*n, edges);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("path", n), n, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Benchmark MaxCut on graphs of varying sizes.
fn bench_max_cut(c: &mut Criterion) {
    let mut group = c.benchmark_group("MaxCut");

    for n in [4, 6, 8, 10].iter() {
        let edges: Vec<(usize, usize, i32)> = (0..*n - 1).map(|i| (i, i + 1, 1)).collect();
        let problem = MaxCut::new(*n, edges);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("path", n), n, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Benchmark SAT on problems with varying numbers of clauses.
fn bench_satisfiability(c: &mut Criterion) {
    let mut group = c.benchmark_group("Satisfiability");

    for num_vars in [4, 6, 8, 10].iter() {
        // Create random-ish 3-SAT clauses
        let clauses: Vec<CNFClause> = (0..*num_vars)
            .map(|i| {
                CNFClause::new(vec![
                    (i as i32 + 1),
                    -((i + 1) as i32 % *num_vars as i32 + 1),
                    ((i + 2) as i32 % *num_vars as i32 + 1),
                ])
            })
            .collect();

        let problem = Satisfiability::<i32>::new(*num_vars, clauses);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("3-sat", num_vars), num_vars, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Benchmark SpinGlass on varying sizes.
#[allow(clippy::manual_is_multiple_of)] // Type inference issues with is_multiple_of
fn bench_spin_glass(c: &mut Criterion) {
    let mut group = c.benchmark_group("SpinGlass");

    for n in [4, 6, 8, 10].iter() {
        let interactions: Vec<((usize, usize), f64)> = (0..*n - 1)
            .map(|i| ((i, i + 1), if i % 2 == 0 { 1.0 } else { -1.0 }))
            .collect();
        let onsite: Vec<f64> = vec![0.1; *n];
        let problem = SpinGlass::new(*n, interactions, onsite);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("chain", n), n, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Benchmark SetCovering on varying sizes.
fn bench_set_covering(c: &mut Criterion) {
    let mut group = c.benchmark_group("SetCovering");

    for num_sets in [4, 6, 8, 10].iter() {
        // Create overlapping sets
        let sets: Vec<Vec<usize>> = (0..*num_sets)
            .map(|i| vec![i, (i + 1) % *num_sets, (i + 2) % *num_sets])
            .collect();
        let problem = SetCovering::<i32>::new(*num_sets, sets);
        let solver = BruteForce::new();

        group.bench_with_input(
            BenchmarkId::new("overlapping", num_sets),
            num_sets,
            |b, _| b.iter(|| solver.find_best(black_box(&problem))),
        );
    }

    group.finish();
}

/// Benchmark KColoring on varying graph sizes.
fn bench_coloring(c: &mut Criterion) {
    let mut group = c.benchmark_group("KColoring");

    for n in [3, 4, 5, 6].iter() {
        let edges: Vec<(usize, usize)> = (0..*n - 1).map(|i| (i, i + 1)).collect();
        let problem = KColoring::<3, SimpleGraph, i32>::new(*n, edges);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("path_3colors", n), n, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Benchmark Matching on varying graph sizes.
fn bench_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matching");

    for n in [4, 6, 8, 10].iter() {
        let edges: Vec<(usize, usize, i32)> = (0..*n - 1).map(|i| (i, i + 1, 1)).collect();
        let problem = Matching::new(*n, edges);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("path", n), n, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Benchmark PaintShop on varying sizes.
fn bench_paintshop(c: &mut Criterion) {
    let mut group = c.benchmark_group("PaintShop");

    for n in [2, 3, 4, 5].iter() {
        // Create sequence where each car i appears at positions i and n+i
        let sequence: Vec<String> = (0..*n)
            .flat_map(|i| vec![format!("car{}", i)])
            .chain((0..*n).map(|i| format!("car{}", i)))
            .collect();
        let problem = PaintShop::from_strings(sequence);
        let solver = BruteForce::new();

        group.bench_with_input(BenchmarkId::new("sequential", n), n, |b, _| {
            b.iter(|| solver.find_best(black_box(&problem)))
        });
    }

    group.finish();
}

/// Compare problem types at the same solution space size.
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparison_8vars");

    let solver = BruteForce::new();

    // IndependentSet with 8 vertices
    let is_problem = IndependentSet::<SimpleGraph, i32>::new(8, vec![(0, 1), (2, 3), (4, 5), (6, 7)]);
    group.bench_function("IndependentSet", |b| {
        b.iter(|| solver.find_best(black_box(&is_problem)))
    });

    // SAT with 8 variables
    let sat_problem = Satisfiability::<i32>::new(
        8,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 4, 5]),
            CNFClause::new(vec![-2, -3, 6]),
            CNFClause::new(vec![7, 8, -4]),
        ],
    );
    group.bench_function("Satisfiability", |b| {
        b.iter(|| solver.find_best(black_box(&sat_problem)))
    });

    // SpinGlass with 8 spins
    let sg_problem = SpinGlass::new(
        8,
        vec![((0, 1), 1.0), ((2, 3), -1.0), ((4, 5), 1.0), ((6, 7), -1.0)],
        vec![0.0; 8],
    );
    group.bench_function("SpinGlass", |b| {
        b.iter(|| solver.find_best(black_box(&sg_problem)))
    });

    // MaxCut with 8 vertices
    let mc_problem = MaxCut::new(8, vec![(0, 1, 1), (2, 3, 1), (4, 5, 1), (6, 7, 1)]);
    group.bench_function("MaxCut", |b| {
        b.iter(|| solver.find_best(black_box(&mc_problem)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_independent_set,
    bench_vertex_covering,
    bench_max_cut,
    bench_satisfiability,
    bench_spin_glass,
    bench_set_covering,
    bench_coloring,
    bench_matching,
    bench_paintshop,
    bench_comparison,
);

criterion_main!(benches);
