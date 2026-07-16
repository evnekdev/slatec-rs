# Architecture

Planned layering:

1. Native SLATEC Fortran sources.
2. Raw ABI declarations in `slatec-sys`.
3. Shared safe infrastructure in `slatec-core`.
4. Domain-focused safe APIs exposed by `slatec`.
5. Development and source-analysis utilities in `slatec-tools`.

This document intentionally contains no implementation design yet.
