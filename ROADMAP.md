# BreakFree Sheets — Roadmap

**Status: planning.** Proposed work, not shipped functionality. No release dates are committed.

## 0 · Choose the foundation

- [ ] Agree the software license with the owner.
- [ ] Select a first platform, UI toolkit and implementation language.
- [ ] Define worksheet storage, cell types, formula grammar and numeric semantics.
- [ ] Create a buildable shell, reproducible build instructions and test runner.

**Exit:** architecture decisions and a minimal application shell are reviewable.

## 1 · A grid that calculates reliably

- [ ] Cell selection, editing and basic keyboard navigation.
- [ ] Typed text and numeric values.
- [ ] A small documented formula subset with arithmetic and cell references.
- [ ] Recalculation, visible errors and circular-reference detection.
- [ ] Undo/redo, local save/reopen and unsaved-change handling.

**Exit:** all [first-milestone scenarios](docs/REQUIREMENTS.md) pass. A changed input produces the specified dependent results; the workbook round-trips without losing supported data or formulas.

## 2 · Everyday spreadsheet work

- [ ] Number display formats, without changing underlying values.
- [ ] Copy/paste with defined reference behavior.
- [ ] Sort and filter with explicit row/range semantics.
- [ ] Multiple worksheets and a tested set of common functions.
- [ ] Keyboard and accessibility review.

**Exit:** each new feature has model-level tests and documented interaction with formulas, undo and persistence.

## 3 · Data exchange and visualization

- [ ] CSV import/export with explicit encoding, delimiter and type choices.
- [ ] A scoped XLSX importer/exporter and synthetic fixtures.
- [ ] A compatibility matrix for workbook elements and formulas.
- [ ] Basic charts after the data and calculation models are stable.
- [ ] Warnings or safe refusal for unsupported and potentially lossy operations.

**Exit:** compatibility claims identify tested cases and limitations. No unknown formula is presented as a verified result.

## 4 · First public preview

- [ ] Verified packaging for the declared platform.
- [ ] Regression tests for numeric edge cases and failed saves.
- [ ] Dependency/asset license review and a private security-reporting route.
- [ ] Real release notes, installation instructions and known limitations.

Macros, external data connections, cloud collaboration and complete Excel parity are outside the initial scope.

[Back to Sheets](README.md) · [Suite roadmap](https://github.com/PerchAnton/breakfree-office/blob/main/ROADMAP.md)
