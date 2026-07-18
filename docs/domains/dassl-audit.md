# DASSL audit

The selected SLATEC snapshot provides real `SDASSL` and `DDASSL` drivers for
implicit residual systems `G(t,y,y')=0`. The initial safe family fixes DASSL
to requested-output mode, caller-supplied initial derivatives, dense storage,
and internal finite-difference iteration matrices.

The residual ABI is `RES(T,Y,YPRIME,DELTA,IRES,RPAR,IPAR)`. DASSL accepts
`IRES=-1` for a recoverable invalid iterate and returns `IDID=-11` after a
fatal `IRES=-2`. The disabled Jacobian ABI is
`JAC(T,Y,YPRIME,PD,CJ,RPAR,IPAR)`; the safe wrapper supplies an ABI-compatible
trap and treats any unexpected call as a native contract violation.

`INFO(2)` supports scalar and vector tolerance pairs. Exposed controls map to
reviewed stop-time, initial-step, maximum-step, and maximum-order positions;
raw `INFO` words, user Jacobians, banded mode, and initial-derivative
calculation remain private or deferred.

The closure has no retained external-file protocol. It is nevertheless
`SerializedGlobal`: `SDAINI`/`DDAINI` and `SDASTP`/`DDASTP` contain DATA
initialized saved state, XERROR is process-global, and residual dispatch is
call-scoped. A global lock prevents races; it does not make DASSL reentrant.
