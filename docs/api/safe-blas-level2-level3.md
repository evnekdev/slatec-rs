# Safe BLAS Levels 2 and 3 API

`slatec::blas::level2` and `slatec::blas::level3` are explicit, slice-based
facades over the selected original SLATEC BLAS routines. They require the
`blas-level2` or `blas-level3` feature and the existing
`ffi-profile-gnu-mingw-x86_64` native profile. The default `bundled` provider
performs verified source acquisition, compilation, and static linking;
`source-build`, `system`, and `external-backend` are explicit alternatives.

Matrices are ordinary column-major backing slices. The element at `row`, `col`
is `a[row + col * lda]`. Dimensions and leading dimensions are explicit in the
core APIs; no custom matrix/view type is introduced and row-major data is never
reinterpreted or copied. `lda` may exceed the logical row count.

The initial real-valued subset provides f32/f64 `GEMV`, `GER`, `SYMV`, `TRMV`,
`TRSV`, `GEMM`, `TRMM`, `TRSM`, and `SYRK`. `GEMV` and `GEMM` also provide
`*_contiguous` helpers, which infer only unit vector increments and tightly
packed column-major leading dimensions. They never infer matrix dimensions.

```rust,no_run
use slatec::blas::level2::dgemv_contiguous;
use slatec::blas::level2::dtrsv;
use slatec::blas::level3::dgemm_contiguous;
use slatec::blas::{Diagonal, Transpose, Triangle};

let a = [1.0, 2.0, 3.0, 4.0]; // [[1, 3], [2, 4]] in column-major order
let mut y = [0.0, 0.0];
dgemv_contiguous(Transpose::None, 2, 2, 1.0, &a, &[1.0, 1.0], 0.0, &mut y)?;

let mut c = [0.0; 4];
dgemm_contiguous(Transpose::None, Transpose::None, 2, 2, 2, 1.0, &a, &a, 0.0, &mut c)?;

let triangular = [1.0, 0.0, 2.0, 3.0];
let mut rhs = [5.0, 6.0];
dtrsv(Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, &triangular, 2, &mut rhs, 1)?;
# Ok::<(), slatec::blas::BlasError>(())
```

`Transpose`, `Triangle`, `Diagonal`, and `Side` replace raw Fortran selector
characters. For this real-only API, `Transpose::ConjugateTranspose` is rejected
rather than silently normalized. Matrix and vector spans, leading dimensions,
increments, and every Fortran `INTEGER` conversion use checked arithmetic.

Complex Level 2/3 routines, including `zgemm`, remain deferred. Level 1 did
not establish a safe complex value/conversion policy, so exposing `CGEMM` or
`ZGEMM` now would create a second policy and violate the safe-API boundary.
Consequently, no safe `zgemm` example exists in this milestone. The raw ABI
remains available through `slatec-sys` for qualified callers.

`TRMV` and `TRSV` overwrite their vector; `TRMM` and `TRSM` overwrite `B`.
For symmetric routines only the selected triangle is read or updated. This
subset does not expose Hermitian operations, so it makes no claim about the
special BLAS convention for imaginary diagonal elements.
