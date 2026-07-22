# SLATEC ODE-family audit

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Public drivers span DEPAC RKF/Adams/BDF, SDRIVE, and DASSL DAE families.
- **Reviewed sessions:** `SDRIV1`/`DDRIV1`, `SDRIV2`/`DDRIV2`, `SDRIV3`/`DDRIV3`, and `CDRIV1`/`CDRIV2` have checked owned sessions for their reviewed modes.  The expert real sessions are RHS-only; their documented mutable-`N` callback abort supports panic and user-error containment.
- `SDASSL`/`DDASSL` have checked residual-only owned sessions with a dense finite-difference iteration matrix.  Callers provide initially consistent `(y, y')`; analytic and banded Jacobian callbacks are deliberately outside this reviewed scope.
- DEPAC RKF/Adams drivers are deferred because their RHS callbacks have no documented native abort signal. DEBDF and `INTYD` use process-global COMMON history. `CDRIV3` remains unreviewed on the complex expert ABI.
- SDRIVE supports sign-change roots but no direction/terminal filtering; SDRIV3 event modes remain outside the RHS-only expert session.
