# Runtime concurrency audit

- Classified safe functions: 188 (`BackendDependent`: 60, `SerializedGlobal`: 128).
- `SerializedGlobal` records enter the reentrant-per-thread, process-wide native runtime guard. This permits a non-callback native helper inside an active hosted call, but callback-based nested native operations remain rejected.
- `BackendDependent` records preserve existing `no_std`/`alloc` capability and do not make a Rust thread-safety claim. They require provider provenance, source-level reentrancy evidence, and parallel stress tests before any narrower class is adopted.
- Rust ownership safety, exact retained-native-closure reentrancy, and provider/runtime thread safety are separate fields; independent buffers do not prove native reentrancy.
- The origin audit hash-verifies and scans 330 provider sources, compiles and inspects their objects plus three profile objects, records no selected-closure COMMON block, and separately preserves 172 COMMON declarations from the 1,452-file reviewed corpus. GNU Fortran parse-tree validation reports no selected-source scanner disagreement.
- `SDRIV3`/`DDRIV3` sessions remain `SerializedGlobal`: `SDSTP`/`DDSTP` emit saved writable `IER` storage, XERROR is process-global, callback dispatch is scoped, and provider/runtime reentrancy is not a public guarantee. Cross-family native instrumentation observes at most one active hosted native lock scope.
