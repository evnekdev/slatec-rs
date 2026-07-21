# SLATEC Routine Coverage and Reconciliation

[Complete index](slatec-routine-index.md) · [Browse by family](routines-by-family.md) · [Alphabetical lookup](routines-alphabetical.md)

| Measure | Count |
| --- | ---: |
| Logical identities | 1517 |
| Historically user-callable | 902 |
| Subsidiary/helper | 539 |
| Source files | 2192 |
| Providers | 2238 |
| Catalogue-only | 26 |
| Generated raw declaration candidates | 1253 |
| Safely wrapped | 210 |
| Deeply audited | 210 |

The generated-candidate count is compiler/profile evidence only. The canonical raw inventory below separates it from reviewed declarations, provider coverage, link/runtime evidence, and documentation.

The documented reference values are approximately 902 user-callable and at least 1,400 total routines. Computed values are reported as found; unresolved entries and provider variants remain visible rather than being invented or collapsed.

## Outstanding reconciliation

**103** diagnostics cover catalogue-only identities, conflicting providers, and unresolved roles. Description text uses a concise TOC or pilot purpose only where current evidence provides it; all other records state explicit unavailability.

## Family classification

| Measure | Count |
| --- | ---: |
| Retained routine identities | 1517 |
| Historical numerical program units | 1478 |
| Subsidiary routines | 539 |
| Shared numerical utilities | 31 |
| Runtime and machine support units | 24 |
| Documentation/tooling program units | 15 |
| Excluded intrinsic references | 0 |
| Excluded external symbols | 0 |
| Excluded parser/prose candidates | 4 |
| Classified with verified GAMS/package evidence | 1102 |
| Classified with high-confidence inheritance | 355 |
| Classified by conservative description inference | 60 |
| Genuinely unresolved | 0 |

<!-- raw-api-coverage:start -->
## Canonical raw API inventory

The former aggregate is retired because it conflated generated declarations, reviewed declarations, and validation evidence. The [authoritative status records](../../generated/raw-api/routine-status.json) make every metric reproducible.

| Metric | Count |
| --- | ---: |
| Retained identities | 1517 |
| Historically user-callable routines | 902 |
| Generated raw declaration candidates | 1286 |
| ABI-validated generated declarations | 1286 |
| Reviewed family raw declarations | 175 |
| Reviewed user-callable raw drivers | 175 |
| Reviewed public subsidiaries | 0 |
| Provider-backed callable raw routines | 1476 |
| Link-tested raw routines | 1373 |
| Runtime-tested raw routines | 770 |
| Fully documented raw routines | 814 |
| Pre-existing family declarations pending R1 review | 1342 |
| Safely wrapped routines | 210 |
| Explicitly excluded routines | 132 |
| Unclassified routines | 0 |

The definitions and exclusions are generated in [coverage-summary.json](../../generated/raw-api/coverage-summary.json) and [exclusion-report.json](../../generated/raw-api/exclusion-report.json).
<!-- raw-api-coverage:end -->

The fully documented count is calculated from the rendered canonical public
rustdoc semantic audit, not merely from generated routine-reference Markdown.
It requires bounded, argument-specific evidence; source-first direction
resolution; applicable structured status/workspace/callback semantics; and no
detected cross-contamination. See
[semantic-quality-final.json](../../generated/slatec-routines/semantic-quality-final.json),
[direction-evidence-conflicts.json](../../generated/slatec-routines/direction-evidence-conflicts.json),
[argument-contamination-audit.json](../../generated/slatec-routines/argument-contamination-audit.json),
and [rendered-rustdoc-semantic-audit.json](../../generated/slatec-routines/rendered-rustdoc-semantic-audit.json).

## Callback-bearing raw interfaces

The callback-specific public raw tier is source-reconstructed and
ABI-fingerprint guarded. Its reproducible counts are generated in the internal
callback evidence reports:

| Metric | Count |
| --- | ---: |
| Callback-bearing retained identities | 161 |
| Canonical callback-bearing routines | 47 |
| Already-public eligible callback routines | 22 |
| Unique callback ABI fingerprints | 10 |
| Forwarded callback signatures | 21 |
| Runtime callback smoke routines | 3 |

The complete per-routine callback classification and deferment reasons are in
[`callback-classification.json`](../../generated/raw-api/callback-classification.json)
and the generated raw API evidence directory.
