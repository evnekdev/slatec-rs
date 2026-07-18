# PCHIP audit

The selected Netlib PCHIP package contains 41 single- and double-precision
program units. The safe surface is deliberately small and coherent: PCHIM
and DPCHIM for default monotone derivatives; PCHIC and DPCHIC for typed
endpoint/switch control; PCHSP and DPCHSP for the package's cubic-spline
Hermite derivative representation; PCHFE/DPCHFE and PCHFD/DPCHFD for values
and first derivatives; and PCHIA/DPCHIA for arbitrary-limit definite
integration.

The supporting CHFDV, CHFEV, CHFIE, PCHID, PCHDF, PCHCI, PCHCS, PCHCE,
PCHSW, and sign helpers are linkage-only. PCHBS, PCHCM, PCHKT, PCHDOC and
their available double counterparts remain inventoried but deferred: they do
not add a distinct safe operation to the constructed Hermite curve API.

PCHFE and PCHFD accept unordered evaluation points, though increasing points
are faster. Both extrapolate with the nearest endpoint cubic and report a
positive count. PCHIA permits arbitrary limits, uses the same endpoint cubic
outside the data domain, reports lower/upper extrapolation with bit-coded
positive statuses, and preserves the sign for reversed limits.

Every exposed native root has DATA/SAVE storage in its transitive closure or
can reach process-global XERROR. There is no callback or Fortran-file protocol
in this closure, but PCHIP remains globally serialized because locking avoids
races around shared state; it does not change native reentrancy. The generated
source-closure, native-state, status, endpoint, and extrapolation records are
the concise machine-readable audit evidence.
