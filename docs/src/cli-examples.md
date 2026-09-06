# Input examples

Create small instances for different input structures. Use `pred show <model>` for its schema and `pred create --help` for flag syntax.

## Boolean formula

```bash
pred create SAT --num-vars 3 --clauses '1,2;-1,3' -o sat.json
```

Clauses are separated by semicolons. Literals are signed, one-based variable indices; `-1` negates variable 1.

## Quadratic matrix

```bash
pred create QUBO --matrix '1,0.5;0.5,2' -o qubo.json
```

Semicolons separate rows; commas separate entries.

## Set system

```bash
pred create X3C --universe-size 6 --subsets '0,1,2;3,4,5;0,3,4' -o x3c.json
```

Subset elements use zero-based indices into the universe.

## Integer factoring

```bash
pred create Factoring --target 6 --m 2 --n 2 -o factoring.json
```

## Check an instance

```bash
pred inspect sat.json
pred solve sat.json --solver brute-force
```

For other models, start from `pred create --example <variant>` and inspect its fields. Brute-force cost is the product of variable domain sizes; use tiny examples.
