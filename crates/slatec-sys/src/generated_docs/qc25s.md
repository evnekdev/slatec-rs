# Purpose

Integration rules for integrands having ALGEBRAICO-LOGARITHMIC end point singularities Standard fortran subroutine Real version

# Description

This canonical unsafe binding exposes original SLATEC routine `QC25S`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QC25S](https://www.netlib.org/slatec/src/qc25s.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Function subprogram defining the integrand F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

a part of (A,B). Left end point of the original interval.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

a part of (A,B). Right end point of the original interval, B. GT. A.

## `BL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Lower limit of integration, BL. GE. A.

## `BR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Upper limit of integration, BR. LE. B.

## `ALFA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

PARAMETER IN THE WEIGHT FUNCTION.

## `BETA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Parameter in the weight function.

## `RI`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (25).

Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO).

## `RJ`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (25).

Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO).

## `RG`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (25).

Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO).

## `RH`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (25).

Modified CHEBYSHEV moments for the application of the generalized CLENSHAW-CURTIS method (computed in subroutine DQMOMO).

## `RESULT`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral is computed by using a generalized CLENSHAW-CURTIS method if B1 = A or BR = B. in all other cases the 15-POINT KRONROD RULE is applied, obtained by optimal addition of Abscissae to the 7-POINT GAUSS RULE.

## `ABSERR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT).

## `RESASC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Approximation to the integral of ABS(F*W-I/(B-A)).

## `INTEGR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Which determines the weight function = 1 W(X) = (X-A)**ALFA*(B-X)**BETA = 2 W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3 W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4 W(X) = (X-A)**ALFA*(B-X)**BETA*LOG(X-A)*.

## `NEV`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of integrand evaluations.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Workspace and array requirements

- `RI`: not a workspace argument
- `RJ`: not a workspace argument
- `RG`: not a workspace argument
- `RH`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qc25s`
- Original SLATEC routine: `QC25S`
- Native symbol: `qc25s_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_f32,mut_i32,mut_i32)`
- Exact Netlib source file: [QC25S](https://www.netlib.org/slatec/src/qc25s.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
