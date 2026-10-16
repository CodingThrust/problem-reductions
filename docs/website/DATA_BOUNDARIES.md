# Frontend data boundary inventory

Source audit: 2026-10-16, against the current working tree. This is a coverage
inventory and migration checklist, not a claim that contracts are implemented.
Generated production artifacts are fingerprinted by `check_website.py`. Scene and
documentation-index and WASM path/comparison-result contracts passed local production checks;
other rows remain open.

| Payload / producer | Consumer | Current protection | Minimum migration requirement |
| --- | --- | --- | --- |
| Native graph and schema exports (`examples/export_graph.rs`, `examples/export_schemas.rs`) | `scripts/build_website.py`: `json.loads` | Node presence and endpoint bounds checked; Python dictionaries have no shared schema | Validate export shape before indexing it; retain endpoint checks and validate the derived artifacts below |
| `home-data.js`, emitted by Python as `window.REDUCTIONS` | `assets/site.js` | Executable object assignment; no runtime shape check | Validate the initial object as unknown before catalog access; validate producer output too; type generation alone cannot protect an embedded object |
| `graph-scene.json.gz`, emitted by Python | `assets/graph-startup.ts`: decompress, parse as unknown, schema/reference validation; consumed by `graph.js` | Versioned schema, generated standalone validator/types; producer validation; strict loader and cross-field checks; full local production verification passed | Local negative-input, all-engine production/nested/failure, screenshot and startup-cost checks passed; CI/manual gates and remaining rows stay open |
| `path-graph.json.gz`, emitted by Python | `assets/path-worker.js`: decompress, `JSON.parse` | HTTP status check; later Rust deserialization and endpoint validation | Validate the worker's parsed unknown before caching/using it; parameter contracts and formulas remain exact strings; retain Rust's validation |
| Per-family `problem-data/*.json`, emitted by `build_problem_records` | `assets/site.js`: `response.json()` | HTTP check; producer test covers incident endpoints and reindexing | Schema for nodes/edges/schemas plus endpoint-reference checks; distinguish absent optional metadata from null |
| `graph-details.json`, emitted by `build_graph_details.py` | `assets/details.ts`: parsed unknown, validated before caching; shared by graph and Atlas | Versioned schema, generated guard/types, producer validation; URLs confined to the current release's article subtree; anchor targets checked | Local malformed-input and all-engine rejection/retry, production and nested-path checks passed; preserve bounded cache, HTML safety guard and CSP; CI/manual gates remain open |
| Main thread → worker `{source,target,limit,paths?}` | `path-worker.js`: `onmessage` | Untyped structured clone; WASM later validates a constructed request | Validate incoming unknown against a request union, version, safe index/limit bounds and optional cached route shape |
| Worker → main thread `{phase,result}` or `{phase,error}` | `graph.js`: `onmessage` | Untyped envelope; stale worker identity checked | Validate phase and mutually exclusive result/error shape plus the appropriate payload; retain worker termination, identity checks and bounded cache |
| JSON strings for `find_paths`, `compare_paths`, `summarize_paths` | `problemreductions-web/src/lib.rs` | Typed Rust `Deserialize` requests and semantic validation | Derive request schemas from real wire structs; validate browser-side requests before serialization and preserve Rust validation |
| WASM path response string | `path-worker.js` calls strict `wasm-paths-data.mts`: parse unknown, validate, then post routes | Rust `WasmPathResult`, version 1; Schemars-derived serialization schema; generated guard/types; safe step counts and chain continuity checked | Local malformed-input, real-worker rejection and all-engine production checks passed; request/metadata/envelope validation and registered-endpoint cross-checks remain open |
| WASM comparison response strings | Worker consumes summary through strict `wasm-comparison-data.mts`; benchmark sample mode validates full and summary APIs | Typed Rust responses; Schemars-derived schema preserves version 5; generated guard/types, safe counts/indices and cross-field checks before use | Local negative-input, semantic before/after, exact rational and all-engine production checks passed; preserve pair/front/coordinate relationships, exact strings, absent/null semantics and independent native proof verification; request/envelope contracts remain open |
| Built Open Question Board `registry.json` and its related payloads | Separately built board scripts under `open-questions/` | Owned by the private board project; this repo tests real navigation, registry use and prompt generation | Record that build's own schema/validation coverage when auditing all delivered pages; do not claim this repo's TS migration validates external scripts |

## Shared requirements for the contract stage

- Producer validation and runtime consumer validation are both required. A valid
  schema version does not validate the remaining fields. Treat results from
  `response.json()` and structured clones as unknown, just like `JSON.parse()`.
- Schema validation handles shape; cross-field checks must additionally enforce
  endpoint bounds, known IDs, and applicable parameter/route relationships. A
  structural schema cannot by itself prove that an edge index refers to a node
  in the same graph. Keep those checks at the entry boundary and inside Rust.
- Exact certificates, rational coefficients and expression text remain strings.
  Encode any wide integer as a validated decimal string before it can reach
  JavaScript's number parser. Mathematical rational components must remain exact
  strings with a nonzero denominator; approximate layout coordinates are a
  separate finite-number domain. Audit numeric defaults/bounds in schema metadata
  as well as the obvious mathematical response fields.
- Certificate exponents are arbitrary-precision integers in `src/polynomial.rs`.
  `graph-growth.mts` now uses `bigint` for displayed degrees, degree sums and route
  ordering, preserving the original certificate strings. A real single-route WASM
  fixture covers exponents beyond 2^53; remaining wire validation is still open.
- Keep numeric indices and small counts only with explicit safe-integer and domain
  checks. Do not convert exact mathematical values through floating point to
  satisfy a TypeScript type. Test wide integers and fractions independently.
- Preserve array ordering, exact endpoint variants and certificate strings,
  optional-field omission versus null, and full/summary verdict agreement.
  Preserve existing CLI comparison and copy/reduce/solve tests and independent
  proof/certificate checks; schema agreement does not establish mathematical truth.
- Prefer typed Rust wire structs with Schemars-derived schemas and generated TS
  types/runtime validators, alongside Python-owned artifact schemas. Confirm the
  actual Serde union/optional-field representations in one complete migration
  before extending the migration. Ajv standalone guards and generated TS types now
  cover the two Python-owned contracts and both Rust response contracts. Request
  schemas, worker envelopes and remaining payloads are still open. The legacy path
  benchmark's default entry discovery also needs repair for the current Vite output;
  only its direct sample mode is currently verified.

## Related boundaries that are not JSON parsing

`details/*.html` are repository-generated Typst articles inserted into the DOM.
`SafeDetailHTML` rejects active content, handlers and unsafe resource URLs during
the build; preserve that guard and the existing CSP. JSON Schema for the index
does not replace article validation. WASM bytes need the existing decompression,
initialization and failure/retry checks, rather than a JSON schema.

Route-copy JSON is an outgoing artifact, validated by the real CLI workflow.
There is currently no frontend JSON-upload, clipboard-import, localStorage or
sessionStorage parser in these assets; do not invent persistence or import APIs.
The unused legacy `graph-layout.js` is not loaded by the current page entry points;
confirm its remaining callers before any later removal.

Engineering reports such as `verification.json` and benchmark samples are not
consumed by the product runtime. Keep their provenance and sample units explicit;
do not confuse those report formats with the public mathematical contracts.
