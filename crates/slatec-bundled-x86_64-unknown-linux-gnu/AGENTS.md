# Linux bundled carrier instructions

Scope: `crates/slatec-bundled-x86_64-unknown-linux-gnu/**`.

This carrier owns only hash-verified Linux native archives and generated
metadata. It exposes no numerical Rust API. Never hand-edit files under
`metadata/` or `native/`; update `slatec-tools` inputs and regenerate them in
the reviewed WSL GNU Fortran environment.
