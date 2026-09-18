#!/usr/bin/env python3
"""Compile the Typst reference once into lazily loaded graph detail articles."""

from html.parser import HTMLParser
import json
import re
from pathlib import Path
import subprocess
import tempfile
import time
from urllib.parse import quote


class SafeDetailHTML(HTMLParser):
    def handle_starttag(self, tag, attrs):
        if tag in {"script", "iframe", "object", "embed", "base", "link", "meta", "form"}:
            raise ValueError(f"Active element in documentation: {tag}")
        for key, value in attrs:
            if key.lower().startswith("on"):
                raise ValueError(f"Event handler in documentation: {key}")
            if key in {"href", "src", "xlink:href"} and value:
                url = re.sub(r"[\x00-\x20]", "", value).lower()
                if ":" in url.split("/")[0] and not url.startswith(("https:", "http:", "mailto:")):
                    raise ValueError(f"Unsafe URL in documentation: {key}")
                if (key != "href" or tag in {"image", "use"}) and not value.startswith("#"):
                    raise ValueError(f"Non-local resource in documentation: {key}")

    handle_startendtag = handle_starttag

ROOT = Path(__file__).resolve().parents[1]


class DetailArticles(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=False)
        self.articles = {}
        self.key = None
        self.parts = []
        self.depth = 0
        self.anchors = {}

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if self.key is None and ("data-detail-key" in attrs or attrs.get("role") == "doc-endnotes"):
            self.key = attrs.get("data-detail-key", "footnotes")
            if self.key in self.articles:
                raise ValueError(f"Duplicate Typst detail: {self.key}")
            self.parts = []
            self.depth = 1
        elif self.key is not None:
            self.parts.append(self.get_starttag_text())
            if tag not in {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}:
                self.depth += 1
        if self.key is not None and "id" in attrs and tag in {"article", "a", "figure", "span", "li", "h1", "h2", "h3", "h4", "h5", "h6", "section"}:
            self.anchors[attrs["id"]] = self.key

    def handle_endtag(self, tag):
        if self.key is not None:
            self.depth -= 1
            if self.depth == 0:
                self.articles[self.key] = "".join(self.parts)
                self.key = None
            else:
                self.parts.append(f"</{tag}>")

    def handle_startendtag(self, tag, attrs):
        if self.key is not None:
            self.parts.append(self.get_starttag_text())

    def handle_data(self, value):
        if self.key is not None:
            self.parts.append(value)

    def handle_entityref(self, name):
        self.handle_data(f"&{name};")

    def handle_charref(self, name):
        self.handle_data(f"&#{name};")


def deduplicate_glyphs(content):
    """Typst repeats identical SVG symbol definitions in each equation."""
    symbols = {}

    def collect(match):
        symbol = match.group(0)
        identifier = re.search(r'\bid="([^"]+)"', symbol).group(1)
        symbols[identifier] = symbol
        return ""

    content = re.sub(r"<symbol\b[^>]*>.*?</symbol>", collect, content, flags=re.S)
    content = content.replace("<defs></defs>", "")
    if symbols:
        content = '<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" style="position:absolute;width:0;height:0;overflow:hidden"><defs>' + "".join(symbols.values()) + "</defs></svg>" + content
    return content


def build_details(output):
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="reduction-details-") as directory:
        html_path = Path(directory) / "details.html"
        subprocess.run([
            "typst", "compile", "--root", str(ROOT), "--features", "html",
            "--input", "details=true",
            str(ROOT / "docs/paper/reductions.typ"), str(html_path),
        ], check=True)
        parser = DetailArticles()
        parser.feed(html_path.read_text())
    if not any(key.startswith("problem:") for key in parser.articles) or not any(key.startswith("rule:") for key in parser.articles):
        raise ValueError("Typst export must contain problems and rules")
    output.mkdir(parents=True, exist_ok=True)
    entries = {}
    original_bytes = total_bytes = 0
    for key, content in parser.articles.items():
        SafeDetailHTML().feed(content)
        original_bytes += len(content.encode())
        content = deduplicate_glyphs(content)
        kind, _, name = key.partition(":")
        relative = Path("details") / kind / (name.replace("/", "__").replace("->", "--") + ".html") if name else Path("details") / (kind + ".html")
        path = output / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)
        entries[key] = "./assets/" + quote(relative.as_posix(), safe="/")
        total_bytes += path.stat().st_size
    (output / "graph-details.json").write_text(
        json.dumps({"entries": entries, "anchors": parser.anchors}, ensure_ascii=True))
    print(f"Built {len(entries)} detail files in {time.monotonic() - started:.1f}s: "
          f"{original_bytes / 1e6:.1f} MB → {total_bytes / 1e6:.1f} MB; "
          f"index {(output / 'graph-details.json').stat().st_size / 1e3:.1f} KB")


if __name__ == "__main__":
    build_details(ROOT / "book/assets")
