# Application versions and owner approval

Status: owner-decided release sequence and per-change increments, amended
2026-10-16. The current exact candidate number awaits owner agreement.
This policy does not authorize a product release.

## Sequence

Use [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) syntax.
The owner's `0.1.0x` prerelease line is represented as follows:

| Stage | Application version | Progression |
| :--- | :--- | :--- |
| Alpha | `0.1.0-alpha.N` | Early working iterations; N starts at 1 |
| Beta | `0.1.0-beta.N` | Broader acceptance testing; N restarts at 1 |
| Release candidate | `0.1.0-rc.N` | Candidate for release acceptance; N restarts at 1 |
| First stable product release | `0.2.0` | Only after explicit approval and successful publication |

N is a positive integer without leading zeros. Increment it by one for every
completed changeset in the current stage under the owner's standing instruction.
Example sequence: `0.1.0-alpha.1`,
`0.1.0-alpha.2`, `0.1.0-beta.1`, `0.1.0-rc.1`, `0.2.0`.
These are progression examples, not a promise that every stage is accepted.
The first stable release uses `0.2.0`, the canonical spelling of the owner's
`0.2`. No stable `0.1.0` release is scheduled. Later versions need their own
scope and owner approval.

## Current state

| Field | State |
| :--- | :--- |
| Current development candidate | `0.1.0-alpha.3` |
| Previously agreed candidate | `0.1.0-alpha.2`; owner agreement on 2026-10-16, not released |
| Exact-number approval | Pending owner agreement |
| Published product version | None; no release or tag created |
| First stable release target | `0.2.0`; not yet accepted or published |

The current candidate advances the owner-agreed `0.1.0-alpha.2` with native
ribbon and worksheet-view refinements. Its new exact number is pending agreement.
The earlier provisional `0.2.0-dev.1` label and alpha.1 were never published as
product releases. Historical commits, validation records and captured images
retain the versions they actually had; they are not retroactively relabeled.
A public source branch, PR or CI artifact does not advance product-release state.

## One version and changelog entry for every change

Every completed project changeset increments N by one and adds a dated section
to the root [CHANGELOG.md](../CHANGELOG.md). This includes implementation, design,
fixes, dependency/configuration changes, tests and documentation-only work.
The owner's instruction authorizes preparing the next candidate without waiting
for a separate pre-implementation number approval. Report it for owner agreement
at completion; preparing a number does not mean it has been agreed or released.

A changeset is a coherent unit of work delivered for review. Its intermediate
edits, verification results, changelog, required separate journal commit and
publication metadata belong to the same version. Those accompanying records do
not recursively produce more versions. Once delivered, a subsequent independent
change or correction gets the next number. Read-only checks or discussion without
repository changes keep the version; edits resulting from a check require a bump.

Keep changelog entries newest first, with the exact application version, date
(Europe/Kyiv), status (development candidate, agreed or released), concise
Added/Changed/Fixed items as applicable, actual verification and material limits.
Preserve prior entries and distinguish historical baseline reconstruction from
work performed in the current change. Never label an unpublished candidate as a
release or invent a release date. Channel promotion still needs owner agreement.

## Source and checks

`Cargo.toml` package.version is the single application-version source used by
`CARGO_PKG_VERSION`, `--version` and Office `app_version`. Keep the root package
entry in Cargo.lock, `contracts/v1/capabilities.example.json`, README and this
current-state table and CHANGELOG.md consistent when preparing a version change.
Dependency versions and historical fixtures are not changed to match the application.

Integration `protocol_version` and workbook `file_version` remain independently
versioned; both are currently 1. An application-version change does not migrate
workbook contents or imply a protocol/format compatibility change.

Run the existing contract/CLI tests and verify the built executable's version and
capabilities from outside the checkout. Record actual local/CI results and any
platform gaps. Before publishing, verify that the approved version, immutable
source commit, release notes and artifacts all identify the same candidate.
The future tag convention is `v<application-version>`; no tag is created here.

## Approval and reporting

End **every final task report** with the current application version read from
Cargo.toml or the built executable, plus its approval status. If pending, present
that exact version to the owner for agreement. Report the current version even
when the task did not change it. A task that changes the repository must include
the next prerelease number and changelog entry before it is delivered.

Keep prepared, owner-agreed and published states distinct. Silence, passing CI
or an earlier version's approval does not approve the current exact number.
The mandatory per-change increment replaces the earlier rule that blocked all
increments pending separate approval. Owner agreement remains necessary for
channel promotion and release acceptance; publication requires its own explicit
instruction. Once published, a release's contents are immutable. No automatic
release, tag, merge or published-history rewrite is part of this workflow.
