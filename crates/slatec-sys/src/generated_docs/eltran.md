# Purpose

This subroutine is a translation of the ALGOL procedure ELMTRANS, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). This subroutine accumulates the stabilized elementary similarity transformations used in the reduction of a REAL GENERAL matrix to upper Hessenberg form by ELMHES.

# Description

This canonical unsafe binding exposes original SLATEC routine `ELTRAN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ELTRAN](https://www.netlib.org/slatec/lin/eltran.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 3. `LOW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  BALANC.  If  BALANC  has not been 1 and IGH equal to the order of the matrix, N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  BALANC.  If  BALANC  has not been not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the multipliers which were used in the reduction by  ELMHES  in its lower triangle below the subdiagonal. dimensional REAL array, dimensioned A(NM,IGH). dimensional REAL array, dimensioned A(NM,IGH). dimensional INTEGER array, dimensioned INT(IGH). dimensional REAL array, dimensioned contains the multipliers which were used in the reduction by  ELMHES  in its lower triangle below the subdiagonal. dimensional REAL array, dimensioned A(NM,IGH). dimensional REAL array, dimensioned A(NM,IGH). dimensional INTEGER array, dimensioned INT(IGH). dimensional REAL array, dimensioned not applicable or not stated by selected source not a workspace argument

## 6. `INT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). contains information on the rows and columns interchanged in the reduction by  ELMHES.  Only elements LOW through IGH dimensional INTEGER array, dimensioned INT(IGH). contains information on the rows and columns interchanged in the reduction by  ELMHES.  Only elements LOW through IGH dimensional INTEGER array, dimensioned INT(IGH). not applicable or not stated by selected source not a workspace argument

## 7. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the transformation matrix produced in the reduction dimensional REAL array, dimensioned Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the transformation matrix produced in the reduction dimensional REAL array, dimensioned Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `LOW`: not a workspace argument
- `IGH`: not a workspace argument
- `A`: not a workspace argument
- `INT`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::eltran`
- Original SLATEC routine: `ELTRAN`
- Native symbol: `eltran_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank2)`
- Exact Netlib source file: [ELTRAN](https://www.netlib.org/slatec/lin/eltran.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
