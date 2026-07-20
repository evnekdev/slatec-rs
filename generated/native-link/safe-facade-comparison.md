# Safe-facade versus raw link comparison

The raw row is the direct source-archive baseline. `safe-only` may contain checked wrapper/runtime support and genuine native dependencies; the native-symbol assertions reject unrelated numerical drivers. Link maps remain under ignored `target/native-link/probes`.

| Safe probe | Raw probe | Raw native symbols | Safe native symbols | Safe-only native symbols | Assertions |
|---|---|---:|---:|---:|---|
| safe_ddot_only | raw_ddot_only | 1 | 1 | 0 | true |
| safe_dgemm_only | raw_dgemm_only | 12 | 12 | 0 | true |
| safe_dgemv_only | raw_dgemv_only | 12 | 12 | 0 | true |
| safe_fishpack_hwscrt_only | raw_hwscrt_only | 11 | 11 | 0 | true |
| safe_roots_only | raw_fzero_only | 11 | 13 | 2 | true |
| safe_saxpy_only | raw_saxpy_only | 1 | 1 | 0 | true |
| safe_special_only | raw_dgamma_only | 15 | 15 | 0 | true |
