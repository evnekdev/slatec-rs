# slatec

Safe, family-featured Rust APIs over validated original SLATEC Fortran routines.

The hosted `least-squares-nonlinear-easy` family provides residual-only,
finite-difference nonlinear least-squares fitting through the original
`SNLS1E` and `DNLS1E` easy drivers. It requires `std`, an explicit native
backend, and the validated GNU MinGW profile; expert least-squares and
covariance interfaces remain deferred.

Native implementations are selected explicitly with `prebuilt`, `source-build`, `system`, or `external-backend`. No redistributable prebuilt provider is currently available because historical source rights remain unresolved.
