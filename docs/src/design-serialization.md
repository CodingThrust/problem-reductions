# JSON serialization

All problem types support JSON serialization via serde:

```rust,ignore
use problemreductions::io::{to_json, from_json};

let json: String = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i32> = from_json(&json)?;
```

These helpers serialize typed Rust problem data. The CLI additionally wraps data with `type` and `variant` for dynamic loading. Keep that wrapper when passing files between CLI commands.

Next: [CLI JSON and automation](cli-automation.md).
