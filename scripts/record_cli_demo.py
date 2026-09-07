#!/usr/bin/env python3
"""Record the documentation CLI demo as an asciinema v2 cast from real output.

Usage: scripts/record_cli_demo.py [--pred target/debug/pred] [-o docs/src/static/cli-demo.cast]

Each command runs in a pseudo-terminal inside a temporary directory holding a
`pred` symlink to the given binary. Typing is simulated; output is never
edited except that trailing blank lines are collapsed so one blank line
separates commands. The cast is also embedded into `cli-demo.html`, the
offline player page. The script fails if any command exits non-zero.
"""

import argparse
import json
import os
import pty
import random
import re
import shlex
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WIDTH, HEIGHT = 88, 22
TYPING_DELAY = 1 / 35
STEP_PAUSE = 2.5

# (comment, command) pairs, run in order.
STEPS = [
    ("Discover a route to an available solver", "./pred path MIS ILP --json -o paths.json"),
    ("Select the first returned route", "python3 -c 'import json; json.dump(json.load(open(\"paths.json\"))[\"paths\"][0], open(\"path.json\", \"w\"))'"),
    ("Create a five-vertex cycle", "./pred create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o cycle.json"),
    ("Transform the instance and preserve the way back", "./pred reduce cycle.json --via path.json -o reduced.json"),
    ("Solve the target and recover a source solution", "./pred solve reduced.json"),
    ("Check an independent set of size two", "./pred evaluate cycle.json --config '[true,false,true,false,false]'"),
    ("Solve the original instance with the default solver", "./pred solve cycle.json"),
]


def run_in_pty(command, cwd):
    master, slave = pty.openpty()
    env = dict(os.environ, TERM="xterm-256color", COLUMNS=str(WIDTH), LINES=str(HEIGHT))
    process = subprocess.Popen(shlex.split(command), cwd=cwd, stdin=slave, stdout=slave,
                               stderr=slave, env=env, close_fds=True)
    os.close(slave)
    chunks = []
    while True:
        try:
            data = os.read(master, 65536)
        except OSError:
            break
        if not data:
            break
        chunks.append(data)
    os.close(master)
    code = process.wait()
    if code != 0:
        raise SystemExit(f"{command!r} exited with {code}:\n{b''.join(chunks).decode()}")
    return b"".join(chunks).decode()


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--pred", default=ROOT / "target/debug/pred", type=Path)
    parser.add_argument("-o", "--output", default=ROOT / "docs/src/static/cli-demo.cast", type=Path)
    parser.add_argument("--seed", default=7, type=int)
    args = parser.parse_args()
    if not args.pred.exists():
        raise SystemExit(f"CLI binary not found: {args.pred}")

    rng = random.Random(args.seed)
    events, clock = [], 0.0

    def emit(text, delay=0.0):
        nonlocal clock
        clock += delay
        events.append([round(clock, 6), "o", text])

    with tempfile.TemporaryDirectory(prefix="pred-demo-") as work:
        os.symlink(args.pred.resolve(), Path(work) / "pred")
        for index, (comment, command) in enumerate(STEPS):
            if index:
                emit("\r\n", 0.2)
            emit(f"\x1b[90m# {comment}\x1b[0m\r\n")
            emit("$ ", 0.45)
            for char in command:
                emit(char, TYPING_DELAY * rng.uniform(0.6, 1.6))
            emit("\r\n", 0.3)
            output = run_in_pty(command, work).replace("\r\n", "\n").rstrip("\n")
            emit(output.replace("\n", "\r\n") + "\r\n", 0.15)
            clock += STEP_PAUSE
        emit("$ ", 0.2)

    header = {"version": 2, "width": WIDTH, "height": HEIGHT,
              "title": "From a graph to a verified solution",
              "env": {"TERM": "xterm-256color", "SHELL": "/bin/bash"}}
    cast = "".join(json.dumps(item) + "\n" for item in [header, *events])
    args.output.write_text(cast)
    player = args.output.with_name("cli-demo.html")
    html, count = re.subn(r"var castData = \".*?\";", lambda _: "var castData = " + json.dumps(cast) + ";",
                          player.read_text(), count=1, flags=re.S)
    if count != 1:
        raise SystemExit(f"castData not found in {player}")
    player.write_text(html)
    print(f"Wrote {args.output} and embedded it in {player.name} ({len(events)} events, {clock:.1f}s)")


if __name__ == "__main__":
    main()
