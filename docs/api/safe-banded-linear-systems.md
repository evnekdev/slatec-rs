# Safe general banded linear systems

`slatec::linear_algebra::banded` provides `f32` and `f64` LU factorization and reusable direct or transpose solves over LINPACK `SGBFA`/`DGBFA` and `SGBSL`/`DGBSL`.

Input is compact column-major band storage: logical `A[i,j]` is stored at row `ku+i-j` of column `j`; out-of-band entries are structural zero. The borrowed view uses `kl+ku+1` rows, while private factor storage has `2*kl+ku+1` rows to accommodate pivot fill-in. No dense or packed matrix type is introduced.

Condition estimation (`SGBCO`/`DGBCO`) and scaled determinants (`SGBDI`/`DGBDI`) are deferred from the first narrow closure. Native calls are process-globally serialized under the hosted runtime policy.
