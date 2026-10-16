# BreakFree Sheets — Roadmap

**Status: first independent prototype implemented for review.** Later stages
remain planned. No release dates are committed. Code readiness, local test
results, CI, owner review, merge and release are separate states.

| Stage | Dependencies | Completion criteria | Required checks | Status |
| :--- | :--- | :--- | :--- | :--- |
| 0. Foundation | Owner's independent-app scope; available Office decisions | Reviewable stack, cell/numeric/file ADRs; local build/test commands; explicit license gate | Standalone build, offline headless tests, pinned local contract/brand provenance | Rust and Windows Excel 2024 reference decided by owner; native GUI/toolkit review and license decision remain open |
| 0a. Native desktop layout | Owner Rust/Excel 2024 decisions; working engine | Real ribbon/Home/address/formula/grid/sheet/status layout, viewport virtualization, supported keyboard/mouse gestures | Native Rust event tests; real matched Windows 100/125/150% captures; transparent own logo | Working macOS native frame; actual Windows/Excel visual comparison unverified |
| 1. Model and calculations | Stage 0 prototype decisions | One sheet; explicit types; restricted arithmetic and references; correct recalc/errors; undo/redo; safe save/open; unsaved prompts; Office CLI | S01–S10 in [requirements](docs/REQUIREMENTS.md); long chains; direct/indirect cycles; decimal boundaries; failed writes; JSON CLI; real desktop tests | Rust port implemented with v1 conformance tests; current Rust CI and platform review recorded separately from earlier Python CI |
| 2. Range editing and copy references | Stage 1 model/history safety | Range selection/editing; clipboard semantics; relative/absolute reference adjustment; atomic multi-cell undo | Copy across boundaries, overlapping ranges, mixed references, partial invalid input, undo and save/open | Partial: range selection/Clear and view resizing implemented; clipboard ranges and reference adjustment planned |
| 3. Multiple worksheets | Stages 1–2; versioned model migration | Sheet identities, add/rename/delete/reorder; defined cross-sheet references and deletion behavior | Cross-sheet dependencies/cycles, rename/delete undo, old-format migration, round trips | Planned |
| 4. Formatting and functions | Stable typed sources, ranges and sheet model | Display formats independent of values; small documented function registry; safe unsupported-function behavior | Precision/display separation, function arity/type/error propagation, Unicode and locale fixtures | Planned |
| 5. Sort and filter | Range semantics, sheets, history and formatting | Explicit row/range rules; formula/reference policy; non-destructive filtering; batch undo | Stable sorts, mixed types, hidden rows, formulas after moves, persistence and undo | Planned |
| 6. CSV/XLSX exchange | Stable typed model and functions; reviewed compatibility strategy | Explicit CSV encoding/delimiters/type choices; scoped XLSX subset; public compatibility matrix; safe refusal of lossy unsupported content | Redistributable synthetic round trips, leading-zero identifiers, unknown formulas/elements, ZIP/resource limits, malformed imports | Planned; no compatibility claim today |
| 7. Charts and analytics | Stable data ranges, formatting, functions and exchange | Basic charts tied to ranges; defined missing/error data behavior; scoped analytic features | Data-change refresh, chart persistence, errors/blanks, export and accessible alternatives | Planned |
| 8. Automation | Stable engine/API; separate owner-reviewed security design | Explicit bounded automation interface, consent and isolation; no automatic imported macro execution | Resource limits, permissions, deterministic runs, hostile fixtures, undo/persistence consequences | Planned; macros/network execution remain excluded from the prototype |
| 9. Stabilization and releases | Accepted functional stages; license and private reporting policy; real platform evidence | Reviewed platform matrix; native packaging; recovery strategy; accessibility; install/upgrade/uninstall; release notes and checksums | Windows/macOS/Linux workflows on declared targets, interruption/failure matrix, dependency/asset notices, regression suite and release review | Planned; no automatic merge or release |

The application remains independently buildable, testable and runnable at every
stage. Suite integration must use versioned local contracts, and changes to
Office, Docs or Slides are separate tasks. Advanced parity, cloud collaboration
and enterprise features are not implied by this sequence.

[Back to Sheets](README.md) · [Suite roadmap](https://github.com/PerchAnton/breakfree-office/blob/main/ROADMAP.md)
