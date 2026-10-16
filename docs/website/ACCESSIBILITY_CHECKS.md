# Focused manual engineering acceptance

Run this after the automated production-artifact checks, against the same artifact.
This is functional/accessibility acceptance; preserve the current visual direction.
Automated accessibility scans and Playwright WebKit do not replace these checks.

Record the date, engineering commit, artifact verification/hash, hardware, OS,
browser, screen reader/version, input method and viewport. Record what was actually
announced and whether each task succeeded. Do not infer results from DOM attributes.
No manual result has been recorded yet.

Desktop target: Safari 27.0.1 on macOS with VoiceOver. Mobile target: a physical
device/browser/screen reader must be named in the result; device selection is
pending. Use a keyboard-only pass with the screen reader off, then a screen-reader
pass. Configure ordinary keyboard navigation to reach webpage controls, and record
any screen-reader interaction mode required to operate the graph.

## Desktop tasks

1. Navigate Home → Atlas using the keyboard. Search `MIS`; confirm one Maximum
   Independent Set result and open its definition. Confirm headings, mathematical
   content and links have understandable reading order and names. Clear the query,
   enter an impossible name, and use Reset to recover from the empty state.
2. Open Graph. Confirm initial loading and completion are announced without reading
   the entire route panel repeatedly. Confirm the main navigation, search fields,
   graph, results and zoom/navigation controls are discoverable and named.
3. Use `/` to reach search. Type `MIS`, navigate suggestions with arrows, choose with
   Enter, then choose QUBO as target. Confirm suggestion names, active option and
   expanded/collapsed state are understandable. Escape closes suggestions and
   leaves no stale active-option announcement.
4. Tab into the graph. Confirm a single entry stop, visible focus and an audible
   problem name. Follow the documented arrow behavior. Reach an isolated node using
   the documented non-neighbor traversal, and confirm focus reveals obscured nodes.
   Confirm selected-route/start/end information is understandable on affected nodes.
5. Activate a node with Enter and Space. Confirm the popover is named and focus
   enters its controls. Change a variant without losing usable focus. Escape closes
   it and returns to the originating node. Pointer activation must still work.
6. Open Details from a node and from a route rule. Confirm the definition/proof
   content is readable. On desktop, confirm the drawer is nonmodal: Tab can leave it,
   and background controls remain operable. Close it with Escape and the close
   button; confirm focus returns to a visible, meaningful control.
7. Select another route, expand its steps and copy it. Confirm the copy result or
   selectable-text fallback is announced and usable. The copied route must still
   pass the existing independent CLI reduction/solve workflow.
8. Operate the provided click controls for pan/zoom and node positioning without
   dragging. Confirm equivalent content remains reachable using keyboard controls,
   and that pointer/touch dragging still behaves correctly.
9. During a real path search, cancel and run it again. Confirm status announcements,
   recovered controls and the final route. Exercise error/retry using the documented
   failure-test setup once implemented; confirm an actionable message and successful
   recovery. That fixture is pending; cancellation alone cannot pass this task.

## Physical mobile tasks

1. At the device's normal text size, use the screen reader to navigate Home, Atlas
   search and Graph search. Repeat at an increased text size. Confirm controls are
   reachable, labels are announced, and page/panel scrolling reveals all content.
2. Open a graph node and Details using touch exploration and screen-reader
   activation. Confirm the mobile drawer is announced as a modal, focus enters it,
   background content cannot receive focus/activation, and its article can scroll.
3. Close Details and confirm focus returns to the originating node/control. With a
   hardware keyboard if available, check Tab containment, Shift+Tab, and Escape.
   Record hardware-keyboard checks as unavailable if no keyboard is available.
4. Change orientation or cross the responsive breakpoint with the drawer open.
   Confirm modal/nonmodal behavior changes correctly and focus remains usable.
5. Use click/tap alternatives to drag and pinch, then exercise ordinary pan, drag,
   pinch, route selection, cancellation, copy and selectable-text fallback. Confirm
   no gesture requirement makes a mathematical workflow inaccessible.

## Reporting and acceptance

Use one row per failed or unavailable task with its observed announcement/behavior,
reproduction steps, severity, artifact/browser versions and a concrete follow-up.
Record successful tasks explicitly too. Required manual tasks remain unverified
until executed; an unavailable device or screen reader is not a pass. Keep Stage 1
and the final engineering handoff incomplete while blocking defects or required
manual checks remain open. Recheck affected tasks after a fix, preserving independent
mathematical, screenshot and performance evidence. Claude's visual phase starts
only after the complete engineering handoff gate passes.
