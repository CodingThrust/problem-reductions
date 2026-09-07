"""Run with python3 scripts/test_website_math.py."""

import unittest
import xml.etree.ElementTree as ET

from build_website import expression_mathml


class WebsiteMathTests(unittest.TestCase):
    def test_expression_structure_and_accessible_source(self):
        source = "(num_vertices + 1)^2 / sqrt(num_edges) - factorial(k)"
        math = ET.fromstring(expression_mathml(source))
        ns = {"m": "http://www.w3.org/1998/Math/MathML"}
        self.assertEqual(math.attrib["aria-label"], source)
        power = math.find(".//m:mfrac/m:msup", ns)
        self.assertEqual("".join(power[0].itertext()), "(num_vertices+1)")
        self.assertEqual(power[1].text, "2")
        self.assertEqual(math.find(".//m:msqrt/m:mi", ns).text, "num_edges")
        self.assertIn("k!", "".join(math.itertext()))
        with self.assertRaises(ValueError):
            expression_mathml("unknown(num_vertices)")


if __name__ == "__main__":
    unittest.main()
