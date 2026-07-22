# slatec-src

`no_std` implementation-provider selection and native link integration for
`slatec-rs`. Applications that need an implementation provider use this crate;
declaration-only users can depend on `slatec-sys` directly.

`bundled` is the canonical ordinary-provider feature. Its target-specific
carrier does not currently contain a historical SLATEC archive: the generated
source-level provenance audit blocks distribution until every selected source
has accepted, hash-guarded evidence. A bundled native-family request therefore
fails before touching a compiler, source cache, system directory, or network.
`source-build` consumes a separately acquired, checksum-verified cache and
never accesses the network. `system` links an explicitly located archive, while
`external-backend` emits no link directives.

For the supported GNU MinGW source backend, every selected original Fortran
source is compiled separately and retained as a separate member in a normal
static archive. No partial link, generated amalgamation, or whole-archive link
is used. A broad family can therefore produce a large intermediate archive
while a final executable extracts only referenced member closures. This is
measured by the deterministic reports under `generated/native-link/`; an
external or system provider is not assumed to share those properties. See
[`docs/architecture/native-link-granularity.md`](../../docs/architecture/native-link-granularity.md).
Safe-wrapper module layout is audited separately and does not modify this
provider's one-source-per-object archive construction; see
[`docs/architecture/safe-facade-link-granularity.md`](../../docs/architecture/safe-facade-link-granularity.md).

This crate owns the workspace's Cargo `links = "slatec"` namespace. It never
downloads source during a build. The strongest validated source-build profile
is GNU MinGW on `x86_64-pc-windows-gnu`; system and external backends must supply
an independently compatible ABI. Packages contain closure metadata and the
reviewed machine-constant overrides, not the separately acquired source cache.
