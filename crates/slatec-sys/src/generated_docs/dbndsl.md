# Purpose

These subroutines solve the least squares problem Ax = b for banded matrices A using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted. These subroutines are intended for the type of least squares systems that arise in applications such as curve or surface fitting of data. The least squares equations are accumulated and processed using only part of the data. This requires a certain user interaction during the solution of Ax = b. Specifically, suppose the data matrix (A B) is row partitioned into Q submatrices. Let (E F) be the T-th one of these submatrices where E = (0 C 0). Here the dimension of E is MT by N and the dimension of C is MT by NB. The value of NB is the bandwidth of A. The dimensions of the leading block of zeros in E are MT by JT-1. The user of the subroutine DBNDAC provides MT,JT,C and F for T=1,...,Q. Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB. The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL DBNDSL(1,...). The sequence of values for JT must be nondecreasing. This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by

# Description

This canonical unsafe binding exposes original SLATEC routine `DBNDSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBNDSL](https://www.netlib.org/slatec/src/dbndsl.f).

# Arguments

## 1. `MODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Set by the user to one of the values 1, 2, or 3.  These values respectively indicate that the solution of AX = B, YR = H or RZ = W is required. 2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. 1 RNORM is the Euclidean length of the residual vector AX-B.  When MODE=2 or 3 RNORM is set to zero. Remarks.. To obtain the upper triangular matrix and transformed right-hand side vector D so that the super diagonals of R form the columns of G(*,*), execute the following Fortran statements. Set by the user to one of the values 1, 2, or 3.  These values respectively indicate that the solution of AX = B, YR = H or RZ = W is required. 2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. 1 RNORM is the Euclidean length of the residual vector AX-B.  When MODE=2 or 3 RNORM is set to zero. Remarks.. To obtain the upper triangular matrix and transformed right-hand side vector D so that the super diagonals of R form the columns of G(*,*), execute the following Fortran statements. not applicable or not stated by selected source not a workspace argument

## 2. `G`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (MDG, *). Description of calling sequence for DBNDAC.. The entire set of parameters for DBNDAC are Input.. All Type REAL variables are DOUBLE PRECISION The working array into which the user will place the MT by NB+1 block (C F) in rows IR should be .GE. MU. The value of MU is defined in the abstract of these subprograms. The working array which will contain the processed rows of that part of the data matrix which has been passed to DBNDAC. X(N) The entire set of parameters for DBNDSL are These arguments all have the same meaning and 0.E0 MT=1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `MDG`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Description of calling sequence for DBNDAC.. The entire set of parameters for DBNDAC are Input.. All Type REAL variables are DOUBLE PRECISION The number of rows in the working array should be .GE. MU. The value of MU is defined in the abstract of these subprograms. is considered an error. MT,JT             Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. Output.. All Type REAL variables are DOUBLE PRECISION X(N) The entire set of parameters for DBNDSL are These arguments all have the same meaning and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An acceptable value for MU is MU = MAX(MT + N + 1), where N is the number of unknowns. Here the maximum is taken over all values of MT for T=1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2.  The subprogram DBNDAC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL DBNDAC(...)  Introduce new blocks of data. CALL DBNDSL(1,...)Compute solution vector and length of residual vector. CALL DBNDSL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL DBNDSL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list.. Description of calling sequence for DBNDAC.. The entire set of parameters for DBNDAC are Input.. All Type REAL variables are DOUBLE PRECISION The bandwidth of the data matrix A. X(N) The entire set of parameters for DBNDSL are contents as following the last call to DBNDAC. DO 10 J=1, NBP1 An acceptable value for MU is MU = MAX(MT + N + 1), where N is the number of unknowns. Here the maximum is taken over all values of MT for T=1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2.  The subprogram DBNDAC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL DBNDAC(...)  Introduce new blocks of data. CALL DBNDSL(1,...)Compute solution vector and length of residual vector. CALL DBNDSL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL DBNDSL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list.. Description of calling sequence for DBNDAC.. The entire set of parameters for DBNDAC are Input.. All Type REAL variables are DOUBLE PRECISION The bandwidth of the data matrix A. X(N) The entire set of parameters for DBNDSL are contents as following the last call to DBNDAC. DO 10 J=1, NBP1 not applicable or not stated by selected source not a workspace argument

## 5. `IP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Set by the user to the value 1 before the first call to DBNDAC.  Its subsequent value is controlled by DBNDAC to set up for the next call to DBNDAC. are advanced by DBNDAC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for DBNDSL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to DBNDAC. Set by the user to the value 1 before the first call to DBNDAC.  Its subsequent value is controlled by DBNDAC to set up for the next call to DBNDAC. are advanced by DBNDAC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for DBNDSL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to DBNDAC. not applicable or not stated by selected source not a workspace argument

## 6. `IR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, columns 1 through NB+1. See descriptions of IR and MT below. Index of the row of G(*,*) where the user is the user to the value 1 before the first call to DBNDAC.  Its subsequent value is controlled is considered an error. MT,JT             Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. Output.. All Type REAL variables are DOUBLE PRECISION are advanced by DBNDAC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for DBNDSL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to DBNDAC. 0.E0 MT=1 1, columns 1 through NB+1. See descriptions of IR and MT below. Index of the row of G(*,*) where the user is the user to the value 1 before the first call to DBNDAC.  Its subsequent value is controlled is considered an error. MT,JT             Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. Output.. All Type REAL variables are DOUBLE PRECISION are advanced by DBNDAC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for DBNDSL.. The user must dimension the arrays appearing in the call list.. contents as following the last call to DBNDAC. 0.E0 MT=1 not applicable or not stated by selected source not a workspace argument

## 7. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). 2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. This array contains the solution vectors X, Y or Z of the systems AX = B, YR = H or RZ = W depending on the value of MODE=1, 2 or 3. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The number of variables in the solution vector.  If any of the N diagonal terms are zero the subroutine DBNDSL prints an appropriate message.  This condition is considered an error. CALL DBNDAC(G,MDG,NB,IP,IR,MT,JT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RNORM`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1 RNORM is the Euclidean length of the residual vector AX-B.  When MODE=2 or 3 RNORM is set to zero. Remarks.. To obtain the upper triangular matrix and transformed right-hand side vector D so that the super diagonals of R form the columns of G(*,*), execute the following Fortran statements. 1 RNORM is the Euclidean length of the residual vector AX-B.  When MODE=2 or 3 RNORM is set to zero. Remarks.. To obtain the upper triangular matrix and transformed right-hand side vector D so that the super diagonals of R form the columns of G(*,*), execute the following Fortran statements. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `MODE`: not a workspace argument
- `G`: not a workspace argument
- `MDG`: not a workspace argument
- `NB`: not a workspace argument
- `IP`: not a workspace argument
- `IR`: not a workspace argument
- `X`: not a workspace argument
- `N`: not a workspace argument
- `RNORM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dbndsl`
- Original SLATEC routine: `DBNDSL`
- Native symbol: `dbndsl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_f64)`
- Exact Netlib source file: [DBNDSL](https://www.netlib.org/slatec/src/dbndsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
