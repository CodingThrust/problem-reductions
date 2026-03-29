#!/usr/bin/env sh

# --- Runner ---

# Build a prompt for the given skill, adapting for claude vs codex.
#   skill_prompt <skill-name> <claude-slash-command> [codex-description]
# Examples:
#   skill_prompt project-pipeline "/project-pipeline"       "pick and process the next Ready issue"
#   skill_prompt project-pipeline "/project-pipeline 97"    "process GitHub issue 97"
#   skill_prompt review-pipeline  "/review-pipeline"        "pick and process the next Review pool PR"
skill_prompt() {
    skill=$1
    slash_cmd=$2
    codex_desc=${3-}

    if [ "${RUNNER:-codex}" = "claude" ]; then
        echo "$slash_cmd"
    else
        echo "Use the repo-local skill at '.claude/skills/${skill}/SKILL.md'. Follow it to ${codex_desc}. Read the skill file directly instead of assuming Claude slash-command support."
    fi
}

# Build a prompt and optionally append structured context for Codex.
#   skill_prompt_with_context <skill> <slash-cmd> <codex-desc> <context-label> <context-json>
skill_prompt_with_context() {
    skill=$1
    slash_cmd=$2
    codex_desc=${3-}
    context_label=${4-}
    context_json=${5-}

    base_prompt=$(skill_prompt "$skill" "$slash_cmd" "$codex_desc")
    if [ "${RUNNER:-codex}" = "claude" ] || [ -z "$context_json" ]; then
        echo "$base_prompt"
    else
        printf '%s\n\n## %s\n%s\n' "$base_prompt" "$context_label" "$context_json"
    fi
}

# Run an agent with the configured runner (claude or codex).
#   run_agent <log-file> <prompt>
run_agent() {
    output_file=$1
    prompt=$2

    if [ "${RUNNER:-codex}" = "claude" ]; then
        claude --dangerously-skip-permissions \
            --model "${CLAUDE_MODEL:-opus}" \
            --verbose \
            --output-format text \
            --max-turns 500 \
            -p "$prompt" 2>&1 | tee "$output_file"
    else
        codex exec \
            --enable multi_agent \
            -m "${CODEX_MODEL:-gpt-5.4}" \
            -s danger-full-access \
            "$prompt" 2>&1 | tee "$output_file"
    fi
}

# --- Project board ---

# Detect the next eligible item from the current board snapshot.
#   poll_project_items <mode> <state-file> [repo] [number] [format]
poll_project_items() {
    mode=$1
    state_file=$2
    repo=${3-}
    number=${4-}
    fmt=${5-text}

    set -- scripts/pipeline_board.py next "$mode" "$state_file" --format "$fmt"
    if [ -n "$repo" ]; then
        set -- "$@" --repo "$repo"
    fi
    if [ -n "$number" ]; then
        set -- "$@" --number "$number"
    fi
    # Filter blocked [Rule] issues whose model dependency is missing on main
    if [ "$mode" = "ready" ]; then
        set -- "$@" --repo-root .
    fi
    python3 "$@"
}

ack_polled_item() {
    state_file=$1
    item_id=$2
    python3 scripts/pipeline_board.py ack "$state_file" "$item_id"
}

board_next_json() {
    mode=$1
    repo=${2-}
    number=${3-}
    state_file=${4-}

    if [ -z "$state_file" ]; then
        state_file="/tmp/problemreductions-${mode}-state.json"
    fi

    poll_project_items "$mode" "$state_file" "$repo" "$number" json
}

claim_project_items() {
    mode=$1
    state_file=$2
    repo=${3-}
    number=${4-}
    fmt=${5-json}

    set -- scripts/pipeline_board.py claim-next "$mode" "$state_file" --format "$fmt"
    if [ -n "$repo" ]; then
        set -- "$@" --repo "$repo"
    fi
    if [ -n "$number" ]; then
        set -- "$@" --number "$number"
    fi
    # Filter blocked [Rule] issues whose model dependency is missing on main
    if [ "$mode" = "ready" ]; then
        set -- "$@" --repo-root .
    fi
    python3 "$@"
}

board_claim_json() {
    mode=$1
    repo=${2-}
    number=${3-}
    state_file=${4-}

    if [ -z "$state_file" ]; then
        state_file="/tmp/problemreductions-${mode}-state.json"
    fi

    claim_project_items "$mode" "$state_file" "$repo" "$number" json
}

move_board_item() {
    item_id=$1
    status=$2
    python3 scripts/pipeline_board.py move "$item_id" "$status"
}

# --- PR helpers ---

pr_snapshot() {
    repo=$1
    pr=$2
    python3 scripts/pipeline_pr.py snapshot --repo "$repo" --pr "$pr" --format json
}

pr_wait_ci() {
    repo=$1
    pr=$2
    timeout=${3:-900}
    interval=${4:-30}
    python3 scripts/pipeline_pr.py wait-ci --repo "$repo" --pr "$pr" --timeout "$timeout" --interval "$interval" --format json
}

review_pipeline_context() {
    repo=$1
    pr=${2-}
    fmt=${3:-json}

    set -- scripts/pipeline_skill_context.py review-pipeline --repo "$repo" --format "$fmt"
    if [ -n "$pr" ]; then
        set -- "$@" --pr "$pr"
    fi
    python3 "$@"
}

# --- Issue helpers ---

issue_guards() {
    repo=$1
    issue=$2
    repo_root=${3:-.}
    python3 scripts/pipeline_checks.py issue-guards --repo "$repo" --issue "$issue" --repo-root "$repo_root" --format json
}

issue_context() {
    repo=$1
    issue=$2
    repo_root=${3:-.}
    python3 scripts/pipeline_checks.py issue-context --repo "$repo" --issue "$issue" --repo-root "$repo_root" --format json
}

# --- Worktree helpers ---

create_issue_worktree() {
    issue=$1
    slug=$2
    base=${3:-origin/main}
    python3 scripts/pipeline_worktree.py create-issue --issue "$issue" --slug "$slug" --base "$base" --format json
}

checkout_pr_worktree() {
    repo=$1
    pr=$2
    python3 scripts/pipeline_worktree.py checkout-pr --repo "$repo" --pr "$pr" --format json
}

merge_main_worktree() {
    worktree=$1
    python3 scripts/pipeline_worktree.py merge-main --worktree "$worktree" --format json
}

cleanup_pipeline_worktree() {
    worktree=$1
    python3 scripts/pipeline_worktree.py cleanup --worktree "$worktree" --format json
}

# Poll a board column and dispatch a make target when new items appear.
#   watch_and_dispatch <mode> <make-target> <label> [repo]
# Example:
#   watch_and_dispatch ready  run-pipeline "Ready issues"
#   watch_and_dispatch review run-review   "Review pool PRs" "$REPO"
watch_and_dispatch() {
    mode=$1
    make_target=$2
    label=$3
    repo=${4-}
    interval=${POLL_INTERVAL:-1800}
    max_retries=${MAX_RETRIES:-3}

    state_file=${STATE_FILE:-/tmp/problemreductions-${mode}-forever-state.json}

    trap 'exit 130' INT TERM
    echo "Watching for new ${label} (polling every $((interval / 60))m, max retries ${max_retries})..."
    while true; do
        next_item=$(poll_project_items "$mode" "$state_file" "$repo" "" text)
        status=$?
        if [ "$status" -eq 0 ]; then
            item_id=$(printf '%s\n' "$next_item" | cut -f1)
            number=$(printf '%s\n' "$next_item" | cut -f2)
            echo "$(date '+%Y-%m-%d %H:%M:%S') New ${label}: item $number ($item_id)"
            if ${MAKE:-make} "$make_target" N="$number"; then
                ack_polled_item "$state_file" "$item_id" || exit $?
                echo "$(date '+%Y-%m-%d %H:%M:%S') Processed ${label} item $number; sleeping $((interval / 60))m..."
                sleep "$interval"
            else
                # Track retries in state file; move to On Hold after max_retries
                retry_count=$(python3 -c "
import json, sys
state_file, item_id = sys.argv[1], sys.argv[2]
try:
    state = json.load(open(state_file))
except (FileNotFoundError, json.JSONDecodeError, ValueError):
    state = {}
retries = state.get('retries', {})
retries[item_id] = retries.get(item_id, 0) + 1
state['retries'] = retries
json.dump(state, open(state_file, 'w'), indent=2, sort_keys=True)
print(retries[item_id])
" "$state_file" "$item_id" 2>/dev/null || echo 1)
                if [ "$retry_count" -ge "$max_retries" ]; then
                    echo "$(date '+%Y-%m-%d %H:%M:%S') Item $number ($item_id) failed ${retry_count} times; moving to On Hold." >&2
                    move_board_item "$item_id" "on-hold" 2>/dev/null || true
                    ack_polled_item "$state_file" "$item_id" 2>/dev/null || true
                else
                    echo "$(date '+%Y-%m-%d %H:%M:%S') Dispatch failed for ${label} item $number (attempt ${retry_count}/${max_retries}); will retry after sleep." >&2
                fi
                sleep "$interval"
                continue
            fi
        elif [ "$status" -eq 1 ]; then
            echo "$(date '+%Y-%m-%d %H:%M:%S') No new ${label}, sleeping $((interval / 60))m..."
            sleep "$interval"
        else
            exit "$status"
        fi
    done
}
