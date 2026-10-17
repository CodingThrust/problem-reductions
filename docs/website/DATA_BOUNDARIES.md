# Frontend data boundary inventory

This inventory describes implemented contracts and their acceptance limits.
Run the production verifier in [ENGINEERING.md](ENGINEERING.md) for current
artifact evidence; manual/device acceptance is tracked separately. The Open
Question Board owns its own contracts.

| Payload / producer | Consumer | Current protection | Required invariant / acceptance scope |
| --- | --- | --- | --- |
| Native graph and schema exports (`examples/export_graph.rs`, `examples/export_schemas.rs`) | `scripts/build_website.py`: `json.loads` | Native-export schema and generated guard/types validate both raw exports before builder access; shared graph-reference checks | Preserve exact expressions, family/category agreement, aliases, variants/defaults, endpoints and target-parameter coverage; validate derived artifacts separately at their own boundaries |
| `home-data.js`, emitted by Python as `window.REDUCTIONS` | `src/pages/site.ts` | Version-1 summary schema, generated guard/types and producer validation; unknown input validated before catalog use | Reject malformed summaries and inconsistent family/variant/endpoint references with an alert; preserve optional labels/aliases |
| `graph-scene.json.gz`, emitted by Python | `src/pages/graph-startup.ts`: decompress, parse as unknown, schema/reference validation; consumed by `graph.ts` | Versioned schema, generated standalone validator/types; producer validation; strict loader and cross-field checks | Reject malformed scenes and preserve production, nested-path and failure-recovery behavior |
| `path-graph.json.gz`, emitted by Python | `src/workers/path-worker.ts`: decompress, parse unknown, call strict `path-graph-data.mts` | Version-1 schema and generated guard/types; same producer/consumer validation; safe indices, unique variants/defaults/edges, coordinate and target-field checks | Preserve exact formula strings, tagged nullable fields and explicit native contract errors; check worker envelopes and registered witness routes |
| Per-family `problem-data/*.json`, emitted by `build_problem_records` | `src/pages/site.ts`: `response.json()` | Version-1 record schema, generated guard/types and batch producer validation; unknown response validated before caching | Query family and endpoint checks; exact formula strings and nullable contracts; safe MathML/link fields; failed loads evicted for retry |
| `graph-details.json`, emitted by `build_graph_details.py` | `src/pages/details.ts`: parsed unknown, validated before caching; shared by graph and Atlas | Versioned schema, generated guard/types, producer validation; URLs confined to the current release's article subtree; anchor targets checked | Preserve bounded cache, HTML safety guard and CSP |
| Main thread → worker version-1 search/compare request | `path-worker.ts`: `onmessage` | Unknown structured clone validated against generated request schema and cached-route contract | Safe bounds, query/limit agreement, registered simple witness paths; retain native request validation |
| Worker → main thread `{phase,result}` or `{phase,error}` | `graph.ts`: `onmessage` | Unknown envelope validated against generated reply schema and the corresponding WASM payload contract; stale worker identity checked | Require paths before comparison and query/route agreement; preserve termination, retry and bounded cache |
| JSON strings for `find_paths`, `summarize_paths` | `problemreductions-web/src/lib.rs` | Rust-derived input schemas/types; worker construction uses already-validated graph, message and route data | Rust deserializes the incoming string and validates native semantics; preserve optional/null fields and ignored metadata without repeating full graph validation inside typed callers |
| WASM path response string | `path-worker.ts` calls strict `wasm-paths-data.mts`: parse unknown, validate, then post routes | Rust `WasmPathResult`, version 1; Schemars-derived serialization schema; generated guard/types; safe step counts and chain continuity checked | Check metadata, envelopes and registered witness endpoints |
| WASM comparison response strings | Worker consumes summaries through strict `wasm-comparison-data.mts` | Typed Rust responses; Schemars-derived schema preserves version 5; generated guard/types, safe counts/indices and cross-field checks before use | Preserve count/front/coordinate relationships, exact strings, absent/null semantics and independent native proof verification; check envelopes and selected endpoint parameters |
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
  `graph-growth.mts` uses `bigint` for displayed degrees, degree sums and route
  ordering, preserving the original certificate strings. A real single-route WASM
  fixture covers exponents beyond 2^53; schema and wire validation preserve them exactly.
- Keep numeric indices and small counts only with explicit safe-integer and domain
  checks. Do not convert exact mathematical values through floating point to
  satisfy a TypeScript type. Test wide integers and fractions independently.
- Preserve array ordering, exact endpoint variants and certificate strings,
  optional-field omission versus null, and native full/summary verdict agreement.
  Preserve existing CLI comparison and copy/reduce/solve tests and independent
  proof/certificate checks; schema agreement does not establish mathematical truth.
- Prefer typed Rust wire structs with Schemars-derived schemas and generated TS
  types/runtime validators, alongside Python-owned artifact schemas. Confirm the
  actual Serde union/optional-field representations in one complete migration
  before extending the migration. Browser checks compare WASM summaries against native CLI counts, fronts and profiles.

## Related boundaries that are not JSON parsing

`details/*.html` are repository-generated Typst articles inserted into the DOM.
`SafeDetailHTML` rejects active content, handlers and unsafe resource URLs during
the build; preserve that guard and the existing CSP. JSON Schema for the index
does not replace article validation. WASM bytes need the existing decompression,
initialization and failure/retry checks, rather than a JSON schema.

Route-copy JSON is an outgoing artifact, validated by the real CLI workflow.
There is currently no frontend JSON-upload, clipboard-import, localStorage or
sessionStorage parser in these assets; do not invent persistence or import APIs.
`graph-layout.js` is a Node-only input to `generate_website_graph_layout.js`; it
is not loaded by the application. The builder validates native exports before
invoking its internal layout/article generation steps.

Engineering reports such as `verification.json` and benchmark samples are not
consumed by the product runtime. Keep their provenance and sample units explicit;
do not confuse those report formats with the public mathematical contracts.
