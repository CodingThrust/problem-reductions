# MCP example session

**Before you start:** [connect the server](mcp.md). This is a reproducible task specification; the [CLI recording](cli-demo.md) shows a real execution of the corresponding workflow.

## Ask the agent

```text
Create Maximum Independent Set on the cycle with edges
0-1, 1-2, 2-3, 3-4, 4-0. Discover a route to ILP, reduce and solve it.
Evaluate the recovered source configuration on the original instance.
Cross-check the optimum with brute force and report both evaluations.
```

## Expected tool sequence

| Tool | Key input |
|---|---|
| `show_problem` | `problem: "MIS"` |
| `find_path` | `source: "MIS", target: "ILP"` |
| `create_problem` | `problem_type: "MIS", params: {"edges": "0-1,1-2,2-3,3-4,4-0"}` |
| `reduce` | Original problem JSON, `target: "ILP"` |
| `solve` | Reduction bundle JSON, `solver: "ilp"` |
| `evaluate` | Original problem JSON, recovered `config` |
| `solve` | Original problem JSON, `solver: "brute-force"` |

## Check the result

The maximum independent set has size 2. Both solves and the source evaluation should return `Max(2)`. Valid witnesses include `[1,0,1,0,0]` and `[0,0,1,0,1]`.

Retain the original JSON for evaluation. Passing only the transformed target loses the context required to check the original problem.
