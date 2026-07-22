# Bundled provider and provenance gate

`slatec-src` selects exactly one implementation provider: `bundled`,
`source-build`, `system`, or `external-backend`. `bundled` is the ordinary
default. It never downloads source, reads `SLATEC_SOURCE_CACHE`, probes a
Fortran compiler, or searches a system archive.

## Current supported bundle

`x86_64-pc-windows-gnu` ships one reviewed compiler-free native family:
`special-elementary`. A normal user can select it with:

```toml
[dependencies]
slatec = { version = "0.1", features = ["special-elementary"] }
```

Its deterministic archive contains 28 hash-pinned historical SLATEC units and
the three project-owned GNU MinGW machine-profile units. It has a unique
family archive instead of one broad archive, so enabling a later family does
not make this first archive a hidden implementation root. All other families
remain unavailable with `bundled` until their entire closure is accepted.

| Mode | Compiler | Source cache | System archive | Current status |
| --- | --- | --- | --- | --- |
| `bundled` | Never | Never | Never | `special-elementary` on `x86_64-pc-windows-gnu` |
| `source-build` | GNU Fortran | Required, verified | No | Expert provider |
| `system` | No | No | Explicitly supplied | Expert provider |
| `external-backend` | No | No | Caller-managed | Emits no link directives |

An unsupported family or target fails with a provider-specific error and does
not fall back. Experts can still use `default-features = false` with
`source-build`, `system`, or `external-backend`.

## Provenance decision

The authoritative reviewed inputs are:

- `crates/slatec-src/metadata/bundled-provenance-evidence.json` — primary
  institutional evidence, retrieval date, short excerpt, and exact scope;
- `crates/slatec-src/metadata/bundled-provenance-overrides.json` — stable
  source ID, exact SHA-256, author/institution prologue evidence, classification,
  governing notice, conditions, and evidence references.

The elementary clearance relies on express public-domain statements by NIST
and NTIS, plus Netlib's Version 4.1 index that identifies the selected `src`
and `fnlib` units as SLATEC subsets. It is **not** inferred from hosting,
government sponsorship, or a laboratory name. Every accepted record is
hash-bound; an unknown ID, changed hash, incomplete accepted record, missing
evidence, or evidence whose source scope omits the record is rejected.

The classification vocabulary is finite:

- `us-government-public-domain`
- `explicit-public-domain`
- `explicit-permissive`
- `explicit-copyleft-compatible`
- `third-party-notice-required`
- `unresolved-authorship`
- `unresolved-rights`
- `excluded-from-bundle`

The first five may be accepted only with complete source-specific review data.
The other three cannot enter an archive. Current generated counts are 28
`explicit-public-domain`, 903 `unresolved-authorship`, and 356
`unresolved-rights`; unresolved units remain blocked even though one family is
now available.

## Reproducible build and carrier contents

Run the following only with the reviewed GNU MinGW 14.2.0 toolchain and an
explicit verified cache:

```text
set SLATEC_SOURCE_CACHE=<absolute verified cache path>
set SLATEC_GFORTRAN=<reviewed gfortran.exe>
cargo run -p slatec-tools --bin slatec-corpus -- build-bundled-provider --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-bundled-provider-evidence --offline
```

The command verifies every source hash before compiling it separately with
fixed Fortran flags, builds the archive with deterministic `ar crsD`, repeats
the build in the same root and compares SHA-256, records members/symbols,
builds a clean consumer with cache/system settings absent and an invalid
`SLATEC_GFORTRAN`, runs it, compares its rendered result with `source-build`,
and audits imported DLLs. It never performs network access.

The target carrier includes its family archive, the required static
`libgfortran` and `libquadmath` archives, checksums, source-unit manifest,
build recipe, compiler receipt, archive/runtime audits, SPDX SBOM, third-party
notices, and redistribution notice. `libquadmath` is included because the
reviewed final Rust GNU consumer link requires `quadmath_snprintf`, not because
the SLATEC archive references it. The recorded clean consumer imports no
`libgfortran` or `libquadmath` DLL.

The runtime archives retain their upstream obligations: `libgfortran` is
recorded as GPL-3.0 with GCC Runtime Library Exception; `libquadmath` is
recorded as LGPL-2.1-or-later. Notices and corresponding-source/relinking
information must travel with a distribution. This is a provenance record, not
legal advice.

Generated evidence lives in `generated/licensing/`, and the target carrier
contains the package-ready copies under `metadata/`. The carrier remains
`publish = false` until a separate publication review; no crate or tag is
published by the build workflow.
