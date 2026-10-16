"""Capture real website states and check the resulting reference image sizes."""

import os
from pathlib import Path
import struct
import tempfile
import unittest

from playwright.sync_api import sync_playwright

import capture_website_baselines


@unittest.skipUnless(os.environ.get("WEBSITE_BASE_URL"), "Needs an explicitly served production artifact")
class WebsiteBaselineTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.playwright = sync_playwright().start()
        cls.browser = cls.playwright.chromium.launch()

    @classmethod
    def tearDownClass(cls):
        cls.browser.close()
        cls.playwright.stop()

    def capture(self, width, height):
        with tempfile.TemporaryDirectory() as directory:
            context = self.browser.new_context(viewport={"width": width, "height": height},
                                              reduced_motion="reduce", device_scale_factor=1)
            try:
                capture_website_baselines.capture(context.new_page(), os.environ["WEBSITE_BASE_URL"], Path(directory))
                self.assertEqual({p.name for p in Path(directory).glob("*.png")}, {
                    "home.png", "atlas.png", "graph-route.png", "graph-popover.png",
                    "graph-drawer.png", "atlas-empty.png", "unknown-record.png"})
                for path in Path(directory).glob("*.png"):
                    self.assertEqual(struct.unpack(">II", path.read_bytes()[16:24]), (width, height), path.name)
            finally:
                context.close()

    def test_desktop_reference_images(self):
        self.capture(1440, 1000)

    def test_mobile_reference_images(self):
        self.capture(390, 844)


if __name__ == "__main__":
    unittest.main()
