# Random instances

Generate a small graph for exploration:

```bash
pred create MIS --random --num-vertices 10 -o random.json
pred inspect random.json
pred solve random.json --solver brute-force
```

Use `pred create --help` for generation options supported by the current build. Random generation is available for selected input structures; it is not a constructor for every model.

Save the generated JSON when reporting results so another agent can reproduce the exact instance. Use [canonical examples](cli-create.md#use-a-canonical-example) for stable demonstrations.
