# SLATEC ODE-family audit

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Public drivers span DEPAC RKF/Adams/BDF, SDRIVE, and DASSL DAE families.
- **Reviewed sessions:** `SDRIV1`/`DDRIV1`, `SDRIV2`/`DDRIV2`, `SDRIV3`/`DDRIV3`, and `CDRIV1`/`CDRIV2` have checked owned sessions for their reviewed modes. The expert real sessions support functional iteration, internal dense/banded finite differences, and checked analytic dense/banded `JACOBN`; both native callbacks document mutable-`N` abort for contained Rust failures.
- `SDASSL`/`DDASSL` have checked residual-only owned sessions with internal dense or banded finite-difference iteration matrices. Callers provide initially consistent `(y, y')`. The DASSL analytic `JAC` callback has no source-defined termination channel, so it remains outside the safe API.
- DEPAC RKF/Adams drivers are deferred because their RHS callbacks have no documented native abort signal. DEBDF and `INTYD` use process-global COMMON history. `CDRIV3` remains unreviewed on the complex expert ABI.
- SDRIVE supports sign-change roots but no direction/terminal filtering; mass-matrix and event modes remain outside the reviewed expert session.
