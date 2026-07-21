# Purpose

Subroutine POISTG solves the linear system of equations

# Description

This canonical unsafe binding exposes original SLATEC routine `POISTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POISTG](https://www.netlib.org/slatec/fishfft/poistg.f).

# Arguments

## 1. `NPEROD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the values which X(I,0) and X(I,N+1) are assumed to have. = 1 If X(I,0) = -X(I,1) and X(I,N+1) = -X(I,N) = 2 If X(I,0) = -X(I,1) and X(I,N+1) =  X(I,N) = 3 If X(I,0) =  X(I,1) and X(I,N+1) =  X(I,N) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. X(I,N) The number of unknowns in the J-direction.  N must be greater than 2. are listed are listed in the table below.  More comprehensive timing in the table below.  More comprehensive timing charts may be found in the reference. charts may be found in the reference. To measure the accuracy of the algorithm a To measure the accuracy of the algorithm a uniform random number generator was used to create uniform random number generator was used to create X(I,N) The number of unknowns in the J-direction.  N must be greater than 2. are listed are listed in the table below.  More comprehensive timing in the table below.  More comprehensive timing charts may be found in the reference. charts may be found in the reference. To measure the accuracy of the algorithm a To measure the accuracy of the algorithm a uniform random number generator was used to create uniform random number generator was used to create not applicable or not stated by selected source not a workspace argument

## 3. `MPEROD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 0 the array elements must not depend on the index I, but must be constant. Specifically, the subroutine checks the following condition 1 0 the array elements must not depend on the index I, but must be constant. Specifically, the subroutine checks the following condition 1 not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are not zero = 1 If A(1) = C(M) = 0 The number of unknowns in the I-direction.  M must be greater than 2. are listed in the table below.  More comprehensive timing charts may be found in the reference. To measure the accuracy of the algorithm a uniform random number generator was used to create 1. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POISTG was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of and N. N)    MPEROD    NPEROD    T(MSECS)    E are not zero = 1 If A(1) = C(M) = 0 The number of unknowns in the I-direction.  M must be greater than 2. are listed in the table below.  More comprehensive timing charts may be found in the reference. To measure the accuracy of the algorithm a uniform random number generator was used to create 1. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POISTG was called to produce an approximate solution Z.  Then the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of and N. N)    MPEROD    NPEROD    T(MSECS)    E not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I=1,2,...,M and J=1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e. X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to X(I,1) or -X(I,1) and X(I,N+1) may be equal to X(I,N) or -X(I,N) depending on an input parameter. * * * * * * *    Parameter Description     * * * * * * * * * * are not zero = 1 If A(1) = C(M) = 0 One-dimensional arrays of length M that specify the coefficients C(1) dimensional array that specifies the values of the right side of the linear system of equations given above. dimensional work array that must be provided by the user for work space.  W may require up to 9M + 4N + M(INT(log2(N))) locations.  The actual number of locations used is computed by POISTG and returned in location W(1). .NE. C(1) or B(I) .NE. B(1) or C(I) .NE. C(1) for some I = 1, 2, ..., M. = 7 If MPEROD .EQ. 1 .AND. (A(1).NE.0 .OR. C(M).NE.0) solution array X for the system given in the 'PURPOSE ' with 0.5*B(I) = 1,       I=1,2,...,M C(M) = 0 1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I=1,2,...,M and J=1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e. X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to X(I,1) or -X(I,1) and X(I,N+1) may be equal to X(I,N) or -X(I,N) depending on an input parameter. * * * * * * *    Parameter Description     * * * * * * * * * * are not zero = 1 If A(1) = C(M) = 0 One-dimensional arrays of length M that specify the coefficients C(1) dimensional array that specifies the values of the right side of the linear system of equations given above. dimensional work array that must be provided by the user for work space.  W may require up to 9M + 4N + M(INT(log2(N))) locations.  The actual number of locations used is computed by POISTG and returned in location W(1). .NE. C(1) or B(I) .NE. B(1) or C(I) .NE. C(1) for some I = 1, 2, ..., M. = 7 If MPEROD .EQ. 1 .AND. (A(1).NE.0 .OR. C(M).NE.0) solution array X for the system given in the 'PURPOSE ' with 0.5*B(I) = 1,       I=1,2,...,M C(M) = 0 not applicable or not stated by selected source

## 6. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). One-dimensional arrays of length M that specify the coefficients B(1) 1. 1. The solution X was substituted into the given sys- The solution X was substituted into the given sys- tem and, using double precision, a right side Y was tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POISTG was computed.  Using this array Y subroutine POISTG was called to produce an approximate solution Z.  Then called to produce an approximate solution Z.  Then the relative error, defined as the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed.  The value of E is and J=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of given in the table below for some typical values of One-dimensional arrays of length M that specify the coefficients B(1) 1. 1. The solution X was substituted into the given sys- The solution X was substituted into the given sys- tem and, using double precision, a right side Y was tem and, using double precision, a right side Y was computed.  Using this array Y subroutine POISTG was computed.  Using this array Y subroutine POISTG was called to produce an approximate solution Z.  Then called to produce an approximate solution Z.  Then the relative error, defined as the relative error, defined as E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) E = MAX(ABS(Z(I,J)-X(I,J)))/MAX(ABS(X(I,J))) where the two maxima are taken over all I=1,2,...,M where the two maxima are taken over all I=1,2,...,M and J=1,2,...,N, was computed.  The value of E is and J=1,2,...,N, was computed.  The value of E is given in the table below for some typical values of given in the table below for some typical values of not applicable or not stated by selected source not a workspace argument

## 7. `C`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are not zero = 1 If A(1) = C(M) = 0 One-dimensional arrays of length M that specify the coefficients C(1) for I = 1, 2, ..., M. 0.5*B(I) = 1,       I=1,2,...,M are not zero = 1 If A(1) = C(M) = 0 One-dimensional arrays of length M that specify the coefficients C(1) for I = 1, 2, ..., M. 0.5*B(I) = 1,       I=1,2,...,M not applicable or not stated by selected source not a workspace argument

## 8. `IDIMY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The row (or first) dimension of the two-dimensional array Y as it appears in the program calling POISTG.  This parameter is must be at least M. The row (or first) dimension of the two-dimensional array Y as it appears in the program calling POISTG.  This parameter is must be at least M. The row (or first) dimension of the two-dimensional array Y as it appears in the program calling POISTG.  This parameter is must be at least M. not a workspace argument

## 9. `Y`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (IDIMY, *). must be at least M. must be dimensioned at least M X N. Contains the solution X. must be at least M. must be dimensioned at least M X N. Contains the solution X. not applicable or not stated by selected source not a workspace argument

## 10. `IERROR`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An error flag that indicates invalid input parameters.  Except for number zero, a solution is not attempted. = 0  No error = 1  If M .LE. 2 = 2  If N .LE. 2 = 3  IDIMY .LT. M = 4  If NPEROD .LT. 1 or NPEROD .GT. 4 = 5  If MPEROD .LT. 0 or MPEROD .GT. 1 = 6  If MPEROD = 0 and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(M),B(M),C(M),Y(IDIMY,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    POISTG,POSTG2,COSGEN,MERGE,TRIX,TRI3,PIMACH Required Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(M),B(M),C(M),Y(IDIMY,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    POISTG,POSTG2,COSGEN,MERGE,TRIX,TRI3,PIMACH Required Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NPEROD`: not a workspace argument
- `N`: not a workspace argument
- `MPEROD`: not a workspace argument
- `M`: not a workspace argument
- `A`: 1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I=1,2,...,M and J=1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e. X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to X(I,1) or -X(I,1) and X(I,N+1) may be equal to X(I,N) or -X(I,N) depending on an input parameter. * * * * * * *    Parameter Description     * * * * * * * * * * are not zero = 1 If A(1) = C(M) = 0 One-dimensional arrays of length M that specify the coefficients C(1) dimensional array that specifies the values of the right side of the linear system of equations given above. dimensional work array that must be provided by the user for work space.  W may require up to 9M + 4N + M(INT(log2(N))) locations.  The actual number of locations used is computed by POISTG and returned in location W(1). .NE. C(1) or B(I) .NE. B(1) or C(I) .NE. C(1) for some I = 1, 2, ..., M. = 7 If MPEROD .EQ. 1 .AND. (A(1).NE.0 .OR. C(M).NE.0) solution array X for the system given in the 'PURPOSE ' with 0.5*B(I) = 1,       I=1,2,...,M C(M) = 0
- `B`: not a workspace argument
- `C`: not a workspace argument
- `IDIMY`: not a workspace argument
- `Y`: not a workspace argument
- `IERROR`: not a workspace argument
- `W`: contains the required length of W. Long Description: * * * * * *   Program Specifications    * * * * * * * * * * * * Dimension of   A(M),B(M),C(M),Y(IDIMY,N), Arguments      W(see argument list) Latest         June 1, 1977 Revision Subprograms    POISTG,POSTG2,COSGEN,MERGE,TRIX,TRI3,PIMACH Required Special        NONE Conditions Common         NONE Blocks I/O            NONE Precision      Single Specialist     Roland Sweet Language       FORTRAN

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::poistg`
- Original SLATEC routine: `POISTG`
- Native symbol: `poistg_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [POISTG](https://www.netlib.org/slatec/fishfft/poistg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
