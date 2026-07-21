# Purpose

This subroutine is a companion routine to DNSQ and DNSQE. It may be used to check the coding of the Jacobian calculation. SUBROUTINE DCKDER This subroutine checks the gradients of M nonlinear functions in N variables, evaluated at a point X, for consistency with the functions themselves. The user must call DCKDER twice,

# Description

This canonical unsafe binding exposes original SLATEC routine `DCKDER`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCKDER](https://www.netlib.org/slatec/src/dckder.f).

# Arguments

## 1. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a positive integer input variable set to the number of functions. 2, 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. 2, 2, is a positive integer input variable set to the number of functions. 2, 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. 2, 2, not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a positive integer input variable set to the number of variables. 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. 1, is a positive integer input variable set to the number of variables. 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. 1, not applicable or not stated by selected source not a workspace argument

## 3. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). contain the functions evaluated at XP. On output, ERR contains measures of correctness of the respective gradients. The subroutine does not perform reliably if cancellation or rounding errors cause a severe loss of significance in the evaluation of a function. Therefore, none of the components of X should be unusually small (in particular, zero) or any other value which may cause loss of significance. The SUBROUTINE statement is SUBROUTINE DCKDER(M,N,X,FVEC,FJAC,LDFJAC,XP,FVECP,MODE,ERR) where is an input array of length N. contain the functions evaluated at XP. On output, ERR contains measures of correctness of the respective gradients. The subroutine does not perform reliably if cancellation or rounding errors cause a severe loss of significance in the evaluation of a function. Therefore, none of the components of X should be unusually small (in particular, zero) or any other value which may cause loss of significance. The SUBROUTINE statement is SUBROUTINE DCKDER(M,N,X,FVEC,FJAC,LDFJAC,XP,FVECP,MODE,ERR) where is an input array of length N. not applicable or not stated by selected source not a workspace argument

## 4. `FVEC`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). 2, must contain the functions evaluated at X. 2, must contain the functions evaluated at X. not applicable or not stated by selected source not a workspace argument

## 5. `FJAC`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDFJAC, *). 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. not applicable or not stated by selected source not a workspace argument

## 6. `LDFJAC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a positive integer input parameter not less than M which specifies the leading dimension of the array FJAC. is a positive integer input parameter not less than M which specifies the leading dimension of the array FJAC. is a positive integer input parameter not less than M which specifies the leading dimension of the array FJAC. not a workspace argument

## 7. `XP`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). 1, is set to a neighboring point of X. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `FVECP`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). contain the functions evaluated at XP. On output, ERR contains measures of correctness of the respective gradients. The subroutine does not perform reliably if cancellation or rounding errors cause a severe loss of significance in the evaluation of a function. Therefore, none of the components of X should be unusually small (in particular, zero) or any other value which may cause loss of significance. The SUBROUTINE statement is SUBROUTINE DCKDER(M,N,X,FVEC,FJAC,LDFJAC,XP,FVECP,MODE,ERR) where 2, must contain the functions evaluated at XP. contain the functions evaluated at XP. On output, ERR contains measures of correctness of the respective gradients. The subroutine does not perform reliably if cancellation or rounding errors cause a severe loss of significance in the evaluation of a function. Therefore, none of the components of X should be unusually small (in particular, zero) or any other value which may cause loss of significance. The SUBROUTINE statement is SUBROUTINE DCKDER(M,N,X,FVEC,FJAC,LDFJAC,XP,FVECP,MODE,ERR) where 2, must contain the functions evaluated at XP. not applicable or not stated by selected source not a workspace argument

## 9. `MODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 and then with MODE = 2. 1. On input, X must contain the point of evaluation. On output, XP is set to a neighboring point. 2. On input, FVEC must contain the functions and the rows of FJAC must contain the gradients of the respective functions each evaluated 2, 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. 1, 2, is an integer input variable set to 1 on the first call and 2 on the second. Other values of MODE are equivalent 1. 2, 1 and then with MODE = 2. 1. On input, X must contain the point of evaluation. On output, XP is set to a neighboring point. 2. On input, FVEC must contain the functions and the rows of FJAC must contain the gradients of the respective functions each evaluated 2, 2, the rows of FJAC must contain the gradients of the respective functions evaluated at X. 1, 2, is an integer input variable set to 1 on the first call and 2 on the second. Other values of MODE are equivalent 1. 2, not applicable or not stated by selected source not a workspace argument

## 10. `ERR`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). 2, contains measures of correctness of the respective gradients. If there is no severe loss of significance, th gradient is correct, th gradient is incorrect. For values of ERR between 0.0 and 1.0, the categorization is less certain. In general, a value of ERR(I) greater than 0.5 indicates that the I-th gradient is probably correct, while a value of ERR(I) less than 0.5 indicates that the I-th gradient is probably incorrect. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `M`: not a workspace argument
- `N`: not a workspace argument
- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `FJAC`: not a workspace argument
- `LDFJAC`: not a workspace argument
- `XP`: not a workspace argument
- `FVECP`: not a workspace argument
- `MODE`: not a workspace argument
- `ERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::nonlinear::jacobian_check::dckder`
- Original SLATEC routine: `DCKDER`
- Native symbol: `dckder_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DCKDER](https://www.netlib.org/slatec/src/dckder.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
