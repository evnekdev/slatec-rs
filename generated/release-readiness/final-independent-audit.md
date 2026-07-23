# Final independent release audit

Independent local status: **pass**. This report is deliberately package-first and does not treat a generated release-readiness report as proof of itself.

## Packaged crates

| Crate | Canonical compressed bytes | Files | Result |
| --- | ---: | ---: | --- |
| `slatec-bundled-x86_64-pc-windows-gnu` | 389614 | 22 | pass |
| `slatec-bundled-x86_64-unknown-linux-gnu` | 406269 | 23 | pass |
| `slatec-sys` | 612664 | 898 | pass |
| `slatec-core` | 7231 | 12 | pass |
| `slatec-src` | 157289 | 35 | pass |
| `slatec` | 344073 | 319 | pass |

## Carrier archives

| Carrier | Target | Result |
| --- | --- | --- |
| `slatec-bundled-x86_64-pc-windows-gnu` | `x86_64-pc-windows-gnu` | pass |
| `slatec-bundled-x86_64-unknown-linux-gnu` | `x86_64-unknown-linux-gnu` | pass |

## Discrepancies

None. The independently recomputed counts and hashes match the committed release-candidate report.

## Decision boundary

The independent local audit passes only its direct evidence. Publication remains conditional on the explicit manual release gates in release-blockers.json; this report never claims an unobserved CI workflow or publication succeeded.
