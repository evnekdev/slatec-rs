# Runtime concurrency audit

- Classified safe functions: 188 (`BackendDependent`: 60, `SerializedGlobal`: 128).
- `SerializedGlobal` records enter the reentrant-per-thread, process-wide native runtime guard. This permits a non-callback native helper inside an active hosted call, but callback-based nested native operations remain rejected.
- `BackendDependent` records preserve existing `no_std`/`alloc` capability and do not make a Rust thread-safety claim. They require provider provenance, source-level reentrancy evidence, and parallel stress tests before any narrower class is adopted.
- `SDRIV3`/`DDRIV3` sessions remain `SerializedGlobal`: the session callback context and scoped XERROR state make a per-family or lock-free claim premature.
- The complete SDRIVE source closure could not be rebuilt from the available offline cache: `DDCOR` was missing first, and the composed local cache also lacked LINPACK `DGBFA`. No source was downloaded, copied into the repository, or treated as a successful native validation.
