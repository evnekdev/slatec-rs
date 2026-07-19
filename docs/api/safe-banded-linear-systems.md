# Safe general banded linear systems

`slatec::linear_algebra::banded` provides `f32` and `f64` LU factorization and reusable direct or transpose solves over LINPACK `SGBFA`/`DGBFA` and `SGBSL`/`DGBSL`. It also provides factorization plus a reciprocal 1-norm condition estimate through `SGBCO`/`DGBCO`, and a scaled determinant through `SGBDI`/`DGBDI`.

Input is compact column-major band storage: logical `A[i,j]` is stored at row `ku+i-j` of column `j`; out-of-band entries are structural zero. The borrowed view uses `kl+ku+1` rows, while private factor storage has `2*kl+ku+1` rows to accommodate pivot fill-in. No dense or packed matrix type is introduced.

`BandMatrixRef::factorize_with_condition_estimate` performs a fresh private factorization because `SGBCO`/`DGBCO` calculate the original matrix 1-norm before overwriting the expanded matrix with LU factors. Its `ReciprocalCondition::value` is an estimate of `1 / (||A||_1 ||A^-1||_1)`, not the condition number. An exact zero pivot remains `BandError::Singular`; a zero successful estimate can instead indicate numerical singularity or estimate underflow and must not be inverted automatically.

`BandLu32::scaled_determinant` and `BandLu64::scaled_determinant` read, but do not consume or mutate, existing compatible factors. They return `ScaledDeterminant`, representing `det(A) = mantissa * 10^exponent10`. Nonzero native mantissas have absolute value in `[1, 10)`; for a zero mantissa the exponent is mathematically irrelevant. The scaled pair is primary so callers do not accidentally overflow or underflow a direct value.

The source closure uses the canonical provider IDs for shared BLAS sources. The prior duplicate-path defect came from re-listing those source paths under distinct `ffi-source-*` IDs; provider-manifest validation now rejects such ambiguity and the banded overlay reuses the canonical selected-source records. Focused source inspection of the `*GB*` and BLAS closure found no COMMON, SAVE, DATA, BLOCK DATA, XERROR, callback, or Fortran I/O path. Native calls remain process-globally serialized under the hosted runtime policy because backend/runtime parallel qualification is intentionally not claimed.
