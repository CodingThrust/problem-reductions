"""Security and release contracts for the static website build."""

from pathlib import Path
import re
import tempfile
import unittest

from build_graph_details import SafeDetailHTML
from finalize_website import finalize


class WebsiteBuildTests(unittest.TestCase):
    def test_documentation_rejects_active_content(self):
        for markup in ['<script>alert(1)</script>', '<img onerror="alert(1)">',
                       '<a href="java&#10;script:alert(1)">bad</a>',
                       '<iframe src="https://example.com"></iframe>',
                       '<img src="https://example.com/track">']:
            with self.subTest(markup=markup), self.assertRaises(ValueError):
                SafeDetailHTML().feed(markup)
        SafeDetailHTML().feed('<p><a href="https://example.com">Reference</a>'
                              '<svg><use href="#glyph" /></svg></p>')

    def test_release_urls_and_csp_cover_inline_scripts(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'assets').mkdir()
            (root / 'assets/app.js').write_text('fetch("./assets/data.json")')
            (root / 'assets/data.json').write_text('{}')
            (root / 'index.html').write_text('<html><head><script>window.ready=true;</script>'
                '<script src="./assets/app.js"></script></head><body></body></html>')
            finalize(root)
            html = (root / 'index.html').read_text()
            self.assertIn("script-src 'self'", html)
            self.assertNotIn('<script>window.ready', html)
            scripts = re.findall(r'<script[^>]*src="([^"]+)"', html)
            self.assertEqual(len(scripts), 2)
            for script in scripts:
                self.assertTrue((root / script.split('?')[0]).is_file())
                self.assertIn('/assets/', script)
                self.assertIn('releases/', script)
            app = (root / scripts[1].split('?')[0]).read_text()
            self.assertIn('releases/', app)
            self.assertFalse((root / 'assets').exists())


if __name__ == '__main__':
    unittest.main()
