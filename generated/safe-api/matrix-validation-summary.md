# BLAS Level 2 and 3 safe API validation summary

- ABI profile: `ffi-profile-gnu-mingw-x86_64`
- Core representation: ordinary column-major slices with explicit dimensions and leading dimensions
- Exposed real wrappers: 18 raw-symbol wrappers plus 4 tightly packed GEMV/GEMM conveniences
- Checked selectors: transpose, triangle, diagonal, side
- Complex policy: deferred because Level 1 has no safe complex representation policy
- Native and differential tests: authored and profile-gated; pending verified local native evidence

No custom matrix type, source text, object file, static library, or compiler log is committed.
