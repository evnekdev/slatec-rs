# Complete SLATEC Routine Index

This deterministic catalogue is both a historical inventory and a `slatec-rs` coverage map. It contains **1521** logical identities: **902** historically user-callable and **539** explicitly subsidiary identities. Source files, program units, logical identities, and providers are deliberately distinct.

- [Browse by Function Family](routines-by-family.md)
- [Browse Alphabetically](routines-alphabetical.md)
- [Coverage and Reconciliation](routine-coverage.md)

## Evidence and status

Evidence reconciles immutable `main-src`, Netlib `slatec/list`, the Version 4.1 TOC, live `src`, relocated subsets, supplements, pilot metadata, raw-interface inventories, and safe-API indexes. A **provider** is a source location; **raw binding** means generated ABI coverage under a reviewed profile; **safe API** means a public Rust wrapper; **audited** is reserved for reviewed safe wrappers; **deferred** means it is not exposed as a safe API.

Inclusion does not imply a modern recommendation, ABI safety, compilation, a canonical provider, a raw binding, or a stable public interface for a subsidiary routine.

## Description evidence

Descriptions are assembled from canonical Netlib source prologues, the official Version 4.1 TOC, cached Netlib directory metadata, NIST GAMS where a verified module match exists, and reviewed secondary sources. Source revisions can differ; the canonical source prologue takes precedence. Compact indexes show a complete short purpose; [routine details and evidence](routine-details.md) retain full source descriptions and external cross-references. External references are cross-checks, not replacements for local evidence.
