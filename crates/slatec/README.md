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
The distinct `least-squares-linear-nonnegative` feature wraps `WNNLS` and
`DWNNLS` for equality-constrained linear least squares with selected
nonnegative variables; it also requires `std`, an explicit backend, and the
validated GNU MinGW profile.
The separate `least-squares-linear-bounded` feature wraps `SBOLS` and `DBOLS`
for dense linear least squares with typed closed per-variable bounds under the
same hosted-profile requirement; it does not provide general equalities,
inequalities, or linear programming.
`least-squares-linear-bounded-constrained` separately wraps `SBOCLS` and
`DBOCLS`: it bounds original variables and the constraint expressions `C x`,
and therefore is neither a generic combination of the other drivers nor a
linear-programming interface.

The hosted `ode-sdrive-expert` feature provides owned real explicit-IVP
sessions over original `SDRIV3`/`DDRIV3`. Its first scope has only a
panic-contained RHS callback and same-direction continuation; event roots,
Jacobians, mass matrices, DAEs, and interpolation are deliberately deferred.

Native implementations are selected explicitly with `prebuilt`, `source-build`, `system`, or `external-backend`. No redistributable prebuilt provider is currently available because historical source rights remain unresolved.
