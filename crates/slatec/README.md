# slatec

Safe, family-featured Rust APIs over validated original SLATEC Fortran routines.

The hosted `least-squares-nonlinear-easy` family provides residual-only,
finite-difference nonlinear least-squares fitting through the original
`SNLS1E` and `DNLS1E` easy drivers. The separate
`least-squares-nonlinear-expert` feature provides reviewed `SNLS1` and
`DNLS1` finite-difference and dense analytic-Jacobian modes. Both require
`std`, an explicit native backend, and the validated GNU MinGW profile;
the independent `least-squares-covariance` feature provides reviewed `SCOV`
and `DCOV` covariance estimation under the same hosted-profile requirement.

Native implementations are selected explicitly with `prebuilt`, `source-build`, `system`, or `external-backend`. No redistributable prebuilt provider is currently available because historical source rights remain unresolved.
