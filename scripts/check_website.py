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


def file_digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


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


def source_state(root=ROOT, output=None):
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=root)

    paths = ["--", "."]
    if output is not None and output.resolve().is_relative_to(root.resolve()):
        paths.append(":(exclude,literal)" + output.resolve().relative_to(root.resolve()).as_posix())
    untracked = [name for name in git("ls-files", "--others", "--exclude-standard", "-z", *paths).decode().split("\0") if name]
    nested = {}
    for name in untracked:
        if (root / name).is_dir():
            revision = subprocess.run(["git", "-C", str(root / name), "rev-parse", "--verify", "HEAD"],
                                      capture_output=True, text=True)
            nested[name] = revision.stdout.strip() if revision.returncode == 0 else None
    return {
        "revision": git("rev-parse", "HEAD").decode().strip(),
        "working_tree_status": git("status", "--porcelain", *paths).decode(),
        "working_tree_diff_sha256": hashlib.sha256(git("diff", "--binary", "HEAD", *paths)).hexdigest(),
        "untracked_source": {name: file_digest(root / name)
                             for name in untracked if (root / name).is_file()},
        "nested_checkouts": nested,
    }


def verify_source_unchanged(expected, root=ROOT, output=None):
    if source_state(root, output) != expected:
        raise ValueError("Source checkout changed during verification; retry after edits and commits are finished")


def write_provenance(site, verification, *, source=None):
    tools = {"python": sys.version, **{name: version(name) for name in
             ["playwright", "pyee", "greenlet", "typing_extensions"]}}
    for command in ["git", "cargo", "rustc", "node", "npm", "mdbook", "typst", "wasm-bindgen"]:
        tools[command] = subprocess.check_output([command, "--version"], text=True).strip() if shutil.which(command) else None
    evidence = {
        **(source if source is not None else source_state(output=site)),
        "lockfiles": {name: file_digest(ROOT / name) for name in
                      ["Cargo.lock", "package-lock.json", "scripts/uv.lock", "scripts/website-requirements.txt"]
                      if (ROOT / name).is_file()},
        "platform": platform.platform(), "tools": tools, "verification": verification,
        "github_runner": {"image": os.environ.get("ImageOS"), "image_version": os.environ.get("ImageVersion")},
        "artifacts": {path.relative_to(site).as_posix(): file_digest(path)
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
        if not path.is_file() or file_digest(path) != expected:
            raise ValueError(f"Artifact changed or missing since verification: {name}")
    return report


def check(site, *, build, open_questions, required):
    verification = {"status": "failed", "commands": [], "open_questions_required": required,
                    "mode": "fresh_build" if build else "existing_artifact",
                    "source_role": "build_and_verifier" if build else "verifier_only"}
    source = source_state(output=site)
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
        run(["npm", "run", "check:frontend-contracts"])
        run(["cargo", "test", "--locked", "-p", "problemreductions-web", "--features", "schema-export", "--test", "paths"])
        run(["npm", "run", "test:frontend-contracts"])
        run(["npm", "run", "typecheck"])
        run(["npm", "run", "lint:frontend"])
        if build:
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
            run([sys.executable, "-m", "unittest", "discover", "-s", "scripts", "-p", pattern, "--durations", "5", "-v"])
        with serve_website(site) as base:
            environment = {**os.environ, "WEBSITE_BASE_URL": base,
                           "WEBSITE_REQUIRE_OPEN_QUESTIONS": "1" if required else "0"}
            verification["browser_checks"] = {"root": [], "nested": [], "failure_recovery": []}
            for engine in ["chromium", "firefox", "webkit"]:
                run([sys.executable, "scripts/test_website.py", "--durations", "5"],
                    env={**environment, "WEBSITE_BROWSER": engine})
                verification["browser_checks"]["root"].append(engine)
                run([sys.executable, "scripts/test_website_failures.py", "--durations", "5", "-v"],
                    env={**environment, "WEBSITE_BROWSER": engine, "WEBSITE_SITE_DIR": str(site.resolve())})
                verification["browser_checks"]["failure_recovery"].append(engine)
        with tempfile.TemporaryDirectory(prefix="problem-reductions-nested-") as directory:
            root = Path(directory)
            (root / "research").mkdir()
            (root / "research/reductions").symlink_to(site.resolve(), target_is_directory=True)
            with serve_website(root) as base:
                cases = ["test_alias_search_and_reset", "test_large_resources_load_only_when_needed",
                         "test_graph_path_route_copy_solves_and_unreachable_query_is_explicit",
                         "test_graph_details_load_lazily_reuse_family_article_and_preserve_atlas_variant",
                         "test_reduction_evidence_and_typst_proof", "test_published_documentation_resources_and_csp",
                         "test_home_navigation_opens_question_board"]
                for engine in ["chromium", "firefox", "webkit"]:
                    run([sys.executable, "scripts/test_website.py", "--durations", "5", *["WebsiteTests." + name for name in cases]],
                        env={**os.environ, "WEBSITE_BASE_URL": base + "research/reductions/", "WEBSITE_BROWSER": engine,
                             "WEBSITE_REQUIRE_OPEN_QUESTIONS": "1" if required else "0"})
                    verification["browser_checks"]["nested"].append(engine)
        verification["source_unchanged"] = False
        verify_source_unchanged(source, output=site)
        verification["source_unchanged"] = True
        verification["status"] = "passed"
    finally:
        log.seek(0)
        (site / "verification.log").write_text(log.read())
        log.close()
        write_provenance(site, verification, source=source)


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
    tools = ["git", "node", "npm", "cargo", "rustc"]
    if args.site is None:
        tools += ["make", "bash", "python3", "mdbook", "typst", "wasm-bindgen"]
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
    try:
        check(site, build=args.site is None, open_questions=args.open_questions,
              required=args.require_open_questions or bool(os.environ.get("CI")) or args.open_questions is not None)
    except ValueError as error:
        parser.exit(1, f"{error}\nArtifact and provenance retained: {site}\n")
    print(f"Verified artifact and provenance: {site}")


if __name__ == "__main__":
    main()
