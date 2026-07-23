# Final independent release audit

Independent local status: **pass**. This report is deliberately package-first and does not treat a generated release-readiness report as proof of itself.

## Packaged crates

| Crate | Package size | Files | Result |
| --- | --- | ---: | --- |
| `slatec-bundled-x86_64-pc-windows-gnu` | within 10 MiB | 22 | pass |
| `slatec-bundled-x86_64-unknown-linux-gnu` | within 10 MiB | 23 | pass |
| `slatec-sys` | within 10 MiB | 898 | pass |
| `slatec-core` | within 10 MiB | 12 | pass |
| `slatec-src` | within 10 MiB | 35 | pass |
| `slatec` | within 10 MiB | 319 | pass |

## Carrier archives

| Carrier | Target | Result |
| --- | --- | --- |
| `slatec-bundled-x86_64-pc-windows-gnu` | `x86_64-pc-windows-gnu` | pass |
| `slatec-bundled-x86_64-unknown-linux-gnu` | `x86_64-unknown-linux-gnu` | pass |

## Discrepancies

None. The independently recomputed counts and hashes match the committed release-candidate report.

## Decision boundary

The independent local audit passes only its direct evidence. Publication remains conditional on the explicit manual release gates in release-blockers.json; this report never claims an unobserved CI workflow or publication succeeded.
