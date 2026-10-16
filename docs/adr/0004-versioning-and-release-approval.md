# ADR 0004 — Prerelease numbering and owner approval

- Date: 2026-10-16
- Status: accepted owner policy, amended 2026-10-16; exact current candidate pending agreement
- Scope: application release versions; no protocol or workbook-format migration

## Decision

All initial alpha, beta and release-candidate builds use base `0.1.0` with
SemVer labels `alpha.N`, `beta.N` and `rc.N`. The first stable published product
version is `0.2.0`. Each task's final report must end with the actual current
version and its approval status so the owner can agree to it.

The initial candidate was `0.1.0-alpha.1`, replacing the unreleased provisional
`0.2.0-dev.1` label. Preserve those historical labels. Cargo package metadata
supplies both CLI and Office application-version output; keep the fixture and
root lockfile entry consistent without changing dependency versions.

### Owner amendment: every change increments the version

Every completed changeset now increments the current prerelease counter by one
and adds a dated root CHANGELOG.md entry. This includes code, design, tests,
configuration and documentation. It replaces the earlier prohibition on preparing
an increment without a separate number approval. The implementation and its
accompanying verification/changelog/journal records form one changeset; those
records do not trigger recursive increments. Read-only work leaves the version
unchanged. Further independent changes require another increment.

Prepare `0.1.0-alpha.2` for the preceding native design refinement and this policy
correction. The exact candidate is presented to the owner at completion. Preparing
it under the standing instruction does not imply owner acceptance or publication.
See [VERSIONING.md](../VERSIONING.md) for the sequence and change boundaries.
Channel promotion and publication remain separate owner decisions. No tag,
release, default-branch merge or history rewrite is authorized by this amendment.

## Consequences

Application versions remain separate from integration protocol v1 and native
content v1. Saved books need no migration. Version proposals do not constitute
accepted releases, and passing local tests or CI cannot stand in for approval.
Subsequent tasks report the current version even when no increment is needed.
