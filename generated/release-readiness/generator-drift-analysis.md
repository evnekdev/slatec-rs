# Generator-drift analysis

The prior 825-file release blocker was a deterministic **generator regression**, not a bulk regeneration to accept. The current transactional recomputation is `pass` with **0** changed files and **0** ownerless files.

## Reproduction

- Base commit: `14f9f7ef4038723be47bfb38f9af57e0b6b25fd6`
- Clean recomputations: **2**, byte-identical: **true**
- Patch SHA-256: `186da7b446f676c195140ce2a0dfb3da0c584daf7de96dde2464ced0b5c64eca`
- Source snapshot: `slatec-corpus-695f28f400f2a4fac333`

## Historical partition

| Category | Files | Added lines | Removed lines | Owner | Risk |
| --- | ---: | ---: | ---: | --- | --- |
| routine semantic content | 821 | 14276 | 24193 | `public_api_semantic_review::generate/write_public_routine_page` | `critical` |
| generated inventories/indexes | 4 | 227546 | 443698 | `public_api_semantic_review::generate/write_public_routine_page` | `critical` |

## Resolution

Canonical-public routine pages and both semantic inventory mirrors are now written only by `public_api_semantic_review`. Release readiness still writes secondary pages, family navigation, cross-checks, and its own reconciliation evidence. The transactional validator reports owner, category, risk, line counts, EOL-only changes, ordering-only changes, schema changes, and canonical/safe-path changes; it does not auto-accept any of them.

## Representative semantic protections

- `docs/reference/routines/acosh.md` — semantic documentation downgrade. preserve the semantic-review page; prohibit release-readiness from writing canonical-public routine pages
- `generated/release-readiness/documentation-quality.json` — schema and semantic downgrade. semantic-review generator is the sole owner of both documentation-quality mirrors
- `generated/release-readiness/argument-documentation-coverage.json` — semantic coverage downgrade. semantic-review generator is the sole owner of both argument-coverage mirrors

The JSON companion enumerates all 825 historical paths, exact input hashes, output-to-input ownership, environment requirements, and the current transactional result.
