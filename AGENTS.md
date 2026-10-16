# Working in BreakFree Sheets

Read README.md, ROADMAP.md, docs/REQUIREMENTS.md, CONTRIBUTING.md, existing ADRs and docs/PROJECT_LOG.md first. The owner requires Rust for the complete independent native application and Microsoft Excel 2024 for Windows as the visual reference. The product identity is BreakFree Sheets. The software license remains unselected.

## Scope

Prioritize the grid, deterministic calculation behavior and reliable local persistence. Keep suite-wide branding in `PerchAnton/breakfree-office`.

## Rules

- Preserve the approved BreakFree Sheets name and dedicated Sheets logo with transparent exterior; follow the shared Office palette.
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

Run `cargo test --locked --no-default-features`, `cargo test --locked`,
`cargo fmt --all -- --check` and the applicable native capture checks documented
in README.md. Headless egui tests do not replace real Windows DPI checks.
Check Markdown links, local image references and YAML syntax. Report only checks
that actually ran and distinguish them from pending platform/CI verification.

## Project journal and remote checks

- Maintain this project's authoritative history in `docs/PROJECT_LOG.md` in this
  repository. Keep this project's entries separate from other projects and
  general work summaries.
- Before writing, read this file and the current journal, fetch safely, and
  preserve local and remote changes. Never force-push or include secrets.
- Record Europe/Kyiv timestamps, scope/period, sources checked and inaccessible
  sources. The first check covers all accessible history; later checks include
  new replies to old issues/PRs since the previous check.
- Use tables for events/actions. Preserve history and update an event with the
  same URL instead of duplicating it. Record author, human/bot identity, date,
  summary, direct link, required action and processing status for comments,
  reviews, review threads and Discussions when enabled.
- Record explicit offers/requests for help separately, with task, message URL
  and whether an owner response is needed. Offers are not assignments; stars
  and forks are not offers. Assign an owner only with confirmed authorization.
- Keep the six journal sections requested by the owner. Entries are in English;
  the requested section labels are retained. Do not publish private messages.
- Commit journal updates separately through a branch/PR in this repository.
  Claim a GitHub update only after a successful push. If unavailable, provide
  the prepared entry and describe the limitation.
- Summaries use the owner's project/status/comments/help/next actions/check
  date table. Distinguish recorded evidence from fresh checks. Do not send
  public participant replies or assignments without explicit authorization.
- No recurring monitoring is configured. Do not promise scheduled checks.

## Owner decisions: Rust and Windows desktop reference

- Rust is the required development language for workbook/cell models, parser,
  dependency graph, calculation, undo/redo, persistence, Office integration and
  the GUI. This supersedes the provisional Python/Tk decision in ADR 0001.
- The active application must be a native desktop Rust application. Do not use
  Electron, a WebView/HTML/JavaScript table, or Rust only as a backend for one.
  The historical Python code is a conformance archive, never a runtime/build
  dependency of the Rust product.
- Use desktop Microsoft Excel 2024 for Windows as the layout/interaction target,
  while retaining the BreakFree Sheets name, own approved transparent icon and
  shared BreakFree palette. No Microsoft logo, invented Microsoft account,
  compatibility claim or visual-identity claim without evidence.
- Separate grid painting/interaction from the tested computation engine. Paint
  only the visible viewport; do not allocate a heavyweight widget per cell.
- Keep selection and editing modes explicit. Grid entry and formula bar must
  share one draft backed by the same typed cell model. Display formatting never
  rewrites stored content. Unsupported commands/fill/sort are explicitly disabled.
- Preserve the File/ribbon, Home groups, name box, formula bar, headers, selection,
  sheet-tab area, scrollbars and status/zoom layout. Record geometry, verified
  reference sources and differences in docs/UI_SPEC.md. Distinguish simulated
  scale tests from real Windows 100%, 125%, 150% measurements and matched screenshots.
- Preserve working calculation semantics/native v1 data when porting language;
  use conformance/regression tests. Do not replace a correct engine for cosmetic
  changes. Rust is decided; toolkit evolution and project licensing remain
  separate review topics. No automatic merge or release.

## Version and release approval

- Follow docs/VERSIONING.md and ADR 0004. Cargo.toml package.version is the
  source of the actual application version; CLI and Office JSON use that value.
- Keep initial prereleases on `0.1.0-alpha.N`, `0.1.0-beta.N` and `0.1.0-rc.N`,
  with positive integer N. The first stable product release is `0.2.0`.
- End every final task report with the current version and its approval status,
  presenting a pending version to the owner for agreement. Read the current
  value; do not guess it or claim approval from silence.
- Do not increment automatically after each task or promote a release channel
  without owner approval. Keep Cargo.lock, the capabilities fixture and current
  version documentation consistent with each proposed change.
- Version agreement and release publication are separate decisions. No automatic
  tags, releases, merges or published-history rewrites. Keep released versions
  immutable. App, integration-protocol and workbook-content versions are separate.
