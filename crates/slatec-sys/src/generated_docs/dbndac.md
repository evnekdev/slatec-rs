# Purpose

These subroutines solve the least squares problem Ax = b for banded matrices A using sequential accumulation of rows of the data matrix. Exactly one right-hand side vector is permitted. These subroutines are intended for the type of least squares systems that arise in applications such as curve or surface fitting of data. The least squares equations are accumulated and processed using only part of the data. This requires a certain user interaction during the solution of Ax = b. Specifically, suppose the data matrix (A B) is row partitioned into Q submatrices. Let (E F) be the T-th one of these submatrices where E = (0 C 0). Here the dimension of E is MT by N and the dimension of C is MT by NB. The value of NB is the bandwidth of A. The dimensions of the leading block of zeros in E are MT by JT-1. The user of the subroutine DBNDAC provides MT,JT,C and F for T=1,...,Q. Not all of this data must be supplied at once. Following the processing of the various blocks (E F), the matrix (A B) has been transformed to the form (R D) where R is upper triangular and banded with bandwidth NB. The least squares system Rx = d is then easily solved using back substitution by executing the statement CALL DBNDSL(1,...). The sequence of values for JT must be nondecreasing. This may require some preliminary interchanges of rows and columns of the matrix A. The primary reason for these subroutines is that the total processing can take place in a working array of dimension MU by NB+1. An acceptable value for MU is MU = MAX(MT + N + 1), where N is the number of unknowns. Here the maximum is taken over all values of MT for T=1,...,Q. Notice that MT can be taken to be a small as one, showing that MU can be as small as N+2. The subprogram DBNDAC processes the rows more efficiently if MU is large enough so that each new block (C F) has a distinct value of JT. The four principle parts of these algorithms are obtained by the following call statements CALL DBNDAC(...) Introduce new blocks of data. CALL DBNDSL(1,...)Compute solution vector and length of residual vector. CALL DBNDSL(2,...)Given any row vector H solve YR = H for the row vector Y. CALL DBNDSL(3,...)Given any column vector W solve RZ = W for the column vector Z. The dots in the above call statements indicate additional arguments that will be specified in the following paragraphs. The user must dimension the array appearing in the call list..

# Description

This canonical unsafe binding exposes original SLATEC routine `DBNDAC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBNDAC](https://www.netlib.org/slatec/src/dbndac.f).

# Arguments

## `G`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (MDG, *).

Description of calling sequence for DBNDAC. The entire set of parameters for DBNDAC are The working array into which the user will place the MT by NB+1 block (C F) in rows IR through IR+MT-1, columns 1 through NB+1. See descriptions of IR and MT below. The value of MDG should be. GE. MU.

## `MDG`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of rows in the working array These arguments all have the same meaning and.

## `NB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The bandwidth of the data matrix A. contents as following the last call to DBNDAC. X(*) With mode=2 or 3 this array contains, respectively, the right-side vectors H or W of the systems YR = H or RZ = W. N The number of variables in the solution vector. If any of the N diagonal terms are zero the subroutine DBNDSL prints an appropriate message. This condition is considered an error.

## `IP`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Set by the user to the value 1 before the first call to DBNDAC. Its subsequent value is controlled by DBNDAC to set up for the next call to DBNDAC. The values of these arguments are advanced by DBNDAC to be ready for storing and processing a new block of data in G(*,*). Description of calling sequence for DBNDSL. The user must dimension the arrays appearing in the call list. G(MDG,NB+1), X(N) The entire set of parameters for DBNDSL are contents as following the last call to DBNDAC.

## `IR`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Index of the row of G(*,*) where the user is to place the new block of data (C F). Set by the user to the value 1 before the first call to DBNDAC. Its subsequent value is controlled by DBNDAC. A value of IR. GT. MDG is considered an error.

## `MT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed.

## `JT`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Set by the user to indicate respectively the number of new rows of data in the block and the index of the first nonzero column in that set of rows (E F) = (0 C 0 F) being processed. N+1 CALL DBNDAC(G,MDG,NB,IP,IR,MT,JT).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `G`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dbndac`
- Original SLATEC routine: `DBNDAC`
- Native symbol: `dbndac_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [DBNDAC](https://www.netlib.org/slatec/src/dbndac.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
