# BreakFree Sheets — Desktop UI specification

Status: working Rust native first layout; Windows reference comparison pending.
Owner decisions: Rust throughout; desktop **Microsoft Excel 2024 for Windows**
as visual/interaction target; **BreakFree Sheets** as the independent brand.

## Reference evidence and limits

Reference review date: 2026-10-16, Europe/Kyiv. The environment is macOS, not
Windows, and has no accessible Excel 2024 installation or supplied reference
screenshots. **No exact Excel build number, ribbon measurements or pixel-level
identity has been verified.** Microsoft 365/web/Mac images are not substituted
for the requested Windows 2024 reference.

The following primary sources were read and explicitly cover Excel/Office 2024:

| Source | Verified documentary evidence | What it does not verify |
| :--- | :--- | :--- |
| [Office 2024 / LTSC 2024 changes](https://support.microsoft.com/en-us/office/what-s-new-in-office-2024-and-office-ltsc-2024) | Modern default theme and Windows desktop direction | Exact Excel build, dimensions or screenshots for this machine |
| [Excel keyboard shortcuts](https://support.microsoft.com/en-us/accessibility/excel/keyboard-shortcuts-in-excel) | Windows navigation/editing and formula-bar interactions | That every shortcut is implemented here |
| [Ribbon keyboard navigation](https://support.microsoft.com/en-us/accessibility/windows/use-the-keyboard-to-work-with-the-ribbon) | Tabs, command groups and focus/KeyTips model | Alt KeyTips parity; those remain unimplemented |
| [Developer tab](https://support.microsoft.com/en-us/office/add-ins/show-the-developer-tab) | Optional, hidden-by-default tab | A requirement to expose unsupported macros here |
| [Ribbon customization](https://support.microsoft.com/en-us/office/foundations-experiences/customize-the-ribbon-in-office) | Tabs/groups can differ by user configuration | One universal default layout for every edition/install |

Home, Insert, Page Layout, Formulas, Data, Review, View and Help are visible;
File opens the working document menu. Developer is omitted because its tools
are unsupported and the documented default hides it. Extra/contextual tabs,
including Draw/Add-ins/Accessibility contexts, need verification on the chosen
Windows edition/configuration before being added; they are not simulated.

## Prototype geometry

All values below are **implemented logical UI points**, not measured Excel
values. Native title-bar borders and OS menu chrome are excluded. At native
DPI the toolkit maps logical points to physical pixels; Windows scaling changes
that mapping. Worksheet zoom is a separate control and does not scale the ribbon.

| Zone | Implemented geometry / behavior |
| :--- | :--- |
| Initial client area | 1366 × 768 logical points; minimum 980 × 600 |
| Title/quick-access strip | 34 high; approved icon 24 × 24, app and workbook names, Save/Undo/Redo; native OS title also shows workbook/app |
| File and ribbon tabs | 28 high; selected tab has teal emphasis |
| Expanded compact ribbon | 110 high; groups separated by thin lines; horizontal scrolling when the window is too narrow |
| Home groups | Clipboard 120, Font 164, Alignment 174, Number 142, Styles 154, Cells 140, Editing 138 wide, plus separators/spacing |
| Formula row | 32 high; name box 94 × 24; cancel/apply and unavailable function control; remaining width is the real source editor |
| Grid headers | Row gutter 44 wide; column header 24 high, pinned to viewport edges |
| Scrollbars | Solid 12-point tracks, horizontal and vertical; actual viewport scrolling |
| Default cells | Columns 80 wide, rows 20 high, 13-point fallback font; white surface with 0.6-point gray grid strokes |
| Selection | 2-point teal range border, pale teal range fill, white active cell; no nonfunctional fill handle |
| Resizing | Header-edge drag, column width 32–400 and row height 16–100; Home Cells also exposes numeric dimensions |
| Sheet strip | 28 high; Sheet1 active, sheet navigation/add disabled with explanations |
| Status | 24 high; Ready/Edit/Enter, selected address/range, errors and real grid zoom 50–200%; no Sum/Average/Count |

Palette follows pinned Office BRAND.md: teal `#248A83`, ink/aubergine `#241337`,
light neutral chrome and white grid. The dedicated approved Sheets WebP from
Sheets commit `e7de5a9745c5b29527e38c9022b58d6b53a9b686` is embedded unchanged;
alpha-corner tests and light-background rendering verify transparent exterior.
No Excel mark, Microsoft account/profile, cloud subscription or fake sign-in is
shown. Default toolkit fonts are used with their notices, not unlicensed Office
fonts. Therefore exact typeface/metrics parity is not claimed.

## State and supported behavior

Selection and edit are separate modes. Click selects; Shift+arrows/Shift+click
and drag extend a rectangular range. Arrows, Tab and Enter navigate within
A1–Z100. F2/double click starts inline source editing. Formula-bar focus edits the
same draft string; Enter applies and moves down, Tab applies and moves right,
Shift reverses direction, and Escape discards the draft. Committing updates the
same Rust editor/engine used by headless tests; no displayed result is invented.

The name field accepts an address or rectangle such as `A1:C4` followed by Enter.
There are no named-range definitions yet. New input uses safe Auto typing;
`00123` remains text. Existing kinds remain explicit. The Home Number selector
can intentionally change a source kind; currency/percent/display formatting
controls are disabled. Resizing/zoom changes only view state and never source
values. Those view dimensions reset on New/Open because native v1 does not yet
persist them.

Undo/redo restores cell sources and recalculates. Clear removes the selected
range in one undo command. Clipboard buttons copy/cut/paste one typed cell;
range paste and reference translation are unavailable. A pasted formula keeps
its literal references. Native New/Open/Save/Save As and close protection include
pending source text; failed Open/Save preserve the active model or prior file.

Font, Alignment, Styles and unimplemented Cells/Editing commands are disabled
with explanatory tooltips. Other tabs identify their unavailable commands.
Fill handle, sorting/filtering and additional sheets are unavailable. Windows
Alt KeyTips, formula-reference picking while editing, drag auto-scroll, rich
clipboard, screen-reader grid semantics and full keyboard ribbon parity remain
future work. File currently uses a native-app menu rather than a full Backstage
page. Supported operations keep familiar gestures; unavailable ones do not
pretend to execute.

## Captures and DPI verification

Real native Rust/OpenGL framebuffer captures were made on this macOS host with
client size 1366 × 768 logical points, Retina native factor 2, white/light theme,
`examples/first-workbook.bfsheet`, C1 selected, worksheet zoom 100%, fixed fonts
and no pending edit. OS decorations are not part of the framebuffer. Each image
is 2732 × 1536 physical pixels. Diagnostic UI zoom 1.0/1.25/1.5 changes the UI
inside that same client framebuffer; **this is not an OS display-DPI change**.

| Condition | Own application evidence | Matched Excel 2024 Windows capture | Status |
| :--- | :--- | :--- | :--- |
| macOS diagnostic 1.0 | [Native frame](screenshots/macos-diagnostic-100.png) | Not available | Real own-app capture; inspected for layout/glyph/selection issues |
| macOS diagnostic 1.25 | [Native frame](screenshots/macos-diagnostic-125.png) | Not available | Diagnostic only; narrower logical ribbon may scroll |
| macOS diagnostic 1.5 | [Native frame](screenshots/macos-diagnostic-150.png) | Not available | Diagnostic only; narrower logical ribbon may scroll |
| Windows display 100% | Not captured | Not captured | **Not verified: Windows/Excel unavailable** |
| Windows display 125% | Not captured | Not captured | **Not verified: Windows/Excel unavailable** |
| Windows display 150% | Not captured | Not captured | **Not verified: Windows/Excel unavailable** |

Headless egui tests additionally simulate three scale values and verify zone
ordering/visibility and bounded visible-cell painting. These are behavioral and
geometry tests, not visual screenshot equivalence or native Windows DPI tests.
Real snapshots are compared only within the same available macOS conditions;
no pixel-diff score against an absent Excel image is fabricated.

To reproduce own-app captures, build `cargo build --locked --features capture`,
set `BREAKFREE_CAPTURE_TO` to an absolute PNG path, optionally set
`BREAKFREE_CAPTURE_SCALE` to 1.0/1.25/1.5 and `BREAKFREE_CAPTURE_CELL=C1`, then
launch the binary with the absolute native example path. The capture build exits
after rendering. These developer-only variables are absent from the ordinary
product interaction flow.

For the required Windows comparison, record exact Excel 2024 edition/build,
Windows build, physical client resolution, display scale (100%,125%,150%), theme,
font, workbook data, active cell, worksheet zoom and ribbon state. Capture both
applications under identical settings; measure each zone and compare aligned
images at native pixels without resampling. Record differences and rerun the
editing/resize/scroll workflow at each DPI before claiming a match. This remains
an explicit acceptance gap, not a completed task hidden by simulated scaling.
