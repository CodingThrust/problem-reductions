#!/usr/bin/env bash
# Generate CLI output snippets for mdBook documentation.
# Called by `make doc` before `mdbook build`.
# Outputs go to docs/src/generated/ and are {{#include}}'d by markdown files.

set -euo pipefail

PRED="${1:-target/release/pred}"
OUT="docs/src/generated"
mkdir -p "$OUT"

echo "Generating doc snippets with $PRED ..."

# 1. pred list (full output)
"$PRED" list > "$OUT/pred-list.txt"

# 2. pred show MIS
"$PRED" show MIS > "$OUT/pred-show-mis.txt"

# 3. pred to MIS --hops 2
"$PRED" to MIS --hops 2 > "$OUT/pred-to-mis.txt"

# 4. pred from QUBO --hops 1
"$PRED" from QUBO --hops 1 > "$OUT/pred-from-qubo.txt"

# 5. pred path MIS QUBO
"$PRED" path MIS QUBO > "$OUT/pred-path-mis-qubo.txt"

# 6. pred path Factoring SpinGlass
"$PRED" path Factoring SpinGlass > "$OUT/pred-path-factoring-spinglass.txt"

# 7. pred create + solve (pipe, brute-force) — JSON output
"$PRED" create MIS --graph 0-1,1-2,2-3 2>/dev/null | "$PRED" solve - --solver brute-force 2>/dev/null > "$OUT/pred-solve-bf.txt"

# 8. pred create + solve (pipe, ILP) — JSON output
"$PRED" create MIS --graph 0-1,1-2,2-3 2>/dev/null | "$PRED" solve - 2>/dev/null > "$OUT/pred-solve-ilp.txt"

# 9. pred create + reduce + solve bundle
"$PRED" create MIS --graph 0-1,1-2,2-3 -o /tmp/pred_doc_problem.json 2>/dev/null
"$PRED" reduce /tmp/pred_doc_problem.json --to QUBO -o /tmp/pred_doc_reduced.json 2>/dev/null
"$PRED" solve /tmp/pred_doc_reduced.json --solver brute-force 2>/dev/null > "$OUT/pred-solve-bundle.txt"
rm -f /tmp/pred_doc_problem.json /tmp/pred_doc_reduced.json

# 10. pred evaluate
"$PRED" create MIS --graph 0-1,1-2,2-3 2>/dev/null | "$PRED" evaluate - --config 1,0,1,0 2>/dev/null > "$OUT/pred-evaluate.txt"

# 11. pred show typo suggestion (goes to stderr)
"$PRED" show MaximumIndependentSe 2> "$OUT/pred-show-typo.txt" || true

# 12. Alias table (extract from pred list --json)
"$PRED" list --json 2>/dev/null | python3 -c "
import json, sys
data = json.load(sys.stdin)
rows = []
for v in data.get('variants', []):
    aliases = v.get('aliases', '')
    if aliases:
        name = v['name'].split('/')[0]
        for a in aliases.split(', '):
            a = a.strip()
            if a:
                rows.append((a, name))
rows.sort()
print('| Alias | Full Name |')
print('|-------|-----------|')
for alias, name in rows:
    print(f'| \`{alias}\` | \`{name}\` |')
" > "$OUT/pred-aliases.txt"

# 13. Factoring example output (path discovery line + overhead)
FACTORING_OUTPUT=$(cargo run --example chained_reduction_factoring_to_spinglass 2>/dev/null)
echo "$FACTORING_OUTPUT" | head -1 > "$OUT/factoring-path.txt"
echo "$FACTORING_OUTPUT" | sed -n '2p' > "$OUT/factoring-result.txt"
echo "$FACTORING_OUTPUT" | sed -n '3,$p' > "$OUT/factoring-overhead.txt"

echo "Done. Generated $(ls "$OUT" | wc -l | tr -d ' ') snippets in $OUT/"
