# Final independent release audit

Independent local status: **fail**. This report is deliberately package-first and does not treat a generated release-readiness report as proof of itself.

## Packaged crates

| Crate | Compressed bytes | Files | Result |
| --- | ---: | ---: | --- |
| `slatec-bundled-x86_64-pc-windows-gnu` | 385724 | 22 | pass |
| `slatec-bundled-x86_64-unknown-linux-gnu` | 398565 | 23 | pass |
| `slatec-sys` | 611732 | 898 | pass |
| `slatec-core` | 7439 | 12 | pass |
| `slatec-src` | 159512 | 35 | pass |
| `slatec` | 344483 | 319 | pass |

## Carrier archives

| Carrier | Target | Result |
| --- | --- | --- |
| `slatec-bundled-x86_64-pc-windows-gnu` | `x86_64-pc-windows-gnu` | pass |
| `slatec-bundled-x86_64-unknown-linux-gnu` | `x86_64-unknown-linux-gnu` | pass |

## Discrepancies

- `{"changed_file_count":825,"gate":"committed release-readiness evidence freshness","reason":"the transactional generator recomputation differs from committed evidence","sample_changed_files":["docs/reference/routines/acosh.md","docs/reference/routines/ai.md","docs/reference/routines/aie.md","docs/reference/routines/albeta.md","docs/reference/routines/algams.md","docs/reference/routines/ali.md","docs/reference/routines/alngam.md","docs/reference/routines/alnrel.md","docs/reference/routines/asinh.md","docs/reference/routines/atanh.md","docs/reference/routines/avint.md","docs/reference/routines/bakvec.md","docs/reference/routines/balanc.md","docs/reference/routines/balbak.md","docs/reference/routines/bandr.md","docs/reference/routines/bandv.md","docs/reference/routines/besi.md","docs/reference/routines/besi0.md","docs/reference/routines/besi0e.md","docs/reference/routines/besi1.md"]}`

## Decision boundary

The independent local audit found discrepancies or critical blockers. Publication is not permitted until they are resolved and the audit is regenerated; unobserved CI and publication outcomes remain manual release gates.
