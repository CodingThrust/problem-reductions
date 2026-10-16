# Frontend data boundary inventory

Source audit: 2026-10-16, against the current working tree. This is a coverage
inventory and migration checklist, not a claim that contracts are implemented.
Generated production artifacts are fingerprinted by `check_website.py`.

| Payload / producer | Consumer | Current protection | Minimum migration requirement |
| --- | --- | --- | --- |
| Native graph and schema exports (`examples/export_graph.rs`, `examples/export_schemas.rs`) | `scripts/build_website.py`: `json.loads` | Node presence and endpoint bounds checked; Python dictionaries have no shared schema | Validate export shape before indexing it; retain endpoint checks and validate the derived artifacts below |
| `home-data.js`, emitted by Python as `window.REDUCTIONS` | `assets/site.js` | Executable object assignment; no runtime shape check | Validate the initial object as unknown before catalog access; validate producer output too; type generation alone cannot protect an embedded object |
| `graph-scene.json.gz`, emitted by Python | `assets/graph-startup.js`: decompress, `JSON.parse`; consumed by `graph.js` | HTTP status check only | Schema for nodes, edges, aliases and layout; finite coordinates, valid indices and existing family IDs; version and actionable failure before scene construction |
| `path-graph.json.gz`, emitted by Python | `assets/path-worker.js`: decompress, `JSON.parse` | HTTP status check; later Rust deserialization and endpoint validation | Validate the worker's parsed unknown before caching/using it; parameter contracts and formulas remain exact strings; retain Rust's validation |
| Per-family `problem-data/*.json`, emitted by `build_problem_records` | `assets/site.js`: `response.json()` | HTTP check; producer test covers incident endpoints and reindexing | Schema for nodes/edges/schemas plus endpoint-reference checks; distinguish absent optional metadata from null |
| `graph-details.json`, emitted by `build_graph_details.py` | `assets/details.js`: `response.json()`, shared by graph and Atlas | HTTP check and rejected-fetch cache eviction | Schema for `entries` and `anchors`; validate URLs against the same origin/release and retain retry/eviction behavior |
| Main thread → worker `{source,target,limit,paths?}` | `path-worker.js`: `onmessage` | Untyped structured clone; WASM later validates a constructed request | Validate incoming unknown against a request union, version, safe index/limit bounds and optional cached route shape |
| Worker → main thread `{phase,result}` or `{phase,error}` | `graph.js`: `onmessage` | Untyped envelope; stale worker identity checked | Validate phase and mutually exclusive result/error shape plus the appropriate payload; retain worker termination, identity checks and bounded cache |
| JSON strings for `find_paths`, `compare_paths`, `summarize_paths` | `problemreductions-web/src/lib.rs` | Typed Rust `Deserialize` requests and semantic validation | Derive request schemas from real wire structs; validate browser-side requests before serialization and preserve Rust validation |
| WASM path response string | `path-worker.js`: `JSON.parse(find_paths(...))` | Dynamic Rust `json!` response; no output type/schema or consumer validation | A serialized wire response struct and generated schema/types/validator; parse as unknown and validate before posting routes |
| WASM comparison response strings | Worker consumes summary; benchmark tool exercises full and summary APIs | Dynamic Rust `json!`; comparison `schema_version` is already 5 | Serialized wire structs covering counts, profiles, verdicts and optional `pairs`/`dominates`; retain version compatibility, exact strings and semantic equality |
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
  before selecting tools. No contract tooling is adopted by this audit.

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
