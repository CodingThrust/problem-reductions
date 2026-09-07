# Names and variants

Aliases such as `MIS` resolve to full problem names. A bare name selects that model's declared default variant; `MIS` resolves to `MaximumIndependentSet/SimpleGraph/One`.

## Select an exact variant

```bash
pred show MIS/SimpleGraph/i32
pred path MIS/SimpleGraph/i32 ILP/bool
```

Slash-separated parameters select graph, weight, or other variant values. Use `pred list` and `pred show` to inspect the registry instead of guessing a variant.

`One` means unit weights. Providing non-unit `--weights` when creating a default MIS instance upgrades it to `i32`. Specify `MIS/SimpleGraph/i32` explicitly when a reproducible endpoint matters.

## Common aliases

{{#include generated/pred-aliases.txt}}

Next: [find a path](cli-paths.md).
