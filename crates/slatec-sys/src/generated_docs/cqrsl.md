# Purpose

CQRSL applies the output of CQRDC to compute coordinate transformations, projections, and least squares solutions. For K .LE. MIN(N,P), let XK be the matrix XK = (X(JVPT(1)),X(JVPT(2)), ... ,X(JVPT(K))) formed from columns JVPT(1), ... ,JVPT(K) of the original

# Description

This canonical unsafe binding exposes original SLATEC routine `CQRSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CQRSL](https://www.netlib.org/slatec/lin/cqrsl.f).

# Arguments

## 1. `X`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDX, *). and QRAUX. On Entry COMPLEX(LDX,P). contains the output of CQRDC. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array X. INTEGER. is the leading dimension of the array X. INTEGER. is the leading dimension of the array X. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. x P matrix X that was input to CQRDC (if no pivoting was done, XK consists of the first K columns of X in their original order).  CQRDC produces a factored unitary matrix Q and an upper triangular matrix R such that XK = Q * (R) (0) This information is contained in coded form in the arrays INTEGER. is the number of rows of the matrix XK.  It must have the same value as N in CQRDC. vector that is to be manipulated by CQRSL. x P matrix X that was input to CQRDC (if no pivoting was done, XK consists of the first K columns of X in their original order).  CQRDC produces a factored unitary matrix Q and an upper triangular matrix R such that XK = Q * (R) (0) This information is contained in coded form in the arrays INTEGER. is the number of rows of the matrix XK.  It must have the same value as N in CQRDC. vector that is to be manipulated by CQRSL. not applicable or not stated by selected source not a workspace argument

## 4. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the number of columns of the matrix XK.  K must not be greater than (N,P), where P is the same as in the calling sequence to CQRDC. INTEGER. is the number of columns of the matrix XK.  K must not be greater than (N,P), where P is the same as in the calling sequence to CQRDC. not applicable or not stated by selected source not a workspace argument

## 5. `QRAUX`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(P). contains the auxiliary output from CQRDC. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `Y`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) vector that is to be manipulated by CQRSL. XK*B), if its computation has been requested.  (Note that if pivoting was requested in CQRDC, the J-th component of B will be associated with column JVPT(J) of the original matrix X that was input into CQRDC.) XK*B, if its computation has been requested.  RSD is also the orthogonal projection of Y onto the orthogonal complement of the column space of XK. COMPLEX(N) vector that is to be manipulated by CQRSL. XK*B), if its computation has been requested.  (Note that if pivoting was requested in CQRDC, the J-th component of B will be associated with column JVPT(J) of the original matrix X that was input into CQRDC.) XK*B, if its computation has been requested.  RSD is also the orthogonal projection of Y onto the orthogonal complement of the column space of XK. not applicable or not stated by selected source not a workspace argument

## 7. `QY`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N). contains Q*Y, if its computation has been requested. are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence.  A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY.  In this case one may identify Y, QTY, and one of B, RSD, or XB, while providing separate arrays for anything else that is to be computed.  Thus the calling sequence CALL CQRSL(X,LDX,N,K,QRAUX,Y,DUM,Y,B,Y,DUM,110,INFO) will result in the computation of B and RSD, with RSD overwriting Y.  More generally, each item in the following list contains groups of permissible identifications for a single calling sequence. 1. (Y,QTY,B) (RSD) (XB) (QY) 2. (Y,QTY,RSD) (B) (XB) (QY) 3. (Y,QTY,XB) (B) (RSD) (QY) 4. (Y,QY) (QTY,B) (RSD) (XB) 5. (Y,QY) (QTY,RSD) (B) (XB) 6. (Y,QY) (QTY,XB) (B) (RSD) In any group the value returned in the array allocated to the group corresponds to the last member of the group. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `QTY`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N). contains CTRANS(Q)*Y, if its computation has been requested.  Here CTRANS(Q) is the conjugate transpose of the matrix Q. are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence.  A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY.  In this case one may identify Y, QTY, and one of B, RSD, or XB, while providing separate arrays for anything else that is to be computed.  Thus the calling sequence CALL CQRSL(X,LDX,N,K,QRAUX,Y,DUM,Y,B,Y,DUM,110,INFO) will result in the computation of B and RSD, with RSD overwriting Y.  More generally, each item in the following list contains groups of permissible identifications for a single calling sequence. 1. (Y,QTY,B) (RSD) (XB) (QY) 2. (Y,QTY,RSD) (B) (XB) (QY) 3. (Y,QTY,XB) (B) (RSD) (QY) 4. (Y,QY) (QTY,B) (RSD) (XB) 5. (Y,QY) (QTY,RSD) (B) (XB) 6. (Y,QY) (QTY,XB) (B) (RSD) In any group the value returned in the array allocated to the group corresponds to the last member of the group. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `B`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(K) contains the solution of the least squares problem are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence.  A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY.  In this case one may identify Y, QTY, and one of B, RSD, or XB, while providing separate arrays for anything else that is to be computed.  Thus the calling sequence CALL CQRSL(X,LDX,N,K,QRAUX,Y,DUM,Y,B,Y,DUM,110,INFO) will result in the computation of B and RSD, with RSD overwriting Y.  More generally, each item in the following list contains groups of permissible identifications for a single calling sequence. 1. (Y,QTY,B) (RSD) (XB) (QY) 2. (Y,QTY,RSD) (B) (XB) (QY) 3. (Y,QTY,XB) (B) (RSD) (QY) 4. (Y,QY) (QTY,B) (RSD) (XB) 5. (Y,QY) (QTY,RSD) (B) (XB) 6. (Y,QY) (QTY,XB) (B) (RSD) In any group the value returned in the array allocated to the group corresponds to the last member of the group. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `RSD`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N). XK*B, if its computation has been requested.  RSD is also the orthogonal projection of Y onto the orthogonal complement of the column space of XK. are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence.  A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY.  In this case one may identify Y, QTY, and one of B, RSD, or XB, while providing separate arrays for anything else that is to be computed.  Thus the calling sequence CALL CQRSL(X,LDX,N,K,QRAUX,Y,DUM,Y,B,Y,DUM,110,INFO) will result in the computation of B and RSD, with RSD overwriting Y.  More generally, each item in the following list contains groups of permissible identifications for a single calling sequence. 1. (Y,QTY,B) (RSD) (XB) (QY) 2. (Y,QTY,RSD) (B) (XB) (QY) 3. (Y,QTY,XB) (B) (RSD) (QY) 4. (Y,QY) (QTY,B) (RSD) (XB) 5. (Y,QY) (QTY,RSD) (B) (XB) 6. (Y,QY) (QTY,XB) (B) (RSD) In any group the value returned in the array allocated to the group corresponds to the last member of the group. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `XB`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N). contains the least squares approximation XK*B, if its computation has been requested.  XB is also the orthogonal projection of Y onto the column space of X. are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence.  A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY.  In this case one may identify Y, QTY, and one of B, RSD, or XB, while providing separate arrays for anything else that is to be computed.  Thus the calling sequence CALL CQRSL(X,LDX,N,K,QRAUX,Y,DUM,Y,B,Y,DUM,110,INFO) will result in the computation of B and RSD, with RSD overwriting Y.  More generally, each item in the following list contains groups of permissible identifications for a single calling sequence. 1. (Y,QTY,B) (RSD) (XB) (QY) 2. (Y,QTY,RSD) (B) (XB) (QY) 3. (Y,QTY,XB) (B) (RSD) (QY) 4. (Y,QY) (QTY,B) (RSD) (XB) 5. (Y,QY) (QTY,RSD) (B) (XB) 6. (Y,QY) (QTY,XB) (B) (RSD) In any group the value returned in the array allocated to the group corresponds to the last member of the group. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. specifies what is to be computed.  JOB has the decimal expansion ABCDE, with the following meaning. If A .NE. 0, compute QY. If B,C,D, or E .NE. 0, compute QTY. If C .NE. 0, compute B. If D .NE. 0, compute RSD . If E .NE. 0, compute  XB. Note that a request to compute B, RSD, or XB automatically triggers the computation of QTY, for which an array must be provided in the calling sequence. On Return INTEGER. specifies what is to be computed.  JOB has the decimal expansion ABCDE, with the following meaning. If A .NE. 0, compute QY. If B,C,D, or E .NE. 0, compute QTY. If C .NE. 0, compute B. If D .NE. 0, compute RSD . If E .NE. 0, compute  XB. Note that a request to compute B, RSD, or XB automatically triggers the computation of QTY, for which an array must be provided in the calling sequence. On Return not applicable or not stated by selected source not a workspace argument

## 13. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is zero unless the computation of B has been requested and R is exactly singular.  In this case, INFO is the index of the first zero diagonal element of R and B is left unaltered. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `LDX`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `QRAUX`: not a workspace argument
- `Y`: not a workspace argument
- `QY`: not a workspace argument
- `QTY`: not a workspace argument
- `B`: not a workspace argument
- `RSD`: not a workspace argument
- `XB`: not a workspace argument
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cqrsl`
- Original SLATEC routine: `CQRSL`
- Native symbol: `cqrsl_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CQRSL](https://www.netlib.org/slatec/lin/cqrsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
