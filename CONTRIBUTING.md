# Contributing to BreakFree Sheets

Read the [shared contribution guide](https://github.com/PerchAnton/breakfree-office/blob/main/CONTRIBUTING.md) and [Code of Conduct](https://github.com/PerchAnton/breakfree-office/blob/main/CODE_OF_CONDUCT.md).

Use this repository for grids, cell models, formulas, recalculation, workbook persistence and data-exchange investigations. Suite-wide identity and policy belong in [BreakFree Office](https://github.com/PerchAnton/breakfree-office).

Start with [ROADMAP.md](ROADMAP.md) and [the first milestone](docs/REQUIREMENTS.md). Discuss calculation semantics, major dependencies and licensing before implementation. The software license is an owner decision still to be made.

Provide small synthetic examples with explicit expected results. Never attach real customer records, private financial workbooks or credentials. Explain checks actually run and any untested behavior. Run `python3 -m unittest discover -s tests -v` (Windows: `py -3 -m unittest discover -s tests -v`) and the opt-in GUI checks documented in [README.md](README.md).

The owner authorized a reversible prototype stack in the implementation task;
[ADRs](docs/adr/0001-independent-prototype.md) record the decision for review.
No permanent stack or software license has been approved by that authorization.
Keep changes on a branch and submit a PR; do not merge or publish a release
without a separate instruction. Keep project evidence in
[docs/PROJECT_LOG.md](docs/PROJECT_LOG.md), with journal updates in their own
commit. Distinguish local tests, remote CI, review, merge and release.
