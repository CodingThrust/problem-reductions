# Explore the catalog

Use the installed registry to discover models, accepted fields, and reductions.

```bash
pred list
pred show MIS
pred show MIS --json
```

`list` reports names and aliases. `show` describes the resolved variant, its size fields, input schema, and incoming/outgoing reductions. Use that schema before constructing an instance.

<details>
<summary>Example: inspect Maximum Independent Set</summary>

```text
{{#include generated/pred-show-mis.txt}}
```

</details>

For machine-readable catalog data, use `pred list --json`. Use [names and variants](cli-variants.md) to select exact endpoints and [path queries](cli-paths.md) to explore their connections.
