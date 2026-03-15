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

# Run an agent with the configured runner (claude or codex).
#   run_agent <log-file> <prompt>
run_agent() {
    output_file=$1
    prompt=$2

    if [ "${RUNNER:-codex}" = "claude" ]; then
        claude --dangerously-skip-permissions \
            --model "${CLAUDE_MODEL:-opus}" \
            --verbose \
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

project_items_json() {
    gh project item-list 8 --owner CodingThrust --format json --limit 500
}

# Detect the next eligible item and preserve retryable state in a queue.
#   poll_project_items <mode> <state-file> [repo]
poll_project_items() {
    mode=$1
    state_file=$2
    repo=${3-}
    board_json=$(project_items_json) || return $?

    if [ -n "$repo" ]; then
        printf '%s\n' "$board_json" | python3 scripts/project_board_poll.py poll "$mode" "$state_file" --repo "$repo"
    else
        printf '%s\n' "$board_json" | python3 scripts/project_board_poll.py poll "$mode" "$state_file"
    fi
}

ack_polled_item() {
    state_file=$1
    item_id=$2
    python3 scripts/project_board_poll.py ack "$state_file" "$item_id"
}

# Poll a board column and dispatch a make target when new items appear.
#   watch_and_dispatch <mode> <make-target> <label> [repo]
# Example:
#   watch_and_dispatch ready  run-pipeline "Ready issues"
#   watch_and_dispatch review run-review   "Copilot-reviewed PRs" "$REPO"
watch_and_dispatch() {
    mode=$1
    make_target=$2
    label=$3
    repo=${4-}
    interval=${POLL_INTERVAL:-600}

    state_file=$(mktemp /tmp/problemreductions-${mode}-state.XXXXXX)
    trap 'rm -f "$state_file"' EXIT INT TERM

    echo "Watching for new ${label} (polling every $((interval / 60))m)..."
    while true; do
        next_item=$(poll_project_items "$mode" "$state_file" "$repo")
        status=$?
        if [ "$status" -eq 0 ]; then
            item_id=$(printf '%s\n' "$next_item" | cut -f1)
            number=$(printf '%s\n' "$next_item" | cut -f2)
            echo "$(date '+%Y-%m-%d %H:%M:%S') New ${label}: item $number ($item_id)"
            if ${MAKE:-make} "$make_target" N="$number"; then
                ack_polled_item "$state_file" "$item_id" || exit $?
            else
                dispatch_status=$?
                echo "$(date '+%Y-%m-%d %H:%M:%S') Dispatch failed for ${label} item $number; will retry after sleep." >&2
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
