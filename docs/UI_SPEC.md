# BreakFree Sheets — Desktop UI specification

Status: refined Rust native ribbon/File layout; Windows reference comparison pending.
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
| [Cell text formatting](https://support.microsoft.com/en-us/excel/format-text-in-cells) | Home font/alignment command organization; page explicitly includes Excel 2024 | Mixed-version/web illustrations are not matched Windows 2024 screenshots |
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
| Title/quick-access strip | 38 high; approved icon 24 × 24, workbook/app names, flat Save/Undo/Redo pictograms; native OS title also shows workbook/app |
| File and ribbon tabs | 31 high; flat tabs, selected tab has a 2.5-point teal underline |
| Expanded compact ribbon | 105 high, 119 below 1280 logical client width to reserve the solid scrollbar; white inset surface with 5-point corners and thin group separators |
| Home groups | Clipboard 168, Font 181, Alignment 222, Number 135, Styles 182, Cells 144, Editing 210 wide, plus separators/spacing |
| Formula row | 34 high; framed name box 100 × 25; cancel/check pictograms and unavailable function control; remaining width is the real source editor |
| Grid headers | Row gutter 44 wide; column header 24 high, pinned to viewport edges |
| Scrollbars | Solid 12-point tracks, horizontal and vertical; actual viewport scrolling |
| Default cells | Columns 80 wide, rows 20 high, 13-point fallback font; white surface with 0.5-point gray grid strokes |
| Selection | 2-point teal range border, pale teal range fill, white active cell; selected headers have teal edge strokes; no nonfunctional fill handle |
| Resizing | Header-edge drag, column width 32–400 and row height 16–100; Home Cells → Format opens numeric dimensions |
| Sheet strip | 30 high; white active Sheet1 tab with teal underline; Sheet1 active, sheet navigation/add disabled with explanations |
| Status | 25 high; Ready/Edit/Enter, selected address/range, errors, Normal view and disabled page view, minus/slider/plus/reset controls for real grid zoom 50–200%; no Sum/Average/Count |

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
`00123` remains text. F2/formula-bar edits retain the stored kind; typing a
complete replacement in selection mode uses safe Auto inference again. The Home Number selector is explicitly labeled **Cell type** and
can intentionally change a stored source kind; currency/percent/display formatting
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
future work. File opens a native document page with a 174-point teal navigation rail,
New/Open/Save/Save As/Close, actual workbook state and local path, and a return
control. Escape returns without discarding the pending cell entry; cell typing
and arrow navigation are suspended on this page. New/Open/Close retain the same
Save/Discard/Cancel protection. This is a limited local document page: recent
files, accounts, templates, printing and export are not simulated. Supported
operations keep familiar gestures; unavailable ones do not pretend to execute.

## Captures and DPI verification

The captures below record the `0.1.0-alpha.1` design iteration; the File page
therefore displays that historical build number. They are retained as visual
evidence, not relabeled as a newer binary. Refreshed native Rust/OpenGL framebuffer captures were made on this macOS host with
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
| macOS File page | [Native frame](screenshots/macos-file.png), blank Book1 | Not available | Real local document page; no personal file paths in this capture |
| macOS range selection | [Native frame](screenshots/macos-range.png), A1:C4 | Not available | Real selection highlighting; active endpoint C4 is blank |
| Windows display 100% | Not captured | Not captured | **Not verified: Windows/Excel unavailable** |
| Windows display 125% | Not captured | Not captured | **Not verified: Windows/Excel unavailable** |
| Windows display 150% | Not captured | Not captured | **Not verified: Windows/Excel unavailable** |

Headless egui tests additionally simulate three scale values and verify zone
ordering/visibility and bounded visible-cell painting. These are behavioral and
geometry tests (including narrow 910/980/1093-point viewports and caption clipping), not visual screenshot equivalence or native Windows DPI tests.
Real snapshots are compared only within the same available macOS conditions;
no pixel-diff score against an absent Excel image is fabricated.

To reproduce own-app captures, build `cargo build --locked --features capture`,
set `BREAKFREE_CAPTURE_TO` to an absolute PNG path, optionally set
`BREAKFREE_CAPTURE_SCALE` to 1.0/1.25/1.5 and `BREAKFREE_CAPTURE_CELL=C1`, then
launch the binary with the absolute native example path. For the File-page
capture set `BREAKFREE_CAPTURE_VIEW=file` and launch with `--new`. Keep a copy
of the capture-enabled executable before running another Cargo feature build.
Use `BREAKFREE_CAPTURE_CELL=A1:C4` for the range example. The capture build exits
after rendering. These developer-only variables are absent from the ordinary
product interaction flow.

For the required Windows comparison, record exact Excel 2024 edition/build,
Windows build, physical client resolution, display scale (100%,125%,150%), theme,
font, workbook data, active cell, worksheet zoom and ribbon state. Capture both
applications under identical settings; measure each zone and compare aligned
images at native pixels without resampling. Record differences and rerun the
editing/resize/scroll workflow at each DPI before claiming a match. This remains
an explicit acceptance gap, not a completed task hidden by simulated scaling.

## Refinement review — 2026-10-16

The previous own-app frame and refreshed frame were compared under the same
1366 × 768 logical client, Retina factor 2, light theme, C1 selection and example
workbook. This is a before/after review of BreakFree Sheets, not a comparison
against an absent Excel installation. The new layout replaces boxed text commands
with original geometric icons and flat hit targets, gives all Home captions a
common baseline, separates the source type from display formatting, frames the
name/formula fields, and tightens sheet/status controls. All seven Home groups
fit at the default width. Narrower logical viewports scroll the ribbon; its
scrollbar has reserved height so captions remain visible.

The command drawings are authored in `src/gui/icons.rs`; no external ribbon
artwork or icon font was added. No dependency, calculation, native-file or Office
contract change is needed for this design refinement. Disabled commands have
muted rendering and explanatory hover text. Clear, typed single-cell clipboard,
undo/redo, Apply/Cancel and zoom are verified through actual egui events. File
navigation and unsaved-draft preservation are also covered.

Remaining visible differences include bundled fallback typography, original
command drawings, a restricted Number/type selector, unavailable formatting and
advanced tabs, a single bounded worksheet, separate native OS title chrome and
a narrower File feature set. Matched Windows measurements remain the next visual
acceptance step; no similarity percentage or visual identity is claimed.
