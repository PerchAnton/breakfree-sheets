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

- Maintain this project's authoritative history in `docs/PROJECT_LOG.md` in this
  repository and its full private mirror as specified below. Keep this project's
  entries separate from other projects and general work summaries.
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

## Журнал и приватное зеркало

- **Two copies are mandatory after every significant milestone and every journal
  update.** The authoritative journal is `docs/PROJECT_LOG.md` in
  `https://github.com/PerchAnton/breakfree-sheets`. The complete private copy is
  `breakfree/breakfree-sheets/PROJECT_LOG.md` on branch `main` of
  `https://github.com/PerchAnton/gitwork`. This replaces the previous prohibition
  against duplicating the journal in `gitwork`.
- Read the current `prompts/breakfree-sheets.md` in `gitwork/main`, applicable
  repository instructions and both existing journals. Fetch safely and preserve
  local changes and other authors' records. If the mirror has entries absent
  from the authoritative journal, reconcile them there without data loss and
  publish that journal before copying it. Never publish private correspondence
  or secrets while reconciling history.
- Publish the authoritative journal in a separate commit through this project's
  branch/PR workflow; do not merge `main` or release without separate authority.
  Then fetch a fresh `gitwork/main`, reread its existing mirror, and copy the full
  text of the published authoritative journal. A summary is not a mirror.
- Store mirror metadata separately at
  `breakfree/breakfree-sheets/SYNC.md` in `gitwork/main`: source repository,
  source branch, source path, full published source commit SHA, a permalink to
  the journal at that commit, and synchronization time with Europe/Kyiv timezone
  and UTC offset. Do not add private-only metadata to the mirrored journal text.
- Commit and push the mirror and metadata in a separate commit to `gitwork/main`.
  Update only the Sheets row of its Breakfree README table, with a short status,
  mirror/source links, check time and next action. Preserve other projects.
- Never force-push. If a push loses a race, fetch and reconcile concurrent work
  without losing either author's records, then retry normally. Reconcile any
  mirror-only records in the authoritative journal before replacing the mirror.
- Read both files back from GitHub after publication and verify identical text
  and the source SHA recorded in `SYNC.md`. Do not report synchronization based
  only on a local copy or a successful authoritative-journal push.
- If the private repository is unavailable, keep a prepared local copy and
  metadata, and explicitly report: «приватная копия не отправлена». Continue with
  the authorized publication that is possible; never imply both copies exist.
