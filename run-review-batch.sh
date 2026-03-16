#!/bin/bash
set -e

PRS=(223 637 623 633 626 625)
RUNNER=${RUNNER:-claude}

echo "=== Batch review started at $(date) ==="
echo "PRs to review: ${PRS[*]}"
echo "Runner: $RUNNER"
echo ""

for pr in "${PRS[@]}"; do
    echo "--- Starting review of PR #$pr at $(date) ---"
    RUNNER=claude make run-review N=$pr || {
        echo "--- PR #$pr review FAILED at $(date) ---"
        continue
    }
    echo "--- PR #$pr review completed successfully at $(date) ---"
    echo ""
done

echo "=== Batch review finished at $(date) ==="
