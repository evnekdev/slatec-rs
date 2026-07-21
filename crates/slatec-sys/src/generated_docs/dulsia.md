# Purpose

DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. User input bounds on the uncertainty in the elements of A are used to detect numerical rank deficiency. The algorithm employs a row and column pivot strategy to minimize the growth of uncertainty and round-off errors. DULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space * WARNING - All input arrays are changed on exit. * * Input.. All TYPE REAL variables are DOUBLE PRECISION

# Description

This canonical unsafe binding exposes original SLATEC routine `DULSIA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DULSIA](https://www.netlib.org/slatec/src/dulsia.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (MDA, *). B, with MDA the must be between 0 and 1. A minimum of 10*machine precision will be enforced. must be greater than or equal to 0. rithm will use that value for each column of A. The parameter KEY indicates whether scalars or vectors are being input. Contains the lower triangular part of the reduced matrix and the transformation information. It togeth with the first M elements of WORK (see below) completely specify the LQ factorization of A. B, with MDA the must be between 0 and 1. A minimum of 10*machine precision will be enforced. must be greater than or equal to 0. rithm will use that value for each column of A. The parameter KEY indicates whether scalars or vectors are being input. Contains the lower triangular part of the reduced matrix and the transformation information. It togeth with the first M elements of WORK (see below) completely specify the LQ factorization of A. not applicable or not stated by selected source not a workspace argument

## 2. `MDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. actual first dimension of A in the calling program. actual first dimension of A in the calling program. not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). Must have MDA.GE.M and M.LE.N. locations contain the order in which the rows of A were used. actual first dimension of A in the calling program. is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). Must have MDA.GE.M and M.LE.N. locations contain the order in which the rows of A were used. not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. actual first dimension of A in the calling program. contain the order in which the columns of A were used. The next actual first dimension of A in the calling program. contain the order in which the columns of A were used. The next not applicable or not stated by selected source not a workspace argument

## 5. `B`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (MDB, *). Right hand side(s), with MDB the actual first is the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N.  If Contains the N by NB solution matrix for X. AX(I), I=1,NB. If the matrix A is of Right hand side(s), with MDB the actual first is the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N.  If Contains the N by NB solution matrix for X. AX(I), I=1,NB. If the matrix A is of not applicable or not stated by selected source not a workspace argument

## 6. `MDB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N.  If is the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N.  If not applicable or not stated by selected source not a workspace argument

## 7. `NB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the is the number of M by 1 right hand sides.  Since the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N.  If solution is returned in B, must have MDB.GE.N.  If 0, B is never accessed. * is the is the number of M by 1 right hand sides.  Since the number of M by 1 right hand sides.  Since the solution is returned in B, must have MDB.GE.N.  If solution is returned in B, must have MDB.GE.N.  If 0, B is never accessed. * not applicable or not stated by selected source not a workspace argument

## 8. `RE`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are what make this             * code significantly different from               * other linear least squares solvers.             * However, the inexperienced user is              * 0.,AE=0.,KEY=0.               * * is a vector of length N such that RE(I) is is a vector of length N such that RE(I) is the maximum relative uncertainty in row I of the maximum relative uncertainty in row I of must be between 0 and 1. A minimum of 10*machine precision will be enforced. or AE have been specified as vectors, dimension WORK 4*M. If both RE and AE have been specified as vectors, dimension WORK 3*M. are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION are what make this             * code significantly different from               * other linear least squares solvers.             * However, the inexperienced user is              * 0.,AE=0.,KEY=0.               * * is a vector of length N such that RE(I) is is a vector of length N such that RE(I) is the maximum relative uncertainty in row I of the maximum relative uncertainty in row I of must be between 0 and 1. A minimum of 10*machine precision will be enforced. or AE have been specified as vectors, dimension WORK 4*M. If both RE and AE have been specified as vectors, dimension WORK 3*M. are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION not applicable or not stated by selected source not a workspace argument

## 9. `AE`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are what make this             * code significantly different from               * other linear least squares solvers.             * However, the inexperienced user is              * is a vector of length N such that AE(I) is is a vector of length N such that AE(I) is the maximum absolute uncertainty in row I of the maximum absolute uncertainty in row I of must be greater than or equal to 0. are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION are what make this             * code significantly different from               * other linear least squares solvers.             * However, the inexperienced user is              * is a vector of length N such that AE(I) is is a vector of length N such that AE(I) is the maximum absolute uncertainty in row I of the maximum absolute uncertainty in row I of must be greater than or equal to 0. are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION not applicable or not stated by selected source not a workspace argument

## 10. `KEY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. For ease of use, RE and AE may be input as either 0     RE scalar  AE scalar 1     RE vector  AE scalar 2     RE scalar  AE vector 3     RE vector  AE vector are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `MODE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The integer MODE indicates how the routine is to react if rank deficiency is detected. 0 return immediately, no solution 1 compute truncated solution 2 compute minimal length least squares sol 0 The integer MODE indicates how the routine is to react if rank deficiency is detected. 0 return immediately, no solution 1 compute truncated solution 2 compute minimal length least squares sol 0 not applicable or not stated by selected source not a workspace argument

## 12. `NP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. 0. WORK()        A real work array dimensioned 5*M.  However, if are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. 0. WORK()        A real work array dimensioned 5*M.  However, if are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION not applicable or not stated by selected source

## 13. `KRANK`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The numerical rank of A,  based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. The algorithm returns a solution based on KRANK. KSURE provides an indication of the precision of the rank. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `KSURE`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The numerical rank of A,  based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. The algorithm returns a solution based on KRANK. KSURE provides an indication of the precision of the rank. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 15. `RNORM`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Contains the Euclidean length of the NB residual 0.0. WORK()        The first M locations of WORK contain values necessary to reproduce the Householder transformation. Contains the Euclidean length of the NB residual 0.0. WORK()        The first M locations of WORK contain values necessary to reproduce the Householder transformation. not applicable or not stated by selected source not a workspace argument

## 16. `W`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Writable real work array for the rank-revealing LQ solve. It requires `5*M` elements when `RE` and `AE` are scalar, `4*M` when either is vector-valued, and `3*M` when both are vector-valued. Its leading entries are persistent factorization state for an `INFO=1` continuation call. not stated by selected source not applicable or not stated by selected source

## 17. `LW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Actual dimension of WORK IWORK, LIW, and the first 2*M locations of WORK as output by the original call to DULSIA. MODE must be equal to the value of MODE in the original call. If MODE.LT.2, only the first N locations of WORK Actual dimension of WORK IWORK, LIW, and the first 2*M locations of WORK as output by the original call to DULSIA. MODE must be equal to the value of MODE in the original call. If MODE.LT.2, only the first N locations of WORK not applicable or not stated by selected source not a workspace argument

## 18. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer work array dimensioned at least N+M. contain the order in which the columns of A were used. The next Integer work array dimensioned at least N+M. contain the order in which the columns of A were used. The next not applicable or not stated by selected source

## 19. `LIW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Actual dimension of IWORK. Actual dimension of IWORK. not applicable or not stated by selected source not a workspace argument

## 20. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Is a flag which provides for the efficient solution of subsequent problems involving the same A but different B. 0 original call 1 subsequent calls On subsequent calls, the user must supply A, KRANK, Flag to indicate status of computation on completion -1   Parameter error(s) 0 - Rank deficient, no solution 1 - Rank deficient, truncated solution 2 - Rank deficient, minimal length least squares sol 3 - Numerical rank 0, zero solution 4 - Rank .LT. NP 5 - Full rank Is a flag which provides for the efficient solution of subsequent problems involving the same A but different B. 0 original call 1 subsequent calls On subsequent calls, the user must supply A, KRANK, Flag to indicate status of computation on completion -1   Parameter error(s) 0 - Rank deficient, no solution 1 - Rank deficient, truncated solution 2 - Rank deficient, minimal length least squares sol 3 - Numerical rank 0, zero solution 4 - Rank .LT. NP 5 - Full rank not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `MDA`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `B`: not a workspace argument
- `MDB`: not a workspace argument
- `NB`: not a workspace argument
- `RE`: not a workspace argument
- `AE`: not a workspace argument
- `KEY`: not a workspace argument
- `MODE`: not a workspace argument
- `NP`: The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. 0. WORK()        A real work array dimensioned 5*M.  However, if are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION
- `KRANK`: not a workspace argument
- `KSURE`: not a workspace argument
- `RNORM`: not a workspace argument
- `W`: Writable real work array for the rank-revealing LQ solve. It requires `5*M` elements when `RE` and `AE` are scalar, `4*M` when either is vector-valued, and `3*M` when both are vector-valued. Its leading entries are persistent factorization state for an `INFO=1` continuation call.
- `LW`: not a workspace argument
- `IWORK`: Integer work array dimensioned at least N+M. contain the order in which the columns of A were used. The next
- `LIW`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dulsia`
- Original SLATEC routine: `DULSIA`
- Native symbol: `dulsia_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [DULSIA](https://www.netlib.org/slatec/src/dulsia.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
