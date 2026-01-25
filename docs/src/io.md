# File I/O

All problem types support JSON serialization.

## Writing Problems

```rust
use problemreductions::io::{write_problem, FileFormat};
use problemreductions::prelude::*;

let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2)]);
write_problem(&problem, "problem.json", FileFormat::Json).unwrap();
```

## Reading Problems

```rust
use problemreductions::io::{read_problem, FileFormat};
use problemreductions::prelude::*;

let problem: IndependentSet<i32> = read_problem("problem.json", FileFormat::Json).unwrap();
```

## String Serialization

```rust
use problemreductions::io::{to_json, from_json};
use problemreductions::prelude::*;

let problem = IndependentSet::<i32>::new(3, vec![(0, 1)]);

// Serialize to string
let json = to_json(&problem).unwrap();

// Deserialize from string
let restored: IndependentSet<i32> = from_json(&json).unwrap();
```

## File Formats

| Format | Description |
|--------|-------------|
| `Json` | Pretty-printed JSON |
| `JsonCompact` | Compact JSON (no whitespace) |
