# Native link probe comparison

Sizes are release-probe section measurements. They are diagnostic measurements, not exact-byte regression thresholds. Executable file bytes are deliberately omitted because PE linker layout can vary by a few bytes between clean roots.

| Probe | .text | .data/.rdata | .bss |
|---|---:|---:|---:|
| all_saxpy_only | 634408 | 132104 | 624 |
| baseline_no_slatec | 633240 | 132072 | 624 |
| raw_all_no_call | 633240 | 132040 | 624 |
| raw_dai_only | 957592 | 179344 | 4172 |
| raw_ddot_only | 634296 | 132104 | 624 |
| raw_dgamma_only | 957784 | 175472 | 4108 |
| raw_dgemm_only | 957208 | 174064 | 4044 |
| raw_dgemv_only | 955992 | 174064 | 4044 |
| raw_fzero_only | 954904 | 174032 | 4012 |
| raw_hwscrt_only | 676168 | 132424 | 624 |
| raw_pois3d_only | 698712 | 132728 | 624 |
| raw_saxpy_ddot | 635480 | 132072 | 624 |
| raw_saxpy_only | 634408 | 132104 | 624 |
| safe_airy_only | 959832 | 179952 | 4204 |
| safe_all_no_call | 633240 | 132040 | 624 |
| safe_ddot_only | 636808 | 132936 | 624 |
| safe_dgemm_only | 961176 | 174992 | 4044 |
| safe_dgemv_only | 960344 | 175088 | 4044 |
| safe_fishpack_hwscrt_only | 694760 | 135752 | 656 |
| safe_roots_only | 965272 | 176112 | 4044 |
| safe_saxpy_only | 637384 | 132968 | 624 |
| safe_special_only | 960024 | 176080 | 4140 |

The static GNU runtime is selected only when a referenced SLATEC member needs it; archive and runtime on-disk sizes are not final-executable contributions. Link maps in `target/native-link/probes` provide the per-probe selected-member evidence.

Direct raw probes are the source-archive granularity regression baseline. Safe-facade probes are reported separately: a broad safe Rust compilation unit can retain a broader raw symbol set before the archive extractor runs, even though the Fortran archive itself remains one source per member. The symbol report makes that distinction explicit rather than treating safe-wrapper size as evidence of archive coalescing.
