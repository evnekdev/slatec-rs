# nonlinear-optimization safe-coverage disposition

This generated inventory joins canonical raw status with raw-to-safe coverage. `expert-raw-only` and blocked records are explicit decisions, not missing data. Call-graph fields are recorded as unavailable where the committed authoritative inputs do not contain a complete Fortran call graph.

## Disposition counts

- `blocked-by-abi`: 2
- `direct-safe-wrapper`: 10
- `expert-raw-only`: 70

## Routine records

| Routine | Role | Raw path | Safe path | Provider feature | Docs | Link | Runtime | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `CHKDER` | historically_user_callable_driver | `slatec_sys::nonlinear::jacobian_check::chkder` | `slatec::nonlinear::check_jacobian_f32` | `nonlinear-jacobian` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `CPEVL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `CPEVLR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `D1MPYQ` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `D1UPDT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DCKDER` | historically_user_callable_driver | `slatec_sys::nonlinear::jacobian_check::dckder` | `slatec::nonlinear::check_jacobian` | `nonlinear-jacobian` | `complete_generated_abi_contract` | `passed` | `not_required_batch_a` | `direct-safe-wrapper` |
| `DDOGLG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DENORM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFDJC1` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DFULMT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DNSQ` | historically_user_callable_driver | `slatec_sys::nonlinear::dnsq` | `slatec::nonlinear::solve_system_expert; slatec::nonlinear::solve_system_with_jacobian` | `nonlinear-expert` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DNSQE` | historically_user_callable_driver | `slatec_sys::nonlinear::dnsqe` | `slatec::nonlinear::solve_system` | `nonlinear-easy` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DOGLEG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPCHNG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPINCW` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPINIT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPINTM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPCE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPDM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPFE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPFL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `blocked-by-abi` |
| `DPLPMU` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPLPUP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPNNZR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPOPT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPRWPG` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DPRWVR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DQFORM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DREADP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DSOS` | historically_user_callable_driver | `slatec_sys::nonlinear::systems::dsos` | `slatec::nonlinear::solve_scalar_equations` | `nonlinear-systems` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DSOSEQ` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DSOSSL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DSPLP` | historically_user_callable_driver | `slatec_sys::linear_programming::dsplp` | `slatec::linear_programming::LinearProgram::<f64>::solve` | `optimization-linear-programming-in-memory` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `DUSRMT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `DVOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `DWRITP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FDJAC1` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `FULMAT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `IDLOC` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `IPLOC` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `IVOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `LA05AD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05AS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05BD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05BS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05CD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05CS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05ED` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `LA05ES` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `MC20AD` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `MC20AS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PCHNGS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PINITM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PNNZRS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PRWPGE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `PRWVIR` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `QFORM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `R1MPYQ` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `R1UPDT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SCLOSM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SNSQ` | historically_user_callable_driver | `slatec_sys::nonlinear::snsq` | `slatec::nonlinear::solve_system_expert_f32; slatec::nonlinear::solve_system_with_jacobian_f32` | `nonlinear-expert` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SNSQE` | historically_user_callable_driver | `slatec_sys::nonlinear::snsqe` | `slatec::nonlinear::solve_system_f32` | `nonlinear-easy` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SOPENM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SOS` | historically_user_callable_driver | `slatec_sys::nonlinear::systems::sos` | `slatec::nonlinear::solve_scalar_equations_f32` | `nonlinear-systems` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SOSEQS` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SOSSOL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPINCW` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPINIT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLP` | historically_user_callable_driver | `slatec_sys::linear_programming::splp` | `slatec::linear_programming::LinearProgram::<f32>::solve` | `optimization-linear-programming-in-memory` | `complete_authored` | `passed` | `passed` | `direct-safe-wrapper` |
| `SPLPCE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPDM` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPFE` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPFL` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPMN` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `blocked-by-abi` |
| `SPLPMU` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPLPUP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SPOPT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SREADP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `SVOUT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `not_tested` | `not_tested` | `expert-raw-only` |
| `SWRITP` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
| `USRMAT` | internal_subsidiary | `not_promoted` | `none` | `not_assigned` | `not_documented` | `passed` | `passed` | `expert-raw-only` |
