# Quick start

## Install

```bash
cargo install problemreductions-cli
pred --version
```

Rust and a native build toolchain are required. The default ILP backend is HiGHS; the `cplex` and `lp-solvers` features enable separately installed backends listed in the [CLI manifest](https://github.com/CodingThrust/problem-reductions/blob/main/problemreductions-cli/Cargo.toml).

The published crate may lag behind the catalog on this site. To build the current checkout:

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
cargo install --path problemreductions-cli
```

## First solve

```bash
pred create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o cycle.json
pred solve cycle.json
pred evaluate cycle.json --config 1,0,1,0,0
```

`MIS` is Maximum Independent Set: select as many pairwise non-adjacent vertices as possible. The graph is a cycle on five vertices. `solve` discovers a route to ILP, solves the target, and maps the solution back, reporting `Max(2)` with a configuration such as `[1, 0, 1, 0, 0]`. `evaluate` scores a configuration of your own against the same instance. Several optimal configurations exist, so the solver's choice may differ from yours.

## Terminal session

A recording of the real CLI: discover a route, transform the instance, solve, and check the result.

<iframe class="cli-cast" src="static/cli-demo.html" title="Terminal recording: reduce Maximum Independent Set to ILP and verify the solution" loading="lazy" allowfullscreen></iframe>

[Open the player](static/cli-demo.html) · [Download the cast](static/cli-demo.cast)

```bash
pred path MIS ILP
pred create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o cycle.json
pred reduce cycle.json --to ILP -o reduced.json
pred solve reduced.json
pred evaluate cycle.json --config 1,0,1,0,0
pred solve cycle.json
```

The route passes through Maximum Set Packing and a weight cast before reaching binary ILP. `reduced.json` keeps the source instance and the path, so solving the bundle recovers a source solution. The final command solves the original file directly and discovers the same route on its own.

| Check | Recorded result |
|---|---|
| Solve the reduction bundle | `Max(2)`, configuration `[1, 0, 1, 0, 0]` |
| Evaluate that configuration on the source | `Max(2)` |
| Solve the original instance directly | `Max(2)`, solver `ilp (via ILP)` |
