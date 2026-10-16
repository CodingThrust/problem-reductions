#!/usr/bin/env python3
"""Capture current appearance, with state assertions and reproducible browser settings."""

import argparse
import hashlib
from importlib.metadata import version
import json
from pathlib import Path
import platform

from playwright.sync_api import expect, sync_playwright

from check_website import prepare_output, serve_website, verified_artifact


def capture(page, base, output):
    page.set_default_timeout(3000)
    errors = []
    page.on("pageerror", lambda error: errors.append(str(error)))
    output.mkdir(parents=True, exist_ok=True)

    def save(name):
        page.evaluate("() => document.fonts.ready")
        page.evaluate("() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))")
        assert not errors, errors
        page.screenshot(path=str(output / f"{name}.png"), animations="disabled", caret="hide")

    page.goto(base)
    expect(page.locator("main .hero")).to_be_visible()
    save("home")
    page.goto(base + "#atlas")
    expect(page.locator(".atlas-row").first).to_be_visible()
    save("atlas")
    page.goto(base + "graph.html")
    expect(page.locator("#results .summary strong")).to_contain_text("best route")
    expect(page.locator(".path").first).to_be_visible()
    expect(page.locator(".size.pending")).to_have_count(0)
    save("graph-route")
    if page.locator('[data-node="Satisfiability"] .node-disc').count():
        page.locator('[data-node="Satisfiability"] .node-disc').click()
    else:
        point = page.evaluate("() => document.querySelector('#cy')._cyreg.cy.getElementById('Satisfiability').renderedPosition()")
        page.locator("#cy").click(position=point)
    expect(page.locator("#pop")).to_be_visible()
    save("graph-popover")
    page.get_by_role("button", name="Details", exact=True).click()
    expect(page.locator("#details-drawer")).to_be_visible()
    expect(page.locator("#drawer-body article")).to_contain_text("Definition")
    save("graph-drawer")
    page.goto(base + "#atlas")
    page.locator("#atlas-search").fill("NoSuchRegisteredProblem")
    expect(page.locator(".empty-state")).to_be_visible()
    expect(page.locator(".atlas-row")).to_have_count(0)
    save("atlas-empty")
    page.goto(base + "#problem/NotARegisteredProblem")
    expect(page.get_by_text("This connection is not in the atlas.")).to_be_visible()
    save("unknown-record")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("site", type=Path, help="Verified production artifact")
    parser.add_argument("--output", type=Path, required=True, help="New reference directory; existing paths refused")
    args = parser.parse_args()
    manifest = args.site / "verification.json"
    try:
        verification = verified_artifact(args.site)
        output = prepare_output(args.output)
    except (OSError, ValueError) as error:
        parser.error(str(error))
    settings = {"device_scale_factor": 1, "reduced_motion": "reduce", "locale": "en-US", "timezone_id": "UTC"}
    viewports = [("desktop", 1440, 1000), ("mobile", 390, 844)]
    with serve_website(args.site.resolve()) as base, sync_playwright() as p:
        browser = p.chromium.launch()
        try:
            for name, width, height in viewports:
                context = browser.new_context(viewport={"width": width, "height": height}, **settings)
                try:
                    capture(context.new_page(), base, output / name)
                finally:
                    context.close()
            browser_version = browser.version
        finally:
            browser.close()
    verified_artifact(args.site)
    evidence = {"status": "reference_only", "blocking": False, "platform": platform.platform(),
                "browser": browser_version, "playwright": version("playwright"), "settings": settings,
                "viewports": {name: {"width": width, "height": height} for name, width, height in viewports},
                "capture_script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                "source_verification_sha256": hashlib.sha256(manifest.read_bytes()).hexdigest(),
                "source_graph_html_sha256": verification["artifacts"]["graph.html"],
                "images": {p.relative_to(output).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
                           for p in sorted(output.rglob("*.png"))}}
    (output / "manifest.json").write_text(json.dumps(evidence, indent=2) + "\n")
    print(f"Captured {len(evidence['images'])} reference images in {output}; no pixel gate enabled")


if __name__ == "__main__":
    main()
