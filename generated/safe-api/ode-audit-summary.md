# SLATEC ODE-family audit

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Public drivers span DEPAC RKF/Adams/BDF, SDRIVE, and DASSL DAE families.
- **Selection:** `SDRIV3`/`DDRIV3`, restricted initially to real RHS-only IVPs in an owned non-cloneable session. Their documented mutable-`N` callback abort supports panic and user-error containment; caller work arrays hold continuation state and the executable driver has no COMMON or external I/O.
- DEPAC RKF/Adams drivers are deferred because their RHS callbacks have no documented native abort signal. DEBDF and `INTYD` use process-global COMMON history. DASSL is a distinct DAE/Jacobian/consistent-initial-state milestone.
- SDRIVE supports sign-change roots but no direction/terminal filtering; events remain deferred from the first wrapper scope.
- No public ODE feature, raw declaration, provider closure, native source, or translated algorithm is added.
