#!/usr/bin/env python3
"""Build a fresh production website, verify it, and retain artifact provenance.

uv run --no-project --with playwright==1.58.0 python scripts/check_website.py
Use --site to verify an already assembled deployment artifact without rebuilding.
"""

import argparse
from contextlib import contextmanager
from functools import partial
import hashlib
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from importlib.metadata import version
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tempfile
from threading import Thread

ROOT = Path(__file__).resolve().parents[1]


def tool_path(environment):
    current = environment.get("PATH", "")
    cargo_bin = str(Path(environment.get("CARGO_HOME") or Path.home() / ".cargo") / "bin")
    if cargo_bin in current.split(os.pathsep):
        return current
    return os.pathsep.join(part for part in [current, cargo_bin] if part)


class PreviewServer(ThreadingHTTPServer):
    # The cold asset burst exceeds the default accept queue on some Python versions.
    request_queue_size = 128


class Handler(SimpleHTTPRequestHandler):
    def handle(self):
        try:
            super().handle()
        except (BrokenPipeError, ConnectionResetError):
            pass  # Browser navigation and worker cancellation can abandon an asset response.

    def log_message(self, *args):
        pass


def prepare_output(path=None):
    if path is None:
        return Path(tempfile.mkdtemp(prefix="problem-reductions-website-"))
    path = path.resolve()
    path.mkdir(parents=True)
    return path


@contextmanager
def serve_website(site):
    handler = partial(Handler, directory=str(site))
    with PreviewServer(("127.0.0.1", 0), handler) as server:
        thread = Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            yield f"http://127.0.0.1:{server.server_port}/"
        finally:
            server.shutdown()
            thread.join()


def write_provenance(site, verification):
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=ROOT)

    def digest(path):
        return hashlib.sha256(path.read_bytes()).hexdigest()

    tools = {"python": sys.version, **{name: version(name) for name in
             ["playwright", "pyee", "greenlet", "typing_extensions"]}}
    for command in ["cargo", "rustc", "node", "npm", "mdbook", "typst", "wasm-bindgen"]:
        tools[command] = subprocess.check_output([command, "--version"], text=True).strip() if shutil.which(command) else None
    untracked = [name for name in git("ls-files", "--others", "--exclude-standard", "-z").decode().split("\0") if name]
    nested = {}
    for name in untracked:
        if (ROOT / name).is_dir():
            revision = subprocess.run(["git", "-C", str(ROOT / name), "rev-parse", "--verify", "HEAD"],
                                      capture_output=True, text=True)
            nested[name] = revision.stdout.strip() if revision.returncode == 0 else None
    evidence = {
        "revision": git("rev-parse", "HEAD").decode().strip(),
        "working_tree_status": git("status", "--porcelain").decode(),
        "working_tree_diff_sha256": hashlib.sha256(git("diff", "--binary", "HEAD")).hexdigest(),
        "untracked_source": {name: digest(ROOT / name) for name in untracked if (ROOT / name).is_file()},
        "nested_checkouts": nested,
        "lockfiles": {name: digest(ROOT / name) for name in
                      ["Cargo.lock", "package-lock.json", "scripts/uv.lock", "scripts/website-requirements.txt"]
                      if (ROOT / name).is_file()},
        "platform": platform.platform(), "tools": tools, "verification": verification,
        "github_runner": {"image": os.environ.get("ImageOS"), "image_version": os.environ.get("ImageVersion")},
        "artifacts": {path.relative_to(site).as_posix(): digest(path)
                      for path in sorted(site.rglob("*")) if path.is_file() and path != site / "verification.json"},
    }
    (site / "verification.json").write_text(json.dumps(evidence, indent=2) + "\n")
    return evidence


def verified_artifact(site):
    report = json.loads((site / "verification.json").read_text())
    if report.get("verification", {}).get("status") != "passed" or not report.get("artifacts"):
        raise ValueError("Use a production artifact with passed verification and artifact hashes")
    for name, expected in report["artifacts"].items():
        path = site / name
        if not path.resolve().is_relative_to(site.resolve()):
            raise ValueError(f"Artifact path leaves the verified directory: {name}")
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"Artifact changed or missing since verification: {name}")
    return report


def check(site, *, build, open_questions, required):
    verification = {"status": "failed", "commands": [], "open_questions_required": required,
                    "mode": "fresh_build" if build else "existing_artifact",
                    "source_role": "build_and_verifier" if build else "verifier_only"}
    log = tempfile.TemporaryFile(mode="w+")

    def run(command, **kwargs):
        verification["commands"].append(command)
        print("Running:", command, flush=True)
        log.write(f"\nRunning: {command}\n")
        log.flush()
        try:
            subprocess.run(command, cwd=ROOT, check=True, stdout=log, stderr=subprocess.STDOUT, **kwargs)
        except subprocess.CalledProcessError:
            log.seek(0)
            print("".join(log.readlines()[-40:]), file=sys.stderr)
            raise

    try:
        if build:
            run(["npm", "ci"])
            run(["make", "website-data"])
            run(["mdbook", "build", "--dest-dir", str(site)])
            run(["typst", "compile", "--root", ".", "docs/paper/reductions.typ", str(site / "reductions.pdf")])
            run(["cargo", "doc", "--no-deps"], env={**os.environ, "RUSTDOCFLAGS": "--default-theme=dark"})
            shutil.copytree(ROOT / "target/doc", site / "api")
            command = [sys.executable, "scripts/build_website.py", "--output", str(site)]
            if open_questions is not None:
                command += ["--open-questions", str(open_questions)]
            run(command)
        for name in ["index.html", "graph.html", "introduction.html", "reductions.pdf", "api/problemreductions/index.html"]:
            if not (site / name).is_file():
                raise FileNotFoundError(f"Incomplete production artifact: {site / name}")
        if required:
            for name in ["index.html", "registry.json"]:
                if not (site / "open-questions" / name).is_file():
                    raise FileNotFoundError(f"Required Open Question Board output absent: {name}")
        run(["npm", "run", "test:reduction-graph-js"])
        for pattern in ["test_website_build.py", "test_website_math.py", "test_website_verification.py"]:
            run([sys.executable, "-m", "unittest", "discover", "-s", "scripts", "-p", pattern, "-v"])
        with serve_website(site) as base:
            environment = {**os.environ, "WEBSITE_BASE_URL": base,
                           "WEBSITE_REQUIRE_OPEN_QUESTIONS": "1" if required else "0"}
            run([sys.executable, "-m", "unittest", "discover", "-s", "scripts", "-p", "test_graph_*", "-v"],
                env=environment)
            run([sys.executable, "scripts/test_website.py"], env=environment)
            run([sys.executable, "-m", "unittest", "discover", "-s", "scripts",
                 "-p", "test_website_baselines.py", "-v"], env=environment)
        verification["status"] = "passed"
    finally:
        log.seek(0)
        (site / "verification.log").write_text(log.read())
        log.close()
        write_provenance(site, verification)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--output", type=Path, help="Fresh output directory; existing paths are refused")
    mode.add_argument("--site", type=Path, help="Verify an existing complete production artifact")
    parser.add_argument("--open-questions", type=Path, help="Built Open Question Board directory to include")
    parser.add_argument("--require-open-questions", action="store_true", help="Fail instead of skipping when board output is absent")
    args = parser.parse_args()
    if args.site and args.open_questions:
        parser.error("--open-questions applies to fresh builds; --site verifies the existing artifact")
    if args.open_questions and not all((args.open_questions / name).is_file() for name in ["index.html", "registry.json"]):
        parser.error("--open-questions must contain index.html and registry.json")
    os.environ["PATH"] = tool_path(os.environ)
    tools = ["git", "node", "npm"]
    if args.site is None:
        tools += ["make", "bash", "python3", "cargo", "rustc", "mdbook", "typst", "wasm-bindgen"]
    missing = [command for command in tools if shutil.which(command) is None]
    if missing:
        parser.error(f"Missing required tools: {', '.join(missing)}. "
                     "Install them before retrying; see docs/website/ENGINEERING.md.")
    try:
        site = args.site.resolve() if args.site else prepare_output(args.output)
        if not site.is_dir():
            raise FileNotFoundError(site)
    except OSError as error:
        parser.exit(2, f"{error}\nUse a new --output path or --site for an existing artifact.\n")
    print(f"Verification artifact: {site}", flush=True)
    check(site, build=args.site is None, open_questions=args.open_questions,
          required=args.require_open_questions or bool(os.environ.get("CI")) or args.open_questions is not None)
    print(f"Verified artifact and provenance: {site}")


if __name__ == "__main__":
    main()
