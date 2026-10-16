# Working in BreakFree Sheets

Read README.md, ROADMAP.md, docs/REQUIREMENTS.md, CONTRIBUTING.md, existing ADRs and docs/PROJECT_LOG.md first. The repository contains an independent Python/Tk prototype with a provisional stack decision; the software license remains unselected.

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

Run the headless tests and applicable opt-in GUI tests documented in README.md.
Check Markdown links, local image references and YAML syntax. Report only checks
that actually ran and distinguish them from pending platform/CI verification.

## Project journal and remote checks

- Maintain this project's history in `docs/PROJECT_LOG.md` in this repository;
  do not mix detailed history with other projects or a shared work log.
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
