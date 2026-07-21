# Purpose

Subroutine EZFFTF computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined

# Description

This canonical unsafe binding exposes original SLATEC routine `EZFFTF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EZFFTF](https://www.netlib.org/slatec/fishfft/ezfftf.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the array R to be transformed.  The method is most efficient when N is the product of small primes. contains the sequence to be transformed.  R is not destroyed. 0. and A(N/2) is the sum from I=1 to 0. and A(N/2) is the sum from I=1 to 1)**(I-1)*R(I)/N 1 1 1)/2 1)/2 then for  K=1,...,KMAX then for  K=1,...,KMAX 1)*2*PI/N) 1)*2*PI/N) the length of the array R to be transformed.  The method is most efficient when N is the product of small primes. contains the sequence to be transformed.  R is not destroyed. 0. and A(N/2) is the sum from I=1 to 0. and A(N/2) is the sum from I=1 to 1)**(I-1)*R(I)/N 1 1 1)/2 1)/2 then for  K=1,...,KMAX then for  K=1,...,KMAX 1)*2*PI/N) 1)*2*PI/N) not applicable or not stated by selected source not a workspace argument

## 2. `R`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the sequence to be transformed.  R is not destroyed. 1)*2*PI/N) 1)*2*PI/N) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `AZERO`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a simplified but slower version of RFFTF. 1 to I=N of R(I)/N not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a simplified but slower version of RFFTF. contains the sequence to be transformed.  R is not destroyed. must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. 0. and A(N/2) is the sum from I=1 to 1 to I=N of changing dummy array size declarations (1) to (*), is a simplified but slower version of RFFTF. contains the sequence to be transformed.  R is not destroyed. must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. 0. and A(N/2) is the sum from I=1 to 1 to I=N of changing dummy array size declarations (1) to (*), not applicable or not stated by selected source not a workspace argument

## 5. `B`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a simplified but slower version of RFFTF. 0. and A(N/2) is the sum from I=1 to 0. and A(N/2) is the sum from I=1 to 1 to I=N of changing references to intrinsic function FLOAT to REAL. 881128  Modified by Dick Valent to meet prologue standards. 890531  Changed all specific intrinsics to generic.  (WRB) 890531  REVISION DATE from Version 3.2 891214  Prologue converted to Version 4.0 format.  (BAB) 920501  Reformatted the REFERENCES section.  (WRB) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `WSAVE`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `R`: not a workspace argument
- `AZERO`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `WSAVE`: must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::ezfftf`
- Original SLATEC routine: `EZFFTF`
- Native symbol: `ezfftf_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [EZFFTF](https://www.netlib.org/slatec/fishfft/ezfftf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
