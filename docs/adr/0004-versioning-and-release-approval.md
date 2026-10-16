# ADR 0004 — Prerelease numbering and owner approval

- Date: 2026-10-16
- Status: accepted owner policy; exact current candidate pending agreement
- Scope: application release versions; no protocol or workbook-format migration

## Decision

All initial alpha, beta and release-candidate builds use base `0.1.0` with
SemVer labels `alpha.N`, `beta.N` and `rc.N`. The first stable published product
version is `0.2.0`. Each task's final report must end with the actual current
version and its approval status so the owner can agree to it.

Prepare `0.1.0-alpha.1` as the current development candidate, pending exact-number
agreement. Replace the unreleased provisional `0.2.0-dev.1` label; preserve its
historical records. Cargo package metadata already supplies both CLI and Office
application-version output. Keep that existing single source and its contract
fixture consistent; dependency versions stay pinned.

The concrete sequence, candidate state and approval rules are documented in
[VERSIONING.md](../VERSIONING.md). An increment or channel promotion requires
owner agreement. Publishing requires a separate instruction; no tag, release,
default-branch merge or history rewrite is authorized by this decision.

## Consequences

Application versions remain separate from integration protocol v1 and native
content v1. Saved books need no migration. Version proposals do not constitute
accepted releases, and passing local tests or CI cannot stand in for approval.
Subsequent tasks report the current version even when no increment is needed.
