# First milestone — a reliable calculation loop

**First iteration implemented for review.** One worksheet with a deliberately
small formula language. Local checks and remaining platform/review gates are
listed in [VALIDATION.md](VALIDATION.md); this is not a release acceptance claim.

## User story

A person enters a short list of values, adds formulas, changes an input, sees the dependent results update, saves the workbook and reopens it with all supported data and formulas intact.

## In scope

Cell navigation, text/numeric values, arithmetic, single-cell references, visible calculation errors, undo/redo and versioned local persistence. The prototype decisions are recorded in [ADR 0001](adr/0001-independent-prototype.md) and [ADR 0002](adr/0002-calculation-and-storage.md).

## Acceptance scenarios

| ID | Scenario | Observable result |
| :--- | :--- | :--- |
| S01 | Select and edit cells using mouse and keyboard | The intended cell is edited; focus and selection are visible. |
| S02 | Store text and numeric values | The model distinguishes types and preserves supported values on save/reopen. |
| S03 | Set A1 to 2, B1 to 3 and C1 to `=A1+B1` | C1 evaluates to 5 under the documented formula grammar. |
| S04 | Change A1 to 4 | C1 recalculates to 7 without stale results. |
| S05 | Enter malformed syntax, division by zero or an unsupported function | A visible, documented error is shown, not a guessed value. |
| S06 | Create a circular dependency | The cycle is detected and reported without a hang. |
| S07 | Undo/redo input and formula edits | The cell model and dependent results return to the appropriate states. |
| S08 | Save, reopen and edit a dependency | Values, formulas and their relationships survive the round trip. |
| S09 | Close with unsaved edits or simulate a failed save | The user gets explicit choices/errors; a valid prior workbook is not silently destroyed. |
| S10 | Open malformed input | The operation fails safely without modifying the source file. |

## Required decisions and tests

Specify operator precedence, numeric precision/rounding, blank-cell behavior, reference rules, error propagation and recalculation order. Add boundary tests once those semantics are selected. Decimal and date handling must not be left to unreviewed locale-dependent guesses.

Imported formulas must be parsed as a restricted language, not passed to a general-purpose `eval`. Macro execution, external links and network-backed functions are outside this milestone.

## Implementation mapping

| Scenarios | Evidence | Status |
| :--- | :--- | :--- |
| S01 | Tk mouse selection, source entry, keyboard events in `tests/test_gui.py` | Local macOS GUI checked; accessibility review pending |
| S02–S06 | `tests/test_core.py`: explicit types, arithmetic, dependencies, errors and cycles | Local automated checks passed |
| S07 | Headless history and GUI undo/redo scenarios | Local automated checks passed |
| S08 | Headless and real-Tk save/open/recalculate scenarios | Local automated checks passed |
| S09 | Pending-entry prompts, Cancel/Discard/Save, failed flush/replace and dirty-state tests | Local automated checks passed; power-loss recovery outside scope |
| S10 | Foreign, malformed, unknown version/field, duplicate keys and source preservation | Local automated checks passed |
| Office CLI | `tests/test_cli.py`, exact JSON fixture and isolated artifact execution | Local baseline verified; upstream spec reconciliation not performed |

Source files contain no Office/Docs/Slides imports or repository-relative runtime
references. The build includes only this application's source and local assets.

## Not included yet

XLSX compatibility, charts, pivot tables, multiple worksheets, macros, CSV type inference, cloud accounts and collaboration. CSV and richer workbook exchange need later, separately testable milestones.

## Completion evidence

Reproducible build/test commands; automated parser, calculation, cycle, undo and persistence tests; synthetic fixtures; manual keyboard checks; documented limits. A demo that calculates one formula is not evidence of Excel compatibility.

[Back to roadmap](../ROADMAP.md)
