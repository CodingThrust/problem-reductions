"""Fresh artifact verification uses real files, HTTP, and browser requests."""

from pathlib import Path
import subprocess
import tempfile
import unittest
from urllib.request import build_opener, ProxyHandler

from playwright.sync_api import sync_playwright

from check_website import prepare_output, serve_website, write_provenance
from test_website import require_open_questions


class WebsiteVerificationTests(unittest.TestCase):
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
