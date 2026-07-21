# Purpose

This subroutine is a translation of the ALGOL procedure CBALANCE, which is a complex version of BALANCE, NUM. MATH. 13, 293-304(1969) by Parlett and Reinsch. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 315-326(1971). This subroutine balances a COMPLEX matrix and isolates eigenvalues whenever possible.

# Description

This canonical unsafe binding exposes original SLATEC routine `CBAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBAL](https://www.netlib.org/slatec/lin/cbal.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, AR and AI, as declared in the calling program dimension statement.  NM is an INTEGER variable. and AI(NM,N). dimensional array parameters, AR and AI, as declared in the calling program dimension statement.  NM is an INTEGER variable. and AI(NM,N). not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (AR,AI).  N is an INTEGER variable.  N must be less than or equal to NM. and AI(NM,N). (AR,AI).  N is an INTEGER variable.  N must be less than or equal to NM. and AI(NM,N). not applicable or not stated by selected source not a workspace argument

## 3. `AR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex matrix to be balanced. dimensional REAL arrays, dimensioned and AI(NM,N). contain the real and imaginary parts, respectively, of the balanced matrix. contain the real and imaginary parts, respectively, of the complex matrix to be balanced. dimensional REAL arrays, dimensioned and AI(NM,N). contain the real and imaginary parts, respectively, of the balanced matrix. not applicable or not stated by selected source not a workspace argument

## 4. `AI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex matrix to be balanced. dimensional REAL arrays, dimensioned contain the real and imaginary parts, respectively, of the balanced matrix. contain the real and imaginary parts, respectively, of the complex matrix to be balanced. dimensional REAL arrays, dimensioned contain the real and imaginary parts, respectively, of the balanced matrix. not applicable or not stated by selected source not a workspace argument

## 5. `LOW`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables such that AR(I,J) and AI(I,J) are equal to zero if (1) I is greater than J and 1 or I=IGH+1,...,N. 1 = D(J,J)       J = LOW,...,IGH = P(J)         J = IGH+1,...,N. The order in which the interchanges are made is N to IGH+1, 1. Note that 1 is returned for IGH if IGH is zero formally. The ALGOL procedure EXC contained in CBALANCE appears in CBAL  in line.  (Note that the ALGOL roles of identifiers K,L have been reversed.) Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IGH`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables such that AR(I,J) and AI(I,J) are equal to zero if (1) I is greater than J and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `SCALE`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains information determining the permutations and dimensional REAL array, dimensioned SCALE(N). Suppose that the principal submatrix in rows LOW through IGH has been balanced, that P(J) denotes the index interchanged with J during the permutation step, and that the elements of the diagonal matrix used are denoted by D(I,J).  Then 1 = D(J,J)       J = LOW,...,IGH = P(J)         J = IGH+1,...,N. The order in which the interchanges are made is N to IGH+1, contains information determining the permutations and dimensional REAL array, dimensioned SCALE(N). Suppose that the principal submatrix in rows LOW through IGH has been balanced, that P(J) denotes the index interchanged with J during the permutation step, and that the elements of the diagonal matrix used are denoted by D(I,J).  Then 1 = D(J,J)       J = LOW,...,IGH = P(J)         J = IGH+1,...,N. The order in which the interchanges are made is N to IGH+1, not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `LOW`: not a workspace argument
- `IGH`: not a workspace argument
- `SCALE`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cbal`
- Original SLATEC routine: `CBAL`
- Native symbol: `cbal_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [CBAL](https://www.netlib.org/slatec/lin/cbal.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
