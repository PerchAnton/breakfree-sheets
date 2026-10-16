# Working in BreakFree Sheets

Read README.md, ROADMAP.md, docs/REQUIREMENTS.md and CONTRIBUTING.md first. The repository is currently documentation-only; language, toolkit, license and file format are not selected.

## Scope

Prioritize the grid, deterministic calculation behavior and reliable local persistence. Keep suite-wide branding in `PerchAnton/breakfree-office`.

## Rules

- Preserve the approved name and shared office-document logo.
- Keep public documentation in English; distinguish plans from verified features.
- Obtain owner review for license, architecture and major dependency choices.
- Define numeric semantics, type handling, reference behavior and error propagation explicitly.
- Parse formulas through a restricted grammar; never evaluate imported strings as general-purpose code.
- Do not execute macros, external links or network-backed formulas automatically.
- Test calculation dependencies, cycles, undo/redo and save/reopen behavior.
- Never silently reinterpret identifiers, dates or decimal values without a documented user-facing import policy.
- Do not invent XLSX compatibility, passing tests or supported platforms.
- Use synthetic fixtures; do not commit real financial records or unlicensed assets.

## Validation

At this stage, check Markdown links, image references and YAML syntax. Add actual build/test commands with the first implementation and report only checks that were run. Update development status as verified code is added.
