# Purpose

These subroutines solve the least squares problem Ax = b for banded matrices A using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted. These subroutines are intended for the type of least squares systems that arise in applications such as curve or surface fitting of data. The least squares equations are accumulated and processed using only part of the data. This requires a certain user interaction during the solution of Ax = b. Specifically, suppose the data matrix (A B) is row partitioned into Q submatrices. Let (E F) be the T-th one of these submatrices where E = (0 C 0). Here the dimension of E is MT by N and the dimension of C is MT by NB. The value of NB is the bandwidth of A. The dimensions of the leading block of zeros in E

# Description

This canonical unsafe binding exposes original SLATEC routine `BNDACC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BNDACC](https://www.netlib.org/slatec/src/bndacc.f).

# Arguments

## 1. `G`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (MDG, *). Description of calling sequence for BNDACC.. The entire set of parameters for BNDACC are The working array into which the user will place the MT by NB+1 block (C F) in rows IR should be .GE. MU. The value of MU is defined in the abstract of these subprograms. The working array which will contain the processed rows of that part of the data matrix which has been passed to BNDACC. X(N) The entire set of parameters for BNDSOL are These arguments all have the same meaning and 0.E0 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `MDG`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Description of calling sequence for BNDACC.. The entire set of parameters for BNDACC are The number of rows in the working array should be .GE. MU. The value of MU is defined in the abstract of these subprograms. is considered an error. X(N) The entire set of parameters for BNDSOL are These arguments all have the same meaning and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `NB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An acceptable value for MU is MU = MAX(MT + N + 1), where N is the number of unknowns. Description of calling sequence for BNDACC.. The entire set of parameters for BNDACC are The bandwidth of the data matrix A. X(N) The entire set of parameters for BNDSOL are contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. DO 10 J=1, NBP1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Set by the user to the value 1 before the first call to BNDACC.  Its subsequent value is controlled by BNDACC to set up for the next call to BNDACC. are advanced by BNDACC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for BNDSOL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. Set by the user to the value 1 before the first call to BNDACC.  Its subsequent value is controlled by BNDACC to set up for the next call to BNDACC. are advanced by BNDACC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for BNDSOL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. not applicable or not stated by selected source not a workspace argument

## 5. `IR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, columns 1 through NB+1. See descriptions of IR and MT below. Index of the row of G(*,*) where the user is to place the new block of data (C F).  Set by the user to the value 1 before the first call to BNDACC.  Its subsequent value is controlled is considered an error. are advanced by BNDACC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for BNDSOL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. 0.E0 1, columns 1 through NB+1. See descriptions of IR and MT below. Index of the row of G(*,*) where the user is to place the new block of data (C F).  Set by the user to the value 1 before the first call to BNDACC.  Its subsequent value is controlled is considered an error. are advanced by BNDACC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for BNDSOL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to BNDACC. X(*)              With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N                 The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine BNDSOL prints an appropriate message.  This condition is considered an error. 0.E0 not applicable or not stated by selected source not a workspace argument

## 6. `MT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1. The user of the subroutine BNDACC provides MT,JT,C and F for T=1,...,Q.  Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB.  The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL BNDSOL(1,...). The sequence of values for JT must be nondecreasing.  This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by 1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2.  The subprogram BNDACC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL BNDACC(...)  Introduce new blocks of data. CALL BNDSOL(1,...)Compute solution vector and length of residual vector. CALL BNDSOL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL BNDSOL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list.. 1, columns 1 through NB+1. See descriptions of IR and MT below. Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. 1 1. The user of the subroutine BNDACC provides MT,JT,C and F for T=1,...,Q.  Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB.  The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL BNDSOL(1,...). The sequence of values for JT must be nondecreasing.  This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by 1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2.  The subprogram BNDACC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL BNDACC(...)  Introduce new blocks of data. CALL BNDSOL(1,...)Compute solution vector and length of residual vector. CALL BNDSOL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL BNDSOL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list.. 1, columns 1 through NB+1. See descriptions of IR and MT below. Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. 1 not applicable or not stated by selected source not a workspace argument

## 7. `JT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1. The user of the subroutine BNDACC provides MT,JT,C and F for T=1,...,Q.  Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB.  The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL BNDSOL(1,...). The sequence of values for JT must be nondecreasing.  This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. N+1 CALL BNDACC(G,MDG,NB,IP,IR,MT,JT) 1. The user of the subroutine BNDACC provides MT,JT,C and F for T=1,...,Q.  Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB.  The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL BNDSOL(1,...). The sequence of values for JT must be nondecreasing.  This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. N+1 CALL BNDACC(G,MDG,NB,IP,IR,MT,JT) not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `G`: not a workspace argument
- `MDG`: not a workspace argument
- `NB`: not a workspace argument
- `IP`: not a workspace argument
- `IR`: not a workspace argument
- `MT`: not a workspace argument
- `JT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::bndacc`
- Original SLATEC routine: `BNDACC`
- Native symbol: `bndacc_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [BNDACC](https://www.netlib.org/slatec/src/bndacc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
