"""Publish immutable asset URLs and a CSP for the generated static site."""

from pathlib import Path
import re
import time


def finalize(output):
    release = f"releases/{time.time_ns()}"
    policy = (
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; "
        "img-src 'self' data:; font-src 'self'; connect-src 'self'; "
        "frame-src 'self'; object-src 'none'; base-uri 'self'; form-action 'none'"
    )
    for page in output.rglob("*.html"):
        if "releases" in page.relative_to(output).parts or "assets" in page.relative_to(output).parts:
            continue
        html = page.read_text()
        if "<head" not in html:
            continue
        if 'http-equiv="Content-Security-Policy"' in html:
            continue
        root = "../" * (len(page.relative_to(output).parts) - 1)
        index = 0

        def externalize(match):
            nonlocal index
            attrs, code = match.groups()
            if re.search(r"\bsrc\s*=", attrs) or not code.strip():
                return match.group(0)
            if 'type="application/json"' in attrs:
                return match.group(0)
            target = Path("assets/inline") / page.relative_to(output) / f"{index}.js"
            index += 1
            (output / target).parent.mkdir(parents=True, exist_ok=True)
            (output / target).write_text(code)
            return f'<script{attrs} src="{root}{target.as_posix()}"></script>'

        html = re.sub(r"<script\b([^>]*)>(.*?)</script>", externalize, html, flags=re.S)
        html = re.sub(r'((?:src|href)="[^"?#]+\.(?:js|css))"',
                      lambda m: m[1] + f'?release={release.split("/")[1]}"', html)
        html = re.sub(r"(<head\b[^>]*>)", lambda m: m[0] +
                      f'<meta http-equiv="Content-Security-Policy" content="{policy}">', html, count=1)
        page.write_text(html)
    # Relative references in CSS, scripts, and generated manifests follow the
    # same build directory. Older open pages cannot read newer article payloads.
    for path in output.rglob("*"):
        if not path.is_file() or path.suffix not in {".html", ".js", ".css", ".json"}:
            continue
        if "releases" in path.relative_to(output).parts:
            continue
        content = path.read_text()
        content = re.sub(r'(?<=["\'/])assets/',
                         lambda m: m[0] if re.search(r"releases/\d+/$", content[:m.start()])
                         else release + "/assets/", content)
        path.write_text(content)
    destination = output / release
    destination.mkdir(parents=True)
    (output / "assets").rename(destination / "assets")
