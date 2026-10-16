# Contributing to BreakFree Sheets

Read the [shared contribution guide](https://github.com/PerchAnton/breakfree-office/blob/main/CONTRIBUTING.md) and [Code of Conduct](https://github.com/PerchAnton/breakfree-office/blob/main/CODE_OF_CONDUCT.md).

Use this repository for grids, cell models, formulas, recalculation, workbook persistence and data-exchange investigations. Suite-wide identity and policy belong in [BreakFree Office](https://github.com/PerchAnton/breakfree-office).

Start with [ROADMAP.md](ROADMAP.md) and [the first milestone](docs/REQUIREMENTS.md). Discuss calculation semantics, major dependencies and licensing before implementation. The software license is an owner decision still to be made.

Provide small synthetic examples with explicit expected results. Never attach real customer records, private financial workbooks or credentials. Explain checks actually run and any untested behavior. Run `cargo test --locked --no-default-features`, `cargo test --locked`,
`cargo fmt --all -- --check` and the checks documented in [README.md](README.md).

The owner has selected Rust for every layer and Excel 2024 for Windows as the
visual reference; [ADR 0003](docs/adr/0003-rust-native-desktop.md) supersedes the
Python prototype decision. The native toolkit remains reviewable. No software
license is selected by the language/UI decision.
Keep changes on a branch and submit a PR; do not merge or publish a release
without a separate instruction. Keep project evidence in
[docs/PROJECT_LOG.md](docs/PROJECT_LOG.md), with journal updates in their own
commit. Distinguish local tests, remote CI, review, merge and release.
