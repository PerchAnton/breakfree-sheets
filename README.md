<p align="center">
  <a href="https://github.com/PerchAnton/breakfree-office"><img src="breakfree_sheets/assets/breakfree-icon.png" alt="Approved shared BreakFree mark" width="160" height="160"></a>
</p>

<h1 align="center">BreakFree Sheets</h1>

<p align="center">
  <strong>Your numbers. Your freedom.</strong><br>
  An independent local spreadsheet prototype.<br>
  Organize data. Explore numbers. Make things add up.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/status-prototype-248A83?style=flat-square" alt="Status: prototype">
  <img src="https://img.shields.io/badge/focus-spreadsheets-248A83?style=flat-square" alt="Focus: spreadsheets">
  <img src="https://img.shields.io/badge/suite-BreakFree%20Office-241337?style=flat-square" alt="Part of BreakFree Office">
</p>

<p align="center">
  <a href="#make-room-for-your-numbers">Vision</a> ·
  <a href="ROADMAP.md">Roadmap</a> ·
  <a href="docs/REQUIREMENTS.md">First milestone</a> ·
  <a href="CONTRIBUTING.md">Contribute</a> ·
  <a href="https://github.com/PerchAnton/breakfree-office">Office hub</a>
</p>

---

> [!IMPORTANT]
> **Working prototype for review, not a public release.** One worksheet, typed cells,
> restricted arithmetic formulas, dependency recalculation, undo/redo and local
> `.bfsheet` save/open are implemented. No installers or XLSX compatibility are claimed.

## Run independently

Requires **Python 3.10+** with **Tk 8.6+** for the GUI. No Office, Docs, Slides,
third-party Python packages, account, network or adjacent checkout is required.
Only macOS Python 3.14.3 / Tk 8.6 has been tested locally so far. Windows/Linux
instructions and CI definitions are provided; they are not a tested support promise.

From this repository on macOS/Linux:

```sh
python3 -m breakfree_sheets --new
./breakfree-sheets --capabilities
./breakfree-sheets --version
python3 -m breakfree_sheets --open "/absolute/path/My workbook.bfsheet"
```

On Windows PowerShell, install Python with Tcl/Tk and the Python launcher enabled,
then run from this repository:

```powershell
py -3 -m breakfree_sheets --new
.\breakfree-sheets.cmd --capabilities
.\breakfree-sheets.cmd --version
py -3 -m breakfree_sheets --open "C:\Users\You\Documents\My workbook.bfsheet"
```

Check Tk with `python3 -m tkinter` (Windows: `py -3 -m tkinter`). Linux distributions
may package Tk separately as `python3-tk`; use the package matching your interpreter.
Headless commands and core tests work without Tk. A plain launch defaults to `--new`.
`--open` accepts an absolute path as one argument, validates before opening a window,
and never rewrites an input just because it was opened.

## Try the calculation loop

Select A1, choose **number**, enter `2` and press **Apply** or Enter. Set B1 to
number `3`. Select C1, choose **formula**, enter `=A1+B1`: C1 displays `5`.
Change A1 to `4`: C1 displays `7`. Save as `.bfsheet`, open it again, and edit A1:
the formula remains a formula and recalculates. A synthetic starting book is
included at [examples/first-workbook.bfsheet](examples/first-workbook.bfsheet).

Blank cells default to **text**, so `00123` stays text. There is no implicit type
conversion. Choose the type explicitly; formulas start with `=`. Clicking another
cell applies the pending entry. Escape cancels it. Arrow keys and Tab/Enter navigate
when the grid has focus; F2 or a double click focuses the source entry. Use the
scrollbars for the rest of A1–Z100. New, Open and Close prompt for unsaved edits.
Toolbar Undo/Redo (Ctrl+Z/Y, Command+Z/Y on macOS) operate on cell edits; an unapplied
entry is cancelled first by Undo. Clear cell removes the cell, rather than storing
empty text. The most recent 500 edits are retained per session.

The numeric model is decimal floating-point, 28 significant digits and half-even
rounding for every operation. This is **not a financial-accuracy or compatibility
guarantee**. See [numeric, blank and error semantics](docs/adr/0002-calculation-and-storage.md).
Syntax, division-by-zero, cycle, reference, text-operand and numeric-limit errors
are displayed explicitly. Unsupported functions, macros, external references and
network formulas are never executed.

## Test and build

No dependency installation is necessary:

```sh
python3 -m unittest discover -s tests -v
BREAKFREE_GUI_TESTS=1 python3 -m unittest discover -s tests -v
python3 tools/build.py
python3 dist/breakfree-sheets --capabilities
python3 dist/breakfree-sheets --new
```

The GUI tests need a real desktop. On Windows PowerShell:

```powershell
py -3 -m unittest discover -s tests -v
$env:BREAKFREE_GUI_TESTS = "1"
py -3 -m unittest discover -s tests -v
Remove-Item Env:BREAKFREE_GUI_TESTS
py -3 tools/build.py
.\dist\breakfree-sheets.cmd --new
```

The deterministic zip application in `dist/` embeds the application and approved
local assets; copy `breakfree-sheets` and, for Windows, `breakfree-sheets.cmd` together.
Python/Tk remain runtime prerequisites. This is not a native installer or a release.
Builds never fetch Office or a mutable remote specification. See
[validation and remaining checks](docs/VALIDATION.md), [architecture](docs/adr/0001-independent-prototype.md),
[the temporary Office contract](contracts/v1/SNAPSHOT.md) and [notices](NOTICES.md).

## Make room for your numbers

BreakFree Sheets aims to bring structure to lists, budgets, inventories and everyday analysis. Our starting point is a dependable grid with understandable calculations, not an immediate replacement for every Excel workflow.

Clear errors and reproducible results matter more than a large function count.

## The first useful workflow

> **Enter data → Add a formula → Change an input → Recalculate → Save → Reopen.**

The prototype implements this workflow with a single bounded worksheet. Its storage
model and numeric semantics are documented in ADRs; platform and interoperability
claims remain limited to the checks actually run.

| Implemented for review | Investigate next | Outside the initial scope |
| :--- | :--- | :--- |
| Select and edit cells | Formatting, sort and filter | Full Excel feature parity |
| Text, numbers and simple formulas | Multiple worksheets and charts | VBA and macros |
| Recalculation and visible errors | CSV import/export with explicit options | Cloud collaboration |
| Undo, redo, save and reopen | A documented XLSX subset | External data connections |

See the [acceptance criteria](docs/REQUIREMENTS.md) and [roadmap](ROADMAP.md) for the proposed sequence.

## Design direction

**Trust the result.** Define calculation behavior, test edge cases and show errors instead of quietly returning a misleading value.

**Respect the source data.** Make import choices explicit. A code with leading zeros, a date and a decimal value should not be silently treated as interchangeable.

**Keep it in your control.** Prioritize local workbooks, offline essentials and clear file ownership. Network access and executable document content are outside the initial design.

These principles guide the prototype and its next milestones; they do not imply a finished product.

## File compatibility

XLSX support is a development goal. Workbook structure, formulas, formatting and charts need separate test coverage and a published compatibility matrix. Unknown formulas must not be presented as verified calculations; unsupported workbook elements need an explicit handling policy.

CSV is a separate import/export investigation with documented delimiter, quoting, encoding and type-inference choices. It is not a substitute for a full workbook format.

## Development status

| Area | Current position |
| :--- | :--- |
| Product direction | Documented |
| Implementation language and UI toolkit | Python/Tk; reversible prototype ADR, owner review pending |
| Formula engine and native workbook format | Restricted arithmetic engine; versioned `.bfsheet` JSON |
| Windows, macOS and Linux | macOS GUI checked locally; all three headless CI targets and Linux/Xvfb GUI passed; interactive Windows/Linux review pending |
| Application code and automated tests | Implemented; see [validation](docs/VALIDATION.md) |
| Software license | Owner decision pending |

Sheets builds, tests and runs independently. Office integration uses a locally
recorded temporary baseline; upstream `contracts/v1/` reconciliation is pending.
There is no automatic release, telemetry, document upload or automatic update check.

## The BreakFree family

| Project | Purpose |
| :--- | :--- |
| [**BreakFree Office**](https://github.com/PerchAnton/breakfree-office) | Suite hub, identity and shared direction |
| [**BreakFree Docs**](https://github.com/PerchAnton/breakfree-docs) | Documents and writing |
| **BreakFree Sheets** | Spreadsheets — you are here |
| [**BreakFree Slides**](https://github.com/PerchAnton/breakfree-slides) | Presentations and visual storytelling |

## Get involved

Suggest a spreadsheet workflow, an anonymized calculation example or a focused technical investigation in [Issues](https://github.com/PerchAnton/breakfree-sheets/issues). Never upload confidential financial records or customer data. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md) before preparing changes.

Everyone is welcome, including LGBTQ+ contributors and users. Follow the [shared Code of Conduct](https://github.com/PerchAnton/breakfree-office/blob/main/CODE_OF_CONDUCT.md).

## License and independence

No software license has been selected; public visibility alone is not a reuse license. Licensing will be resolved before substantive third-party implementation contributions are accepted.

BreakFree Sheets is independent and is not affiliated with Microsoft. Microsoft Excel and XLSX references describe intended workflows and interoperability goals, not endorsement or a completed replacement.

---

<p align="center"><strong>BreakFree Office · Your work. Your freedom.</strong><br><sub>Created by <a href="https://github.com/PerchAnton">Anton Perchenko</a></sub></p>
