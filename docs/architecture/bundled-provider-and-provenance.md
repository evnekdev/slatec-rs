# Bundled provider and provenance gate

`slatec-src` has four mutually exclusive implementation-provider features:
`bundled`, `source-build`, `system`, and `external-backend`. `bundled` is the
canonical default feature in the pre-release API. It is target-specific and is
intended to let a normal consumer use a reviewed mathematical feature without a
Fortran compiler, source cache, system archive, environment variable, or build
script network access.

That intended experience is not currently available for a native family. The
carrier is deliberately metadata-only because the source-level provenance audit
has not admitted any historical source unit. This is a release gate, not a
fallback policy: selecting a native family with `bundled` fails before the build
script reads `SLATEC_SOURCE_CACHE`, probes GFortran, searches a system directory,
or invokes a network client.

## Provider modes

| Mode | Default feature | Compiler | Source cache | System archive | Status |
| --- | --- | --- | --- | --- | --- |
| `bundled` | Yes | Never | Never | Never | Target carrier architecture present; no historical archive until provenance clearance |
| `source-build` | No | GNU Fortran | Verified local cache | No | Explicit expert provider |
| `system` | No | No | No | Explicit archive and runtime directories | Explicit expert provider |
| `external-backend` | No | No | No | Caller-managed | Emits no link directives |

No mode silently selects another provider. The `prebuilt` feature was removed:
there is one canonical public name, `bundled`.

## Carrier layout

The first carrier package is
`slatec-bundled-x86_64-pc-windows-gnu`. It has no numerical Rust API. When the
gate is cleared it will contain a deterministic `libslatec_bundle.a`, its
checksum, source-unit manifest, member and symbol reports, compiler recipe,
runtime notices, and target metadata. Cargo selects the package only for its
target, so a future archive is not downloaded for unrelated targets.

The current generated carrier manifest is intentionally marked
`blocked_by_source_provenance`, has no archive checksum, and the package remains
`publish = false`. Package-content validation rejects an archive in that state.

## Provenance policy

The authoritative input for a clearance is
`crates/slatec-src/metadata/bundled-provenance-overrides.json`. Every record is
keyed by a stable selected-source ID and SHA-256. Generation rejects an unknown
source, an altered hash, and a classification outside the closed vocabulary:

- `us-government-public-domain`
- `explicit-public-domain`
- `permissive-license`
- `copyleft-license`
- `third-party-license`
- `government-provenance-no-explicit-notice`
- `unresolved-provenance`
- `excluded-from-bundle`

Only the first three classifications are currently accepted for a bundled
archive. Government sponsorship, an institutional name, and Netlib hosting are
not evidence of public-domain status. GNU compiler-runtime licensing is audited
separately from SLATEC source provenance.

Generated evidence lives in `generated/licensing/`:

- `slatec-source-provenance.json` and `.md` give one record per physical source;
- `bundled-source-eligibility.json` maps every provider family to its block;
- `unresolved-provenance.json`, `third-party-notices.md`, and
  `bundled-sbom.spdx.json` preserve the distribution boundary;
- `bundled-runtime-audit.json` and `bundled-archive-audit.json` explicitly say
  `not_applicable_no_redistributable_archive` until a real archive exists.

Regenerate and validate this evidence with:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-bundled-provider-evidence --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-bundled-provider-evidence --offline
```

`build-bundled-provider` deliberately fails before compilation while the gate
is blocked. This prevents an opaque local archive from becoming de facto
distribution input. Once every required source unit is cleared, the command must
be extended to compile one original source per object with the pinned toolchain,
normalize archive ordering and timestamps, verify symbols/imports/runtime
closure, and prove a clean-root deterministic archive hash before the carrier is
published.

## Link and runtime boundary

The existing source-build archive remains one object per selected original
source, with no whole-archive link. Its GNU runtime observations do not prove a
future bundled runtime closure. In particular, `libquadmath` must not be added
to a carrier merely because it exists in a compiler distribution: actual object
references and final imported DLLs must be recorded after a cleared archive is
built.
