# Reviewed raw real Airy functions

R2C promotes the eight real, historically user-callable FNLIB Airy functions:
Ai and Bi, in single and double precision, plus their exponentially scaled
forms. Their canonical raw paths are:

```rust
slatec_sys::special::airy::{ai, aie, bi, bie, dai, daie, dbi, dbie}
```

Enable `slatec-sys` feature `special-airy` and link a compatible provider. The
matching source-provider feature is `slatec-src/special-airy`; enabling either
raw declaration feature never selects a provider on its own. The former
`slatec_sys::families::special_airy::*` paths remain compatibility re-exports,
not duplicate declarations.

```rust,no_run
let mut x = 0.0_f64;
let ai = unsafe { slatec_sys::special::airy::dai(&mut x) };
assert!(ai.is_finite());
```

Every function takes one non-null, aligned, readable real scalar pointer by
address. It treats that scalar as input-only, retains no pointer, and returns
its real result directly for the supported GNU MinGW ABI. The unscaled Ai and
Bi routines retain their source-documented XERMSG underflow and overflow
paths; scaled forms apply the documented positive-axis exponential factors.
FNLIB saved initialization and legacy error handling are process-global, so a
raw consumer must provide synchronization when calls can overlap.

The deterministic [`Airy family report`](../../generated/raw-api/airy-family-report.json)
records these eight reviewed drivers and explicitly defers complex Amos Airy
drivers and Airy subsidiaries until their separate ABI and semantic contracts
are reviewed.
