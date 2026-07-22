# Safe residual-only DASSL sessions

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Drivers: `SDASSL` (`f32`) and `DDASSL` (`f64`) for real index-1 `G(t,y,y')=0` problems.
- Scope: owned continuation sessions, scalar/vector tolerances, internally differenced dense or banded iteration matrices, and requested-output advancement only. The caller provides an initially consistent pair. Root finding, consistent-initial-condition calculation, and complex drivers remain deferred.
- Callback: `IRES=0` continues, `-1` requests documented recoverable residual failure, and `-2` ends the native operation. Rust errors, panics, malformed calls, and non-finite residuals are contained without unwinding across Fortran. The optional analytic `JAC` callback receives `CJ` but has no source-defined abort channel, so it remains deliberately unexposed.
- Workspace: dense `RWORK=40+(MAXORD+4)*NEQ+NEQ^2`; banded `RWORK=40+(MAXORD+4)*NEQ+(2*ML+MU+1)*NEQ+2*(NEQ/(ML+MU+1)+1)`; `IWORK=20+NEQ`; `MAXORD` is 1 through 5.
- Runtime: all calls remain `SerializedGlobal`; `SDAINI/DDAINI` and `SDASTP/DDASTP` use saved DATA state and reachable XERROR is process-global. The focused provider closure includes the safe-facade `XGETF/XSETF` calls and has no DASSL external-file protocol.
- Native-origin audit: focused DASSL closure inspection is complete; broad audit retry status is reported independently and never weakens serialization.
