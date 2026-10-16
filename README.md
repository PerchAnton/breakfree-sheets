<p align="center">
  <a href="https://github.com/PerchAnton/breakfree-office"><img src="https://raw.githubusercontent.com/PerchAnton/breakfree-office/main/docs/brand/breakfree-icon.webp" alt="BreakFree Office" width="160" height="160"></a>
</p>

<h1 align="center">BreakFree Sheets</h1>

<p align="center">
  <strong>Your numbers. Your freedom.</strong><br>
  An independent spreadsheet application in the making.<br>
  Organize data. Explore numbers. Make things add up.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/status-planning-248A83?style=flat-square" alt="Status: planning">
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
> **Planning stage — not a working spreadsheet yet.** This repository contains project documentation and the development plan. There are no installers, calculation engine or verified XLSX compatibility claims. Capabilities below are proposed, not available features.

## Make room for your numbers

BreakFree Sheets aims to bring structure to lists, budgets, inventories and everyday analysis. Our starting point is a dependable grid with understandable calculations, not an immediate replacement for every Excel workflow.

Clear errors and reproducible results matter more than a large function count.

## The first useful workflow

> **Enter data → Add a formula → Change an input → Recalculate → Save → Reopen.**

The first milestone will combine a basic worksheet, typed values, a deliberately small formula language, undo/redo and safe local persistence. The storage model and numeric semantics must be chosen and documented before compatibility promises are made.

| Proposed first | Investigate next | Outside the initial scope |
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

These are goals for implementation, not statements about an existing application.

## File compatibility

XLSX support is a development goal. Workbook structure, formulas, formatting and charts need separate test coverage and a published compatibility matrix. Unknown formulas must not be presented as verified calculations; unsupported workbook elements need an explicit handling policy.

CSV is a separate import/export investigation with documented delimiter, quoting, encoding and type-inference choices. It is not a substitute for a full workbook format.

## Development status

| Area | Current position |
| :--- | :--- |
| Product direction | Documented |
| Implementation language and UI toolkit | Not selected |
| Formula engine and native workbook format | Not selected |
| Windows, macOS and Linux | Candidate targets; no supported builds yet |
| Application code and automated tests | Not present yet |
| Software license | Owner decision pending |

There is nothing to install yet. Verified build instructions will be added with the first implementation. The suite begins with Docs; Sheets has its own roadmap and can progress independently when its foundation is ready.

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
