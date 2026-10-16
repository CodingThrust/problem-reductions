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

The following isolated contrast fix reuses the existing secondary text color for
small metadata, placeholders and rule links. The popup metadata test failed at
3.57925:1 before the fix and passes at 7.49836:1 against its opaque background.
The test passed in all three pinned engines; the fresh required-board build passed
all 74 browser workflows and the existing checks. `contrast-screenshots.json.gz`
records its capture: only Graph route/popover/drawer images changed at each viewport;
the eight Home/Atlas/empty/unknown images remain identical. Layout, palette values
and fonts did not change. Other incomplete contrast results still require review.

The next isolated increment adds click/tap pan controls, popup node-movement buttons,
mobile zoom/fit access and bounded popup scrolling. The fresh required-board build
passed all 78 workflows plus the existing mathematical/build/harness checks. The
four new behavior cases failed against the old runtime before implementation.
The two mobile cases also passed in all three pinned engines, including emulated
touch taps; physical-device results remain unverified. The
`navigation-controls-screenshots.json.gz` capture records the necessary Graph
control changes. Original references remain intact. No aesthetic styling or palette
change accompanies these controls.

The camera/responsive/semantic follow-up passes all 79 workflows in a fresh
required-board production build. `accessibility-followup-source-verification.json.gz`
records its unchanged source snapshot and complete artifact hashes;
`accessibility-followup.json.gz` retains all eight Chromium scans, now with zero
violations and explicit incomplete results. It also records four focused tests per
engine against the preceding scratch overlay build of the same application source.
This focused compatibility scope remains smaller than full browser acceptance.

The tests reproduce and fix startup framing that obscured focused isolated Bin
Packing, exercise modal isolation below 1168px and nonmodal operation at/above it,
and check the corrected Atlas heading/group and drawer landmark semantics. The
shared article check preserves text, exact formula markup and links while accepting
the required h3/h2 embedding difference. All fourteen reference hashes match the
preceding navigation-controls capture (`focus-semantics-screenshots.json.gz`).
Atlas detail heading computed styles and full-page screenshots also match at 1440px
and 390px in Chromium. These resolve the earlier heading/group/banner findings;
incomplete contrast/mathematical-label results and real manual acceptance remain open.
