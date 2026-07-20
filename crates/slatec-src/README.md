# slatec-src

`no_std` implementation-provider selection for `slatec-rs`.

`source-build` consumes a separately acquired, checksum-verified cache and never accesses the network. `system` links an explicitly located archive, while `external-backend` emits no link directives. `prebuilt` is intentionally unavailable until redistribution rights are resolved.

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
