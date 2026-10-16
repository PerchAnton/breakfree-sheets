# Changelog

Every completed project changeset receives a new application version and a dated
entry here. Dates use Europe/Kyiv. Candidate entries are development records, not
published releases. See [the version policy](docs/VERSIONING.md).

## 0.1.0-alpha.2 — 2026-10-16

Status: development candidate; exact-number agreement pending; not released.

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
