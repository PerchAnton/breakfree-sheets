# Changelog

Every completed project changeset receives a new application version and a dated
entry here. Dates use Europe/Kyiv. Candidate entries are development records, not
published releases. See [the version policy](docs/VERSIONING.md).

## 0.1.0-alpha.3 — 2026-10-16

Status: development candidate; exact-number agreement pending; not released.

### Added

- Native View ribbon with Workbook Views, Show, Zoom, Window and Macros groups.
  Gridlines, Headings, Formula Bar, Zoom and 100% controls operate on the real
  worksheet view; unavailable operations remain disabled.
- Ribbon collapse/restore through Ctrl+F1, double click on a tab or the tab-row
  chevron. Clicking a collapsed tab opens commands persistently.
- Select-all corner for the bounded A1–Z100 sheet, with normal range Clear/Undo.
- Original page-view/zoom/pane drawings; no additional fonts or dependencies.

### Changed

- Preserve the shared draft in the grid editor when hiding its formula bar.
  Hidden headings reclaim their space and use the same coordinate hit testing.
  Visibility choices do not change stored data, history or dirty state.
- Record the owner's exact alpha.2 agreement; advance build/CLI/Office metadata
  to the independently reviewable alpha.3 candidate. Protocol/file versions stay 1.

### Verification and limits

39 default tests (20 core + 19 UI), 20 headless tests, fmt, all-target/all-feature
Clippy, offline optimized build, isolated CLI/Office JSON, documentation links,
YAML and whitespace checks passed. Six actual macOS frames were inspected.
Local checks and actual native capture conditions are recorded in
[VALIDATION.md](docs/VALIDATION.md) and [UI_SPEC.md](docs/UI_SPEC.md).
New UI regressions exercise view changes during formula entry, recalculation,
undo, save/reopen, visible grid strokes, whole-sheet Clear/Undo and ribbon
collapse. Windows/Excel matched captures and real Windows DPI checks remain
unavailable. No formatting, page layout, window management or macro support is
implied by disabled controls. No merge, tag or release is included.

## 0.1.0-alpha.2 — 2026-10-16

Status: development candidate; exact number agreed by the owner on 2026-10-16; not released.

### Changed

- Require a prerelease counter increment and changelog entry for every completed
  changeset, including documentation, tests and configuration. Related validation
  and journal commits share that changeset's version.
- Advance Cargo metadata, root lockfile entry, CLI/Office application version and
  current documentation from alpha.1 to alpha.2. Protocol and file versions stay 1.
- Carry forward the native design refinement from [5a7220e](https://github.com/PerchAnton/breakfree-sheets/commit/5a7220e098f703cee820713dc6efef8cfc723af6):
  compact Home icon groups, a working local File page, framed name/formula fields,
  selected headers, sheet tabs and status zoom controls.

### Fixed

- Account for the preceding interface changes in the version/changelog workflow.
  That source commit used alpha.1 under the prior policy; history is preserved.
- Retain the narrow-ribbon caption fix and four UI regression scenarios from the
  design refinement. Unsupported commands remain visibly unavailable.

### Verification and limits

The same Rust engine, native v1 format, dependencies and GUI behavior are retained.
For this candidate, 35 default tests, 20 headless tests, formatting checks,
an offline optimized build and isolated CLI/Office JSON checks passed locally.
Commit-specific server results are recorded in [the project journal](docs/PROJECT_LOG.md).
Existing design captures retain their actual alpha.1 label. Matched Excel 2024
Windows screenshots, real Windows display DPI and interactive accessibility
checks remain unavailable. No merge, tag or product release is included.

## 0.1.0-alpha.1 — 2026-10-16

Status: historical development baseline; no product release or exact-number
agreement recorded. This entry is reconstructed from existing project records.

### Added

- Independent native Rust application with a bounded virtualized worksheet,
  typed sources, restricted arithmetic/A1 formulas, dependency recalculation,
  explicit errors, undo/redo and protected versioned local files.
- Office v1 CLI commands and a frozen local integration baseline.
- Native grid/formula editing, range selection/Clear, resizing, one-cell typed
  clipboard, unsaved-change protection and diagnostic captures.
- Initial prerelease policy and actual alpha.1 build metadata in [a0bf9a8](https://github.com/PerchAnton/breakfree-sheets/commit/a0bf9a80bca37d6d26377d8f76aecd14f9c69826).

The subsequent design commit also carried the alpha.1 label; the alpha.2 entry
above records this transition explicitly rather than claiming an earlier release.
Historical provisional labels and validation remain in the project journal.
