# SLATEC source-corpus evidence

This directory contains the E01 candidate-artifact inventory for the source-corpus freezing programme. It is intentionally broader than the eventual canonical corpus.

## Files

- [`candidate-artifacts.md`](candidate-artifacts.md) — human-readable artifact register.
- [`artifact-relationships.md`](artifact-relationships.md) — provisional duplicate, relocation, upstream, documentation, generated and machine-specific relationships.
- [`retrieval-backlog.md`](retrieval-backlog.md) — explicit acquisition, checksum and reconciliation work for later tasks.
- [`corpus-completeness-audit-policy.md`](corpus-completeness-audit-policy.md) — scope boundary between the reproducible `main-src` subset and a future complete selected collection.
- [`full-corpus-audit-results.md`](full-corpus-audit-results.md) — compact measured explanation of the 735-unit `main-src` subset versus the 1,491-identity SLATEC-hosted union.
- [`../../metadata/candidate-artifacts.toml`](../../metadata/candidate-artifacts.toml) — valid TOML representation using stable project IDs.

## Status semantics

See [complete selected-corpus policy](complete-selected-corpus-policy.md) for
the derived hosted-provider selection that follows the completeness audit.

- `opened-index`: the authoritative index page was inspected; linked files were not necessarily opened.
- `link-verified`: an authoritative index advertises the artifact.
- `content-not-inspected` or `not-downloaded`: no content-level or archive-member claim is made.
- `pending-download`: no checksum was calculated.

## Programme boundary

E01 registers candidates only. It does not select a canonical baseline, infer package ownership from directory placement, or treat dependency products as executable linkage truth. E02 should acquire bytes and reconcile archives, live directories and relocated subsets.
