# Accessibility engineering evidence

This increment preserves the initial appearance. All fourteen desktop/mobile PNG
hashes match the initial capture; `accessibility-screenshots.json.gz` records the
new capture/source hashes. No aesthetic work was performed.

`accessibility-source-verification.json.gz` retains the fresh required-board
artifact's passed production verification: 73 browser workflows plus mathematical,
build, provenance, graph-harness and reference-capture checks. The build includes
the accessibility fixes on parent revision `0cea215f`; the report records the dirty
source hashes. The later test-only refinement checks vertical overflow after focus
and uses a 100 ms keypress rather than an immediate down/up pair.

`accessibility-scan.json.gz` includes the exact temporary cross-browser runner,
test-source hash, complete output and 24 axe reports. It reuses the production
test methods, serves the same unchanged artifact and uses real browser engines.
Firefox explicitly uses direct loopback access. No clipboard permissions are needed
by these cases. All twelve cases per engine passed: eight axe states and four
functional checks for combobox state/refocus, variant focus, footnote focus and
keyboard scrolling. Versions: Playwright 1.58.0; Chromium 145.0.7632.6, Firefox
146.0.1 and WebKit 26.0; axe-core 4.14.0. This is focused compatibility evidence,
not the complete supported-browser acceptance suite.

WebKit did not give the original overflowing example a native Tab stop. With an
explicit Tab stop, an immediate first arrow down/up pair still did not scroll;
a 100 ms press did. The initial failure and diagnostic are retained in
`accessibility-first-scroll-failure.log.gz`. Native scrolling remains in use.
Real Safari/VoiceOver testing must check this interaction independently.

The scans inject the pinned test-only axe bundle through Playwright evaluation;
the page CSP stays in force. Serious/critical violations and invalid explicit roles
block tests. Lower-impact violations and incomplete results remain review items,
not passes. All three engines report:

- Atlas details: heading-order violation, and an incomplete check of the named
  generic problem-tools container. Some contrast checks need review.
- Desktop Graph details: duplicate/nonunique banner landmarks. The mobile modal
  hides the background banner and does not report these violations.
- Graph states: incomplete contrast checks, often because the full-screen SVG
  prevents axe from resolving backgrounds. These require direct contrast review.
- Drawer states: incomplete label/content-name checks for mathematical SVG content.

Reports preserve each affected selector and reason. The scan only evaluates rendered
states; it cannot certify other states or real assistive-technology behavior.
Follow the required [manual acceptance script](../../ACCESSIBILITY_CHECKS.md).
Manual keyboard/VoiceOver and physical mobile results remain unrecorded, and Stage 1
remains incomplete. Roll back this isolated increment with its test-only dependency;
retain the evidence and known defects until the replacement passes the same checks.
