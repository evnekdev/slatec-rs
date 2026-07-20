# Native link probe comparison

Sizes are release-probe section measurements. They are diagnostic measurements, not exact-byte regression thresholds.

| Probe | File bytes | .text | .data/.rdata | .bss |
|---|---:|---:|---:|---:|
| all_saxpy_only | 4875661 | 634408 | 132104 | 624 |
| baseline_no_slatec | 4873449 | 633240 | 132072 | 624 |
| raw_all_no_call | 1178768 | 633240 | 132040 | 624 |
| raw_ddot_only | 4875123 | 634296 | 132104 | 624 |
| raw_dgamma_only | 5488315 | 957784 | 175472 | 4108 |
| raw_dgemm_only | 5484685 | 957208 | 174064 | 4044 |
| raw_dgemv_only | 5483661 | 955992 | 174064 | 4044 |
| raw_fzero_only | 5482349 | 954904 | 174032 | 4012 |
| raw_hwscrt_only | 4919470 | 676168 | 132424 | 624 |
| raw_pois3d_only | 4948521 | 698712 | 132728 | 624 |
| raw_saxpy_ddot | 4876353 | 635480 | 132072 | 624 |
| raw_saxpy_only | 4875661 | 634408 | 132104 | 624 |
| safe_all_no_call | 1180054 | 633240 | 132040 | 624 |
| safe_ddot_only | 1185261 | 636808 | 132936 | 624 |
| safe_dgemm_only | 1718300 | 961176 | 174992 | 4044 |
| safe_dgemv_only | 1717443 | 960344 | 175088 | 4044 |
| safe_fishpack_hwscrt_only | 1259796 | 694760 | 135752 | 656 |
| safe_roots_only | 1743006 | 965272 | 176112 | 4044 |
| safe_saxpy_only | 1186931 | 637384 | 132968 | 624 |
| safe_special_only | 1719510 | 960024 | 176080 | 4140 |

The static GNU runtime is selected only when a referenced SLATEC member needs it; archive and runtime on-disk sizes are not final-executable contributions. Link maps in `target/native-link/probes` provide the per-probe selected-member evidence.

Direct raw probes are the source-archive granularity regression baseline. Safe-facade probes are reported separately: a broad safe Rust compilation unit can retain a broader raw symbol set before the archive extractor runs, even though the Fortran archive itself remains one source per member. The symbol report makes that distinction explicit rather than treating safe-wrapper size as evidence of archive coalescing.
