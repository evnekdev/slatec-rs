# Runtime concurrency audit

- Classified safe functions: 241 (`BackendDependent`: 60, `SerializedGlobal`: 181).
- `SerializedGlobal` records enter the reentrant-per-thread, process-wide native runtime guard. This permits a non-callback native helper inside an active hosted call, but callback-based nested native operations remain rejected.
- `BackendDependent` records preserve existing `no_std`/`alloc` capability and do not make a Rust thread-safety claim. They require provider provenance, source-level reentrancy evidence, and parallel stress tests before any narrower class is adopted.
- Rust ownership safety, exact retained-native-closure reentrancy, and provider/runtime thread safety are separate fields; independent buffers do not prove native reentrancy.
- The origin audit hash-verifies, scans, compiles, and inspects every selected base and overlay source. It records selected LP COMMON blocks and preserves full-corpus COMMON findings separately; any parser disagreement or source/object mismatch remains conservatively unresolved rather than weakening serialization.
- `SDRIV3`/`DDRIV3` and the safe in-memory `SPLP`/`DSPLP` subset remain `SerializedGlobal`: mutable saved/COMMON storage, XERROR, callback dispatch, and provider/runtime uncertainty require the lock. Cross-family native instrumentation observes at most one active hosted native lock scope.
