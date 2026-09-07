# Quick start

## Install

```bash
cargo install problemreductions-cli
pred --version
```

Rust and a native build toolchain are required. ILP solving uses the bundled HiGHS backend.

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
pred evaluate cycle.json --config '[true,false,true,false,false]'
```

`MIS` is Maximum Independent Set: select as many pairwise non-adjacent vertices as possible. The graph is a cycle on five vertices. `solve` executes the registered ILP pipeline and maps the solution back, reporting `Max(2)` with a configuration such as `[true, false, true, false, false]`. `evaluate` scores a configuration of your own against the same instance. Several optimal configurations exist, so the solver's choice may differ from yours.

## Terminal session

A recording of the real CLI: discover a route, transform the instance, solve, and check the result.

<iframe class="cli-cast" src="static/cli-demo.html" title="Terminal recording: reduce Maximum Independent Set to ILP and verify the solution" loading="lazy" allowfullscreen></iframe>

[Open the player](static/cli-demo.html) · [Download the cast](static/cli-demo.cast)

```bash
pred path MIS ILP --json -o paths.json
python3 -c 'import json; print(json.dumps(json.load(open("paths.json"))["paths"][0]))' > path.json
pred create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o cycle.json
pred reduce cycle.json --via path.json -o reduced.json
pred solve reduced.json
pred evaluate cycle.json --config '[true,false,true,false,false]'
pred solve cycle.json
```

`path.json` contains one explicitly selected route from the returned path set. `reduced.json` keeps the source instance and that route, so solving the bundle recovers a source solution. The final command solves the original file through its registered ILP pipeline. Both solves and the independent evaluation return `Max(2)`; optimal solutions may differ.
