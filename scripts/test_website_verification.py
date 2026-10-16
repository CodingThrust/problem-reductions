"""Fresh artifact verification uses real files, HTTP, and browser requests."""

from pathlib import Path
from contextlib import redirect_stderr
import io
import os
import json
import shutil
import subprocess
import socket
import struct
import sys
import tempfile
import unittest
from urllib.request import build_opener, ProxyHandler
from urllib.parse import urlsplit

from playwright.sync_api import sync_playwright

from check_website import prepare_output, serve_website, write_provenance
import check_website
from test_website import require_open_questions


class WebsiteVerificationTests(unittest.TestCase):
    def test_canceled_asset_request_does_not_print_a_server_traceback(self):
        with tempfile.TemporaryDirectory() as directory:
            site = Path(directory)
            (site / "large.bin").write_bytes(b"x" * (8 * 1024 * 1024))
            errors = io.StringIO()
            with redirect_stderr(errors), serve_website(site) as base:
                target = urlsplit(base)
                with socket.create_connection((target.hostname, target.port), timeout=2) as client:
                    client.sendall(b"GET /large.bin HTTP/1.0\r\nHost: localhost\r\n\r\n")
                    headers = b""
                    while b"\r\n\r\n" not in headers:
                        chunk = client.recv(1024)
                        self.assertTrue(chunk)
                        headers += chunk
                    self.assertIn(b"200 OK", headers)
                    client.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, struct.pack("ii", 1, 0))
            self.assertEqual(errors.getvalue(), "")

    def test_verification_only_report_does_not_claim_the_artifact_build_source(self):
        with tempfile.TemporaryDirectory() as directory:
            site = Path(directory)
            with self.assertRaises(FileNotFoundError):
                check_website.check(site, build=False, open_questions=None, required=False)
            report = json.loads((site / "verification.json").read_text())
            self.assertEqual(report["verification"]["status"], "failed")
            self.assertEqual(report["verification"]["mode"], "existing_artifact")
            self.assertEqual(report["verification"]["source_role"], "verifier_only")

    def test_missing_tools_fail_before_creating_output(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "new-output"
            result = subprocess.run([sys.executable, str(Path(check_website.__file__)), "--output", str(output)],
                                    env={**os.environ, "PATH": "", "CARGO_HOME": directory},
                                    capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertFalse(output.exists())
            self.assertIn("mdbook", result.stderr)
            self.assertIn("wasm-bindgen", result.stderr)
            self.assertNotIn("Traceback", result.stderr)

    def test_cargo_bin_fallback_preserves_the_active_cargo(self):
        environment = dict(os.environ)
        active_cargo = shutil.which("cargo", path=environment["PATH"])
        self.assertIsNotNone(active_cargo)
        environment.pop("CARGO_HOME", None)
        path = check_website.tool_path(environment)
        self.assertIn(str(Path.home() / ".cargo/bin"), path.split(os.pathsep))
        self.assertEqual(shutil.which("cargo", path=path), active_cargo)

    def test_configured_cargo_bin_runs_real_executables(self):
        interpreter = Path(sys.executable).resolve()
        environment = {**os.environ, "PATH": "", "CARGO_HOME": str(interpreter.parent.parent)}
        environment["PATH"] = check_website.tool_path(environment)
        result = subprocess.run([interpreter.name, "-c", "print(6 * 7)"],
                                env=environment, check=True, capture_output=True, text=True)
        self.assertEqual(result.stdout.strip(), "42")

    def test_existing_output_is_preserved_and_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "existing"
            output.mkdir()
            marker = output / "index.html"
            marker.write_text("previous build")
            with self.assertRaises(FileExistsError):
                prepare_output(output)
            self.assertEqual(marker.read_text(), "previous build")
            fresh = prepare_output(Path(directory) / "fresh")
            self.assertTrue(fresh.is_dir())
            self.assertEqual(list(fresh.iterdir()), [])

    def test_server_serves_selected_artifact_and_closes_after_failure(self):
        direct = build_opener(ProxyHandler({}))
        with tempfile.TemporaryDirectory() as directory:
            site = Path(directory)
            (site / "index.html").write_text("selected artifact")
            with self.assertRaisesRegex(RuntimeError, "verification failed"):
                with serve_website(site) as base:
                    self.assertEqual(direct.open(base, timeout=1).read(), b"selected artifact")
                    raise RuntimeError("verification failed")
            with self.assertRaises(OSError):
                direct.open(base, timeout=1)

    def test_provenance_fingerprints_artifact_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            site = Path(directory)
            (site / "index.html").write_bytes(b"abc")
            evidence = write_provenance(site, {"result": "passed"})
            self.assertEqual(evidence["artifacts"]["index.html"],
                             "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
            self.assertEqual(evidence["verification"], {"result": "passed"})

    def test_baseline_inputs_must_match_the_verified_artifact(self):
        with tempfile.TemporaryDirectory() as directory:
            site = Path(directory)
            page = site / "index.html"
            page.write_text("verified contents")
            write_provenance(site, {"status": "passed"})
            self.assertIn("index.html", check_website.verified_artifact(site)["artifacts"])
            page.write_text("changed after verification")
            with self.assertRaises(ValueError):
                check_website.verified_artifact(site)

    def test_baseline_rejects_failed_reports_and_external_paths(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            site = root / "site"
            site.mkdir()
            (site / "index.html").write_text("abc")
            write_provenance(site, {"status": "failed"})
            with self.assertRaises(ValueError):
                check_website.verified_artifact(site)
            (root / "outside").write_text("abc")
            report = write_provenance(site, {"status": "passed"})
            report["artifacts"] = {"../outside": report["artifacts"]["index.html"]}
            (site / "verification.json").write_text(json.dumps(report))
            with self.assertRaises(ValueError):
                check_website.verified_artifact(site)

    def test_absent_question_board_skips_only_in_optional_mode(self):
        with tempfile.TemporaryDirectory() as directory, sync_playwright() as p:
            request = p.request.new_context()
            try:
                with serve_website(Path(directory)) as base:
                    with self.assertRaises(unittest.SkipTest):
                        require_open_questions(request, base, required=False)
                    with self.assertRaises(AssertionError):
                        require_open_questions(request, base, required=True)
                    board = Path(directory) / "open-questions"
                    board.mkdir()
                    (board / "registry.json").write_text('{"questions": []}')
                    require_open_questions(request, base, required=True)
            finally:
                request.dispose()

    def test_nested_checkout_is_recorded_without_hashing_a_directory(self):
        root = Path(__file__).resolve().parents[1]
        with tempfile.TemporaryDirectory(prefix="verification-checkout-", dir=root) as checkout:
            subprocess.run(["git", "init", "--quiet", checkout], check=True)
            (Path(checkout) / "README.md").write_text("separately built source")
            with tempfile.TemporaryDirectory() as directory:
                evidence = write_provenance(Path(directory), {"result": "passed"})
            self.assertIn(Path(checkout).name + "/", evidence["nested_checkouts"])
            self.assertIsNone(evidence["nested_checkouts"][Path(checkout).name + "/"])
            subprocess.run(["git", "-C", checkout, "-c", "user.name=Fixture",
                            "-c", "user.email=fixture@example.invalid", "-c", "commit.gpgsign=false",
                            "-c", "core.hooksPath=/dev/null", "commit", "--quiet", "--allow-empty", "-m", "Source snapshot"], check=True)
            expected = subprocess.check_output(["git", "-C", checkout, "log", "-1", "--format=%H"], text=True).strip()
            with tempfile.TemporaryDirectory() as directory:
                evidence = write_provenance(Path(directory), {"result": "passed"})
            self.assertEqual(evidence["nested_checkouts"][Path(checkout).name + "/"], expected)


if __name__ == "__main__":
    unittest.main()
