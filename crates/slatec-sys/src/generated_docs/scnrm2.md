# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SCNRM2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SCNRM2](https://www.netlib.org/slatec/lin/scnrm2.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) vector stored in CX with storage increment INCX. 0. must be .GE. 1 Four phase method using two built-in constants that are hopefully applicable to all machines. CUTLO = maximum of  SQRT(U/EPS)  over all known machines. CUTHI = minimum of  SQRT(V)      over all known machines. where EPS = smallest no. such that EPS + 1. .GT. 1. U   = smallest positive no.   (underflow limit) V   = largest  no.            (overflow  limit) Brief outline of algorithm. Phase 1 scans zero components. Move to phase 2 when a component is nonzero and .LE. CUTLO Move to phase 3 when a component is .GT. CUTLO Move to phase 4 when a component is .GE. CUTHI/M where M = N for X() real and M = 2*N for complex. Values for CUTLO and CUTHI. From the environmental parameters listed in the IMSL converter document the limiting values are as follows: CUTLO, S.P.   U/EPS = 2**(-102) for  Honeywell.  Close seconds are Univac and DEC at 2**(-103) Thus CUTLO = 2**(-51) = 4.44089E-16 CUTHI, S.P.   V = 2**127 for Univac, Honeywell, and DEC. Thus CUTHI = 2**(63.5) = 1.30438E19 CUTLO, D.P.   U/EPS = 2**(-67) for Honeywell and DEC. Thus CUTLO = 2**(-33.5) = 8.23181D-11 CUTHI, D.P.   same as S.P.  CUTHI = 1.30438D19 DATA CUTLO, CUTHI /8.232D-11,  1.304D19/ DATA CUTLO, CUTHI /4.441E-16,  1.304E19/ number of elements in input vector(s) vector stored in CX with storage increment INCX. 0. must be .GE. 1 Four phase method using two built-in constants that are hopefully applicable to all machines. CUTLO = maximum of  SQRT(U/EPS)  over all known machines. CUTHI = minimum of  SQRT(V)      over all known machines. where EPS = smallest no. such that EPS + 1. .GT. 1. U   = smallest positive no.   (underflow limit) V   = largest  no.            (overflow  limit) Brief outline of algorithm. Phase 1 scans zero components. Move to phase 2 when a component is nonzero and .LE. CUTLO Move to phase 3 when a component is .GT. CUTLO Move to phase 4 when a component is .GE. CUTHI/M where M = N for X() real and M = 2*N for complex. Values for CUTLO and CUTHI. From the environmental parameters listed in the IMSL converter document the limiting values are as follows: CUTLO, S.P.   U/EPS = 2**(-102) for  Honeywell.  Close seconds are Univac and DEC at 2**(-103) Thus CUTLO = 2**(-51) = 4.44089E-16 CUTHI, S.P.   V = 2**127 for Univac, Honeywell, and DEC. Thus CUTHI = 2**(63.5) = 1.30438E19 CUTLO, D.P.   U/EPS = 2**(-67) for Honeywell and DEC. Thus CUTLO = 2**(-33.5) = 8.23181D-11 CUTHI, D.P.   same as S.P.  CUTHI = 1.30438D19 DATA CUTLO, CUTHI /8.232D-11,  1.304D19/ DATA CUTLO, CUTHI /4.441E-16,  1.304E19/ not applicable or not stated by selected source not a workspace argument

## 2. `CX`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). complex vector with N elements not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of CX must be .GE. 1 Four phase method using two built-in constants that are hopefully applicable to all machines. CUTLO = maximum of  SQRT(U/EPS)  over all known machines. CUTHI = minimum of  SQRT(V)      over all known machines. where EPS = smallest no. such that EPS + 1. .GT. 1. U   = smallest positive no.   (underflow limit) V   = largest  no.            (overflow  limit) Brief outline of algorithm. Phase 1 scans zero components. Move to phase 2 when a component is nonzero and .LE. CUTLO Move to phase 3 when a component is .GT. CUTLO Move to phase 4 when a component is .GE. CUTHI/M where M = N for X() real and M = 2*N for complex. Values for CUTLO and CUTHI. From the environmental parameters listed in the IMSL converter document the limiting values are as follows: CUTLO, S.P.   U/EPS = 2**(-102) for  Honeywell.  Close seconds are Univac and DEC at 2**(-103) Thus CUTLO = 2**(-51) = 4.44089E-16 CUTHI, S.P.   V = 2**127 for Univac, Honeywell, and DEC. Thus CUTHI = 2**(63.5) = 1.30438E19 CUTLO, D.P.   U/EPS = 2**(-67) for Honeywell and DEC. Thus CUTLO = 2**(-33.5) = 8.23181D-11 CUTHI, D.P.   same as S.P.  CUTHI = 1.30438D19 DATA CUTLO, CUTHI /8.232D-11,  1.304D19/ DATA CUTLO, CUTHI /4.441E-16,  1.304E19/ storage spacing between elements of CX must be .GE. 1 Four phase method using two built-in constants that are hopefully applicable to all machines. CUTLO = maximum of  SQRT(U/EPS)  over all known machines. CUTHI = minimum of  SQRT(V)      over all known machines. where EPS = smallest no. such that EPS + 1. .GT. 1. U   = smallest positive no.   (underflow limit) V   = largest  no.            (overflow  limit) Brief outline of algorithm. Phase 1 scans zero components. Move to phase 2 when a component is nonzero and .LE. CUTLO Move to phase 3 when a component is .GT. CUTLO Move to phase 4 when a component is .GE. CUTHI/M where M = N for X() real and M = 2*N for complex. Values for CUTLO and CUTHI. From the environmental parameters listed in the IMSL converter document the limiting values are as follows: CUTLO, S.P.   U/EPS = 2**(-102) for  Honeywell.  Close seconds are Univac and DEC at 2**(-103) Thus CUTLO = 2**(-51) = 4.44089E-16 CUTHI, S.P.   V = 2**127 for Univac, Honeywell, and DEC. Thus CUTHI = 2**(63.5) = 1.30438E19 CUTLO, D.P.   U/EPS = 2**(-67) for Honeywell and DEC. Thus CUTLO = 2**(-33.5) = 8.23181D-11 CUTHI, D.P.   same as S.P.  CUTHI = 1.30438D19 DATA CUTLO, CUTHI /8.232D-11,  1.304D19/ DATA CUTLO, CUTHI /4.441E-16,  1.304E19/ not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `CX`: not a workspace argument
- `INCX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::scnrm2`
- Original SLATEC routine: `SCNRM2`
- Native symbol: `scnrm2_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SCNRM2](https://www.netlib.org/slatec/lin/scnrm2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
