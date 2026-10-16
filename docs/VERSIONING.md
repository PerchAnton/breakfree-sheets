# Application versions and owner approval

Status: owner-decided release sequence; the current exact candidate number awaits
owner agreement. This policy does not authorize a product release.

## Sequence

Use [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) syntax.
The owner's `0.1.0x` prerelease line is represented as follows:

| Stage | Application version | Progression |
| :--- | :--- | :--- |
| Alpha | `0.1.0-alpha.N` | Early working iterations; N starts at 1 |
| Beta | `0.1.0-beta.N` | Broader acceptance testing; N restarts at 1 |
| Release candidate | `0.1.0-rc.N` | Candidate for release acceptance; N restarts at 1 |
| First stable product release | `0.2.0` | Only after explicit approval and successful publication |

N is a positive integer without leading zeros. Increase it within a stage only
for an agreed new candidate. Example sequence: `0.1.0-alpha.1`,
`0.1.0-alpha.2`, `0.1.0-beta.1`, `0.1.0-rc.1`, `0.2.0`.
These are progression examples, not a promise that every stage is accepted.
The first stable release uses `0.2.0`, the canonical spelling of the owner's
`0.2`. No stable `0.1.0` release is scheduled. Later versions need their own
scope and owner approval.

## Current state

| Field | State |
| :--- | :--- |
| Current development candidate | `0.1.0-alpha.1` |
| Exact-number approval | Pending owner agreement |
| Published product version | None; no release or tag created |
| First stable release target | `0.2.0`; not yet accepted or published |

This replaces the earlier provisional `0.2.0-dev.1` source label, which was never
published as a product release. Existing historical records and the archived
prototype retain their original labels. A public source branch, PR or CI artifact
does not advance the product-release state.

## Source and checks

`Cargo.toml` package.version is the single application-version source used by
`CARGO_PKG_VERSION`, `--version` and Office `app_version`. Keep the root package
entry in Cargo.lock, `contracts/v1/capabilities.example.json`, README and this
current-state table consistent when preparing a version change. Dependency
versions and historical fixtures are not changed to match the application.

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
when the task did not change it. Do not increment it merely because a task ended.

Keep proposed, owner-agreed and published states distinct. Silence, passing CI,
an earlier version's approval or approval of this policy does not approve a new
candidate number. Record explicit owner agreement before advancing the number
or channel. Version agreement alone does not authorize publishing a release.
Once published, a version's contents are immutable; corrections need an agreed
new version. No automatic release, tag, merge or published-history rewrite is
part of this workflow.
