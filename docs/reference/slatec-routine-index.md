# Complete SLATEC Routine Index

This deterministic catalogue is both a historical inventory and a `slatec-rs` coverage map. It contains **1521** logical identities: **902** historically user-callable and **539** explicitly subsidiary identities. Source files, program units, logical identities, and providers are deliberately distinct.

- [Browse by Function Family](routines-by-family.md)
- [Browse Alphabetically](routines-alphabetical.md)
- [Coverage and Reconciliation](routine-coverage.md)

## Evidence and status

Evidence reconciles immutable `main-src`, Netlib `slatec/list`, the Version 4.1 TOC, live `src`, relocated subsets, supplements, pilot metadata, raw-interface inventories, and safe-API indexes. A **provider** is a source location; **raw binding** means generated ABI coverage under a reviewed profile; **safe API** means a public Rust wrapper; **audited** is reserved for reviewed safe wrappers; **deferred** means it is not exposed as a safe API.

Inclusion does not imply a modern recommendation, ABI safety, compilation, a canonical provider, a raw binding, or a stable public interface for a subsidiary routine.
