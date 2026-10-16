# ADR 0003 — Rust application and Windows desktop UI target

- Date: 2026-10-16
- Status: **accepted owner decisions** for language, visual reference and brand
- Supersedes: ADR 0001's provisional Python/Tk implementation choice
- Does not select a project license or authorize a merge/release

## Owner decisions

The whole application is Rust: workbook/cell model, parser, dependency graph,
calculation, history, files, Office CLI and GUI. The visual/interaction reference
is **desktop Microsoft Excel 2024 for Windows**. The application identity stays
**BreakFree Sheets**, with its approved transparent Sheets mark and shared
BreakFree colors. Rust is no longer an unselected or provisional language.

This is a language migration required by the owner, not an engine rewrite for
cosmetic similarity. The Python code remains a historical conformance archive in
`legacy/python/`; Cargo and the executable never import, embed or execute it.
The active source is `src/`. A frozen set of 35 synthetic Python-v1 scenarios is
checked by native Rust tests without Python. Native v1 files and the restricted
formula grammar are preserved.

## Native rendering choice

Use egui/eframe 0.32.3 with its native winit/OpenGL backend for this iteration,
not its web backend. There is no Electron, WebView, HTML/JavaScript grid or
JavaScript frontend. This concrete toolkit choice implements the owner's native
Rust requirement; later toolkit evaluation is separate from the decided language.
Cargo.lock pins the resolved dependency graph; Rust 1.99.1 is pinned separately.

The grid is one painted desktop component with one interaction surface and at
most one active inline text editor. It computes visible row/column ranges from
scroll position, dimensions and zoom, clips to the viewport, and paints only
those cells plus visible headers. No persistent/heavy widget is allocated per
possible cell. Native scrollbars move the viewport; headers stay at its edges.

`interaction::Session` owns selection, mode and one draft. The formula bar and
inline grid editor bind to that same draft; committing creates a typed model
edit through `history::Editor`. `gui` and `gui::grid` only render/interact with
engine outputs. No formula evaluation lives in ribbon or paint callbacks.

## Calculation and files

ADR 0002's 28-digit decimal/half-even semantics remain. Rust stores decimal
coefficient/exponent values and uses exact big-integer rational intermediates,
then explicitly rounds once per arithmetic operation. It does not use binary
float for workbook calculations. The same exponent/blank/text/error rules and
iterative cycle handling apply. This is not a financial-correctness guarantee.

Serde validates the strict native v1 envelope and rejects unknown/duplicate
fields. The file layer writes/flushed/synced same-directory temporary files and
atomically persists the replacement. Injected partial-write, flush and replace
failures preserve prior valid bytes. The local v1 Office contract remains
independent of Office installation and is not fetched while building.

## Interaction scope and differences

Light compact ribbon, File menu, requested tabs/Home groups, name/address field,
formula bar, headers, active/range selection, single-sheet tab area, scrollbars
and status/zoom are implemented. Arrow/Tab/Enter/F2/Escape, shift/drag ranges,
source edits, column/row resizing and cell history are supported.

Safe Auto entry is deliberate: a new `=` source is a formula, canonical decimal
input is numeric, leading-zero identifiers remain text. Existing stored cell
kinds remain explicit unless the user changes the type selector. Number display
does not alter stored source. This is narrower than Excel's type inference.

The first model remains one sheet A1–Z100. Clear supports a rectangular range in
one undo step. Clipboard buttons support one typed cell only, without relative
reference translation. Font/formatting, AutoSum, fill, sorting/filtering, insert/
delete rows, multiple sheets and charts are visibly unavailable. Status has no
invented Sum/Average/Count. Column/row dimensions and zoom are session view state,
not silently added to native v1 files. See [UI_SPEC](../UI_SPEC.md) for evidence,
geometry, limitations and required real-Windows DPI comparisons.

## Validation and review gates

Native Rust tests exercise parser/calculation/persistence/contract and actual
egui pointer/keyboard events through the rendered grid and formula bar. macOS
framebuffer captures are real application output, while diagnostic scaling is
explicitly not Windows DPI evidence. The selected Excel 2024 build, matched
reference screenshots and Windows 100/125/150% measurements are still unavailable.
No visual identity, Excel compatibility or supported-platform claim follows.

Dependency notices, including embedded font notices, are retained separately.
Licensing, accessibility, installer/signing policy and public releases still
require their own owner review. Other application repositories are unchanged;
only the required private journal mirror is updated outside Sheets.
