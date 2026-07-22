# Safe callback-driver extension plan

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Scope: `DPFQAD`, `SOS`/`DSOS`, `SDRIV1`/`DDRIV1`, `SDRIV2`/`DDRIV2`, and `CDRIV1`/`CDRIV2`.
- Existing callback storage: private lexical TLS in `callback_runtime` plus the current SDRIV3/DDRIV3 ODE context. The implementation extends the shared guard rather than adding a public trampoline framework.
- Runtime: all native callback calls remain process-serialized; `catch_unwind` records callback panic in scoped state and no panic crosses Fortran.
- Nesting: active callback scopes reject another callback-bearing SLATEC operation, including across ODE and non-ODE families.
- Workspaces: safe APIs allocate them; DRIV continuation sessions own and preserve opaque state across same-direction calls.
- Public vocabulary: preserve `Integration*`, `Nonlinear*`, and `Ode*` naming and add family-local report/options only where the raw driver has distinct controls.
- Features: extend `quadrature`, `nonlinear`, and `ode` subfeatures with exact declaration/provider closures; no milestone feature is exposed.
- Out of scope: `CDRIV3`, `DBVSUP`, `DFQAD`, and `DVSUP`.
