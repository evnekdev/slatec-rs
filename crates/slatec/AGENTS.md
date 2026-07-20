# `slatec` safe-facade instructions

Scope: `crates/slatec/**`.

## Responsibility

This crate provides checked, ergonomic Rust APIs over reviewed `slatec-sys` declarations.

## Rules

- Safe wrappers must validate every precondition needed to uphold the raw routine's contract.
- Preserve numerical semantics, argument meaning, workspace requirements, error/warning mapping, and concurrency restrictions.
- Safe wrappers must not reference unrelated numerical routines merely because they share a source file, module, family, trait, registry, or feature.
- Avoid production function-pointer registries or dispatch tables that take the address of every routine.
- Keep tooling/documentation metadata out of runtime production reachability unless explicitly feature-gated.
- Broad family features may expose many operations, but enabling them without calling an operation must not retain native implementations.
- Calling one operation should retain only that wrapper, shared safety/runtime infrastructure, its raw routine, and legitimate native transitive dependencies.
- Do not solve granularity by adding one Cargo feature per function unless explicitly required by evidence.
- Keep public paths stable unless a breaking change is explicitly authorized.
- Prefer owned/borrowed APIs that make dimensions, mutability, storage, and workspace lifetimes clear.
- Native calls that rely on non-thread-safe SLATEC state must use the repository's established lock policy.

## Tests expected for new wrappers

Include, as applicable:

- validation/error-path tests;
- independent oracle or manufactured-solution tests;
- boundary and singular cases;
- precision-specific tests;
- native ABI/integration tests;
- concurrency/locking tests;
- reduced-feature compilation;
- safe-vs-raw link-closure probes for representative operations.
